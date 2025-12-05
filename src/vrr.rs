use crate::display_backend::SharedDisplayRunner;
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

/// Detect VRR displays (legacy - creates own runner internally)
pub fn detect_vrr_displays() -> NvResult<Vec<DisplayVrrCapability>> {
    let runner = crate::display_backend::create_real_runner();
    detect_vrr_displays_with_backend(&runner)
}

/// Detect VRR displays using the provided display backend
pub fn detect_vrr_displays_with_backend(
    runner: &SharedDisplayRunner,
) -> NvResult<Vec<DisplayVrrCapability>> {
    let mut displays = Vec::new();

    // Try different methods based on desktop environment
    let desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_uppercase();

    // Check for KDE session even if XDG_CURRENT_DESKTOP is empty
    let is_kde = desktop.contains("KDE")
        || desktop.contains("PLASMA")
        || std::env::var("KDE_SESSION_VERSION").is_ok()
        || runner.run_command("which", &["kscreen-doctor"]).is_ok();

    if is_kde {
        if let Ok(kde_displays) = detect_vrr_kde_with_backend(runner) {
            if !kde_displays.is_empty() {
                displays = kde_displays;
            }
        }
    } else if desktop.contains("GNOME") {
        displays = detect_vrr_gnome_with_backend(runner)?;
    } else if desktop.contains("HYPRLAND") {
        displays = detect_vrr_hyprland_with_backend(runner)?;
    } else if desktop.contains("SWAY") {
        displays = detect_vrr_sway_with_backend(runner)?;
    }

    // If still empty, try multiple methods as fallback
    if displays.is_empty() {
        if let Ok(kde_displays) = detect_vrr_kde_with_backend(runner) {
            if !kde_displays.is_empty() {
                displays = kde_displays;
            }
        }
    }
    if displays.is_empty() {
        if let Ok(x11_displays) = detect_vrr_x11_with_backend(runner) {
            displays = x11_displays;
        }
    }

    if displays.is_empty() {
        // Try to get display names from xrandr as fallback
        if let Ok(output) = runner.run_command("xrandr", &["--query"]) {
            for line in output.lines() {
                if line.contains(" connected") {
                    if let Some(name) = line.split_whitespace().next() {
                        displays.push(DisplayVrrCapability {
                            display_name: name.to_string(),
                            supports_vrr: true,
                            supports_gsync: name.starts_with("DP-"),
                            supports_freesync: true,
                            min_refresh: 48,
                            max_refresh: 144,
                            current_settings: VrrSettings::default(),
                        });
                    }
                }
            }
        }
    }

    Ok(displays)
}

fn detect_vrr_kde_with_backend(
    runner: &SharedDisplayRunner,
) -> NvResult<Vec<DisplayVrrCapability>> {
    // Use kscreen-doctor to query display capabilities
    if let Ok(output) = runner.run_command("kscreen-doctor", &["-j"]) {
        return parse_kscreen_vrr_info(&output);
    }

    Ok(vec![])
}

