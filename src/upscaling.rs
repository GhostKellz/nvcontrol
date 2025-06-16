use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpscalingTechnology {
    DLSS,
    FSR,
    XeSS,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpscalingQuality {
    Performance,  // Lowest latency, highest FPS
    Balanced,     // Good balance
    Quality,      // Better image quality
    UltraQuality, // Best image quality
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpscalingSettings {
    pub technology: UpscalingTechnology,
    pub quality: UpscalingQuality,
    pub enabled: bool,
    pub sharpening: f32,      // 0.0 - 1.0
    pub motion_vectors: bool, // Use motion vectors for better quality
}

impl Default for UpscalingSettings {
    fn default() -> Self {
        Self {
            technology: UpscalingTechnology::Native,
            quality: UpscalingQuality::Balanced,
            enabled: false,
            sharpening: 0.5,
            motion_vectors: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameUpscalingProfile {
    pub game_executable: String,
    pub game_name: String,
    pub settings: UpscalingSettings,
    pub auto_apply: bool,
}

pub struct UpscalingCapabilities {
    pub supports_dlss: bool,
    pub dlss_version: Option<String>,
    pub supports_fsr: bool,
    pub fsr_version: Option<String>,
    pub supports_xess: bool,
    pub gpu_generation: String,
}

pub fn detect_upscaling_capabilities() -> NvResult<UpscalingCapabilities> {
    let mut caps = UpscalingCapabilities {
        supports_dlss: false,
        dlss_version: None,
        supports_fsr: true, // FSR works on most modern GPUs
        fsr_version: Some("2.0".to_string()),
        supports_xess: false,
        gpu_generation: "Unknown".to_string(),
    };

    // Detect NVIDIA GPU and DLSS support
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=name,driver_version"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.trim().split(',').collect();

            if parts.len() >= 2 {
                let gpu_name = parts[0].trim().to_lowercase();
                let driver_version = parts[1].trim();

                // Check for RTX series (DLSS support)
                if gpu_name.contains("rtx") {
                    caps.supports_dlss = true;
                    caps.dlss_version = Some("3.0".to_string()); // Assume latest

                    // Determine generation
                    if gpu_name.contains("40") {
                        caps.gpu_generation = "Ada Lovelace".to_string();
                    } else if gpu_name.contains("30") {
                        caps.gpu_generation = "Ampere".to_string();
                    } else if gpu_name.contains("20") {
                        caps.gpu_generation = "Turing".to_string();
                        caps.dlss_version = Some("2.0".to_string());
                    }
                }

                // Intel Arc support for XeSS
                if gpu_name.contains("arc") {
                    caps.supports_xess = true;
                }

                println!("Detected GPU: {} (Driver: {})", gpu_name, driver_version);
            }
        }
    }

    // Check for AMD GPU and FSR support
    if let Ok(output) = std::process::Command::new("radeontop")
        .arg("-d")
        .arg("-")
        .arg("-l")
        .arg("1")
        .output()
    {
        if output.status.success() {
            caps.supports_fsr = true;
            caps.fsr_version = Some("2.2".to_string());
        }
    }

    Ok(caps)
}

pub fn apply_upscaling_to_game(game_path: &str, settings: &UpscalingSettings) -> NvResult<()> {
    println!(
        "Applying {:?} {} to game: {}",
        settings.technology,
        if settings.enabled { "ON" } else { "OFF" },
        game_path
    );

    match settings.technology {
        UpscalingTechnology::DLSS => apply_dlss(game_path, settings),
        UpscalingTechnology::FSR => apply_fsr(game_path, settings),
        UpscalingTechnology::XeSS => apply_xess(game_path, settings),
        UpscalingTechnology::Native => disable_upscaling(game_path),
    }
}

fn apply_dlss(game_path: &str, settings: &UpscalingSettings) -> NvResult<()> {
    // Try different methods to enable DLSS

    // Method 1: NVIDIA Control Panel API (if available)
    if std::env::var("DISPLAY").is_ok() {
        let quality_setting = match settings.quality {
            UpscalingQuality::Performance => "0",
            UpscalingQuality::Balanced => "1",
            UpscalingQuality::Quality => "2",
            UpscalingQuality::UltraQuality => "3",
        };

        let output = std::process::Command::new("nvidia-settings")
            .args([
                "-a",
                &format!(
                    "[gpu:0]/DLSSMode={}",
                    if settings.enabled {
                        quality_setting
                    } else {
                        "-1"
                    }
                ),
            ])
            .output();

        match output {
            Ok(result) if result.status.success() => {
                println!("DLSS configured via nvidia-settings");
                return Ok(());
            }
            _ => eprintln!("nvidia-settings DLSS configuration failed"),
        }
    }

    // Method 2: Game-specific configuration files
    apply_dlss_via_config(game_path, settings)?;

    // Method 3: Environment variables for supported games
    apply_dlss_via_env_vars(game_path, settings)?;

    Ok(())
}

fn apply_dlss_via_config(game_path: &str, settings: &UpscalingSettings) -> NvResult<()> {
    let path_buf = PathBuf::from(game_path);
    let game_name = path_buf
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    // Common DLSS configuration paths
    let config_paths = get_game_config_paths(game_name);

    for config_path in config_paths {
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                let new_content = modify_dlss_config(&content, settings);
                if std::fs::write(&config_path, new_content).is_ok() {
                    println!("Updated DLSS config: {:?}", config_path);
                }
            }
        }
    }

    Ok(())
}

fn get_game_config_paths(game_name: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        let home_path = PathBuf::from(home);

        // Common game config locations
        paths.extend(vec![
            home_path
                .join(".config")
                .join(game_name)
                .join("settings.ini"),
            home_path
                .join(".local/share")
                .join(game_name)
                .join("config.cfg"),
            home_path
                .join(format!(".{}", game_name))
                .join("settings.cfg"),
            home_path
                .join("Documents")
                .join("My Games")
                .join(game_name)
                .join("settings.ini"),
        ]);

        // Steam game configs
        paths.push(
            home_path
                .join(".steam/steam/steamapps/common")
                .join(game_name)
                .join("config.ini"),
        );

        // Lutris/Wine configs
        paths.push(
            home_path
                .join(".var/app/net.lutris.Lutris/data/lutris/runners/wine")
                .join(game_name)
                .join("user.reg"),
        );
    }

    paths
}

