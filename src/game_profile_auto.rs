// Game Profile Auto-Application
// Automatically detect game launches and apply GPU profiles

use crate::game_detection::GameDetector;
use crate::game_launcher::GameLauncher;
use crate::overclocking::OverclockProfile;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const SERVICE_STATE_FILE: &str = "game_profile_auto.state";
const SERVICE_PID_FILE: &str = "game_profile_auto.pid";
const SYSTEMD_SERVICE_NAME: &str = "nvcontrol-game-profile-auto.service";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoProfileConfig {
    pub enabled: bool,
    pub poll_interval_secs: u64,
    pub restore_on_exit: bool,
    pub apply_delay_secs: u64, // Wait before applying (in case game crashes immediately)
}

impl Default for AutoProfileConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            poll_interval_secs: 2,
            restore_on_exit: true,
            apply_delay_secs: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProfileState {
    pub game_executable: String,
    pub profile_name: String,
    pub applied_at: std::time::Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutoServiceState {
    pub enabled: bool,
    pub running: bool,
    pub pid: Option<u32>,
    pub last_profile_name: Option<String>,
}

/// Game profile auto-applier
pub struct GameProfileAutoApplier {
    detector: Arc<Mutex<GameDetector>>,
    config: AutoProfileConfig,
    active_profile: Arc<Mutex<Option<ProfileState>>>,
    default_profile: Option<OverclockProfile>,
    running: Arc<Mutex<bool>>,
}

impl GameProfileAutoApplier {
    pub fn new(detector: GameDetector, config: AutoProfileConfig) -> Self {
        Self {
            detector: Arc::new(Mutex::new(detector)),
            config,
            active_profile: Arc::new(Mutex::new(None)),
            default_profile: None,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Set the default profile to restore when games exit
    pub fn set_default_profile(&mut self, profile: OverclockProfile) {
        self.default_profile = Some(profile);
    }

    /// Start monitoring for game launches
    pub fn start(&self) -> NvResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let mut running = self.running.lock().unwrap_or_else(|e| e.into_inner());
        if *running {
            return Err(NvControlError::RuntimeError("Already running".into()));
        }
        *running = true;
        drop(running);

        let detector = Arc::clone(&self.detector);
        let active_profile = Arc::clone(&self.active_profile);
        let running_flag = Arc::clone(&self.running);
        let config = self.config.clone();
        let default_profile = self.default_profile.clone();
        save_service_state(&AutoServiceState {
            enabled: self.config.enabled,
            running: true,
            pid: Some(std::process::id()),
            last_profile_name: self.get_active_profile().map(|state| state.profile_name),
        })?;

        thread::spawn(move || {
            println!("🎮 Game profile auto-applier started");

            let mut last_detected_game: Option<String> = None;

            loop {
                // Check if we should stop
                {
                    let running = running_flag.lock().unwrap_or_else(|e| e.into_inner());
                    if !*running {
                        break;
                    }
                }

                // Scan for running games
                let detected_games = {
                    let mut det = detector.lock().unwrap_or_else(|e| e.into_inner());
                    det.scan_running_games()
                };

                if let Some(game) = detected_games.first() {
                    let game_exe = game.executable.clone();

                    // New game detected
                    if last_detected_game.as_ref() != Some(&game_exe) {
                        println!("🎮 Detected game: {} ({})", game.name, game_exe);

                        // Wait before applying (anti-crash protection)
                        if config.apply_delay_secs > 0 {
                            println!(
                                "   ⏳ Waiting {} seconds before applying profile...",
                                config.apply_delay_secs
                            );
                            thread::sleep(Duration::from_secs(config.apply_delay_secs));
                        }

                        // Apply profile if available
                        if let Some(ref profile) = game.profile {
                            println!("   ✅ Applying profile for {}", game.name);

                            // Apply GPU overclock
                            if let (Some(gpu), Some(mem)) =
                                (profile.gpu_clock_offset, profile.mem_clock_offset)
                            {
                                let oc_profile = crate::overclocking::OverclockProfile {
                                    name: profile.name.clone(),
                                    gpu_clock_offset: gpu,
                                    memory_clock_offset: mem,
                                    power_limit: profile.power_limit.unwrap_or(100) as u8,
                                    voltage_offset: 0,
                                    temp_limit: 85,
                                    fan_curve: Vec::new(),
                                };
                                if let Err(e) =
                                    crate::overclocking::apply_overclock_profile(&oc_profile)
                                {
                                    println!("   ⚠️  Failed to apply overclock: {}", e);
                                }
                            }

                            // Apply power limit
                            if let Some(power) = profile.power_limit {
                                if let Err(e) = crate::power::set_power_limit_percentage(power) {
                                    println!("   ⚠️  Failed to apply power limit: {}", e);
                                }
                            }

                            // Apply vibrance
                            if let Some(vib) = profile.vibrance {
                                if let Err(e) = crate::vibrance_native::set_vibrance_all_native(vib)
                                {
                                    println!("   ⚠️  Failed to apply vibrance: {}", e);
                                }
                            }

                            let state = ProfileState {
                                game_executable: game_exe.clone(),
                                profile_name: profile.name.clone(),
                                applied_at: std::time::Instant::now(),
                            };

                            *active_profile.lock().unwrap_or_else(|e| e.into_inner()) = Some(state);
                            let _ = save_service_state(&AutoServiceState {
                                enabled: config.enabled,
                                running: true,
                                pid: Some(std::process::id()),
                                last_profile_name: Some(profile.name.clone()),
                            });
                        } else {
                            println!("   ⚠️  No profile configured for {}", game.name);
                        }

                        last_detected_game = Some(game_exe);
                    }
                } else {
                    // No games detected - restore default if needed
                    if last_detected_game.is_some() {
                        println!("🎮 Game exited");

                        if config.restore_on_exit {
                            if let Some(ref default) = default_profile {
                                println!("   🔄 Restoring default profile");
                                if let Err(e) =
                                    crate::overclocking::apply_overclock_profile(default)
                                {
                                    println!("   ⚠️  Failed to restore default profile: {}", e);
                                }
                            }
                        }

                        last_detected_game = None;
                        *active_profile.lock().unwrap_or_else(|e| e.into_inner()) = None;
                        let _ = save_service_state(&AutoServiceState {
                            enabled: config.enabled,
                            running: true,
                            pid: Some(std::process::id()),
                            last_profile_name: None,
                        });
                    }
                }

                thread::sleep(Duration::from_secs(config.poll_interval_secs));
            }

            println!("🎮 Game profile auto-applier stopped");
            let _ = save_service_state(&AutoServiceState {
                enabled: config.enabled,
                running: false,
                pid: None,
                last_profile_name: None,
            });
            let _ = remove_pid_file();
        });

        Ok(())
    }

    /// Stop monitoring
    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap_or_else(|e| e.into_inner());
        *running = false;
        let _ = save_service_state(&AutoServiceState {
            enabled: self.config.enabled,
            running: false,
            pid: None,
            last_profile_name: self.get_active_profile().map(|state| state.profile_name),
        });
        let _ = remove_pid_file();
    }

    /// Get currently active profile
    pub fn get_active_profile(&self) -> Option<ProfileState> {
        self.active_profile
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap_or_else(|e| e.into_inner())
    }
}

fn service_state_path() -> NvResult<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?
        .join("nvcontrol");
    fs::create_dir_all(&config_dir)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to create config dir: {}", e)))?;
    Ok(config_dir.join(SERVICE_STATE_FILE))
}

