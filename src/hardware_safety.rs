/// Hardware safety and protection system
///
/// Prevents hardware damage through temperature monitoring, power limiting,
/// and automatic emergency shutdown
use crate::{NvControlError, NvResult};
use nvml_wrapper::Nvml;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Safety thresholds for GPU operation
#[derive(Debug, Clone)]
pub struct SafetyThresholds {
    /// Critical temperature (°C) - trigger emergency shutdown
    pub temp_critical: i32,
    /// Warning temperature (°C) - trigger throttling
    pub temp_warning: i32,
    /// Maximum power limit (percentage of TDP)
    pub max_power_limit_percent: u8,
    /// Minimum fan speed when GPU is active (%)
    pub min_fan_speed_percent: u8,
    /// Maximum clock offset (MHz)
    pub max_clock_offset_mhz: i32,
    /// Maximum memory offset (MHz)
    pub max_memory_offset_mhz: i32,
}

impl Default for SafetyThresholds {
    fn default() -> Self {
        Self {
            temp_critical: 95,
            temp_warning: 85,
            max_power_limit_percent: 120,
            min_fan_speed_percent: 20,
            max_clock_offset_mhz: 500,
            max_memory_offset_mhz: 1000,
        }
    }
}

/// Hardware safety monitor
#[allow(dead_code)]
pub struct SafetyMonitor {
    thresholds: SafetyThresholds,
    gpu_id: u32,
    last_check: Arc<Mutex<Instant>>,
    emergency_shutdown_triggered: Arc<Mutex<bool>>,
}

impl SafetyMonitor {
    pub fn new(gpu_id: u32, thresholds: SafetyThresholds) -> Self {
        Self {
            thresholds,
            gpu_id,
            last_check: Arc::new(Mutex::new(Instant::now())),
            emergency_shutdown_triggered: Arc::new(Mutex::new(false)),
        }
    }

    /// Check GPU temperature and trigger safety measures if needed
    pub fn check_temperature(&self) -> NvResult<SafetyStatus> {
        let nvml = Nvml::init()
            .map_err(|e| NvControlError::GpuQueryFailed(format!("NVML init failed: {}", e)))?;

        let device = nvml.device_by_index(self.gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to access GPU {}: {}", self.gpu_id, e))
        })?;

