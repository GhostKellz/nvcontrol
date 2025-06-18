use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamescopeConfig {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: Option<u32>,
    pub upscaling: GamescopeUpscaling,
    pub hdr_enabled: bool,
    pub frame_limiter: Option<u32>,
    pub adaptive_sync: bool,
    pub fullscreen: bool,
    pub borderless: bool,
    pub nvidia_optimizations: bool,
    pub environment_variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GamescopeUpscaling {
    None,
    Linear,
    Nearest,
    Fsr, // AMD FidelityFX Super Resolution
    Nis, // NVIDIA Image Scaling
    Integer,
}

impl GamescopeUpscaling {
    pub fn as_str(&self) -> &str {
        match self {
            GamescopeUpscaling::None => "auto",
            GamescopeUpscaling::Linear => "linear",
            GamescopeUpscaling::Nearest => "nearest",
            GamescopeUpscaling::Fsr => "fsr",
            GamescopeUpscaling::Nis => "nis",
            GamescopeUpscaling::Integer => "integer",
        }
    }
}

impl Default for GamescopeConfig {
    fn default() -> Self {
        let mut env_vars = HashMap::new();

        // NVIDIA-specific optimizations
        env_vars.insert("__GL_THREADED_OPTIMIZATIONS".to_string(), "1".to_string());
        env_vars.insert("__GL_SYNC_TO_VBLANK".to_string(), "0".to_string());
        env_vars.insert("NVIDIA_PRIME_RENDER_OFFLOAD".to_string(), "1".to_string());
        env_vars.insert("__NV_PRIME_RENDER_OFFLOAD".to_string(), "1".to_string());
        env_vars.insert(
            "__GLX_VENDOR_LIBRARY_NAME".to_string(),
            "nvidia".to_string(),
        );

        Self {
            width: 1920,
            height: 1080,
            refresh_rate: Some(60),
            upscaling: GamescopeUpscaling::Fsr,
            hdr_enabled: false,
            frame_limiter: None,
            adaptive_sync: true,
            fullscreen: true,
            borderless: false,
            nvidia_optimizations: true,
            environment_variables: env_vars,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GamescopePreset {
    SteamDeck,     // Steam Deck optimized (800p, FSR)
    Handheld1080p, // Handheld 1080p (ROG Ally, etc.)
    Desktop,       // Desktop gaming
    Performance,   // Maximum performance
    Quality,       // Maximum quality
    Battery,       // Battery optimized
}

impl GamescopePreset {
    pub fn to_config(&self) -> GamescopeConfig {
        match self {
            GamescopePreset::SteamDeck => GamescopeConfig {
                width: 1280,
                height: 800,
                refresh_rate: Some(60),
                upscaling: GamescopeUpscaling::Fsr,
                frame_limiter: Some(60),
                ..Default::default()
            },
            GamescopePreset::Handheld1080p => GamescopeConfig {
                width: 1920,
                height: 1080,
                refresh_rate: Some(120),
                upscaling: GamescopeUpscaling::Fsr,
                frame_limiter: Some(90),
                ..Default::default()
            },
            GamescopePreset::Desktop => GamescopeConfig {
                width: 2560,
                height: 1440,
                refresh_rate: Some(144),
                upscaling: GamescopeUpscaling::Nis,
                adaptive_sync: true,
                hdr_enabled: true,
                ..Default::default()
            },
            GamescopePreset::Performance => GamescopeConfig {
                width: 1920,
                height: 1080,
                refresh_rate: Some(240),
                upscaling: GamescopeUpscaling::Nis,
                frame_limiter: None,
                fullscreen: true,
                ..Default::default()
            },
            GamescopePreset::Quality => GamescopeConfig {
                width: 3840,
                height: 2160,
                refresh_rate: Some(60),
                upscaling: GamescopeUpscaling::None,
                hdr_enabled: true,
                frame_limiter: Some(60),
                ..Default::default()
            },
            GamescopePreset::Battery => GamescopeConfig {
                width: 1280,
                height: 720,
                refresh_rate: Some(30),
                upscaling: GamescopeUpscaling::Fsr,
                frame_limiter: Some(30),
                nvidia_optimizations: false, // Power saving
                ..Default::default()
            },
        }
    }
}

/// Check if Gamescope is available on the system
pub fn is_gamescope_available() -> bool {
    which::which("gamescope").is_ok()
}

/// Get current Gamescope version
pub fn get_gamescope_version() -> NvResult<String> {
    let output = Command::new("gamescope")
        .arg("--version")
        .output()
        .map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!(
                "Failed to get Gamescope version: {}",
                e
            ))
        })?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(version)
    } else {
        Err(NvControlError::DisplayDetectionFailed(
            "Gamescope not available".to_string(),
        ))
    }
}

