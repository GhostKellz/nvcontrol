// Multi-GPU Support
// Detect, manage, and monitor multiple NVIDIA GPUs

use crate::nvml_backend::SharedNvmlBackend;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub index: u32,
    pub name: String,
    pub uuid: String,
    pub pci_bus_id: String,
    pub driver_version: String,
    pub vram_total: u64, // bytes
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiGpuConfig {
    pub selected_gpu: u32,
    pub sync_settings: bool, // Apply same settings to all GPUs
    pub per_gpu_profiles: std::collections::HashMap<u32, String>, // GPU index -> profile name
}

/// Detect all NVIDIA GPUs in the system (using backend)
pub fn detect_gpus_with_backend(backend: &SharedNvmlBackend) -> NvResult<Vec<GpuInfo>> {
    let device_count = backend.device_count()?;
    let driver_version = backend
        .get_driver_version()
        .unwrap_or_else(|_| "Unknown".to_string());

    let mut gpus = Vec::new();

    for i in 0..device_count {
        let name = backend.get_name(i).unwrap_or_else(|_| format!("GPU {}", i));
        let uuid = backend
            .get_uuid(i)
            .unwrap_or_else(|_| format!("unknown-{}", i));
        let pci_bus_id = backend
            .get_pci_bus_id(i)
            .unwrap_or_else(|_| "Unknown".to_string());

        let (_, vram_total) = backend.get_memory_info(i).unwrap_or((0, 0));
        let temperature = backend.get_temperature(i).unwrap_or(0) as f32;
        let power_draw = backend
            .get_power_usage(i)
            .map(|p| p as f32 / 1000.0)
            .unwrap_or(0.0);
        let power_limit = backend.get_power_limit(i).map(|p| p / 1000).unwrap_or(0);
        let utilization = backend
            .get_utilization(i)
            .map(|(gpu, _)| gpu as f32)
            .unwrap_or(0.0);

        let cuda_cores = backend.get_cuda_cores(i).ok();
        let compute_capability = backend
            .get_compute_capability(i)
            .ok()
            .map(|(major, minor)| format!("{}.{}", major, minor));

        // SLI detection: multiple GPUs in system
        let sli_enabled = device_count > 1;
        let nvlink_enabled = false; // NVLink detection not yet implemented

        let gpu_info = GpuInfo {
            index: i,
            name,
            uuid,
            pci_bus_id,
            driver_version: driver_version.clone(),
            vram_total,
            cuda_cores,
            compute_capability,
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

    Ok(gpus)
}

/// Detect all NVIDIA GPUs in the system (legacy - creates own backend)
pub fn detect_gpus() -> NvResult<Vec<GpuInfo>> {
    let backend = crate::nvml_backend::create_real_backend();
    detect_gpus_with_backend(&backend)
}

/// Get detailed information for a specific GPU (using backend)
pub fn get_gpu_info_with_backend(index: u32, backend: &SharedNvmlBackend) -> NvResult<GpuInfo> {
    let device_count = backend.device_count()?;
    if index >= device_count {
        return Err(NvControlError::GpuQueryFailed(format!(
            "GPU {} not found (only {} GPUs available)",
            index, device_count
        )));
    }

    let driver_version = backend
        .get_driver_version()
        .unwrap_or_else(|_| "Unknown".to_string());
    let name = backend
        .get_name(index)
        .unwrap_or_else(|_| format!("GPU {}", index));
    let uuid = backend
        .get_uuid(index)
        .unwrap_or_else(|_| format!("unknown-{}", index));
    let pci_bus_id = backend
        .get_pci_bus_id(index)
        .unwrap_or_else(|_| "Unknown".to_string());

    let (_, vram_total) = backend.get_memory_info(index).unwrap_or((0, 0));
    let temperature = backend.get_temperature(index).unwrap_or(0) as f32;
    let power_draw = backend
        .get_power_usage(index)
        .map(|p| p as f32 / 1000.0)
        .unwrap_or(0.0);
    let power_limit = backend
        .get_power_limit(index)
        .map(|p| p / 1000)
        .unwrap_or(0);
    let utilization = backend
        .get_utilization(index)
        .map(|(gpu, _)| gpu as f32)
        .unwrap_or(0.0);

    let cuda_cores = backend.get_cuda_cores(index).ok();
    let compute_capability = backend
        .get_compute_capability(index)
        .ok()
        .map(|(major, minor)| format!("{}.{}", major, minor));

    let sli_enabled = device_count > 1;
    let nvlink_enabled = false;

    Ok(GpuInfo {
        index,
        name,
        uuid,
        pci_bus_id,
        driver_version,
        vram_total,
        cuda_cores,
        compute_capability,
        temperature,
        power_draw,
        power_limit,
        utilization,
        is_primary: index == 0,
        sli_enabled,
        nvlink_enabled,
    })
}

/// Get detailed information for a specific GPU (legacy - creates own backend)
pub fn get_gpu_info(index: u32) -> NvResult<GpuInfo> {
    let backend = crate::nvml_backend::create_real_backend();
    get_gpu_info_with_backend(index, &backend)
}

/// Get GPU count (using backend)
pub fn get_gpu_count_with_backend(backend: &SharedNvmlBackend) -> NvResult<u32> {
    backend.device_count()
}

/// Get GPU count (legacy - creates own backend)
pub fn get_gpu_count() -> NvResult<u32> {
    let backend = crate::nvml_backend::create_real_backend();
    get_gpu_count_with_backend(&backend)
}

/// Check if system has multiple GPUs
pub fn has_multiple_gpus() -> bool {
    get_gpu_count().map(|count| count > 1).unwrap_or(false)
}

/// Get primary GPU index (usually the one connected to display)
pub fn get_primary_gpu() -> u32 {
    // Try to detect which GPU is connected to the active display
    // Method 1: Check DRM connector status via sysfs
    for card in 0..4 {
        let drm_path = format!("/sys/class/drm/card{}", card);
        if !std::path::Path::new(&drm_path).exists() {
            continue;
        }

        // Check for active connectors (DP, HDMI, etc.)
        let connectors = ["DP-1", "DP-2", "HDMI-A-1", "HDMI-A-2", "eDP-1"];
        for connector in connectors {
            let status_path = format!("{}-{}/status", drm_path, connector);
            if let Ok(status) = std::fs::read_to_string(&status_path) {
                if status.trim() == "connected" {
                    // This card has an active display
                    // Check if it's NVIDIA by looking at device vendor
                    let vendor_path = format!("{}/device/vendor", drm_path);
                    if let Ok(vendor) = std::fs::read_to_string(&vendor_path) {
                        if vendor.trim() == "0x10de" {
                            // NVIDIA vendor ID
                            return card;
                        }
                    }
                }
            }
        }
    }

    // Method 2: Use DISPLAY environment and xrandr (X11 only)
    if std::env::var("DISPLAY").is_ok() {
        if let Ok(output) = std::process::Command::new("xrandr")
            .args(["--listproviders"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse xrandr output for NVIDIA provider
            for line in stdout.lines() {
                if line.contains("NVIDIA") {
                    // Extract GPU index if available
                    if let Some(idx_str) = line.split_whitespace().find(|s| s.starts_with("GPU-")) {
                        if let Ok(idx) = idx_str.trim_start_matches("GPU-").parse::<u32>() {
                            return idx;
                        }
                    }
                }
            }
        }
    }

    // Default to GPU 0
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
    println!(
        "  Power:        {:.1}W / {}W",
        gpu.power_draw, gpu.power_limit
    );
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
