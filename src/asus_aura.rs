/// ASUS Aura RGB/ARGB Control
///
/// RGB lighting control for ASUS ROG graphics cards via OpenRGB integration
/// Reference: asusctl aura implementation for ASUS laptops
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// RGB color
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn red() -> Self {
        Self { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> Self {
        Self { r: 0, g: 255, b: 0 }
    }

    pub fn blue() -> Self {
        Self { r: 0, g: 0, b: 255 }
    }

    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn off() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

/// Aura RGB effect modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuraMode {
    Static,
    Breathing,
    ColorCycle,
    Rainbow,
    Strobing,
    Music,
    Direct,
}

/// Aura effect speed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuraSpeed {
    Slow,
    Medium,
    Fast,
}

/// Aura effect configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraEffect {
    pub mode: AuraMode,
    pub speed: AuraSpeed,
    pub colors: Vec<RgbColor>,
    pub brightness: u8, // 0-100
}

impl AuraEffect {
    pub fn static_color(color: RgbColor, brightness: u8) -> Self {
        Self {
            mode: AuraMode::Static,
            speed: AuraSpeed::Medium,
            colors: vec![color],
            brightness,
        }
    }

    pub fn breathing(color: RgbColor, speed: AuraSpeed) -> Self {
        Self {
            mode: AuraMode::Breathing,
            speed,
            colors: vec![color],
            brightness: 100,
        }
    }

    pub fn rainbow(speed: AuraSpeed) -> Self {
        Self {
            mode: AuraMode::Rainbow,
            speed,
            colors: Vec::new(),
            brightness: 100,
        }
    }
}

/// ASUS Aura RGB controller
pub struct AsusAuraController {
    gpu_device_id: Option<u32>,
    openrgb_available: bool,
}

impl AsusAuraController {
    pub fn new() -> Self {
        let openrgb_available = Self::check_openrgb();

        Self {
            gpu_device_id: None,
            openrgb_available,
        }
    }

    fn check_openrgb() -> bool {
        Command::new("openrgb").arg("--version").output().is_ok()
    }

    /// Detect ASUS GPU device in OpenRGB
    pub fn detect_gpu(&mut self) -> NvResult<bool> {
        if !self.openrgb_available {
            return Err(NvControlError::UnsupportedFeature(
                "OpenRGB not installed (install: paru -S openrgb)".to_string(),
            ));
        }

        // List OpenRGB devices
        let output = Command::new("openrgb")
            .args(&["--list-devices"])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("OpenRGB list failed: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Find ASUS GPU device
        for (index, line) in stdout.lines().enumerate() {
            if line.to_lowercase().contains("asus") && line.to_lowercase().contains("nvidia") {
                self.gpu_device_id = Some(index as u32);
                println!("Found ASUS GPU at device {}: {}", index, line);
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Apply Aura effect
    pub fn apply_effect(&self, effect: &AuraEffect) -> NvResult<()> {
        if !self.openrgb_available {
            return Err(NvControlError::UnsupportedFeature(
                "OpenRGB not available".to_string(),
            ));
        }

        let device_id = self.gpu_device_id.ok_or_else(|| {
            NvControlError::RuntimeError("GPU not detected, run detect_gpu() first".to_string())
        })?;

        // Set mode
        let mode_name = match effect.mode {
            AuraMode::Static => "static",
            AuraMode::Breathing => "breathing",
            AuraMode::ColorCycle => "spectrum cycle",
            AuraMode::Rainbow => "rainbow",
            AuraMode::Strobing => "flashing",
            AuraMode::Music => "music",
            AuraMode::Direct => "direct",
        };

        let output = Command::new("openrgb")
            .args(&["--device", &device_id.to_string(), "--mode", mode_name])
            .output()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("OpenRGB mode set failed: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NvControlError::RuntimeError(format!(
                "Failed to set mode: {}",
                stderr
            )));
        }

        // Set colors
        for (index, color) in effect.colors.iter().enumerate() {
            let output = Command::new("openrgb")
                .args(&[
                    "--device",
                    &device_id.to_string(),
                    "--color",
                    &format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
                ])
                .output()
                .map_err(|e| {
                    NvControlError::CommandFailed(format!("OpenRGB color set failed: {}", e))
                })?;

            if !output.status.success() {
                eprintln!("Warning: Failed to set color {}", index);
            }
        }

        // Set brightness
        let output = Command::new("openrgb")
            .args(&[
                "--device",
                &device_id.to_string(),
                "--brightness",
                &effect.brightness.to_string(),
            ])
            .output()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("OpenRGB brightness set failed: {}", e))
            })?;

        if !output.status.success() {
            eprintln!("Warning: Failed to set brightness");
        }

        println!("Applied Aura effect: {:?}", effect.mode);

        Ok(())
    }

    /// Turn off RGB lighting
    pub fn turn_off(&self) -> NvResult<()> {
        let effect = AuraEffect::static_color(RgbColor::off(), 0);
        self.apply_effect(&effect)
    }

    /// Set static color
    pub fn set_static_color(&self, color: RgbColor, brightness: u8) -> NvResult<()> {
        let effect = AuraEffect::static_color(color, brightness);
        self.apply_effect(&effect)
    }

    /// Check if OpenRGB is available
    pub fn is_available(&self) -> bool {
        self.openrgb_available
    }

    /// Get current GPU device ID
    pub fn device_id(&self) -> Option<u32> {
        self.gpu_device_id
    }
}