/// Detect if currently running inside Gamescope
pub fn is_running_in_gamescope() -> bool {
    // Check for Gamescope-specific environment variables
    std::env::var("GAMESCOPE_WAYLAND_DISPLAY").is_ok()
        || std::env::var("GAMESCOPE").is_ok()
        || std::env::var("STEAM_GAMESCOPE").is_ok()
}

/// Configure Gamescope with optimization preset
pub fn configure_gamescope() -> NvResult<()> {
    if !is_gamescope_available() {
        return Err(NvControlError::DisplayDetectionFailed(
            "Gamescope not installed. Install with: sudo pacman -S gamescope".to_string(),
        ));
    }

    let version = get_gamescope_version()?;
    println!("Gamescope version: {}", version);

    if is_running_in_gamescope() {
        println!("Currently running inside Gamescope session");
        apply_runtime_optimizations()?;
    } else {
        println!("Gamescope available but not currently active");
        print_gamescope_usage();
    }

    Ok(())
}

/// Apply runtime optimizations when running inside Gamescope
fn apply_runtime_optimizations() -> NvResult<()> {
    println!("Applying Gamescope runtime optimizations...");

    // Set NVIDIA-specific environment variables
    let nvidia_env_vars = vec![
        ("__GL_THREADED_OPTIMIZATIONS", "1"),
        ("__GL_SYNC_TO_VBLANK", "0"),
        ("__GL_MaxFramesAllowed", "1"),
        ("NVIDIA_PRIME_RENDER_OFFLOAD", "1"),
        ("__NV_PRIME_RENDER_OFFLOAD", "1"),
        ("__GLX_VENDOR_LIBRARY_NAME", "nvidia"),
    ];

    for (key, value) in nvidia_env_vars {
        unsafe {
            std::env::set_var(key, value);
        }
        println!("  Set {}={}", key, value);
    }

    // Apply CPU governor optimization for gaming
    let _ = crate::latency::optimize_latency();

    println!("Gamescope optimizations applied");
    Ok(())
}

/// Launch a game with Gamescope using specified configuration
pub fn launch_with_gamescope(game_path: &str) -> NvResult<()> {
    launch_with_gamescope_config(game_path, &GamescopeConfig::default())
}

/// Launch a game with Gamescope using a preset
pub fn launch_with_gamescope_preset(game_path: &str, preset: GamescopePreset) -> NvResult<()> {
    let config = preset.to_config();
    launch_with_gamescope_config(game_path, &config)
}

