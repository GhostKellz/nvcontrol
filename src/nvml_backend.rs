//! NVML Backend Abstraction
//!
//! Provides a trait-based abstraction over NVML for testability.
//! Real implementation wraps nvml-wrapper, mock implementation returns configurable data.

use crate::{NvControlError, NvResult};
use std::sync::Arc;

/// GPU device information snapshot
#[derive(Debug, Clone, Default)]
pub struct GpuDeviceInfo {
    pub index: u32,
    pub name: String,
    pub uuid: String,
    pub pci_bus_id: String,
}

/// GPU process information
#[derive(Debug, Clone, Default)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// GPU memory used by this process in bytes, None if unavailable
    pub used_gpu_memory_bytes: Option<u64>,
}

/// GPU metrics snapshot
#[derive(Debug, Clone, Default)]
pub struct GpuMetrics {
    pub gpu_utilization: u32,
    pub memory_utilization: u32,
    pub temperature: u32,
    pub power_draw_mw: u32,
    pub fan_speed: u32,
    pub gpu_clock_mhz: u32,
    pub memory_clock_mhz: u32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
}

/// NVML backend trait for GPU operations
pub trait NvmlBackend: Send + Sync {
    /// Get the number of NVIDIA GPUs in the system
    fn device_count(&self) -> NvResult<u32>;

    /// Get device info by index
    fn get_device_info(&self, index: u32) -> NvResult<GpuDeviceInfo>;

    /// Get current metrics for a device
    fn get_metrics(&self, index: u32) -> NvResult<GpuMetrics>;

    /// Get temperature for a device
    fn get_temperature(&self, index: u32) -> NvResult<u32>;

    /// Get utilization rates (gpu%, memory%)
    fn get_utilization(&self, index: u32) -> NvResult<(u32, u32)>;

    /// Get power usage in milliwatts
    fn get_power_usage(&self, index: u32) -> NvResult<u32>;

    /// Get fan speed percentage for a specific fan
    fn get_fan_speed(&self, index: u32, fan_index: u32) -> NvResult<u32>;

    /// Get memory info (used, total) in bytes
    fn get_memory_info(&self, index: u32) -> NvResult<(u64, u64)>;

    /// Get GPU clock in MHz
    fn get_gpu_clock(&self, index: u32) -> NvResult<u32>;

    /// Get memory clock in MHz
    fn get_memory_clock(&self, index: u32) -> NvResult<u32>;

    /// Get device name
    fn get_name(&self, index: u32) -> NvResult<String>;

    /// Check if NVML is available
    fn is_available(&self) -> bool;

    // =========================================================================
    // Extended methods for multi-GPU, fan, and power module support
    // =========================================================================

    /// Get driver version string
    fn get_driver_version(&self) -> NvResult<String>;

    /// Get power management limit in milliwatts
    fn get_power_limit(&self, index: u32) -> NvResult<u32>;

    /// Get power management limit constraints (min, max) in milliwatts
    fn get_power_limit_constraints(&self, index: u32) -> NvResult<(u32, u32)>;

    /// Get CUDA core count
    fn get_cuda_cores(&self, index: u32) -> NvResult<u32>;

    /// Get compute capability (major, minor)
    fn get_compute_capability(&self, index: u32) -> NvResult<(u32, u32)>;

    /// Get device UUID
    fn get_uuid(&self, index: u32) -> NvResult<String>;

    /// Get PCI bus ID string
    fn get_pci_bus_id(&self, index: u32) -> NvResult<String>;

    /// Get number of fans on device
    fn get_fan_count(&self, index: u32) -> NvResult<u32>;

    /// Check if fan control is supported
    fn is_fan_control_supported(&self, index: u32) -> bool;

    // =========================================================================
    // Power management methods (for advanced_power.rs)
    // =========================================================================

    /// Get default power management limit in milliwatts
    fn get_power_limit_default(&self, index: u32) -> NvResult<u32>;

    /// Set power management limit in milliwatts (requires root/admin)
    fn set_power_limit(&self, index: u32, limit_mw: u32) -> NvResult<()>;

    // =========================================================================
    // Process listing methods (for TUI process tab)
    // =========================================================================

    /// Get list of running graphics processes on a device
    fn get_running_graphics_processes(&self, index: u32) -> NvResult<Vec<ProcessInfo>>;

    /// Get list of running compute processes on a device
    fn get_running_compute_processes(&self, index: u32) -> NvResult<Vec<ProcessInfo>>;

    // =========================================================================
    // Clock info methods (for TUI OC tab)
    // =========================================================================

    /// Get max GPU clock in MHz
    fn get_max_gpu_clock(&self, index: u32) -> NvResult<u32>;

    /// Get max memory clock in MHz
    fn get_max_memory_clock(&self, index: u32) -> NvResult<u32>;
}

/// Real NVML backend using nvml-wrapper
pub struct RealNvmlBackend {
    nvml: Option<nvml_wrapper::Nvml>,
}

