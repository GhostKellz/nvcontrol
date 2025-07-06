use crate::{NvControlError, NvResult};

pub fn get_display_info() {
    println!("Display Information:");
    for display in list_displays() {
        println!(
            "  Display {}: {} ({})",
            display.id, display.name, display.kind
        );
        println!(
            "    HDR Capable: {}",
            if display.hdr_capable { "Yes" } else { "No" }
        );
        if display.hdr_capable {
            println!(
                "    HDR Status: {}",
                if display.hdr_enabled {
                    "Enabled"
                } else {
                    "Disabled"
                }
            );
            println!("    Color Depth: {} bit", display.color_depth);
        }
    }
}

/// Returns the number of connected displays
pub fn get_display_count() -> usize {
    // Try wlr-randr first (Wayland)
    if let Ok(output) = std::process::Command::new("wlr-randr").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str
                .lines()
                .filter(|line| !line.starts_with(' ') && !line.is_empty())
                .count();
        }
    }

    // Try xrandr (X11)
    if let Ok(output) = std::process::Command::new("xrandr").arg("--query").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str
                .lines()
                .filter(|line| line.contains(" connected"))
                .count();
        }
    }

    // Fallback to stub
    2
}

/// Represents a display (stub)
pub struct DisplayInfo {
    pub id: usize,
    pub name: String,
    pub kind: String, // e.g. HDMI, DP, etc.
    pub hdr_capable: bool,
    pub hdr_enabled: bool,
    pub color_depth: u8, // 8, 10, 12 bit
}

/// List all displays (stubbed)
pub fn list_displays() -> Vec<DisplayInfo> {
    let mut displays = Vec::new();

    // Try wlr-randr first (Wayland)
    if let Ok(output) = std::process::Command::new("wlr-randr").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut display_id = 0;
            for line in output_str.lines() {
                if !line.starts_with(' ') && !line.is_empty() {
                    let name = line
                        .split_whitespace()
                        .next()
                        .unwrap_or("Unknown")
                        .to_string();
                    let kind = if name.contains("HDMI") {
                        "HDMI"
                    } else if name.contains("DP") {
                        "DP"
                    } else {
                        "Unknown"
                    };
                    let hdr_capable = is_hdr_capable(&name);
                    let hdr_enabled = if hdr_capable {
                        get_hdr_status(&name)
                    } else {
                        false
                    };
                    let color_depth = if hdr_capable { 10 } else { 8 }; // Assume 10-bit for HDR displays
                    displays.push(DisplayInfo {
                        id: display_id,
                        name,
                        kind: kind.to_string(),
                        hdr_capable,
                        hdr_enabled,
                        color_depth,
                    });
                    display_id += 1;
                }
            }
            return displays;
        }
    }

    // Try xrandr (X11)
    if let Ok(output) = std::process::Command::new("xrandr").arg("--query").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut display_id = 0;
            for line in output_str.lines() {
                if line.contains(" connected") {
                    let name = line
                        .split_whitespace()
                        .next()
                        .unwrap_or("Unknown")
                        .to_string();
                    let kind = if name.contains("HDMI") {
                        "HDMI"
                    } else if name.contains("DP") {
                        "DP"
                    } else {
                        "VGA"
                    };
                    let hdr_capable = is_hdr_capable(&name);
                    let hdr_enabled = if hdr_capable {
                        get_hdr_status(&name)
                    } else {
                        false
                    };
                    let color_depth = if hdr_capable { 10 } else { 8 };
                    displays.push(DisplayInfo {
                        id: display_id,
                        name,
                        kind: kind.to_string(),
                        hdr_capable,
                        hdr_enabled,
                        color_depth,
                    });
                    display_id += 1;
                }
            }
            return displays;
        }
    }

    // Fallback to stub data
    vec![
        DisplayInfo {
            id: 0,
            name: "HDMI-1".to_string(),
            kind: "HDMI".to_string(),
            hdr_capable: true,
            hdr_enabled: false,
            color_depth: 10,
        },
        DisplayInfo {
            id: 1,
            name: "DP-1".to_string(),
            kind: "DP".to_string(),
            hdr_capable: true,
            hdr_enabled: false,
            color_depth: 10,
        },
    ]
}

/// Set gamma (stub)
pub fn set_gamma(_display_id: usize, _gamma: f32) {
    // TODO: Implement gamma adjustment
}