fn parse_kscreen_vrr_info(json_str: &str) -> NvResult<Vec<DisplayVrrCapability>> {
    let mut displays = Vec::new();

    // Parse kscreen-doctor JSON using serde_json
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(outputs) = json.get("outputs").and_then(|o| o.as_array()) {
            for output in outputs {
                // Get display name (e.g., "DP-2", "HDMI-1")
                let display_name = output
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("Unknown")
                    .to_string();

                // Skip disconnected displays
                let connected = output
                    .get("connected")
                    .and_then(|c| c.as_bool())
                    .unwrap_or(false);
                if !connected {
                    continue;
                }

                // vrrPolicy: 0 = Never, 1 = Always, 2 = Automatic
                let vrr_policy = output
                    .get("vrrPolicy")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let vrr_enabled = vrr_policy > 0;

                // Get max refresh rate from modes
                let mut max_refresh: u32 = 60;
                if let Some(modes) = output.get("modes").and_then(|m| m.as_array()) {
                    for mode in modes {
                        if let Some(rate) = mode.get("refreshRate").and_then(|r| r.as_f64()) {
                            max_refresh = max_refresh.max(rate as u32);
                        }
                    }
                }

                // G-Sync compatible displays support VRR via NVIDIA
                let supports_gsync = display_name.starts_with("DP-"); // DP typically supports G-Sync

                displays.push(DisplayVrrCapability {
                    display_name,
                    supports_vrr: true, // If connected via DP/HDMI 2.1, assume VRR capable
                    supports_gsync,
                    supports_freesync: true, // Most modern displays support FreeSync
                    min_refresh: 48,
                    max_refresh,
                    current_settings: VrrSettings {
                        enabled: vrr_enabled,
                        min_refresh_rate: 48,
                        max_refresh_rate: max_refresh,
                        adaptive_sync: vrr_enabled,
                        low_framerate_compensation: true,
                    },
                });
            }
        }
    }

    Ok(displays)
}

fn detect_vrr_gnome_with_backend(
    runner: &SharedDisplayRunner,
) -> NvResult<Vec<DisplayVrrCapability>> {
    // GNOME VRR support via mutter experimental features
    let settings = runner
        .run_command(
            "gsettings",
            &["get", "org.gnome.mutter", "experimental-features"],
        )
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("gsettings failed: {e}")))?;

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

fn detect_vrr_hyprland_with_backend(
    runner: &SharedDisplayRunner,
) -> NvResult<Vec<DisplayVrrCapability>> {
    if let Ok(output) = runner.run_command("hyprctl", &["monitors", "-j"]) {
        return parse_hyprland_vrr_info(&output);
    }

    Ok(vec![])
}

fn parse_hyprland_vrr_info(json_str: &str) -> NvResult<Vec<DisplayVrrCapability>> {
    let mut displays = Vec::new();

    // Parse Hyprland monitor JSON array
    // Format: [{"id":0,"name":"DP-1","width":2560,"height":1440,"refreshRate":165.0,"vrr":true,...}]
    if let Ok(monitors) = serde_json::from_str::<Vec<serde_json::Value>>(json_str) {
        for monitor in monitors {
            // Skip disabled monitors
            if monitor
                .get("disabled")
                .and_then(|d| d.as_bool())
                .unwrap_or(false)
            {
                continue;
            }

            let display_name = monitor
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown")
                .to_string();

            // VRR status - can be bool or int (0=off, 1=on, 2=fullscreen-only)
            let vrr_enabled = match monitor.get("vrr") {
                Some(serde_json::Value::Bool(b)) => *b,
                Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) > 0,
                _ => false,
            };

            // Get refresh rate
            let refresh_rate = monitor
                .get("refreshRate")
                .and_then(|r| r.as_f64())
                .unwrap_or(60.0) as u32;

            // Get available modes for max refresh
            let mut max_refresh = refresh_rate;
            if let Some(modes) = monitor.get("availableModes").and_then(|m| m.as_array()) {
                for mode in modes {
                    if let Some(mode_str) = mode.as_str() {
                        // Format: "2560x1440@165.00Hz"
                        if let Some(hz_part) = mode_str.split('@').nth(1) {
                            if let Ok(hz) = hz_part.trim_end_matches("Hz").parse::<f64>() {
                                max_refresh = max_refresh.max(hz as u32);
                            }
                        }
                    }
                }
            }

            // G-Sync typically on DisplayPort
            let supports_gsync = display_name.starts_with("DP-");

            displays.push(DisplayVrrCapability {
                display_name,
                supports_vrr: true, // If Hyprland reports it, assume capable
                supports_gsync,
                supports_freesync: true,
                min_refresh: 48,
                max_refresh,
                current_settings: VrrSettings {
                    enabled: vrr_enabled,
                    min_refresh_rate: 48,
                    max_refresh_rate: max_refresh,
                    adaptive_sync: vrr_enabled,
                    low_framerate_compensation: true,
                },
            });
        }
    }

    Ok(displays)
}

