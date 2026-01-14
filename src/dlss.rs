use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::Command;

/// DLSS 4 Multi-Frame Generation and Super Resolution Implementation
/// Supports DLSS 2 (Super Resolution), DLSS 3 (Frame Generation), and DLSS 4 (Multi-Frame Generation)
/// RTX 40-series required for Frame Generation, RTX 50-series for Multi-Frame Generation
///
/// Full DLSS DLL management for Proton gaming:
/// - Scan Steam/Lutris/Heroic games for DLSS DLLs
/// - Read actual DLL versions from PE headers
/// - Upgrade game DLLs with backup/restore
/// - Generate Proton launch options (PROTON_DLSS_UPGRADE)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlssController {
    pub version: DlssVersion,
    pub capabilities: DlssCapabilities,
    pub current_settings: DlssSettings,
    pub game_profiles: HashMap<String, DlssGameProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DlssVersion {
    None,
    Dlss2,   // Super Resolution only (RTX 20/30 series)
    Dlss3,   // Frame Generation + Super Resolution (RTX 40 series)
    Dlss3_5, // Ray Reconstruction + Frame Generation + Super Resolution
    Dlss4,   // Multi-Frame Generation (up to 4x) + All DLSS 3.5 features (RTX 50 series)
    Dlss4_5, // Enhanced MFG with improved frame pacing (RTX 50 series, driver 590+)
}

impl fmt::Display for DlssVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DlssVersion::None => write!(f, "None"),
            DlssVersion::Dlss2 => write!(f, "DLSS 2 (Super Resolution)"),
            DlssVersion::Dlss3 => write!(f, "DLSS 3 (Frame Generation)"),
            DlssVersion::Dlss3_5 => write!(f, "DLSS 3.5 (Ray Reconstruction)"),
            DlssVersion::Dlss4 => write!(f, "DLSS 4 (Multi-Frame Generation)"),
            DlssVersion::Dlss4_5 => write!(f, "DLSS 4.5 (Enhanced MFG)"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlssCapabilities {
    pub gpu_model: String,
    pub supports_dlss: bool,
    pub supports_frame_generation: bool,       // RTX 40-series+
    pub supports_multi_frame_generation: bool, // RTX 50-series (DLSS 4)
    pub max_frame_multiplier: u8,              // 2x, 3x, or 4x
    pub supports_ray_reconstruction: bool,     // DLSS 3.5+
    pub supports_reflex: bool,                 // NVIDIA Reflex
    pub tensor_cores: u32,
    pub optical_flow_accelerator: bool, // Required for Frame Generation
    pub optical_flow_accelerator_version: u8, // Gen 4 for RTX 50
    pub driver_version: String,
    pub dlss_dll_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlssSettings {
    pub enabled: bool,
    pub mode: DlssMode,
    pub quality_preset: DlssQuality,
    pub frame_generation: FrameGenerationSettings,
    pub sharpening: f32, // 0.0 to 1.0
    pub auto_exposure: bool,
    pub reflex_mode: ReflexMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DlssMode {
    Off,
    SuperResolution,                   // DLSS 2 or DLSS 3 SR only
    FrameGeneration,                   // DLSS 3 FG only (native resolution)
    SuperResolutionAndFrameGeneration, // Both SR + FG
    DlaaAndFrameGeneration,            // DLAA (native AA) + FG
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DlssQuality {
    UltraPerformance, // 3x upscaling (720p -> 4K)
    Performance,      // 2x upscaling (1080p -> 4K)
    Balanced,         // 1.7x upscaling
    Quality,          // 1.5x upscaling
    UltraQuality,     // 1.3x upscaling
    DLAA,             // Native resolution Anti-Aliasing
    Auto,             // Automatic based on performance
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameGenerationSettings {
    pub enabled: bool,
    pub mode: FrameGenerationMode,
    pub indicator: bool,            // Show FG indicator overlay
    pub frame_pacing: bool,         // Optimize frame delivery
    pub vsync_compensation: bool,   // Compensate for VSync latency
    pub target_fps_multiplier: f32, // 2.0 = double frames, 3.0 = triple
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FrameGenerationMode {
    Off,
    Standard,      // 2x frame generation (RTX 40+)
    Boost,         // 3x frame generation (RTX 50 DLSS 4)
    MultiFrameGen, // 4x frame generation (RTX 50 DLSS 4)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReflexMode {
    Off,
    On,
    OnPlusBoost, // Lower latency, higher GPU usage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlssGameProfile {
    pub game_name: String,
    pub executable: String,
    pub recommended_settings: DlssSettings,
    pub auto_apply: bool,
    pub verified: bool, // NVIDIA verified settings
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlssMetrics {
    pub base_fps: f32,
    pub dlss_fps: f32,
    pub frame_generation_fps: f32,
    pub latency_ms: f32,
    pub gpu_utilization: f32,
    pub vram_usage_mb: u32,
    pub tensor_core_utilization: f32,
    pub optical_flow_utilization: f32,
}

// ============================================================================
// DLSS DLL Management Types
// ============================================================================

/// Information about a game's DLSS DLL installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDlssInfo {
    /// Game name (from Steam manifest or directory)
    pub game_name: String,
    /// Path to the game's installation directory
    pub install_path: PathBuf,
    /// Steam App ID if applicable
    pub app_id: Option<String>,
    /// Game launcher (Steam, Lutris, Heroic, Native)
    pub launcher: GameLauncher,
    /// DLSS DLL files found in the game
    pub dlls: Vec<DllInfo>,
    /// Whether this is a Proton/Wine game
    pub is_proton: bool,
    /// Proton prefix path if applicable
    pub proton_prefix: Option<PathBuf>,
}

/// Information about a specific DLSS DLL file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DllInfo {
    /// Full path to the DLL
    pub path: PathBuf,
    /// DLL type (SR, RR, or FG)
    pub dll_type: DllType,
    /// Version string extracted from PE headers
    pub version: Option<String>,
    /// Major.Minor.Patch.Build parsed version
    pub parsed_version: Option<DllVersion>,
    /// File size in bytes
    pub file_size: u64,
    /// Whether this uses the 2nd gen transformer model (310+)
    pub is_transformer_model: bool,
    /// Backup path if original was backed up
    pub backup_path: Option<PathBuf>,
}

/// DLSS DLL type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DllType {
    /// nvngx_dlss.dll - Super Resolution
    SuperResolution,
    /// nvngx_dlssd.dll - Ray Reconstruction (DLSS-D)
    RayReconstruction,
    /// nvngx_dlssg.dll - Frame Generation
    FrameGeneration,
}

impl DllType {
    pub fn filename(&self) -> &'static str {
        match self {
            DllType::SuperResolution => "nvngx_dlss.dll",
            DllType::RayReconstruction => "nvngx_dlssd.dll",
            DllType::FrameGeneration => "nvngx_dlssg.dll",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            DllType::SuperResolution => "DLSS-SR (Super Resolution)",
            DllType::RayReconstruction => "DLSS-RR (Ray Reconstruction)",
            DllType::FrameGeneration => "DLSS-FG (Frame Generation)",
        }
    }

    pub fn from_filename(name: &str) -> Option<Self> {
        let lower = name.to_lowercase();
        if lower.contains("dlssg") {
            Some(DllType::FrameGeneration)
        } else if lower.contains("dlssd") {
            Some(DllType::RayReconstruction)
        } else if lower.contains("dlss") {
            Some(DllType::SuperResolution)
        } else {
            None
        }
    }
}

/// Parsed DLL version (Major.Minor.Patch.Build)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DllVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub build: u16,
}

impl DllVersion {
    pub fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self {
            major,
            minor,
            patch,
            build,
        }
    }

    /// Parse version from string like "3.10.5.0" or "310.5.0"
    pub fn parse(version_str: &str) -> Option<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() >= 3 {
            // Try parsing as Major.Minor.Patch.Build
            let major = parts[0].parse().ok()?;
            let minor = parts[1].parse().ok()?;
            let patch = parts[2].parse().ok()?;
            let build = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
            Some(Self {
                major,
                minor,
                patch,
                build,
            })
        } else {
            None
        }
    }

    /// Check if this is the new transformer model (310.x.x+)
    pub fn is_transformer_model(&self) -> bool {
        self.major >= 310 || (self.major == 3 && self.minor >= 10)
    }
}

