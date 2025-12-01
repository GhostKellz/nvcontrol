// On-Screen Display (OSD) System
// In-game overlay for FPS, temperature, GPU usage, etc.
// Inspired by MSI Afterburner's OSD functionality

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsdConfig {
    pub enabled: bool,
    pub position: OsdPosition,
    pub metrics: Vec<OsdMetric>,
    pub update_interval_ms: u64,
    pub font_size: u32,
    pub background_opacity: f32,
    pub text_color: (u8, u8, u8, u8), // RGBA
    pub hotkey: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OsdPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom { x: i32, y: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OsdMetric {
    Fps,
    Frametime,
    GpuName,
    GpuTemperature,
    GpuUtilization,
    GpuMemoryUsed,
    GpuMemoryTotal,
    GpuPowerDraw,
    GpuFanSpeed,
    GpuClockSpeed,
    CpuTemperature,
    CpuUtilization,
    RamUsed,
    RamTotal,
    Custom { label: String, command: String },
}

impl Default for OsdConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            position: OsdPosition::TopLeft,
            metrics: vec![
                OsdMetric::Fps,
                OsdMetric::GpuName,
                OsdMetric::GpuTemperature,
                OsdMetric::GpuUtilization,
                OsdMetric::GpuMemoryUsed,
            ],
            update_interval_ms: 500,
            font_size: 16,
            background_opacity: 0.5,
            text_color: (255, 255, 255, 255),
            hotkey: Some("Ctrl+Shift+O".to_string()),
        }
    }
}

pub struct OsdManager {
    config: OsdConfig,
    config_path: PathBuf,
}

impl OsdManager {
    pub fn new() -> NvResult<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find config directory".into()))?
            .join("nvcontrol");

        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("osd.toml");

        let config = if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            toml::from_str(&contents).map_err(|e| {
                NvControlError::ConfigError(format!("Failed to parse OSD config: {}", e))
            })?
        } else {
            OsdConfig::default()
        };

        Ok(Self {
            config,
            config_path,
        })
    }

    pub fn save_config(&self) -> NvResult<()> {
        let toml_str = toml::to_string_pretty(&self.config).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize config: {}", e))
        })?;
        fs::write(&self.config_path, toml_str)?;
        Ok(())
    }

    pub fn enable(&mut self) -> NvResult<()> {
        self.config.enabled = true;
        self.save_config()?;
        self.setup_mangohud_integration()
    }

    pub fn disable(&mut self) -> NvResult<()> {
        self.config.enabled = false;
        self.save_config()
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    pub fn set_position(&mut self, position: OsdPosition) -> NvResult<()> {
        self.config.position = position;
        self.save_config()
    }

    pub fn add_metric(&mut self, metric: OsdMetric) -> NvResult<()> {
        if !self.config.metrics.contains(&metric) {
            self.config.metrics.push(metric);
            self.save_config()?;
        }
        Ok(())
    }

    pub fn remove_metric(&mut self, metric: &OsdMetric) -> NvResult<()> {
        self.config
            .metrics
            .retain(|m| std::mem::discriminant(m) != std::mem::discriminant(metric));
        self.save_config()
    }

    pub fn get_metrics(&self) -> &[OsdMetric] {
        &self.config.metrics
    }

    /// Set up MangoHud integration for OSD
    /// MangoHud is the de-facto standard for gaming OSD on Linux
    fn setup_mangohud_integration(&self) -> NvResult<()> {
        let mangohud_config_dir = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find config directory".into()))?
            .join("MangoHud");

        fs::create_dir_all(&mangohud_config_dir)?;
        let mangohud_config_path = mangohud_config_dir.join("MangoHud.conf");

        // Generate MangoHud config from our settings
        let mut config_lines = vec![
            "# nvcontrol OSD configuration".to_string(),
            "# Auto-generated - DO NOT EDIT MANUALLY".to_string(),
            "".to_string(),
        ];

        // Position
        match self.config.position {
            OsdPosition::TopLeft => {
                config_lines.push("position=top-left".to_string());
            }
            OsdPosition::TopRight => {
                config_lines.push("position=top-right".to_string());
            }
            OsdPosition::BottomLeft => {
                config_lines.push("position=bottom-left".to_string());
            }
            OsdPosition::BottomRight => {
                config_lines.push("position=bottom-right".to_string());
            }
            OsdPosition::Custom { x, y } => {
                config_lines.push(format!("position=custom"));
                config_lines.push(format!("custom_position={},{}", x, y));
            }
        }

        // Metrics
        for metric in &self.config.metrics {
            match metric {
                OsdMetric::Fps => config_lines.push("fps".to_string()),
                OsdMetric::Frametime => config_lines.push("frametime".to_string()),
                OsdMetric::GpuName => config_lines.push("gpu_name".to_string()),
                OsdMetric::GpuTemperature => config_lines.push("gpu_temp".to_string()),
                OsdMetric::GpuUtilization => config_lines.push("gpu_load".to_string()),
                OsdMetric::GpuMemoryUsed => config_lines.push("vram".to_string()),
                OsdMetric::GpuPowerDraw => config_lines.push("gpu_power".to_string()),
                OsdMetric::GpuFanSpeed => config_lines.push("fan".to_string()),
                OsdMetric::GpuClockSpeed => config_lines.push("gpu_core_clock".to_string()),
                OsdMetric::CpuTemperature => config_lines.push("cpu_temp".to_string()),
                OsdMetric::CpuUtilization => config_lines.push("cpu_load".to_string()),
                OsdMetric::RamUsed => config_lines.push("ram".to_string()),
                _ => {}
            }
        }

        // Appearance
        config_lines.push(format!("font_size={}", self.config.font_size));
        config_lines.push(format!(
            "background_alpha={}",
            self.config.background_opacity
        ));
        config_lines.push(format!(
            "update_interval={}",
            self.config.update_interval_ms
        ));

        fs::write(mangohud_config_path, config_lines.join("\n"))?;

        println!("✅ MangoHud OSD configuration written");
        println!("💡 Launch games with: mangohud <game_command>");
        println!("💡 Or set MANGOHUD=1 environment variable");

        Ok(())
    }

    /// Generate environment variables for enabling OSD in games
    pub fn get_env_vars(&self) -> HashMap<String, String> {
        let mut env = HashMap::new();

        if self.config.enabled {
            env.insert("MANGOHUD".to_string(), "1".to_string());
            env.insert("MANGOHUD_CONFIG".to_string(), "nvcontrol".to_string());
        }

        env
    }

    /// Check if MangoHud is installed
    pub fn check_mangohud_installed() -> bool {
        which::which("mangohud").is_ok()
    }

    /// Install MangoHud (distribution-specific)
    pub fn install_mangohud_instructions() -> String {
        "To enable OSD, install MangoHud:\n\
         - Arch: sudo pacman -S mangohud\n\
         - Ubuntu: sudo apt install mangohud\n\
         - Fedora: sudo dnf install mangohud\n\
         - Flatpak: flatpak install flathub org.freedesktop.Platform.VulkanLayer.MangoHud\n\
         \n\
         Then enable OSD with: nvctl osd enable"
            .to_string()
    }

    pub fn get_config(&self) -> &OsdConfig {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut OsdConfig {
        &mut self.config
    }
}

// Partial equality for OsdMetric (for contains checks)
impl PartialEq for OsdMetric {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
