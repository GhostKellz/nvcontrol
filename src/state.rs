/// Unified State Management for nvcontrol
///
/// Persists user settings, profiles, and session data across application restarts
/// Saves to ~/.config/nvcontrol/state.json

use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Global application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub version: String,
    pub last_updated: std::time::SystemTime,
    pub settings: Settings,
    pub profiles: Vec<SavedProfile>,
    pub theme: ThemeState,
    pub session: SessionState,
}

/// User settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub selected_gpu: usize,
    pub update_interval_ms: u64,
    pub auto_apply_profiles: bool,
    pub enable_notifications: bool,
    pub safety_limits_enabled: bool,
    pub export_path: PathBuf,
}

/// Saved profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedProfile {
    pub name: String,
    pub gpu_id: u32,
    pub core_clock_offset: i32,
    pub memory_clock_offset: i32,
    pub power_limit: u32,
    pub fan_mode: String,
    pub fan_speed: u32,
    pub created_at: std::time::SystemTime,
}

/// Theme state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeState {
    pub current_theme: String,
    pub auto_detect: bool,
    pub custom_colors: Option<CustomThemeColors>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomThemeColors {
    pub accent: String,
    pub background: String,
    pub text: String,
}

/// Session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub total_runtime_seconds: u64,
    pub last_tab: usize,
    pub favorite_profiles: Vec<String>,
    pub recent_exports: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            last_updated: std::time::SystemTime::now(),
            settings: Settings::default(),
            profiles: Vec::new(),
            theme: ThemeState::default(),
            session: SessionState::default(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            selected_gpu: 0,
            update_interval_ms: 1000,
            auto_apply_profiles: true,
            enable_notifications: true,
            safety_limits_enabled: true,
            export_path: PathBuf::from("~/nvcontrol-exports"),
        }
    }
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            current_theme: "TokyoNightNight".to_string(),
            auto_detect: true,
            custom_colors: None,
        }
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            total_runtime_seconds: 0,
            last_tab: 0,
            favorite_profiles: Vec::new(),
            recent_exports: Vec::new(),
        }
    }
}

impl AppState {
    /// Get the state file path
    pub fn state_file_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nvcontrol");

        fs::create_dir_all(&config_dir).ok();
        config_dir.join("state.json")
    }

    /// Load state from disk
    pub fn load() -> NvResult<Self> {
        let path = Self::state_file_path();

        if !path.exists() {
            println!("📁 No state file found, using defaults");
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path).map_err(|e| {
            crate::NvControlError::ConfigError(format!("Failed to read state file: {}", e))
        })?;

        let state: AppState = serde_json::from_str(&contents).map_err(|e| {
            crate::NvControlError::ConfigError(format!("Failed to parse state file: {}", e))
        })?;

        println!("✅ Loaded state from {:?}", path);
        Ok(state)
    }

    /// Save state to disk
    pub fn save(&self) -> NvResult<()> {
        let path = Self::state_file_path();

        let json = serde_json::to_string_pretty(self).map_err(|e| {
            crate::NvControlError::ConfigError(format!("Failed to serialize state: {}", e))
        })?;

        fs::write(&path, json).map_err(|e| {
            crate::NvControlError::ConfigError(format!("Failed to write state file: {}", e))
        })?;

        println!("💾 Saved state to {:?}", path);
        Ok(())
    }

    /// Add a profile
    pub fn add_profile(&mut self, profile: SavedProfile) {
        self.profiles.push(profile);
        self.last_updated = std::time::SystemTime::now();
    }

    /// Remove a profile by name
    pub fn remove_profile(&mut self, name: &str) -> bool {
        let initial_len = self.profiles.len();
        self.profiles.retain(|p| p.name != name);
        self.last_updated = std::time::SystemTime::now();
        self.profiles.len() < initial_len
    }

    /// Get profile by name
    pub fn get_profile(&self, name: &str) -> Option<&SavedProfile> {
        self.profiles.iter().find(|p| p.name == name)
    }

    /// Update session runtime
    pub fn add_runtime(&mut self, seconds: u64) {
        self.session.total_runtime_seconds += seconds;
        self.last_updated = std::time::SystemTime::now();
    }

    /// Add to recent exports
    pub fn add_export(&mut self, path: String) {
        self.session.recent_exports.push(path);
        if self.session.recent_exports.len() > 10 {
            self.session.recent_exports.remove(0);
        }
        self.last_updated = std::time::SystemTime::now();
    }
}

/// State manager with auto-save
pub struct StateManager {
    state: AppState,
    auto_save: bool,
}

impl StateManager {
    /// Create new state manager
    pub fn new() -> NvResult<Self> {
        let state = AppState::load().unwrap_or_default();
        Ok(Self {
            state,
            auto_save: true,
        })
    }

    /// Get mutable reference to state
    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    /// Get reference to state
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Save state (manual or auto)
    pub fn save(&self) -> NvResult<()> {
        if self.auto_save {
            self.state.save()
        } else {
            Ok(())
        }
    }

    /// Enable/disable auto-save
    pub fn set_auto_save(&mut self, enabled: bool) {
        self.auto_save = enabled;
    }
}

impl Drop for StateManager {
    fn drop(&mut self) {
        if self.auto_save {
            let _ = self.state.save();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = AppState::default();
        assert_eq!(state.settings.selected_gpu, 0);
        assert!(state.settings.safety_limits_enabled);
    }

    #[test]
    fn test_add_remove_profile() {
        let mut state = AppState::default();

        let profile = SavedProfile {
            name: "Test".to_string(),
            gpu_id: 0,
            core_clock_offset: 100,
            memory_clock_offset: 200,
            power_limit: 110,
            fan_mode: "Auto".to_string(),
            fan_speed: 50,
            created_at: std::time::SystemTime::now(),
        };

        state.add_profile(profile);
        assert_eq!(state.profiles.len(), 1);

        assert!(state.remove_profile("Test"));
        assert_eq!(state.profiles.len(), 0);
    }

    #[test]
    fn test_state_persistence() {
        let state = AppState::default();

        // Save
        assert!(state.save().is_ok());

        // Load
        let loaded = AppState::load().unwrap();
        assert_eq!(loaded.version, state.version);
    }
}