impl RealNvmlBackend {
    pub fn new() -> Self {
        let nvml = nvml_wrapper::Nvml::init().ok();
        Self { nvml }
    }

    fn get_device(&self, index: u32) -> NvResult<nvml_wrapper::Device<'_>> {
        let nvml = self
            .nvml
            .as_ref()
            .ok_or_else(|| NvControlError::NvmlNotAvailable("NVML not initialized".to_string()))?;

        nvml.device_by_index(index).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device {}: {}", index, e))
        })
    }
}

impl Default for RealNvmlBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl NvmlBackend for RealNvmlBackend {
    fn device_count(&self) -> NvResult<u32> {
        let nvml = self
            .nvml
            .as_ref()
            .ok_or_else(|| NvControlError::NvmlNotAvailable("NVML not initialized".to_string()))?;

        nvml.device_count().map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device count: {}", e))
        })
    }

    fn get_device_info(&self, index: u32) -> NvResult<GpuDeviceInfo> {
        let device = self.get_device(index)?;

        Ok(GpuDeviceInfo {
            index,
            name: device.name().unwrap_or_else(|_| "Unknown".to_string()),
            uuid: device.uuid().unwrap_or_else(|_| "Unknown".to_string()),
            pci_bus_id: device
                .pci_info()
                .map(|p| p.bus_id)
                .unwrap_or_else(|_| "Unknown".to_string()),
        })
    }

    fn get_metrics(&self, index: u32) -> NvResult<GpuMetrics> {
        let device = self.get_device(index)?;

        let (gpu_util, mem_util) = device
            .utilization_rates()
            .map(|u| (u.gpu, u.memory))
            .unwrap_or((0, 0));

        let temperature = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0);

        let power_draw_mw = device.power_usage().unwrap_or(0);

        let fan_speed = device.fan_speed(0).unwrap_or(0);

        let gpu_clock = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .unwrap_or(0);

        let memory_clock = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
            .unwrap_or(0);

        let (memory_used, memory_total) = device
            .memory_info()
            .map(|m| (m.used, m.total))
            .unwrap_or((0, 0));

        Ok(GpuMetrics {
            gpu_utilization: gpu_util,
            memory_utilization: mem_util,
            temperature,
            power_draw_mw,
            fan_speed,
            gpu_clock_mhz: gpu_clock,
            memory_clock_mhz: memory_clock,
            memory_used_bytes: memory_used,
            memory_total_bytes: memory_total,
        })
    }

    fn get_temperature(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get temperature: {}", e))
            })
    }

    fn get_utilization(&self, index: u32) -> NvResult<(u32, u32)> {
        let device = self.get_device(index)?;
        device
            .utilization_rates()
            .map(|u| (u.gpu, u.memory))
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get utilization: {}", e))
            })
    }

    fn get_power_usage(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device.power_usage().map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get power usage: {}", e))
        })
    }

    fn get_fan_speed(&self, index: u32, fan_index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device
            .fan_speed(fan_index)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get fan speed: {}", e)))
    }

    fn get_memory_info(&self, index: u32) -> NvResult<(u64, u64)> {
        let device = self.get_device(index)?;
        device
            .memory_info()
            .map(|m| (m.used, m.total))
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get memory info: {}", e))
            })
    }

    fn get_gpu_clock(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get GPU clock: {}", e)))
    }

    fn get_memory_clock(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get memory clock: {}", e))
            })
    }

    fn get_name(&self, index: u32) -> NvResult<String> {
        let device = self.get_device(index)?;
        device
            .name()
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get name: {}", e)))
    }

    fn is_available(&self) -> bool {
        self.nvml.is_some()
    }

    fn get_driver_version(&self) -> NvResult<String> {
        let nvml = self
            .nvml
            .as_ref()
            .ok_or_else(|| NvControlError::NvmlNotAvailable("NVML not initialized".to_string()))?;
        nvml.sys_driver_version().map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get driver version: {}", e))
        })
    }

    fn get_power_limit(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device.power_management_limit().map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get power limit: {}", e))
        })
    }

    fn get_power_limit_constraints(&self, index: u32) -> NvResult<(u32, u32)> {
        let device = self.get_device(index)?;
        device
            .power_management_limit_constraints()
            .map(|c| (c.min_limit, c.max_limit))
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get power constraints: {}", e))
            })
    }

    fn get_cuda_cores(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device
            .num_cores()
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get CUDA cores: {}", e)))
    }

    fn get_compute_capability(&self, index: u32) -> NvResult<(u32, u32)> {
        let device = self.get_device(index)?;
        device
            .cuda_compute_capability()
            .map(|cc| (cc.major as u32, cc.minor as u32))
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get compute capability: {}", e))
            })
    }

    fn get_uuid(&self, index: u32) -> NvResult<String> {
        let device = self.get_device(index)?;
        device
            .uuid()
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get UUID: {}", e)))
    }

    fn get_pci_bus_id(&self, index: u32) -> NvResult<String> {
        let device = self.get_device(index)?;
        device
            .pci_info()
            .map(|p| format!("{:04x}:{:02x}:{:02x}.0", p.domain, p.bus, p.device))
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get PCI bus ID: {}", e)))
    }

    fn get_fan_count(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device
            .num_fans()
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get fan count: {}", e)))
    }

    fn is_fan_control_supported(&self, index: u32) -> bool {
        if let Ok(device) = self.get_device(index) {
            // Try to get fan speed - if it works, fan control is likely supported
            device.fan_speed(0).is_ok()
        } else {
            false
        }
    }

    fn get_power_limit_default(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device.power_management_limit_default().map_err(|e| {
            NvControlError::PowerManagementFailed(format!(
                "Failed to get default power limit: {}",
                e
            ))
        })
    }

    fn set_power_limit(&self, index: u32, limit_mw: u32) -> NvResult<()> {
        let nvml = self
            .nvml
            .as_ref()
            .ok_or_else(|| NvControlError::NvmlNotAvailable("NVML not initialized".to_string()))?;

        let mut device = nvml.device_by_index(index).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device {}: {}", index, e))
        })?;

        device.set_power_management_limit(limit_mw).map_err(|e| {
            NvControlError::PowerManagementFailed(format!("Failed to set power limit: {}", e))
        })
    }

    fn get_running_graphics_processes(&self, index: u32) -> NvResult<Vec<ProcessInfo>> {
        let device = self.get_device(index)?;
        device
            .running_graphics_processes()
            .map(|procs| {
                procs
                    .into_iter()
                    .map(|p| ProcessInfo {
                        pid: p.pid,
                        used_gpu_memory_bytes: match p.used_gpu_memory {
                            nvml_wrapper::enums::device::UsedGpuMemory::Used(bytes) => Some(bytes),
                            nvml_wrapper::enums::device::UsedGpuMemory::Unavailable => None,
                        },
                    })
                    .collect()
            })
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get graphics processes: {}", e))
            })
    }

    fn get_running_compute_processes(&self, index: u32) -> NvResult<Vec<ProcessInfo>> {
        let device = self.get_device(index)?;
        device
            .running_compute_processes()
            .map(|procs| {
                procs
                    .into_iter()
                    .map(|p| ProcessInfo {
                        pid: p.pid,
                        used_gpu_memory_bytes: match p.used_gpu_memory {
                            nvml_wrapper::enums::device::UsedGpuMemory::Used(bytes) => Some(bytes),
                            nvml_wrapper::enums::device::UsedGpuMemory::Unavailable => None,
                        },
                    })
                    .collect()
            })
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get compute processes: {}", e))
            })
    }

    fn get_max_gpu_clock(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device
            .max_clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get max GPU clock: {}", e))
            })
    }

    fn get_max_memory_clock(&self, index: u32) -> NvResult<u32> {
        let device = self.get_device(index)?;
        device
            .max_clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
            .map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get max memory clock: {}", e))
            })
    }
}

