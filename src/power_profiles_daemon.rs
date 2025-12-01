// Dynamic Power Profile Integration with systemd and power-profiles-daemon
// Auto-switch NVIDIA power modes based on workload and system state

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemPowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NvidiaPowerMode {
    MaxPerformance, // Highest clocks, no power saving
    Adaptive,       // Balance performance and power
    MaxPowerSaving, // Lowest clocks, maximum power saving
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerProfileConfig {
    pub system_profile: SystemPowerProfile,
    pub nvidia_mode: NvidiaPowerMode,
    pub gpu_clock_offset: i32,
    pub mem_clock_offset: i32,
    pub power_limit: Option<u32>,
    pub fan_control: FanMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FanMode {
    Auto,
    Aggressive,
    Silent,
    Manual(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityProfile {
    pub activity_name: String,
    pub power_config: PowerProfileConfig,
}

pub struct PowerProfileManager {
    current_profile: SystemPowerProfile,
    activity_profiles: HashMap<String, PowerProfileConfig>,
}

impl PowerProfileManager {
    pub fn new() -> NvResult<Self> {
        let current_profile = Self::detect_current_profile()?;

        Ok(Self {
            current_profile,
            activity_profiles: HashMap::new(),
        })
    }

    /// Detect current system power profile
    pub fn detect_current_profile() -> NvResult<SystemPowerProfile> {
        // Try power-profiles-daemon first
        if let Ok(output) = Command::new("powerprofilesctl").arg("get").output() {
            let profile = String::from_utf8_lossy(&output.stdout);
            return match profile.trim() {
                "performance" => Ok(SystemPowerProfile::Performance),
                "balanced" => Ok(SystemPowerProfile::Balanced),
                "power-saver" => Ok(SystemPowerProfile::PowerSaver),
                _ => Ok(SystemPowerProfile::Balanced),
            };
        }

        // Fallback to checking AC power
        if Self::is_on_ac_power()? {
            Ok(SystemPowerProfile::Performance)
        } else {
            Ok(SystemPowerProfile::PowerSaver)
        }
    }

    /// Check if system is on AC power
    pub fn is_on_ac_power() -> NvResult<bool> {
        // Check via upower
        if let Ok(output) = Command::new("upower")
            .args(&["-i", "/org/freedesktop/UPower/devices/line_power_AC"])
            .output()
        {
            let info = String::from_utf8_lossy(&output.stdout);
            return Ok(info.contains("online: yes"));
        }

        // Fallback: check /sys
        if let Ok(content) = std::fs::read_to_string("/sys/class/power_supply/AC/online") {
            return Ok(content.trim() == "1");
        }

        // Default to AC if we can't determine
        Ok(true)
    }

    /// Set system power profile
    pub fn set_system_profile(&mut self, profile: SystemPowerProfile) -> NvResult<()> {
        println!("⚡ Setting system power profile: {:?}", profile);

        let profile_name = match profile {
            SystemPowerProfile::Performance => "performance",
            SystemPowerProfile::Balanced => "balanced",
            SystemPowerProfile::PowerSaver => "power-saver",
        };

        // Try power-profiles-daemon
        let status = Command::new("powerprofilesctl")
            .arg("set")
            .arg(profile_name)
            .status();

        if status.is_err() {
            eprintln!("⚠️  power-profiles-daemon not available, applying manual settings");
        }

        // Apply corresponding NVIDIA settings
        self.apply_nvidia_profile(profile)?;
        self.current_profile = profile;

        println!("✅ Power profile applied");
        Ok(())
    }

    /// Apply NVIDIA-specific power settings
    fn apply_nvidia_profile(&self, profile: SystemPowerProfile) -> NvResult<()> {
        let power_mode = match profile {
            SystemPowerProfile::Performance => NvidiaPowerMode::MaxPerformance,
            SystemPowerProfile::Balanced => NvidiaPowerMode::Adaptive,
            SystemPowerProfile::PowerSaver => NvidiaPowerMode::MaxPowerSaving,
        };

        self.set_nvidia_power_mode(power_mode)?;

        Ok(())
    }

    /// Set NVIDIA power management mode
    fn set_nvidia_power_mode(&self, mode: NvidiaPowerMode) -> NvResult<()> {
        let mode_value = match mode {
            NvidiaPowerMode::MaxPerformance => "0",
            NvidiaPowerMode::Adaptive => "1",
            NvidiaPowerMode::MaxPowerSaving => "2",
        };

        // Use nvidia-settings to set power mode
        let status = Command::new("nvidia-settings")
            .args(&["-a", &format!("[gpu:0]/GPUPowerMizerMode={}", mode_value)])
            .status();

        if let Err(e) = status {
            eprintln!("⚠️  Failed to set NVIDIA power mode: {}", e);
        }

        Ok(())
    }

    /// Create KDE Activity-based profile
    pub fn create_activity_profile(
        &mut self,
        activity: &str,
        config: PowerProfileConfig,
    ) -> NvResult<()> {
        println!("📝 Creating activity profile: {}", activity);

        self.activity_profiles.insert(activity.to_string(), config);

        // Save to config file
        self.save_activity_profiles()?;

        println!("✅ Activity profile created");
        Ok(())
    }

    /// Apply profile based on KDE Activity
    pub fn apply_activity_profile(&mut self, activity: &str) -> NvResult<()> {
        if let Some(config) = self.activity_profiles.get(activity).cloned() {
            println!("🎯 Applying profile for activity: {}", activity);

            // Set system profile
            self.set_system_profile(config.system_profile)?;

            // Apply GPU overclocking if specified
            if config.gpu_clock_offset != 0 || config.mem_clock_offset != 0 {
                self.apply_gpu_overclocking(config.gpu_clock_offset, config.mem_clock_offset)?;
            }

            // Apply power limit
            if let Some(power_limit) = config.power_limit {
                self.set_power_limit(power_limit)?;
            }

            // Apply fan control
            self.set_fan_mode(config.fan_control)?;

            println!("✅ Activity profile applied");
            Ok(())
        } else {
            Err(NvControlError::ConfigError(format!(
                "Activity profile not found: {}",
                activity
            )))
        }
    }

    /// Get current KDE Activity
    pub fn get_current_activity() -> NvResult<String> {
        let output = Command::new("qdbus")
            .args(&[
                "org.kde.ActivityManager",
                "/ActivityManager/Activities",
                "org.kde.ActivityManager.Activities.CurrentActivity",
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to get activity: {}", e)))?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Monitor activity changes and auto-switch profiles
    pub fn monitor_activity_changes(&mut self) -> NvResult<()> {
        println!("👁️  Monitoring KDE Activity changes...");
        println!("   Press Ctrl+C to stop\n");

        let mut last_activity = Self::get_current_activity().unwrap_or_default();

        loop {
            std::thread::sleep(std::time::Duration::from_secs(2));

            if let Ok(current_activity) = Self::get_current_activity() {
                if current_activity != last_activity && !current_activity.is_empty() {
                    println!(
                        "🔄 Activity changed: {} -> {}",
                        last_activity, current_activity
                    );

                    if let Err(e) = self.apply_activity_profile(&current_activity) {
                        eprintln!("⚠️  Failed to apply profile: {}", e);
                    }

                    last_activity = current_activity;
                }
            }
        }
    }

    /// Auto-switch based on AC/Battery
    pub fn auto_switch_on_power_change(&mut self) -> NvResult<()> {
        println!("🔌 Monitoring AC/Battery changes...");
        println!("   Press Ctrl+C to stop\n");

        let mut last_on_ac = Self::is_on_ac_power()?;

        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));

            if let Ok(on_ac) = Self::is_on_ac_power() {
                if on_ac != last_on_ac {
                    println!(
                        "🔄 Power source changed: {} -> {}",
                        if last_on_ac { "AC" } else { "Battery" },
                        if on_ac { "AC" } else { "Battery" }
                    );

                    let profile = if on_ac {
                        SystemPowerProfile::Performance
                    } else {
                        SystemPowerProfile::PowerSaver
                    };

                    if let Err(e) = self.set_system_profile(profile) {
                        eprintln!("⚠️  Failed to switch profile: {}", e);
                    }

                    last_on_ac = on_ac;
                }
            }
        }
    }

    /// Detect idle state and reduce power
    pub fn idle_detection(&mut self, idle_timeout_secs: u64) -> NvResult<()> {
        println!(
            "💤 Idle detection enabled (timeout: {}s)",
            idle_timeout_secs
        );

        let mut idle_start: Option<std::time::Instant> = None;
        let mut in_idle_mode = false;

        loop {
            std::thread::sleep(std::time::Duration::from_secs(10));

            let idle_time = Self::get_idle_time()?;

            if idle_time > idle_timeout_secs {
                if idle_start.is_none() {
                    idle_start = Some(std::time::Instant::now());
                }

                if !in_idle_mode {
                    println!("💤 System idle detected, reducing GPU power");
                    self.set_nvidia_power_mode(NvidiaPowerMode::MaxPowerSaving)?;
                    in_idle_mode = true;
                }
            } else {
                if in_idle_mode {
                    println!("⚡ System active, restoring GPU power");
                    self.apply_nvidia_profile(self.current_profile)?;
                    in_idle_mode = false;
                }
                idle_start = None;
            }
        }
    }

    /// Get system idle time in seconds
    fn get_idle_time() -> NvResult<u64> {
        // Use xprintidle or wayland equivalent
        if let Ok(output) = Command::new("xprintidle").output() {
            let idle_ms = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<u64>()
                .unwrap_or(0);
            return Ok(idle_ms / 1000);
        }

        // Fallback: assume not idle
        Ok(0)
    }

    /// Apply GPU overclocking
    fn apply_gpu_overclocking(&self, gpu_offset: i32, mem_offset: i32) -> NvResult<()> {
        println!(
            "⚙️  Applying GPU overclocking: GPU +{} MHz, Memory +{} MHz",
            gpu_offset, mem_offset
        );

        // Set GPU clock offset
        Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!(
                    "[gpu:0]/GPUGraphicsClockOffsetAllPerformanceLevels={}",
                    gpu_offset
                ),
            ])
            .status()
            .ok();

        // Set memory clock offset
        Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!(
                    "[gpu:0]/GPUMemoryTransferRateOffsetAllPerformanceLevels={}",
                    mem_offset
                ),
            ])
            .status()
            .ok();

        Ok(())
    }

    /// Set GPU power limit
    fn set_power_limit(&self, watts: u32) -> NvResult<()> {
        println!("⚡ Setting power limit: {} W", watts);

        // nvidia-smi for power limit
        let status = Command::new("nvidia-smi")
            .args(&["-pl", &watts.to_string()])
            .status();

        if let Err(e) = status {
            return Err(NvControlError::CommandFailed(format!(
                "Failed to set power limit: {}",
                e
            )));
        }

        Ok(())
    }

    /// Set fan control mode
    fn set_fan_mode(&self, mode: FanMode) -> NvResult<()> {
        match mode {
            FanMode::Auto => {
                println!("🌡️  Setting fan to auto mode");
                Command::new("nvidia-settings")
                    .args(&["-a", "[gpu:0]/GPUFanControlState=0"])
                    .status()
                    .ok();
            }
            FanMode::Manual(speed) => {
                println!("🌡️  Setting fan to {}%", speed);
                Command::new("nvidia-settings")
                    .args(&["-a", "[gpu:0]/GPUFanControlState=1"])
                    .status()
                    .ok();
                Command::new("nvidia-settings")
                    .args(&["-a", &format!("[fan:0]/GPUTargetFanSpeed={}", speed)])
                    .status()
                    .ok();
            }
            _ => {}
        }

        Ok(())
    }

    /// Save activity profiles to config
    fn save_activity_profiles(&self) -> NvResult<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| {
                NvControlError::ConfigError("Could not find config directory".to_string())
            })?
            .join("nvcontrol");

        std::fs::create_dir_all(&config_dir).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create config dir: {}", e))
        })?;

        let config_file = config_dir.join("activity_profiles.toml");

        let content = toml::to_string_pretty(&self.activity_profiles).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize config: {}", e))
        })?;

        std::fs::write(&config_file, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    /// Print current status
    pub fn print_status(&self) -> NvResult<()> {
        println!("⚡ Power Profile Status\n");

        println!("System Profile: {:?}", self.current_profile);

        if let Ok(on_ac) = Self::is_on_ac_power() {
            println!(
                "Power Source: {}",
                if on_ac { "AC 🔌" } else { "Battery 🔋" }
            );
        }

        if let Ok(activity) = Self::get_current_activity() {
            println!("Current KDE Activity: {}", activity);
        }

        println!("\nActivity Profiles:");
        if self.activity_profiles.is_empty() {
            println!("   (none configured)");
        } else {
            for (name, _config) in &self.activity_profiles {
                println!("   • {}", name);
            }
        }

        Ok(())
    }

    /// Create default activity profiles
    pub fn create_default_profiles(&mut self) -> NvResult<()> {
        println!("📝 Creating default activity profiles...\n");

        // Gaming profile
        self.create_activity_profile(
            "Gaming",
            PowerProfileConfig {
                system_profile: SystemPowerProfile::Performance,
                nvidia_mode: NvidiaPowerMode::MaxPerformance,
                gpu_clock_offset: 100,
                mem_clock_offset: 200,
                power_limit: None,
                fan_control: FanMode::Aggressive,
            },
        )?;

        // Work profile
        self.create_activity_profile(
            "Work",
            PowerProfileConfig {
                system_profile: SystemPowerProfile::Balanced,
                nvidia_mode: NvidiaPowerMode::Adaptive,
                gpu_clock_offset: 0,
                mem_clock_offset: 0,
                power_limit: None,
                fan_control: FanMode::Auto,
            },
        )?;

        // Media profile
        self.create_activity_profile(
            "Media",
            PowerProfileConfig {
                system_profile: SystemPowerProfile::Balanced,
                nvidia_mode: NvidiaPowerMode::Adaptive,
                gpu_clock_offset: 0,
                mem_clock_offset: 0,
                power_limit: None,
                fan_control: FanMode::Silent,
            },
        )?;

        println!("✅ Created 3 default activity profiles");
        Ok(())
    }
}