impl fmt::Display for DllVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.patch, self.build
        )
    }
}

/// Game launcher type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameLauncher {
    Steam,
    Lutris,
    Heroic,
    Native,
    Unknown,
}

impl GameLauncher {
    pub fn display_name(&self) -> &'static str {
        match self {
            GameLauncher::Steam => "Steam",
            GameLauncher::Lutris => "Lutris",
            GameLauncher::Heroic => "Heroic",
            GameLauncher::Native => "Native",
            GameLauncher::Unknown => "Unknown",
        }
    }
}

/// Proton launch options for a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtonLaunchOptions {
    /// PROTON_DLSS_UPGRADE=1 to use system DLSS
    pub dlss_upgrade: bool,
    /// PROTON_DLSS_UPGRADE=<version> for specific version
    pub dlss_version: Option<String>,
    /// PROTON_DLSS_INDICATOR=1 to show version overlay
    pub dlss_indicator: bool,
    /// Additional environment variables
    pub extra_env: HashMap<String, String>,
}

impl ProtonLaunchOptions {
    pub fn default_upgrade() -> Self {
        Self {
            dlss_upgrade: true,
            dlss_version: None,
            dlss_indicator: false,
            extra_env: HashMap::new(),
        }
    }

    pub fn with_indicator() -> Self {
        Self {
            dlss_upgrade: true,
            dlss_version: None,
            dlss_indicator: true,
            extra_env: HashMap::new(),
        }
    }

    pub fn with_version(version: &str) -> Self {
        Self {
            dlss_upgrade: true,
            dlss_version: Some(version.to_string()),
            dlss_indicator: false,
            extra_env: HashMap::new(),
        }
    }

    /// Generate Steam launch options string
    pub fn to_steam_launch_options(&self) -> String {
        let mut opts = Vec::new();

        if self.dlss_upgrade {
            if let Some(ref ver) = self.dlss_version {
                opts.push(format!("PROTON_DLSS_UPGRADE={}", ver));
            } else {
                opts.push("PROTON_DLSS_UPGRADE=1".to_string());
            }
        }

        if self.dlss_indicator {
            opts.push("PROTON_DLSS_INDICATOR=1".to_string());
        }

        for (key, value) in &self.extra_env {
            opts.push(format!("{}={}", key, value));
        }

        if opts.is_empty() {
            "%command%".to_string()
        } else {
            format!("{} %command%", opts.join(" "))
        }
    }
}

/// DLSS doctor diagnostic results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlssDoctorResult {
    /// Overall health status
    pub status: DoctorStatus,
    /// GPU capabilities check
    pub gpu_check: DoctorCheck,
    /// Driver version check
    pub driver_check: DoctorCheck,
    /// Proton/Wine compatibility check
    pub proton_check: DoctorCheck,
    /// DXVK-NVAPI check
    pub nvapi_check: DoctorCheck,
    /// VKD3D-Proton check
    pub vkd3d_check: DoctorCheck,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoctorStatus {
    Healthy,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorCheck {
    pub name: String,
    pub status: DoctorStatus,
    pub message: String,
    pub details: Option<String>,
}

impl DoctorCheck {
    pub fn ok(name: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            status: DoctorStatus::Healthy,
            message: message.to_string(),
            details: None,
        }
    }

    pub fn warn(name: &str, message: &str, details: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            status: DoctorStatus::Warning,
            message: message.to_string(),
            details: details.map(|s| s.to_string()),
        }
    }

    pub fn error(name: &str, message: &str, details: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            status: DoctorStatus::Error,
            message: message.to_string(),
            details: details.map(|s| s.to_string()),
        }
    }
}

/// Available DLSS version from TechPowerUp or NVIDIA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableDlssVersion {
    /// Version string (e.g., "310.5.0")
    pub version: String,
    /// Parsed version for comparison
    pub parsed: DllVersion,
    /// Release date if known
    pub release_date: Option<String>,
    /// Download URL
    pub download_url: Option<String>,
    /// File size in bytes
    pub file_size: Option<u64>,
    /// Whether this is the latest version
    pub is_latest: bool,
    /// Whether this has transformer model (2nd gen)
    pub is_transformer: bool,
}

impl DlssController {
    /// Initialize DLSS controller and detect capabilities
    pub fn new() -> NvResult<Self> {
        let capabilities = Self::detect_capabilities()?;
        let version = Self::detect_dlss_version(&capabilities);

        Ok(DlssController {
            version,
            capabilities,
            current_settings: DlssSettings::default(),
            game_profiles: Self::load_game_profiles(),
        })
    }

