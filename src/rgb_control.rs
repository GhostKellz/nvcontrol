use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// RGB/ARGB LED control for ASUS Aura Sync, OpenRGB, and other RGB systems
/// Supports ASUS ROG Astral and other RGB-enabled GPUs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbController {
    pub backend: RgbBackend,
    pub devices: Vec<RgbDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RgbBackend {
    OpenRGB,
    AsusAuraCore, // via asusctl for Linux
    I2CDev,       // Direct I2C control
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbDevice {
    pub name: String,
    pub device_type: DeviceType,
    pub num_leds: u32,
    pub supports_modes: Vec<RgbMode>,
    pub current_mode: RgbMode,
    pub current_color: RgbColor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    GPU,
    Motherboard,
    Ram,
    Fan,
    Strip,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RgbMode {
    Static,
    Breathing,
    ColorCycle,
    Rainbow,
    Strobing,
    Flashing,
    GpuTemperature, // Color based on GPU temp
    GpuLoad,        // Color based on GPU utilization
    Off,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbController {
    /// Detect available RGB control backend
    pub fn new() -> NvResult<Self> {
        let backend = Self::detect_backend();
        let devices = Self::detect_devices(&backend)?;

        Ok(RgbController { backend, devices })
    }

    /// Detect which RGB backend is available
    fn detect_backend() -> RgbBackend {
        // Check for OpenRGB (most universal)
        if Command::new("openrgb").arg("--version").output().is_ok() {
            return RgbBackend::OpenRGB;
        }

        // Check for asusctl (ASUS laptops/some motherboards)
        if Command::new("asusctl").arg("--version").output().is_ok() {
            return RgbBackend::AsusAuraCore;
        }

        // Check for i2c-dev access (direct control)
        if std::path::Path::new("/dev/i2c-0").exists() {
            return RgbBackend::I2CDev;
        }

        RgbBackend::None
    }

    /// Detect RGB devices
    fn detect_devices(backend: &RgbBackend) -> NvResult<Vec<RgbDevice>> {
        match backend {
            RgbBackend::OpenRGB => Self::detect_openrgb_devices(),
            RgbBackend::AsusAuraCore => Self::detect_asus_devices(),
            _ => Ok(Vec::new()),
        }
    }

    /// Detect devices via OpenRGB
    fn detect_openrgb_devices() -> NvResult<Vec<RgbDevice>> {
        let mut devices = Vec::new();

        if let Ok(output) = Command::new("openrgb").args(["--list-devices"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);

            for line in output_str.lines() {
                if line.contains("GPU") || line.contains("Graphics") || line.contains("ASUS") {
                    devices.push(RgbDevice {
                        name: line.trim().to_string(),
                        device_type: DeviceType::GPU,
                        num_leds: 1,
                        supports_modes: vec![
                            RgbMode::Static,
                            RgbMode::Breathing,
                            RgbMode::ColorCycle,
                            RgbMode::Rainbow,
                            RgbMode::Off,
                        ],
                        current_mode: RgbMode::Static,
                        current_color: RgbColor { r: 255, g: 0, b: 0 },
                    });
                }
            }
        }

        Ok(devices)
    }

    /// Detect ASUS devices
    fn detect_asus_devices() -> NvResult<Vec<RgbDevice>> {
        // ASUS ROG devices typically detected via asusctl
        let mut devices = Vec::new();

        if let Ok(output) = Command::new("asusctl").args(["led-mode", "-l"]).output() {
            if output.status.success() {
                devices.push(RgbDevice {
                    name: "ASUS Aura Device".to_string(),
                    device_type: DeviceType::GPU,
                    num_leds: 1,
                    supports_modes: vec![
                        RgbMode::Static,
                        RgbMode::Breathing,
                        RgbMode::ColorCycle,
                        RgbMode::Rainbow,
                    ],
                    current_mode: RgbMode::Static,
                    current_color: RgbColor { r: 255, g: 0, b: 0 },
                });
            }
        }

        Ok(devices)
    }

    /// Set RGB mode for all GPU devices
    pub fn set_gpu_mode(&mut self, mode: RgbMode, color: RgbColor) -> NvResult<()> {
        match self.backend {
            RgbBackend::OpenRGB => self.set_openrgb_mode(mode, color),
            RgbBackend::AsusAuraCore => self.set_asus_mode(mode, color),
            _ => {
                println!("No RGB backend available");
                Ok(())
            }
        }
    }

    /// Set RGB mode via OpenRGB
    fn set_openrgb_mode(&mut self, mode: RgbMode, color: RgbColor) -> NvResult<()> {
        let mode_str = match mode {
            RgbMode::Static => "static",
            RgbMode::Breathing => "breathing",
            RgbMode::ColorCycle => "spectrum cycle",
            RgbMode::Rainbow => "rainbow",
            RgbMode::Off => "off",
            _ => "static",
        };

        let color_str = format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b);

        let _ = Command::new("openrgb")
            .args([
                "--mode", mode_str, "--color", &color_str, "--device",
                "0", // GPU typically device 0
            ])
            .output();

        // Update internal state
        for device in &mut self.devices {
            if device.device_type == DeviceType::GPU {
                device.current_mode = mode.clone();
                device.current_color = color;
            }
        }

        Ok(())
    }

    /// Set RGB mode via asusctl
    fn set_asus_mode(&mut self, mode: RgbMode, color: RgbColor) -> NvResult<()> {
        let mode_str = match mode {
            RgbMode::Static => "static",
            RgbMode::Breathing => "breathe",
            RgbMode::ColorCycle => "pulse",
            RgbMode::Rainbow => "rainbow",
            _ => "static",
        };

        let _ = Command::new("asusctl")
            .args([
                "led-mode",
                "-m",
                mode_str,
                "-c",
                &format!("{},{},{}", color.r, color.g, color.b),
            ])
            .output();

        Ok(())
    }

    /// Set GPU temperature reactive colors
    pub fn set_temp_reactive(&mut self) -> NvResult<()> {
        // This would need to be implemented as a background service
        // that monitors GPU temp and adjusts colors accordingly
        println!("Temperature reactive mode set (requires background service)");

        for device in &mut self.devices {
            if device.device_type == DeviceType::GPU {
                device.current_mode = RgbMode::GpuTemperature;
            }
        }

        Ok(())
    }

    /// Get RGB color based on temperature
    pub fn temp_to_color(temp_celsius: f32) -> RgbColor {
        // Cool (< 50°C): Blue/Cyan
        // Warm (50-70°C): Green/Yellow
        // Hot (70-80°C): Orange
        // Critical (> 80°C): Red

        if temp_celsius < 50.0 {
            RgbColor {
                r: 0,
                g: 150,
                b: 255,
            } // Cyan
        } else if temp_celsius < 60.0 {
            RgbColor {
                r: 0,
                g: 255,
                b: 100,
            } // Green
        } else if temp_celsius < 70.0 {
            RgbColor {
                r: 200,
                g: 255,
                b: 0,
            } // Yellow
        } else if temp_celsius < 80.0 {
            RgbColor {
                r: 255,
                g: 100,
                b: 0,
            } // Orange
        } else {
            RgbColor { r: 255, g: 0, b: 0 } // Red
        }
    }

    /// Print RGB device info
    pub fn print_devices(&self) {
        println!("\n=== RGB Control ===");
        println!("Backend: {:?}", self.backend);
        println!("\nDevices:");
        for (idx, device) in self.devices.iter().enumerate() {
            println!("\n  Device {}:", idx);
            println!("    Name: {}", device.name);
            println!("    Type: {:?}", device.device_type);
            println!("    LEDs: {}", device.num_leds);
            println!("    Current Mode: {:?}", device.current_mode);
            println!(
                "    Current Color: RGB({}, {}, {})",
                device.current_color.r, device.current_color.g, device.current_color.b
            );
        }
    }
}

/// Preset RGB profiles for ASUS ROG Astral
pub fn get_astral_presets() -> Vec<(String, RgbMode, RgbColor)> {
    vec![
        (
            "ROG Red".to_string(),
            RgbMode::Static,
            RgbColor { r: 255, g: 0, b: 0 },
        ),
        (
            "Cyberpunk Cyan".to_string(),
            RgbMode::Breathing,
            RgbColor {
                r: 0,
                g: 255,
                b: 255,
            },
        ),
        (
            "Purple Glow".to_string(),
            RgbMode::Breathing,
            RgbColor {
                r: 128,
                g: 0,
                b: 255,
            },
        ),
        (
            "Rainbow Wave".to_string(),
            RgbMode::Rainbow,
            RgbColor { r: 0, g: 0, b: 0 }, // Not used in rainbow mode
        ),
        (
            "Temp Reactive".to_string(),
            RgbMode::GpuTemperature,
            RgbColor { r: 0, g: 0, b: 0 },
        ),
        (
            "Stealth Mode".to_string(),
            RgbMode::Off,
            RgbColor { r: 0, g: 0, b: 0 },
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_to_color() {
        let cold = RgbController::temp_to_color(40.0);
        assert_eq!(cold.b, 255); // Should be blue-ish

        let hot = RgbController::temp_to_color(85.0);
        assert_eq!(hot.r, 255); // Should be red
    }

    #[test]
    fn test_astral_presets() {
        let presets = get_astral_presets();
        assert!(presets.len() >= 5);
    }
}