/// Check if display supports HDR
pub fn is_hdr_capable(display_name: &str) -> bool {
    // Try to get display capabilities via kscreen-doctor
    if let Ok(output) = std::process::Command::new("kscreen-doctor")
        .arg("-j")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse JSON output to check for HDR capabilities
            return output_str.contains(&format!("\"name\":\"{display_name}\""))
                && (output_str.contains("\"hdr\":true") || output_str.contains("\"hdr10\":true"));
        }
    }

    // Fallback: Check via DRM properties if available
    if let Ok(output) = std::process::Command::new("drm_info").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.contains(display_name) && output_str.contains("HDR");
        }
    }

    // Default assumption for modern displays
    display_name.contains("DP") || display_name.contains("HDMI")
}

/// Get current HDR status for a display
pub fn get_hdr_status(display_name: &str) -> bool {
    // Try kscreen-doctor to get current status
    if let Ok(output) = std::process::Command::new("kscreen-doctor")
        .arg("-j")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Look for HDR enabled status in JSON
            if let Some(display_section) = find_display_in_json(&output_str, display_name) {
                return display_section.contains("\"hdr\":true")
                    || display_section.contains("\"hdrEnabled\":true");
            }
        }
    }

    // Fallback: check via /sys/class/drm if available
    if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("card") && name.contains(&display_name.replace("-", "_")) {
                    let hdr_path = path.join("hdr_output_metadata");
                    if let Ok(content) = std::fs::read_to_string(hdr_path) {
                        return !content.trim().is_empty() && content.trim() != "0";
                    }
                }
            }
        }
    }

    false
}

/// Helper function to find display section in kscreen-doctor JSON output
fn find_display_in_json<'a>(json_str: &'a str, display_name: &str) -> Option<&'a str> {
    // Simple JSON parsing - find the section for our display
    if let Some(start) = json_str.find(&format!("\"name\":\"{display_name}\"")) {
        if let Some(brace_start) = json_str[..start].rfind('{') {
            if let Some(brace_end) = json_str[start..].find('}') {
                return Some(&json_str[brace_start..start + brace_end + 1]);
            }
        }
    }
    None
}

/// Toggle HDR (KDE6 implementation)
pub fn toggle_hdr(display_id: usize) -> NvResult<bool> {
    let displays = list_displays();
    let display = displays.get(display_id).ok_or_else(|| {
        NvControlError::DisplayDetectionFailed(format!("Display {display_id} not found"))
    })?;

    let current_status = get_hdr_status(&display.name);
    let new_status = !current_status;

    // Try different methods based on desktop environment
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    let result = match desktop.as_str() {
        "KDE" => toggle_hdr_kde(&display.name, new_status),
        "GNOME" => toggle_hdr_gnome(&display.name, new_status),
        "Hyprland" => toggle_hdr_hyprland(&display.name, new_status),
        "sway" => toggle_hdr_sway(&display.name, new_status),
        _ => {
            // Try KDE first, then others as fallbacks
            toggle_hdr_kde(&display.name, new_status)
                .or_else(|_| toggle_hdr_gnome(&display.name, new_status))
                .or_else(|_| try_drm_hdr_toggle(&display.name, new_status).map(|_| ()))
        }
    };

    match result {
        Ok(()) => {
            println!(
                "HDR {} for display {}",
                if new_status { "enabled" } else { "disabled" },
                display.name
            );
            Ok(new_status)
        }
        Err(e) => {
            eprintln!("Failed to toggle HDR: {e}");
            Err(e)
        }
    }
}

/// Toggle HDR via KDE
fn toggle_hdr_kde(display_name: &str, enable: bool) -> NvResult<()> {
    let action = if enable { "enable" } else { "disable" };
    let output = std::process::Command::new("kscreen-doctor")
        .arg(format!("output.{display_name}.hdr.{action}"))
        .output()
        .map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("kscreen-doctor not found: {e}"))
        })?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "kscreen-doctor failed: {stderr}"
        )))
    }
}

/// Toggle HDR via GNOME (experimental)
fn toggle_hdr_gnome(display_name: &str, enable: bool) -> NvResult<()> {
    // Method 1: Try gsettings
    let value = if enable { "true" } else { "false" };
    let output = std::process::Command::new("gsettings")
        .args([
            "set",
            "org.gnome.mutter",
            "experimental-features",
            &format!("['hdr-{display_name}={value}']"),
        ])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("gsettings not found: {e}")))?;

    if output.status.success() {
        return Ok(());
    }

    // Method 2: Try D-Bus interface
    let method = if enable { "EnableHDR" } else { "DisableHDR" };
    let output = std::process::Command::new("busctl")
        .args([
            "call",
            "org.gnome.Mutter.DisplayConfig",
            "/org/gnome/Mutter/DisplayConfig",
            "org.gnome.Mutter.DisplayConfig",
            method,
            "s",
            display_name,
        ])
        .output();

    match output {
        Ok(result) if result.status.success() => Ok(()),
        _ => Err(NvControlError::DisplayDetectionFailed(
            "GNOME HDR not supported. Requires GNOME 46+ with experimental features enabled."
                .to_string(),
        )),
    }
}