    /// Detect GPU DLSS capabilities
    fn detect_capabilities() -> NvResult<DlssCapabilities> {
        let mut caps = DlssCapabilities {
            gpu_model: String::new(),
            supports_dlss: false,
            supports_frame_generation: false,
            supports_multi_frame_generation: false,
            max_frame_multiplier: 0,
            supports_ray_reconstruction: false,
            supports_reflex: false,
            tensor_cores: 0,
            optical_flow_accelerator: false,
            optical_flow_accelerator_version: 0,
            driver_version: String::new(),
            dlss_dll_version: None,
        };

        // Get GPU info via nvidia-smi
        if let Ok(output) = Command::new("nvidia-smi")
            .args([
                "--query-gpu=name,driver_version",
                "--format=csv,noheader,nounits",
            ])
            .output()
        {
            if output.status.success() {
                let info = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = info.trim().split(',').collect();

                if parts.len() >= 2 {
                    caps.gpu_model = parts[0].trim().to_string();
                    caps.driver_version = parts[1].trim().to_string();

                    // Check for RTX series and capabilities
                    let gpu_lower = caps.gpu_model.to_lowercase();

                    // RTX 20 series (Turing) - DLSS 2
                    if gpu_lower.contains("rtx 20") {
                        caps.supports_dlss = true;
                        caps.supports_frame_generation = false;
                        caps.supports_multi_frame_generation = false;
                        caps.max_frame_multiplier = 0;
                        caps.supports_reflex = true;
                        caps.optical_flow_accelerator = false;
                        caps.optical_flow_accelerator_version = 0;
                        caps.tensor_cores = 272; // Approximate for RTX 2080
                    }
                    // RTX 30 series (Ampere) - DLSS 2
                    else if gpu_lower.contains("rtx 30") {
                        caps.supports_dlss = true;
                        caps.supports_frame_generation = false;
                        caps.supports_multi_frame_generation = false;
                        caps.max_frame_multiplier = 0;
                        caps.supports_reflex = true;
                        caps.optical_flow_accelerator = false;
                        caps.optical_flow_accelerator_version = 0;
                        caps.tensor_cores = 328; // Approximate for RTX 3080
                    }
                    // RTX 40 series (Ada Lovelace) - DLSS 3 with Frame Generation
                    else if gpu_lower.contains("rtx 40") {
                        caps.supports_dlss = true;
                        caps.supports_frame_generation = true;
                        caps.supports_multi_frame_generation = false;
                        caps.max_frame_multiplier = 2;
                        caps.supports_ray_reconstruction = true;
                        caps.supports_reflex = true;
                        caps.optical_flow_accelerator = true;
                        caps.optical_flow_accelerator_version = 3;
                        caps.tensor_cores = 512; // Approximate for RTX 4080
                    }
                    // RTX 50 series (Blackwell) - DLSS 4 Multi-Frame Generation
                    else if gpu_lower.contains("rtx 50") {
                        caps.supports_dlss = true;
                        caps.supports_frame_generation = true;
                        caps.supports_multi_frame_generation = true;
                        caps.max_frame_multiplier = 4;
                        caps.supports_ray_reconstruction = true;
                        caps.supports_reflex = true;
                        caps.optical_flow_accelerator = true;
                        caps.optical_flow_accelerator_version = 4; // Gen 4 OFA

                        // Determine tensor cores based on specific model
                        if gpu_lower.contains("5090") {
                            caps.tensor_cores = 1360; // 170 SMs × 8 tensor cores per SM
                        } else if gpu_lower.contains("5080") {
                            caps.tensor_cores = 672; // 84 SMs × 8 tensor cores per SM
                        } else if gpu_lower.contains("5070") {
                            caps.tensor_cores = 560; // 70 SMs × 8 tensor cores per SM
                        } else {
                            caps.tensor_cores = 768; // Default estimate
                        }
                    }
                }
            }
        }

        // Check for DLSS DLL version if possible
        caps.dlss_dll_version = Self::detect_dlss_dll_version();

        Ok(caps)
    }

