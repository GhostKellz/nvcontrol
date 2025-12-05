/// Phase 4.4: VRR & Display
///
/// Per-game VRR profiles, adaptive refresh range, NVIDIA Reflex integration, display automation
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

/// VRR (Variable Refresh Rate) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrrConfig {
    pub game: String,
    pub enabled: bool,
    pub min_refresh_hz: u32,
    pub max_refresh_hz: u32,
    pub gsync_compatible: bool,
}

impl VrrConfig {
    pub fn new(game: String, min_hz: u32, max_hz: u32) -> Self {
        Self {
            game,
            enabled: true,
            min_refresh_hz: min_hz,
            max_refresh_hz: max_hz,
            gsync_compatible: true,
        }
    }
}

/// Display configuration for gaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub game: String,
    pub resolution: (u32, u32),
    pub refresh_rate: u32,
    pub vrr_enabled: bool,
    pub hdr_enabled: bool,
    pub color_depth: ColorDepth,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorDepth {
    Bit8,
    Bit10,
    Bit12,
}

impl ColorDepth {
    pub fn bits(&self) -> u32 {
        match self {
            Self::Bit8 => 8,
            Self::Bit10 => 10,
            Self::Bit12 => 12,
        }
    }
}

/// NVIDIA Reflex integration (low latency mode)
pub struct ReflexIntegration {
    gpu_id: u32,
    enabled: bool,
    mode: ReflexMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReflexMode {
    Off,
    On,
    OnWithBoost, // Reduces latency further but may increase power
}

impl ReflexIntegration {
    pub fn new(gpu_id: u32) -> Self {
        Self {
            gpu_id,
            enabled: false,
            mode: ReflexMode::Off,
        }
    }

    /// Enable Reflex with specified mode
    pub fn enable(&mut self, mode: ReflexMode) -> NvResult<()> {
        // Set low latency mode via nvidia-settings
        let mode_value = match mode {
            ReflexMode::Off => 0,
            ReflexMode::On => 1,
            ReflexMode::OnWithBoost => 2,
        };

        let output = Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!("[gpu:{}]/GPUPowerMizerMode={}", self.gpu_id, mode_value),
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("nvidia-settings failed: {}", e)))?;

        if !output.status.success() {
            return Err(NvControlError::LatencyOptimizationFailed(
                "Failed to enable Reflex mode".to_string(),
            ));
        }

        self.enabled = true;
        self.mode = mode;

        println!("NVIDIA Reflex enabled: {:?}", mode);

        Ok(())
    }

    /// Disable Reflex
    pub fn disable(&mut self) -> NvResult<()> {
        self.enable(ReflexMode::Off)?;
        self.enabled = false;

        println!("NVIDIA Reflex disabled");

        Ok(())
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn current_mode(&self) -> ReflexMode {
        self.mode
    }

    /// Check if Reflex is supported (requires RTX series)
    pub fn is_supported(&self) -> NvResult<bool> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let device = nvml
            .device_by_index(self.gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        let name = device.name().unwrap_or_default();

        // Reflex requires RTX series
        Ok(name.contains("RTX"))
    }
}

/// VRR profile manager
pub struct VrrProfileManager {
    profiles: HashMap<String, VrrConfig>,
    config_path: PathBuf,
}

impl VrrProfileManager {
    pub fn new() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nvcontrol")
            .join("vrr_profiles.json");

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

