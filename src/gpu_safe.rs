/// Safe GPU operations with automatic error recovery and hardware protection
///
/// This module wraps GPU operations with comprehensive error handling,
/// automatic fallbacks, and hardware safety checks
use crate::{
    NvControlError, NvResult,
    error_recovery::{ErrorContext, NvmlFallback, RetryHandler, handle_nvml_error},
    hardware_safety::{SafetyMonitor, SafetyThresholds},
};
use nvml_wrapper::Nvml;
use serde::{Deserialize, Serialize};

/// Enhanced GPU information with architecture detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfoEnhanced {
    pub id: u32,
    pub name: String,
    pub pci_address: String,
    pub driver_version: String,
    pub cuda_version: Option<String>,
    pub memory_total_mb: u64,
    pub memory_used_mb: u64,
    pub temperature: i32,
    pub power_draw_watts: f32,
    pub power_limit_watts: f32,
    pub fan_speed_percent: u32,
    pub gpu_utilization: u32,
    pub memory_utilization: u32,
    pub clock_gpu_mhz: u32,
    pub clock_memory_mhz: u32,
    pub architecture: Option<String>,
    pub compute_capability: Option<(u32, u32)>,
}

/// Safe GPU controller with automatic fallbacks
pub struct SafeGpuController {
    gpu_id: u32,
    fallback: NvmlFallback,
    retry_handler: RetryHandler,
    safety_monitor: SafetyMonitor,
}

impl SafeGpuController {
    pub fn new(gpu_id: u32) -> Self {
        Self {
            gpu_id,
            fallback: NvmlFallback::new(),
            retry_handler: RetryHandler::default(),
            safety_monitor: SafetyMonitor::new(gpu_id, SafetyThresholds::default()),
        }
    }

    /// Get GPU information with automatic fallback
    pub fn get_info(&self) -> NvResult<GpuInfoEnhanced> {
        let _ctx = ErrorContext::new("get GPU information")
            .with_gpu(self.gpu_id)
            .with_suggestion("Check NVIDIA driver installation with: nvidia-smi");

        // Try NVML first
        if self.fallback.available_methods().contains(&"NVML") {
            match self.get_info_nvml() {
                Ok(info) => return Ok(info),
                Err(e) => {
                    eprintln!("NVML failed, trying fallback: {}", e);
                }
            }
        }

        // Fallback to nvidia-smi
        if self
            .fallback
            .available_methods()
            .contains(&"nvidia-settings")
        {
            match self.get_info_nvidia_smi() {
                Ok(info) => return Ok(info),
                Err(e) => {
                    eprintln!("nvidia-smi failed: {}", e);
                }
            }
        }

        Err(NvControlError::GpuQueryFailed(format!(
            "All methods failed. Available: {:?}",
            self.fallback.available_methods()
        )))
    }

    /// Get GPU info via NVML
    fn get_info_nvml(&self) -> NvResult<GpuInfoEnhanced> {
        let ctx = ErrorContext::new("NVML GPU query").with_gpu(self.gpu_id);

        let nvml = handle_nvml_error(Nvml::init(), ErrorContext::new("initialize NVML"))?;

        let device = handle_nvml_error(
            nvml.device_by_index(self.gpu_id),
            ctx.clone()
                .with_suggestion("Check GPU index with: nvidia-smi"),
        )?;

        let name = handle_nvml_error(device.name(), ctx.clone())?;
        let driver = handle_nvml_error(nvml.sys_driver_version(), ctx.clone())?;
        let cuda = nvml
            .sys_cuda_driver_version()
            .ok()
            .map(|v| format!("{}", v));

        let mem = handle_nvml_error(device.memory_info(), ctx.clone())?;
        let temp = handle_nvml_error(
            device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu),
            ctx.clone(),
        )?;
        let power_usage = handle_nvml_error(device.power_usage(), ctx.clone())? as f32 / 1000.0;
        let power_limit =
            handle_nvml_error(device.power_management_limit(), ctx.clone())? as f32 / 1000.0;

        let fan = device.fan_speed(0).unwrap_or(0);
        let util = device
            .utilization_rates()
            .map(|u| (u.gpu, u.memory))
            .unwrap_or((0, 0));

        let clock_gpu = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .unwrap_or(0);
        let clock_memory = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
            .unwrap_or(0);