/// Mock NVML backend for testing
#[derive(Debug, Clone)]
pub struct MockNvmlBackend {
    pub device_count: u32,
    pub devices: Vec<MockGpuDevice>,
    pub driver_version: String,
}

#[derive(Debug, Clone)]
pub struct MockGpuDevice {
    pub info: GpuDeviceInfo,
    pub metrics: GpuMetrics,
    pub power_limit_mw: u32,
    pub power_limit_default_mw: u32,
    pub power_limit_min_mw: u32,
    pub power_limit_max_mw: u32,
    pub cuda_cores: u32,
    pub compute_major: u32,
    pub compute_minor: u32,
    pub fan_count: u32,
    pub fan_control_supported: bool,
    pub graphics_processes: Vec<ProcessInfo>,
    pub compute_processes: Vec<ProcessInfo>,
    pub max_gpu_clock_mhz: u32,
    pub max_memory_clock_mhz: u32,
}

impl MockNvmlBackend {
    /// Create a mock with a single default GPU
    pub fn single_gpu() -> Self {
        Self {
            device_count: 1,
            devices: vec![MockGpuDevice::default()],
            driver_version: "565.57.01".to_string(),
        }
    }

    /// Create a mock with multiple GPUs
    pub fn multi_gpu(count: u32) -> Self {
        let devices = (0..count)
            .map(|i| MockGpuDevice {
                info: GpuDeviceInfo {
                    index: i,
                    name: format!("Mock GPU {}", i),
                    uuid: format!("GPU-MOCK-{:04}", i),
                    pci_bus_id: format!("0000:{:02x}:00.0", i),
                },
                metrics: GpuMetrics {
                    gpu_utilization: 50 + i * 10,
                    memory_utilization: 40 + i * 5,
                    temperature: 55 + i * 3,
                    power_draw_mw: 150_000 + i * 20_000,
                    fan_speed: 45 + i * 5,
                    gpu_clock_mhz: 1800 + i * 100,
                    memory_clock_mhz: 7000 + i * 500,
                    memory_used_bytes: 4_000_000_000 + (i as u64) * 1_000_000_000,
                    memory_total_bytes: 8_000_000_000,
                },
                power_limit_mw: 320_000,
                power_limit_default_mw: 320_000,
                power_limit_min_mw: 200_000,
                power_limit_max_mw: 450_000,
                cuda_cores: 9728,
                compute_major: 8,
                compute_minor: 9,
                fan_count: 2,
                fan_control_supported: true,
                graphics_processes: vec![
                    ProcessInfo {
                        pid: 1000 + i * 100,
                        used_gpu_memory_bytes: Some(256 * 1024 * 1024),
                    },
                    ProcessInfo {
                        pid: 1001 + i * 100,
                        used_gpu_memory_bytes: Some(512 * 1024 * 1024),
                    },
                ],
                compute_processes: vec![ProcessInfo {
                    pid: 2000 + i * 100,
                    used_gpu_memory_bytes: Some(1024 * 1024 * 1024),
                }],
                max_gpu_clock_mhz: 2520,
                max_memory_clock_mhz: 10501,
            })
            .collect();

        Self {
            device_count: count,
            devices,
            driver_version: "565.57.01".to_string(),
        }
    }