fn detect_vrr_sway_with_backend(
    runner: &SharedDisplayRunner,
) -> NvResult<Vec<DisplayVrrCapability>> {
    if let Ok(output) = runner.run_command("swaymsg", &["-t", "get_outputs"]) {
        return parse_sway_vrr_info(&output);
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

fn detect_vrr_x11_with_backend(
    runner: &SharedDisplayRunner,
) -> NvResult<Vec<DisplayVrrCapability>> {
    // X11 VRR detection via xrandr
    if let Ok(output) = runner.run_command("xrandr", &["--verbose"]) {
        return parse_xrandr_vrr_info(&output);
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

/// Apply VRR settings (legacy - creates own runner internally)
pub fn apply_vrr_settings(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    let runner = crate::display_backend::create_real_runner();
    apply_vrr_settings_with_backend(display_name, settings, &runner)
}

/// Apply VRR settings using the provided display backend
pub fn apply_vrr_settings_with_backend(
    display_name: &str,
    settings: &VrrSettings,
    runner: &SharedDisplayRunner,
) -> NvResult<()> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    match desktop.as_str() {
        "KDE" => apply_vrr_kde_with_backend(display_name, settings, runner),
        "GNOME" => apply_vrr_gnome_with_backend(settings, runner),
        "Hyprland" => apply_vrr_hyprland_with_backend(display_name, settings, runner),
        "sway" => apply_vrr_sway_with_backend(display_name, settings, runner),
        _ => apply_vrr_x11_with_backend(display_name, settings, runner),
    }
}

fn apply_vrr_kde_with_backend(
    display_name: &str,
    settings: &VrrSettings,
    runner: &SharedDisplayRunner,
) -> NvResult<()> {
    // VRR policy values: 0 = Never, 1 = Always, 2 = Automatic
    let vrr_policy = if settings.enabled {
        if settings.adaptive_sync { "2" } else { "1" }
    } else {
        "0"
    };

    // Try kscreen-doctor first with vrrpolicy parameter
    let arg = format!("output.{}.vrrpolicy.{}", display_name, vrr_policy);
    match runner.run_command("kscreen-doctor", &[&arg]) {
        Ok(_) => {
            let vrr_state = match vrr_policy {
                "0" => "disabled",
                "1" => "enabled (always)",
                "2" => "enabled (automatic)",
                _ => "unknown",
            };
            println!("VRR {} for display {}", vrr_state, display_name);
            Ok(())
        }
        Err(_) => {
            // If kscreen-doctor fails, try direct config file modification
            apply_vrr_kde_config(display_name, settings)
        }
    }
}

/// Fallback: directly modify kscreen config files
fn apply_vrr_kde_config(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    let home = std::env::var("HOME").unwrap_or_default();
    let control_dir = std::path::PathBuf::from(&home).join(".local/share/kscreen/control/outputs");

    if !control_dir.exists() {
        return Err(NvControlError::DisplayDetectionFailed(
            "kscreen control directory not found".to_string(),
        ));
    }

    // VRR policy: 0 = Never, 1 = Always, 2 = Automatic
    let vrr_policy = if settings.enabled {
        if settings.adaptive_sync { 2 } else { 1 }
    } else {
        0
    };

    // Find and update config files for this display
    for entry in std::fs::read_dir(&control_dir).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to read kscreen dir: {e}"))
    })? {
        let entry = entry.map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("Failed to read entry: {e}"))
        })?;

        let path = entry.path();
        if let Ok(contents) = std::fs::read_to_string(&path) {
            if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&contents) {
                // Check if this config is for our display
                if let Some(metadata) = json.get("metadata") {
                    if let Some(name) = metadata.get("name").and_then(|n| n.as_str()) {
                        if name == display_name {
                            // Update vrrpolicy
                            json["vrrpolicy"] = serde_json::json!(vrr_policy);

                            // Write back
                            let new_contents =
                                serde_json::to_string_pretty(&json).map_err(|e| {
                                    NvControlError::DisplayDetectionFailed(format!(
                                        "Failed to serialize: {e}"
                                    ))
                                })?;

                            std::fs::write(&path, new_contents).map_err(|e| {
                                NvControlError::DisplayDetectionFailed(format!(
                                    "Failed to write config: {e}"
                                ))
                            })?;

                            let vrr_state = match vrr_policy {
                                0 => "disabled",
                                1 => "enabled (always)",
                                2 => "enabled (automatic)",
                                _ => "unknown",
                            };
                            println!(
                                "VRR {} for display {} (config updated)",
                                vrr_state, display_name
                            );
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    Err(NvControlError::DisplayDetectionFailed(format!(
        "Could not find kscreen config for display {}",
        display_name
    )))
}

fn apply_vrr_gnome_with_backend(
    settings: &VrrSettings,
    runner: &SharedDisplayRunner,
) -> NvResult<()> {
    let features = if settings.enabled {
        "['variable-refresh-rate']"
    } else {
        "[]"
    };

    runner
        .run_command(
            "gsettings",
            &["set", "org.gnome.mutter", "experimental-features", features],
        )
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("gsettings failed: {e}")))?;

    println!(
        "GNOME VRR {}",
        if settings.enabled {
            "enabled"
        } else {
            "disabled"
        }
    );
    Ok(())
}

fn apply_vrr_hyprland_with_backend(
    display_name: &str,
    settings: &VrrSettings,
    runner: &SharedDisplayRunner,
) -> NvResult<()> {
    let vrr_value = if settings.enabled { "1" } else { "0" };
    let monitor_arg = format!("{},vrr,{}", display_name, vrr_value);

    runner
        .run_command("hyprctl", &["keyword", "monitor", &monitor_arg])
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("hyprctl failed: {e}")))?;

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
}