fn service_pid_path() -> NvResult<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?
        .join("nvcontrol");
    fs::create_dir_all(&config_dir)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to create config dir: {}", e)))?;
    Ok(config_dir.join(SERVICE_PID_FILE))
}

fn write_pid_file(pid: u32) -> NvResult<()> {
    let path = service_pid_path()?;
    fs::write(&path, pid.to_string())
        .map_err(|e| NvControlError::ConfigError(format!("Failed to write PID file: {}", e)))
}

fn remove_pid_file() -> NvResult<()> {
    let path = service_pid_path()?;
    if path.exists() {
        fs::remove_file(path).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to remove PID file: {}", e))
        })?;
    }
    Ok(())
}

fn read_pid_file() -> NvResult<Option<u32>> {
    let path = service_pid_path()?;
    if !path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(&path)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to read PID file: {}", e)))?;
    Ok(contents.trim().parse::<u32>().ok())
}

fn pid_is_running(pid: u32) -> bool {
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn systemd_user_dir() -> NvResult<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?;
    let path = config_dir.join("systemd/user");
    fs::create_dir_all(&path)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to create systemd dir: {}", e)))?;
    Ok(path)
}

pub fn systemd_service_path() -> NvResult<PathBuf> {
    Ok(systemd_user_dir()?.join(SYSTEMD_SERVICE_NAME))
}