    /// Create an empty mock (no GPUs)
    pub fn no_gpu() -> Self {
        Self {
            device_count: 0,
            devices: vec![],
            driver_version: "Unknown".to_string(),
        }
    }

    fn get_device(&self, index: u32) -> NvResult<&MockGpuDevice> {
        self.devices
            .get(index as usize)
            .ok_or_else(|| NvControlError::GpuQueryFailed(format!("No device at index {}", index)))
    }
}

impl Default for MockGpuDevice {
    fn default() -> Self {
        Self {
            info: GpuDeviceInfo {
                index: 0,
                name: "Mock NVIDIA GeForce RTX 4080".to_string(),
                uuid: "GPU-MOCK-0000".to_string(),
                pci_bus_id: "0000:01:00.0".to_string(),
            },
            metrics: GpuMetrics {
                gpu_utilization: 45,
                memory_utilization: 35,
                temperature: 58,
                power_draw_mw: 180_000,
                fan_speed: 40,
                gpu_clock_mhz: 2100,
                memory_clock_mhz: 11200,
                memory_used_bytes: 6_000_000_000,
                memory_total_bytes: 16_000_000_000,
            },
            power_limit_mw: 320_000,
            power_limit_default_mw: 320_000,
            power_limit_min_mw: 200_000,
            power_limit_max_mw: 450_000,
            cuda_cores: 9728,
            compute_major: 8,
            compute_minor: 9,
            fan_count: 2,
            fan_control_supported: true,
            graphics_processes: vec![
                ProcessInfo {
                    pid: 1234,
                    used_gpu_memory_bytes: Some(512 * 1024 * 1024),
                },
                ProcessInfo {
                    pid: 5678,
                    used_gpu_memory_bytes: Some(1024 * 1024 * 1024),
                },
            ],
            compute_processes: vec![ProcessInfo {
                pid: 9999,
                used_gpu_memory_bytes: Some(2048 * 1024 * 1024),
            }],
            max_gpu_clock_mhz: 2520,
            max_memory_clock_mhz: 11200,
        }
    }
}

impl NvmlBackend for MockNvmlBackend {
    fn device_count(&self) -> NvResult<u32> {
        Ok(self.device_count)
    }

    fn get_device_info(&self, index: u32) -> NvResult<GpuDeviceInfo> {
        Ok(self.get_device(index)?.info.clone())
    }

    fn get_metrics(&self, index: u32) -> NvResult<GpuMetrics> {
        Ok(self.get_device(index)?.metrics.clone())
    }

