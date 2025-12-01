/// Phase 3.1: Enhanced Overclocking
///
/// Per-game overclock profiles, automatic stability testing, voltage curves, and memory timing
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Overclock profile for a specific game or application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverclockProfile {
    pub name: String,
    pub game_exe: Option<String>,
    pub gpu_offset_mhz: i32,
    pub memory_offset_mhz: i32,
    pub power_limit_watts: Option<u32>,
    pub voltage_curve: Option<VoltageCurve>,
    pub fan_curve: Option<FanCurve>,
    pub enabled: bool,
}

/// Voltage curve with frequency/voltage points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoltageCurve {
    pub points: Vec<VoltagePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoltagePoint {
    pub frequency_mhz: u32,
    pub voltage_mv: u32,
}

/// Custom fan curve for temperature-based fan control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurve {
    pub points: Vec<FanPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanPoint {
    pub temperature_c: i32,
    pub fan_speed_percent: u32,
}

/// Stability test result
#[derive(Debug, Clone)]
pub enum StabilityResult {
    Stable {
        duration_secs: u64,
        max_temp: i32,
    },
    Unstable {
        reason: String,
        failed_after_secs: u64,
    },
    Aborted {
        reason: String,
    },
}

/// Automatic stability tester
pub struct StabilityTester {
    gpu_id: u32,
    test_duration_secs: u64,
}

impl StabilityTester {
    pub fn new(gpu_id: u32, duration_secs: u64) -> Self {
        Self {
            gpu_id,
            test_duration_secs: duration_secs,
        }
    }

    /// Run stability test with given overclock settings
    pub fn test_stability(&self, gpu_offset: i32, memory_offset: i32) -> NvResult<StabilityResult> {
        use std::process::Command;
        use std::time::{Duration, Instant};

        println!("Starting stability test:");
        println!("  GPU Offset: +{} MHz", gpu_offset);
        println!("  Memory Offset: +{} MHz", memory_offset);
        println!("  Duration: {} seconds", self.test_duration_secs);

        let start = Instant::now();
        let mut max_temp = 0i32;

        // Apply overclock
        self.apply_test_overclock(gpu_offset, memory_offset)?;

        // Run GPU stress test (using CUDA or OpenGL workload)
        let stress_test = Command::new("gpu-burn")
            .arg("-d")
            .arg(&self.test_duration_secs.to_string())
            .spawn();

        if stress_test.is_err() {
            // Try alternative stress test
            let alt_stress = Command::new("glmark2").arg("--run-forever").spawn();

            if alt_stress.is_err() {
                return Err(NvControlError::RuntimeError(
                    "No GPU stress test tool available (install gpu-burn or glmark2)".to_string(),
                ));
            }
        }

        // Monitor GPU during test
        while start.elapsed() < Duration::from_secs(self.test_duration_secs) {
            match self.check_gpu_status() {
                Ok((temp, throttling)) => {
                    max_temp = max_temp.max(temp);

                    if throttling {
                        self.revert_overclock()?;
                        return Ok(StabilityResult::Unstable {
                            reason: "Thermal throttling detected".to_string(),
                            failed_after_secs: start.elapsed().as_secs(),
                        });
                    }

                    if temp > 90 {
                        self.revert_overclock()?;
                        return Ok(StabilityResult::Aborted {
                            reason: format!("Temperature too high: {}°C", temp),
                        });
                    }
                }
                Err(e) => {
                    self.revert_overclock()?;
                    return Ok(StabilityResult::Aborted {
                        reason: format!("GPU monitoring failed: {}", e),
                    });
                }
            }

            std::thread::sleep(Duration::from_secs(2));
        }

        // Test completed successfully
        self.revert_overclock()?;

        Ok(StabilityResult::Stable {
            duration_secs: self.test_duration_secs,
            max_temp,
        })
    }

    fn apply_test_overclock(&self, gpu_offset: i32, memory_offset: i32) -> NvResult<()> {
        use std::process::Command;

        let output = Command::new("nvidia-smi")
            .args(&[
                "-i",
                &self.gpu_id.to_string(),
                "-lgc",
                &format!("+{}", gpu_offset),
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("nvidia-smi failed: {}", e)))?;

        if !output.status.success() {
            return Err(NvControlError::RuntimeError(
                "Failed to apply GPU offset".to_string(),
            ));
        }

        let output = Command::new("nvidia-smi")
            .args(&[
                "-i",
                &self.gpu_id.to_string(),
                "-lmc",
                &format!("+{}", memory_offset),
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("nvidia-smi failed: {}", e)))?;

        if !output.status.success() {
            return Err(NvControlError::RuntimeError(
                "Failed to apply memory offset".to_string(),
            ));
        }

        Ok(())
    }

    fn revert_overclock(&self) -> NvResult<()> {
        use std::process::Command;

        let _ = Command::new("nvidia-smi")
            .args(&["-i", &self.gpu_id.to_string(), "-rgc"])
            .output();

        let _ = Command::new("nvidia-smi")
            .args(&["-i", &self.gpu_id.to_string(), "-rmc"])
            .output();

        Ok(())
    }

    fn check_gpu_status(&self) -> NvResult<(i32, bool)> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let device = nvml
            .device_by_index(self.gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        let temp = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Temp query failed: {}", e)))?;

        // Check for thermal throttling
        let perf_state = device.performance_state().ok();
        let throttling = perf_state.map(|p| p.as_c() > 2).unwrap_or(false);

        Ok((temp as i32, throttling))
    }

