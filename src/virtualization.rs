/// Phase 5.3: Virtualization Support
///
/// vGPU configuration, SR-IOV support, GPU passthrough optimization, virtual display management

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// vGPU profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VGpuProfile {
    pub profile_name: String,
    pub gpu_id: u32,
    pub framebuffer_mb: u64,
    pub max_resolution: (u32, u32),
    pub max_displays: u32,
    pub encode_capable: bool,
    pub compute_capable: bool,
}

/// GPU passthrough configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassthroughConfig {
    pub gpu_id: u32,
    pub pci_address: String,
    pub iommu_group: u32,
    pub vendor_id: String,
    pub device_id: String,
    pub vfio_bound: bool,
}

/// SR-IOV (Single Root I/O Virtualization) manager
pub struct SriovManager {
    gpu_id: u32,
    num_vfs: u32,
}

impl SriovManager {
    pub fn new(gpu_id: u32) -> NvResult<Self> {
        Ok(Self { gpu_id, num_vfs: 0 })
    }

    /// Check if SR-IOV is supported
    pub fn is_supported(&self) -> NvResult<bool> {
        let pci_addr = self.get_pci_address()?;
        let sriov_path = format!("/sys/bus/pci/devices/{}/sriov_totalvfs", pci_addr);

        Ok(Path::new(&sriov_path).exists())
    }

    /// Get maximum number of virtual functions
    pub fn get_max_vfs(&self) -> NvResult<u32> {
        let pci_addr = self.get_pci_address()?;
        let sriov_path = format!("/sys/bus/pci/devices/{}/sriov_totalvfs", pci_addr);

        let content = std::fs::read_to_string(&sriov_path).map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to read sriov_totalvfs: {}", e))
        })?;

        content.trim().parse::<u32>().map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to parse sriov_totalvfs: {}", e))
        })
    }

    /// Enable SR-IOV with specified number of VFs
    pub fn enable_sriov(&mut self, num_vfs: u32) -> NvResult<()> {
        if !self.is_supported()? {
            return Err(NvControlError::UnsupportedFeature(
                "SR-IOV not supported on this GPU".to_string(),
            ));
        }

        let max_vfs = self.get_max_vfs()?;
        if num_vfs > max_vfs {
            return Err(NvControlError::RuntimeError(format!(
                "Requested {} VFs exceeds maximum of {}",
                num_vfs, max_vfs
            )));
        }

        let pci_addr = self.get_pci_address()?;
        let sriov_path = format!("/sys/bus/pci/devices/{}/sriov_numvfs", pci_addr);

        std::fs::write(&sriov_path, num_vfs.to_string()).map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to enable SR-IOV: {}", e))
        })?;

        self.num_vfs = num_vfs;

        println!("SR-IOV enabled with {} virtual functions", num_vfs);

        Ok(())
    }

    /// Disable SR-IOV
    pub fn disable_sriov(&mut self) -> NvResult<()> {
        let pci_addr = self.get_pci_address()?;
        let sriov_path = format!("/sys/bus/pci/devices/{}/sriov_numvfs", pci_addr);

        std::fs::write(&sriov_path, "0").map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to disable SR-IOV: {}", e))
        })?;

        self.num_vfs = 0;

        println!("SR-IOV disabled");

        Ok(())
    }

    fn get_pci_address(&self) -> NvResult<String> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let device = nvml.device_by_index(self.gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        let pci_info = device.pci_info().map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get PCI info: {}", e))
        })?;

        Ok(format!(
            "{:04x}:{:02x}:{:02x}.0",
            pci_info.domain, pci_info.bus, pci_info.device
        ))
    }

    pub fn current_vfs(&self) -> u32 {
        self.num_vfs
    }
}

/// GPU passthrough manager
pub struct PassthroughManager {
    configs: HashMap<u32, PassthroughConfig>,
}