/// Launch a game with Gamescope using custom configuration
pub fn launch_with_gamescope_config(game_path: &str, config: &GamescopeConfig) -> NvResult<()> {
    if !is_gamescope_available() {
        return Err(NvControlError::DisplayDetectionFailed(
            "Gamescope not available. Install with your package manager.".to_string(),
        ));
    }

    if !Path::new(game_path).exists() {
        return Err(NvControlError::DisplayDetectionFailed(format!(
            "Game not found: {}",
            game_path
        )));
    }

    println!("Launching {} with Gamescope...", game_path);

    let mut cmd = Command::new("gamescope");

    // Basic resolution and display settings
    cmd.args(&[
        "-w",
        &config.width.to_string(),
        "-h",
        &config.height.to_string(),
    ]);

    // Refresh rate
    if let Some(refresh_rate) = config.refresh_rate {
        cmd.args(&["-r", &refresh_rate.to_string()]);
    }

    // Upscaling method
    cmd.args(&["-U", config.upscaling.as_str()]);

    // Frame limiter
    if let Some(fps_limit) = config.frame_limiter {
        cmd.args(&["-o", &fps_limit.to_string()]);
    }

    // HDR support
    if config.hdr_enabled {
        cmd.arg("--hdr-enabled");
    }

    // Adaptive sync
    if config.adaptive_sync {
        cmd.arg("--adaptive-sync");
    }

    // Fullscreen mode
    if config.fullscreen {
        cmd.arg("-f");
    }

    // Borderless mode
    if config.borderless {
        cmd.arg("-b");
    }

    // NVIDIA-specific optimizations
    if config.nvidia_optimizations {
        cmd.args(&["--backend", "sdl"]);
        cmd.arg("--immediate-flips");
        cmd.arg("--force-grab-cursor");
    }

    // Add environment variables
    for (key, value) in &config.environment_variables {
        cmd.env(key, value);
    }

    // Add the game command
    cmd.arg("--");
    cmd.arg(game_path);

    println!("Gamescope command: {:?}", cmd);

    // Launch Gamescope
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("Failed to launch Gamescope: {}", e))
        })?;

    println!("Gamescope launched with PID: {}", child.id());

    // Wait for the process to complete
    let exit_status = child.wait().map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Error waiting for Gamescope: {}", e))
    })?;

    if exit_status.success() {
        println!("Gamescope session completed successfully");
    } else {
        println!("Gamescope session ended with errors");
    }

    Ok(())
}

/// Create a Steam integration for Gamescope
pub fn create_steam_gamescope_wrapper(
    game_name: &str,
    game_path: &str,
    preset: GamescopePreset,
) -> NvResult<PathBuf> {
    let config = preset.to_config();

    // Create wrapper script
    let script_content = generate_steam_wrapper_script(game_path, &config)?;

    // Save to user's local bin directory
    let user_dirs = directories::UserDirs::new().ok_or_else(|| {
        NvControlError::DisplayDetectionFailed("Could not find home directory".to_string())
    })?;
    let home_dir = user_dirs.home_dir();

    let local_bin = home_dir.join(".local/bin");
    fs::create_dir_all(&local_bin).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!(
            "Failed to create local bin directory: {}",
            e
        ))
    })?;

    let script_path = local_bin.join(format!(
        "gamescope-{}.sh",
        game_name.to_lowercase().replace(' ', "-")
    ));

    fs::write(&script_path, script_content).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to write script: {}", e))
    })?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    println!("Created Gamescope wrapper: {}", script_path.display());
    println!("Add this to Steam as a non-Steam game to use Gamescope integration");

    Ok(script_path)
}

/// Generate a wrapper script for Steam integration
fn generate_steam_wrapper_script(game_path: &str, config: &GamescopeConfig) -> NvResult<String> {
    let mut script = String::new();

    script.push_str("#!/bin/bash\n");
    script.push_str("# Generated by nvcontrol - Gamescope wrapper\n\n");

    // Add environment variables
    script.push_str("# NVIDIA optimizations\n");
    for (key, value) in &config.environment_variables {
        script.push_str(&format!("export {}=\"{}\"\n", key, value));
    }
    script.push('\n');

    // Build Gamescope command
    script.push_str("# Launch with Gamescope\n");
    script.push_str("exec gamescope");

    script.push_str(&format!(" -w {} -h {}", config.width, config.height));

    if let Some(refresh_rate) = config.refresh_rate {
        script.push_str(&format!(" -r {}", refresh_rate));
    }

    script.push_str(&format!(" -U {}", config.upscaling.as_str()));

    if let Some(fps_limit) = config.frame_limiter {
        script.push_str(&format!(" -o {}", fps_limit));
    }

    if config.hdr_enabled {
        script.push_str(" --hdr-enabled");
    }

    if config.adaptive_sync {
        script.push_str(" --adaptive-sync");
    }

    if config.fullscreen {
        script.push_str(" -f");
    }

    if config.borderless {
        script.push_str(" -b");
    }

    if config.nvidia_optimizations {
        script.push_str(" --backend sdl --immediate-flips --force-grab-cursor");
    }

    script.push_str(&format!(" -- \"{}\" \"$@\"\n", game_path));

    Ok(script)
}

