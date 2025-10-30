// Power Limit Curves
// Dynamic power limiting based on temperature, time, and workload

use crate::gui_widgets::CurvePoint;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerCurve {
    pub points: Vec<CurvePoint>,  // x = temperature (°C), y = power limit (%)
    pub selected_point: Option<usize>,
}

impl Default for PowerCurve {
    fn default() -> Self {
        Self {
            points: vec![
                CurvePoint::new(40.0, 100.0),  // Cool: 100% power
                CurvePoint::new(60.0, 90.0),   // Warm: 90% power
                CurvePoint::new(75.0, 80.0),   // Hot: 80% power
                CurvePoint::new(85.0, 70.0),   // Very hot: 70% power
            ],
            selected_point: None,
        }
    }
}

impl PowerCurve {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_point(&mut self, temp: f64, power: f64) {
        let point = CurvePoint::new(temp, power);

        let insert_pos = self.points.iter()
            .position(|p| p.x > temp)
            .unwrap_or(self.points.len());

        self.points.insert(insert_pos, point);
    }

    pub fn remove_point(&mut self, index: usize) {
        if self.points.len() > 2 {
            self.points.remove(index);
        }
    }

    pub fn update_point(&mut self, index: usize, temp: f64, power: f64) {
        if let Some(point) = self.points.get_mut(index) {
            point.x = temp.clamp(0.0, 100.0);
            point.y = power.clamp(50.0, 120.0);  // 50-120% of TDP
        }

        self.points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    }

    /// Get power limit at a given temperature
    pub fn get_power_at_temp(&self, temp: f64) -> f64 {
        if self.points.is_empty() {
            return 100.0;
        }

        if temp <= self.points[0].x {
            return self.points[0].y;
        }

        for i in 0..self.points.len() - 1 {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];

            if temp >= p1.x && temp <= p2.x {
                // Linear interpolation
                let t = (temp - p1.x) / (p2.x - p1.x);
                return p1.y + t * (p2.y - p1.y);
            }
        }

        self.points.last().unwrap().y
    }

    /// Convert to format for applying to GPU
    pub fn to_power_settings(&self) -> Vec<(u32, u32)> {
        self.points
            .iter()
            .map(|p| (p.x as u32, p.y as u32))
            .collect()
    }
}

/// Time-based power schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSchedule {
    pub enabled: bool,
    pub schedules: Vec<ScheduleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleEntry {
    pub start_hour: u8,      // 0-23
    pub end_hour: u8,        // 0-23
    pub power_limit: u32,    // Percentage of TDP
    pub days: Vec<Weekday>,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn from_chrono(wd: chrono::Weekday) -> Self {
        match wd {
            chrono::Weekday::Mon => Weekday::Monday,
            chrono::Weekday::Tue => Weekday::Tuesday,
            chrono::Weekday::Wed => Weekday::Wednesday,
            chrono::Weekday::Thu => Weekday::Thursday,
            chrono::Weekday::Fri => Weekday::Friday,
            chrono::Weekday::Sat => Weekday::Saturday,
            chrono::Weekday::Sun => Weekday::Sunday,
        }
    }
}

impl Default for PowerSchedule {
    fn default() -> Self {
        Self {
            enabled: false,
            schedules: vec![
                ScheduleEntry {
                    start_hour: 9,
                    end_hour: 17,
                    power_limit: 80,
                    days: vec![
                        Weekday::Monday,
                        Weekday::Tuesday,
                        Weekday::Wednesday,
                        Weekday::Thursday,
                        Weekday::Friday,
                    ],
                    name: "Work Hours (Lower Power)".to_string(),
                },
                ScheduleEntry {
                    start_hour: 18,
                    end_hour: 23,
                    power_limit: 100,
                    days: vec![
                        Weekday::Monday,
                        Weekday::Tuesday,
                        Weekday::Wednesday,
                        Weekday::Thursday,
                        Weekday::Friday,
                        Weekday::Saturday,
                        Weekday::Sunday,
                    ],
                    name: "Gaming Hours (Full Power)".to_string(),
                },
            ],
        }
    }
}

impl PowerSchedule {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get active power limit for current time
    pub fn get_active_power_limit(&self) -> Option<u32> {
        if !self.enabled {
            return None;
        }

        use chrono::Timelike;
        use chrono::Datelike;

        let now = chrono::Local::now();
        let current_hour = now.hour() as u8;
        let current_weekday = Weekday::from_chrono(now.weekday());

        for schedule in &self.schedules {
            if schedule.days.contains(&current_weekday) {
                if schedule.start_hour <= current_hour && current_hour < schedule.end_hour {
                    return Some(schedule.power_limit);
                }
            }
        }

        None
    }