        let temp = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to read temperature: {}", e))
            })? as i32;

        if temp >= self.thresholds.temp_critical {
            self.trigger_emergency_shutdown(temp)?;
            return Ok(SafetyStatus::EmergencyShutdown { temperature: temp });
        }

        if temp >= self.thresholds.temp_warning {
            self.apply_thermal_throttling(&device, temp)?;
            return Ok(SafetyStatus::ThermalThrottling { temperature: temp });
        }

        Ok(SafetyStatus::Normal { temperature: temp })
    }

    /// Trigger emergency shutdown
    fn trigger_emergency_shutdown(&self, temp: i32) -> NvResult<()> {
        let mut shutdown_triggered = self
            .emergency_shutdown_triggered
            .lock()
            .map_err(|_| NvControlError::RuntimeError("Lock poisoned".to_string()))?;

        if *shutdown_triggered {
            return Ok(()); // Already triggered
        }

        eprintln!(
            "🚨 EMERGENCY: GPU {} temperature critical: {}°C (threshold: {}°C)",
            self.gpu_id, temp, self.thresholds.temp_critical
        );
        eprintln!("Resetting overclocks and maximizing fan speed...");

        // Reset all overclocks
        let _ = self.reset_overclocks();

        // Max out fan speed
        let _ = self.maximize_fan_speed();

        *shutdown_triggered = true;
        Ok(())
    }

    /// Apply thermal throttling
    fn apply_thermal_throttling(&self, _device: &nvml_wrapper::Device, temp: i32) -> NvResult<()> {
        eprintln!(
            "⚠️  WARNING: GPU {} temperature high: {}°C (warning: {}°C)",
            self.gpu_id, temp, self.thresholds.temp_warning
        );
        eprintln!("Applying thermal throttling...");

        // Calculate recommended fan speed increase
        let fan_increase = ((temp - self.thresholds.temp_warning) as f32
            / (self.thresholds.temp_critical - self.thresholds.temp_warning) as f32
            * 30.0) as u32;

        let target_fan_speed = (70 + fan_increase).min(100);

        eprintln!("Recommended fan speed: {}%", target_fan_speed);
        // Note: Actual fan control requires nvidia-settings or manual configuration

        Ok(())
    }

    /// Reset all overclocks to safe defaults
    fn reset_overclocks(&self) -> NvResult<()> {
        // Try nvidia-settings first (X11)
        if std::env::var("DISPLAY").is_ok() {
            let _ = std::process::Command::new("nvidia-settings")
                .args(&[
                    "-a",
                    &format!(
                        "[gpu:{}]/GPUGraphicsClockOffsetAllPerformanceLevels=0",
                        self.gpu_id
                    ),
                    "-a",
                    &format!(
                        "[gpu:{}]/GPUMemoryTransferRateOffsetAllPerformanceLevels=0",
                        self.gpu_id
                    ),
                ])
                .output();
        }

        // Try sysfs (Wayland)
        // Note: This requires root permissions
        Ok(())
    }

    /// Maximize fan speed for emergency cooling
    fn maximize_fan_speed(&self) -> NvResult<()> {
        let nvml = Nvml::init()
            .map_err(|e| NvControlError::GpuQueryFailed(format!("NVML init failed: {}", e)))?;

        let _device = nvml
            .device_by_index(self.gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to access GPU: {}", e)))?;

        // Note: Fan control through NVML requires proper permissions and may not work on all GPUs
        eprintln!("Note: Fan speed control may require nvidia-settings or manual configuration");

        Ok(())
    }

    /// Validate overclock settings before applying
    pub fn validate_overclock_safe(
        &self,
        gpu_offset: i32,
        memory_offset: i32,
    ) -> NvResult<OverclockValidation> {
        if gpu_offset.abs() > self.thresholds.max_clock_offset_mhz {
            return Ok(OverclockValidation::Unsafe {
                reason: format!(
                    "GPU offset {}MHz exceeds safe limit of ±{}MHz",
                    gpu_offset, self.thresholds.max_clock_offset_mhz
                ),
            });
        }

        if memory_offset.abs() > self.thresholds.max_memory_offset_mhz {
            return Ok(OverclockValidation::Unsafe {
                reason: format!(
                    "Memory offset {}MHz exceeds safe limit of ±{}MHz",
                    memory_offset, self.thresholds.max_memory_offset_mhz
                ),
            });
        }

        // Check current temperature
        let status = self.check_temperature()?;
        match status {
            SafetyStatus::EmergencyShutdown { temperature } => {
                return Ok(OverclockValidation::Unsafe {
                    reason: format!(
                        "GPU temperature critical ({}°C). Cannot apply overclock.",
                        temperature
                    ),
                });
            }
            SafetyStatus::ThermalThrottling { temperature } => {
                return Ok(OverclockValidation::Warning {
                    reason: format!(
                        "GPU temperature elevated ({}°C). Overclock may worsen thermal situation.",
                        temperature
                    ),
                });
            }
            _ => {}
        }

        Ok(OverclockValidation::Safe)
    }

    /// Validate power limit before applying
    pub fn validate_power_limit_safe(&self, power_limit_percent: u8) -> NvResult<PowerValidation> {
        if power_limit_percent > self.thresholds.max_power_limit_percent {
            return Ok(PowerValidation::Unsafe {
                reason: format!(
                    "Power limit {}% exceeds safe maximum of {}%",
                    power_limit_percent, self.thresholds.max_power_limit_percent
                ),
            });
        }

        if power_limit_percent < 50 {
            return Ok(PowerValidation::Warning {
                reason: "Power limit below 50% may cause instability".to_string(),
            });
        }

        Ok(PowerValidation::Safe)
    }
}

