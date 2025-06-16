pub fn get_display_info() {
    println!("Display Information:");
    for display in list_displays() {
        println!("  Display {}: {} ({})", display.id, display.name, display.kind);
    }
}

/// Returns the number of connected displays
pub fn get_display_count() -> usize {
    // Try wlr-randr first (Wayland)
    if let Ok(output) = std::process::Command::new("wlr-randr").output()
        && output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.lines().filter(|line| !line.starts_with(' ') && !line.is_empty()).count();
        }
    
    // Try xrandr (X11)
    if let Ok(output) = std::process::Command::new("xrandr").arg("--query").output()
        && output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.lines().filter(|line| line.contains(" connected")).count();
        }
    
    // Fallback to stub
    2
}

/// Represents a display (stub)
pub struct DisplayInfo {
    pub id: usize,
    pub name: String,
    pub kind: String, // e.g. HDMI, DP, etc.
}

/// List all displays (stubbed)
pub fn list_displays() -> Vec<DisplayInfo> {
    let mut displays = Vec::new();
    
    // Try wlr-randr first (Wayland)
    if let Ok(output) = std::process::Command::new("wlr-randr").output()
        && output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for (id, line) in output_str.lines().enumerate() {
                if !line.starts_with(' ') && !line.is_empty() {
                    let name = line.split_whitespace().next().unwrap_or("Unknown").to_string();
                    let kind = if name.contains("HDMI") { "HDMI" } else if name.contains("DP") { "DP" } else { "Unknown" };
                    displays.push(DisplayInfo { id, name, kind: kind.to_string() });
                }
            }
            return displays;
        }
    
    // Try xrandr (X11)
    if let Ok(output) = std::process::Command::new("xrandr").arg("--query").output()
        && output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for (id, line) in output_str.lines().enumerate() {
                if line.contains(" connected") {
                    let name = line.split_whitespace().next().unwrap_or("Unknown").to_string();
                    let kind = if name.contains("HDMI") { "HDMI" } else if name.contains("DP") { "DP" } else { "VGA" };
                    displays.push(DisplayInfo { id, name, kind: kind.to_string() });
                }
            }
            return displays;
        }
    
    // Fallback to stub data
    vec![
        DisplayInfo {
            id: 0,
            name: "HDMI-1".to_string(),
            kind: "HDMI".to_string(),
        },
        DisplayInfo {
            id: 1,
            name: "DP-1".to_string(),
            kind: "DP".to_string(),
        },
    ]
}

/// Set gamma (stub)
pub fn set_gamma(_display_id: usize, _gamma: f32) {
    // TODO: Implement gamma adjustment
}

/// Toggle HDR (stub)
pub fn toggle_hdr(_display_id: usize) {
    // TODO: Implement HDR toggle (KDE/Wayland/NVIDIA)
    println!("HDR toggled (stub)");
}

/// List ICC profiles (stub)
pub fn list_icc_profiles() -> Vec<String> {
    // TODO: Scan ~/.icc or system ICC profile dirs
    vec!["sRGB.icc".to_string(), "AdobeRGB.icc".to_string()]
}

/// Load ICC profile (stub)
pub fn load_icc_profile(_display_id: usize, _profile: &str) {
    // TODO: Implement ICC profile loading
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
        };
        
        assert_eq!(display.id, 0);
        assert_eq!(display.name, "TEST-1");
        assert_eq!(display.kind, "HDMI");
    }

    #[test]
    fn test_icc_profiles_list() {
        let profiles = list_icc_profiles();
        // Should return at least stub profiles
        assert!(!profiles.is_empty());
        assert!(profiles.iter().any(|p| p.contains(".icc")));
    }
}
