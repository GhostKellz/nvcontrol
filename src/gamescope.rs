use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub steam_deck_mode: bool,
    pub hdr_metadata: HdrMetadata,
    pub color_management: ColorManagement,
    pub performance_profile: PerformanceProfile,
    pub filter_settings: FilterSettings,
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
    FsrSharpness(f32), // FSR with custom sharpness
    Custom(String),    // Custom upscaling algorithm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GamescopePreset {
    Performance,
    Quality,
    Balanced,
    PowerSaving,
    SteamDeck,
    SteamDeckDocked,
    SteamDeckHandheld,
    Handheld1080p,
    Desktop,
    Competitive,
    Cinematic,
    Custom(GamescopeConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdrMetadata {
    pub enabled: bool,
    pub color_space: ColorSpace,
    pub max_luminance: u32, // nits
    pub min_luminance: f32, // nits
    pub mastering_display: Option<MasteringDisplay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorSpace {
    Srgb,
    Rec2020,
    DciP3,
    Rec709,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteringDisplay {
    pub red_x: f32,
    pub red_y: f32,
    pub green_x: f32,
    pub green_y: f32,
    pub blue_x: f32,
    pub blue_y: f32,
    pub white_x: f32,
    pub white_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorManagement {
    pub enabled: bool,
    pub gamma_correction: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub contrast: f32,
    pub color_temperature: u32, // Kelvin
    pub night_mode: bool,
    pub custom_lut: Option<String>, // Path to custom LUT file
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub name: String,
    pub cpu_governor: Option<String>,
    pub gpu_performance_level: Option<String>,
    pub frame_pacing: bool,
    pub power_management: PowerManagement,
    pub thermal_throttling: bool,
    pub async_compute: bool,
    pub shader_cache_precompile: bool,
    pub memory_pool_size: Option<u64>,
    pub thread_priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSettings {
    pub sharpening: f32,      // 0.0-2.0
    pub noise_reduction: f32, // 0.0-1.0
    pub color_vibrance: f32,  // 0.0-2.0
    pub tonemapping: bool,
    pub anti_aliasing: AntiAliasing,
    pub motion_blur_reduction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AntiAliasing {
    None,
    Fxaa,
    Smaa,
    Taa,
    Msaa2x,
    Msaa4x,
    Msaa8x,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerManagement {
    Performance,
    Balanced,
    PowerSaver,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamDeckPreset {
    pub name: String,
    pub description: String,
    pub config: GamescopeConfig,
    pub optimizations: SteamDeckOptimizations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamDeckOptimizations {
    pub battery_saver: bool,
    pub thermal_throttling: bool,
    pub aggressive_power_saving: bool,
    pub dock_mode: bool,
    pub external_display_config: Option<GamescopeConfig>,
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
            GamescopeUpscaling::FsrSharpness(_) => "fsr",
            GamescopeUpscaling::Custom(_) => "auto",
        }
    }

    pub fn get_sharpness(&self) -> Option<f32> {
        match self {
            GamescopeUpscaling::FsrSharpness(sharpness) => Some(*sharpness),
            _ => None,
        }
    }
}

impl Default for HdrMetadata {
    fn default() -> Self {
        Self {
            enabled: false,
            color_space: ColorSpace::Srgb,
            max_luminance: 1000,
            min_luminance: 0.1,
            mastering_display: None,
        }
    }
}

impl Default for ColorManagement {
    fn default() -> Self {
        Self {
            enabled: false,
            gamma_correction: 2.2,
            saturation: 1.0,
            brightness: 1.0,
            contrast: 1.0,
            color_temperature: 6500,
            night_mode: false,
            custom_lut: None,
        }
    }
}

impl Default for PerformanceProfile {
    fn default() -> Self {
        Self {
            name: "Balanced".to_string(),
            cpu_governor: Some("performance".to_string()),
            gpu_performance_level: Some("auto".to_string()),
            frame_pacing: true,
            power_management: PowerManagement::Balanced,
            thermal_throttling: true,
            async_compute: true,
            shader_cache_precompile: true,
            memory_pool_size: None,
            thread_priority: None,
        }
    }
}

impl Default for FilterSettings {
    fn default() -> Self {
        Self {
            sharpening: 0.5,
            noise_reduction: 0.2,
            color_vibrance: 1.0,
            tonemapping: false,
            anti_aliasing: AntiAliasing::Fxaa,
            motion_blur_reduction: false,
        }
    }
}

impl Default for GamescopeConfig {
    fn default() -> Self {
        let mut env_vars = HashMap::new();

        // NVIDIA-specific optimizations
        env_vars.insert("__GL_THREADED_OPTIMIZATIONS".to_string(), "1".to_string());
        env_vars.insert("__GL_SHADER_DISK_CACHE".to_string(), "1".to_string());
        env_vars.insert(
            "__GL_SHADER_DISK_CACHE_SKIP_CLEANUP".to_string(),
            "1".to_string(),
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
            steam_deck_mode: false,
            hdr_metadata: HdrMetadata::default(),
            color_management: ColorManagement::default(),
            performance_profile: PerformanceProfile::default(),
            filter_settings: FilterSettings::default(),
            environment_variables: env_vars,
        }
    }
}

impl GamescopePreset {
    /// Convert preset to GamescopeConfig
    pub fn to_config(&self) -> GamescopeConfig {
        match self {
            GamescopePreset::Performance => GamescopeConfig {
                width: 1920,
                height: 1080,
                refresh_rate: Some(144),
                upscaling: GamescopeUpscaling::Nis,
                hdr_enabled: false,
                frame_limiter: None,
                adaptive_sync: true,
                fullscreen: true,
                borderless: false,
                nvidia_optimizations: true,
                steam_deck_mode: false,
                hdr_metadata: Default::default(),
                color_management: ColorManagement {
                    enabled: false,
                    gamma_correction: 1.0,
                    saturation: 1.0,
                    brightness: 1.0,
                    contrast: 1.0,
                    color_temperature: 6500,
                    night_mode: false,
                    custom_lut: None,
                },
                performance_profile: PerformanceProfile {
                    name: "Performance".to_string(),
                    cpu_governor: Some("performance".to_string()),
                    gpu_performance_level: Some("max".to_string()),
                    frame_pacing: false,
                    power_management: PowerManagement::Performance,
                    thermal_throttling: false,
                    async_compute: true,
                    shader_cache_precompile: true,
                    memory_pool_size: Some(2 * 1024 * 1024 * 1024), // 2GB
                    thread_priority: Some(-10),
                },
                filter_settings: Default::default(),
                environment_variables: HashMap::new(),
            },
            GamescopePreset::Quality => GamescopeConfig {
                width: 3840,
                height: 2160,
                refresh_rate: Some(60),
                upscaling: GamescopeUpscaling::Fsr,
                hdr_enabled: true,
                frame_limiter: Some(60),
                adaptive_sync: true,
                fullscreen: true,
                borderless: false,
                nvidia_optimizations: true,
                steam_deck_mode: false,
                hdr_metadata: HdrMetadata {
                    enabled: true,
                    color_space: ColorSpace::Rec2020,
                    max_luminance: 1000,
                    min_luminance: 0.01,
                    mastering_display: None,
                },
                color_management: ColorManagement {
                    enabled: true,
                    gamma_correction: 2.2,
                    saturation: 1.1,
                    brightness: 1.0,
                    contrast: 1.0,
                    color_temperature: 6500,
                    night_mode: false,
                    custom_lut: None,
                },
                performance_profile: PerformanceProfile {
                    name: "Quality".to_string(),
                    cpu_governor: Some("powersave".to_string()),
                    gpu_performance_level: Some("auto".to_string()),
                    frame_pacing: true,
                    power_management: PowerManagement::Balanced,
                    thermal_throttling: true,
                    async_compute: true,
                    shader_cache_precompile: true,
                    memory_pool_size: Some(1 * 1024 * 1024 * 1024), // 1GB
                    thread_priority: Some(-5),
                },
                filter_settings: Default::default(),
                environment_variables: HashMap::new(),
            },
            GamescopePreset::Balanced => GamescopeConfig {
                width: 2560,
                height: 1440,
                refresh_rate: Some(120),
                upscaling: GamescopeUpscaling::Fsr,
                hdr_enabled: false,
                frame_limiter: Some(120),
                adaptive_sync: true,
                fullscreen: true,
                borderless: false,
                nvidia_optimizations: true,
                steam_deck_mode: false,
                hdr_metadata: Default::default(),
                color_management: ColorManagement {
                    enabled: false,
                    gamma_correction: 1.0,
                    saturation: 1.0,
                    brightness: 1.0,
                    contrast: 1.0,
                    color_temperature: 6500,
                    night_mode: false,
                    custom_lut: None,
                },
                performance_profile: PerformanceProfile {
                    name: "Balanced".to_string(),
                    cpu_governor: Some("schedutil".to_string()),
                    gpu_performance_level: Some("auto".to_string()),
                    frame_pacing: true,
                    power_management: PowerManagement::Balanced,
                    thermal_throttling: true,
                    async_compute: true,
                    shader_cache_precompile: true,
                    memory_pool_size: Some(512 * 1024 * 1024), // 512MB
                    thread_priority: Some(0),
                },
                filter_settings: Default::default(),
                environment_variables: HashMap::new(),
            },
            GamescopePreset::SteamDeckDocked => GamescopeConfig {
                width: 1920,
                height: 1080,
                refresh_rate: Some(60),
                upscaling: GamescopeUpscaling::Fsr,
                hdr_enabled: false,
                frame_limiter: Some(60),
                adaptive_sync: true,
                fullscreen: true,
                borderless: false,
                nvidia_optimizations: false,
                steam_deck_mode: true,
                hdr_metadata: Default::default(),
                color_management: Default::default(),
                performance_profile: PerformanceProfile {
                    name: "Steam Deck Docked".to_string(),
                    cpu_governor: Some("schedutil".to_string()),
                    gpu_performance_level: Some("auto".to_string()),
                    frame_pacing: true,
                    power_management: PowerManagement::Balanced,
                    thermal_throttling: true,
                    async_compute: true,
                    shader_cache_precompile: true,
                    memory_pool_size: Some(512 * 1024 * 1024), // 512MB
                    thread_priority: Some(0),
                },
                filter_settings: Default::default(),
                environment_variables: HashMap::new(),
            },
            GamescopePreset::SteamDeckHandheld => GamescopeConfig {
                width: 1280,
                height: 800,
                refresh_rate: Some(60),
                upscaling: GamescopeUpscaling::Linear,
                hdr_enabled: false,
                frame_limiter: Some(40),
                adaptive_sync: false,
                fullscreen: true,
                borderless: false,
                nvidia_optimizations: false,
                steam_deck_mode: true,
                hdr_metadata: Default::default(),
                color_management: Default::default(),
                performance_profile: PerformanceProfile {
                    name: "Steam Deck Handheld".to_string(),
                    cpu_governor: Some("powersave".to_string()),
                    gpu_performance_level: Some("min".to_string()),
                    frame_pacing: true,
                    power_management: PowerManagement::PowerSaver,
                    thermal_throttling: true,
                    async_compute: true,
                    shader_cache_precompile: true,
                    memory_pool_size: Some(256 * 1024 * 1024), // 256MB
                    thread_priority: Some(0),
                },
                filter_settings: Default::default(),
                environment_variables: HashMap::new(),
            },
            GamescopePreset::SteamDeck => GamescopeConfig {
                width: 1280,
                height: 800,
                refresh_rate: Some(60),
                upscaling: GamescopeUpscaling::Fsr,
                hdr_enabled: false,
                frame_limiter: Some(60),
                adaptive_sync: true,
                fullscreen: true,
                borderless: false,
                nvidia_optimizations: false,
                steam_deck_mode: true,
                hdr_metadata: Default::default(),
                color_management: Default::default(),
                performance_profile: PerformanceProfile {
                    name: "Steam Deck".to_string(),
                    cpu_governor: Some("schedutil".to_string()),
                    gpu_performance_level: Some("auto".to_string()),
                    frame_pacing: true,
                    power_management: PowerManagement::Balanced,
                    thermal_throttling: true,
                    async_compute: true,
                    shader_cache_precompile: true,
                    memory_pool_size: Some(512 * 1024 * 1024), // 512MB
                    thread_priority: Some(0),
                },
                filter_settings: Default::default(),
                environment_variables: HashMap::new(),
            },
            GamescopePreset::Handheld1080p => GamescopeConfig {
                width: 1920,
                height: 1080,
                refresh_rate: Some(60),
                upscaling: GamescopeUpscaling::Fsr,
                hdr_enabled: false,
                frame_limiter: Some(60),
                adaptive_sync: true,
                fullscreen: true,
                borderless: false,
                nvidia_optimizations: true,
                steam_deck_mode: false,
                hdr_metadata: Default::default(),
                color_management: Default::default(),
                performance_profile: PerformanceProfile {
                    name: "Handheld 1080p".to_string(),
                    cpu_governor: Some("schedutil".to_string()),
                    gpu_performance_level: Some("auto".to_string()),
                    frame_pacing: true,
                    power_management: PowerManagement::Balanced,
                    thermal_throttling: true,
                    async_compute: true,
                    shader_cache_precompile: true,
                    memory_pool_size: Some(512 * 1024 * 1024), // 512MB
                    thread_priority: Some(0),
                },
                filter_settings: Default::default(),
                environment_variables: HashMap::new(),
            },
            GamescopePreset::Desktop => GamescopeConfig {
                width: 1920,
                height: 1080,
                refresh_rate: Some(144),
                upscaling: GamescopeUpscaling::Nis,
                hdr_enabled: false,
                frame_limiter: None,
                adaptive_sync: true,
                fullscreen: true,
                borderless: false,
                nvidia_optimizations: true,
                steam_deck_mode: false,
                hdr_metadata: Default::default(),
                color_management: Default::default(),
                performance_profile: PerformanceProfile {
                    name: "Desktop".to_string(),
                    cpu_governor: Some("performance".to_string()),
                    gpu_performance_level: Some("max".to_string()),
                    frame_pacing: false,
                    power_management: PowerManagement::Performance,
                    thermal_throttling: false,
                    async_compute: true,
                    shader_cache_precompile: true,
                    memory_pool_size: Some(1024 * 1024 * 1024), // 1GB
                    thread_priority: Some(-10),
                },
                filter_settings: Default::default(),
                environment_variables: HashMap::new(),
            },
            GamescopePreset::Custom(config) => config.clone(),
            // Add other presets with similar configs...
            _ => self.get_default_config(),
        }
    }

    fn get_default_config(&self) -> GamescopeConfig {
        GamescopeConfig {
            width: 1920,
            height: 1080,
            refresh_rate: Some(60),
            upscaling: GamescopeUpscaling::Linear,
            hdr_enabled: false,
            frame_limiter: Some(60),
            adaptive_sync: false,
            fullscreen: true,
            borderless: false,
            nvidia_optimizations: true,
            steam_deck_mode: false,
            hdr_metadata: Default::default(),
            color_management: Default::default(),
            performance_profile: Default::default(),
            filter_settings: Default::default(),
            environment_variables: HashMap::new(),
        }
    }
}

/// Generate advanced command arguments
pub fn generate_advanced_command(config: &GamescopeConfig, command: &str) -> Vec<String> {
    let mut args = vec![];

    // Basic settings
    args.push("-w".to_string());
    args.push(config.width.to_string());
    args.push("-h".to_string());
    args.push(config.height.to_string());

    if let Some(refresh) = config.refresh_rate {
        args.push("-r".to_string());
        args.push(refresh.to_string());
    }

    // Upscaling
    args.push("-F".to_string());
    args.push(config.upscaling.as_str().to_string());

    // HDR
    if config.hdr_enabled {
        args.push("--hdr-enabled".to_string());
    }

    // Frame limiter
    if let Some(limit) = config.frame_limiter {
        args.push("--fps-limit".to_string());
        args.push(limit.to_string());
    }

    // Add the command to execute
    args.push("--".to_string());
    args.extend(command.split_whitespace().map(|s| s.to_string()));

    args
}

/// Create Steam Deck presets
pub fn create_steam_deck_presets() -> Vec<GamescopePreset> {
    vec![
        GamescopePreset::SteamDeckHandheld,
        GamescopePreset::SteamDeckDocked,
    ]
}

/// Get recording status as a formatted string
pub fn get_recording_status() -> NvResult<String> {
    match crate::recording::get_recording_pid()? {
        Some(pid) => Ok(format!("Recording active (PID: {})", pid)),
        None => Ok("No active recording".to_string()),
    }
}

/// Get recording presets as a list
pub fn get_recording_presets() -> NvResult<Vec<crate::recording::RecordingSettings>> {
    crate::recording::load_recording_presets()
}

/// Apply a Gamescope configuration (saves to config file for persistence)
pub fn apply_gamescope_config(config: &GamescopeConfig) -> NvResult<()> {
    use crate::NvControlError;
    use std::fs;

    // Get config directory
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("Failed to get config directory".to_string()))?
        .join("nvcontrol");

    // Create directory if it doesn't exist
    fs::create_dir_all(&config_dir).map_err(|e| {
        NvControlError::ConfigError(format!("Failed to create config directory: {}", e))
    })?;

    let config_path = config_dir.join("gamescope.toml");

    // Serialize config to TOML
    let toml_string = toml::to_string_pretty(&config)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to serialize config: {}", e)))?;

    // Write to file
    fs::write(&config_path, toml_string)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to write config file: {}", e)))?;

    // Also apply environment variables immediately if requested
    if config.nvidia_optimizations {
        for (key, value) in &config.environment_variables {
            unsafe {
                std::env::set_var(key, value);
            }
        }
    }

    println!(
        "✅ Gamescope configuration saved to: {}",
        config_path.display()
    );
    println!(
        "   Resolution: {}x{}@{}Hz",
        config.width,
        config.height,
        config.refresh_rate.unwrap_or(60)
    );
    println!("   Upscaling: {:?}", config.upscaling);
    println!(
        "   HDR: {}",
        if config.hdr_enabled {
            "enabled"
        } else {
            "disabled"
        }
    );
    println!(
        "   Adaptive Sync: {}",
        if config.adaptive_sync {
            "enabled"
        } else {
            "disabled"
        }
    );

    Ok(())
}

/// Load the saved Gamescope configuration
pub fn load_gamescope_config() -> NvResult<GamescopeConfig> {
    use crate::NvControlError;
    use std::fs;

    let config_path = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("Failed to get config directory".to_string()))?
        .join("nvcontrol")
        .join("gamescope.toml");

    if !config_path.exists() {
        return Ok(GamescopeConfig::default());
    }

    let toml_string = fs::read_to_string(&config_path)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to read config file: {}", e)))?;

    let config: GamescopeConfig = toml::from_str(&toml_string)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to parse config: {}", e)))?;

    Ok(config)
}

/// Launch an application with Gamescope using the current config
pub fn launch_with_gamescope(command: &str, config: Option<&GamescopeConfig>) -> NvResult<()> {
    use crate::NvControlError;
    use std::process::Command;

    let config = match config {
        Some(c) => c.clone(),
        None => load_gamescope_config()?,
    };

    // Build gamescope command
    let mut cmd = Command::new("gamescope");

    // Basic resolution and refresh rate
    cmd.args(&["-w", &config.width.to_string()]);
    cmd.args(&["-h", &config.height.to_string()]);

    if let Some(rate) = config.refresh_rate {
        cmd.args(&["-r", &rate.to_string()]);
    }

    // Upscaling
    cmd.args(&["-F", config.upscaling.as_str()]);

    // FSR sharpness if applicable
    if let Some(sharpness) = config.upscaling.get_sharpness() {
        cmd.args(&["--fsr-sharpness", &sharpness.to_string()]);
    }

    // HDR
    if config.hdr_enabled {
        cmd.arg("--hdr-enabled");
    }

    // Frame limiter
    if let Some(limit) = config.frame_limiter {
        cmd.args(&["--fps-limit", &limit.to_string()]);
    }

    // Adaptive sync / VRR
    if config.adaptive_sync {
        cmd.arg("--adaptive-sync");
    }

    // Fullscreen / borderless
    if config.fullscreen {
        cmd.arg("-f");
    }
    if config.borderless {
        cmd.arg("-b");
    }

    // Steam Deck mode
    if config.steam_deck_mode {
        cmd.arg("--steam");
    }

    // Set environment variables
    if config.nvidia_optimizations {
        for (key, value) in &config.environment_variables {
            cmd.env(key, value);
        }
    }

    // Add the command to execute
    cmd.arg("--");
    cmd.args(command.split_whitespace());

    println!("🚀 Launching with Gamescope: {}", command);
    println!(
        "   Config: {}x{}@{}Hz",
        config.width,
        config.height,
        config.refresh_rate.unwrap_or(60)
    );

    // Execute
    let status = cmd
        .status()
        .map_err(|e| NvControlError::CommandFailed(format!("Failed to launch gamescope: {}", e)))?;

    if !status.success() {
        return Err(NvControlError::CommandFailed(format!(
            "Gamescope exited with error code: {:?}",
            status.code()
        )));
    }

    Ok(())
}