    fn get_temperature(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.metrics.temperature)
    }

    fn get_utilization(&self, index: u32) -> NvResult<(u32, u32)> {
        let m = &self.get_device(index)?.metrics;
        Ok((m.gpu_utilization, m.memory_utilization))
    }

    fn get_power_usage(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.metrics.power_draw_mw)
    }

    fn get_fan_speed(&self, index: u32, _fan_index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.metrics.fan_speed)
    }

    fn get_memory_info(&self, index: u32) -> NvResult<(u64, u64)> {
        let m = &self.get_device(index)?.metrics;
        Ok((m.memory_used_bytes, m.memory_total_bytes))
    }

    fn get_gpu_clock(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.metrics.gpu_clock_mhz)
    }

    fn get_memory_clock(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.metrics.memory_clock_mhz)
    }

    fn get_name(&self, index: u32) -> NvResult<String> {
        Ok(self.get_device(index)?.info.name.clone())
    }

    fn is_available(&self) -> bool {
        self.device_count > 0
    }

    fn get_driver_version(&self) -> NvResult<String> {
        Ok(self.driver_version.clone())
    }

    fn get_power_limit(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.power_limit_mw)
    }

    fn get_power_limit_constraints(&self, index: u32) -> NvResult<(u32, u32)> {
        let d = self.get_device(index)?;
        Ok((d.power_limit_min_mw, d.power_limit_max_mw))
    }

    fn get_cuda_cores(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.cuda_cores)
    }

    fn get_compute_capability(&self, index: u32) -> NvResult<(u32, u32)> {
        let d = self.get_device(index)?;
        Ok((d.compute_major, d.compute_minor))
    }

    fn get_uuid(&self, index: u32) -> NvResult<String> {
        Ok(self.get_device(index)?.info.uuid.clone())
    }

    fn get_pci_bus_id(&self, index: u32) -> NvResult<String> {
        Ok(self.get_device(index)?.info.pci_bus_id.clone())
    }

    fn get_fan_count(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.fan_count)
    }

    fn is_fan_control_supported(&self, index: u32) -> bool {
        self.get_device(index)
            .map(|d| d.fan_control_supported)
            .unwrap_or(false)
    }

    fn get_power_limit_default(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.power_limit_default_mw)
    }

    fn set_power_limit(&self, index: u32, _limit_mw: u32) -> NvResult<()> {
        // Mock just validates the device exists
        self.get_device(index)?;
        Ok(())
    }

    fn get_running_graphics_processes(&self, index: u32) -> NvResult<Vec<ProcessInfo>> {
        Ok(self.get_device(index)?.graphics_processes.clone())
    }

    fn get_running_compute_processes(&self, index: u32) -> NvResult<Vec<ProcessInfo>> {
        Ok(self.get_device(index)?.compute_processes.clone())
    }

    fn get_max_gpu_clock(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.max_gpu_clock_mhz)
    }

    fn get_max_memory_clock(&self, index: u32) -> NvResult<u32> {
        Ok(self.get_device(index)?.max_memory_clock_mhz)
    }
}

/// Shared backend type for use across modules
pub type SharedNvmlBackend = Arc<dyn NvmlBackend>;

/// Create a shared real backend
pub fn create_real_backend() -> SharedNvmlBackend {
    Arc::new(RealNvmlBackend::new())
}

/// Create a shared mock backend for testing
pub fn create_mock_backend() -> SharedNvmlBackend {
    Arc::new(MockNvmlBackend::single_gpu())
}

// =============================================================================
// GUI/TUI Backend Context
// =============================================================================

use crate::display_backend::SharedDisplayRunner;
use std::sync::RwLock;
use std::time::Instant;

/// Backend availability status for unified UI messaging
#[derive(Debug, Clone, PartialEq)]
pub enum BackendStatus {
    /// All backends available and working
    Available,
    /// NVML unavailable - likely no NVIDIA driver or GPU
    NvmlUnavailable(String),
    /// Display runner unavailable - headless or no display server
    DisplayUnavailable(String),
    /// Both NVML and display runner unavailable
    AllUnavailable {
        nvml_reason: String,
        display_reason: String,
    },
}

impl BackendStatus {
    /// Check if NVML is available
    pub fn is_nvml_available(&self) -> bool {
        matches!(
            self,
            BackendStatus::Available | BackendStatus::DisplayUnavailable(_)
        )
    }

    /// Check if display runner is available
    pub fn is_display_available(&self) -> bool {
        matches!(
            self,
            BackendStatus::Available | BackendStatus::NvmlUnavailable(_)
        )
    }

    /// Get human-readable status message
    pub fn status_message(&self) -> &str {
        match self {
            BackendStatus::Available => "All backends available",
            BackendStatus::NvmlUnavailable(_) => "NVIDIA driver/GPU unavailable",
            BackendStatus::DisplayUnavailable(_) => "Display server unavailable",
            BackendStatus::AllUnavailable { .. } => "All backends unavailable",
        }
    }
}

/// Cached metrics snapshot with timestamp for staleness detection
#[derive(Clone)]
pub struct CachedMetrics {
    pub metrics: GpuMetrics,
    pub timestamp: Instant,
}

impl CachedMetrics {
    pub fn new(metrics: GpuMetrics) -> Self {
        Self {
            metrics,
            timestamp: Instant::now(),
        }
    }

    /// Check if metrics are stale (older than specified duration)
    pub fn is_stale(&self, max_age_secs: u64) -> bool {
        self.timestamp.elapsed().as_secs() > max_age_secs
    }

    /// Get age of cached metrics in seconds
    pub fn age_secs(&self) -> u64 {
        self.timestamp.elapsed().as_secs()
    }
}

/// Minimum time (in seconds) a status must persist before it's reported as changed.
/// Prevents UI flicker during rapid hotplug events (eGPU attach/detach, USB-C dock).
const STATUS_DEBOUNCE_SECS: u64 = 2;