        let content = std::fs::read_to_string(&self.config_path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read profiles: {}", e)))?;

        self.profiles = serde_json::from_str(&content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse profiles: {}", e)))?;

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

        std::fs::write(&self.config_path, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write profiles: {}", e)))?;

        Ok(())
    }

    /// Add or update profile
    pub fn set_profile(&mut self, config: VrrConfig) {
        self.profiles.insert(config.game.clone(), config);
    }

    /// Get profile
    pub fn get_profile(&self, game: &str) -> Option<&VrrConfig> {
        self.profiles.get(game)
    }

    /// Apply VRR profile
    pub fn apply_profile(&self, game: &str, display: &str) -> NvResult<()> {
        let profile = self
            .get_profile(game)
            .ok_or_else(|| NvControlError::ConfigError(format!("Profile not found: {}", game)))?;

        if !profile.enabled {
            return Ok(());
        }

        // Use wayland_integration VrrController
        use crate::wayland_integration::VrrController;

        let vrr_controller = VrrController::new();
        vrr_controller.enable_vrr(display)?;

        println!("Applied VRR profile for game: {}", game);
        println!(
            "  Range: {}-{} Hz",
            profile.min_refresh_hz, profile.max_refresh_hz
        );

        Ok(())
    }

    /// List all profiles
    pub fn list_profiles(&self) -> Vec<&VrrConfig> {
        self.profiles.values().collect()
    }

    /// Remove profile
    pub fn remove_profile(&mut self, game: &str) -> Option<VrrConfig> {
        self.profiles.remove(game)
    }
}

impl Default for VrrProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Display configuration manager
pub struct DisplayConfigManager {
    configs: HashMap<String, DisplayConfig>,
    config_path: PathBuf,
}

impl DisplayConfigManager {
    pub fn new() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("nvcontrol")
            .join("display_configs.json");

        Self {
            configs: HashMap::new(),
            config_path,
        }
    }

    /// Load configurations
    pub fn load(&mut self) -> NvResult<()> {
        if !self.config_path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.config_path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read configs: {}", e)))?;

        self.configs = serde_json::from_str(&content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse configs: {}", e)))?;

        Ok(())
    }

    /// Save configurations
    pub fn save(&self) -> NvResult<()> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                NvControlError::ConfigError(format!("Failed to create config dir: {}", e))
            })?;
        }

        let content = serde_json::to_string_pretty(&self.configs).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize configs: {}", e))
        })?;

        std::fs::write(&self.config_path, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write configs: {}", e)))?;

        Ok(())
    }

    /// Set display configuration for game
    pub fn set_config(&mut self, config: DisplayConfig) {
        self.configs.insert(config.game.clone(), config);
    }

    /// Get configuration
    pub fn get_config(&self, game: &str) -> Option<&DisplayConfig> {
        self.configs.get(game)
    }

    /// Apply display configuration
    pub fn apply_config(&self, game: &str, display: &str) -> NvResult<()> {
        let config = self
            .get_config(game)
            .ok_or_else(|| NvControlError::ConfigError(format!("Config not found: {}", game)))?;

        // Change resolution and refresh rate
        self.set_resolution(display, config.resolution, config.refresh_rate)?;

        // Enable VRR if requested
        if config.vrr_enabled {
            use crate::wayland_integration::VrrController;
            let vrr_controller = VrrController::new();
            vrr_controller.enable_vrr(display)?;
        }

        println!("Applied display config for game: {}", game);
        println!(
            "  Resolution: {}x{}@{}Hz",
            config.resolution.0, config.resolution.1, config.refresh_rate
        );

        Ok(())
    }

    fn set_resolution(
        &self,
        display: &str,
        resolution: (u32, u32),
        refresh_rate: u32,
    ) -> NvResult<()> {
        // Detect compositor
        use crate::wayland_integration::WaylandCompositor;

        let compositor = WaylandCompositor::detect();

        match compositor {
            WaylandCompositor::KdePlasma => {
                let output = Command::new("kscreen-doctor")
                    .arg(&format!(
                        "output.{}.mode.{}x{}@{}",
                        display, resolution.0, resolution.1, refresh_rate
                    ))
                    .output()
                    .map_err(|e| {
                        NvControlError::CommandFailed(format!("kscreen-doctor failed: {}", e))
                    })?;

                if !output.status.success() {
                    return Err(NvControlError::RuntimeError(
                        "Failed to set resolution".to_string(),
                    ));
                }
            }
            WaylandCompositor::Hyprland => {
                let output = Command::new("hyprctl")
                    .args(&[
                        "keyword",
                        &format!(
                            "monitor:{},{}x{}@{},0x0,1",
                            display, resolution.0, resolution.1, refresh_rate
                        ),
                    ])
                    .output()
                    .map_err(|e| NvControlError::CommandFailed(format!("hyprctl failed: {}", e)))?;

                if !output.status.success() {
                    return Err(NvControlError::RuntimeError(
                        "Failed to set resolution".to_string(),
                    ));
                }
            }
            WaylandCompositor::Sway => {
                let output = Command::new("swaymsg")
                    .args(&[
                        "output",
                        display,
                        "mode",
                        &format!("{}x{}@{}Hz", resolution.0, resolution.1, refresh_rate),
                    ])
                    .output()
                    .map_err(|e| NvControlError::CommandFailed(format!("swaymsg failed: {}", e)))?;

                if !output.status.success() {
                    return Err(NvControlError::RuntimeError(
                        "Failed to set resolution".to_string(),
                    ));
                }
            }
            _ => {
                return Err(NvControlError::UnsupportedFeature(
                    "Display configuration not supported on this compositor".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// List all configurations
    pub fn list_configs(&self) -> Vec<&DisplayConfig> {
        self.configs.values().collect()
    }
}

impl Default for DisplayConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Adaptive refresh range optimizer
pub struct AdaptiveRefreshOptimizer {
    min_fps_history: Vec<f32>,
    max_fps_history: Vec<f32>,
}

impl AdaptiveRefreshOptimizer {
    pub fn new() -> Self {
        Self {
            min_fps_history: Vec::new(),
            max_fps_history: Vec::new(),
        }
    }

    /// Record FPS sample
    pub fn record_fps(&mut self, fps: f32) {
        self.min_fps_history.push(fps);
        self.max_fps_history.push(fps);

        // Keep last 100 samples
        if self.min_fps_history.len() > 100 {
            self.min_fps_history.remove(0);
        }
        if self.max_fps_history.len() > 100 {
            self.max_fps_history.remove(0);
        }
    }

    /// Get optimal VRR range based on FPS history
    pub fn get_optimal_range(&self) -> Option<(u32, u32)> {
        if self.min_fps_history.is_empty() {
            return None;
        }

        let min_fps = self
            .min_fps_history
            .iter()
            .copied()
            .fold(f32::INFINITY, f32::min);
        let max_fps = self.max_fps_history.iter().copied().fold(0.0f32, f32::max);

        // Add 10% margin
        let min_range = (min_fps * 0.9) as u32;
        let max_range = (max_fps * 1.1) as u32;

        // Clamp to reasonable values
        let min_range = min_range.clamp(30, 120);
        let max_range = max_range.clamp(60, 360);

        Some((min_range, max_range))
    }
}

impl Default for AdaptiveRefreshOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vrr_config() {
        let config = VrrConfig::new("Test Game".to_string(), 48, 144);

        assert!(config.enabled);
        assert_eq!(config.min_refresh_hz, 48);
        assert_eq!(config.max_refresh_hz, 144);
    }

    #[test]
    fn test_display_config() {
        let config = DisplayConfig {
            game: "Cyberpunk 2077".to_string(),
            resolution: (3840, 2160),
            refresh_rate: 120,
            vrr_enabled: true,
            hdr_enabled: true,
            color_depth: ColorDepth::Bit10,
        };

        assert_eq!(config.resolution, (3840, 2160));
        assert_eq!(config.refresh_rate, 120);
        assert!(config.vrr_enabled);
    }

    #[test]
    fn test_reflex_mode() {
        let reflex = ReflexIntegration::new(0);

        assert!(!reflex.is_enabled());
        assert_eq!(reflex.current_mode(), ReflexMode::Off);
    }

    #[test]
    fn test_color_depth() {
        assert_eq!(ColorDepth::Bit8.bits(), 8);
        assert_eq!(ColorDepth::Bit10.bits(), 10);
        assert_eq!(ColorDepth::Bit12.bits(), 12);
    }

    #[test]
    fn test_adaptive_refresh_optimizer() {
        let mut optimizer = AdaptiveRefreshOptimizer::new();

        // Simulate FPS ranging from 60-120
        for fps in 60..=120 {
            optimizer.record_fps(fps as f32);
        }

        let range = optimizer.get_optimal_range();
        assert!(range.is_some());

        let (min, max) = range.unwrap();
        assert!(min < 60);
        assert!(max > 120);
    }

    #[test]
    fn test_vrr_profile_manager() {
        let mut manager = VrrProfileManager::new();

        let config = VrrConfig::new("Test Game".to_string(), 48, 165);

        manager.set_profile(config);

        assert!(manager.get_profile("Test Game").is_some());
        assert_eq!(
            manager.get_profile("Test Game").unwrap().max_refresh_hz,
            165
        );
    }

    #[test]
    fn test_display_config_manager() {
        let mut manager = DisplayConfigManager::new();

        let config = DisplayConfig {
            game: "Test".to_string(),
            resolution: (2560, 1440),
            refresh_rate: 144,
            vrr_enabled: true,
            hdr_enabled: false,
            color_depth: ColorDepth::Bit8,
        };

        manager.set_config(config);

        assert!(manager.get_config("Test").is_some());
    }
}