/// Optimize Gamescope for NVIDIA + Wayland
pub fn optimize_gamescope_nvidia_wayland() -> NvResult<()> {
    println!("Optimizing Gamescope for NVIDIA + Wayland...");

    // Check prerequisites
    if !is_gamescope_available() {
        return Err(NvControlError::DisplayDetectionFailed(
            "Gamescope not available".to_string(),
        ));
    }

    // Set optimal environment variables for NVIDIA + Wayland + Gamescope
    let optimizations = vec![
        ("GBM_BACKEND", "nvidia-drm"),
        ("__GLX_VENDOR_LIBRARY_NAME", "nvidia"),
        ("WLR_NO_HARDWARE_CURSORS", "1"),
        ("GAMESCOPE_WAYLAND_DISPLAY", "gamescope-0"),
        ("SDL_VIDEODRIVER", "wayland"),
        ("QT_QPA_PLATFORM", "wayland"),
        ("GDK_BACKEND", "wayland"),
        ("CLUTTER_BACKEND", "wayland"),
        ("__GL_GSYNC_ALLOWED", "1"),
        ("__GL_VRR_ALLOWED", "1"),
    ];

    println!("Applied optimizations:");
    for (key, value) in optimizations {
        unsafe {
            std::env::set_var(key, value);
        }
        println!("  {}={}", key, value);
    }

    // Create user configuration file
    create_gamescope_config_file()?;

    println!("\nGamescope optimization complete!");
    println!("Restart your session for all changes to take effect.");

    Ok(())
}

/// Create user configuration file for Gamescope
fn create_gamescope_config_file() -> NvResult<()> {
    let user_dirs = directories::UserDirs::new().ok_or_else(|| {
        crate::NvControlError::DisplayDetectionFailed("Could not find home directory".to_string())
    })?;
    let config_dir = user_dirs.home_dir().join(".config/gamescope");

    let config_content = r#"# Gamescope configuration generated by nvcontrol
# NVIDIA + Wayland optimizations

# Environment variables
export GBM_BACKEND=nvidia-drm
export __GLX_VENDOR_LIBRARY_NAME=nvidia
export WLR_NO_HARDWARE_CURSORS=1
export __GL_GSYNC_ALLOWED=1
export __GL_VRR_ALLOWED=1

# Default Gamescope options for NVIDIA
GAMESCOPE_NVIDIA_ARGS="--backend sdl --immediate-flips --force-grab-cursor"
"#;

    std::fs::create_dir_all(&config_dir).map_err(|e| {
        crate::NvControlError::DisplayDetectionFailed(format!(
            "Failed to create config directory: {}",
            e
        ))
    })?;

    let config_file = config_dir.join("nvcontrol-gamescope.conf");
    std::fs::write(&config_file, config_content).map_err(|e| {
        crate::NvControlError::DisplayDetectionFailed(format!("Failed to write config file: {}", e))
    })?;

    println!("Created Gamescope config at: {}", config_file.display());
    Ok(())
}

/// Print usage information for Gamescope
fn print_gamescope_usage() {
    println!("Gamescope Usage:");
    println!("===============");
    println!("Basic usage:");
    println!("  gamescope -w 1920 -h 1080 -r 144 -- your-game");
    println!("Performance mode:");
    println!("  gamescope -w 1920 -h 1080 -r 144 -U -- your-game");
    println!("With FSR:");
    println!("  gamescope -w 1920 -h 1080 -W 2560 -H 1440 -U -- your-game");
}

/// Get Gamescope status and information
pub fn get_gamescope_info() -> NvResult<()> {
    println!("Gamescope Information:");
    println!("=====================");

    if is_gamescope_available() {
        let version = get_gamescope_version()?;
        println!("Version: {}", version);

        if is_running_in_gamescope() {
            println!("Status: Running inside Gamescope");
        } else {
            println!("Status: Gamescope available but not currently running");
        }
    } else {
        println!("Status: Gamescope not found");
        println!("Install with: sudo apt install gamescope  # or your package manager");
    }

    print_gamescope_usage();
    Ok(())
}