    /// Add a new schedule entry
    pub fn add_schedule(&mut self, entry: ScheduleEntry) {
        self.schedules.push(entry);
    }

    /// Remove schedule by index
    pub fn remove_schedule(&mut self, index: usize) {
        if index < self.schedules.len() {
            self.schedules.remove(index);
        }
    }
}

/// Per-game power profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePowerProfiles {
    pub profiles: HashMap<String, u32>,  // game_executable -> power_limit (%)
}

impl Default for GamePowerProfiles {
    fn default() -> Self {
        let mut profiles = HashMap::new();

        // Common games with recommended power limits
        profiles.insert("cyberpunk2077.exe".to_string(), 100);
        profiles.insert("witcher3.exe".to_string(), 95);
        profiles.insert("minecraft.exe".to_string(), 70);
        profiles.insert("leagueoflegends.exe".to_string(), 80);
        profiles.insert("valorant.exe".to_string(), 85);

        Self { profiles }
    }
}

impl GamePowerProfiles {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get power limit for a game
    pub fn get_power_limit(&self, game_executable: &str) -> Option<u32> {
        self.profiles.get(game_executable).copied()
    }

    /// Set power limit for a game
    pub fn set_power_limit(&mut self, game_executable: String, power_limit: u32) {
        self.profiles.insert(game_executable, power_limit);
    }

    /// Remove game profile
    pub fn remove_profile(&mut self, game_executable: &str) {
        self.profiles.remove(game_executable);
    }
}

/// Master power management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerManagementConfig {
    pub curve_enabled: bool,
    pub schedule_enabled: bool,
    pub game_profiles_enabled: bool,
    pub power_curve: PowerCurve,
    pub schedule: PowerSchedule,
    pub game_profiles: GamePowerProfiles,
}

impl Default for PowerManagementConfig {
    fn default() -> Self {
        Self {
            curve_enabled: false,
            schedule_enabled: false,
            game_profiles_enabled: false,
            power_curve: PowerCurve::default(),
            schedule: PowerSchedule::default(),
            game_profiles: GamePowerProfiles::default(),
        }
    }
}

impl PowerManagementConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load config from file
    pub fn load() -> NvResult<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?
            .join("nvcontrol");

        let config_path = config_dir.join("power_management.toml");

        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)
                .map_err(|e| NvControlError::ConfigError(format!("Failed to read config: {}", e)))?;

            toml::from_str(&contents)
                .map_err(|e| NvControlError::ConfigError(format!("Failed to parse config: {}", e)))
        } else {
            Ok(Self::default())
        }
    }

    /// Save config to file
    pub fn save(&self) -> NvResult<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?
            .join("nvcontrol");

        std::fs::create_dir_all(&config_dir)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to create config dir: {}", e)))?;

        let config_path = config_dir.join("power_management.toml");

        let toml = toml::to_string_pretty(self)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(&config_path, toml)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    /// Get recommended power limit based on all enabled systems
    pub fn get_recommended_power_limit(
        &self,
        current_temp: f64,
        game_executable: Option<&str>,
    ) -> u32 {
        let mut limits = Vec::new();

        // Temperature-based curve
        if self.curve_enabled {
            limits.push(self.power_curve.get_power_at_temp(current_temp) as u32);
        }

        // Time-based schedule
        if self.schedule_enabled {
            if let Some(limit) = self.schedule.get_active_power_limit() {
                limits.push(limit);
            }
        }

        // Game-specific profile
        if self.game_profiles_enabled {
            if let Some(game) = game_executable {
                if let Some(limit) = self.game_profiles.get_power_limit(game) {
                    limits.push(limit);
                }
            }
        }

        // Use the most restrictive limit (lowest value)
        limits.into_iter().min().unwrap_or(100)
    }
}

/// Load power management configuration
pub fn load_power_config() -> NvResult<PowerManagementConfig> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?
        .join("nvcontrol");

    let config_path = config_dir.join("power_management.toml");

    if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read config: {}", e)))?;

        toml::from_str(&contents)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse config: {}", e)))
    } else {
        Ok(PowerManagementConfig::default())
    }
}

/// Save power management configuration
pub fn save_power_config(config: &PowerManagementConfig) -> NvResult<()> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("No config directory".into()))?
        .join("nvcontrol");

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to create config dir: {}", e)))?;

    let config_path = config_dir.join("power_management.toml");

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
    fn test_power_curve_interpolation() {
        let curve = PowerCurve::default();
        let power = curve.get_power_at_temp(70.0);
        assert!(power > 70.0 && power < 90.0);
    }

    #[test]
    fn test_schedule() {
        let schedule = PowerSchedule::default();
        // This test depends on current time, so just ensure it doesn't crash
        let _ = schedule.get_active_power_limit();
    }
}
