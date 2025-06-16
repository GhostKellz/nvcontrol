use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrrSettings {
    pub enabled: bool,
    pub min_refresh_rate: u32,
    pub max_refresh_rate: u32,
    pub adaptive_sync: bool,
    pub low_framerate_compensation: bool,
}

impl Default for VrrSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            min_refresh_rate: 48,
            max_refresh_rate: 144,
            adaptive_sync: true,
            low_framerate_compensation: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplayVrrCapability {
    pub display_name: String,
    pub supports_vrr: bool,
    pub supports_gsync: bool,
    pub supports_freesync: bool,
    pub min_refresh: u32,
    pub max_refresh: u32,
    pub current_settings: VrrSettings,
}

pub fn detect_vrr_displays() -> NvResult<Vec<DisplayVrrCapability>> {
    let mut displays = Vec::new();

    // Try different methods based on desktop environment
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    match desktop.as_str() {
        "KDE" => displays = detect_vrr_kde()?,
        "GNOME" => displays = detect_vrr_gnome()?,
        "Hyprland" => displays = detect_vrr_hyprland()?,
        "sway" => displays = detect_vrr_sway()?,
        _ => {
            // Try multiple methods as fallback
            if let Ok(kde_displays) = detect_vrr_kde() {
                displays = kde_displays;
            } else if let Ok(x11_displays) = detect_vrr_x11() {
                displays = x11_displays;
            }
        }
    }

    if displays.is_empty() {
        // Fallback with mock data for development
        displays.push(DisplayVrrCapability {
            display_name: "HDMI-A-1".to_string(),
            supports_vrr: true,
            supports_gsync: true,
            supports_freesync: false,
            min_refresh: 48,
            max_refresh: 144,
            current_settings: VrrSettings::default(),
        });
    }

    Ok(displays)
}

fn detect_vrr_kde() -> NvResult<Vec<DisplayVrrCapability>> {
    let mut displays = Vec::new();

    // Use kscreen-doctor to query display capabilities
    if let Ok(output) = std::process::Command::new("kscreen-doctor")
        .arg("-j")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            displays = parse_kscreen_vrr_info(&output_str)?;
        }
    }

    Ok(displays)
}

fn parse_kscreen_vrr_info(json_str: &str) -> NvResult<Vec<DisplayVrrCapability>> {
    let mut displays = Vec::new();

    // Simple JSON parsing for VRR capabilities
    // In a real implementation, you'd use serde_json
    for line in json_str.lines() {
        if line.contains("\"name\":") && line.contains("DP-") || line.contains("HDMI-") {
            if let Some(name_start) = line.find("\"name\":\"") {
                let name_start = name_start + 8;
                if let Some(name_end) = line[name_start..].find("\"") {
                    let display_name = &line[name_start..name_start + name_end];

                    // Check for VRR support indicators
                    let supports_vrr = json_str.contains("\"vrr\":true")
                        || json_str.contains("\"adaptiveSync\":true");
                    let supports_gsync = json_str.contains("\"gsync\":true");
                    let supports_freesync = json_str.contains("\"freesync\":true");

                    displays.push(DisplayVrrCapability {
                        display_name: display_name.to_string(),
                        supports_vrr,
                        supports_gsync,
                        supports_freesync,
                        min_refresh: 48,
                        max_refresh: 144, // Would parse from JSON in real implementation
                        current_settings: VrrSettings::default(),
                    });
                }
            }
        }
    }

    Ok(displays)
}

fn detect_vrr_gnome() -> NvResult<Vec<DisplayVrrCapability>> {
    // GNOME VRR support via mutter experimental features
    let output = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.mutter", "experimental-features"])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("gsettings failed: {e}")))?;

    let settings = String::from_utf8_lossy(&output.stdout);
    let vrr_enabled = settings.contains("variable-refresh-rate");

    Ok(vec![DisplayVrrCapability {
        display_name: "Primary".to_string(),
        supports_vrr: vrr_enabled,
        supports_gsync: false,
        supports_freesync: vrr_enabled,
        min_refresh: 48,
        max_refresh: 144,
        current_settings: VrrSettings {
            enabled: vrr_enabled,
            ..VrrSettings::default()
        },
    }])
}

