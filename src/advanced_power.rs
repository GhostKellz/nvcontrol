/// Phase 3.3: Power Optimization
///
/// Dynamic power management, per-application power profiles, battery boost, power analytics
use crate::nvml_backend::SharedNvmlBackend;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Power management mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PowerMode {
    MaxPerformance,
    Balanced,
    PowerSaver,
    Adaptive,
    Custom,
}

/// Power profile for specific application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerProfile {
    pub name: String,
    pub executable: Option<String>,
    pub power_limit_watts: u32,
    pub power_mode: PowerMode,
    pub clock_limit_mhz: Option<u32>,
    pub memory_clock_limit_mhz: Option<u32>,
    pub enabled: bool,
}

/// Dynamic power management controller
pub struct DynamicPowerManager {
    gpu_id: u32,
    backend: SharedNvmlBackend,
    current_mode: PowerMode,
    load_history: Vec<LoadSample>,
    idle_timeout: Duration,
    last_activity: Instant,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct LoadSample {
    timestamp: Instant,
    gpu_utilization: u32,
    memory_utilization: u32,
    power_draw: f32,
}

impl DynamicPowerManager {
    pub fn new(gpu_id: u32, backend: SharedNvmlBackend) -> Self {
        Self {
            gpu_id,
            backend,
            current_mode: PowerMode::Balanced,
            load_history: Vec::with_capacity(60), // 1 minute history
            idle_timeout: Duration::from_secs(30),
            last_activity: Instant::now(),
        }
    }

    /// Create with default real backend (legacy compatibility)
    pub fn new_with_real_backend(gpu_id: u32) -> Self {
        Self::new(gpu_id, crate::nvml_backend::create_real_backend())
    }

    /// Update with current GPU load
    pub fn update(&mut self, gpu_util: u32, mem_util: u32, power_draw: f32) {
        let sample = LoadSample {
            timestamp: Instant::now(),
            gpu_utilization: gpu_util,
            memory_utilization: mem_util,
            power_draw,
        };

        self.load_history.push(sample);

        // Keep only last 60 samples
        if self.load_history.len() > 60 {
            self.load_history.remove(0);
        }

        // Update activity timestamp
        if gpu_util > 10 {
            self.last_activity = Instant::now();
        }
    }

    /// Get recommended power mode based on workload
    pub fn recommend_mode(&self) -> PowerMode {
        if self.load_history.is_empty() {
            return PowerMode::Balanced;
        }

        // Check if idle
        if self.last_activity.elapsed() > self.idle_timeout {
            return PowerMode::PowerSaver;
        }

        // Calculate average load over recent history
        let recent_samples: Vec<_> = self.load_history.iter().rev().take(10).collect();

        let avg_util: f32 = recent_samples
            .iter()
            .map(|s| s.gpu_utilization as f32)
            .sum::<f32>()
            / recent_samples.len() as f32;

        let avg_power: f32 =
            recent_samples.iter().map(|s| s.power_draw).sum::<f32>() / recent_samples.len() as f32;

        // Classify workload
        if avg_util > 80.0 && avg_power > 200.0 {
            PowerMode::MaxPerformance
        } else if avg_util > 40.0 {
            PowerMode::Balanced
        } else if avg_util < 20.0 {
            PowerMode::PowerSaver
        } else {
            PowerMode::Balanced
        }
    }

    /// Apply power mode
    pub fn apply_mode(&mut self, mode: PowerMode) -> NvResult<()> {
        // Get power limit constraints via backend
        let (min_limit_mw, max_limit_mw) = self.backend.get_power_limit_constraints(self.gpu_id)?;

        let min_power = min_limit_mw / 1000; // mW to W
        let max_power = max_limit_mw / 1000;
        let default_power = self
            .backend
            .get_power_limit_default(self.gpu_id)
            .unwrap_or(max_limit_mw)
            / 1000;

        // Calculate target power based on mode
        let target_power = match mode {
            PowerMode::MaxPerformance => max_power,
            PowerMode::Balanced => default_power,
            PowerMode::PowerSaver => min_power + (default_power - min_power) / 2,
            PowerMode::Adaptive => {
                // Use recommended mode
                let recommended = self.recommend_mode();
                return self.apply_mode(recommended);
            }
            PowerMode::Custom => {
                // Keep current setting
                self.backend
                    .get_power_limit(self.gpu_id)
                    .unwrap_or(default_power * 1000)
                    / 1000
            }
        };

        // Apply power limit via backend
        self.backend
            .set_power_limit(self.gpu_id, target_power * 1000)?;

        self.current_mode = mode;

        println!("Applied power mode: {:?}", mode);
        println!("  Power Limit: {} W", target_power);

        Ok(())
    }