impl Default for AsusAuraController {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset Aura effects for quick access
pub struct AuraPresets;

impl AuraPresets {
    pub fn rog_red() -> AuraEffect {
        AuraEffect::static_color(RgbColor::new(255, 0, 0), 100)
    }

    pub fn rog_rainbow() -> AuraEffect {
        AuraEffect::rainbow(AuraSpeed::Medium)
    }

    pub fn gaming_pulse() -> AuraEffect {
        AuraEffect::breathing(RgbColor::new(255, 0, 255), AuraSpeed::Fast)
    }

    pub fn stealth_mode() -> AuraEffect {
        AuraEffect::static_color(RgbColor::off(), 0)
    }

    pub fn performance_mode() -> AuraEffect {
        AuraEffect::breathing(RgbColor::red(), AuraSpeed::Fast)
    }

    pub fn silent_mode() -> AuraEffect {
        AuraEffect::static_color(RgbColor::blue(), 30)
    }

    /// Cyberpunk purple/cyan
    pub fn cyberpunk() -> AuraEffect {
        AuraEffect {
            mode: AuraMode::Breathing,
            speed: AuraSpeed::Slow,
            colors: vec![
                RgbColor::new(255, 0, 128), // Hot pink
                RgbColor::new(0, 255, 255), // Cyan
            ],
            brightness: 100,
        }
    }

    /// Purple glow
    pub fn purple_glow() -> AuraEffect {
        AuraEffect::breathing(RgbColor::new(128, 0, 255), AuraSpeed::Slow)
    }
}

/// Aura configuration for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraConfig {
    /// Active effect configuration
    pub effect: AuraEffect,
    /// Whether to apply on startup
    pub apply_on_startup: bool,
    /// Temperature-reactive mode enabled
    pub temperature_reactive: bool,
    /// Temperature thresholds for reactive mode (in Celsius)
    pub temp_thresholds: TempThresholds,
}

/// Temperature thresholds for reactive RGB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempThresholds {
    pub cold: u32,      // Below this: blue/cyan
    pub cool: u32,      // Below this: green
    pub warm: u32,      // Below this: yellow
    pub hot: u32,       // Below this: orange
                        // Above hot: red
}

impl Default for TempThresholds {
    fn default() -> Self {
        Self {
            cold: 50,
            cool: 60,
            warm: 70,
            hot: 80,
        }
    }
}

impl Default for AuraConfig {
    fn default() -> Self {
        Self {
            effect: AuraPresets::rog_red(),
            apply_on_startup: false,
            temperature_reactive: false,
            temp_thresholds: TempThresholds::default(),
        }
    }
}