pub fn systemd_service_unit() -> NvResult<String> {
    let exe = std::env::current_exe()
        .map_err(|e| NvControlError::RuntimeError(format!("Failed to locate nvctl: {}", e)))?;
    Ok(format!(
        "[Unit]\nDescription=nvcontrol Game Profile Auto Service\nAfter=graphical-session.target\nPartOf=graphical-session.target\n\n[Service]\nType=simple\nExecStart={} gaming auto daemon\nRestart=on-failure\nRestartSec=2\n\n[Install]\nWantedBy=default.target\n",
        exe.display()
    ))
}

fn run_systemctl_user(args: &[&str]) -> NvResult<()> {
    let output = Command::new("systemctl")
        .arg("--user")
        .args(args)
        .output()
        .map_err(|e| NvControlError::RuntimeError(format!("Failed to run systemctl: {}", e)))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(NvControlError::RuntimeError(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ))
    }
}

pub fn install_systemd_user_service() -> NvResult<PathBuf> {
    let service_path = systemd_service_path()?;
    fs::write(&service_path, systemd_service_unit()?).map_err(|e| {
        NvControlError::ConfigError(format!("Failed to write systemd service: {}", e))
    })?;
    run_systemctl_user(&["daemon-reload"])?;
    Ok(service_path)
}

pub fn uninstall_systemd_user_service() -> NvResult<()> {
    let service_path = systemd_service_path()?;
    let _ = run_systemctl_user(&["disable", "--now", SYSTEMD_SERVICE_NAME]);
    if service_path.exists() {
        fs::remove_file(&service_path).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to remove systemd service: {}", e))
        })?;
    }
    run_systemctl_user(&["daemon-reload"])
}

pub fn enable_systemd_user_service() -> NvResult<()> {
    run_systemctl_user(&["enable", "--now", SYSTEMD_SERVICE_NAME])
}

pub fn disable_systemd_user_service() -> NvResult<()> {
    run_systemctl_user(&["disable", "--now", SYSTEMD_SERVICE_NAME])
}

pub fn systemd_service_status() -> NvResult<String> {
    let output = Command::new("systemctl")
        .arg("--user")
        .args(["is-enabled", SYSTEMD_SERVICE_NAME])
        .output()
        .map_err(|e| NvControlError::RuntimeError(format!("Failed to run systemctl: {}", e)))?;
    let enabled = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "disabled".to_string()
    };

    let active_output = Command::new("systemctl")
        .arg("--user")
        .args(["is-active", SYSTEMD_SERVICE_NAME])
        .output()
        .map_err(|e| NvControlError::RuntimeError(format!("Failed to run systemctl: {}", e)))?;
    let active = if active_output.status.success() {
        String::from_utf8_lossy(&active_output.stdout)
            .trim()
            .to_string()
    } else {
        "inactive".to_string()
    };

    Ok(format!("systemd enabled={} active={}", enabled, active))
}

pub fn load_service_state() -> NvResult<AutoServiceState> {
    let path = service_state_path()?;
    if !path.exists() {
        return Ok(AutoServiceState::default());
    }

    let contents = fs::read_to_string(&path)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to read state: {}", e)))?;
    let mut state: AutoServiceState = toml::from_str(&contents)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to parse state: {}", e)))?;

    if let Some(pid) = state.pid {
        if !pid_is_running(pid) {
            state.running = false;
            state.pid = None;
        }
    } else {
        state.running = false;
    }

    Ok(state)
}

