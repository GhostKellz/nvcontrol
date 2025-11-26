use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

/// Gaming-specific Linux optimizations for NVIDIA GPUs
/// Optimizations for Proton/Wine, native Linux games, and system-level tweaks

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingOptimizations {
    pub nvidia_settings: NvidiaGamingSettings,
    pub system_settings: SystemGamingSettings,
    pub proton_wine_settings: ProtonWineSettings,
    pub shader_cache_settings: ShaderCacheSettings,
    pub vulkan_settings: VulkanSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvidiaGamingSettings {
    pub power_mizer_mode: PowerMizerMode,
    pub threaded_optimization: bool,
    pub triple_buffer: bool,
    pub texture_filtering_quality: TextureFilteringQuality,
    pub anisotropic_filtering: AnisotropicFiltering,
    pub antialiasing_mode: AntiAliasingMode,
    pub sync_to_vblank: bool,
    pub allow_flipping: bool,
    pub image_sharpening: bool,
    pub image_sharpening_amount: u8, // 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PowerMizerMode {
    Adaptive = 0,          // Balance performance and power
    PreferMaxPerformance = 1, // Always max clocks
    AdaptivePerformance = 2,  // Aggressive power saving
    PreferConsistentPerformance = 3, // Consistent clocks
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TextureFilteringQuality {
    HighPerformance,
    Quality,
    HighQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnisotropicFiltering {
    Off = 0,
    X2 = 1,
    X4 = 2,
    X8 = 3,
    X16 = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AntiAliasingMode {
    Off = 0,
    Application = 1,
    Override2x = 2,
    Override4x = 3,
    Override8x = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemGamingSettings {
    pub cpu_governor: CpuGovernor,
    pub disable_compositor: bool, // For X11 gaming
    pub enable_game_mode: bool,   // systemd gamemode
    pub enable_latencyflex: bool,
    pub nice_priority: i8, // Process priority adjustment
    pub transparent_hugepages: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CpuGovernor {
    Performance,
    Schedutil,
    Ondemand,
    Powersave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtonWineSettings {
    pub enable_esync: bool,
    pub enable_fsync: bool,
    pub enable_async_shader_compilation: bool,
    pub dxvk_async: bool,
    pub proton_hide_nvidia_gpu: bool,
    pub proton_enable_nvapi: bool,
    pub wine_large_address_aware: bool,
    pub staging_shared_memory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderCacheSettings {
    pub enabled: bool,
    pub cache_path: String,
    pub max_size_mb: u32,
    pub precompile_on_launch: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulkanSettings {
    pub enable_pipeline_cache: bool,
    pub force_device_local_memory: bool,
    pub enable_graphics_pipeline_library: bool,
    pub enable_descriptor_indexing: bool,
    pub nvidia_threaded_optimization: bool,
}

impl Default for GamingOptimizations {
    fn default() -> Self {
        Self {
            nvidia_settings: NvidiaGamingSettings {
                power_mizer_mode: PowerMizerMode::PreferMaxPerformance,
                threaded_optimization: true,
                triple_buffer: false, // Disable for competitive games
                texture_filtering_quality: TextureFilteringQuality::Quality,
                anisotropic_filtering: AnisotropicFiltering::X16,
                antialiasing_mode: AntiAliasingMode::Application,
                sync_to_vblank: false, // Disable for max FPS
                allow_flipping: true,
                image_sharpening: false,
                image_sharpening_amount: 0,
            },
            system_settings: SystemGamingSettings {
                cpu_governor: CpuGovernor::Performance,
                disable_compositor: false, // Keep enabled for modern compositors
                enable_game_mode: true,
                enable_latencyflex: true,
                nice_priority: -10, // Higher priority for games
                transparent_hugepages: true,
            },
            proton_wine_settings: ProtonWineSettings {
                enable_esync: true,
                enable_fsync: true,
                enable_async_shader_compilation: true,
                dxvk_async: true,
                proton_hide_nvidia_gpu: false,
                proton_enable_nvapi: true,
                wine_large_address_aware: true,
                staging_shared_memory: true,
            },
            shader_cache_settings: ShaderCacheSettings {
                enabled: true,
                cache_path: "~/.cache/nvidia/GLCache".to_string(),
                max_size_mb: 2048,
                precompile_on_launch: true,
            },
            vulkan_settings: VulkanSettings {
                enable_pipeline_cache: true,
                force_device_local_memory: true,
                enable_graphics_pipeline_library: true,
                enable_descriptor_indexing: true,
                nvidia_threaded_optimization: true,
            },
        }
    }
}

impl GamingOptimizations {
    /// Apply all gaming optimizations
    pub fn apply(&self) -> NvResult<()> {
        println!("Applying gaming optimizations...");

        self.apply_nvidia_settings()?;
        self.apply_system_settings()?;
        self.apply_shader_cache_settings()?;

        println!("Gaming optimizations applied successfully!");
        Ok(())
    }

    /// Apply NVIDIA-specific settings
    fn apply_nvidia_settings(&self) -> NvResult<()> {
        if std::env::var("DISPLAY").is_err() {
            println!("No X11 display found, skipping nvidia-settings");
            return Ok(());
        }

        let settings = &self.nvidia_settings;
        let power_mode = settings.power_mizer_mode.clone() as u8;
        let commands = vec![
            // Power management
            format!(
                "nvidia-settings -a '[gpu:0]/GPUPowerMizerMode={}'",
                power_mode
            ),
            // OpenGL settings
            format!(
                "nvidia-settings -a '[opengl]/GLThreadedOptimizations={}'",
                if settings.threaded_optimization { 1 } else { 0 }
            ),
            format!(
                "nvidia-settings -a '[opengl]/TripleBuffer={}'",
                if settings.triple_buffer { 1 } else { 0 }
            ),
            format!(
                "nvidia-settings -a '[opengl]/SyncToVBlank={}'",
                if settings.sync_to_vblank { 1 } else { 0 }
            ),
            format!(
                "nvidia-settings -a '[opengl]/AllowFlipping={}'",
                if settings.allow_flipping { 1 } else { 0 }
            ),
        ];

        for cmd in commands {
            let _ = Command::new("sh").arg("-c").arg(&cmd).output();
        }

        Ok(())
    }

    /// Apply system-level optimizations
    fn apply_system_settings(&self) -> NvResult<()> {
        let settings = &self.system_settings;

        // Set CPU governor
        let governor = match settings.cpu_governor {
            CpuGovernor::Performance => "performance",
            CpuGovernor::Schedutil => "schedutil",
            CpuGovernor::Ondemand => "ondemand",
            CpuGovernor::Powersave => "powersave",
        };

        // Try to set CPU governor (may require root)
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "echo {} | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor",
                governor
            ))
            .output();

        // Enable transparent hugepages if requested
        if settings.transparent_hugepages {
            let _ = Command::new("sh")
                .arg("-c")
                .arg("echo always | sudo tee /sys/kernel/mm/transparent_hugepage/enabled")
                .output();
        }

        Ok(())
    }

    /// Apply shader cache settings
    fn apply_shader_cache_settings(&self) -> NvResult<()> {
        if !self.shader_cache_settings.enabled {
            return Ok(());
        }

        // Set NVIDIA shader cache environment variables
        // SAFETY: Setting environment variables for shader caching
        unsafe {
            std::env::set_var(
                "__GL_SHADER_DISK_CACHE",
                if self.shader_cache_settings.enabled {
                    "1"
                } else {
                    "0"
                },
            );
            std::env::set_var(
                "__GL_SHADER_DISK_CACHE_PATH",
                &self.shader_cache_settings.cache_path,
            );
            std::env::set_var(
                "__GL_SHADER_DISK_CACHE_SIZE",
                &format!("{}", self.shader_cache_settings.max_size_mb * 1024 * 1024),
            );
        }

        Ok(())
    }

    /// Generate environment variables for Proton/Wine games
    pub fn get_proton_env_vars(&self) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();
        let settings = &self.proton_wine_settings;

        // ESYNC/FSYNC
        if settings.enable_esync {
            env_vars.insert("PROTON_NO_ESYNC".to_string(), "0".to_string());
        }
        if settings.enable_fsync {
            env_vars.insert("PROTON_NO_FSYNC".to_string(), "0".to_string());
        }

        // DXVK async
        if settings.dxvk_async {
            env_vars.insert("DXVK_ASYNC".to_string(), "1".to_string());
        }

        // NVIDIA-specific
        if settings.proton_enable_nvapi {
            env_vars.insert("PROTON_ENABLE_NVAPI".to_string(), "1".to_string());
        }
        if settings.proton_hide_nvidia_gpu {
            env_vars.insert("PROTON_HIDE_NVIDIA_GPU".to_string(), "1".to_string());
        }

        // Wine staging
        if settings.staging_shared_memory {
            env_vars.insert("WINE_STAGING_SHARED_MEMORY".to_string(), "1".to_string());
        }

        // Large Address Aware
        if settings.wine_large_address_aware {
            env_vars.insert("WINE_LARGE_ADDRESS_AWARE".to_string(), "1".to_string());
        }

        // Vulkan
        if self.vulkan_settings.nvidia_threaded_optimization {
            env_vars.insert(
                "__GL_THREADED_OPTIMIZATIONS".to_string(),
                "1".to_string(),
            );
        }

        // Shader caching
        if self.shader_cache_settings.enabled {
            env_vars.insert("__GL_SHADER_DISK_CACHE".to_string(), "1".to_string());
            env_vars.insert(
                "__GL_SHADER_DISK_CACHE_PATH".to_string(),
                self.shader_cache_settings.cache_path.clone(),
            );
        }

        env_vars
    }

    /// Print current gaming optimizations
    pub fn print_status(&self) {
        println!("\n=== Gaming Optimizations Status ===");
        println!(
            "PowerMizer Mode: {:?}",
            self.nvidia_settings.power_mizer_mode
        );
        println!(
            "Threaded Optimization: {}",
            self.nvidia_settings.threaded_optimization
        );
        println!("Triple Buffer: {}", self.nvidia_settings.triple_buffer);
        println!("Sync to VBlank: {}", self.nvidia_settings.sync_to_vblank);
        println!("\nSystem Settings:");
        println!("CPU Governor: {:?}", self.system_settings.cpu_governor);
        println!("GameMode: {}", self.system_settings.enable_game_mode);
        println!(
            "LatencyFleX: {}",
            self.system_settings.enable_latencyflex
        );
        println!("\nProton/Wine Settings:");
        println!("ESYNC: {}", self.proton_wine_settings.enable_esync);
        println!("FSYNC: {}", self.proton_wine_settings.enable_fsync);
        println!("DXVK Async: {}", self.proton_wine_settings.dxvk_async);
        println!(
            "NVIDIA NVAPI: {}",
            self.proton_wine_settings.proton_enable_nvapi
        );
    }
}

/// Preset configurations for different gaming scenarios
pub fn get_gaming_presets() -> HashMap<String, GamingOptimizations> {
    let mut presets = HashMap::new();

    // Competitive gaming preset (max FPS, low latency)
    let mut competitive = GamingOptimizations::default();
    competitive.nvidia_settings.sync_to_vblank = false;
    competitive.nvidia_settings.triple_buffer = false;
    competitive.system_settings.enable_latencyflex = true;
    competitive.system_settings.nice_priority = -15;
    presets.insert("competitive".to_string(), competitive);

    // Visual quality preset (best graphics)
    let mut quality = GamingOptimizations::default();
    quality.nvidia_settings.texture_filtering_quality = TextureFilteringQuality::HighQuality;
    quality.nvidia_settings.antialiasing_mode = AntiAliasingMode::Override8x;
    quality.nvidia_settings.image_sharpening = true;
    quality.nvidia_settings.image_sharpening_amount = 50;
    presets.insert("quality".to_string(), quality);

    // Power saving preset
    let mut power_saving = GamingOptimizations::default();
    power_saving.nvidia_settings.power_mizer_mode = PowerMizerMode::Adaptive;
    power_saving.system_settings.cpu_governor = CpuGovernor::Ondemand;
    presets.insert("power_saving".to_string(), power_saving);

    // VR gaming preset
    let mut vr = GamingOptimizations::default();
    vr.nvidia_settings.sync_to_vblank = true; // Important for VR
    vr.nvidia_settings.power_mizer_mode = PowerMizerMode::PreferMaxPerformance;
    vr.system_settings.enable_latencyflex = true;
    vr.system_settings.nice_priority = -20;
    presets.insert("vr".to_string(), vr);

    presets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_optimizations() {
        let opts = GamingOptimizations::default();
        assert_eq!(
            opts.nvidia_settings.power_mizer_mode,
            PowerMizerMode::PreferMaxPerformance
        );
        assert!(opts.nvidia_settings.threaded_optimization);
        assert!(opts.proton_wine_settings.enable_fsync);
    }

    #[test]
    fn test_gaming_presets() {
        let presets = get_gaming_presets();
        assert!(presets.contains_key("competitive"));
        assert!(presets.contains_key("quality"));
        assert!(presets.contains_key("power_saving"));
        assert!(presets.contains_key("vr"));
    }

    #[test]
    fn test_proton_env_vars() {
        let opts = GamingOptimizations::default();
        let env_vars = opts.get_proton_env_vars();
        assert!(env_vars.contains_key("PROTON_ENABLE_NVAPI"));
        assert!(env_vars.contains_key("DXVK_ASYNC"));
    }
}
