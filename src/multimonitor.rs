// Multi-Monitor Wayland Workflow Optimizer
// Per-display settings, layouts, and Gamescope integration

use crate::monitor_profiles::{self, DisplayType, MonitorProfile, MultiMonitorLayout};
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub name: String,
    pub connector: String,
    pub enabled: bool,
    pub resolution: (u32, u32),
    pub refresh_rate: u32,
    pub position: (i32, i32),
    pub scale: f32,
    pub rotation: Rotation,
    pub vrr_enabled: bool,
    pub digital_vibrance: i16,
    pub color_profile: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Rotation {
    Normal,
    Left,
    Right,
    Inverted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayLayout {
    pub name: String,
    pub displays: Vec<DisplayConfig>,
}

pub struct MultiMonitorManager {
    layouts_dir: PathBuf,
    current_layout: Option<DisplayLayout>,
}

#[derive(Debug, Clone)]
pub struct PresetSuggestion {
    pub preset_key: String,
    pub reason: String,
}

impl MultiMonitorManager {
    pub fn new() -> NvResult<Self> {
        let layouts_dir = Self::get_layouts_dir();
        fs::create_dir_all(&layouts_dir).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create layouts dir: {}", e))
        })?;

        Ok(Self {
            layouts_dir,
            current_layout: None,
        })
    }

    fn get_layouts_dir() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("nvcontrol/display_layouts")
        } else {
            PathBuf::from("display_layouts")
        }
    }

    /// Detect connected displays via KDE/Wayland
    pub fn detect_displays() -> NvResult<Vec<DisplayConfig>> {
        let output = Command::new("kscreen-doctor")
            .arg("-o")
            .output()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("Failed to run kscreen-doctor: {}", e))
            })?;

        let info = String::from_utf8_lossy(&output.stdout);
        let mut displays = Vec::new();

        // Parse kscreen-doctor output
        for line in info.lines() {
            if line.starts_with("Output:") {
                // Example: "Output: 1 DP-1 enabled connected..."
                if let Some(display) = Self::parse_display_line(&info, line) {
                    displays.push(display);
                }
            }
        }

        Ok(displays)
    }

    fn parse_display_line(_full_output: &str, line: &str) -> Option<DisplayConfig> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let connector = parts[2].to_string();
        let enabled = line.contains("enabled");

        // Extract more details from subsequent lines
        // This is simplified - real implementation would parse the full output
        Some(DisplayConfig {
            name: connector.clone(),
            connector,
            enabled,
            resolution: (1920, 1080), // Would parse from output
            refresh_rate: 60,
            position: (0, 0),
            scale: 1.0,
            rotation: Rotation::Normal,
            vrr_enabled: false,
            digital_vibrance: 0,
            color_profile: None,
        })
    }

    /// Get current display layout
    pub fn get_current_layout(&self) -> NvResult<DisplayLayout> {
        let displays = Self::detect_displays()?;

        Ok(DisplayLayout {
            name: "current".to_string(),
            displays,
        })
    }

    pub fn list_builtin_presets(&self) -> Vec<DisplayLayout> {
        let mut presets: Vec<DisplayLayout> = monitor_profiles::get_preset_layouts()
            .into_iter()
            .map(|(key, layout)| Self::preset_to_display_layout(&key, &layout))
            .collect();
        presets.sort_by(|a, b| a.name.cmp(&b.name));
        presets
    }

    pub fn suggest_builtin_presets(&self) -> NvResult<Vec<PresetSuggestion>> {
        let suggestions = monitor_profiles::detect_and_suggest_layout()?;
        let presets = monitor_profiles::get_preset_layouts();
        let mut results = Self::normalize_preset_suggestions(&suggestions, &presets);

        if results.is_empty() {
            let display_count = Self::detect_displays()
                .map(|displays| displays.len())
                .unwrap_or(0);
            for (key, layout) in &presets {
                if layout.monitors.len() == display_count && display_count > 0 {
                    results.push(PresetSuggestion {
                        preset_key: key.clone(),
                        reason: format!("matches detected display count ({display_count})"),
                    });
                }
            }
        }

        results.sort_by(|a, b| a.preset_key.cmp(&b.preset_key));
        results.dedup_by(|a, b| a.preset_key == b.preset_key);
        Ok(results)
    }

    pub fn preview_builtin_preset(&self, preset_key: &str) -> NvResult<DisplayLayout> {
        let mut presets = monitor_profiles::get_preset_layouts();
        let layout = presets.remove(preset_key).ok_or_else(|| {
            NvControlError::ConfigError(format!("Unknown monitor preset: {preset_key}"))
        })?;

        Ok(Self::preset_to_display_layout(preset_key, &layout))
    }

    pub fn apply_builtin_preset(&mut self, preset_key: &str) -> NvResult<()> {
        let layout = self.preview_builtin_preset(preset_key)?;
        self.apply_layout(&layout)?;
        self.current_layout = Some(layout);
        Ok(())
    }

    /// Save current layout with a name
    pub fn save_layout(&mut self, name: &str) -> NvResult<()> {
        println!("💾 Saving display layout: {}", name);

        let layout = self.get_current_layout()?;
        let mut layout = layout;
        layout.name = name.to_string();

        let layout_path = self.layouts_dir.join(format!("{}.toml", name));
        let content = toml::to_string_pretty(&layout).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize layout: {}", e))
        })?;

        fs::write(&layout_path, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write layout: {}", e)))?;

        println!("✅ Layout saved: {}", layout_path.display());
        Ok(())
    }

    /// Load and apply a saved layout
    pub fn load_layout(&mut self, name: &str) -> NvResult<()> {
        println!("📂 Loading display layout: {}", name);

        let layout_path = self.layouts_dir.join(format!("{}.toml", name));
        let content = fs::read_to_string(&layout_path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read layout: {}", e)))?;

        let layout: DisplayLayout = toml::from_str(&content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse layout: {}", e)))?;

        self.apply_layout(&layout)?;
        self.current_layout = Some(layout);

        println!("✅ Layout applied");
        Ok(())
    }

    /// Apply a display layout
    fn apply_layout(&self, layout: &DisplayLayout) -> NvResult<()> {
        println!(
            "⚙️  Applying layout with {} displays",
            layout.displays.len()
        );

        for display in &layout.displays {
            self.apply_display_config(display)?;
        }

        Ok(())
    }

    pub fn apply_live_layout(&mut self, layout: &DisplayLayout) -> NvResult<()> {
        self.apply_layout(layout)?;
        self.current_layout = Some(layout.clone());
        Ok(())
    }

    fn preset_to_display_layout(preset_key: &str, layout: &MultiMonitorLayout) -> DisplayLayout {
        DisplayLayout {
            name: preset_key.to_string(),
            displays: layout
                .monitors
                .iter()
                .map(Self::monitor_profile_to_display_config)
                .collect(),
        }
    }

    fn monitor_profile_to_display_config(profile: &MonitorProfile) -> DisplayConfig {
        DisplayConfig {
            name: profile.name.clone(),
            connector: profile.connector.clone(),
            enabled: true,
            resolution: (profile.resolution.width, profile.resolution.height),
            refresh_rate: profile.refresh_rate,
            position: (profile.position.x, profile.position.y),
            scale: 1.0,
            rotation: Rotation::Normal,
            vrr_enabled: profile.refresh_rate >= 120,
            digital_vibrance: Self::preset_vibrance_to_display_value(
                profile.display_type.clone(),
                profile.vibrance_settings.vibrance,
            ),
            color_profile: Some(Self::preset_color_profile_name(profile)),
        }
    }

    fn preset_color_profile_name(profile: &MonitorProfile) -> String {
        let display_type = match &profile.display_type {
            DisplayType::OLED => "oled",
            DisplayType::IPS => "ips",
            DisplayType::TN => "tn",
            DisplayType::VA => "va",
            DisplayType::MiniLED => "miniled",
            DisplayType::Unknown => "generic",
        };

        let hdr = if profile.hdr_enabled { "hdr" } else { "sdr" };
        format!("{display_type}-{hdr}")
    }

    fn preset_vibrance_to_display_value(display_type: DisplayType, vibrance: i32) -> i16 {
        let baseline = match display_type {
            DisplayType::OLED => 300,
            DisplayType::MiniLED => 350,
            DisplayType::IPS => 500,
            DisplayType::VA => 450,
            DisplayType::TN => 550,
            DisplayType::Unknown => 500,
        };

        let adjusted = vibrance - baseline;
        adjusted.clamp(-200, 200) as i16
    }

    fn normalize_preset_suggestions(
        suggestions: &[String],
        presets: &HashMap<String, MultiMonitorLayout>,
    ) -> Vec<PresetSuggestion> {
        let mut results = Vec::new();

        for suggestion in suggestions {
            for key in suggestion
                .split(" or ")
                .map(str::trim)
                .filter(|value| presets.contains_key(*value))
            {
                results.push(PresetSuggestion {
                    preset_key: key.to_string(),
                    reason: suggestion.clone(),
                });
            }
        }

        results
    }

    /// Apply configuration to a single display
    fn apply_display_config(&self, config: &DisplayConfig) -> NvResult<()> {
        println!("   Configuring: {}", config.name);

        let mut cmd_parts = Vec::new();

        // Resolution and refresh rate
        cmd_parts.push(format!(
            "output.{}.mode.{}x{}@{}",
            config.connector, config.resolution.0, config.resolution.1, config.refresh_rate
        ));

        // Position
        cmd_parts.push(format!(
            "output.{}.position.{},{}",
            config.connector, config.position.0, config.position.1
        ));

        // Scale
        if config.scale != 1.0 {
            cmd_parts.push(format!(
                "output.{}.scale.{}",
                config.connector, config.scale
            ));
        }

        // VRR
        cmd_parts.push(format!(
            "output.{}.vrrpolicy.{}",
            config.connector,
            if config.vrr_enabled {
                "automatic"
            } else {
                "never"
            }
        ));

        // Apply via kscreen-doctor
        for part in cmd_parts {
            Command::new("kscreen-doctor").arg(&part).status().ok();
        }

        // Apply digital vibrance if set
        if config.digital_vibrance != 0 {
            self.set_display_vibrance(&config.connector, config.digital_vibrance)?;
        }

        Ok(())
    }

    /// Set digital vibrance for a display
    fn set_display_vibrance(&self, connector: &str, value: i16) -> NvResult<()> {
        // Get display index from connector name
        let display_id = Self::connector_to_display_id(connector);

        Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!("[gpu:0]/DigitalVibrance[{}]={}", display_id, value),
            ])
            .status()
            .ok();

        Ok(())
    }

    fn connector_to_display_id(connector: &str) -> usize {
        // Simple mapping - would need to be more sophisticated
        if connector.starts_with("DP-1") || connector.starts_with("HDMI-0") {
            0
        } else if connector.starts_with("DP-2") || connector.starts_with("HDMI-1") {
            1
        } else {
            2
        }
    }

    /// Set VRR per display
    pub fn set_display_vrr(&self, connector: &str, enabled: bool) -> NvResult<()> {
        println!(
            "🖥️  Setting VRR for {}: {}",
            connector,
            if enabled { "enabled" } else { "disabled" }
        );

        let status = Command::new("kscreen-doctor")
            .arg(format!(
                "output.{}.vrrpolicy.{}",
                connector,
                if enabled { "automatic" } else { "never" }
            ))
            .status()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to set VRR: {}", e)))?;

        if !status.success() {
            return Err(NvControlError::CommandFailed(
                "kscreen-doctor failed".to_string(),
            ));
        }

        println!("✅ VRR updated");
        Ok(())
    }

    /// Launch Gamescope on specific display
    pub fn launch_gamescope_on_display(
        &self,
        connector: &str,
        width: u32,
        height: u32,
        refresh: u32,
        command: &str,
    ) -> NvResult<()> {
        println!("🎮 Launching Gamescope on {}", connector);
        println!("   Resolution: {}x{}@{}", width, height, refresh);
        println!("   Command: {}", command);

        // Build gamescope command
        let mut cmd = Command::new("gamescope");

        // Resolution
        cmd.args(&["-w", &width.to_string()]);
        cmd.args(&["-h", &height.to_string()]);
        cmd.args(&["-r", &refresh.to_string()]);

        // Fullscreen
        cmd.arg("-f");

        // Output to specific display (if supported)
        cmd.env("GAMESCOPE_OUTPUT", connector);

        // Application command
        cmd.arg("--");
        cmd.args(command.split_whitespace());

        // Launch
        cmd.spawn().map_err(|e| {
            NvControlError::CommandFailed(format!("Failed to launch gamescope: {}", e))
        })?;

        println!("✅ Gamescope launched");
        Ok(())
    }

    /// Detect docking station connection
    pub fn detect_docking_station(&self) -> NvResult<bool> {
        let displays = Self::detect_displays()?;

        // Simple heuristic: more than 2 displays = docked
        Ok(displays.len() > 2)
    }

    /// Auto-apply layout based on connected displays
    pub fn auto_apply_layout(&mut self) -> NvResult<()> {
        println!("🔍 Detecting display configuration...");

        let displays = Self::detect_displays()?;
        let display_count = displays.len();

        println!("   Detected {} display(s)", display_count);

        // Try to find matching saved layout
        let layouts = self.list_layouts();

        for layout_name in layouts {
            if let Ok(layout) = self.load_saved_layout(&layout_name) {
                if layout.displays.len() == display_count {
                    println!("   Found matching layout: {}", layout_name);
                    return self.load_layout(&layout_name);
                }
            }
        }

        println!("   No matching layout found");
        Ok(())
    }

    fn load_saved_layout(&self, name: &str) -> NvResult<DisplayLayout> {
        let layout_path = self.layouts_dir.join(format!("{}.toml", name));
        let content = fs::read_to_string(&layout_path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read layout: {}", e)))?;

        toml::from_str(&content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse layout: {}", e)))
    }

    /// List all saved layouts
    pub fn list_layouts(&self) -> Vec<String> {
        if !self.layouts_dir.exists() {
            return Vec::new();
        }

        fs::read_dir(&self.layouts_dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.extension()? == "toml" {
                            path.file_stem()?.to_str().map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Create common layouts
    pub fn create_example_layouts(&mut self) -> NvResult<()> {
        println!("📝 Creating example display layouts...\n");

        // Single display (laptop)
        let laptop_layout = DisplayLayout {
            name: "laptop".to_string(),
            displays: vec![DisplayConfig {
                name: "eDP-1".to_string(),
                connector: "eDP-1".to_string(),
                enabled: true,
                resolution: (1920, 1080),
                refresh_rate: 60,
                position: (0, 0),
                scale: 1.0,
                rotation: Rotation::Normal,
                vrr_enabled: false,
                digital_vibrance: 0,
                color_profile: None,
            }],
        };

        // Dual monitor (work setup)
        let dual_layout = DisplayLayout {
            name: "dual-work".to_string(),
            displays: vec![
                DisplayConfig {
                    name: "Primary".to_string(),
                    connector: "DP-1".to_string(),
                    enabled: true,
                    resolution: (2560, 1440),
                    refresh_rate: 144,
                    position: (0, 0),
                    scale: 1.0,
                    rotation: Rotation::Normal,
                    vrr_enabled: true,
                    digital_vibrance: 20,
                    color_profile: None,
                },
                DisplayConfig {
                    name: "Secondary".to_string(),
                    connector: "DP-2".to_string(),
                    enabled: true,
                    resolution: (1920, 1080),
                    refresh_rate: 60,
                    position: (2560, 0),
                    scale: 1.0,
                    rotation: Rotation::Normal,
                    vrr_enabled: false,
                    digital_vibrance: 0,
                    color_profile: None,
                },
            ],
        };

        // Gaming (single high refresh)
        let gaming_layout = DisplayLayout {
            name: "gaming".to_string(),
            displays: vec![DisplayConfig {
                name: "Gaming".to_string(),
                connector: "DP-1".to_string(),
                enabled: true,
                resolution: (2560, 1440),
                refresh_rate: 240,
                position: (0, 0),
                scale: 1.0,
                rotation: Rotation::Normal,
                vrr_enabled: true,
                digital_vibrance: 50,
                color_profile: None,
            }],
        };

        // Save layouts
        for layout in [laptop_layout, dual_layout, gaming_layout] {
            let path = self.layouts_dir.join(format!("{}.toml", layout.name));
            let content = toml::to_string_pretty(&layout)
                .map_err(|e| NvControlError::ConfigError(format!("Failed to serialize: {}", e)))?;
            fs::write(&path, content)
                .map_err(|e| NvControlError::ConfigError(format!("Failed to write: {}", e)))?;
            println!("✅ Created layout: {}", layout.name);
        }

        println!("\n✅ Example layouts created!");
        Ok(())
    }

    /// Print current display status
    pub fn print_status(&self) -> NvResult<()> {
        println!("🖥️  Multi-Monitor Display Status\n");

        let displays = Self::detect_displays()?;

        println!("Connected Displays: {}", displays.len());
        for (i, display) in displays.iter().enumerate() {
            println!("\n   Display {}:", i + 1);
            println!("      Connector: {}", display.connector);
            println!("      Enabled: {}", display.enabled);
            println!(
                "      Resolution: {}x{}@{}Hz",
                display.resolution.0, display.resolution.1, display.refresh_rate
            );
            println!(
                "      VRR: {}",
                if display.vrr_enabled { "✅" } else { "❌" }
            );
        }

        let layouts = self.list_layouts();
        if !layouts.is_empty() {
            println!("\nSaved Layouts:");
            for layout in layouts {
                println!("   • {}", layout);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monitor_profiles::{DisplayType, MonitorPosition, Resolution};
    use crate::vibrance::EnhancedVibranceSettings;

    #[test]
    fn converts_monitor_profile_to_display_config() {
        let profile = MonitorProfile {
            name: "Gaming".to_string(),
            connector: "DP-1".to_string(),
            display_type: DisplayType::IPS,
            resolution: Resolution {
                width: 2560,
                height: 1440,
            },
            refresh_rate: 240,
            is_primary: true,
            hdr_enabled: false,
            vibrance_settings: EnhancedVibranceSettings {
                vibrance: 700,
                saturation: 1.2,
                contrast: 1.1,
                brightness: 1.0,
                gamma: 2.2,
                hue_shift: 0.0,
                color_temperature: 6500,
                enabled: true,
            },
            position: MonitorPosition { x: 0, y: 0 },
        };

        let config = MultiMonitorManager::monitor_profile_to_display_config(&profile);
        assert_eq!(config.connector, "DP-1");
        assert_eq!(config.resolution, (2560, 1440));
        assert_eq!(config.refresh_rate, 240);
        assert!(config.vrr_enabled);
        assert_eq!(config.digital_vibrance, 200);
        assert_eq!(config.color_profile.as_deref(), Some("ips-sdr"));
    }

    #[test]
    fn converts_builtin_preset_to_layout() {
        let preset = monitor_profiles::get_preset_layouts()
            .remove("dual_oled_ips")
            .expect("preset exists");

        let layout = MultiMonitorManager::preset_to_display_layout("dual_oled_ips", &preset);
        assert_eq!(layout.name, "dual_oled_ips");
        assert_eq!(layout.displays.len(), 2);
        assert_eq!(layout.displays[0].connector, "DP-0");
        assert_eq!(layout.displays[1].position, (3840, 0));
    }

    #[test]
    fn normalizes_preset_suggestions() {
        let presets = monitor_profiles::get_preset_layouts();
        let suggestions = vec!["single_oled_4k or single_1440p".to_string()];

        let normalized = MultiMonitorManager::normalize_preset_suggestions(&suggestions, &presets);
        assert_eq!(normalized.len(), 2);
        assert_eq!(normalized[0].preset_key, "single_oled_4k");
        assert_eq!(normalized[1].preset_key, "single_1440p");
    }
}