        let pci = device.pci_info().ok().map(|p| {
            format!(
                "{:04x}:{:02x}:{:02x}.{:x}",
                p.domain, p.bus, p.device, p.pci_device_id
            )
        });

        let compute_cap = device
            .cuda_compute_capability()
            .ok()
            .map(|c| (c.major as u32, c.minor as u32));
        let arch = Self::detect_architecture(&name, compute_cap);

        Ok(GpuInfoEnhanced {
            id: self.gpu_id,
            name,
            pci_address: pci.unwrap_or_else(|| format!("Unknown")),
            driver_version: driver,
            cuda_version: cuda,
            memory_total_mb: mem.total / (1024 * 1024),
            memory_used_mb: mem.used / (1024 * 1024),
            temperature: temp as i32,
            power_draw_watts: power_usage,
            power_limit_watts: power_limit,
            fan_speed_percent: fan,
            gpu_utilization: util.0,
            memory_utilization: util.1,
            clock_gpu_mhz: clock_gpu,
            clock_memory_mhz: clock_memory,
            architecture: arch,
            compute_capability: compute_cap,
        })
    }

    /// Fallback: Get GPU info via nvidia-smi
    fn get_info_nvidia_smi(&self) -> NvResult<GpuInfoEnhanced> {
        use std::process::Command;

        let output = Command::new("nvidia-smi")
            .args(&[
                "--query-gpu=name,memory.total,memory.used,temperature.gpu,power.draw,fan.speed,utilization.gpu",
                "--format=csv,noheader,nounits",
                "--id",
                &self.gpu_id.to_string(),
            ])
            .output()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("nvidia-smi execution failed: {}", e))
            })?;

        if !output.status.success() {
            return Err(NvControlError::CommandFailed(
                "nvidia-smi returned error".to_string(),
            ));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.trim().split(',').map(|s| s.trim()).collect();

        if parts.len() < 7 {
            return Err(NvControlError::GpuQueryFailed(
                "Unexpected nvidia-smi output format".to_string(),
            ));
        }

        Ok(GpuInfoEnhanced {
            id: self.gpu_id,
            name: parts[0].to_string(),
            pci_address: "Unknown".to_string(),
            driver_version: "Unknown".to_string(),
            cuda_version: None,
            memory_total_mb: parts[1].parse().unwrap_or(0),
            memory_used_mb: parts[2].parse().unwrap_or(0),
            temperature: parts[3].parse().unwrap_or(0),
            power_draw_watts: parts[4].parse().unwrap_or(0.0),
            power_limit_watts: 0.0,
            fan_speed_percent: parts[5].parse().unwrap_or(0),
            gpu_utilization: parts[6].parse().unwrap_or(0),
            memory_utilization: 0,
            clock_gpu_mhz: 0,
            clock_memory_mhz: 0,
            architecture: None,
            compute_capability: None,
        })
    }

    /// Detect GPU architecture from name and compute capability
    pub fn detect_architecture(name: &str, compute_cap: Option<(u32, u32)>) -> Option<String> {
        if let Some((major, minor)) = compute_cap {
            let arch = match (major, minor) {
                (10, 0) => "Blackwell",
                (8, 9) => "Ada Lovelace",
                (8, 6) | (8, 0) => "Ampere",
                (7, 5) => "Turing",
                (7, 0) => "Volta",
                (6, _) => "Pascal",
                (5, _) => "Maxwell",
                _ => "Unknown",
            };
            return Some(arch.to_string());
        }

        // Fallback: detect from name
        if name.contains("50") {
            Some("Blackwell".to_string())
        } else if name.contains("40") {
            Some("Ada Lovelace".to_string())
        } else if name.contains("30") {
            Some("Ampere".to_string())
        } else if name.contains("20") || name.contains("16") {
            Some("Turing".to_string())
        } else if name.contains("10") {
            Some("Pascal".to_string())
        } else {
            None
        }
    }

    /// Apply overclock with safety validation
    pub fn apply_overclock_safe(&self, gpu_offset: i32, memory_offset: i32) -> NvResult<()> {
        // Validate safety first
        let validation = self
            .safety_monitor
            .validate_overclock_safe(gpu_offset, memory_offset)?;

        match validation {
            crate::hardware_safety::OverclockValidation::Unsafe { reason } => {
                return Err(NvControlError::UnsupportedFeature(format!(
                    "Overclock rejected: {}",
                    reason
                )));
            }
            crate::hardware_safety::OverclockValidation::Warning { reason } => {
                eprintln!("⚠️  Warning: {}", reason);
                eprintln!("Proceeding with overclock anyway...");
            }
            crate::hardware_safety::OverclockValidation::Safe => {
                println!("✓ Overclock settings validated as safe");
            }
        }

        // Apply overclock with retry
        self.retry_handler
            .retry(|| self.apply_overclock_impl(gpu_offset, memory_offset))
    }

    fn apply_overclock_impl(&self, gpu_offset: i32, memory_offset: i32) -> NvResult<()> {
        // Try nvidia-settings (X11)
        if std::env::var("DISPLAY").is_ok() {
            let output = std::process::Command::new("nvidia-settings")
                .args(&[
                    "-a",
                    &format!(
                        "[gpu:{}]/GPUGraphicsClockOffsetAllPerformanceLevels={}",
                        self.gpu_id, gpu_offset
                    ),
                    "-a",
                    &format!(
                        "[gpu:{}]/GPUMemoryTransferRateOffsetAllPerformanceLevels={}",
                        self.gpu_id, memory_offset
                    ),
                ])
                .output()
                .map_err(|e| {
                    NvControlError::CommandFailed(format!("nvidia-settings failed: {}", e))
                })?;

            if output.status.success() {
                println!(
                    "✓ Applied overclock: GPU +{} MHz, Memory +{} MHz",
                    gpu_offset, memory_offset
                );
                return Ok(());
            }
        }

        // Wayland fallback (requires root)
        Err(NvControlError::UnsupportedFeature(
            "Overclocking on Wayland requires additional setup".to_string(),
        ))
    }

    /// Set power limit with safety validation
    pub fn set_power_limit_safe(&self, watts: u32) -> NvResult<()> {
        // Get current limit to calculate percentage
        let info = self.get_info()?;
        let percent = ((watts as f32 / info.power_limit_watts) * 100.0) as u8;

        // Validate safety
        let validation = self.safety_monitor.validate_power_limit_safe(percent)?;

        match validation {
            crate::hardware_safety::PowerValidation::Unsafe { reason } => {
                return Err(NvControlError::PowerManagementFailed(format!(
                    "Power limit rejected: {}",
                    reason
                )));
            }
            crate::hardware_safety::PowerValidation::Warning { reason } => {
                eprintln!("⚠️  Warning: {}", reason);
            }
            crate::hardware_safety::PowerValidation::Safe => {
                println!("✓ Power limit validated as safe");
            }
        }

        // Apply with NVML
        let ctx = ErrorContext::new("set power limit")
            .with_gpu(self.gpu_id)
            .with_suggestion("Try running with sudo for power management");

        let nvml = handle_nvml_error(Nvml::init(), ErrorContext::new("initialize NVML"))?;
        let mut device = handle_nvml_error(nvml.device_by_index(self.gpu_id), ctx.clone())?;

        handle_nvml_error(
            device.set_power_management_limit(watts * 1000), // Convert to milliwatts
            ctx,
        )?;

        println!("✓ Power limit set to {} W", watts);
        Ok(())
    }

    /// Check safety status
    pub fn check_safety(&self) -> NvResult<()> {
        let status = self.safety_monitor.check_temperature()?;

        match status {
            crate::hardware_safety::SafetyStatus::Normal { temperature } => {
                println!("✓ GPU temperature normal: {}°C", temperature);
            }
            crate::hardware_safety::SafetyStatus::ThermalThrottling { temperature } => {
                eprintln!("⚠️  GPU thermal throttling active: {}°C", temperature);
            }
            crate::hardware_safety::SafetyStatus::EmergencyShutdown { temperature } => {
                eprintln!("🚨 GPU emergency shutdown triggered: {}°C", temperature);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_detection() {
        assert_eq!(
            SafeGpuController::detect_architecture("RTX 5090", Some((10, 0))),
            Some("Blackwell".to_string())
        );
        assert_eq!(
            SafeGpuController::detect_architecture("RTX 4090", Some((8, 9))),
            Some("Ada Lovelace".to_string())
        );
        assert_eq!(
            SafeGpuController::detect_architecture("RTX 3090", Some((8, 6))),
            Some("Ampere".to_string())
        );
    }

    #[test]
    fn test_fallback_detection() {
        let controller = SafeGpuController::new(0);
        assert!(!controller.fallback.available_methods().is_empty());
    }
}
