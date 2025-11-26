/// Phase 4.3: Upscaling Technology
///
/// DLSS configuration, FSR integration, XeSS support, quality preset management, resolution scaling

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Upscaling technology type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpscalingTech {
    DLSS,
    FSR,
    XeSS,
    Native,
}

/// DLSS quality mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DlssQuality {
    UltraPerformance, // 720p -> 4K (3x)
    Performance,      // 1080p -> 4K (2x)
    Balanced,         // 1440p -> 4K (1.5x)
    Quality,          // 1800p -> 4K (1.3x)
    UltraQuality,     // Native-like quality
}

impl DlssQuality {
    /// Get scaling factor for target resolution
    pub fn scaling_factor(&self) -> f32 {
        match self {
            Self::UltraPerformance => 3.0,
            Self::Performance => 2.0,
            Self::Balanced => 1.5,
            Self::Quality => 1.3,
            Self::UltraQuality => 1.1,
        }
    }

    /// Calculate render resolution from display resolution
    pub fn render_resolution(&self, display_width: u32, display_height: u32) -> (u32, u32) {
        let factor = self.scaling_factor();
        (
            (display_width as f32 / factor) as u32,
            (display_height as f32 / factor) as u32,
        )
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::UltraPerformance => "Ultra Performance",
            Self::Performance => "Performance",
            Self::Balanced => "Balanced",
            Self::Quality => "Quality",
            Self::UltraQuality => "Ultra Quality",
        }
    }
}

/// FSR quality mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FsrQuality {
    UltraPerformance,
    Performance,
    Balanced,
    Quality,
    UltraQuality,
}

impl FsrQuality {
    pub fn scaling_factor(&self) -> f32 {
        match self {
            Self::UltraPerformance => 3.0,
            Self::Performance => 2.0,
            Self::Balanced => 1.7,
            Self::Quality => 1.5,
            Self::UltraQuality => 1.3,
        }
    }

    pub fn render_resolution(&self, display_width: u32, display_height: u32) -> (u32, u32) {
        let factor = self.scaling_factor();
        (
            (display_width as f32 / factor) as u32,
            (display_height as f32 / factor) as u32,
        )
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::UltraPerformance => "Ultra Performance",
            Self::Performance => "Performance",
            Self::Balanced => "Balanced",
            Self::Quality => "Quality",
            Self::UltraQuality => "Ultra Quality",
        }
    }
}

/// XeSS quality mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum XessQuality {
    Performance,
    Balanced,
    Quality,
    UltraQuality,
}

impl XessQuality {
    pub fn scaling_factor(&self) -> f32 {
        match self {
            Self::Performance => 2.0,
            Self::Balanced => 1.7,
            Self::Quality => 1.5,
            Self::UltraQuality => 1.3,
        }
    }

    pub fn render_resolution(&self, display_width: u32, display_height: u32) -> (u32, u32) {
        let factor = self.scaling_factor();
        (
            (display_width as f32 / factor) as u32,
            (display_height as f32 / factor) as u32,
        )
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Performance => "Performance",
            Self::Balanced => "Balanced",
            Self::Quality => "Quality",
            Self::UltraQuality => "Ultra Quality",
        }
    }
}

/// Upscaling configuration for a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpscalingConfig {
    pub game: String,
    pub tech: UpscalingTech,
    pub dlss_quality: Option<DlssQuality>,
    pub fsr_quality: Option<FsrQuality>,
    pub xess_quality: Option<XessQuality>,
    pub sharpness: f32, // 0.0 - 1.0
    pub enabled: bool,
}

impl UpscalingConfig {
    pub fn new_dlss(game: String, quality: DlssQuality) -> Self {
        Self {
            game,
            tech: UpscalingTech::DLSS,
            dlss_quality: Some(quality),
            fsr_quality: None,
            xess_quality: None,
            sharpness: 0.5,
            enabled: true,
        }
    }

    pub fn new_fsr(game: String, quality: FsrQuality) -> Self {
        Self {
            game,
            tech: UpscalingTech::FSR,
            dlss_quality: None,
            fsr_quality: Some(quality),
            xess_quality: None,
            sharpness: 0.5,
            enabled: true,
        }
    }

    pub fn new_xess(game: String, quality: XessQuality) -> Self {
        Self {
            game,
            tech: UpscalingTech::XeSS,
            dlss_quality: None,
            fsr_quality: None,
            xess_quality: Some(quality),
            sharpness: 0.5,
            enabled: true,
        }
    }
}

/// DLSS capability checker
pub struct DlssCapability {
    gpu_id: u32,
    dlss_supported: bool,
    dlss_version: Option<String>,
}

