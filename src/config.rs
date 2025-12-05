use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Current state file version for migration tracking
const STATE_VERSION: u32 = 1;

/// TUI session state that persists across restarts
#[derive(Serialize, Deserialize, Clone)]
pub struct TuiSessionState {
    /// State file version for migration (introduced in v0.7.6)
    #[serde(default)]
    pub version: u32,
    /// Last selected GPU index
    #[serde(default)]
    pub selected_gpu: usize,
    /// Last active tab index
    #[serde(default)]
    pub current_tab: usize,
    /// Fan curve points: (temp, fan_percent)
    #[serde(default)]
    pub fan_curve_points: Vec<(u8, u8)>,
    /// GPU clock offset in MHz
    #[serde(default)]
    pub gpu_offset: i32,
    /// Memory clock offset in MHz
    #[serde(default)]
    pub memory_offset: i32,
    /// Power limit percentage (50-100)
    #[serde(default = "default_power_limit")]
    pub power_limit_percent: u8,
    /// OC preset name
    #[serde(default)]
    pub oc_preset: String,
}

fn default_power_limit() -> u8 {
    100
}

impl Default for TuiSessionState {
    fn default() -> Self {
        Self {
            version: 0,
            selected_gpu: 0,
            current_tab: 0,
            fan_curve_points: Vec::new(),
            gpu_offset: 0,
            memory_offset: 0,
            power_limit_percent: default_power_limit(),
            oc_preset: String::new(),
        }
    }
}