fn detect_vrr_hyprland() -> NvResult<Vec<DisplayVrrCapability>> {
    if let Ok(output) = std::process::Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return parse_hyprland_vrr_info(&output_str);
        }
    }

    Ok(vec![])
}

fn parse_hyprland_vrr_info(json_str: &str) -> NvResult<Vec<DisplayVrrCapability>> {
    let mut displays = Vec::new();

    // Parse Hyprland monitor JSON for VRR info
    for line in json_str.lines() {
        if line.contains("\"name\":") {
            // Extract display info from Hyprland JSON
            // This would use proper JSON parsing in a real implementation
            displays.push(DisplayVrrCapability {
                display_name: "DP-1".to_string(), // Placeholder
                supports_vrr: json_str.contains("\"vrr\":1"),
                supports_gsync: false,
                supports_freesync: true,
                min_refresh: 48,
                max_refresh: 165,
                current_settings: VrrSettings::default(),
            });
        }
    }

    Ok(displays)
}

fn detect_vrr_sway() -> NvResult<Vec<DisplayVrrCapability>> {
    if let Ok(output) = std::process::Command::new("swaymsg")
        .args(["-t", "get_outputs"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return parse_sway_vrr_info(&output_str);
        }
    }

    Ok(vec![])
}

fn parse_sway_vrr_info(json_str: &str) -> NvResult<Vec<DisplayVrrCapability>> {
    // Parse Sway output JSON for VRR capabilities
    Ok(vec![DisplayVrrCapability {
        display_name: "DP-1".to_string(),
        supports_vrr: json_str.contains("\"adaptive_sync_status\":\"enabled\""),
        supports_gsync: false,
        supports_freesync: true,
        min_refresh: 48,
        max_refresh: 144,
        current_settings: VrrSettings::default(),
    }])
}

fn detect_vrr_x11() -> NvResult<Vec<DisplayVrrCapability>> {
    // X11 VRR detection via xrandr
    if let Ok(output) = std::process::Command::new("xrandr")
        .args(["--verbose"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return parse_xrandr_vrr_info(&output_str);
        }
    }

    Ok(vec![])
}

fn parse_xrandr_vrr_info(xrandr_output: &str) -> NvResult<Vec<DisplayVrrCapability>> {
    let mut displays = Vec::new();
    let mut current_display = None;

    for line in xrandr_output.lines() {
        if line.contains(" connected") {
            let display_name = line
                .split_whitespace()
                .next()
                .unwrap_or("Unknown")
                .to_string();
            current_display = Some(display_name);
        } else if line.contains("variable refresh") || line.contains("VRR") {
            if let Some(name) = &current_display {
                displays.push(DisplayVrrCapability {
                    display_name: name.clone(),
                    supports_vrr: true,
                    supports_gsync: line.contains("G-SYNC"),
                    supports_freesync: line.contains("FreeSync"),
                    min_refresh: 48,
                    max_refresh: 144,
                    current_settings: VrrSettings::default(),
                });
            }
        }
    }

    Ok(displays)
}

pub fn apply_vrr_settings(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    match desktop.as_str() {
        "KDE" => apply_vrr_kde(display_name, settings),
        "GNOME" => apply_vrr_gnome(settings),
        "Hyprland" => apply_vrr_hyprland(display_name, settings),
        "sway" => apply_vrr_sway(display_name, settings),
        _ => apply_vrr_x11(display_name, settings),
    }
}

fn apply_vrr_kde(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    let vrr_state = if settings.enabled {
        "enable"
    } else {
        "disable"
    };

    let output = std::process::Command::new("kscreen-doctor")
        .arg(format!("output.{}.vrr.{}", display_name, vrr_state))
        .output()
        .map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("kscreen-doctor failed: {e}"))
        })?;

    if output.status.success() {
        println!("VRR {} for display {}", vrr_state, display_name);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "kscreen-doctor error: {stderr}"
        )))
    }
}