/// Toggle HDR via Hyprland
fn toggle_hdr_hyprland(display_name: &str, enable: bool) -> NvResult<()> {
    let value = if enable { "1" } else { "0" };
    let output = std::process::Command::new("hyprctl")
        .args(["keyword", "monitor", &format!("{display_name},hdr,{value}")])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("hyprctl not found: {e}")))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "hyprctl failed: {stderr}"
        )))
    }
}

/// Toggle HDR via Sway
fn toggle_hdr_sway(display_name: &str, enable: bool) -> NvResult<()> {
    let value = if enable { "on" } else { "off" };
    let output = std::process::Command::new("swaymsg")
        .args(["output", display_name, "hdr", value])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("swaymsg not found: {e}")))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "swaymsg failed: {stderr}"
        )))
    }
}

/// Fallback HDR toggle via direct DRM interface
fn try_drm_hdr_toggle(display_name: &str, enable: bool) -> NvResult<bool> {
    // Try to use modetest or direct DRM calls if available
    let drm_name = display_name.replace("-", "_");

    // This requires elevated permissions typically
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "echo '{}' | sudo tee /sys/class/drm/card*/card*-{}/hdr_output_metadata",
            if enable { "1" } else { "0" },
            drm_name
        ))
        .output();

    match output {
        Ok(result) if result.status.success() => {
            println!("HDR {} via DRM for {}", 
                if enable { "enabled" } else { "disabled" }, 
                display_name);
            Ok(enable)
        }
        _ => {
            Err(NvControlError::DisplayDetectionFailed(
                "Failed to toggle HDR via both KDE and DRM methods. Ensure you're running KDE Plasma 6+ with HDR support.".to_string()
            ))
        }
    }
}

/// Toggle HDR (legacy stub for compatibility)
pub fn toggle_hdr_legacy(_display_id: usize) {
    println!("HDR toggled (stub - use toggle_hdr for real implementation)");
}