/// Tracks backend status transitions with debouncing to prevent flicker
#[derive(Clone)]
struct StatusTracker {
    /// Currently reported status (after debounce)
    reported: BackendStatus,
    /// Pending status (may differ from reported during transition)
    pending: BackendStatus,
    /// When the pending status was first observed
    pending_since: Instant,
}

impl StatusTracker {
    fn new(initial: BackendStatus) -> Self {
        Self {
            reported: initial.clone(),
            pending: initial,
            pending_since: Instant::now(),
        }
    }

    /// Update with a new observed status, returning the debounced status to display.
    fn update(&mut self, observed: BackendStatus) -> BackendStatus {
        if observed == self.pending {
            // Same as pending - check if debounce period has passed
            if self.pending_since.elapsed().as_secs() >= STATUS_DEBOUNCE_SECS {
                // Debounce complete, update reported status
                self.reported = self.pending.clone();
            }
        } else {
            // Status changed - start new debounce period
            self.pending = observed;
            self.pending_since = Instant::now();
        }
        self.reported.clone()
    }

    /// Get the currently reported (debounced) status
    #[cfg(test)]
    fn get_reported(&self) -> &BackendStatus {
        &self.reported
    }

    /// Check if a status transition is pending (in debounce period)
    fn is_transitioning(&self) -> bool {
        self.reported != self.pending
    }
}

/// Unified backend context for GUI/TUI applications
///
/// Bundles all backend handles needed by UI layers so they don't construct
/// their own NVML/display sessions. Pass this into TuiApp, GUI state, tray,
/// and notification handlers.
///
/// ## Hotplug Debouncing
///
/// Backend status changes (NVML/display availability) are debounced to prevent
/// UI flicker during rapid attach/detach cycles. Status must persist for
/// `STATUS_DEBOUNCE_SECS` before being reported to the UI.
#[derive(Clone)]
pub struct GuiBackendContext {
    /// Shared NVML backend for GPU queries
    pub nvml: SharedNvmlBackend,
    /// Shared display command runner for HDR/VRR/vibrance
    pub display: SharedDisplayRunner,
    /// Cached device count (avoids repeated queries)
    pub device_count: u32,
    /// Cached driver version
    pub driver_version: String,
    /// Backend availability status (debounced)
    pub status: BackendStatus,
    /// Cached metrics per GPU (last good reading with timestamp)
    metrics_cache: Arc<RwLock<Vec<Option<CachedMetrics>>>>,
    /// Status tracker for debouncing hotplug events
    status_tracker: Arc<RwLock<StatusTracker>>,
}

impl GuiBackendContext {
    /// Create a new context with real backends
    pub fn new() -> Self {
        let nvml = create_real_backend();
        let display = crate::display_backend::create_real_runner();

        let device_count = nvml.device_count().unwrap_or(0);
        let driver_version = nvml.get_driver_version().unwrap_or_default();

        // Determine backend status
        let nvml_available = nvml.is_available();
        let display_available = display.is_available();
        let status = match (nvml_available, display_available) {
            (true, true) => BackendStatus::Available,
            (false, true) => BackendStatus::NvmlUnavailable("NVML not initialized".to_string()),
            (true, false) => BackendStatus::DisplayUnavailable("No display server".to_string()),
            (false, false) => BackendStatus::AllUnavailable {
                nvml_reason: "NVML not initialized".to_string(),
                display_reason: "No display server".to_string(),
            },
        };

        // Initialize empty metrics cache for each device
        let metrics_cache = Arc::new(RwLock::new(vec![None; device_count as usize]));
        let status_tracker = Arc::new(RwLock::new(StatusTracker::new(status.clone())));

        Self {
            nvml,
            display,
            device_count,
            driver_version,
            status,
            metrics_cache,
            status_tracker,
        }
    }

    /// Create a context with mock backends for testing
    pub fn mock() -> Self {
        let nvml = create_mock_backend();
        let display = crate::display_backend::create_mock_runner_x11();

        let device_count = nvml.device_count().unwrap_or(0);
        let driver_version = nvml.get_driver_version().unwrap_or_default();

        let metrics_cache = Arc::new(RwLock::new(vec![None; device_count as usize]));
        let status = BackendStatus::Available;
        let status_tracker = Arc::new(RwLock::new(StatusTracker::new(status.clone())));

        Self {
            nvml,
            display,
            device_count,
            driver_version,
            status,
            metrics_cache,
            status_tracker,
        }
    }

    /// Create a context with custom backends
    pub fn with_backends(nvml: SharedNvmlBackend, display: SharedDisplayRunner) -> Self {
        let device_count = nvml.device_count().unwrap_or(0);
        let driver_version = nvml.get_driver_version().unwrap_or_default();

        let nvml_available = nvml.is_available();
        let display_available = display.is_available();
        let status = match (nvml_available, display_available) {
            (true, true) => BackendStatus::Available,
            (false, true) => BackendStatus::NvmlUnavailable("NVML not initialized".to_string()),
            (true, false) => BackendStatus::DisplayUnavailable("No display server".to_string()),
            (false, false) => BackendStatus::AllUnavailable {
                nvml_reason: "NVML not initialized".to_string(),
                display_reason: "No display server".to_string(),
            },
        };

        let metrics_cache = Arc::new(RwLock::new(vec![None; device_count as usize]));
        let status_tracker = Arc::new(RwLock::new(StatusTracker::new(status.clone())));

        Self {
            nvml,
            display,
            device_count,
            driver_version,
            status,
            metrics_cache,
            status_tracker,
        }
    }