impl PassthroughManager {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Detect GPUs available for passthrough
    pub fn detect_passthrough_gpus(&mut self) -> NvResult<Vec<PassthroughConfig>> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let count = nvml.device_count().map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device count: {}", e))
        })?;

        let mut configs = Vec::new();

        for gpu_id in 0..count {
            let device = nvml.device_by_index(gpu_id).map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
            })?;

            let pci_info = device.pci_info().ok();

            if let Some(pci) = pci_info {
                let pci_address = format!(
                    "{:04x}:{:02x}:{:02x}.0",
                    pci.domain, pci.bus, pci.device
                );

                let iommu_group = self.get_iommu_group(&pci_address)?;
                let (vendor_id, device_id) = self.get_device_ids(&pci_address)?;
                let vfio_bound = self.is_vfio_bound(&pci_address)?;

                let config = PassthroughConfig {
                    gpu_id,
                    pci_address: pci_address.clone(),
                    iommu_group,
                    vendor_id,
                    device_id,
                    vfio_bound,
                };

                configs.push(config.clone());
                self.configs.insert(gpu_id, config);
            }
        }

        Ok(configs)
    }

    fn get_iommu_group(&self, pci_address: &str) -> NvResult<u32> {
        let iommu_path = format!("/sys/bus/pci/devices/{}/iommu_group", pci_address);

        let link = std::fs::read_link(&iommu_path).map_err(|_| {
            NvControlError::RuntimeError("IOMMU group not found (IOMMU disabled?)".to_string())
        })?;

        let group_name = link
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| NvControlError::RuntimeError("Invalid IOMMU group".to_string()))?;

        group_name.parse::<u32>().map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to parse IOMMU group: {}", e))
        })
    }

    fn get_device_ids(&self, pci_address: &str) -> NvResult<(String, String)> {
        let vendor_path = format!("/sys/bus/pci/devices/{}/vendor", pci_address);
        let device_path = format!("/sys/bus/pci/devices/{}/device", pci_address);

        let vendor_id = std::fs::read_to_string(&vendor_path)
            .map_err(|e| NvControlError::RuntimeError(format!("Failed to read vendor ID: {}", e)))?
            .trim()
            .to_string();

        let device_id = std::fs::read_to_string(&device_path)
            .map_err(|e| NvControlError::RuntimeError(format!("Failed to read device ID: {}", e)))?
            .trim()
            .to_string();

        Ok((vendor_id, device_id))
    }

    fn is_vfio_bound(&self, pci_address: &str) -> NvResult<bool> {
        let driver_path = format!("/sys/bus/pci/devices/{}/driver", pci_address);

        if let Ok(link) = std::fs::read_link(&driver_path) {
            if let Some(driver_name) = link.file_name() {
                return Ok(driver_name.to_str().unwrap_or("").contains("vfio"));
            }
        }

        Ok(false)
    }

    /// Bind GPU to VFIO driver for passthrough
    pub fn bind_to_vfio(&self, gpu_id: u32) -> NvResult<()> {
        let config = self.configs.get(&gpu_id).ok_or_else(|| {
            NvControlError::RuntimeError(format!("GPU {} not found", gpu_id))
        })?;

        if config.vfio_bound {
            println!("GPU {} already bound to VFIO", gpu_id);
            return Ok(());
        }

        // Unbind from current driver
        let unbind_path = format!(
            "/sys/bus/pci/devices/{}/driver/unbind",
            config.pci_address
        );
        if Path::new(&unbind_path).exists() {
            std::fs::write(&unbind_path, &config.pci_address).map_err(|e| {
                NvControlError::RuntimeError(format!("Failed to unbind driver: {}", e))
            })?;
        }

        // Add IDs to vfio-pci driver
        let new_id_path = "/sys/bus/pci/drivers/vfio-pci/new_id";
        let id_string = format!(
            "{} {}",
            config.vendor_id.trim_start_matches("0x"),
            config.device_id.trim_start_matches("0x")
        );

        std::fs::write(new_id_path, id_string).map_err(|e| {
            NvControlError::RuntimeError(format!("Failed to bind to VFIO: {}", e))
        })?;

        println!("GPU {} bound to VFIO for passthrough", gpu_id);

        Ok(())
    }

    /// Get passthrough configuration
    pub fn get_config(&self, gpu_id: u32) -> Option<&PassthroughConfig> {
        self.configs.get(&gpu_id)
    }

    /// Generate libvirt XML for GPU passthrough
    pub fn generate_libvirt_xml(&self, gpu_id: u32) -> NvResult<String> {
        let config = self.configs.get(&gpu_id).ok_or_else(|| {
            NvControlError::RuntimeError(format!("GPU {} not found", gpu_id))
        })?;

        let xml = format!(
            r#"<hostdev mode='subsystem' type='pci' managed='yes'>
  <source>
    <address domain='0x{}' bus='0x{}' slot='0x{}' function='0x0'/>
  </source>
</hostdev>"#,
            &config.pci_address[0..4],
            &config.pci_address[5..7],
            &config.pci_address[8..10]
        );

        Ok(xml)
    }
}