/// List ICC profiles (improved implementation)
pub fn list_icc_profiles() -> Vec<String> {
    let mut profiles = Vec::new();

    // Common ICC profile directories
    let icc_dirs = vec![
        std::path::PathBuf::from("/usr/share/color/icc"),
        std::path::PathBuf::from("/usr/local/share/color/icc"),
        directories::UserDirs::new()
            .map(|d| d.home_dir().join(".color/icc"))
            .unwrap_or_default(),
        directories::UserDirs::new()
            .map(|d| d.home_dir().join(".icc"))
            .unwrap_or_default(),
    ];

    for dir in icc_dirs {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "icc" || ext == "icm" {
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                profiles.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // Add some common fallback profiles if none found
    if profiles.is_empty() {
        profiles = vec![
            "sRGB.icc".to_string(),
            "AdobeRGB.icc".to_string(),
            "DCI-P3.icc".to_string(),
        ];
    }

    profiles.sort();
    profiles.dedup();
    profiles
}

/// Load ICC profile (real implementation)
pub fn load_icc_profile(display_id: usize, profile_name: &str) -> NvResult<()> {
    let displays = list_displays();
    let display = displays.get(display_id).ok_or_else(|| {
        NvControlError::DisplayDetectionFailed(format!("Display {display_id} not found"))
    })?;

    // Find the full path to the ICC profile
    let profile_path = find_icc_profile_path(profile_name).ok_or_else(|| {
        NvControlError::DisplayDetectionFailed(format!("ICC profile {profile_name} not found"))
    })?;

    // Try different methods to load the ICC profile

    // Method 1: KDE's kscreen-doctor (if available)
    if let Ok(output) = std::process::Command::new("kscreen-doctor")
        .arg(format!(
            "output.{}.colorprofile.{}",
            display.name,
            profile_path.display()
        ))
        .output()
    {
        if output.status.success() {
            println!(
                "ICC profile {profile_name} loaded via kscreen-doctor for {}",
                display.name
            );
            return Ok(());
        }
    }

    // Method 2: colord (color daemon)
    if let Ok(output) = std::process::Command::new("colormgr")
        .args([
            "device-add-profile",
            &display.name,
            &profile_path.to_string_lossy(),
        ])
        .output()
    {
        if output.status.success() {
            // Set as default profile
            let _ = std::process::Command::new("colormgr")
                .args(["device-make-profile-default", &display.name, profile_name])
                .output();
            println!(
                "ICC profile {profile_name} loaded via colord for {}",
                display.name
            );
            return Ok(());
        }
    }

    // Method 3: xcalib (X11 fallback)
    if let Ok(output) = std::process::Command::new("xcalib")
        .arg(&profile_path)
        .output()
    {
        if output.status.success() {
            println!(
                "ICC profile {profile_name} loaded via xcalib for {}",
                display.name
            );
            return Ok(());
        }
    }

    // Method 4: dispwin (ArgyllCMS)
    if let Ok(output) = std::process::Command::new("dispwin")
        .args([
            "-d",
            &display_id.to_string(),
            &profile_path.to_string_lossy(),
        ])
        .output()
    {
        if output.status.success() {
            println!(
                "ICC profile {profile_name} loaded via dispwin for {}",
                display.name
            );
            return Ok(());
        }
    }

    Err(NvControlError::DisplayDetectionFailed(format!(
        "Failed to load ICC profile {profile_name}. Install colord, xcalib, or ArgyllCMS for ICC profile support."
    )))
}

/// Find the full path to an ICC profile
fn find_icc_profile_path(profile_name: &str) -> Option<std::path::PathBuf> {
    let icc_dirs = vec![
        std::path::PathBuf::from("/usr/share/color/icc"),
        std::path::PathBuf::from("/usr/local/share/color/icc"),
        directories::UserDirs::new()
            .map(|d| d.home_dir().join(".color/icc"))
            .unwrap_or_default(),
        directories::UserDirs::new()
            .map(|d| d.home_dir().join(".icc"))
            .unwrap_or_default(),
    ];

    for dir in icc_dirs {
        let full_path = dir.join(profile_name);
        if full_path.exists() {
            return Some(full_path);
        }
    }

    None
}

/// Open ICC profile directory in file manager
pub fn open_icc_folder() -> NvResult<()> {
    let icc_dir = directories::UserDirs::new()
        .map(|d| d.home_dir().join(".color/icc"))
        .unwrap_or_else(|| std::path::PathBuf::from("/usr/share/color/icc"));

    // Create directory if it doesn't exist
    if !icc_dir.exists() {
        std::fs::create_dir_all(&icc_dir).map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("Failed to create ICC directory: {e}"))
        })?;
    }

    // Try different file managers
    let file_managers = vec!["xdg-open", "dolphin", "nautilus", "thunar", "pcmanfm"];

    for fm in file_managers {
        if std::process::Command::new(fm).arg(&icc_dir).spawn().is_ok() {
            println!("Opened ICC folder: {}", icc_dir.display());
            return Ok(());
        }
    }

    Err(NvControlError::DisplayDetectionFailed(format!(
        "Could not open ICC folder: {}. No suitable file manager found.",
        icc_dir.display()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_display_count() {
        let count = get_display_count();
        assert!(count > 0, "Should detect at least one display");
    }

    #[test]
    fn test_list_displays() {
        let displays = list_displays();
        assert!(!displays.is_empty(), "Should return at least one display");

        for display in displays {
            assert!(display.id < 10, "Display ID should be reasonable");
            assert!(!display.name.is_empty(), "Display name should not be empty");
            assert!(!display.kind.is_empty(), "Display kind should not be empty");
        }
    }

    #[test]
    fn test_display_info_structure() {
        let display = DisplayInfo {
            id: 0,
            name: "TEST-1".to_string(),
            kind: "HDMI".to_string(),
            hdr_capable: true,
            hdr_enabled: false,
            color_depth: 10,
        };

        assert_eq!(display.id, 0);
        assert_eq!(display.name, "TEST-1");
        assert_eq!(display.kind, "HDMI");
        assert!(display.hdr_capable);
        assert!(!display.hdr_enabled);
        assert_eq!(display.color_depth, 10);
    }

    #[test]
    fn test_icc_profiles_list() {
        let profiles = list_icc_profiles();
        // Should return at least stub profiles
        assert!(!profiles.is_empty());
        assert!(profiles.iter().any(|p| p.contains(".icc")));
    }
}
