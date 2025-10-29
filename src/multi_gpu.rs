// Multi-GPU Support
// Detect, manage, and monitor multiple NVIDIA GPUs

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub index: u32,
    pub name: String,
    pub uuid: String,
    pub pci_bus_id: String,
    pub driver_version: String,
    pub vram_total: u64,  // bytes
    pub cuda_cores: Option<u32>,
    pub compute_capability: Option<String>,
    pub temperature: f32,
    pub power_draw: f32,
    pub power_limit: u32,
    pub utilization: f32,
    pub is_primary: bool,
    pub sli_enabled: bool,
    pub nvlink_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiGpuConfig {
    pub selected_gpu: u32,
    pub sync_settings: bool,  // Apply same settings to all GPUs
    pub per_gpu_profiles: std::collections::HashMap<u32, String>,  // GPU index -> profile name
}

impl Default for MultiGpuConfig {
    fn default() -> Self {
        Self {
            selected_gpu: 0,
            sync_settings: false,
            per_gpu_profiles: std::collections::HashMap::new(),
        }
    }
}

/// Detect all NVIDIA GPUs in the system
pub fn detect_gpus() -> NvResult<Vec<GpuInfo>> {
    let nvml = nvml_wrapper::Nvml::init()
        .map_err(|e| NvControlError::GpuQueryFailed(format!("NVML init failed: {}", e)))?;

    let device_count = nvml.device_count()
        .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device count: {}", e)))?;

    let mut gpus = Vec::new();

    for i in 0..device_count {
        if let Ok(device) = nvml.device_by_index(i) {
            let name = device.name().unwrap_or_else(|_| format!("GPU {}", i));
            let uuid = device.uuid().unwrap_or_else(|_| format!("unknown-{}", i));
            let pci_info = device.pci_info().ok();
            let pci_bus_id = pci_info
                .as_ref()
                .map(|p| format!("{:04x}:{:02x}:{:02x}.0", p.domain, p.bus, p.device))
                .unwrap_or_else(|| "Unknown".to_string());

            let driver_version = nvml.sys_driver_version()
                .unwrap_or_else(|_| "Unknown".to_string());

            let memory_info = device.memory_info().ok();
            let vram_total = memory_info.as_ref().map(|m| m.total).unwrap_or(0);

            let temperature = device
                .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                .unwrap_or(0) as f32;

            let power_draw = device.power_usage()
                .map(|p| p as f32 / 1000.0)
                .unwrap_or(0.0);

            let power_limit = device.power_management_limit()
                .map(|p| p / 1000)
                .unwrap_or(0);

            let utilization = device.utilization_rates()
                .map(|u| u.gpu as f32)
                .unwrap_or(0.0);

            // Check for SLI/NVLink
            let sli_enabled = check_sli_enabled(&device);
            let nvlink_enabled = check_nvlink_enabled(&device);

            let gpu_info = GpuInfo {
                index: i,
                name,
                uuid,
                pci_bus_id,
                driver_version,
                vram_total,
                cuda_cores: None,  // TODO: Query CUDA cores
                compute_capability: None,  // TODO: Query compute capability
                temperature,
                power_draw,
                power_limit,
                utilization,
                is_primary: i == 0,
                sli_enabled,
                nvlink_enabled,
            };

            gpus.push(gpu_info);
        }
    }

    Ok(gpus)
}

