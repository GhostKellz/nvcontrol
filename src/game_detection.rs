// Game Detection and Profile Auto-Switching
// Monitors running processes and automatically applies GPU profiles per-game

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProfile {
    pub name: String,
    pub executable: String,
    pub gpu_offset: Option<i32>,
    pub memory_offset: Option<i32>,
    pub power_limit: Option<u32>,
    pub fan_curve: Option<Vec<(u32, u32)>>, // (temp, speed) pairs
    pub vibrance: Option<u32>,
    pub fps_limit: Option<u32>,
    pub priority: ProcessPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessPriority {
    Low,
    Normal,
    High,
    Realtime,
}

#[derive(Debug, Clone)]
pub struct DetectedGame {
    pub name: String,
    pub executable: String,
    pub pid: u32,
    pub profile: Option<GameProfile>,
}

pub struct GameDetector {
    profiles: HashMap<String, GameProfile>,
    profiles_dir: PathBuf,
    system: System,
    active_games: Vec<DetectedGame>,
}

impl GameDetector {
    pub fn new() -> NvResult<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find config directory".into()))?
            .join("nvcontrol")
            .join("game_profiles");

        fs::create_dir_all(&config_dir)?;

        let mut detector = Self {
            profiles: HashMap::new(),
            profiles_dir: config_dir,
            system: System::new_all(),
            active_games: Vec::new(),
        };

        detector.load_profiles()?;
        Ok(detector)
    }

    fn load_profiles(&mut self) -> NvResult<()> {
        if !self.profiles_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.profiles_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let contents = fs::read_to_string(&path)?;
                if let Ok(profile) = toml::from_str::<GameProfile>(&contents) {
                    self.profiles.insert(profile.executable.clone(), profile);
                }
            }
        }

        println!("📂 Loaded {} game profiles", self.profiles.len());
        Ok(())
    }

    pub fn save_profile(&self, profile: &GameProfile) -> NvResult<()> {
        let filename = format!("{}.toml", profile.executable.replace("/", "_"));
        let path = self.profiles_dir.join(filename);

        let toml_str = toml::to_string_pretty(profile).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize profile: {}", e))
        })?;

        fs::write(path, toml_str)?;
        println!("✅ Saved profile for {}", profile.name);
        Ok(())
    }

    pub fn delete_profile(&mut self, executable: &str) -> NvResult<()> {
        let filename = format!("{}.toml", executable.replace("/", "_"));
        let path = self.profiles_dir.join(filename);

        if path.exists() {
            fs::remove_file(path)?;
        }

        self.profiles.remove(executable);
        println!("🗑️  Deleted profile for {}", executable);
        Ok(())
    }

    /// Scan for running games and detect profile matches
    pub fn scan_running_games(&mut self) -> Vec<DetectedGame> {
        self.system.refresh_processes();
        self.active_games.clear();

        for (pid, process) in self.system.processes() {
            let exe_name = process.name();
            let exe_path = process
                .exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            // Check if this process matches any known game profile
            if let Some(profile) = self.profiles.get(exe_name).or_else(|| {
                // Try matching by full path
                self.profiles.iter().find_map(
                    |(k, v)| {
                        if exe_path.contains(k) { Some(v) } else { None }
                    },
                )
            }) {
                self.active_games.push(DetectedGame {
                    name: profile.name.clone(),
                    executable: exe_name.to_string(),
                    pid: pid.as_u32(),
                    profile: Some(profile.clone()),
                });
            }
        }

        self.active_games.clone()
    }

    /// Get list of common game executables to watch for
    pub fn get_common_game_executables() -> Vec<&'static str> {
        vec![
            // Steam games
            "steam",
            "steamwebhelper",
            // Popular games
            "cs2",
            "csgo",
            "dota2",
            "valheim",
            "eldenring.exe",
            "cyberpunk2077.exe",
            "witcher3.exe",
            "rdr2.exe",
            "gta5.exe",
            // Wine/Proton
            "wine64-preloader",
            "wine-preloader",
            "wineserver",
            // Launchers
            "lutris",
            "heroic",
            "legendary",
            "bottles",
        ]
    }

    /// Create a default profile for a game
    pub fn create_default_profile(&self, name: &str, executable: &str) -> GameProfile {
        GameProfile {
            name: name.to_string(),
            executable: executable.to_string(),
            gpu_offset: Some(100),
            memory_offset: Some(200),
            power_limit: None,
            fan_curve: None,
            vibrance: Some(150),
            fps_limit: None,
            priority: ProcessPriority::High,
        }
    }

    pub fn get_profile(&self, executable: &str) -> Option<&GameProfile> {
        self.profiles.get(executable)
    }

    pub fn get_all_profiles(&self) -> Vec<&GameProfile> {
        self.profiles.values().collect()
    }

    pub fn get_active_games(&self) -> &[DetectedGame] {
        &self.active_games
    }

    /// Apply a game profile (overclocking, fan curve, vibrance, etc.)
    pub fn apply_profile(&self, profile: &GameProfile) -> NvResult<()> {
        println!("🎮 Applying profile for: {}", profile.name);

        // Apply GPU overclock
        if let (Some(gpu), Some(mem)) = (profile.gpu_offset, profile.memory_offset) {
            println!("   ⚡ GPU: +{} MHz, Memory: +{} MHz", gpu, mem);
            let oc_profile = crate::overclocking::OverclockProfile {
                name: profile.name.clone(),
                gpu_clock_offset: gpu,
                memory_clock_offset: mem,
                power_limit: profile.power_limit.unwrap_or(100) as u8,
                voltage_offset: 0,
                temp_limit: 85,
                fan_curve: Vec::new(),
            };
            crate::overclocking::apply_overclock_profile(&oc_profile)?;
        }

        // Apply power limit
        if let Some(power) = profile.power_limit {
            println!("   🔋 Power limit: {}%", power);
            crate::power::set_power_limit_percentage(power)?;
        }

        // Apply fan curve
        if let Some(ref curve) = profile.fan_curve {
            println!("   🌀 Fan curve: {} points", curve.len());
            let curve_points: Vec<(u8, u8)> = curve
                .iter()
                .map(|(temp, speed)| (*temp as u8, *speed as u8))
                .collect();
            crate::fan::set_fan_curve(0, &curve_points)?;
        }

        // Apply vibrance
        if let Some(vib) = profile.vibrance {
            println!("   🌈 Vibrance: {}%", vib);
            crate::vibrance::set_vibrance_all(vib as i32)?;
        }

        // Set process priority
        self.set_process_priority(profile)?;

        Ok(())
    }

    fn set_process_priority(&self, profile: &GameProfile) -> NvResult<()> {
        #[cfg(target_os = "linux")]
        {
            // Find process by executable name
            if let Some((pid, _)) = self
                .system
                .processes()
                .iter()
                .find(|(_, proc)| proc.name() == profile.executable.as_str())
            {
                let nice_value = match profile.priority {
                    ProcessPriority::Realtime => -20,
                    ProcessPriority::High => -10,
                    ProcessPriority::Normal => 0,
                    ProcessPriority::Low => 10,
                };

                let _ = Command::new("renice")
                    .arg(nice_value.to_string())
                    .arg("-p")
                    .arg(pid.as_u32().to_string())
                    .output();
            }
        }

        Ok(())
    }
}

/// Background monitoring service for game detection
pub struct GameMonitorService {
    detector: GameDetector,
    check_interval_ms: u64,
    auto_apply: bool,
}

impl GameMonitorService {
    pub fn new(auto_apply: bool) -> NvResult<Self> {
        Ok(Self {
            detector: GameDetector::new()?,
            check_interval_ms: 2000, // Check every 2 seconds
            auto_apply,
        })
    }

    /// Run the monitoring service (blocking)
    pub fn run(&mut self) -> NvResult<()> {
        println!("🎮 Game detection service started");
        println!("   Auto-apply profiles: {}", self.auto_apply);

        loop {
            let games = self.detector.scan_running_games();

            for game in &games {
                if let Some(ref profile) = game.profile {
                    println!("🎯 Detected: {} (PID: {})", game.name, game.pid);

                    if self.auto_apply {
                        if let Err(e) = self.detector.apply_profile(profile) {
                            eprintln!("❌ Failed to apply profile: {}", e);
                        }
                    }
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(self.check_interval_ms));
        }
    }

    pub fn set_check_interval(&mut self, interval_ms: u64) {
        self.check_interval_ms = interval_ms;
    }
}
