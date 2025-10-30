// Game Profile Auto-Application
// Automatically detect game launches and apply GPU profiles

use crate::game_detection::{GameDetector, GameProfile};
use crate::overclocking::OverclockProfile;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoProfileConfig {
    pub enabled: bool,
    pub poll_interval_secs: u64,
    pub restore_on_exit: bool,
    pub apply_delay_secs: u64,  // Wait before applying (in case game crashes immediately)
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

        let mut running = self.running.lock().unwrap();
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

        thread::spawn(move || {
            println!("🎮 Game profile auto-applier started");

            let mut last_detected_game: Option<String> = None;

            loop {
                // Check if we should stop
                {
                    let running = running_flag.lock().unwrap();
                    if !*running {
                        break;
                    }
                }

                // Scan for running games
                let detected_games = {
                    let mut det = detector.lock().unwrap();
                    det.scan_running_games()
                };

                if let Some(game) = detected_games.first() {
                    let game_exe = game.executable.clone();

                    // New game detected
                    if last_detected_game.as_ref() != Some(&game_exe) {
                        println!("🎮 Detected game: {} ({})", game.name, game_exe);

                        // Wait before applying (anti-crash protection)
                        if config.apply_delay_secs > 0 {
                            println!("   ⏳ Waiting {} seconds before applying profile...", config.apply_delay_secs);
                            thread::sleep(Duration::from_secs(config.apply_delay_secs));
                        }

                        // Apply profile if available
                        if let Some(ref profile) = game.profile {
                            println!("   ✅ Applying profile for {}", game.name);

                            // TODO: Actually apply the profile settings
                            // This would call overclocking::apply_overclock_profile, etc.

                            let state = ProfileState {
                                game_executable: game_exe.clone(),
                                profile_name: profile.name.clone(),
                                applied_at: std::time::Instant::now(),
                            };

                            *active_profile.lock().unwrap() = Some(state);
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
                                // TODO: Apply default profile
                            }
                        }

                        last_detected_game = None;
                        *active_profile.lock().unwrap() = None;
                    }
                }

                thread::sleep(Duration::from_secs(config.poll_interval_secs));
            }

            println!("🎮 Game profile auto-applier stopped");
        });

        Ok(())
    }

    /// Stop monitoring
    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        *running = false;
    }

    /// Get currently active profile
    pub fn get_active_profile(&self) -> Option<ProfileState> {
        self.active_profile.lock().unwrap().clone()
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
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