impl TuiSessionState {
    /// Load TUI session state from disk with migration support.
    ///
    /// Handles upgrade from v0.7.5 (no version field) to v0.7.6+ gracefully.
    /// Unknown fields are ignored, missing fields get defaults.
    pub fn load() -> Self {
        let path = Self::state_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => match toml::from_str::<TuiSessionState>(&content) {
                    Ok(mut state) => {
                        // Migrate if needed
                        state.migrate();
                        return state;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse TUI state: {e}");
                        // Try to backup corrupt file before returning default
                        Self::backup_corrupt_state(&path);
                    }
                },
                Err(e) => eprintln!("Failed to read TUI state: {e}"),
            }
        }
        Self::default()
    }

    /// Save TUI session state to disk with version stamp.
    pub fn save(&self) {
        let path = Self::state_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Always save with current version
        let mut state = self.clone();
        state.version = STATE_VERSION;

        match toml::to_string_pretty(&state) {
            Ok(content) => {
                if let Err(e) = fs::write(&path, content) {
                    eprintln!("Failed to save TUI state: {e}");
                }
            }
            Err(e) => eprintln!("Failed to serialize TUI state: {e}"),
        }
    }

    /// Migrate state from older versions to current version.
    fn migrate(&mut self) {
        // v0 (pre-0.7.6): No version field, serde default gives 0
        // v1 (0.7.6): Added version field, all fields have serde defaults
        //
        // Future migrations can be added here:
        // if self.version < 2 { ... migrate v1 -> v2 ... }

        // Validate and clamp values to safe ranges
        self.validate();

        // Mark as current version after migration
        self.version = STATE_VERSION;
    }

    /// Validate and clamp values to safe ranges.
    fn validate(&mut self) {
        // Power limit should be 50-150 (allow some headroom for OC)
        if self.power_limit_percent < 50 {
            self.power_limit_percent = 50;
        }
        if self.power_limit_percent > 150 {
            self.power_limit_percent = 100; // Reset to default if wildly out of range
        }

        // GPU offset typically -500 to +500 MHz
        if self.gpu_offset < -500 {
            self.gpu_offset = 0;
        }
        if self.gpu_offset > 500 {
            self.gpu_offset = 0;
        }

        // Memory offset typically -1000 to +2000 MHz
        if self.memory_offset < -1000 {
            self.memory_offset = 0;
        }
        if self.memory_offset > 2000 {
            self.memory_offset = 0;
        }

        // Tab index sanity (we have ~8 tabs)
        if self.current_tab > 10 {
            self.current_tab = 0;
        }

        // GPU index sanity (most systems have 1-4 GPUs)
        if self.selected_gpu > 16 {
            self.selected_gpu = 0;
        }

        // Fan curve validation
        for (temp, percent) in &mut self.fan_curve_points {
            if *temp > 100 {
                *temp = 100;
            }
            if *percent > 100 {
                *percent = 100;
            }
        }
    }

    /// Backup corrupt state file before overwriting with defaults.
    fn backup_corrupt_state(path: &PathBuf) {
        let backup_path = path.with_extension("toml.bak");
        if let Err(e) = fs::copy(path, &backup_path) {
            eprintln!("Failed to backup corrupt state file: {e}");
        } else {
            eprintln!("Backed up corrupt state to: {}", backup_path.display());
        }
    }

    fn state_path() -> PathBuf {
        if let Some(config_dir) = directories::ProjectDirs::from("com", "ghostkellz", "nvcontrol") {
            config_dir.config_dir().join("tui_state.toml")
        } else {
            PathBuf::from("nvcontrol_tui_state.toml")
        }
    }

    /// Check if state was migrated from an older version.
    pub fn was_migrated(&self) -> bool {
        self.version == 0
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub vibrance_levels: Vec<i16>,
    pub hdr_enabled: bool,
    pub selected_icc_profile: String,
    #[serde(default)]
    pub theme: String, // Theme name (e.g., "tokyo_night", "dracula")
    #[serde(default)]
    pub osd_enabled: bool,
    #[serde(default)]
    pub osd_position: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => eprintln!("Failed to parse config: {e}"),
                },
                Err(e) => eprintln!("Failed to read config: {e}"),
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        match toml::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(&config_path, content) {
                    eprintln!("Failed to save config: {e}");
                }
            }
            Err(e) => eprintln!("Failed to serialize config: {e}"),
        }
    }

    fn config_path() -> PathBuf {
        if let Some(config_dir) = directories::ProjectDirs::from("com", "ghostkellz", "nvcontrol") {
            config_dir.config_dir().join("config.toml")
        } else {
            PathBuf::from("nvcontrol_config.toml")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_state_default() {
        let state = TuiSessionState::default();
        assert_eq!(state.version, 0); // Default is 0, gets migrated to 1 on save
        assert_eq!(state.power_limit_percent, 100);
        assert_eq!(state.gpu_offset, 0);
        assert_eq!(state.memory_offset, 0);
    }

    #[test]
    fn test_tui_state_migration_from_v0() {
        // Simulate v0.7.5 state (no version field)
        let old_state_toml = r#"
selected_gpu = 1
current_tab = 2
gpu_offset = 100
memory_offset = 500
power_limit_percent = 90
oc_preset = "Performance"
"#;

        let mut state: TuiSessionState = toml::from_str(old_state_toml).unwrap();
        assert_eq!(state.version, 0); // No version field defaults to 0

        state.migrate();

        assert_eq!(state.version, STATE_VERSION);
        assert_eq!(state.selected_gpu, 1);
        assert_eq!(state.current_tab, 2);
        assert_eq!(state.gpu_offset, 100);
        assert_eq!(state.memory_offset, 500);
        assert_eq!(state.power_limit_percent, 90);
        assert_eq!(state.oc_preset, "Performance");
    }

    #[test]
    fn test_tui_state_validation_clamps_values() {
        let mut state = TuiSessionState {
            version: 0,
            selected_gpu: 100,                  // Too high
            current_tab: 50,                    // Too high
            fan_curve_points: vec![(150, 200)], // Both too high
            gpu_offset: 1000,                   // Too high
            memory_offset: -2000,               // Too low
            power_limit_percent: 200,           // Too high
            oc_preset: String::new(),
        };

        state.validate();

        assert_eq!(state.selected_gpu, 0);
        assert_eq!(state.current_tab, 0);
        assert_eq!(state.fan_curve_points, vec![(100, 100)]);
        assert_eq!(state.gpu_offset, 0);
        assert_eq!(state.memory_offset, 0);
        assert_eq!(state.power_limit_percent, 100);
    }

    #[test]
    fn test_tui_state_validation_preserves_valid_values() {
        let mut state = TuiSessionState {
            version: 1,
            selected_gpu: 2,
            current_tab: 5,
            fan_curve_points: vec![(40, 30), (60, 50), (80, 80)],
            gpu_offset: 150,
            memory_offset: 1000,
            power_limit_percent: 110,
            oc_preset: "MildOc".to_string(),
        };

        state.validate();

        // All values should be preserved
        assert_eq!(state.selected_gpu, 2);
        assert_eq!(state.current_tab, 5);
        assert_eq!(state.fan_curve_points, vec![(40, 30), (60, 50), (80, 80)]);
        assert_eq!(state.gpu_offset, 150);
        assert_eq!(state.memory_offset, 1000);
        assert_eq!(state.power_limit_percent, 110);
        assert_eq!(state.oc_preset, "MildOc");
    }

    #[test]
    fn test_tui_state_handles_unknown_fields() {
        // Future versions might add fields - old parser should ignore them
        let future_state_toml = r#"
version = 1
selected_gpu = 0
current_tab = 0
gpu_offset = 0
memory_offset = 0
power_limit_percent = 100
oc_preset = ""
some_future_field = "should be ignored"
another_new_field = 42
"#;

        // This should not panic, unknown fields should be ignored
        let result: Result<TuiSessionState, _> = toml::from_str(future_state_toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tui_state_serialization_roundtrip() {
        let state = TuiSessionState {
            version: STATE_VERSION,
            selected_gpu: 1,
            current_tab: 3,
            fan_curve_points: vec![(30, 20), (50, 40), (70, 60), (90, 100)],
            gpu_offset: 75,
            memory_offset: 200,
            power_limit_percent: 95,
            oc_preset: "Performance".to_string(),
        };

        let serialized = toml::to_string_pretty(&state).unwrap();
        let deserialized: TuiSessionState = toml::from_str(&serialized).unwrap();

        assert_eq!(state.version, deserialized.version);
        assert_eq!(state.selected_gpu, deserialized.selected_gpu);
        assert_eq!(state.current_tab, deserialized.current_tab);
        assert_eq!(state.fan_curve_points, deserialized.fan_curve_points);
        assert_eq!(state.gpu_offset, deserialized.gpu_offset);
        assert_eq!(state.memory_offset, deserialized.memory_offset);
        assert_eq!(state.power_limit_percent, deserialized.power_limit_percent);
        assert_eq!(state.oc_preset, deserialized.oc_preset);
    }

    #[test]
    fn test_power_limit_low_clamp() {
        let mut state = TuiSessionState {
            power_limit_percent: 30, // Below minimum
            ..Default::default()
        };
        state.validate();
        assert_eq!(state.power_limit_percent, 50);
    }
}