    /// Find maximum stable overclock automatically
    pub fn find_max_stable(&self) -> NvResult<(i32, i32)> {
        println!("Finding maximum stable overclock...");

        let mut gpu_offset = 0;
        let mut memory_offset = 0;

        // Binary search for max stable GPU offset
        let mut low = 0;
        let mut high = 500;

        while low < high {
            let mid = (low + high + 1) / 2;

            println!("\nTesting GPU offset: +{} MHz", mid);
            match self.test_stability(mid, 0)? {
                StabilityResult::Stable { .. } => {
                    gpu_offset = mid;
                    low = mid;
                }
                _ => {
                    high = mid - 1;
                }
            }
        }

        println!("\nMax stable GPU offset: +{} MHz", gpu_offset);

        // Binary search for max stable memory offset
        low = 0;
        high = 1500;

        while low < high {
            let mid = (low + high + 1) / 2;

            println!("\nTesting memory offset: +{} MHz", mid);
            match self.test_stability(gpu_offset, mid)? {
                StabilityResult::Stable { .. } => {
                    memory_offset = mid;
                    low = mid;
                }
                _ => {
                    high = mid - 1;
                }
            }
        }

        println!("\nMax stable memory offset: +{} MHz", memory_offset);

        Ok((gpu_offset, memory_offset))
    }
}

/// Profile manager for overclock profiles
pub struct OverclockProfileManager {
    profiles: HashMap<String, OverclockProfile>,
    config_path: PathBuf,
}

impl OverclockProfileManager {
    pub fn new() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nvcontrol")
            .join("overclock_profiles.json");

        Self {
            profiles: HashMap::new(),
            config_path,
        }
    }

    /// Load profiles from disk
    pub fn load(&mut self) -> NvResult<()> {
        if !self.config_path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.config_path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read profiles: {}", e)))?;

        self.profiles = serde_json::from_str(&content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse profiles: {}", e)))?;

        Ok(())
    }

    /// Save profiles to disk
    pub fn save(&self) -> NvResult<()> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                NvControlError::ConfigError(format!("Failed to create config dir: {}", e))
            })?;
        }

        let content = serde_json::to_string_pretty(&self.profiles).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize profiles: {}", e))
        })?;

        std::fs::write(&self.config_path, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write profiles: {}", e)))?;

        Ok(())
    }

    /// Add or update profile
    pub fn set_profile(&mut self, profile: OverclockProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    /// Get profile by name
    pub fn get_profile(&self, name: &str) -> Option<&OverclockProfile> {
        self.profiles.get(name)
    }

    /// Get profile by game executable
    pub fn get_profile_by_exe(&self, exe: &str) -> Option<&OverclockProfile> {
        self.profiles.values().find(|p| {
            if let Some(game_exe) = &p.game_exe {
                game_exe == exe || exe.ends_with(game_exe)
            } else {
                false
            }
        })
    }

    /// List all profiles
    pub fn list_profiles(&self) -> Vec<&OverclockProfile> {
        self.profiles.values().collect()
    }

    /// Remove profile
    pub fn remove_profile(&mut self, name: &str) -> Option<OverclockProfile> {
        self.profiles.remove(name)
    }

    /// Apply profile
    pub fn apply_profile(&self, name: &str, gpu_id: u32) -> NvResult<()> {
        let profile = self
            .get_profile(name)
            .ok_or_else(|| NvControlError::ConfigError(format!("Profile not found: {}", name)))?;

        if !profile.enabled {
            return Err(NvControlError::ConfigError(format!(
                "Profile '{}' is disabled",
                name
            )));
        }

        use crate::gpu_safe::SafeGpuController;

        let controller = SafeGpuController::new(gpu_id);

        controller.apply_overclock_safe(profile.gpu_offset_mhz, profile.memory_offset_mhz)?;

        if let Some(power_limit) = profile.power_limit_watts {
            controller.set_power_limit_safe(power_limit)?;
        }

        println!("Applied overclock profile: {}", profile.name);
        println!("  GPU Offset: +{} MHz", profile.gpu_offset_mhz);
        println!("  Memory Offset: +{} MHz", profile.memory_offset_mhz);

        Ok(())
    }
}

impl Default for OverclockProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let profile = OverclockProfile {
            name: "Cyberpunk 2077".to_string(),
            game_exe: Some("Cyberpunk2077.exe".to_string()),
            gpu_offset_mhz: 150,
            memory_offset_mhz: 500,
            power_limit_watts: Some(400),
            voltage_curve: None,
            fan_curve: None,
            enabled: true,
        };

        assert_eq!(profile.name, "Cyberpunk 2077");
        assert_eq!(profile.gpu_offset_mhz, 150);
    }

    #[test]
    fn test_profile_manager() {
        let mut manager = OverclockProfileManager::new();

        let profile = OverclockProfile {
            name: "Test Game".to_string(),
            game_exe: Some("test.exe".to_string()),
            gpu_offset_mhz: 100,
            memory_offset_mhz: 200,
            power_limit_watts: None,
            voltage_curve: None,
            fan_curve: None,
            enabled: true,
        };

        manager.set_profile(profile);

        assert!(manager.get_profile("Test Game").is_some());
        assert_eq!(
            manager.get_profile("Test Game").unwrap().gpu_offset_mhz,
            100
        );
    }

    #[test]
    fn test_profile_exe_matching() {
        let mut manager = OverclockProfileManager::new();

        let profile = OverclockProfile {
            name: "Game".to_string(),
            game_exe: Some("game.exe".to_string()),
            gpu_offset_mhz: 100,
            memory_offset_mhz: 200,
            power_limit_watts: None,
            voltage_curve: None,
            fan_curve: None,
            enabled: true,
        };

        manager.set_profile(profile);

        assert!(manager.get_profile_by_exe("/path/to/game.exe").is_some());
        assert!(manager.get_profile_by_exe("game.exe").is_some());
    }
}