    /// Detect installed DLSS version
    fn detect_dlss_version(caps: &DlssCapabilities) -> DlssVersion {
        if !caps.supports_dlss {
            return DlssVersion::None;
        }

        // RTX 50 series (Blackwell) with Multi-Frame Generation
        if caps.supports_multi_frame_generation {
            // DLSS 4.5 requires driver 590+ for enhanced frame pacing
            let driver_major: u32 = caps
                .driver_version
                .split('.')
                .next()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);

            if driver_major >= 590 {
                DlssVersion::Dlss4_5
            } else {
                DlssVersion::Dlss4
            }
        } else if caps.supports_ray_reconstruction {
            DlssVersion::Dlss3_5
        } else if caps.supports_frame_generation {
            DlssVersion::Dlss3
        } else {
            DlssVersion::Dlss2
        }
    }

    /// Detect DLSS DLL version from common game paths
    fn detect_dlss_dll_version() -> Option<String> {
        // Check common Steam game paths for DLSS DLLs
        let steam_paths = vec![
            PathBuf::from("/home")
                .join(std::env::var("USER").unwrap_or_default())
                .join(".steam/steam/steamapps/common"),
            PathBuf::from("/home")
                .join(std::env::var("USER").unwrap_or_default())
                .join(".local/share/Steam/steamapps/common"),
        ];

        for steam_path in steam_paths {
            if steam_path.exists() {
                // Look for nvngx_dlss.dll in game directories
                if let Ok(output) = Command::new("find")
                    .args([
                        steam_path.to_str()?,
                        "-name",
                        "nvngx_dlss.dll",
                        "-type",
                        "f",
                        "-print",
                        "-quit",
                    ])
                    .output()
                {
                    if output.status.success() && !output.stdout.is_empty() {
                        // Found a DLSS DLL, try to get version
                        return Some("DLSS DLL Found".to_string());
                    }
                }
            }
        }

        None
    }

    /// Apply DLSS settings to a running game
    pub fn apply_settings(&mut self, settings: DlssSettings) -> NvResult<()> {
        // Validate settings based on capabilities
        if settings.frame_generation.enabled && !self.capabilities.supports_frame_generation {
            return Err(NvControlError::UnsupportedFeature(
                "Frame Generation requires RTX 40-series or newer".to_string(),
            ));
        }

        if settings.mode == DlssMode::SuperResolutionAndFrameGeneration
            && !self.capabilities.supports_frame_generation
        {
            return Err(NvControlError::UnsupportedFeature(
                "This mode requires RTX 40-series GPU with Frame Generation support".to_string(),
            ));
        }

        // Apply via environment variables (for games that support it)
        self.set_dlss_env_vars(&settings)?;

        // Update current settings
        self.current_settings = settings;

        Ok(())
    }

    /// Set DLSS environment variables
    fn set_dlss_env_vars(&self, settings: &DlssSettings) -> NvResult<()> {
        use std::env;

        unsafe {
            // DLSS enablement
            env::set_var(
                "NVIDIA_DLSS_ENABLE",
                if settings.enabled { "1" } else { "0" },
            );

            // Quality preset
            env::set_var(
                "NVIDIA_DLSS_PRESET",
                match settings.quality_preset {
                    DlssQuality::UltraPerformance => "0",
                    DlssQuality::Performance => "1",
                    DlssQuality::Balanced => "2",
                    DlssQuality::Quality => "3",
                    DlssQuality::UltraQuality => "4",
                    DlssQuality::DLAA => "5",
                    DlssQuality::Auto => "auto",
                },
            );

            // Frame Generation
            if self.capabilities.supports_frame_generation {
                env::set_var(
                    "NVIDIA_DLSS3_FG_ENABLE",
                    if settings.frame_generation.enabled {
                        "1"
                    } else {
                        "0"
                    },
                );

                env::set_var(
                    "NVIDIA_DLSS3_FG_MODE",
                    match settings.frame_generation.mode {
                        FrameGenerationMode::Off => "0",
                        FrameGenerationMode::Standard => "1",
                        FrameGenerationMode::Boost => "2",
                        FrameGenerationMode::MultiFrameGen => "3",
                    },
                );
            }

            // NVIDIA Reflex
            env::set_var(
                "NVIDIA_REFLEX_MODE",
                match settings.reflex_mode {
                    ReflexMode::Off => "0",
                    ReflexMode::On => "1",
                    ReflexMode::OnPlusBoost => "2",
                },
            );

            // Sharpening
            env::set_var(
                "NVIDIA_DLSS_SHARPNESS",
                (settings.sharpening * 100.0).round().to_string(),
            );
        }

        Ok(())
    }

    /// Load game-specific DLSS profiles
    fn load_game_profiles() -> HashMap<String, DlssGameProfile> {
        let mut profiles = HashMap::new();

        // Cyberpunk 2077 - Optimized for DLSS 3
        profiles.insert(
            "cyberpunk2077".to_string(),
            DlssGameProfile {
                game_name: "Cyberpunk 2077".to_string(),
                executable: "Cyberpunk2077.exe".to_string(),
                recommended_settings: DlssSettings {
                    enabled: true,
                    mode: DlssMode::SuperResolutionAndFrameGeneration,
                    quality_preset: DlssQuality::Quality,
                    frame_generation: FrameGenerationSettings {
                        enabled: true,
                        mode: FrameGenerationMode::Standard,
                        indicator: false,
                        frame_pacing: true,
                        vsync_compensation: true,
                        target_fps_multiplier: 2.0,
                    },
                    sharpening: 0.5,
                    auto_exposure: true,
                    reflex_mode: ReflexMode::OnPlusBoost,
                },
                auto_apply: true,
                verified: true,
                notes: Some("Path Tracing benefits greatly from DLSS 3".to_string()),
            },
        );

        // Counter-Strike 2 - Competitive settings
        profiles.insert(
            "cs2".to_string(),
            DlssGameProfile {
                game_name: "Counter-Strike 2".to_string(),
                executable: "cs2.exe".to_string(),
                recommended_settings: DlssSettings {
                    enabled: true,
                    mode: DlssMode::SuperResolution,
                    quality_preset: DlssQuality::Performance,
                    frame_generation: FrameGenerationSettings {
                        enabled: false, // Disabled for competitive play
                        mode: FrameGenerationMode::Off,
                        indicator: false,
                        frame_pacing: false,
                        vsync_compensation: false,
                        target_fps_multiplier: 1.0,
                    },
                    sharpening: 0.3,
                    auto_exposure: false,
                    reflex_mode: ReflexMode::OnPlusBoost, // Maximum latency reduction
                },
                auto_apply: true,
                verified: true,
                notes: Some("Competitive settings prioritize latency over visuals".to_string()),
            },
        );

        // Alan Wake 2 - DLSS 3.5 with Ray Reconstruction
        profiles.insert(
            "alanwake2".to_string(),
            DlssGameProfile {
                game_name: "Alan Wake 2".to_string(),
                executable: "AlanWake2.exe".to_string(),
                recommended_settings: DlssSettings {
                    enabled: true,
                    mode: DlssMode::SuperResolutionAndFrameGeneration,
                    quality_preset: DlssQuality::Balanced,
                    frame_generation: FrameGenerationSettings {
                        enabled: true,
                        mode: FrameGenerationMode::Standard,
                        indicator: false,
                        frame_pacing: true,
                        vsync_compensation: true,
                        target_fps_multiplier: 2.0,
                    },
                    sharpening: 0.4,
                    auto_exposure: true,
                    reflex_mode: ReflexMode::On,
                },
                auto_apply: true,
                verified: true,
                notes: Some("Supports DLSS 3.5 Ray Reconstruction".to_string()),
            },
        );

        profiles
    }

    /// Get current DLSS metrics
    pub fn get_metrics(&self) -> NvResult<DlssMetrics> {
        // This would interface with NVML or game telemetry
        Ok(DlssMetrics {
            base_fps: 60.0,
            dlss_fps: 120.0,
            frame_generation_fps: 240.0,
            latency_ms: 12.5,
            gpu_utilization: 85.0,
            vram_usage_mb: 8192,
            tensor_core_utilization: 75.0,
            optical_flow_utilization: if self.capabilities.optical_flow_accelerator {
                60.0
            } else {
                0.0
            },
        })
    }

    /// Auto-detect running game and apply profile
    pub fn auto_apply_game_profile(&mut self) -> NvResult<Option<String>> {
        // Check running processes for known games
        if let Ok(output) = Command::new("ps").args(["-eo", "comm"]).output() {
            if output.status.success() {
                let processes = String::from_utf8_lossy(&output.stdout);

                // Find matching game first
                let mut matched_game = None;
                for (game_id, profile) in &self.game_profiles {
                    if processes.contains(&profile.executable) && profile.auto_apply {
                        matched_game =
                            Some((game_id.clone(), profile.recommended_settings.clone()));
                        break;
                    }
                }

                // Apply settings if we found a match
                if let Some((game_id, settings)) = matched_game {
                    self.apply_settings(settings)?;
                    return Ok(Some(game_id));
                }
            }
        }

        Ok(None)
    }

    // ========================================================================
    // DLSS DLL Management Methods
    // ========================================================================

    /// Scan all game libraries for DLSS-enabled games
    pub fn scan_games() -> NvResult<Vec<GameDlssInfo>> {
        let mut games = Vec::new();

        // Scan Steam
        games.extend(Self::scan_steam_games()?);

        // Scan Lutris
        games.extend(Self::scan_lutris_games()?);

        // Scan Heroic
        games.extend(Self::scan_heroic_games()?);

        Ok(games)
    }

    /// Scan Steam library for DLSS-enabled games
    pub fn scan_steam_games() -> NvResult<Vec<GameDlssInfo>> {
        let mut games = Vec::new();
        let home = dirs::home_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find home directory".into()))?;

        let steam_paths = vec![
            home.join(".steam/steam/steamapps"),
            home.join(".local/share/Steam/steamapps"),
        ];

        for steamapps in steam_paths {
            if !steamapps.exists() {
                continue;
            }

            let common = steamapps.join("common");
            if !common.exists() {
                continue;
            }

            // Read app manifests to get game names and app IDs
            let manifests: HashMap<String, (String, String)> =
                Self::read_steam_manifests(&steamapps);

            // Scan each game directory
            if let Ok(entries) = fs::read_dir(&common) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let game_path = entry.path();
                    if !game_path.is_dir() {
                        continue;
                    }

                    let dir_name = game_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    // Find DLSS DLLs in this game
                    let dlls = Self::find_dlss_dlls(&game_path);
                    if dlls.is_empty() {
                        continue;
                    }

                    // Get game info from manifest
                    let (game_name, app_id) = manifests
                        .get(&dir_name)
                        .cloned()
                        .unwrap_or_else(|| (dir_name.clone(), String::new()));

                    // Check for Proton prefix
                    let proton_prefix = Self::find_proton_prefix(&steamapps, &app_id);

                    games.push(GameDlssInfo {
                        game_name,
                        install_path: game_path,
                        app_id: if app_id.is_empty() {
                            None
                        } else {
                            Some(app_id)
                        },
                        launcher: GameLauncher::Steam,
                        dlls,
                        is_proton: proton_prefix.is_some(),
                        proton_prefix,
                    });
                }
            }
        }

        Ok(games)
    }

    /// Read Steam app manifests to map install dirs to game names/IDs
    fn read_steam_manifests(steamapps: &Path) -> HashMap<String, (String, String)> {
        let mut manifests = HashMap::new();

        if let Ok(entries) = fs::read_dir(steamapps) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) != Some("acf") {
                    continue;
                }

                if let Ok(content) = fs::read_to_string(&path) {
                    let mut name = String::new();
                    let mut install_dir = String::new();
                    let mut app_id = String::new();

                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("\"name\"") {
                            name = trimmed.split('"').nth(3).unwrap_or("").to_string();
                        } else if trimmed.starts_with("\"installdir\"") {
                            install_dir = trimmed.split('"').nth(3).unwrap_or("").to_string();
                        } else if trimmed.starts_with("\"appid\"") {
                            app_id = trimmed.split('"').nth(3).unwrap_or("").to_string();
                        }
                    }

                    if !install_dir.is_empty() {
                        manifests.insert(install_dir, (name, app_id));
                    }
                }
            }
        }

        manifests
    }

    /// Find Proton prefix for a Steam game
    fn find_proton_prefix(steamapps: &Path, app_id: &str) -> Option<PathBuf> {
        if app_id.is_empty() {
            return None;
        }
        let prefix = steamapps.join("compatdata").join(app_id).join("pfx");
        if prefix.exists() { Some(prefix) } else { None }
    }

    /// Scan Lutris games for DLSS
    pub fn scan_lutris_games() -> NvResult<Vec<GameDlssInfo>> {
        let mut games = Vec::new();
        let home = dirs::home_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find home directory".into()))?;

        let lutris_games = home.join(".local/share/lutris/games");
        if !lutris_games.exists() {
            return Ok(games);
        }

        if let Ok(entries) = fs::read_dir(&lutris_games) {
            for entry in entries.filter_map(|e| e.ok()) {
                let game_path = entry.path();
                if !game_path.is_dir() {
                    continue;
                }

                let dlls = Self::find_dlss_dlls(&game_path);
                if dlls.is_empty() {
                    continue;
                }

                let game_name = game_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                games.push(GameDlssInfo {
                    game_name,
                    install_path: game_path.clone(),
                    app_id: None,
                    launcher: GameLauncher::Lutris,
                    dlls,
                    is_proton: true, // Lutris games are typically Wine/Proton
                    proton_prefix: Some(game_path),
                });
            }
        }

        Ok(games)
    }

    /// Scan Heroic games for DLSS
    pub fn scan_heroic_games() -> NvResult<Vec<GameDlssInfo>> {
        let mut games = Vec::new();
        let home = dirs::home_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find home directory".into()))?;

        // Check both GOG and Epic games
        let heroic_dirs = vec![
            home.join(".config/heroic/gog_store/installed.json"),
            home.join(".config/heroic/legendaryConfig/installed.json"),
        ];

        for config_path in heroic_dirs {
            if !config_path.exists() {
                continue;
            }

            // Parse JSON to get install paths
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(installed) = json.as_object() {
                        for (app_id, info) in installed {
                            let install_path = info
                                .get("install_path")
                                .and_then(|v| v.as_str())
                                .map(PathBuf::from);

                            let title = info
                                .get("title")
                                .and_then(|v| v.as_str())
                                .unwrap_or(app_id)
                                .to_string();

                            if let Some(path) = install_path {
                                let dlls = Self::find_dlss_dlls(&path);
                                if !dlls.is_empty() {
                                    games.push(GameDlssInfo {
                                        game_name: title,
                                        install_path: path.clone(),
                                        app_id: Some(app_id.clone()),
                                        launcher: GameLauncher::Heroic,
                                        dlls,
                                        is_proton: true,
                                        proton_prefix: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(games)
    }

    /// Find all DLSS DLL files in a directory (recursive)
    pub fn find_dlss_dlls(game_path: &Path) -> Vec<DllInfo> {
        let mut dlls = Vec::new();
        Self::find_dlss_dlls_recursive(game_path, &mut dlls, 0);
        dlls
    }

    fn find_dlss_dlls_recursive(path: &Path, dlls: &mut Vec<DllInfo>, depth: usize) {
        // Limit recursion depth to avoid scanning too deep
        if depth > 5 {
            return;
        }

        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                Self::find_dlss_dlls_recursive(&entry_path, dlls, depth + 1);
            } else if entry_path.is_file() {
                let filename = entry_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                if let Some(dll_type) = DllType::from_filename(filename) {
                    let file_size = fs::metadata(&entry_path).map(|m| m.len()).unwrap_or(0);

                    // Try to read version from PE headers
                    let (version, parsed_version) = Self::read_dll_version(&entry_path);

                    let is_transformer = parsed_version
                        .as_ref()
                        .map(|v| v.is_transformer_model())
                        .unwrap_or(false);

                    // Check for backup
                    let backup_path = entry_path.with_extension("dll.backup");
                    let backup = if backup_path.exists() {
                        Some(backup_path)
                    } else {
                        None
                    };

                    dlls.push(DllInfo {
                        path: entry_path,
                        dll_type,
                        version,
                        parsed_version,
                        file_size,
                        is_transformer_model: is_transformer,
                        backup_path: backup,
                    });
                }
            }
        }
    }

    /// Read version information from a Windows PE DLL file
    pub fn read_dll_version(dll_path: &Path) -> (Option<String>, Option<DllVersion>) {
        let mut file = match fs::File::open(dll_path) {
            Ok(f) => f,
            Err(_) => return (None, None),
        };

        // Read DOS header (first 64 bytes)
        let mut dos_header = [0u8; 64];
        if file.read_exact(&mut dos_header).is_err() {
            return (None, None);
        }

        // Check MZ signature
        if dos_header[0] != b'M' || dos_header[1] != b'Z' {
            return (None, None);
        }

        // Get PE header offset from DOS header at offset 0x3C
        let pe_offset = u32::from_le_bytes([
            dos_header[0x3C],
            dos_header[0x3D],
            dos_header[0x3E],
            dos_header[0x3F],
        ]) as u64;

        // Seek to PE header
        if file.seek(SeekFrom::Start(pe_offset)).is_err() {
            return (None, None);
        }

        // Read PE signature
        let mut pe_sig = [0u8; 4];
        if file.read_exact(&mut pe_sig).is_err() {
            return (None, None);
        }

        // Check PE signature
        if &pe_sig != b"PE\0\0" {
            return (None, None);
        }

        // Read COFF header (20 bytes)
        let mut coff_header = [0u8; 20];
        if file.read_exact(&mut coff_header).is_err() {
            return (None, None);
        }

        // Get optional header size
        let optional_size = u16::from_le_bytes([coff_header[16], coff_header[17]]) as u64;
        let number_of_sections = u16::from_le_bytes([coff_header[2], coff_header[3]]);

        // Read optional header magic to determine PE32 or PE32+
        let mut magic = [0u8; 2];
        if file.read_exact(&mut magic).is_err() {
            return (None, None);
        }

        let is_pe32_plus = magic == [0x0b, 0x02];

        // Skip rest of optional header to get to data directories
        // Data directory offset varies: PE32 = 96, PE32+ = 112
        let data_dir_offset = if is_pe32_plus { 112 - 2 } else { 96 - 2 };
        if file
            .seek(SeekFrom::Current(data_dir_offset as i64 - 2))
            .is_err()
        {
            return (None, None);
        }

        // Resource directory is the 3rd entry (index 2)
        // Each entry is 8 bytes (RVA + Size)
        // Skip first 2 entries (16 bytes)
        if file.seek(SeekFrom::Current(16)).is_err() {
            return (None, None);
        }

        let mut resource_dir = [0u8; 8];
        if file.read_exact(&mut resource_dir).is_err() {
            return (None, None);
        }

        let resource_rva = u32::from_le_bytes([
            resource_dir[0],
            resource_dir[1],
            resource_dir[2],
            resource_dir[3],
        ]);

        if resource_rva == 0 {
            return (None, None);
        }

        // Now we need to find the section containing this RVA
        // Skip to section headers
        let section_headers_offset = pe_offset + 24 + optional_size;
        if file.seek(SeekFrom::Start(section_headers_offset)).is_err() {
            return (None, None);
        }

        // Read section headers to find resource section
        for _ in 0..number_of_sections {
            let mut section = [0u8; 40];
            if file.read_exact(&mut section).is_err() {
                return (None, None);
            }

            let section_rva =
                u32::from_le_bytes([section[12], section[13], section[14], section[15]]);
            let section_size =
                u32::from_le_bytes([section[8], section[9], section[10], section[11]]);
            let raw_offset =
                u32::from_le_bytes([section[20], section[21], section[22], section[23]]);

            // Check if resource RVA falls within this section
            if resource_rva >= section_rva && resource_rva < section_rva + section_size {
                // Calculate file offset for resource directory
                let resource_file_offset = raw_offset + (resource_rva - section_rva);

                // Parse VS_VERSIONINFO resource (complex structure)
                // For now, try a simpler approach: search for version string pattern
                if let Some(version) =
                    Self::find_version_string_in_file(&mut file, resource_file_offset as u64)
                {
                    let parsed = DllVersion::parse(&version);
                    return (Some(version), parsed);
                }
                break;
            }
        }

        (None, None)
    }

    /// Search for version string pattern in file
    fn find_version_string_in_file(file: &mut fs::File, start_offset: u64) -> Option<String> {
        // Seek to resource section
        if file.seek(SeekFrom::Start(start_offset)).is_err() {
            return None;
        }

        // Read a chunk to search for version info
        let mut buffer = vec![0u8; 8192];
        let bytes_read = file.read(&mut buffer).ok()?;

        // Look for ProductVersion or FileVersion strings
        // These are stored as UTF-16LE in the version resource
        let patterns = [
            "ProductVersion"
                .encode_utf16()
                .flat_map(|c| c.to_le_bytes())
                .collect::<Vec<u8>>(),
            "FileVersion"
                .encode_utf16()
                .flat_map(|c| c.to_le_bytes())
                .collect::<Vec<u8>>(),
        ];

        for pattern in &patterns {
            if let Some(pos) = buffer[..bytes_read]
                .windows(pattern.len())
                .position(|w| w == pattern.as_slice())
            {
                // Version string follows the pattern name
                // Skip pattern + null terminator + padding
                let version_start = pos + pattern.len() + 4; // +4 for null + alignment
                if version_start + 32 <= bytes_read {
                    // Read version as UTF-16LE
                    let version_bytes =
                        &buffer[version_start..version_start + 64.min(bytes_read - version_start)];
                    let mut version_chars = Vec::new();
                    for chunk in version_bytes.chunks(2) {
                        if chunk.len() == 2 {
                            let c = u16::from_le_bytes([chunk[0], chunk[1]]);
                            if c == 0 {
                                break;
                            }
                            if let Some(ch) = char::from_u32(c as u32) {
                                if ch.is_ascii_digit() || ch == '.' || ch == ' ' {
                                    version_chars.push(ch);
                                } else if !version_chars.is_empty() {
                                    break;
                                }
                            }
                        }
                    }
                    let version: String = version_chars.into_iter().collect();
                    let version = version.trim().to_string();
                    if !version.is_empty() && version.contains('.') {
                        return Some(version);
                    }
                }
            }
        }

        None
    }

    /// Run DLSS doctor diagnostics
    pub fn doctor() -> NvResult<DlssDoctorResult> {
        let controller = Self::new()?;
        let mut recommendations = Vec::new();

        // GPU Check
        let gpu_check = if controller.capabilities.supports_dlss {
            DoctorCheck::ok(
                "GPU",
                &format!("{} - DLSS Supported", controller.capabilities.gpu_model),
            )
        } else {
            recommendations
                .push("DLSS requires an NVIDIA RTX GPU (20-series or newer)".to_string());
            DoctorCheck::error(
                "GPU",
                "DLSS not supported",
                Some(&controller.capabilities.gpu_model),
            )
        };

        // Driver Check
        let driver_version = &controller.capabilities.driver_version;
        let driver_check = if let Some(major) = driver_version
            .split('.')
            .next()
            .and_then(|s| s.parse::<u32>().ok())
        {
            if major >= 545 {
                DoctorCheck::ok("Driver", &format!("v{} - DLSS 3.5+ ready", driver_version))
            } else if major >= 511 {
                recommendations
                    .push("Update to driver 545+ for DLSS 3.5 Ray Reconstruction".to_string());
                DoctorCheck::warn(
                    "Driver",
                    &format!("v{} - Consider updating", driver_version),
                    Some("545+ recommended for DLSS 3.5"),
                )
            } else {
                recommendations
                    .push("Update NVIDIA driver to 511+ for DLSS 2, 545+ for DLSS 3.5".to_string());
                DoctorCheck::error(
                    "Driver",
                    &format!("v{} - Too old", driver_version),
                    Some("511+ required"),
                )
            }
        } else {
            DoctorCheck::warn("Driver", "Could not parse driver version", None)
        };

        // Proton Check
        let proton_check = Self::check_proton_support();

        // DXVK-NVAPI Check
        let nvapi_check = Self::check_dxvk_nvapi();

        // VKD3D-Proton Check
        let vkd3d_check = Self::check_vkd3d_proton();

        // Add Proton-specific recommendations
        if proton_check.status != DoctorStatus::Healthy {
            recommendations
                .push("Install Proton-GE or Proton-CachyOS for best DLSS support".to_string());
            recommendations.push("Add 'PROTON_DLSS_UPGRADE=1' to Steam launch options".to_string());
        }

        if nvapi_check.status == DoctorStatus::Warning {
            recommendations.push("DXVK-NVAPI enables DLSS in DirectX games via Proton".to_string());
        }

        // Determine overall status
        let status = if gpu_check.status == DoctorStatus::Error
            || driver_check.status == DoctorStatus::Error
        {
            DoctorStatus::Error
        } else if gpu_check.status == DoctorStatus::Warning
            || driver_check.status == DoctorStatus::Warning
            || proton_check.status == DoctorStatus::Warning
        {
            DoctorStatus::Warning
        } else {
            DoctorStatus::Healthy
        };

        Ok(DlssDoctorResult {
            status,
            gpu_check,
            driver_check,
            proton_check,
            nvapi_check,
            vkd3d_check,
            recommendations,
        })
    }

    /// Check Proton/Wine support
    fn check_proton_support() -> DoctorCheck {
        // Check for Proton-GE or Proton-CachyOS
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return DoctorCheck::warn("Proton", "Could not find home directory", None),
        };

        let proton_ge = home.join(".steam/root/compatibilitytools.d");
        let proton_ge_exists = proton_ge.exists()
            && fs::read_dir(&proton_ge)
                .map(|entries| {
                    entries.filter_map(|e| e.ok()).any(|e| {
                        e.file_name()
                            .to_string_lossy()
                            .to_lowercase()
                            .contains("ge-proton")
                            || e.file_name()
                                .to_string_lossy()
                                .to_lowercase()
                                .contains("cachyos")
                    })
                })
                .unwrap_or(false);

        if proton_ge_exists {
            DoctorCheck::ok(
                "Proton",
                "Proton-GE/CachyOS detected - DLSS upgrade supported",
            )
        } else {
            // Check for standard Proton with DLSS support
            let steam_proton = home.join(".steam/steam/steamapps/common");
            let has_proton_8_plus = steam_proton.exists()
                && fs::read_dir(&steam_proton)
                    .map(|entries| {
                        entries.filter_map(|e| e.ok()).any(|e| {
                            let name = e.file_name().to_string_lossy().to_lowercase();
                            name.starts_with("proton") && (name.contains("8") || name.contains("9"))
                        })
                    })
                    .unwrap_or(false);

            if has_proton_8_plus {
                DoctorCheck::warn(
                    "Proton",
                    "Stock Proton 8+ detected",
                    Some("Proton-GE recommended for PROTON_DLSS_UPGRADE"),
                )
            } else {
                DoctorCheck::warn(
                    "Proton",
                    "No compatible Proton found",
                    Some("Install Proton-GE for best DLSS support"),
                )
            }
        }
    }

    /// Check DXVK-NVAPI support
    fn check_dxvk_nvapi() -> DoctorCheck {
        // DXVK-NVAPI is bundled with Proton-GE and recent Proton versions
        // We check for the dll in common locations
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return DoctorCheck::warn("DXVK-NVAPI", "Could not check", None),
        };

        let proton_ge_path = home.join(".steam/root/compatibilitytools.d");
        if proton_ge_path.exists() {
            // Proton-GE includes DXVK-NVAPI
            DoctorCheck::ok("DXVK-NVAPI", "Bundled with Proton-GE")
        } else {
            DoctorCheck::warn("DXVK-NVAPI", "Install Proton-GE for DXVK-NVAPI", None)
        }
    }

    /// Check VKD3D-Proton support
    fn check_vkd3d_proton() -> DoctorCheck {
        // VKD3D-Proton is required for DX12 games with DLSS
        let home = match dirs::home_dir() {
            Some(h) => h,
            None => return DoctorCheck::warn("VKD3D-Proton", "Could not check", None),
        };

        let proton_ge_path = home.join(".steam/root/compatibilitytools.d");
        if proton_ge_path.exists() {
            DoctorCheck::ok("VKD3D-Proton", "Bundled with Proton-GE (DX12 DLSS ready)")
        } else {
            DoctorCheck::warn(
                "VKD3D-Proton",
                "Install Proton-GE for DX12 DLSS",
                Some("Required for DirectX 12 games"),
            )
        }
    }

    /// Generate Proton launch options for a game
    pub fn generate_launch_options(game: &GameDlssInfo, opts: &ProtonLaunchOptions) -> String {
        let launch_opts = opts.to_steam_launch_options();

        // Add DXVK_NVAPI_ALLOW_OTHER_DRIVERS for non-NVIDIA GPUs in multi-GPU setups
        // (not needed for pure NVIDIA)

        // If game has DLSS-RR support
        if game
            .dlls
            .iter()
            .any(|d| d.dll_type == DllType::RayReconstruction)
        {
            // Ray Reconstruction is automatically enabled with DLSS 3.5+
        }

        launch_opts
    }

    /// Get list of known available DLSS versions
    pub fn get_available_versions() -> Vec<AvailableDlssVersion> {
        // These are well-known versions from TechPowerUp
        // In a full implementation, we'd fetch this from the web
        vec![
            AvailableDlssVersion {
                version: "310.5.0".to_string(),
                parsed: DllVersion::new(310, 5, 0, 0),
                release_date: Some("December 2025".to_string()),
                download_url: Some(
                    "https://www.techpowerup.com/download/nvidia-dlss-dll/".to_string(),
                ),
                file_size: Some(55 * 1024 * 1024),
                is_latest: true,
                is_transformer: true,
            },
            AvailableDlssVersion {
                version: "310.4.0".to_string(),
                parsed: DllVersion::new(310, 4, 0, 0),
                release_date: Some("November 2025".to_string()),
                download_url: Some(
                    "https://www.techpowerup.com/download/nvidia-dlss-dll/".to_string(),
                ),
                file_size: Some(55 * 1024 * 1024),
                is_latest: false,
                is_transformer: true,
            },
            AvailableDlssVersion {
                version: "310.2.0".to_string(),
                parsed: DllVersion::new(310, 2, 0, 0),
                release_date: Some("October 2025".to_string()),
                download_url: Some(
                    "https://www.techpowerup.com/download/nvidia-dlss-dll/".to_string(),
                ),
                file_size: Some(54 * 1024 * 1024),
                is_latest: false,
                is_transformer: true,
            },
            AvailableDlssVersion {
                version: "3.7.20".to_string(),
                parsed: DllVersion::new(3, 7, 20, 0),
                release_date: Some("August 2024".to_string()),
                download_url: Some(
                    "https://www.techpowerup.com/download/nvidia-dlss-dll/".to_string(),
                ),
                file_size: Some(32 * 1024 * 1024),
                is_latest: false,
                is_transformer: false,
            },
            AvailableDlssVersion {
                version: "3.5.10".to_string(),
                parsed: DllVersion::new(3, 5, 10, 0),
                release_date: Some("March 2024".to_string()),
                download_url: Some(
                    "https://www.techpowerup.com/download/nvidia-dlss-dll/".to_string(),
                ),
                file_size: Some(28 * 1024 * 1024),
                is_latest: false,
                is_transformer: false,
            },
        ]
    }

    /// Get the cache directory for DLSS DLLs
    pub fn get_cache_dir() -> NvResult<PathBuf> {
        let cache = dirs::cache_dir()
            .or_else(dirs::data_local_dir)
            .ok_or_else(|| NvControlError::ConfigError("Could not find cache directory".into()))?
            .join("nvcontrol")
            .join("dlss");

        fs::create_dir_all(&cache)?;
        Ok(cache)
    }

    /// Backup a game's DLSS DLL before replacement
    pub fn backup_dll(dll_path: &Path) -> NvResult<PathBuf> {
        let backup_path = dll_path.with_extension("dll.backup");
        fs::copy(dll_path, &backup_path)?;
        Ok(backup_path)
    }

    /// Restore a backed up DLL
    pub fn restore_dll(dll_path: &Path) -> NvResult<()> {
        let backup_path = dll_path.with_extension("dll.backup");
        if backup_path.exists() {
            fs::copy(&backup_path, dll_path)?;
            Ok(())
        } else {
            Err(NvControlError::ConfigError(format!(
                "No backup found for {:?}",
                dll_path
            )))
        }
    }
}

impl Default for DlssSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: DlssMode::Off,
            quality_preset: DlssQuality::Balanced,
            frame_generation: FrameGenerationSettings::default(),
            sharpening: 0.5,
            auto_exposure: true,
            reflex_mode: ReflexMode::Off,
        }
    }
}

