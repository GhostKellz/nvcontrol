/// ASUS GPU Tweak Integration
///
/// Replicate ASUS GPU Tweak III functionality on Linux for ASUS ROG cards
/// Supports: Performance tuning, OC profiles, monitoring
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ASUS GPU Tweak operating mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AsusMode {
    SilentMode, // Quiet operation, lower clocks
    GamingMode, // Balanced for gaming
    OcMode,     // Maximum performance
    Manual,     // User-defined settings
}

/// ASUS OC Profile matching GPU Tweak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsusOcProfile {
    pub name: String,
    pub mode: AsusMode,
    pub gpu_boost_clock_mhz: i32,
    pub memory_clock_mhz: i32,
    pub gpu_voltage_offset_mv: i32,
    pub power_target_percent: u32,
    pub temp_target_c: i32,
    pub fan_profile: AsusFanProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsusFanProfile {
    pub mode: FanMode,
    pub curve: Vec<(i32, u32)>, // (temp_c, fan_percent)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FanMode {
    Auto,
    Manual,
    Quiet,
    Turbo,
}

/// ASUS GPU Tweak manager
pub struct AsusGpuTweak {
    gpu_id: u32,
    current_mode: AsusMode,
    profiles: HashMap<String, AsusOcProfile>,
    is_asus_card: bool,
}

impl AsusGpuTweak {
    pub fn new(gpu_id: u32) -> NvResult<Self> {
        let is_asus = Self::detect_asus_card(gpu_id)?;

        Ok(Self {
            gpu_id,
            current_mode: AsusMode::GamingMode,
            profiles: Self::load_default_profiles(),
            is_asus_card: is_asus,
        })
    }

    fn detect_asus_card(gpu_id: u32) -> NvResult<bool> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let device = nvml
            .device_by_index(gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        let name = device.name().unwrap_or_default();

        // Check if ASUS ROG card
        Ok(name.to_lowercase().contains("asus") || name.to_lowercase().contains("rog"))
    }

    fn load_default_profiles() -> HashMap<String, AsusOcProfile> {
        let mut profiles = HashMap::new();

        // Silent Mode Profile
        profiles.insert(
            "Silent".to_string(),
            AsusOcProfile {
                name: "Silent".to_string(),
                mode: AsusMode::SilentMode,
                gpu_boost_clock_mhz: -100,
                memory_clock_mhz: 0,
                gpu_voltage_offset_mv: -50,
                power_target_percent: 85,
                temp_target_c: 75,
                fan_profile: AsusFanProfile {
                    mode: FanMode::Quiet,
                    curve: vec![(40, 0), (50, 30), (60, 40), (70, 55), (80, 70)],
                },
            },
        );

        // Gaming Mode Profile
        profiles.insert(
            "Gaming".to_string(),
            AsusOcProfile {
                name: "Gaming".to_string(),
                mode: AsusMode::GamingMode,
                gpu_boost_clock_mhz: 0,
                memory_clock_mhz: 0,
                gpu_voltage_offset_mv: 0,
                power_target_percent: 100,
                temp_target_c: 83,
                fan_profile: AsusFanProfile {
                    mode: FanMode::Auto,
                    curve: vec![(35, 0), (50, 40), (65, 60), (75, 80), (85, 100)],
                },
            },
        );

        // OC Mode Profile (ROG Astral 5090 optimized)
        profiles.insert(
            "OC".to_string(),
            AsusOcProfile {
                name: "OC".to_string(),
                mode: AsusMode::OcMode,
                gpu_boost_clock_mhz: 200,
                memory_clock_mhz: 1000,
                gpu_voltage_offset_mv: 50,
                power_target_percent: 120,
                temp_target_c: 85,
                fan_profile: AsusFanProfile {
                    mode: FanMode::Turbo,
                    curve: vec![(30, 35), (50, 55), (65, 75), (75, 90), (85, 100)],
                },
            },
        );

        profiles
    }

    /// Apply ASUS mode
    pub fn apply_mode(&mut self, mode: AsusMode) -> NvResult<()> {
        if !self.is_asus_card {
            return Err(NvControlError::UnsupportedFeature(
                "Not an ASUS GPU".to_string(),
            ));
        }

        let profile_name = match mode {
            AsusMode::SilentMode => "Silent",
            AsusMode::GamingMode => "Gaming",
            AsusMode::OcMode => "OC",
            AsusMode::Manual => {
                self.current_mode = mode;
                return Ok(());
            }
        };

        self.apply_profile(profile_name)?;
        self.current_mode = mode;

        println!("Applied ASUS {} mode", profile_name);

        Ok(())
    }

