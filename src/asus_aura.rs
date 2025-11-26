/// ASUS Aura RGB/ARGB Control
///
/// RGB lighting control for ASUS ROG graphics cards via OpenRGB integration
/// Reference: asusctl aura implementation for ASUS laptops

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
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
        Self { r: 255, g: 255, b: 255 }
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
        Command::new("openrgb")
            .arg("--version")
            .output()
            .is_ok()
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
            .map_err(|e| {
                NvControlError::CommandFailed(format!("OpenRGB list failed: {}", e))
            })?;

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
            .args(&[
                "--device",
                &device_id.to_string(),
                "--mode",
                mode_name,
            ])
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