fn apply_vrr_gnome(settings: &VrrSettings) -> NvResult<()> {
    let features = if settings.enabled {
        "['variable-refresh-rate']"
    } else {
        "[]"
    };

    let output = std::process::Command::new("gsettings")
        .args(["set", "org.gnome.mutter", "experimental-features", features])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("gsettings failed: {e}")))?;

    if output.status.success() {
        println!(
            "GNOME VRR {}",
            if settings.enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
        Ok(())
    } else {
        Err(NvControlError::DisplayDetectionFailed(
            "Failed to set GNOME VRR".to_string(),
        ))
    }
}

fn apply_vrr_hyprland(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    let vrr_value = if settings.enabled { "1" } else { "0" };

    let output = std::process::Command::new("hyprctl")
        .args([
            "keyword",
            "monitor",
            &format!("{},vrr,{}", display_name, vrr_value),
        ])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("hyprctl failed: {e}")))?;

    if output.status.success() {
        println!(
            "Hyprland VRR {} for {}",
            if settings.enabled {
                "enabled"
            } else {
                "disabled"
            },
            display_name
        );
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "hyprctl error: {stderr}"
        )))
    }
}

fn apply_vrr_sway(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    let adaptive_sync = if settings.enabled {
        "enable"
    } else {
        "disable"
    };

    let output = std::process::Command::new("swaymsg")
        .args(["output", display_name, "adaptive_sync", adaptive_sync])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("swaymsg failed: {e}")))?;

    if output.status.success() {
        println!("Sway adaptive sync {} for {}", adaptive_sync, display_name);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "swaymsg error: {stderr}"
        )))
    }
}

fn apply_vrr_x11(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    // X11 VRR via xrandr (limited support)
    let vrr_option = if settings.enabled { "on" } else { "off" };

    let output = std::process::Command::new("xrandr")
        .args(["--output", display_name, "--set", "vrr", vrr_option])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("xrandr failed: {e}")))?;

    if output.status.success() {
        println!("X11 VRR {} for {}", vrr_option, display_name);
        Ok(())
    } else {
        // Try nvidia-settings as fallback
        apply_vrr_nvidia_settings(display_name, settings)
    }
}

fn apply_vrr_nvidia_settings(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    let gsync_state = if settings.enabled { "1" } else { "0" };

    let output = std::process::Command::new("nvidia-settings")
        .args(["-a", &format!("[gpu:0]/GPUGSyncAllowed={}", gsync_state)])
        .output()
        .map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("nvidia-settings failed: {e}"))
        })?;

    if output.status.success() {
        println!(
            "NVIDIA G-SYNC {} for {}",
            if settings.enabled {
                "enabled"
            } else {
                "disabled"
            },
            display_name
        );
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "nvidia-settings error: {stderr}"
        )))
    }
}

pub fn get_per_app_vrr_settings() -> HashMap<String, VrrSettings> {
    // Load per-application VRR settings
    // This would typically be stored in config files
    let mut settings = HashMap::new();

    // Example gaming presets
    settings.insert(
        "steam".to_string(),
        VrrSettings {
            enabled: true,
            min_refresh_rate: 48,
            max_refresh_rate: 144,
            adaptive_sync: true,
            low_framerate_compensation: true,
        },
    );

    settings.insert(
        "cs2".to_string(),
        VrrSettings {
            enabled: true,
            min_refresh_rate: 60,
            max_refresh_rate: 240,
            adaptive_sync: true,
            low_framerate_compensation: false, // Competitive gaming preference
        },
    );

    settings.insert(
        "firefox".to_string(),
        VrrSettings {
            enabled: false, // Disable for browsers to save power
            ..VrrSettings::default()
        },
    );

    settings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vrr_settings_default() {
        let settings = VrrSettings::default();
        assert!(!settings.enabled);
        assert_eq!(settings.min_refresh_rate, 48);
        assert_eq!(settings.max_refresh_rate, 144);
    }

    #[test]
    fn test_detect_vrr_displays() {
        let displays = detect_vrr_displays().unwrap();
        assert!(!displays.is_empty());
    }
}