impl DlssCapability {
    pub fn new(gpu_id: u32) -> NvResult<Self> {
        let (supported, version) = Self::check_dlss_support(gpu_id)?;

        Ok(Self {
            gpu_id,
            dlss_supported: supported,
            dlss_version: version,
        })
    }

    fn check_dlss_support(gpu_id: u32) -> NvResult<(bool, Option<String>)> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let device = nvml.device_by_index(gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        let name = device.name().unwrap_or_default();

        // DLSS requires RTX series (Turing or newer)
        let dlss_supported = name.contains("RTX") || name.contains("Quadro RTX");

        let version = if dlss_supported {
            // Check compute capability for DLSS version
            let compute_cap = device.cuda_compute_capability().ok();

            match compute_cap {
                Some(cc) if cc.major >= 8 => Some("DLSS 3.0".to_string()), // Ampere+
                Some(cc) if cc.major >= 7 && cc.minor >= 5 => Some("DLSS 2.0".to_string()), // Turing
                _ => Some("DLSS 1.0".to_string()),
            }
        } else {
            None
        };

        Ok((dlss_supported, version))
    }

    pub fn is_supported(&self) -> bool {
        self.dlss_supported
    }

    pub fn version(&self) -> Option<&str> {
        self.dlss_version.as_deref()
    }

    /// Check if DLSS 3 Frame Generation is supported (RTX 40+ series)
    pub fn supports_frame_generation(&self) -> NvResult<bool> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let device = nvml.device_by_index(self.gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        let compute_cap = device.cuda_compute_capability().ok();

        // Frame Generation requires Ada Lovelace or newer (compute 8.9+)
        Ok(compute_cap
            .map(|cc| cc.major > 8 || (cc.major == 8 && cc.minor >= 9))
            .unwrap_or(false))
    }
}

/// Upscaling profile manager
pub struct UpscalingProfileManager {
    profiles: HashMap<String, UpscalingConfig>,
    config_path: PathBuf,
}

impl UpscalingProfileManager {
    pub fn new() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nvcontrol")
            .join("upscaling_profiles.json");

        Self {
            profiles: HashMap::new(),
            config_path,
        }
    }

    /// Load profiles from disk
    pub fn load(&mut self) -> NvResult<()> {
        if !self.config_path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.config_path).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to read profiles: {}", e))
        })?;

        self.profiles = serde_json::from_str(&content).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to parse profiles: {}", e))
        })?;

        Ok(())
    }

    /// Save profiles to disk
    pub fn save(&self) -> NvResult<()> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                NvControlError::ConfigError(format!("Failed to create config dir: {}", e))
            })?;
        }

        let content = serde_json::to_string_pretty(&self.profiles).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize profiles: {}", e))
        })?;

        std::fs::write(&self.config_path, content).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to write profiles: {}", e))
        })?;

        Ok(())
    }

    /// Add or update profile
    pub fn set_profile(&mut self, config: UpscalingConfig) {
        self.profiles.insert(config.game.clone(), config);
    }

    /// Get profile
    pub fn get_profile(&self, game: &str) -> Option<&UpscalingConfig> {
        self.profiles.get(game)
    }

    /// List all profiles
    pub fn list_profiles(&self) -> Vec<&UpscalingConfig> {
        self.profiles.values().collect()
    }

    /// Remove profile
    pub fn remove_profile(&mut self, game: &str) -> Option<UpscalingConfig> {
        self.profiles.remove(game)
    }
}

impl Default for UpscalingProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolution scaler helper
pub struct ResolutionScaler {
    display_resolution: (u32, u32),
}

impl ResolutionScaler {
    pub fn new(display_width: u32, display_height: u32) -> Self {
        Self {
            display_resolution: (display_width, display_height),
        }
    }

    /// Calculate render resolution for given upscaling config
    pub fn calculate_render_resolution(&self, config: &UpscalingConfig) -> (u32, u32) {
        let (width, height) = self.display_resolution;

        match config.tech {
            UpscalingTech::DLSS => {
                if let Some(quality) = config.dlss_quality {
                    quality.render_resolution(width, height)
                } else {
                    (width, height)
                }
            }
            UpscalingTech::FSR => {
                if let Some(quality) = config.fsr_quality {
                    quality.render_resolution(width, height)
                } else {
                    (width, height)
                }
            }
            UpscalingTech::XeSS => {
                if let Some(quality) = config.xess_quality {
                    quality.render_resolution(width, height)
                } else {
                    (width, height)
                }
            }
            UpscalingTech::Native => (width, height),
        }
    }

    /// Get performance gain estimate
    pub fn estimate_performance_gain(&self, config: &UpscalingConfig) -> f32 {
        let (render_width, render_height) = self.calculate_render_resolution(config);
        let (display_width, display_height) = self.display_resolution;

        let render_pixels = (render_width * render_height) as f32;
        let display_pixels = (display_width * display_height) as f32;

        // Rough estimate: performance scales with pixel count
        (display_pixels / render_pixels - 1.0) * 100.0
    }
}