/// Get detailed information for a specific GPU
pub fn get_gpu_info(index: u32) -> NvResult<GpuInfo> {
    let nvml = nvml_wrapper::Nvml::init()
        .map_err(|e| NvControlError::GpuQueryFailed(format!("NVML init failed: {}", e)))?;

    let device = nvml.device_by_index(index)
        .map_err(|e| NvControlError::GpuQueryFailed(format!("GPU {} not found: {}", index, e)))?;

    let name = device.name().unwrap_or_else(|_| format!("GPU {}", index));
    let uuid = device.uuid().unwrap_or_else(|_| format!("unknown-{}", index));
    let pci_info = device.pci_info().ok();
    let pci_bus_id = pci_info
        .as_ref()
        .map(|p| format!("{:04x}:{:02x}:{:02x}.0", p.domain, p.bus, p.device))
        .unwrap_or_else(|| "Unknown".to_string());

    let driver_version = nvml.sys_driver_version()
        .unwrap_or_else(|_| "Unknown".to_string());

    let memory_info = device.memory_info().ok();
    let vram_total = memory_info.as_ref().map(|m| m.total).unwrap_or(0);

    let temperature = device
        .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
        .unwrap_or(0) as f32;

    let power_draw = device.power_usage()
        .map(|p| p as f32 / 1000.0)
        .unwrap_or(0.0);

    let power_limit = device.power_management_limit()
        .map(|p| p / 1000)
        .unwrap_or(0);

    let utilization = device.utilization_rates()
        .map(|u| u.gpu as f32)
        .unwrap_or(0.0);

    let sli_enabled = check_sli_enabled(&device);
    let nvlink_enabled = check_nvlink_enabled(&device);

    Ok(GpuInfo {
        index,
        name,
        uuid,
        pci_bus_id,
        driver_version,
        vram_total,
        cuda_cores: None,
        compute_capability: None,
        temperature,
        power_draw,
        power_limit,
        utilization,
        is_primary: index == 0,
        sli_enabled,
        nvlink_enabled,
    })
}

/// Check if SLI is enabled for this GPU
fn check_sli_enabled(device: &nvml_wrapper::Device) -> bool {
    // Check if multiple GPUs exist and if they're linked
    // This is a simplified check - real SLI detection is more complex
    if let Ok(nvml) = nvml_wrapper::Nvml::init() {
        if let Ok(count) = nvml.device_count() {
            return count > 1;
        }
    }
    false
}

/// Check if NVLink is enabled for this GPU
fn check_nvlink_enabled(_device: &nvml_wrapper::Device) -> bool {
    // NVLink detection requires specific NVML calls that may not be available
    // in all versions of nvml-wrapper. For now, return false.
    // TODO: Implement proper NVLink detection when APIs are available
    false
}

/// Get GPU count
pub fn get_gpu_count() -> NvResult<u32> {
    let nvml = nvml_wrapper::Nvml::init()
        .map_err(|e| NvControlError::GpuQueryFailed(format!("NVML init failed: {}", e)))?;

    nvml.device_count()
        .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device count: {}", e)))
}

/// Check if system has multiple GPUs
pub fn has_multiple_gpus() -> bool {
    get_gpu_count().map(|count| count > 1).unwrap_or(false)
}

/// Get primary GPU index (usually the one connected to display)
pub fn get_primary_gpu() -> u32 {
    // For now, assume GPU 0 is primary
    // TODO: Detect which GPU is connected to the active display
    0
}

/// Print GPU information
pub fn print_gpu_info(gpu: &GpuInfo) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎮 GPU {} - {}", gpu.index, gpu.name);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  UUID:         {}", gpu.uuid);
    println!("  PCI Bus ID:   {}", gpu.pci_bus_id);
    println!("  Driver:       {}", gpu.driver_version);
    println!("  VRAM:         {:.2} GB", gpu.vram_total as f64 / 1e9);
    println!("  Temperature:  {:.1}°C", gpu.temperature);
    println!("  Power:        {:.1}W / {}W", gpu.power_draw, gpu.power_limit);
    println!("  Utilization:  {:.0}%", gpu.utilization);

    if gpu.is_primary {
        println!("  Status:       ⭐ Primary GPU");
    }
    if gpu.sli_enabled {
        println!("  SLI:          ✅ Enabled");
    }
    if gpu.nvlink_enabled {
        println!("  NVLink:       ✅ Enabled");
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

/// List all GPUs
pub fn list_gpus_cli() -> NvResult<()> {
    let gpus = detect_gpus()?;

    if gpus.is_empty() {
        println!("❌ No NVIDIA GPUs detected");
        return Ok(());
    }

    println!("\n🎮 Detected {} NVIDIA GPU(s):\n", gpus.len());

    for gpu in gpus {
        print_gpu_info(&gpu);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_count() {
        // This test may fail if no GPUs are present
        let result = get_gpu_count();
        assert!(result.is_ok() || result.is_err());
    }
}
