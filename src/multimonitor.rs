// Multi-Monitor Wayland Workflow Optimizer
// Per-display settings, layouts, and Gamescope integration

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
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

impl MultiMonitorManager {
    pub fn new() -> NvResult<Self> {
        let layouts_dir = Self::get_layouts_dir();
        fs::create_dir_all(&layouts_dir)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to create layouts dir: {}", e)))?;

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
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to run kscreen-doctor: {}", e)))?;

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

    /// Save current layout with a name
    pub fn save_layout(&mut self, name: &str) -> NvResult<()> {
        println!("💾 Saving display layout: {}", name);

        let layout = self.get_current_layout()?;
        let mut layout = layout;
        layout.name = name.to_string();

        let layout_path = self.layouts_dir.join(format!("{}.toml", name));
        let content = toml::to_string_pretty(&layout)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to serialize layout: {}", e)))?;

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
        println!("⚙️  Applying layout with {} displays", layout.displays.len());

        for display in &layout.displays {
            self.apply_display_config(display)?;
        }

        Ok(())
    }

    /// Apply configuration to a single display
    fn apply_display_config(&self, config: &DisplayConfig) -> NvResult<()> {
        println!("   Configuring: {}", config.name);

        let mut cmd_parts = Vec::new();

        // Resolution and refresh rate
        cmd_parts.push(format!(
            "output.{}.mode.{}x{}@{}",
            config.connector,
            config.resolution.0,
            config.resolution.1,
            config.refresh_rate
        ));

        // Position
        cmd_parts.push(format!(
            "output.{}.position.{},{}",
            config.connector,
            config.position.0,
            config.position.1
        ));

        // Scale
        if config.scale != 1.0 {
            cmd_parts.push(format!(
                "output.{}.scale.{}",
                config.connector,
                config.scale
            ));
        }

        // VRR
        cmd_parts.push(format!(
            "output.{}.vrrpolicy.{}",
            config.connector,
            if config.vrr_enabled { "automatic" } else { "never" }
        ));

        // Apply via kscreen-doctor
        for part in cmd_parts {
            Command::new("kscreen-doctor")
                .arg(&part)
                .status()
                .ok();
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
        println!("🖥️  Setting VRR for {}: {}", connector, if enabled { "enabled" } else { "disabled" });

        let status = Command::new("kscreen-doctor")
            .arg(format!(
                "output.{}.vrrpolicy.{}",
                connector,
                if enabled { "automatic" } else { "never" }
            ))
            .status()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to set VRR: {}", e)))?;

        if !status.success() {
            return Err(NvControlError::CommandFailed("kscreen-doctor failed".to_string()));
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
        cmd.spawn()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to launch gamescope: {}", e)))?;

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
            println!("      Resolution: {}x{}@{}Hz",
                display.resolution.0,
                display.resolution.1,
                display.refresh_rate
            );
            println!("      VRR: {}", if display.vrr_enabled { "✅" } else { "❌" });
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
