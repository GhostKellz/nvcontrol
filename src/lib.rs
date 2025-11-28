use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NvControlError {
    #[error("NVML not available: {0}\n  → Ensure NVIDIA drivers are installed: nvidia-smi")]
    NvmlNotAvailable(String),

    #[error("Display detection failed: {0}\n  → Check display connection and driver status")]
    DisplayDetectionFailed(String),

    #[error("Vibrance control failed: {0}\n  → Try: nvidia-settings -q all | grep -i vibrance")]
    VibranceControlFailed(String),

    #[error("Fan control not supported on this GPU\n  → Manual fan control requires GPU with controllable fans")]
    FanControlNotSupported,

    #[error("Power management failed: {0}\n  → Try running with sudo for power limit changes")]
    PowerManagementFailed(String),

    #[error("Latency optimization failed: {0}")]
    LatencyOptimizationFailed(String),

    #[error("Container operation failed: {0}\n  → Check Docker/Podman status and NVIDIA Container Toolkit")]
    ContainerOperationFailed(String),

    #[error("GPU query failed: {0}\n  → Run 'nvidia-smi' to verify GPU status")]
    GpuQueryFailed(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("Configuration error: {0}\n  → Check ~/.config/nvcontrol/ for config files")]
    ConfigError(String),

    #[error("Unsupported feature: {0}\n  → This feature may require a different GPU or driver version")]
    UnsupportedFeature(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

pub type NvResult<T> = Result<T, NvControlError>;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod bolt_integration;
pub mod nvbind_integration;
pub mod config;
pub mod display;
pub mod fan;
pub mod gpu;
pub mod profiles;
pub mod tray;
pub mod vibrance;
pub mod vibrance_native;
pub mod nvkms_bindings;
pub mod display_controls;
pub mod vrr;
pub mod osd;
pub mod game_detection;
pub mod game_scanner;
pub mod notifications;
pub mod benchmark;
pub mod gui_widgets;
pub mod hdr;
pub mod setup;
pub mod display_info;

// New advanced modules
pub mod container;
pub mod container_runtime;
pub mod dlss;
pub mod drivers;
pub mod gamescope;
pub mod game_launcher;
pub mod gpu_passthrough;
pub mod latency;
pub mod overclocking;
pub mod power;
pub mod recording;
pub mod shaders;
pub mod theme;
pub mod themes;
pub mod upscaling;

// CUDA development tools
pub mod cuda;

// Phase 1: Core Stability & Safety
pub mod error_recovery;
pub mod hardware_safety;
pub mod gpu_safe;

// Phase 2: Wayland-First Experience
pub mod wayland_integration;

// Phase 3: Advanced GPU Control
pub mod enhanced_overclock;
pub mod intelligent_fan;
pub mod advanced_power;
pub mod advanced_multi_gpu;

// Phase 4: Gaming & Performance
pub mod gaming_integration;
pub mod performance_monitoring;
pub mod upscaling_tech;
pub mod advanced_display;

// Phase 5: Container & Virtualization
pub mod nvbind_api;
pub mod virtualization;
pub mod container_specific;

// ASUS ROG Integration
pub mod asus_gpu_tweak;
pub mod asus_aura;
pub mod asus_fan_control;
pub mod asus_power_detector;

// Monitoring and TUI modules
pub mod monitoring;
pub mod tui;

// Arch KDE Wayland optimization modules
pub mod wayland_nvidia;
pub mod kde_optimizer;
pub mod power_profiles_daemon;
pub mod arch_integration;
pub mod gsp_firmware;
pub mod multimonitor;

// Profile management
pub mod profile_manager;

// Multi-GPU support
pub mod multi_gpu;

// Power curves and scheduling
pub mod power_curves;

// Game profile auto-application
pub mod game_profile_auto;

// Automated overclocking
pub mod auto_overclock;

// GPU model specifications and optimized profiles
pub mod gpu_profiles;

// Gaming optimizations for Linux
pub mod gaming_optimizations;

// Per-monitor profiles for multi-monitor setups
pub mod monitor_profiles;

// System validation for RTX 50-series (ReBAR, PCIe Gen 5, etc.)
pub mod system_validation;

// RGB/ARGB LED control (ASUS Aura, OpenRGB, etc.)
pub mod rgb_control;

// Kernel driver information and optimization
pub mod kernel_driver;

// Modern gaming dashboard (MSI Afterburner + ASUS GPU Tweak style)
pub mod gaming_dashboard;

// GPU Profiler and Monitoring (radeon-profile equivalent)
pub mod nvidia_profiler;
pub mod tui_monitor;
pub mod gui_tuner;
pub mod gui_themes;

// Unified state management
pub mod state;

// Interactive CLI mode
pub mod interactive_cli;

// Enhanced error messages with solutions
pub mod error_messages;

// Re-export commonly used types
pub use config::Config;
pub use profiles::Profile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_error_types() {
        let error = NvControlError::FanControlNotSupported;
        assert!(error.to_string().contains("Fan control not supported"));
    }
}