    /// Check if NVML is available
    pub fn is_nvml_available(&self) -> bool {
        self.nvml.is_available()
    }

    /// Get metrics for a GPU, caching successful reads
    ///
    /// If the query fails, returns the last cached value if available.
    /// Call `get_cached_metrics_age()` to check staleness.
    pub fn get_metrics(&self, gpu_index: u32) -> NvResult<GpuMetrics> {
        match self.nvml.get_metrics(gpu_index) {
            Ok(metrics) => {
                // Update cache with fresh data
                if let Ok(mut cache) = self.metrics_cache.write() {
                    if (gpu_index as usize) < cache.len() {
                        cache[gpu_index as usize] = Some(CachedMetrics::new(metrics.clone()));
                    }
                }
                Ok(metrics)
            }
            Err(e) => {
                // Try to return cached metrics
                if let Ok(cache) = self.metrics_cache.read() {
                    if let Some(Some(cached)) = cache.get(gpu_index as usize) {
                        return Ok(cached.metrics.clone());
                    }
                }
                Err(e)
            }
        }
    }

    /// Get the age of cached metrics for a GPU in seconds
    ///
    /// Returns None if no cached metrics exist for this GPU.
    pub fn get_cached_metrics_age(&self, gpu_index: u32) -> Option<u64> {
        if let Ok(cache) = self.metrics_cache.read() {
            if let Some(Some(cached)) = cache.get(gpu_index as usize) {
                return Some(cached.age_secs());
            }
        }
        None
    }

    /// Check if cached metrics are stale (older than threshold)
    pub fn are_metrics_stale(&self, gpu_index: u32, max_age_secs: u64) -> bool {
        self.get_cached_metrics_age(gpu_index)
            .map(|age| age > max_age_secs)
            .unwrap_or(false)
    }

    /// Get device info (convenience wrapper)
    pub fn get_device_info(&self, gpu_index: u32) -> NvResult<GpuDeviceInfo> {
        self.nvml.get_device_info(gpu_index)
    }

    /// Refresh backend status with debouncing.
    ///
    /// Call this periodically (e.g., each frame or poll cycle) to check for
    /// hotplug events. Returns the debounced status, which only changes after
    /// the new status has persisted for `STATUS_DEBOUNCE_SECS`.
    ///
    /// This prevents UI flicker when GPUs or displays are rapidly attached/detached.
    pub fn refresh_status(&mut self) -> BackendStatus {
        // Re-check actual backend availability
        let nvml_available = self.nvml.is_available();
        let display_available = self.display.is_available();
        let observed = match (nvml_available, display_available) {
            (true, true) => BackendStatus::Available,
            (false, true) => BackendStatus::NvmlUnavailable("NVML not initialized".to_string()),
            (true, false) => BackendStatus::DisplayUnavailable("No display server".to_string()),
            (false, false) => BackendStatus::AllUnavailable {
                nvml_reason: "NVML not initialized".to_string(),
                display_reason: "No display server".to_string(),
            },
        };

        // Update with debouncing
        if let Ok(mut tracker) = self.status_tracker.write() {
            self.status = tracker.update(observed);
        }

        self.status.clone()
    }

    /// Get the current debounced status without refreshing.
    pub fn get_status(&self) -> &BackendStatus {
        &self.status
    }

    /// Check if a status transition is currently pending (in debounce period).
    ///
    /// Returns true if the observed status differs from the reported status,
    /// indicating that the UI should expect a potential change soon.
    pub fn is_status_transitioning(&self) -> bool {
        if let Ok(tracker) = self.status_tracker.read() {
            tracker.is_transitioning()
        } else {
            false
        }
    }
}

impl Default for GuiBackendContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_single_gpu() {
        let backend = MockNvmlBackend::single_gpu();
        assert_eq!(backend.device_count().unwrap(), 1);
        assert!(backend.is_available());

        let info = backend.get_device_info(0).unwrap();
        assert!(info.name.contains("Mock"));