/// Automatic upscaling recommender
pub struct UpscalingRecommender {
    target_fps: u32,
    current_fps: f32,
}

impl UpscalingRecommender {
    pub fn new(target_fps: u32) -> Self {
        Self {
            target_fps,
            current_fps: 0.0,
        }
    }

    /// Update current FPS
    pub fn update_fps(&mut self, fps: f32) {
        self.current_fps = fps;
    }

    /// Recommend upscaling quality based on current performance
    pub fn recommend_dlss_quality(&self) -> Option<DlssQuality> {
        if self.current_fps >= self.target_fps as f32 {
            // Already hitting target, use quality mode
            return Some(DlssQuality::Quality);
        }

        let fps_deficit = self.target_fps as f32 - self.current_fps;
        let deficit_percent = (fps_deficit / self.target_fps as f32) * 100.0;

        if deficit_percent > 50.0 {
            Some(DlssQuality::UltraPerformance)
        } else if deficit_percent > 30.0 {
            Some(DlssQuality::Performance)
        } else if deficit_percent > 15.0 {
            Some(DlssQuality::Balanced)
        } else {
            Some(DlssQuality::Quality)
        }
    }

    /// Recommend FSR quality
    pub fn recommend_fsr_quality(&self) -> Option<FsrQuality> {
        if self.current_fps >= self.target_fps as f32 {
            return Some(FsrQuality::Quality);
        }

        let fps_deficit = self.target_fps as f32 - self.current_fps;
        let deficit_percent = (fps_deficit / self.target_fps as f32) * 100.0;

        if deficit_percent > 50.0 {
            Some(FsrQuality::UltraPerformance)
        } else if deficit_percent > 30.0 {
            Some(FsrQuality::Performance)
        } else if deficit_percent > 15.0 {
            Some(FsrQuality::Balanced)
        } else {
            Some(FsrQuality::Quality)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dlss_quality_scaling() {
        let quality = DlssQuality::Performance;
        let (render_w, render_h) = quality.render_resolution(3840, 2160);

        // Performance mode: 2x scaling
        assert_eq!(render_w, 1920);
        assert_eq!(render_h, 1080);
    }

    #[test]
    fn test_fsr_quality_scaling() {
        let quality = FsrQuality::Balanced;
        let (render_w, render_h) = quality.render_resolution(2560, 1440);

        assert!(render_w < 2560);
        assert!(render_h < 1440);
    }

    #[test]
    fn test_upscaling_config() {
        let config = UpscalingConfig::new_dlss(
            "Cyberpunk 2077".to_string(),
            DlssQuality::Quality,
        );

        assert_eq!(config.tech, UpscalingTech::DLSS);
        assert_eq!(config.dlss_quality, Some(DlssQuality::Quality));
        assert!(config.enabled);
    }

    #[test]
    fn test_resolution_scaler() {
        let scaler = ResolutionScaler::new(3840, 2160);

        let config = UpscalingConfig::new_dlss(
            "Test Game".to_string(),
            DlssQuality::Performance,
        );

        let (render_w, render_h) = scaler.calculate_render_resolution(&config);

        assert_eq!(render_w, 1920);
        assert_eq!(render_h, 1080);
    }

    #[test]
    fn test_performance_gain_estimate() {
        let scaler = ResolutionScaler::new(3840, 2160);

        let config = UpscalingConfig::new_dlss(
            "Test Game".to_string(),
            DlssQuality::Performance, // 2x scaling
        );

        let gain = scaler.estimate_performance_gain(&config);

        // 4K -> 1080p = 4x pixels = roughly 300% gain
        assert!(gain > 250.0 && gain < 350.0);
    }

    #[test]
    fn test_upscaling_recommender() {
        let mut recommender = UpscalingRecommender::new(60);

        // Simulate low FPS
        recommender.update_fps(30.0);

        let recommended = recommender.recommend_dlss_quality();
        assert!(matches!(
            recommended,
            Some(DlssQuality::Performance) | Some(DlssQuality::UltraPerformance)
        ));
    }

    #[test]
    fn test_profile_manager() {
        let mut manager = UpscalingProfileManager::new();

        let config = UpscalingConfig::new_dlss(
            "Test Game".to_string(),
            DlssQuality::Balanced,
        );

        manager.set_profile(config);

        assert!(manager.get_profile("Test Game").is_some());
        assert_eq!(
            manager.get_profile("Test Game").unwrap().dlss_quality,
            Some(DlssQuality::Balanced)
        );
    }
}
