use crate::{NvControlError, NvResult};
use std::process::Command;
use std::path::PathBuf;
use std::env;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibranceProfile {
    pub name: String,
    pub display_settings: HashMap<usize, i32>, // display_id -> vibrance_value
    pub auto_apply_games: Vec<String>,          // List of game executables
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVibranceConfig {
    pub game_executable: String,
    pub game_name: String,
    pub vibrance_profile: String,
    pub auto_detect: bool,
    pub process_names: Vec<String>, // Alternative process names to detect
}

#[derive(Debug, Clone)]
pub struct VibranceSettings {
    pub display_name: String,
    pub vibrance: i32,  // -1024 to 1023 (nvibrant range)
    pub enabled: bool,
}

impl Default for VibranceSettings {
    fn default() -> Self {
        Self {
            display_name: String::new(),
            vibrance: 0,  // 0 = default/no effect
            enabled: true,
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
    let output = Command::new(&nvibrant_path)
        .output()
        .map_err(|e| NvControlError::VibranceControlFailed(
            format!("Failed to run nvibrant: {}. Try installing: pip install nvibrant", e)
        ))?;
    
    if !output.status.success() {
        return Err(NvControlError::VibranceControlFailed(
            "nvibrant failed - ensure NVIDIA drivers are installed and nvidia_drm.modeset=1 is set".to_string()
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
            let display_part = &line[start+1..end];
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
    
    let output = Command::new(&nvibrant_path)
        .output()
        .map_err(|e| NvControlError::VibranceControlFailed(format!("Failed to run nvibrant: {}", e)))?;
    
    if !output.status.success() {
        return Err(NvControlError::VibranceControlFailed("Failed to get vibrance".to_string()));
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
            let vibrance_str = line[start..start+end].trim();
            return vibrance_str.parse().ok();
        }
    }
    None
}

/// Set vibrance for specific displays
pub fn set_vibrance(display_values: &[(usize, i32)]) -> NvResult<()> {
    let nvibrant_path = get_nvibrant_path();
    
    // Find the maximum display index to know how many args to pass
    let max_display = display_values.iter().map(|(idx, _)| *idx).max().unwrap_or(0);
    
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
        .map_err(|e| NvControlError::VibranceControlFailed(format!("Failed to set vibrance: {}", e)))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(NvControlError::VibranceControlFailed(format!("nvibrant failed: {}", error)));
    }
    
    println!("Vibrance applied successfully");
    Ok(())
}

/// Set vibrance for all connected displays
pub fn set_vibrance_all(vibrance: i32) -> NvResult<()> {
    let displays = get_displays()?;
    let display_values: Vec<(usize, i32)> = (0..displays.len())
        .map(|i| (i, vibrance))
        .collect();
    
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

/// Check if nvibrant is available on the system
pub fn is_available() -> bool {
    let nvibrant_path = get_nvibrant_path();
    Command::new(&nvibrant_path)
        .arg("--help")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get driver version compatibility info
pub fn get_driver_info() -> NvResult<String> {
    // Try to get nvidia driver version
    if let Ok(output) = Command::new("nvidia-smi")
        .arg("--query-gpu=driver_version")
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Ok(format!("NVIDIA Driver: {} (nvibrant compatible)", version));
        }
    }
    
    Ok("Driver version unknown - nvibrant may not work".to_string())
}

/// Initialize nvibrant submodule and verify installation
pub fn initialize_nvibrant() -> NvResult<()> {
    println!("🔧 Initializing nvibrant integration...");
    
    // Check if nvibrant is already available
    if is_available() {
        println!("✅ nvibrant already available");
        return Ok(());
    }
    
    // Try to run setup script
    let setup_result = std::process::Command::new("./scripts/setup-nvibrant.sh")
        .output();
    
    match setup_result {
        Ok(output) => {
            if output.status.success() {
                println!("✅ nvibrant setup completed successfully");
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(NvControlError::VibranceControlFailed(format!(
                    "nvibrant setup failed: {}", stderr
                )))
            }
        }
        Err(e) => {
            eprintln!("⚠️ Automatic setup failed: {}", e);
            eprintln!("Please run manually: ./scripts/setup-nvibrant.sh");
            Err(NvControlError::VibranceControlFailed(
                "Manual nvibrant setup required".to_string()
            ))
        }
    }
}

/// Set vibrance for each display using the integrated nVibrant.
/// `levels` should be a vector of values (-1024 to 1023) for each display in physical port order.
/// This is the legacy function for backward compatibility
pub fn set_vibrance_legacy(levels: &[i16]) -> NvResult<()> {
    // Convert i16 to (usize, i32) format for new function
    let display_values: Vec<(usize, i32)> = levels
        .iter()
        .enumerate()
        .map(|(idx, &level)| (idx, level as i32))
        .collect();
    
    set_vibrance(&display_values)
}

/// Advanced vibrance profile management
impl VibranceProfile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_settings: HashMap::new(),
            auto_apply_games: Vec::new(),
            description: String::new(),
        }
    }

    pub fn set_display_vibrance(&mut self, display_id: usize, vibrance: i32) {
        self.display_settings.insert(display_id, vibrance.clamp(-1024, 1023));
    }

    pub fn add_game(&mut self, game_executable: String) {
        if !self.auto_apply_games.contains(&game_executable) {
            self.auto_apply_games.push(game_executable);
        }
    }

    pub fn apply_profile(&self) -> NvResult<()> {
        let display_values: Vec<(usize, i32)> = self.display_settings
            .iter()
            .map(|(&display_id, &vibrance)| (display_id, vibrance))
            .collect();
        
        if !display_values.is_empty() {
            set_vibrance(&display_values)?;
            println!("Applied vibrance profile: {}", self.name);
        }
        Ok(())
    }
}

/// Game detection and vibrance management
impl GameVibranceConfig {
    pub fn new(game_executable: String, vibrance_profile: String) -> Self {
        Self {
            game_executable: game_executable.clone(),
            game_name: game_executable,
            vibrance_profile,
            auto_detect: true,
            process_names: Vec::new(),
        }
    }

    pub fn add_process_name(&mut self, process_name: String) {
        if !self.process_names.contains(&process_name) {
            self.process_names.push(process_name);
        }
    }

    pub fn is_game_running(&self) -> bool {
        // Check main game executable
        if is_process_running(&self.game_executable) {
            return true;
        }

        // Check alternative process names
        for process_name in &self.process_names {
            if is_process_running(process_name) {
                return true;
            }
        }

        false
    }
}

/// Check if a process is currently running
fn is_process_running(process_name: &str) -> bool {
    if let Ok(output) = Command::new("pgrep")
        .arg("-f")
        .arg(process_name)
        .output()
    {
        return output.status.success() && !output.stdout.is_empty();
    }
    false
}

/// Vibrance profile manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibranceProfileManager {
    pub profiles: HashMap<String, VibranceProfile>,
    pub game_configs: HashMap<String, GameVibranceConfig>,
    pub active_profile: Option<String>,
    pub monitoring_enabled: bool,
}