        let metrics = backend.get_metrics(0).unwrap();
        assert!(metrics.temperature > 0);
        assert!(metrics.memory_total_bytes > 0);
    }

    #[test]
    fn test_mock_multi_gpu() {
        let backend = MockNvmlBackend::multi_gpu(4);
        assert_eq!(backend.device_count().unwrap(), 4);

        for i in 0..4 {
            let info = backend.get_device_info(i).unwrap();
            assert_eq!(info.index, i);
        }
    }

    #[test]
    fn test_mock_no_gpu() {
        let backend = MockNvmlBackend::no_gpu();
        assert_eq!(backend.device_count().unwrap(), 0);
        assert!(!backend.is_available());
        assert!(backend.get_device_info(0).is_err());
    }

    #[test]
    fn test_mock_individual_queries() {
        let backend = MockNvmlBackend::single_gpu();

        assert!(backend.get_temperature(0).unwrap() > 0);
        let (gpu, mem) = backend.get_utilization(0).unwrap();
        assert!(gpu <= 100);
        assert!(mem <= 100);
        assert!(backend.get_power_usage(0).unwrap() > 0);
        assert!(backend.get_fan_speed(0, 0).unwrap() <= 100);
        let (used, total) = backend.get_memory_info(0).unwrap();
        assert!(used <= total);
        assert!(backend.get_gpu_clock(0).unwrap() > 0);
        assert!(backend.get_memory_clock(0).unwrap() > 0);
        assert!(!backend.get_name(0).unwrap().is_empty());
    }

    #[test]
    fn test_real_backend_creation() {
        // Just test that it doesn't panic - actual NVML may not be available
        let backend = RealNvmlBackend::new();
        let _ = backend.is_available();
    }

    #[test]
    fn test_shared_backend() {
        let backend = create_mock_backend();
        assert!(backend.is_available());

        // Test that it can be cloned and used from multiple places
        let backend2 = Arc::clone(&backend);
        assert_eq!(
            backend.device_count().unwrap(),
            backend2.device_count().unwrap()
        );
    }

    #[test]
    fn test_gui_backend_context_mock() {
        let ctx = GuiBackendContext::mock();

        assert!(ctx.is_nvml_available());
        assert_eq!(ctx.device_count, 1);
        assert!(!ctx.driver_version.is_empty());

        // Test convenience methods
        let metrics = ctx.get_metrics(0).unwrap();
        assert!(metrics.temperature > 0);

        let info = ctx.get_device_info(0).unwrap();
        assert!(info.name.contains("Mock"));
    }

    #[test]
    fn test_gui_backend_context_clone() {
        let ctx1 = GuiBackendContext::mock();
        let ctx2 = ctx1.clone();

        // Both contexts should share the same backend
        assert_eq!(ctx1.device_count, ctx2.device_count);
        assert_eq!(
            ctx1.get_metrics(0).unwrap().temperature,
            ctx2.get_metrics(0).unwrap().temperature
        );
    }

    #[test]
    fn test_status_tracker_initial_state() {
        let tracker = StatusTracker::new(BackendStatus::Available);
        assert_eq!(*tracker.get_reported(), BackendStatus::Available);
        assert!(!tracker.is_transitioning());
    }

    #[test]
    fn test_status_tracker_same_status_no_transition() {
        let mut tracker = StatusTracker::new(BackendStatus::Available);

        // Same status should not trigger transition
        let result = tracker.update(BackendStatus::Available);
        assert_eq!(result, BackendStatus::Available);
        assert!(!tracker.is_transitioning());
    }

    #[test]
    fn test_status_tracker_different_status_starts_transition() {
        let mut tracker = StatusTracker::new(BackendStatus::Available);

        // Different status should start transition (debounce period)
        let result = tracker.update(BackendStatus::NvmlUnavailable("test".to_string()));

        // Should still report old status during debounce
        assert_eq!(result, BackendStatus::Available);
        assert!(tracker.is_transitioning());
    }

    #[test]
    fn test_backend_status_equality() {
        assert_eq!(BackendStatus::Available, BackendStatus::Available);
        assert_ne!(
            BackendStatus::Available,
            BackendStatus::NvmlUnavailable("test".to_string())
        );
        assert_ne!(
            BackendStatus::NvmlUnavailable("a".to_string()),
            BackendStatus::NvmlUnavailable("b".to_string())
        );
    }

    #[test]
    fn test_backend_status_methods() {
        let available = BackendStatus::Available;
        assert!(available.is_nvml_available());
        assert!(available.is_display_available());
        assert_eq!(available.status_message(), "All backends available");

        let nvml_unavail = BackendStatus::NvmlUnavailable("test".to_string());
        assert!(!nvml_unavail.is_nvml_available());
        assert!(nvml_unavail.is_display_available());

        let display_unavail = BackendStatus::DisplayUnavailable("test".to_string());
        assert!(display_unavail.is_nvml_available());
        assert!(!display_unavail.is_display_available());

        let all_unavail = BackendStatus::AllUnavailable {
            nvml_reason: "a".to_string(),
            display_reason: "b".to_string(),
        };
        assert!(!all_unavail.is_nvml_available());
        assert!(!all_unavail.is_display_available());
    }

    #[test]
    fn test_gui_context_status_methods() {
        let ctx = GuiBackendContext::mock();

        // Mock should have Available status
        assert_eq!(*ctx.get_status(), BackendStatus::Available);
        assert!(!ctx.is_status_transitioning());
    }
}
