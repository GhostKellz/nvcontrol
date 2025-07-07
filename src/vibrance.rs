use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibranceProfile {
    pub name: String,
    pub description: String,
    pub display_settings: HashMap<usize, EnhancedVibranceSettings>, // display_id -> settings
    pub auto_apply_games: Vec<String>,                              // List of game executables
    pub quick_preset_hotkey: Option<String>,                        // Keyboard shortcut
    pub created_at: u64,                                            // Unix timestamp
    pub last_used: u64,                                             // Unix timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedVibranceSettings {
    pub vibrance: i32,          // -1024 to 1023
    pub saturation: f32,        // 0.0 to 2.0
    pub contrast: f32,          // 0.0 to 2.0
    pub brightness: f32,        // 0.0 to 2.0
    pub gamma: f32,             // 0.5 to 3.0
    pub hue_shift: f32,         // -180.0 to 180.0 degrees
    pub color_temperature: i32, // 1000K to 10000K
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVibranceConfig {
    pub game_executable: String,
    pub game_name: String,
    pub vibrance_profile: String,
    pub auto_detect: bool,
    pub process_names: Vec<String>, // Alternative process names to detect
    pub window_class: Option<String>, // X11 window class detection
    pub steam_app_id: Option<u32>,  // Steam app ID for detection
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibranceSchedule {
    pub name: String,
    pub enabled: bool,
    pub time_slots: Vec<TimeSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start_hour: u8,   // 0-23
    pub start_minute: u8, // 0-59
    pub end_hour: u8,     // 0-23
    pub end_minute: u8,   // 0-59
    pub profile_name: String,
    pub days_of_week: Vec<u8>, // 0=Sunday, 1=Monday, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickPreset {
    pub name: String,
    pub hotkey: String,
    pub profile_name: String,
    pub icon: Option<String>,
    pub color: Option<String>, // Hex color for UI
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub id: usize,
    pub name: String,
    pub connected: bool,
    pub primary: bool,
    pub resolution: (u32, u32),
    pub refresh_rate: Option<f32>,
    pub color_depth: Option<u8>,
    pub current_settings: EnhancedVibranceSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibranceSettings {
    pub vibrance: i32,
    pub display_id: usize,
}

impl Default for EnhancedVibranceSettings {
    fn default() -> Self {
        Self {
            vibrance: 0,
            saturation: 1.0,
            contrast: 1.0,
            brightness: 1.0,
            gamma: 2.2,
            hue_shift: 0.0,
            color_temperature: 6500,
            enabled: true,
        }
    }
}

impl Default for VibranceProfile {
    fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        Self {
            name: "Default".to_string(),
            description: "Default vibrance profile".to_string(),
            display_settings: HashMap::new(),
            auto_apply_games: Vec::new(),
            quick_preset_hotkey: None,
            created_at: now,
            last_used: now,
        }
    }
}

/// Get the path to the bundled nvibrant binary
fn get_nvibrant_path() -> PathBuf {
    // First check if we have a bundled version from build script
    if let Ok(binary_path) = env::var("NVIBRANT_BINARY_PATH") {
        let path = PathBuf::from(binary_path);
        if path.exists() {
            return path;
        }
    }

    // Check if we have it in the same directory as our binary
    if let Ok(exe_path) = env::current_exe() {
        let bundled = exe_path.parent().unwrap().join("nvibrant");
        if bundled.exists() {
            return bundled;
        }
    }

    // Try system installation paths
    let system_paths = vec![
        PathBuf::from("/usr/local/bin/nvibrant"),
        PathBuf::from("/usr/bin/nvibrant"),
        PathBuf::from("nvibrant"), // PATH lookup
    ];

    for path in system_paths {
        if path.exists() || path.file_name().is_some() {
            return path;
        }
    }

    // Fall back to nvibrant command (will use PATH)
    PathBuf::from("nvibrant")
}

/// Detect connected displays and their vibrance support
pub fn get_displays() -> NvResult<Vec<String>> {
    let nvibrant_path = get_nvibrant_path();

    // Run nvibrant with no args to see current state
    let output = Command::new(&nvibrant_path).output().map_err(|e| {
        NvControlError::VibranceControlFailed(format!(
            "Failed to run nvibrant: {}. Try installing: pip install nvibrant",
            e
        ))
    })?;

    if !output.status.success() {
        return Err(NvControlError::VibranceControlFailed(
            "nvibrant failed - ensure NVIDIA drivers are installed and nvidia_drm.modeset=1 is set"
                .to_string(),
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut displays = Vec::new();

    // Parse nvibrant output to find connected displays
    for line in output_str.lines() {
        if line.contains("Success") && !line.contains("None") {
            if let Some(display_info) = parse_display_line(line) {
                displays.push(display_info);
            }
        }
    }

    // If no displays found, add fallback info
    if displays.is_empty() {
        displays.push("Display 0:HDMI (not detected)".to_string());
        displays.push("Display 1:DP (not detected)".to_string());
    }

    Ok(displays)
}

fn parse_display_line(line: &str) -> Option<String> {
    // Parse lines like: "• (0, HDMI) • Set Vibrance (  512) • Success"
    if let Some(start) = line.find('(') {
        if let Some(end) = line.find(')') {
            let display_part = &line[start + 1..end];
            let parts: Vec<&str> = display_part.split(", ").collect();
            if parts.len() == 2 {
                let index = parts[0];
                let connector = parts[1];
                return Some(format!("Display {}:{}", index, connector));
            }
        }
    }
    None
}

/// Get current vibrance for a specific display
pub fn get_display_vibrance(display_index: usize) -> NvResult<i32> {
    let nvibrant_path = get_nvibrant_path();

    let output = Command::new(&nvibrant_path).output().map_err(|e| {
        NvControlError::VibranceControlFailed(format!("Failed to run nvibrant: {}", e))
    })?;

    if !output.status.success() {
        return Err(NvControlError::VibranceControlFailed(
            "Failed to get vibrance".to_string(),
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        if line.contains(&format!("({},", display_index)) && line.contains("Success") {
            if let Some(vibrance) = extract_vibrance_from_line(line) {
                return Ok(vibrance);
            }
        }
    }

    Ok(0) // Default if not found
}

fn extract_vibrance_from_line(line: &str) -> Option<i32> {
    // Extract from "• (0, HDMI) • Set Vibrance (  512) • Success"
    if let Some(start) = line.find("Set Vibrance (") {
        let start = start + 14; // length of "Set Vibrance ("
        if let Some(end) = line[start..].find(')') {
            let vibrance_str = line[start..start + end].trim();
            return vibrance_str.parse().ok();
        }
    }
    None
}

/// Set vibrance for specific displays
pub fn set_vibrance(display_values: &[(usize, i32)]) -> NvResult<()> {
    let nvibrant_path = get_nvibrant_path();

    // Find the maximum display index to know how many args to pass
    let max_display = display_values
        .iter()
        .map(|(idx, _)| *idx)
        .max()
        .unwrap_or(0);

    // Build arguments array with 0 for unspecified displays
    let mut args = vec![0; max_display + 1];
    for (display_idx, vibrance) in display_values {
        if *display_idx < args.len() {
            // Clamp vibrance to valid range
            args[*display_idx] = (*vibrance).clamp(-1024, 1023);
        }
    }

    // Convert to strings
    let str_args: Vec<String> = args.iter().map(|v| v.to_string()).collect();

    let output = Command::new(&nvibrant_path)
        .args(&str_args)
        .output()
        .map_err(|e| {
            NvControlError::VibranceControlFailed(format!("Failed to set vibrance: {}", e))
        })?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(NvControlError::VibranceControlFailed(format!(
            "nvibrant failed: {}",
            error
        )));
    }

    println!("Vibrance applied successfully");
    Ok(())
}

/// Set vibrance for all connected displays
pub fn set_vibrance_all(vibrance: i32) -> NvResult<()> {
    let displays = get_displays()?;
    let display_values: Vec<(usize, i32)> = (0..displays.len()).map(|i| (i, vibrance)).collect();

    set_vibrance(&display_values)
}

/// Convert percentage (0-200%) to nvibrant range (-1024 to 1023)
/// 100% = 0 (default), 0% = -1024 (grayscale), 200% = 1023 (max saturation)
pub fn percentage_to_vibrance(percentage: u32) -> i32 {
    let percentage = percentage.min(200); // Cap at 200%

    if percentage <= 100 {
        // 0-100% maps to -1024 to 0
        let ratio = percentage as f32 / 100.0;
        ((ratio - 1.0) * 1024.0) as i32
    } else {
        // 100-200% maps to 0 to 1023
        let ratio = (percentage - 100) as f32 / 100.0;
        (ratio * 1023.0) as i32
    }
}

/// Convert nvibrant range to percentage (0-200%)
pub fn vibrance_to_percentage(vibrance: i32) -> u32 {
    if vibrance <= 0 {
        // -1024 to 0 maps to 0-100%
        (((vibrance + 1024) as f32 / 1024.0) * 100.0) as u32
    } else {
        // 0 to 1023 maps to 100-200%
        (100.0 + (vibrance as f32 / 1023.0 * 100.0)) as u32
    }
}

/// Set vibrance for a specific display by ID
pub fn set_display_vibrance(display_id: usize, vibrance: i32) -> NvResult<()> {
    set_vibrance(&[(display_id, vibrance)])
}

/// Get display name for a specific display ID
pub fn get_display_name(display_id: usize) -> NvResult<String> {
    let displays = get_displays()?;
    if display_id < displays.len() {
        Ok(displays[display_id].clone())
    } else {
        Err(NvControlError::VibranceControlFailed(format!(
            "Display ID {} not found (available: 0-{})",
            display_id,
            displays.len().saturating_sub(1)
        )))
    }
}

/// Check if vibrance control is available
pub fn is_available() -> bool {
    let nvibrant_path = get_nvibrant_path();
    std::path::Path::new(&nvibrant_path).exists()
}

/// Get driver information
pub fn get_driver_info() -> NvResult<String> {
    use std::process::Command;

    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=driver_version")
        .arg("--format=csv,noheader,nounits")
        .output()
        .map_err(|e| {
            NvControlError::VibranceControlFailed(format!("Failed to get driver info: {}", e))
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(NvControlError::VibranceControlFailed(
            "Failed to get driver version".to_string(),
        ))
    }
}

/// Initialize nvibrant
pub fn initialize_nvibrant() -> NvResult<()> {
    if is_available() {
        Ok(())
    } else {
        Err(NvControlError::VibranceControlFailed(
            "nvibrant not available".to_string(),
        ))
    }
}

/// Test nvibrant functionality
pub fn test_nvibrant() -> NvResult<()> {
    use std::process::Command;

    let nvibrant_path = get_nvibrant_path();
    let output = Command::new(&nvibrant_path)
        .arg("--help")
        .output()
        .map_err(|e| {
            NvControlError::VibranceControlFailed(format!("Failed to test nvibrant: {}", e))
        })?;

    if output.status.success() {
        Ok(())
    } else {
        Err(NvControlError::VibranceControlFailed(
            "nvibrant test failed".to_string(),
        ))
    }
}

/// Detect enhanced displays
pub fn detect_enhanced_displays() -> NvResult<Vec<(usize, String)>> {
    let displays = get_displays()?;
    let enhanced_displays: Vec<(usize, String)> = displays.into_iter().enumerate().collect();
    Ok(enhanced_displays)
}

/// Load enhanced profiles
pub fn load_enhanced_profiles() -> NvResult<Vec<VibranceProfile>> {
    // Return some default profiles for now
    Ok(vec![
        VibranceProfile {
            name: "Gaming".to_string(),
            description: "Enhanced vibrance for gaming".to_string(),
            display_settings: std::collections::HashMap::new(),
            auto_apply_games: vec![],
            quick_preset_hotkey: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            last_used: 0,
        },
        VibranceProfile {
            name: "Work".to_string(),
            description: "Comfortable vibrance for work".to_string(),
            display_settings: std::collections::HashMap::new(),
            auto_apply_games: vec![],
            quick_preset_hotkey: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            last_used: 0,
        },
    ])
}

/// Apply enhanced vibrance
pub fn apply_enhanced_vibrance(display_id: usize, settings: &VibranceSettings) -> NvResult<()> {
    set_display_vibrance(display_id, settings.vibrance)
}

/// Preview vibrance changes
pub fn preview_vibrance_changes(
    display_id: usize,
    settings: &VibranceSettings,
    duration_ms: u64,
) -> NvResult<()> {
    use std::thread;
    use std::time::Duration;

    // Get current vibrance
    let original_vibrance = get_display_vibrance(display_id)?;

    // Apply new vibrance
    set_display_vibrance(display_id, settings.vibrance)?;

    // Wait for specified duration
    thread::sleep(Duration::from_millis(duration_ms));

    // Restore original vibrance
    set_display_vibrance(display_id, original_vibrance)
}