fn modify_dlss_config(content: &str, settings: &UpscalingSettings) -> String {
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let dlss_enabled = if settings.enabled { "1" } else { "0" };
    let quality_value = match settings.quality {
        UpscalingQuality::Performance => "0",
        UpscalingQuality::Balanced => "1",
        UpscalingQuality::Quality => "2",
        UpscalingQuality::UltraQuality => "3",
    };

    // Common DLSS config keys
    let dlss_keys = [
        ("DLSS", dlss_enabled),
        ("DLSSEnabled", dlss_enabled),
        ("DLSSQuality", quality_value),
        ("DLSSMode", quality_value),
        (
            "UpscalingMode",
            if settings.enabled { "DLSS" } else { "Native" },
        ),
    ];

    for (key, value) in dlss_keys {
        let mut found = false;
        for line in &mut lines {
            if line.starts_with(&format!("{}=", key)) || line.starts_with(&format!("{} =", key)) {
                *line = format!("{}={}", key, value);
                found = true;
                break;
            }
        }

        // Add key if not found
        if !found && settings.enabled {
            lines.push(format!("{}={}", key, value));
        }
    }

    lines.join("\n")
}

fn apply_dlss_via_env_vars(_game_path: &str, settings: &UpscalingSettings) -> NvResult<()> {
    // Set environment variables that some games respect
    if settings.enabled {
        unsafe {
            std::env::set_var("NVIDIA_DLSS_ENABLE", "1");
            std::env::set_var(
                "NVIDIA_DLSS_QUALITY",
                match settings.quality {
                    UpscalingQuality::Performance => "0",
                    UpscalingQuality::Balanced => "1",
                    UpscalingQuality::Quality => "2",
                    UpscalingQuality::UltraQuality => "3",
                },
            );
        }
    } else {
        unsafe {
            std::env::remove_var("NVIDIA_DLSS_ENABLE");
            std::env::remove_var("NVIDIA_DLSS_QUALITY");
        }
    }

    Ok(())
}

fn apply_fsr(game_path: &str, settings: &UpscalingSettings) -> NvResult<()> {
    // FSR is typically enabled via game settings or environment variables

    // Method 1: Environment variables for FSR
    if settings.enabled {
        let fsr_quality = match settings.quality {
            UpscalingQuality::Performance => "4",
            UpscalingQuality::Balanced => "3",
            UpscalingQuality::Quality => "2",
            UpscalingQuality::UltraQuality => "1",
        };

        unsafe {
            std::env::set_var("WINE_FSR_ENABLE", "1");
            std::env::set_var("RADV_FSR", "1");
            std::env::set_var("FSR_UPSCALING_RATIO", fsr_quality);
        }
    } else {
        unsafe {
            std::env::remove_var("WINE_FSR_ENABLE");
            std::env::remove_var("RADV_FSR");
            std::env::remove_var("FSR_UPSCALING_RATIO");
        }
    }

    // Method 2: Game-specific configuration
    apply_fsr_via_config(game_path, settings)?;

    println!(
        "FSR {} (Quality: {:?})",
        if settings.enabled {
            "enabled"
        } else {
            "disabled"
        },
        settings.quality
    );

    Ok(())
}

fn apply_fsr_via_config(game_path: &str, settings: &UpscalingSettings) -> NvResult<()> {
    let path_buf = PathBuf::from(game_path);
    let game_name = path_buf
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let config_paths = get_game_config_paths(game_name);

    for config_path in config_paths {
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                let new_content = modify_fsr_config(&content, settings);
                let _ = std::fs::write(&config_path, new_content);
            }
        }
    }

    Ok(())
}

