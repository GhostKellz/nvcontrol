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
}

pub type NvResult<T> = Result<T, NvControlError>;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod config;
pub mod display;
pub mod fan;
pub mod gpu;
pub mod profiles;
pub mod tray;
pub mod vibrance;

// New advanced modules
pub mod container;
pub mod drivers;
pub mod gamescope;
pub mod latency;
pub mod overclocking;
pub mod power;
pub mod recording;
pub mod shaders;
pub mod theme;
pub mod upscaling;
pub mod vrr;

// CUDA development tools
pub mod cuda;

// Monitoring and TUI modules
pub mod monitoring;
pub mod tui;

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
