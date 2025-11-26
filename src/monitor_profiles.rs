use crate::vibrance::EnhancedVibranceSettings;
use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Per-monitor profiles optimized for different display types
/// Perfect for mixed setups like OLED + IPS, 4K + 1440p, etc.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorProfile {
    pub name: String,
    pub connector: String, // e.g., "DP-0", "HDMI-0"
    pub display_type: DisplayType,
    pub resolution: Resolution,
    pub refresh_rate: u32,
    pub is_primary: bool,
    pub hdr_enabled: bool,
    pub vibrance_settings: EnhancedVibranceSettings,
    pub position: MonitorPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisplayType {
    OLED,
    IPS,
    TN,
    VA,
    MiniLED,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiMonitorLayout {
    pub name: String,
    pub monitors: Vec<MonitorProfile>,
    pub auto_apply: bool,
}

impl MonitorProfile {
    /// Create an optimized profile for OLED displays
    pub fn oled_4k_preset(connector: &str, is_primary: bool) -> Self {
        Self {
            name: "OLED 4K".to_string(),
            connector: connector.to_string(),
            display_type: DisplayType::OLED,
            resolution: Resolution {
                width: 3840,
                height: 2160,
            },
            refresh_rate: 120,
            is_primary,
            hdr_enabled: true,
            vibrance_settings: EnhancedVibranceSettings {
                vibrance: 300,      // Lower for OLED (already saturated)
                saturation: 1.0,    // No boost needed
                contrast: 1.0,      // Perfect contrast on OLED
                brightness: 1.0,
                gamma: 2.2,
                hue_shift: 0.0,
                color_temperature: 6500, // Neutral
                enabled: true,
            },
            position: MonitorPosition { x: 0, y: 0 },
        }
    }

    /// Create an optimized profile for IPS 1440p displays
    pub fn ips_1440p_preset(connector: &str, is_primary: bool) -> Self {
        Self {
            name: "IPS 1440p".to_string(),
            connector: connector.to_string(),
            display_type: DisplayType::IPS,
            resolution: Resolution {
                width: 2560,
                height: 1440,
            },
            refresh_rate: 165,
            is_primary,
            hdr_enabled: false,
            vibrance_settings: EnhancedVibranceSettings {
                vibrance: 600,      // Boost for IPS
                saturation: 1.15,   // Slight boost
                contrast: 1.05,     // Slight boost
                brightness: 1.0,
                gamma: 2.2,
                hue_shift: 0.0,
                color_temperature: 6500,
                enabled: true,
            },
            position: MonitorPosition { x: 0, y: 0 },
        }
    }

    /// Create gaming-optimized profile
    pub fn gaming_preset(display_type: DisplayType, connector: &str) -> Self {
        let base_vibrance = match display_type {
            DisplayType::OLED => 400,
            DisplayType::IPS => 700,
            DisplayType::TN => 800,
            DisplayType::VA => 650,
            DisplayType::MiniLED => 500,
            DisplayType::Unknown => 600,
        };

        Self {
            name: "Gaming".to_string(),
            connector: connector.to_string(),
            display_type,
            resolution: Resolution {
                width: 2560,
                height: 1440,
            },
            refresh_rate: 240,
            is_primary: true,
            hdr_enabled: false,
            vibrance_settings: EnhancedVibranceSettings {
                vibrance: base_vibrance,
                saturation: 1.2,    // Extra pop for gaming
                contrast: 1.1,
                brightness: 1.0,
                gamma: 2.2,
                hue_shift: 0.0,
                color_temperature: 6500,
                enabled: true,
            },
            position: MonitorPosition { x: 0, y: 0 },
        }
    }

    /// Create content creation profile (accurate colors)
    pub fn content_creation_preset(display_type: DisplayType, connector: &str) -> Self {
        Self {
            name: "Content Creation".to_string(),
            connector: connector.to_string(),
            display_type,
            resolution: Resolution {
                width: 3840,
                height: 2160,
            },
            refresh_rate: 60,
            is_primary: true,
            hdr_enabled: false,
            vibrance_settings: EnhancedVibranceSettings {
                vibrance: 0,        // No boost, accurate colors
                saturation: 1.0,
                contrast: 1.0,
                brightness: 1.0,
                gamma: 2.2,         // sRGB standard
                hue_shift: 0.0,
                color_temperature: 6500, // D65 standard
                enabled: true,
            },
            position: MonitorPosition { x: 0, y: 0 },
        }
    }

    /// Create HDR gaming profile
    pub fn hdr_gaming_preset(connector: &str) -> Self {
        Self {
            name: "HDR Gaming".to_string(),
            connector: connector.to_string(),
            display_type: DisplayType::OLED,
            resolution: Resolution {
                width: 3840,
                height: 2160,
            },
            refresh_rate: 120,
            is_primary: true,
            hdr_enabled: true,
            vibrance_settings: EnhancedVibranceSettings {
                vibrance: 250,      // Lower for HDR
                saturation: 1.0,    // HDR handles saturation
                contrast: 1.0,
                brightness: 1.0,
                gamma: 2.4,         // HDR gamma
                hue_shift: 0.0,
                color_temperature: 6500,
                enabled: true,
            },
            position: MonitorPosition { x: 0, y: 0 },
        }
    }
}

impl MultiMonitorLayout {
    /// Create a dual monitor layout (OLED 4K + IPS 1440p)
    pub fn dual_oled_ips() -> Self {
        Self {
            name: "Dual OLED + IPS".to_string(),
            monitors: vec![
                MonitorProfile::oled_4k_preset("DP-0", true),
                {
                    let mut ips = MonitorProfile::ips_1440p_preset("DP-1", false);
                    ips.position = MonitorPosition { x: 3840, y: 0 }; // To the right
                    ips
                },
            ],
            auto_apply: true,
        }
    }

    /// Create a triple monitor layout
    pub fn triple_monitors() -> Self {
        Self {
            name: "Triple Monitor".to_string(),
            monitors: vec![
                MonitorProfile::ips_1440p_preset("DP-0", false),
                {
                    let mut center = MonitorProfile::oled_4k_preset("DP-1", true);
                    center.position = MonitorPosition { x: 2560, y: 0 }; // Center
                    center
                },
                {
                    let mut right = MonitorProfile::ips_1440p_preset("DP-2", false);
                    right.position = MonitorPosition { x: 6400, y: 0 }; // Right
                    right
                },
            ],
            auto_apply: true,
        }
    }

    /// Apply vibrance settings to all monitors in layout
    pub fn apply_vibrance(&self) -> NvResult<()> {
        println!("Applying multi-monitor vibrance layout: {}", self.name);

        for (idx, monitor) in self.monitors.iter().enumerate() {
            println!(
                "  Monitor {}: {} ({}x{} @ {}Hz) - Vibrance: {}",
                idx,
                monitor.name,
                monitor.resolution.width,
                monitor.resolution.height,
                monitor.refresh_rate,
                monitor.vibrance_settings.vibrance
            );

            // Apply settings via nvidia-settings
            // The actual implementation would use the connector name
            let _ = std::process::Command::new("nvidia-settings")
                .arg("-a")
                .arg(format!(
                    "[gpu:0]/DigitalVibrance[{}]={}",
                    monitor.connector, monitor.vibrance_settings.vibrance
                ))
                .output();
        }

        Ok(())
    }

    /// Print layout information
    pub fn print_layout(&self) {
        println!("\n=== Multi-Monitor Layout: {} ===", self.name);
        for (idx, monitor) in self.monitors.iter().enumerate() {
            println!("\nMonitor {}:", idx + 1);
            println!("  Name: {}", monitor.name);
            println!("  Connector: {}", monitor.connector);
            println!("  Type: {:?}", monitor.display_type);
            println!(
                "  Resolution: {}x{} @ {}Hz",
                monitor.resolution.width, monitor.resolution.height, monitor.refresh_rate
            );
            println!("  HDR: {}", if monitor.hdr_enabled { "Yes" } else { "No" });
            println!("  Primary: {}", if monitor.is_primary { "Yes" } else { "No" });
            println!(
                "  Position: ({}, {})",
                monitor.position.x, monitor.position.y
            );
            println!("\n  Vibrance Settings:");
            println!("    Vibrance: {}", monitor.vibrance_settings.vibrance);
            println!("    Saturation: {}", monitor.vibrance_settings.saturation);
            println!("    Contrast: {}", monitor.vibrance_settings.contrast);
            println!("    Brightness: {}", monitor.vibrance_settings.brightness);
            println!("    Gamma: {}", monitor.vibrance_settings.gamma);
            println!(
                "    Color Temp: {}K",
                monitor.vibrance_settings.color_temperature
            );
        }
    }
}

/// Get preset layouts
pub fn get_preset_layouts() -> HashMap<String, MultiMonitorLayout> {
    let mut layouts = HashMap::new();

    layouts.insert("dual_oled_ips".to_string(), MultiMonitorLayout::dual_oled_ips());
    layouts.insert(
        "triple_monitors".to_string(),
        MultiMonitorLayout::triple_monitors(),
    );

    // Single monitor layouts
    layouts.insert(
        "single_oled_4k".to_string(),
        MultiMonitorLayout {
            name: "Single OLED 4K".to_string(),
            monitors: vec![MonitorProfile::oled_4k_preset("DP-0", true)],
            auto_apply: true,
        },
    );

    layouts.insert(
        "single_1440p".to_string(),
        MultiMonitorLayout {
            name: "Single 1440p".to_string(),
            monitors: vec![MonitorProfile::ips_1440p_preset("DP-0", true)],
            auto_apply: true,
        },
    );

    layouts
}

/// Detect current monitor setup and suggest optimal profiles
pub fn detect_and_suggest_layout() -> NvResult<Vec<String>> {
    let mut suggestions = Vec::new();

    // Try to detect monitors via nvidia-settings
    if let Ok(output) = std::process::Command::new("nvidia-settings")
        .arg("-q")
        .arg("dpys")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let monitor_count = output_str.lines().filter(|l| l.contains("DFP-")).count();

            match monitor_count {
                1 => suggestions.push("single_oled_4k or single_1440p".to_string()),
                2 => suggestions.push("dual_oled_ips".to_string()),
                3 => suggestions.push("triple_monitors".to_string()),
                _ => suggestions.push(format!("{} monitors detected", monitor_count)),
            }
        }
    }

    if suggestions.is_empty() {
        suggestions.push("Unable to detect monitors".to_string());
    }

    Ok(suggestions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oled_preset() {
        let profile = MonitorProfile::oled_4k_preset("DP-0", true);
        assert_eq!(profile.display_type, DisplayType::OLED);
        assert_eq!(profile.resolution.width, 3840);
        assert_eq!(profile.resolution.height, 2160);
        assert!(profile.hdr_enabled);
    }

    #[test]
    fn test_dual_layout() {
        let layout = MultiMonitorLayout::dual_oled_ips();
        assert_eq!(layout.monitors.len(), 2);
        assert_eq!(layout.monitors[0].display_type, DisplayType::OLED);
        assert_eq!(layout.monitors[1].display_type, DisplayType::IPS);
    }

    #[test]
    fn test_preset_layouts() {
        let layouts = get_preset_layouts();
        assert!(layouts.contains_key("dual_oled_ips"));
        assert!(layouts.contains_key("triple_monitors"));
        assert!(layouts.contains_key("single_oled_4k"));
    }
}