fn apply_vrr_sway_with_backend(
    display_name: &str,
    settings: &VrrSettings,
    runner: &SharedDisplayRunner,
) -> NvResult<()> {
    let adaptive_sync = if settings.enabled {
        "enable"
    } else {
        "disable"
    };

    runner
        .run_command(
            "swaymsg",
            &["output", display_name, "adaptive_sync", adaptive_sync],
        )
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("swaymsg failed: {e}")))?;

    println!("Sway adaptive sync {} for {}", adaptive_sync, display_name);
    Ok(())
}

fn apply_vrr_x11_with_backend(
    display_name: &str,
    settings: &VrrSettings,
    runner: &SharedDisplayRunner,
) -> NvResult<()> {
    // X11 VRR via xrandr (limited support)
    let vrr_option = if settings.enabled { "on" } else { "off" };

    match runner.run_command(
        "xrandr",
        &["--output", display_name, "--set", "vrr", vrr_option],
    ) {
        Ok(_) => {
            println!("X11 VRR {} for {}", vrr_option, display_name);
            Ok(())
        }
        Err(_) => {
            // Try nvidia-settings as fallback
            apply_vrr_nvidia_settings_with_backend(display_name, settings, runner)
        }
    }
}

fn apply_vrr_nvidia_settings_with_backend(
    display_name: &str,
    settings: &VrrSettings,
    runner: &SharedDisplayRunner,
) -> NvResult<()> {
    let gsync_state = if settings.enabled { "1" } else { "0" };
    let gsync_arg = format!("[gpu:0]/GPUGSyncAllowed={}", gsync_state);

    runner
        .run_command("nvidia-settings", &["-a", &gsync_arg])
        .map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("nvidia-settings failed: {e}"))
        })?;

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
    #[ignore] // Requires VRR-capable display - run with: cargo test -- --ignored
    fn test_detect_vrr_displays() {
        let displays = detect_vrr_displays().unwrap();
        assert!(!displays.is_empty());
    }
}
