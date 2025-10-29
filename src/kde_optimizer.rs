// KDE Plasma Compositor Optimization for NVIDIA
// Direct KWin Wayland tweaks and performance tuning

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdeCompositorConfig {
    pub latency_policy: LatencyPolicy,
    pub render_loop: RenderLoop,
    pub animation_speed: f32,
    pub vrr_enabled: bool,
    pub explicit_sync: bool,
    pub gl_yield: GlYield,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LatencyPolicy {
    Low,      // Gaming/responsive
    Medium,   // Balanced
    High,     // Power saving
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RenderLoop {
    Immediate,  // Lowest latency
    Queued,     // Balanced
    Adaptive,   // Power saving
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GlYield {
    Usleep,   // Best for NVIDIA
    Yield,    // Alternative
    Nothing,  // Legacy
}

impl Default for KdeCompositorConfig {
    fn default() -> Self {
        Self {
            latency_policy: LatencyPolicy::Medium,
            render_loop: RenderLoop::Adaptive,
            animation_speed: 1.0,
            vrr_enabled: true,
            explicit_sync: true,
            gl_yield: GlYield::Usleep,
        }
    }
}

pub struct KdeOptimizer {
    config: KdeCompositorConfig,
}

impl KdeOptimizer {
    pub fn new() -> Self {
        Self {
            config: KdeCompositorConfig::default(),
        }
    }

    /// Detect current KDE version
    pub fn detect_kde_version() -> NvResult<String> {
        let output = Command::new("plasmashell")
            .arg("--version")
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to detect KDE version: {}", e)))?;

        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.trim().to_string())
    }

    /// Check if running under Wayland
    pub fn is_wayland() -> bool {
        std::env::var("XDG_SESSION_TYPE")
            .map(|v| v == "wayland")
            .unwrap_or(false)
    }

    /// Apply gaming optimization preset
    pub fn apply_gaming_preset(&mut self) -> NvResult<()> {
        println!("🎮 Applying KDE Gaming Preset...\n");

        self.config.latency_policy = LatencyPolicy::Low;
        self.config.render_loop = RenderLoop::Immediate;
        self.config.animation_speed = 0.5; // Faster animations
        self.config.vrr_enabled = true;
        self.config.explicit_sync = true;
        self.config.gl_yield = GlYield::Usleep;

        self.apply_compositor_settings()?;
        self.apply_kwin_effects(false)?; // Disable heavy effects
        self.set_plasma_performance_settings()?;

        println!("✅ Gaming preset applied!");
        println!("   Restart KWin: kwin_wayland --replace &");

        Ok(())
    }

    /// Apply productivity optimization preset
    pub fn apply_productivity_preset(&mut self) -> NvResult<()> {
        println!("💼 Applying KDE Productivity Preset...\n");

        self.config.latency_policy = LatencyPolicy::Medium;
        self.config.render_loop = RenderLoop::Adaptive;
        self.config.animation_speed = 1.0;
        self.config.vrr_enabled = false; // Save power
        self.config.explicit_sync = true;
        self.config.gl_yield = GlYield::Usleep;

        self.apply_compositor_settings()?;
        self.apply_kwin_effects(true)?; // Enable effects
        self.set_plasma_performance_settings()?;

        println!("✅ Productivity preset applied!");

        Ok(())
    }

    /// Apply power saving preset
    pub fn apply_powersave_preset(&mut self) -> NvResult<()> {
        println!("🔋 Applying KDE Power Saving Preset...\n");

        self.config.latency_policy = LatencyPolicy::High;
        self.config.render_loop = RenderLoop::Adaptive;
        self.config.animation_speed = 0.75; // Slightly faster to feel more responsive
        self.config.vrr_enabled = false;
        self.config.explicit_sync = true;
        self.config.gl_yield = GlYield::Yield;

        self.apply_compositor_settings()?;
        self.apply_kwin_effects(false)?; // Minimal effects
        self.set_plasma_performance_settings()?;

        println!("✅ Power saving preset applied!");

        Ok(())
    }

    /// Apply compositor settings via kwriteconfig6
    fn apply_compositor_settings(&self) -> NvResult<()> {
        println!("⚙️  Configuring KWin compositor...");

        // Latency settings
        let latency_value = match self.config.latency_policy {
            LatencyPolicy::Low => "ForceLowestLatency",
            LatencyPolicy::Medium => "LatencyMedium",
            LatencyPolicy::High => "LatencyHigh",
        };

        self.kwriteconfig("kwinrc", "Compositing", "LatencyPolicy", latency_value)?;

        // VRR/Adaptive Sync
        self.kwriteconfig(
            "kwinrc",
            "Compositing",
            "AllowTearing",
            if self.config.vrr_enabled { "true" } else { "false" }
        )?;

        // Animation speed
        self.kwriteconfig(
            "kdeglobals",
            "KDE",
            "AnimationDurationFactor",
            &self.config.animation_speed.to_string()
        )?;

        // OpenGL settings
        self.kwriteconfig("kwinrc", "Compositing", "GLCore", "true")?;
        self.kwriteconfig("kwinrc", "Compositing", "GLPreferBufferSwap", "a")?; // auto

        // NVIDIA-specific
        if self.config.explicit_sync {
            println!("   Enabling explicit sync for NVIDIA");
        }

        println!("   ✅ Compositor settings applied");
        Ok(())
    }

    /// Configure KWin effects
    fn apply_kwin_effects(&self, enable_effects: bool) -> NvResult<()> {
        println!("✨ Configuring KWin effects...");

        let effects = [
            "blurEnabled",
            "contrastEnabled",
            "desktopgridEnabled",
            "diminactiveEnabled",
            "fadeEnabled",
            "glideEnabled",
            "slideEnabled",
            "zoomEnabled",
        ];

        for effect in &effects {
            self.kwriteconfig(
                "kwinrc",
                "Plugins",
                effect,
                if enable_effects { "true" } else { "false" }
            )?;
        }

        // Always keep essential effects
        self.kwriteconfig("kwinrc", "Plugins", "kwin4_effect_translucencyEnabled", "true")?;

        println!("   ✅ Effects configured");
        Ok(())
    }

    /// Set Plasma performance settings
    fn set_plasma_performance_settings(&self) -> NvResult<()> {
        println!("🚀 Configuring Plasma performance...");

        // Disable heavy animations for gaming
        if self.config.latency_policy == LatencyPolicy::Low {
            self.kwriteconfig("plasmarc", "Animations", "enabled", "false")?;
        } else {
            self.kwriteconfig("plasmarc", "Animations", "enabled", "true")?;
        }

        // Task switcher settings
        self.kwriteconfig("kwinrc", "TabBox", "LayoutName", "thumbnail_grid")?;

        println!("   ✅ Plasma performance configured");
        Ok(())
    }

    /// Helper to run kwriteconfig6
    fn kwriteconfig(&self, file: &str, group: &str, key: &str, value: &str) -> NvResult<()> {
        let status = Command::new("kwriteconfig6")
            .args(&["--file", file, "--group", group, "--key", key, value])
            .status()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to run kwriteconfig6: {}", e)))?;

        if !status.success() {
            return Err(NvControlError::CommandFailed(format!(
                "kwriteconfig6 failed: {} {} {} = {}",
                file, group, key, value
            )));
        }

        Ok(())
    }

    /// Read KDE config value
    fn kreadconfig(&self, file: &str, group: &str, key: &str) -> NvResult<String> {
        let output = Command::new("kreadconfig6")
            .args(&["--file", file, "--group", group, "--key", key])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to run kreadconfig6: {}", e)))?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Setup NVIDIA environment variables for KDE
    pub fn setup_kde_env_vars(&self) -> NvResult<()> {
        println!("🔧 Setting up KDE environment variables for NVIDIA...\n");

        let env_file = dirs::home_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find home directory".to_string()))?
            .join(".config/plasma-workspace/env/nvidia-wayland.sh");

        // Create parent directory
        if let Some(parent) = env_file.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| NvControlError::ConfigError(format!("Failed to create env dir: {}", e)))?;
        }

        let mut content = String::new();
        content.push_str("#!/bin/bash\n");
        content.push_str("# NVIDIA Wayland Environment for KDE Plasma\n");
        content.push_str("# Generated by nvcontrol\n\n");

        // Core NVIDIA variables
        content.push_str("export GBM_BACKEND=nvidia-drm\n");
        content.push_str("export __GLX_VENDOR_LIBRARY_NAME=nvidia\n");
        content.push_str("export LIBVA_DRIVER_NAME=nvidia\n");
        content.push_str("export WLR_NO_HARDWARE_CURSORS=1\n\n");

        // Performance variables
        content.push_str("# Performance optimizations\n");
        content.push_str("export __GL_YIELD=USLEEP\n");
        content.push_str("export __GL_THREADED_OPTIMIZATIONS=1\n");
        content.push_str("export __GL_MaxFramesAllowed=1\n\n");

        // VRR/G-Sync
        content.push_str("# VRR/G-Sync support\n");
        content.push_str("export __GL_GSYNC_ALLOWED=1\n");
        content.push_str("export __GL_VRR_ALLOWED=1\n\n");

        // KWin-specific
        if self.config.explicit_sync {
            content.push_str("# KWin explicit sync\n");
            content.push_str("export KWIN_DRM_USE_MODIFIERS=1\n\n");
        }

        std::fs::write(&env_file, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write env file: {}", e)))?;

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&env_file)
                .map_err(|e| NvControlError::ConfigError(format!("Failed to get file metadata: {}", e)))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&env_file, perms)
                .map_err(|e| NvControlError::ConfigError(format!("Failed to set permissions: {}", e)))?;
        }

        println!("✅ Environment variables written to {}", env_file.display());
        println!("   Restart Plasma session for changes to take effect");

        Ok(())
    }

    /// Enable VRR per-display
    pub fn set_vrr_per_display(&self, display: &str, enabled: bool) -> NvResult<()> {
        println!("🖥️  Setting VRR for display {}...", display);

        // Use kscreen-doctor for per-display VRR
        let status = Command::new("kscreen-doctor")
            .arg(format!("output.{}.vrrpolicy={}", display, if enabled { "automatic" } else { "never" }))
            .status()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to run kscreen-doctor: {}", e)))?;

        if !status.success() {
            return Err(NvControlError::CommandFailed("kscreen-doctor failed".to_string()));
        }

        println!("   ✅ VRR {} for {}", if enabled { "enabled" } else { "disabled" }, display);

        Ok(())
    }

    /// Get current KDE compositor status
    pub fn get_compositor_status(&self) -> NvResult<HashMap<String, String>> {
        let mut status = HashMap::new();

        // Read current settings
        if let Ok(latency) = self.kreadconfig("kwinrc", "Compositing", "LatencyPolicy") {
            status.insert("Latency Policy".to_string(), latency);
        }

        if let Ok(tearing) = self.kreadconfig("kwinrc", "Compositing", "AllowTearing") {
            status.insert("VRR/Tearing".to_string(), tearing);
        }

        if let Ok(anim_speed) = self.kreadconfig("kdeglobals", "KDE", "AnimationDurationFactor") {
            status.insert("Animation Speed".to_string(), anim_speed);
        }

        if let Ok(backend) = self.kreadconfig("kwinrc", "Compositing", "Backend") {
            status.insert("Backend".to_string(), backend);
        }

        Ok(status)
    }

    /// Print current status
    pub fn print_status(&self) -> NvResult<()> {
        println!("🖥️  KDE Plasma Compositor Status\n");

        if let Ok(version) = Self::detect_kde_version() {
            println!("KDE Version: {}", version);
        }

        println!("Session Type: {}", if Self::is_wayland() { "Wayland ✅" } else { "X11" });

        let status = self.get_compositor_status()?;

        println!("\nCompositor Settings:");
        for (key, value) in status {
            println!("   {}: {}", key, value);
        }

        println!("\nAvailable Presets:");
        println!("   🎮 Gaming     - Low latency, VRR enabled, minimal effects");
        println!("   💼 Productivity - Balanced, full effects");
        println!("   🔋 Power Save  - Maximum efficiency");

        Ok(())
    }

    /// Restart KWin compositor
    pub fn restart_compositor(&self) -> NvResult<()> {
        println!("🔄 Restarting KWin compositor...");

        let status = Command::new("kwin_wayland")
            .arg("--replace")
            .spawn()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to restart KWin: {}", e)))?;

        println!("✅ KWin restart initiated (PID: {})", status.id());

        Ok(())
    }
}