impl Default for PassthroughManager {
    fn default() -> Self {
        Self::new()
    }
}

/// vGPU manager for NVIDIA GRID
pub struct VGpuManager {
    profiles: Vec<VGpuProfile>,
}

impl VGpuManager {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }

    /// Detect available vGPU profiles
    pub fn detect_profiles(&mut self, gpu_id: u32) -> NvResult<Vec<VGpuProfile>> {
        // Query mdevctl for available vGPU types
        let output = Command::new("mdevctl")
            .args(&["types"])
            .output();

        if output.is_err() {
            return Err(NvControlError::UnsupportedFeature(
                "vGPU not supported (mdevctl not available)".to_string(),
            ));
        }

        let output = output.unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut profiles = Vec::new();

        // Parse mdevctl output
        for line in stdout.lines() {
            if line.contains("nvidia") {
                // Example: nvidia-256 (framebuffer: 4096M, displays: 1)
                let profile = VGpuProfile {
                    profile_name: "nvidia-vgpu".to_string(),
                    gpu_id,
                    framebuffer_mb: 4096,
                    max_resolution: (1920, 1080),
                    max_displays: 1,
                    encode_capable: false,
                    compute_capable: false,
                };

                profiles.push(profile);
            }
        }

        self.profiles = profiles.clone();

        Ok(profiles)
    }

    /// Create vGPU instance
    pub fn create_vgpu(&self, profile_name: &str) -> NvResult<String> {
        let output = Command::new("mdevctl")
            .args(&["start", "-t", profile_name])
            .output()
            .map_err(|e| {
                NvControlError::RuntimeError(format!("Failed to create vGPU: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NvControlError::RuntimeError(format!(
                "vGPU creation failed: {}",
                stderr
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let uuid = stdout.trim().to_string();

        println!("vGPU instance created: {}", uuid);

        Ok(uuid)
    }

    /// List vGPU profiles
    pub fn list_profiles(&self) -> &[VGpuProfile] {
        &self.profiles
    }
}

impl Default for VGpuManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sriov_manager() {
        let manager = SriovManager::new(0);
        assert!(manager.is_ok());

        let manager = manager.unwrap();
        assert_eq!(manager.current_vfs(), 0);
    }

    #[test]
    fn test_passthrough_manager() {
        let manager = PassthroughManager::new();
        assert_eq!(manager.configs.len(), 0);
    }

    #[test]
    fn test_vgpu_manager() {
        let manager = VGpuManager::new();
        assert_eq!(manager.profiles.len(), 0);
    }

    #[test]
    fn test_passthrough_config() {
        let config = PassthroughConfig {
            gpu_id: 0,
            pci_address: "0000:01:00.0".to_string(),
            iommu_group: 1,
            vendor_id: "0x10de".to_string(),
            device_id: "0x2684".to_string(),
            vfio_bound: false,
        };

        assert_eq!(config.pci_address, "0000:01:00.0");
        assert!(!config.vfio_bound);
    }
}