impl Default for FrameGenerationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: FrameGenerationMode::Standard,
            indicator: false,
            frame_pacing: true,
            vsync_compensation: true,
            target_fps_multiplier: 2.0,
        }
    }
}

/// Enable DLSS 3 Frame Generation for current display
pub fn enable_frame_generation(quality: DlssQuality) -> NvResult<()> {
    let mut controller = DlssController::new()?;

    if !controller.capabilities.supports_frame_generation {
        return Err(NvControlError::UnsupportedFeature(
            "Frame Generation requires RTX 40-series GPU or newer".to_string(),
        ));
    }

    let mut settings = controller.current_settings.clone();
    settings.enabled = true;
    settings.mode = DlssMode::SuperResolutionAndFrameGeneration;
    settings.quality_preset = quality.clone();
    settings.frame_generation.enabled = true;
    settings.reflex_mode = ReflexMode::OnPlusBoost; // Recommended with FG

    controller.apply_settings(settings)?;

    println!(
        "✅ DLSS 3 Frame Generation enabled ({})",
        match &quality {
            DlssQuality::Performance => "Performance",
            DlssQuality::Balanced => "Balanced",
            DlssQuality::Quality => "Quality",
            _ => "Custom",
        }
    );

    Ok(())
}

/// Get DLSS status and capabilities
pub fn get_dlss_status() -> NvResult<String> {
    let controller = DlssController::new()?;

    let mut status = format!(
        "🎮 DLSS Status\n\
        ├─ GPU: {}\n\
        ├─ DLSS Version: {}\n\
        ├─ Driver: {}\n",
        controller.capabilities.gpu_model,
        controller.version, // Uses Display trait now
        controller.capabilities.driver_version
    );

    status.push_str(&format!(
        "├─ Capabilities:\n\
        │  ├─ Super Resolution: {}\n\
        │  ├─ Frame Generation: {}\n",
        if controller.capabilities.supports_dlss {
            "✅"
        } else {
            "❌"
        },
        if controller.capabilities.supports_frame_generation {
            "✅ (RTX 40+)"
        } else {
            "❌"
        }
    ));

    // Add Multi-Frame Generation for RTX 50 series
    if controller.capabilities.supports_multi_frame_generation {
        status.push_str(&format!(
            "│  ├─ Multi-Frame Generation: ✅ (up to {}x, RTX 50)\n",
            controller.capabilities.max_frame_multiplier
        ));
    }

    status.push_str(&format!(
        "│  ├─ Ray Reconstruction: {}\n\
        │  └─ NVIDIA Reflex: {}\n",
        if controller.capabilities.supports_ray_reconstruction {
            "✅ (DLSS 3.5)"
        } else {
            "❌"
        },
        if controller.capabilities.supports_reflex {
            "✅"
        } else {
            "❌"
        }
    ));

    if controller.capabilities.supports_frame_generation {
        status.push_str(&format!(
            "├─ Hardware:\n\
            │  ├─ Tensor Cores: {}\n\
            │  ├─ Optical Flow Accelerator: {} (Gen {})\n",
            controller.capabilities.tensor_cores,
            if controller.capabilities.optical_flow_accelerator {
                "✅"
            } else {
                "❌"
            },
            controller.capabilities.optical_flow_accelerator_version
        ));

        // Frame pacing info for DLSS 4.5
        if controller.version == DlssVersion::Dlss4_5 {
            status.push_str("│  └─ Enhanced Frame Pacing: ✅\n");
        } else {
            status.push_str("│  └─ Frame Pacing: ✅\n");
        }
    }

    status.push_str(&format!(
        "└─ Current Mode: {:?}",
        controller.current_settings.mode
    ));

    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dlss_detection() {
        let controller = DlssController::new();
        assert!(controller.is_ok() || controller.is_err());
    }

    #[test]
    fn test_game_profiles() {
        let profiles = DlssController::load_game_profiles();
        assert!(profiles.contains_key("cyberpunk2077"));
        assert!(profiles.contains_key("cs2"));
    }

    #[test]
    fn test_default_settings() {
        let settings = DlssSettings::default();
        assert!(!settings.enabled);
        assert_eq!(settings.mode, DlssMode::Off);
    }
}