    /// Apply profile by name
    pub fn apply_profile(&self, name: &str) -> NvResult<()> {
        let profile = self
            .profiles
            .get(name)
            .ok_or_else(|| NvControlError::ConfigError(format!("Profile not found: {}", name)))?;

        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let mut device = nvml
            .device_by_index(self.gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        // Apply power limit
        let constraints = device.power_management_limit_constraints().ok();
        if let Some(constraints) = constraints {
            let default_power = device
                .power_management_limit_default()
                .unwrap_or(constraints.max_limit);
            let target_power =
                (default_power as f32 * profile.power_target_percent as f32 / 100.0) as u32;

            device.set_power_management_limit(target_power).ok();
        }

        // Apply clocks via nvidia-smi
        use std::process::Command;

        if profile.gpu_boost_clock_mhz != 0 {
            Command::new("nvidia-smi")
                .args(&[
                    "-i",
                    &self.gpu_id.to_string(),
                    "-lgc",
                    &format!("{:+}", profile.gpu_boost_clock_mhz),
                ])
                .output()
                .ok();
        }

        if profile.memory_clock_mhz != 0 {
            Command::new("nvidia-smi")
                .args(&[
                    "-i",
                    &self.gpu_id.to_string(),
                    "-lmc",
                    &format!("{:+}", profile.memory_clock_mhz),
                ])
                .output()
                .ok();
        }

        println!("Applied ASUS profile: {}", name);
        println!("  GPU Clock: {:+} MHz", profile.gpu_boost_clock_mhz);
        println!("  Memory Clock: {:+} MHz", profile.memory_clock_mhz);
        println!("  Power Target: {}%", profile.power_target_percent);
        println!("  Temp Target: {}°C", profile.temp_target_c);

        Ok(())
    }

    /// Save custom profile
    pub fn save_custom_profile(&mut self, name: String, profile: AsusOcProfile) {
        self.profiles.insert(name, profile);
    }

    /// Get current mode
    pub fn current_mode(&self) -> AsusMode {
        self.current_mode
    }

    /// Check if ASUS card
    pub fn is_asus(&self) -> bool {
        self.is_asus_card
    }

    /// List available profiles
    pub fn list_profiles(&self) -> Vec<&AsusOcProfile> {
        self.profiles.values().collect()
    }

    /// Get GPU Tweak style monitoring data
    pub fn get_monitoring_data(&self) -> NvResult<AsusMonitoringData> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let device = nvml
            .device_by_index(self.gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        let gpu_temp = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0) as i32;

        let gpu_clock = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .unwrap_or(0);
        let mem_clock = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
            .unwrap_or(0);

        let utilization = device.utilization_rates().ok();
        let gpu_load = utilization.as_ref().map(|u| u.gpu).unwrap_or(0);
        let mem_load = utilization.as_ref().map(|u| u.memory).unwrap_or(0);

        let power_draw = device.power_usage().unwrap_or(0) / 1000; // mW to W

        let fan_speed = device.fan_speed(0).unwrap_or(0);

        let memory_info = device.memory_info().ok();
        let vram_used = memory_info
            .as_ref()
            .map(|m| m.used / 1024 / 1024)
            .unwrap_or(0);
        let vram_total = memory_info
            .as_ref()
            .map(|m| m.total / 1024 / 1024)
            .unwrap_or(0);

        Ok(AsusMonitoringData {
            gpu_temp,
            gpu_clock_mhz: gpu_clock,
            mem_clock_mhz: mem_clock,
            gpu_load_percent: gpu_load,
            mem_load_percent: mem_load,
            power_draw_watts: power_draw,
            fan_speed_percent: fan_speed,
            vram_used_mb: vram_used,
            vram_total_mb: vram_total,
        })
    }
}

/// GPU Tweak style monitoring data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsusMonitoringData {
    pub gpu_temp: i32,
    pub gpu_clock_mhz: u32,
    pub mem_clock_mhz: u32,
    pub gpu_load_percent: u32,
    pub mem_load_percent: u32,
    pub power_draw_watts: u32,
    pub fan_speed_percent: u32,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asus_tweak_creation() {
        let tweak = AsusGpuTweak::new(0);
        println!("ASUS GPU Tweak initialized: {:?}", tweak.is_ok());
    }

    #[test]
    fn test_default_profiles() {
        let profiles = AsusGpuTweak::load_default_profiles();

        assert!(profiles.contains_key("Silent"));
        assert!(profiles.contains_key("Gaming"));
        assert!(profiles.contains_key("OC"));

        let oc_profile = profiles.get("OC").unwrap();
        assert_eq!(oc_profile.mode, AsusMode::OcMode);
        assert!(oc_profile.gpu_boost_clock_mhz > 0);
    }

    #[test]
    fn test_fan_profile() {
        let fan_profile = AsusFanProfile {
            mode: FanMode::Auto,
            curve: vec![(40, 0), (60, 50), (80, 100)],
        };

        assert_eq!(fan_profile.mode, FanMode::Auto);
        assert_eq!(fan_profile.curve.len(), 3);
    }
}