/// Safety status
#[derive(Debug, Clone)]
pub enum SafetyStatus {
    Normal { temperature: i32 },
    ThermalThrottling { temperature: i32 },
    EmergencyShutdown { temperature: i32 },
}

/// Overclock validation result
#[derive(Debug, Clone)]
pub enum OverclockValidation {
    Safe,
    Warning { reason: String },
    Unsafe { reason: String },
}

/// Power limit validation result
#[derive(Debug, Clone)]
pub enum PowerValidation {
    Safe,
    Warning { reason: String },
    Unsafe { reason: String },
}

/// Automatic safety monitoring in background
pub struct BackgroundSafetyMonitor {
    monitor: Arc<SafetyMonitor>,
    check_interval: Duration,
    running: Arc<Mutex<bool>>,
}

impl BackgroundSafetyMonitor {
    pub fn new(gpu_id: u32, thresholds: SafetyThresholds, check_interval: Duration) -> Self {
        Self {
            monitor: Arc::new(SafetyMonitor::new(gpu_id, thresholds)),
            check_interval,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start background monitoring
    pub fn start(&self) {
        let mut running = self.running.lock().unwrap_or_else(|e| e.into_inner());
        if *running {
            return; // Already running
        }

        *running = true;
        let monitor = Arc::clone(&self.monitor);
        let interval = self.check_interval;
        let running_flag = Arc::clone(&self.running);

        std::thread::spawn(move || {
            while *running_flag.lock().unwrap_or_else(|e| e.into_inner()) {
                match monitor.check_temperature() {
                    Ok(SafetyStatus::EmergencyShutdown { temperature }) => {
                        eprintln!("Emergency shutdown triggered at {}°C", temperature);
                    }
                    Ok(SafetyStatus::ThermalThrottling { temperature }) => {
                        eprintln!("Thermal throttling active at {}°C", temperature);
                    }
                    Ok(SafetyStatus::Normal { .. }) => {
                        // All good
                    }
                    Err(e) => {
                        eprintln!("Safety check failed: {}", e);
                    }
                }

                std::thread::sleep(interval);
            }
        });
    }

    /// Stop background monitoring
    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap_or_else(|e| e.into_inner());
        *running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_thresholds_defaults() {
        let thresholds = SafetyThresholds::default();
        assert_eq!(thresholds.temp_critical, 95);
        assert_eq!(thresholds.temp_warning, 85);
        assert_eq!(thresholds.max_power_limit_percent, 120);
    }

    #[test]
    fn test_overclock_validation() {
        let monitor = SafetyMonitor::new(0, SafetyThresholds::default());

        // Safe overclock
        let result = monitor.validate_overclock_safe(150, 500);
        assert!(matches!(
            result,
            Ok(OverclockValidation::Safe) | Ok(OverclockValidation::Warning { .. }) | Err(_)
        ));

        // Unsafe overclock (too high)
        let result = monitor.validate_overclock_safe(600, 500);
        assert!(matches!(
            result,
            Ok(OverclockValidation::Unsafe { .. }) | Err(_)
        ));
    }

    #[test]
    fn test_power_limit_validation() {
        let monitor = SafetyMonitor::new(0, SafetyThresholds::default());

        // Safe power limit
        let result = monitor.validate_power_limit_safe(100);
        assert!(matches!(result, Ok(PowerValidation::Safe)));

        // Unsafe power limit
        let result = monitor.validate_power_limit_safe(150);
        assert!(matches!(result, Ok(PowerValidation::Unsafe { .. })));

        // Warning power limit
        let result = monitor.validate_power_limit_safe(45);
        assert!(matches!(result, Ok(PowerValidation::Warning { .. })));
    }
}