impl AuraConfig {
    /// Get the config file path
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("nvcontrol");
        config_dir.join("aura.json")
    }

    /// Load configuration from file
    pub fn load() -> NvResult<Self> {
        let path = Self::config_path();

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to read Aura config: {}", e))
        })?;

        serde_json::from_str(&content).map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to parse Aura config: {}", e))
        })
    }

    /// Save configuration to file
    pub fn save(&self) -> NvResult<()> {
        let path = Self::config_path();

        // Create config directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                NvControlError::RuntimeError(format!("Failed to create config dir: {}", e))
            })?;
        }

        let content = serde_json::to_string_pretty(self).map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to serialize Aura config: {}", e))
        })?;

        fs::write(&path, content).map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to write Aura config: {}", e))
        })?;

        Ok(())
    }

    /// Get color based on GPU temperature
    pub fn temperature_color(&self, temp_c: u32) -> RgbColor {
        if temp_c < self.temp_thresholds.cold {
            // Cold: Blue/Cyan
            RgbColor::new(0, 128, 255)
        } else if temp_c < self.temp_thresholds.cool {
            // Cool: Green
            RgbColor::new(0, 255, 64)
        } else if temp_c < self.temp_thresholds.warm {
            // Warm: Yellow
            RgbColor::new(255, 255, 0)
        } else if temp_c < self.temp_thresholds.hot {
            // Hot: Orange
            RgbColor::new(255, 128, 0)
        } else {
            // Critical: Red
            RgbColor::new(255, 0, 0)
        }
    }
}

impl AsusAuraController {
    /// Apply effect and save to config for persistence
    pub fn apply_effect_and_save(&self, effect: &AuraEffect) -> NvResult<()> {
        // Apply the effect
        self.apply_effect(effect)?;

        // Save to config
        let mut config = AuraConfig::load().unwrap_or_default();
        config.effect = effect.clone();
        config.apply_on_startup = true;
        config.save()?;

        Ok(())
    }

    /// Load and apply saved configuration
    pub fn restore_saved_effect(&self) -> NvResult<()> {
        let config = AuraConfig::load()?;

        if config.apply_on_startup {
            self.apply_effect(&config.effect)?;
            println!("Restored saved Aura effect: {:?}", config.effect.mode);
        }

        Ok(())
    }

    /// Set temperature-reactive mode
    pub fn set_temperature_reactive(&self, enabled: bool) -> NvResult<()> {
        let mut config = AuraConfig::load().unwrap_or_default();
        config.temperature_reactive = enabled;
        config.save()?;

        if enabled {
            println!("Temperature-reactive RGB mode enabled");
        } else {
            println!("Temperature-reactive RGB mode disabled");
        }

        Ok(())
    }

    /// Update RGB based on current GPU temperature
    pub fn update_for_temperature(&self, temp_c: u32) -> NvResult<()> {
        let config = AuraConfig::load()?;

        if !config.temperature_reactive {
            return Ok(());
        }

        let color = config.temperature_color(temp_c);
        let effect = AuraEffect::static_color(color, config.effect.brightness);

        // Only apply, don't save (this is dynamic)
        self.apply_effect(&effect)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_color_creation() {
        let red = RgbColor::red();
        assert_eq!(red.r, 255);
        assert_eq!(red.g, 0);
        assert_eq!(red.b, 0);

        let custom = RgbColor::new(128, 64, 192);
        assert_eq!(custom.r, 128);
        assert_eq!(custom.g, 64);
        assert_eq!(custom.b, 192);
    }

    #[test]
    fn test_aura_effect_presets() {
        let static_effect = AuraEffect::static_color(RgbColor::red(), 100);
        assert_eq!(static_effect.mode, AuraMode::Static);
        assert_eq!(static_effect.brightness, 100);

        let breathing = AuraEffect::breathing(RgbColor::blue(), AuraSpeed::Slow);
        assert_eq!(breathing.mode, AuraMode::Breathing);
        assert_eq!(breathing.speed, AuraSpeed::Slow);

        let rainbow = AuraEffect::rainbow(AuraSpeed::Fast);
        assert_eq!(rainbow.mode, AuraMode::Rainbow);
    }

    #[test]
    fn test_aura_controller() {
        let controller = AsusAuraController::new();
        println!("OpenRGB available: {}", controller.is_available());
    }

    #[test]
    fn test_aura_presets() {
        let rog_red = AuraPresets::rog_red();
        assert_eq!(rog_red.mode, AuraMode::Static);

        let rainbow = AuraPresets::rog_rainbow();
        assert_eq!(rainbow.mode, AuraMode::Rainbow);
    }
}