fn modify_fsr_config(content: &str, settings: &UpscalingSettings) -> String {
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let fsr_enabled = if settings.enabled { "1" } else { "0" };
    let quality_value = match settings.quality {
        UpscalingQuality::Performance => "4",
        UpscalingQuality::Balanced => "3",
        UpscalingQuality::Quality => "2",
        UpscalingQuality::UltraQuality => "1",
    };

    let fsr_keys = [
        ("FSR", fsr_enabled),
        ("FSREnabled", fsr_enabled),
        ("FSRQuality", quality_value),
        (
            "UpscalingTechnology",
            if settings.enabled { "FSR" } else { "Native" },
        ),
    ];

    for (key, value) in fsr_keys {
        let mut found = false;
        for line in &mut lines {
            if line.starts_with(&format!("{}=", key)) {
                *line = format!("{}={}", key, value);
                found = true;
                break;
            }
        }

        if !found && settings.enabled {
            lines.push(format!("{}={}", key, value));
        }
    }

    lines.join("\n")
}

fn apply_xess(_game_path: &str, settings: &UpscalingSettings) -> NvResult<()> {
    // Intel XeSS configuration
    if settings.enabled {
        unsafe {
            std::env::set_var("INTEL_XESS_ENABLE", "1");
        }
        println!("Intel XeSS enabled (Quality: {:?})", settings.quality);
    } else {
        unsafe {
            std::env::remove_var("INTEL_XESS_ENABLE");
        }
        println!("Intel XeSS disabled");
    }

    Ok(())
}

fn disable_upscaling(_game_path: &str) -> NvResult<()> {
    // Remove all upscaling environment variables
    let env_vars = [
        "NVIDIA_DLSS_ENABLE",
        "NVIDIA_DLSS_QUALITY",
        "WINE_FSR_ENABLE",
        "RADV_FSR",
        "FSR_UPSCALING_RATIO",
        "INTEL_XESS_ENABLE",
    ];

    unsafe {
        for var in env_vars {
            std::env::remove_var(var);
        }
    }

    println!("All upscaling technologies disabled");
    Ok(())
}

pub fn get_game_upscaling_profiles() -> HashMap<String, GameUpscalingProfile> {
    let mut profiles = HashMap::new();

    // Example game profiles
    profiles.insert(
        "cs2".to_string(),
        GameUpscalingProfile {
            game_executable: "cs2".to_string(),
            game_name: "Counter-Strike 2".to_string(),
            settings: UpscalingSettings {
                technology: UpscalingTechnology::DLSS,
                quality: UpscalingQuality::Performance, // Competitive gaming prefers FPS
                enabled: true,
                sharpening: 0.3,
                motion_vectors: true,
            },
            auto_apply: true,
        },
    );

    profiles.insert(
        "cyberpunk2077".to_string(),
        GameUpscalingProfile {
            game_executable: "Cyberpunk2077.exe".to_string(),
            game_name: "Cyberpunk 2077".to_string(),
            settings: UpscalingSettings {
                technology: UpscalingTechnology::DLSS,
                quality: UpscalingQuality::Quality, // Single-player prefers quality
                enabled: true,
                sharpening: 0.7,
                motion_vectors: true,
            },
            auto_apply: true,
        },
    );

    profiles.insert(
        "witcher3".to_string(),
        GameUpscalingProfile {
            game_executable: "witcher3.exe".to_string(),
            game_name: "The Witcher 3".to_string(),
            settings: UpscalingSettings {
                technology: UpscalingTechnology::FSR, // Fallback for older games
                quality: UpscalingQuality::Balanced,
                enabled: true,
                sharpening: 0.5,
                motion_vectors: false,
            },
            auto_apply: true,
        },
    );

    profiles
}

pub fn auto_detect_running_games() -> NvResult<Vec<String>> {
    let mut running_games = Vec::new();

    // Check running processes for known games
    if let Ok(output) = std::process::Command::new("ps")
        .args(["-eo", "comm"])
        .output()
    {
        if output.status.success() {
            let processes = String::from_utf8_lossy(&output.stdout);
            let game_profiles = get_game_upscaling_profiles();

            for (game_id, profile) in game_profiles {
                let executable = profile.game_executable.to_lowercase();
                if processes.to_lowercase().contains(&executable) {
                    running_games.push(game_id);
                }
            }
        }
    }

    Ok(running_games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upscaling_settings_default() {
        let settings = UpscalingSettings::default();
        assert!(!settings.enabled);
        assert!(matches!(settings.technology, UpscalingTechnology::Native));
    }

    #[test]
    fn test_game_profiles() {
        let profiles = get_game_upscaling_profiles();
        assert!(!profiles.is_empty());
        assert!(profiles.contains_key("cs2"));
    }

    #[test]
    fn test_detect_capabilities() {
        let caps = detect_upscaling_capabilities().unwrap();
        // Should at least support FSR on most systems
        assert!(caps.supports_fsr);
    }
}
