/// Error recovery and graceful degradation system
///
/// Provides automatic fallback mechanisms and user-friendly error handling

use crate::{NvControlError, NvResult};
use std::process::Command;

/// Error recovery strategy
#[derive(Debug, Clone, Copy)]
pub enum RecoveryStrategy {
    /// Try alternative method
    Fallback,
    /// Use cached value
    Cache,
    /// Use safe default
    Default,
    /// Fail gracefully with message
    Graceful,
}

/// NVML fallback handler
pub struct NvmlFallback {
    nvml_available: bool,
    nvidia_settings_available: bool,
    sysfs_available: bool,
}

impl Default for NvmlFallback {
    fn default() -> Self {
        Self::new()
    }
}

impl NvmlFallback {
    pub fn new() -> Self {
        Self {
            nvml_available: Self::check_nvml(),
            nvidia_settings_available: Self::check_nvidia_settings(),
            sysfs_available: Self::check_sysfs(),
        }
    }

    fn check_nvml() -> bool {
        match nvml_wrapper::Nvml::init() {
            Ok(nvml) => nvml.device_count().is_ok(),
            Err(_) => false,
        }
    }

    fn check_nvidia_settings() -> bool {
        std::env::var("DISPLAY").is_ok()
            && Command::new("nvidia-settings")
                .arg("--version")
                .output()
                .is_ok()
    }

    fn check_sysfs() -> bool {
        std::path::Path::new("/sys/class/hwmon").exists()
    }

    /// Get available methods for GPU control
    pub fn available_methods(&self) -> Vec<&str> {
        let mut methods = Vec::new();
        if self.nvml_available {
            methods.push("NVML");
        }
        if self.nvidia_settings_available {
            methods.push("nvidia-settings");
        }
        if self.sysfs_available {
            methods.push("sysfs");
        }
        methods
    }

    /// Check if any control method is available
    pub fn has_any_method(&self) -> bool {
        self.nvml_available || self.nvidia_settings_available || self.sysfs_available
    }

    /// Get primary method
    pub fn primary_method(&self) -> Option<&str> {
        if self.nvml_available {
            Some("NVML")
        } else if self.nvidia_settings_available {
            Some("nvidia-settings")
        } else if self.sysfs_available {
            Some("sysfs")
        } else {
            None
        }
    }
}

/// Error context for better error messages
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub gpu_id: Option<u32>,
    pub recovery_suggestion: String,
}

impl ErrorContext {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            gpu_id: None,
            recovery_suggestion: String::new(),
        }
    }

    pub fn with_gpu(mut self, gpu_id: u32) -> Self {
        self.gpu_id = Some(gpu_id);
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.recovery_suggestion = suggestion.into();
        self
    }

    /// Convert error to user-friendly message
    pub fn to_user_message(&self, error: &NvControlError) -> String {
        let mut msg = format!("Failed to {}: {}", self.operation, error);

        if let Some(gpu) = self.gpu_id {
            msg.push_str(&format!(" (GPU {})", gpu));
        }

        if !self.recovery_suggestion.is_empty() {
            msg.push_str(&format!("\n\nSuggestion: {}", self.recovery_suggestion));
        }

        msg
    }
}

/// Retry failed operations with exponential backoff
pub struct RetryHandler {
    max_attempts: u32,
    base_delay_ms: u64,
}

impl Default for RetryHandler {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 100,
        }
    }
}

impl RetryHandler {
    pub fn new(max_attempts: u32, base_delay_ms: u64) -> Self {
        Self {
            max_attempts,
            base_delay_ms,
        }
    }

    /// Retry operation with exponential backoff
    pub fn retry<T, F>(&self, mut operation: F) -> NvResult<T>
    where
        F: FnMut() -> NvResult<T>,
    {
        let mut attempt = 0;

        loop {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;
                    if attempt >= self.max_attempts {
                        return Err(e);
                    }

                    // Exponential backoff
                    let delay = self.base_delay_ms * 2u64.pow(attempt - 1);
                    std::thread::sleep(std::time::Duration::from_millis(delay));
                }
            }
        }
    }
}

/// Error logging and telemetry (privacy-safe)
pub struct ErrorLogger {
    log_path: Option<std::path::PathBuf>,
}

impl ErrorLogger {
    pub fn new() -> Self {
        let log_path = dirs::cache_dir().map(|p| p.join("nvcontrol/errors.log"));

        if let Some(path) = &log_path {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
        }

        Self { log_path }
    }

    /// Log error (no sensitive data)
    pub fn log_error(&self, context: &ErrorContext, error: &NvControlError) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!(
            "[{}] Operation: {} | Error: {:?} | GPU: {:?}\n",
            timestamp, context.operation, error, context.gpu_id
        );

        if let Some(path) = &self.log_path {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .and_then(|mut file| {
                    use std::io::Write;
                    file.write_all(log_entry.as_bytes())
                });
        }

        // Also log to console in debug builds
        #[cfg(debug_assertions)]
        eprintln!("{}", log_entry);
    }
}

impl Default for ErrorLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced error handling with recovery
pub fn handle_nvml_error<T>(
    result: Result<T, nvml_wrapper::error::NvmlError>,
    context: ErrorContext,
) -> NvResult<T> {
    match result {
        Ok(value) => Ok(value),
        Err(nvml_error) => {
            let error = match nvml_error {
                nvml_wrapper::error::NvmlError::LibloadingError(_) => {
                    NvControlError::NvmlNotAvailable(
                        "NVML library not found. Install NVIDIA drivers.".to_string(),
                    )
                }
                nvml_wrapper::error::NvmlError::Uninitialized => {
                    NvControlError::NvmlNotAvailable(
                        "NVML not initialized. Check NVIDIA driver installation.".to_string(),
                    )
                }
                nvml_wrapper::error::NvmlError::NotSupported => {
                    NvControlError::UnsupportedFeature(format!(
                        "{} not supported on this GPU",
                        context.operation
                    ))
                }
                nvml_wrapper::error::NvmlError::NoPermission => {
                    NvControlError::RuntimeError(format!(
                        "{} requires elevated permissions. Try running with sudo.",
                        context.operation
                    ))
                }
                _ => NvControlError::GpuQueryFailed(format!(
                    "{}: {}",
                    context.operation, nvml_error
                )),
            };

            // Log error
            let logger = ErrorLogger::new();
            logger.log_error(&context, &error);

            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvml_fallback_detection() {
        let fallback = NvmlFallback::new();
        assert!(fallback.has_any_method() || !fallback.nvml_available);
    }

    #[test]
    fn test_error_context_message() {
        let ctx = ErrorContext::new("get GPU temperature")
            .with_gpu(0)
            .with_suggestion("Check NVIDIA driver installation");

        let error = NvControlError::GpuQueryFailed("NVML init failed".to_string());
        let msg = ctx.to_user_message(&error);

        assert!(msg.contains("get GPU temperature"));
        assert!(msg.contains("GPU 0"));
        assert!(msg.contains("Check NVIDIA driver installation"));
    }

    #[test]
    fn test_retry_handler() {
        let handler = RetryHandler::new(3, 10);
        let mut attempt_count = 0;

        let result = handler.retry(|| {
            attempt_count += 1;
            if attempt_count < 2 {
                Err(NvControlError::GpuQueryFailed("Transient error".to_string()))
            } else {
                Ok(42)
            }
        });

        assert!(result.is_ok());
        assert_eq!(attempt_count, 2);
    }
}