    pub fn current_mode(&self) -> PowerMode {
        self.current_mode
    }

    /// Get power statistics
    pub fn get_stats(&self) -> PowerStats {
        if self.load_history.is_empty() {
            return PowerStats::default();
        }

        let total_power: f32 = self.load_history.iter().map(|s| s.power_draw).sum();
        let avg_power = total_power / self.load_history.len() as f32;

        let max_power = self
            .load_history
            .iter()
            .map(|s| s.power_draw)
            .fold(0.0f32, |a, b| a.max(b));

        let min_power = self
            .load_history
            .iter()
            .map(|s| s.power_draw)
            .fold(f32::MAX, |a, b| a.min(b));

        PowerStats {
            avg_power_watts: avg_power,
            max_power_watts: max_power,
            min_power_watts: min_power,
            total_energy_wh: total_power / 3600.0, // Rough estimate
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PowerStats {
    pub avg_power_watts: f32,
    pub max_power_watts: f32,
    pub min_power_watts: f32,
    pub total_energy_wh: f32,
}

/// Battery boost for laptops
pub struct BatteryBoost {
    gpu_id: u32,
    backend: SharedNvmlBackend,
    target_fps: u32,
    enabled: bool,
}

impl BatteryBoost {
    pub fn new(gpu_id: u32, target_fps: u32, backend: SharedNvmlBackend) -> Self {
        Self {
            gpu_id,
            backend,
            target_fps,
            enabled: false,
        }
    }

    /// Create with default real backend (legacy compatibility)
    pub fn new_with_real_backend(gpu_id: u32, target_fps: u32) -> Self {
        Self::new(
            gpu_id,
            target_fps,
            crate::nvml_backend::create_real_backend(),
        )
    }

    /// Enable battery boost mode
    pub fn enable(&mut self) -> NvResult<()> {
        // Get power constraints via backend
        let (_, max_limit_mw) = self.backend.get_power_limit_constraints(self.gpu_id)?;

        // Set to 60% of max power for battery savings
        let battery_power = (max_limit_mw as f32 * 0.6) as u32;

        self.backend.set_power_limit(self.gpu_id, battery_power)?;

        self.enabled = true;

        println!("Battery Boost enabled");
        println!("  Target FPS: {}", self.target_fps);
        println!("  Power Limit: {} W", battery_power / 1000);

        Ok(())
    }

    /// Disable battery boost
    pub fn disable(&mut self) -> NvResult<()> {
        // Reset to default power limit
        let default_power = self.backend.get_power_limit_default(self.gpu_id)?;

        self.backend.set_power_limit(self.gpu_id, default_power)?;

        self.enabled = false;

        println!("Battery Boost disabled");

        Ok(())
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Per-application power profile manager
pub struct PowerProfileManager {
    profiles: HashMap<String, PowerProfile>,
    config_path: PathBuf,
}

impl PowerProfileManager {
    pub fn new() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nvcontrol")
            .join("power_profiles.json");

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
    pub fn set_profile(&mut self, profile: PowerProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    /// Get profile by name
    pub fn get_profile(&self, name: &str) -> Option<&PowerProfile> {
        self.profiles.get(name)
    }

    /// Get profile by executable
    pub fn get_profile_by_exe(&self, exe: &str) -> Option<&PowerProfile> {
        self.profiles.values().find(|p| {
            if let Some(profile_exe) = &p.executable {
                profile_exe == exe || exe.ends_with(profile_exe)
            } else {
                false
            }
        })
    }

    /// Apply profile using backend
    pub fn apply_profile_with_backend(
        &self,
        name: &str,
        gpu_id: u32,
        backend: &SharedNvmlBackend,
    ) -> NvResult<()> {
        let profile = self
            .get_profile(name)
            .ok_or_else(|| NvControlError::ConfigError(format!("Profile not found: {}", name)))?;

        if !profile.enabled {
            return Err(NvControlError::ConfigError(format!(
                "Profile '{}' is disabled",
                name
            )));
        }

        // Apply power limit via backend
        backend.set_power_limit(gpu_id, profile.power_limit_watts * 1000)?;

        println!("Applied power profile: {}", profile.name);
        println!("  Power Limit: {} W", profile.power_limit_watts);
        println!("  Mode: {:?}", profile.power_mode);

        Ok(())
    }

    /// Apply profile (legacy - creates own backend)
    pub fn apply_profile(&self, name: &str, gpu_id: u32) -> NvResult<()> {
        let backend = crate::nvml_backend::create_real_backend();
        self.apply_profile_with_backend(name, gpu_id, &backend)
    }

    /// List all profiles
    pub fn list_profiles(&self) -> Vec<&PowerProfile> {
        self.profiles.values().collect()
    }

    /// Remove profile
    pub fn remove_profile(&mut self, name: &str) -> Option<PowerProfile> {
        self.profiles.remove(name)
    }
}

impl Default for PowerProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Power consumption analytics
pub struct PowerAnalytics {
    samples: Vec<PowerSample>,
}

#[derive(Debug, Clone)]
struct PowerSample {
    timestamp: Instant,
    power_watts: f32,
    energy_wh: f32,
}

impl PowerAnalytics {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    /// Record power sample
    pub fn record(&mut self, power_watts: f32) {
        let energy_wh = if let Some(last) = self.samples.last() {
            let duration_hours = last.timestamp.elapsed().as_secs_f32() / 3600.0;
            power_watts * duration_hours
        } else {
            0.0
        };

        self.samples.push(PowerSample {
            timestamp: Instant::now(),
            power_watts,
            energy_wh,
        });
    }

    /// Get total energy consumed
    pub fn total_energy_wh(&self) -> f32 {
        self.samples.iter().map(|s| s.energy_wh).sum()
    }

    /// Get average power
    pub fn avg_power_watts(&self) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }

        let total: f32 = self.samples.iter().map(|s| s.power_watts).sum();
        total / self.samples.len() as f32
    }

    /// Estimate cost (assumes $0.12/kWh)
    pub fn estimated_cost_usd(&self, rate_per_kwh: f32) -> f32 {
        let kwh = self.total_energy_wh() / 1000.0;
        kwh * rate_per_kwh
    }
}

impl Default for PowerAnalytics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nvml_backend::MockNvmlBackend;
    use std::sync::Arc;

    #[test]
    fn test_power_mode() {
        let backend = Arc::new(MockNvmlBackend::single_gpu());
        let mut manager = DynamicPowerManager::new(0, backend);

        // Simulate high load
        for _ in 0..10 {
            manager.update(90, 80, 250.0);
        }

        assert_eq!(manager.recommend_mode(), PowerMode::MaxPerformance);
    }

    #[test]
    fn test_idle_detection() {
        let backend = Arc::new(MockNvmlBackend::single_gpu());
        let mut manager = DynamicPowerManager::new(0, backend);
        manager.idle_timeout = Duration::from_millis(100);

        // Simulate idle
        manager.update(5, 0, 20.0);
        std::thread::sleep(Duration::from_millis(150));

        assert_eq!(manager.recommend_mode(), PowerMode::PowerSaver);
    }

    #[test]
    fn test_power_profile() {
        let profile = PowerProfile {
            name: "Gaming".to_string(),
            executable: Some("game.exe".to_string()),
            power_limit_watts: 350,
            power_mode: PowerMode::MaxPerformance,
            clock_limit_mhz: None,
            memory_clock_limit_mhz: None,
            enabled: true,
        };

        assert_eq!(profile.power_limit_watts, 350);
    }

    #[test]
    fn test_power_analytics() {
        let mut analytics = PowerAnalytics::new();

        analytics.record(200.0);
        analytics.record(250.0);
        analytics.record(220.0);

        let avg = analytics.avg_power_watts();
        assert!((avg - 223.33).abs() < 1.0);
    }

    #[test]
    fn test_battery_boost() {
        let backend = Arc::new(MockNvmlBackend::single_gpu());
        let boost = BatteryBoost::new(0, 60, backend);
        assert!(!boost.is_enabled());
        assert_eq!(boost.target_fps, 60);
    }
}