pub fn save_service_state(state: &AutoServiceState) -> NvResult<()> {
    let path = service_state_path()?;
    let contents = toml::to_string_pretty(state)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to serialize state: {}", e)))?;
    fs::write(&path, contents)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to write state: {}", e)))
}

pub fn start_auto_service() -> NvResult<()> {
    let config = load_config()?;
    if !config.enabled {
        return Err(NvControlError::RuntimeError(
            "Auto-profile service is disabled; run `nvctl gaming auto enable` first".into(),
        ));
    }

    if let Ok(state) = load_service_state() {
        if state.running {
            return Err(NvControlError::RuntimeError(
                "Auto-profile service is already running".into(),
            ));
        }
    }

    let exe = std::env::current_exe()
        .map_err(|e| NvControlError::RuntimeError(format!("Failed to locate nvctl: {}", e)))?;

    let child = Command::new(exe)
        .args(["gaming", "auto", "daemon"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| NvControlError::RuntimeError(format!("Failed to spawn daemon: {}", e)))?;

    write_pid_file(child.id())?;
    save_service_state(&AutoServiceState {
        enabled: config.enabled,
        running: true,
        pid: Some(child.id()),
        last_profile_name: None,
    })
}

pub fn stop_auto_service() -> NvResult<()> {
    let mut state = load_service_state()?;
    if let Some(pid) = state.pid.or(read_pid_file()?) {
        let output = Command::new("kill")
            .args(["-TERM", &pid.to_string()])
            .output()
            .map_err(|e| NvControlError::RuntimeError(format!("Failed to stop daemon: {}", e)))?;

        if !output.status.success() && pid_is_running(pid) {
            return Err(NvControlError::RuntimeError(format!(
                "Failed to stop auto-profile service PID {}",
                pid
            )));
        }
    }

    state.running = false;
    state.pid = None;
    remove_pid_file()?;
    save_service_state(&state)
}

pub fn run_auto_service_foreground() -> NvResult<()> {
    let detector = GameDetector::new()?;
    let config = load_config()?;
    let applier = GameProfileAutoApplier::new(detector, config);
    write_pid_file(std::process::id())?;
    applier.start()?;

    while load_service_state()?.running {
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}

pub fn auto_service_status_summary() -> NvResult<String> {
    let mut state = load_service_state()?;
    if let Ok(config) = load_config() {
        state.enabled = config.enabled;
    }
    Ok(format!(
        "enabled={} running={} last_profile={}",
        state.enabled,
        state.running,
        state
            .last_profile_name
            .unwrap_or_else(|| "none".to_string())
    ))
}

pub fn sync_launcher_profiles_into_detector() -> NvResult<usize> {
    let launcher = GameLauncher::new()?;
    let detector = GameDetector::new()?;
    let mut synced = 0;

    for profile_name in launcher.list_profiles() {
        if let Ok(profile) = launcher.load_profile(&profile_name) {
            detector.save_profile(&profile)?;
            synced += 1;
        }
    }

    Ok(synced)
}

/// Load configuration
pub fn load_config() -> NvResult<AutoProfileConfig> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?
        .join("nvcontrol");

    let config_path = config_dir.join("game_profile_auto.toml");

    if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read config: {}", e)))?;

        toml::from_str(&contents)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse config: {}", e)))
    } else {
        Ok(AutoProfileConfig::default())
    }
}

/// Save configuration
pub fn save_config(config: &AutoProfileConfig) -> NvResult<()> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?
        .join("nvcontrol");

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to create config dir: {}", e)))?;

    let config_path = config_dir.join("game_profile_auto.toml");

    let toml = toml::to_string_pretty(config)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to serialize config: {}", e)))?;

    std::fs::write(&config_path, toml)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to write config: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AutoProfileConfig::default();
        assert!(config.enabled);
        assert_eq!(config.poll_interval_secs, 2);
    }
}
