use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NvControlError {
    #[error("NVML not available: {0}")]
    NvmlNotAvailable(String),
    #[error("Display detection failed: {0}")]
    DisplayDetectionFailed(String),
    #[error("Vibrance control failed: {0}")]
    VibranceControlFailed(String),
    #[error("Fan control not supported")]
    FanControlNotSupported,
    #[error("Power management failed: {0}")]
    PowerManagementFailed(String),
    #[error("Latency optimization failed: {0}")]
    LatencyOptimizationFailed(String),
    #[error("Container operation failed: {0}")]
    ContainerOperationFailed(String),
    #[error("GPU query failed: {0}")]
    GpuQueryFailed(String),
    #[error("Command execution failed: {0}")]
    CommandFailed(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Unsupported feature: {0}")]
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
pub mod upscaling;

// CUDA development tools
pub mod cuda;

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
        assert_eq!(error.to_string(), "Fan control not supported");
    }
}