impl Default for VibranceProfileManager {
    fn default() -> Self {
        Self {
            profiles: HashMap::new(),
            game_configs: HashMap::new(),
            active_profile: None,
            monitoring_enabled: false,
        }
    }
}

impl VibranceProfileManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_profile(&mut self, profile: VibranceProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn add_game_config(&mut self, config: GameVibranceConfig) {
        self.game_configs.insert(config.game_executable.clone(), config);
    }

    pub fn set_active_profile(&mut self, profile_name: String) -> NvResult<()> {
        if let Some(profile) = self.profiles.get(&profile_name) {
            profile.apply_profile()?;
            self.active_profile = Some(profile_name);
            Ok(())
        } else {
            Err(NvControlError::VibranceControlFailed(
                format!("Profile '{}' not found", profile_name)
            ))
        }
    }

    pub fn check_running_games(&mut self) -> NvResult<Option<String>> {
        if !self.monitoring_enabled {
            return Ok(None);
        }

        for (_game_exe, config) in &self.game_configs {
            if config.auto_detect && config.is_game_running() {
                if let Some(profile) = self.profiles.get(&config.vibrance_profile) {
                    // Only apply if not already active
                    if self.active_profile.as_ref() != Some(&config.vibrance_profile) {
                        profile.apply_profile()?;
                        self.active_profile = Some(config.vibrance_profile.clone());
                        return Ok(Some(format!("Auto-applied profile '{}' for game '{}'", 
                            config.vibrance_profile, config.game_name)));
                    }
                }
            }
        }
        Ok(None)
    }

    pub fn enable_monitoring(&mut self) {
        self.monitoring_enabled = true;
    }

    pub fn disable_monitoring(&mut self) {
        self.monitoring_enabled = false;
    }

    pub fn create_gaming_profile(&mut self, name: String, vibrance_level: i32) -> NvResult<()> {
        let displays = get_displays()?;
        let mut profile = VibranceProfile::new(name.clone());
        profile.description = format!("Gaming profile with vibrance level {}", vibrance_level);

        // Apply vibrance to all detected displays
        for i in 0..displays.len() {
            profile.set_display_vibrance(i, vibrance_level);
        }

        self.add_profile(profile);
        Ok(())
    }

    pub fn create_default_profiles(&mut self) -> NvResult<()> {
        // Create standard profiles
        self.create_gaming_profile("Gaming High".to_string(), 800)?;
        self.create_gaming_profile("Gaming Medium".to_string(), 400)?;
        self.create_gaming_profile("Standard".to_string(), 0)?;
        self.create_gaming_profile("Cinematic".to_string(), -200)?;

        // Gaming profile for CS2
        let mut cs2_config = GameVibranceConfig::new("cs2.exe".to_string(), "Gaming".to_string());
        cs2_config.add_process_name("cs2".to_string());
        
        self.add_game_config(cs2_config);
        
        let mut valorant_config = GameVibranceConfig::new("VALORANT.exe".to_string(), "Gaming".to_string());
        valorant_config.add_process_name("VALORANT".to_string());
        self.add_game_config(valorant_config);

        Ok(())
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> NvResult<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| NvControlError::VibranceControlFailed(format!("Serialization failed: {}", e)))?;
        
        std::fs::write(path, json)
            .map_err(|e| NvControlError::VibranceControlFailed(format!("Failed to save profile: {}", e)))?;
        
        Ok(())
    }

    pub fn load_from_file(path: &std::path::Path) -> NvResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| NvControlError::VibranceControlFailed(format!("Failed to read profile: {}", e)))?;
        
        let manager = serde_json::from_str(&content)
            .map_err(|e| NvControlError::VibranceControlFailed(format!("Deserialization failed: {}", e)))?;
        
        Ok(manager)
    }

    pub fn list_profiles(&self) -> Vec<&VibranceProfile> {
        self.profiles.values().collect()
    }

    pub fn list_game_configs(&self) -> Vec<&GameVibranceConfig> {
        self.game_configs.values().collect()
    }
}

/// Verify nvibrant works with current system
pub fn test_nvibrant() -> NvResult<()> {
    if !is_available() {
        return Err(NvControlError::VibranceControlFailed(
            "nvibrant not available - run initialization first".to_string()
        ));
    }
    
    // Test basic functionality
    match get_displays() {
        Ok(displays) => {
            println!("✅ nvibrant test successful - found {} displays", displays.len());
            for (i, display) in displays.iter().enumerate() {
                println!("  Display {}: {}", i, display);
            }
            Ok(())
        }
        Err(e) => {
            Err(NvControlError::VibranceControlFailed(format!(
                "nvibrant test failed: {}", e
            )))
        }
    }
}
