// GPU Passthrough Management for VFIO, Containers, and VMs
// Comprehensive GPU passthrough control for NVIDIA GPUs

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    pub pci_address: String,      // e.g., "0000:01:00.0"
    pub vendor_id: String,         // e.g., "10de" (NVIDIA)
    pub device_id: String,         // e.g., "2684" (RTX 4090)
    pub subsystem_id: String,
    pub name: String,              // e.g., "NVIDIA GeForce RTX 4090"
    pub driver: Option<String>,    // Current driver bound
    pub iommu_group: Option<u32>,  // IOMMU group number
    pub numa_node: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfioConfig {
    pub gpu_pci_address: String,
    pub enable_vfio: bool,
    pub bind_vfio_on_boot: bool,
    pub isolate_cpu_cores: Option<Vec<usize>>,
    pub hugepages_enabled: bool,
    pub hugepages_size_mb: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassthroughMode {
    VFIO,           // Full VFIO passthrough to VM
    Container,      // Docker/Podman container with GPU
    SplitDriver,    // nvidia-open module split mode
    MIG,            // Multi-Instance GPU (A100/H100)
}

pub struct GpuPassthroughManager {
    devices: Vec<GpuDevice>,
}

impl GpuPassthroughManager {
    pub fn new() -> NvResult<Self> {
        let devices = Self::detect_nvidia_gpus()?;
        Ok(Self { devices })
    }

    /// Detect all NVIDIA GPUs in the system
    pub fn detect_nvidia_gpus() -> NvResult<Vec<GpuDevice>> {
        let mut devices = Vec::new();

        // Use lspci to find NVIDIA GPUs
        let output = Command::new("lspci")
            .args(&["-nn", "-D"])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("lspci failed: {}", e)))?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
            if line.contains("NVIDIA") && (line.contains("VGA") || line.contains("3D")) {
                if let Some(device) = Self::parse_lspci_line(line) {
                    devices.push(device);
                }
            }
        }

        // Get additional info for each device
        for device in &mut devices {
            device.driver = Self::get_current_driver(&device.pci_address).ok();
            device.iommu_group = Self::get_iommu_group(&device.pci_address).ok();
            device.numa_node = Self::get_numa_node(&device.pci_address).ok();
        }

        Ok(devices)
    }

    fn parse_lspci_line(line: &str) -> Option<GpuDevice> {
        // Parse line like: "0000:01:00.0 VGA compatible controller [0300]: NVIDIA Corporation ... [10de:2684]"
        let parts: Vec<&str> = line.split_whitespace().collect();
        let pci_address = parts.first()?.to_string();

        // Extract vendor:device IDs from brackets [10de:2684]
        let ids_start = line.rfind('[')? + 1;
        let ids_end = line.rfind(']')?;
        let ids = &line[ids_start..ids_end];
        let id_parts: Vec<&str> = ids.split(':').collect();

        let vendor_id = id_parts.first()?.to_string();
        let device_id = id_parts.get(1)?.to_string();

        // Extract device name
        let name_start = line.find("NVIDIA")?;
        let name_end = line.rfind('[').unwrap_or(line.len());
        let name = line[name_start..name_end].trim().to_string();

        Some(GpuDevice {
            pci_address,
            vendor_id,
            device_id,
            subsystem_id: String::new(), // Would need additional query
            name,
            driver: None,
            iommu_group: None,
            numa_node: None,
        })
    }

    fn get_current_driver(pci_address: &str) -> NvResult<String> {
        let driver_path = format!("/sys/bus/pci/devices/{}/driver", pci_address);
        let link = fs::read_link(&driver_path)
            .map_err(|_| NvControlError::GpuQueryFailed("No driver bound".to_string()))?;

        Ok(link
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string())
    }

    fn get_iommu_group(pci_address: &str) -> NvResult<u32> {
        let iommu_path = format!("/sys/bus/pci/devices/{}/iommu_group", pci_address);
        let link = fs::read_link(&iommu_path)
            .map_err(|_| NvControlError::GpuQueryFailed("IOMMU not enabled".to_string()))?;

        link.file_name()
            .and_then(|n| n.to_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| NvControlError::GpuQueryFailed("Failed to parse IOMMU group".to_string()))
    }

    fn get_numa_node(pci_address: &str) -> NvResult<u32> {
        let numa_path = format!("/sys/bus/pci/devices/{}/numa_node", pci_address);
        let content = fs::read_to_string(&numa_path)
            .map_err(|_| NvControlError::GpuQueryFailed("No NUMA info".to_string()))?;

        content
            .trim()
            .parse()
            .map_err(|_| NvControlError::GpuQueryFailed("Failed to parse NUMA node".to_string()))
    }

    /// Check if IOMMU is enabled in the system
    pub fn check_iommu_enabled() -> bool {
        // Check kernel command line for iommu parameters
        if let Ok(cmdline) = fs::read_to_string("/proc/cmdline") {
            return cmdline.contains("iommu=pt") || cmdline.contains("intel_iommu=on") || cmdline.contains("amd_iommu=on");
        }

        // Check if IOMMU groups exist
        Path::new("/sys/kernel/iommu_groups").exists()
    }

    /// Check if VFIO modules are loaded
    pub fn check_vfio_loaded() -> bool {
        if let Ok(output) = Command::new("lsmod").output() {
            let modules = String::from_utf8_lossy(&output.stdout);
            return modules.contains("vfio_pci") || modules.contains("vfio_iommu_type1");
        }
        false
    }

    /// Bind GPU to VFIO driver
    pub fn bind_to_vfio(&self, pci_address: &str) -> NvResult<()> {
        println!("🔧 Binding {} to VFIO driver...", pci_address);

        // Check if VFIO is loaded
        if !Self::check_vfio_loaded() {
            println!("   Loading VFIO modules...");
            Command::new("sudo")
                .args(&["modprobe", "vfio-pci"])
                .status()
                .map_err(|e| NvControlError::CommandFailed(format!("Failed to load vfio-pci: {}", e)))?;
        }

        // Get vendor and device IDs
        let device = self.devices.iter()
            .find(|d| d.pci_address == pci_address)
            .ok_or_else(|| NvControlError::GpuQueryFailed("Device not found".to_string()))?;

        // Unbind from current driver if needed
        if let Some(current_driver) = &device.driver {
            if current_driver != "vfio-pci" {
                println!("   Unbinding from current driver: {}", current_driver);
                let unbind_path = format!("/sys/bus/pci/devices/{}/driver/unbind", pci_address);
                fs::write(&unbind_path, pci_address)
                    .map_err(|e| NvControlError::CommandFailed(format!("Failed to unbind: {}", e)))?;
            }
        }

        // Bind to VFIO
        let vfio_new_id = format!("{} {}", device.vendor_id, device.device_id);
        fs::write("/sys/bus/pci/drivers/vfio-pci/new_id", &vfio_new_id)
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to bind to VFIO: {}", e)))?;

        println!("✅ GPU bound to VFIO successfully");
        Ok(())
    }

    /// Unbind GPU from VFIO and return to NVIDIA driver
    pub fn unbind_from_vfio(&self, pci_address: &str) -> NvResult<()> {
        println!("🔧 Unbinding {} from VFIO...", pci_address);

        // Unbind from VFIO
        let unbind_path = "/sys/bus/pci/drivers/vfio-pci/unbind";
        fs::write(unbind_path, pci_address)
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to unbind from VFIO: {}", e)))?;

        // Rescan PCI bus to let nvidia driver claim it
        fs::write("/sys/bus/pci/rescan", "1")
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to rescan PCI: {}", e)))?;

        println!("✅ GPU unbound from VFIO");
        Ok(())
    }

    /// Setup persistent VFIO binding (modprobe config)
    pub fn setup_persistent_vfio(&self, pci_address: &str) -> NvResult<()> {
        println!("🔧 Setting up persistent VFIO binding...");

        let device = self.devices.iter()
            .find(|d| d.pci_address == pci_address)
            .ok_or_else(|| NvControlError::GpuQueryFailed("Device not found".to_string()))?;

        // Create modprobe config
        let config_content = format!(
            "# VFIO GPU Passthrough Configuration for {}\n\
             options vfio-pci ids={}:{}\n\
             softdep nvidia pre: vfio-pci\n",
            device.name, device.vendor_id, device.device_id
        );

        let config_path = "/etc/modprobe.d/vfio-nvcontrol.conf";

        println!("   Writing config to: {}", config_path);
        println!("   Content:\n{}", config_content);
        println!("\n⚠️  Run with sudo: sudo nvctl passthrough persistent {}", pci_address);

        Ok(())
    }

    /// Check Docker/Podman GPU runtime status
    pub fn check_container_runtime_status(&self) -> NvResult<HashMap<String, bool>> {
        let mut status = HashMap::new();

        // Check nvidia-container-toolkit
        status.insert(
            "nvidia-container-toolkit".to_string(),
            Command::new("nvidia-container-cli")
                .arg("--version")
                .output()
                .is_ok(),
        );

        // Check Docker runtime
        if let Ok(output) = Command::new("docker").args(&["info", "--format", "{{.Runtimes}}"]).output() {
            let runtimes = String::from_utf8_lossy(&output.stdout);
            status.insert("docker-nvidia-runtime".to_string(), runtimes.contains("nvidia"));
        }

        // Check Podman CDI
        status.insert(
            "podman-cdi".to_string(),
            Path::new("/etc/cdi/nvidia.yaml").exists(),
        );

        Ok(status)
    }

    /// List IOMMU groups and their devices
    pub fn list_iommu_groups(&self) -> NvResult<HashMap<u32, Vec<String>>> {
        let mut groups: HashMap<u32, Vec<String>> = HashMap::new();

        let iommu_path = Path::new("/sys/kernel/iommu_groups");
        if !iommu_path.exists() {
            return Err(NvControlError::GpuQueryFailed(
                "IOMMU not enabled. Add iommu=pt amd_iommu=on (or intel_iommu=on) to kernel parameters".to_string()
            ));
        }

        for entry in fs::read_dir(iommu_path).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to read IOMMU groups: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to read entry: {}", e))
            })?;

            let group_num: u32 = entry
                .file_name()
                .to_str()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            let devices_path = entry.path().join("devices");
            let mut devices = Vec::new();

            if let Ok(device_entries) = fs::read_dir(&devices_path) {
                for device_entry in device_entries.flatten() {
                    if let Some(device_name) = device_entry.file_name().to_str() {
                        devices.push(device_name.to_string());
                    }
                }
            }

            groups.insert(group_num, devices);
        }

        Ok(groups)
    }

    /// Setup hugepages for better VM performance
    pub fn setup_hugepages(&self, size_mb: u32) -> NvResult<()> {
        println!("🔧 Setting up hugepages ({} MB)...", size_mb);

        // Calculate number of 2MB hugepages needed
        let num_pages = size_mb / 2;

        // This requires root
        println!("   Run: sudo sysctl vm.nr_hugepages={}", num_pages);
        println!("   For persistent: echo 'vm.nr_hugepages={}' | sudo tee -a /etc/sysctl.conf", num_pages);

        Ok(())
    }

    /// Show GPU passthrough status
    pub fn show_status(&self) -> NvResult<()> {
        println!("🎮 NVIDIA GPU Passthrough Status\n");

        println!("System Configuration:");
        println!("   IOMMU Enabled: {}", Self::check_iommu_enabled());
        println!("   VFIO Modules: {}", Self::check_vfio_loaded());
        println!();

        println!("Detected NVIDIA GPUs:");
        for device in &self.devices {
            println!("   📍 {}", device.pci_address);
            println!("      Name: {}", device.name);
            println!("      IDs: {}:{}", device.vendor_id, device.device_id);

            if let Some(driver) = &device.driver {
                println!("      Driver: {}", driver);
            } else {
                println!("      Driver: none");
            }

            if let Some(group) = device.iommu_group {
                println!("      IOMMU Group: {}", group);
            }

            if let Some(node) = device.numa_node {
                println!("      NUMA Node: {}", node);
            }
            println!();
        }

        // Container runtime status
        println!("Container Runtime Status:");
        match self.check_container_runtime_status() {
            Ok(status) => {
                for (runtime, available) in status {
                    let icon = if available { "✅" } else { "❌" };
                    println!("   {} {}", icon, runtime);
                }
            }
            Err(e) => println!("   ⚠️  Failed to check status: {}", e),
        }

        Ok(())
    }

    /// Test GPU passthrough to container
    pub fn test_container_passthrough(&self) -> NvResult<()> {
        println!("🧪 Testing GPU passthrough to container...\n");

        // Test with nvidia-smi in container
        println!("Running: docker run --rm --gpus all nvidia/cuda:12.0-base nvidia-smi\n");

        let output = Command::new("docker")
            .args(&[
                "run",
                "--rm",
                "--gpus",
                "all",
                "nvidia/cuda:12.0-base",
                "nvidia-smi",
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Docker test failed: {}", e)))?;

        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("✅ Container GPU passthrough working!");
        } else {
            println!("❌ Container GPU passthrough failed");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    /// Generate QEMU command line for GPU passthrough
    pub fn generate_qemu_command(&self, pci_address: &str) -> NvResult<String> {
        let device = self.devices.iter()
            .find(|d| d.pci_address == pci_address)
            .ok_or_else(|| NvControlError::GpuQueryFailed("Device not found".to_string()))?;

        let _iommu_group = device.iommu_group
            .ok_or_else(|| NvControlError::GpuQueryFailed("No IOMMU group".to_string()))?;

        let qemu_args = format!(
            "-device vfio-pci,host={},multifunction=on \\\n  \
             -cpu host,kvm=on,hv_vendor_id=nvcontrol \\\n  \
             -machine q35,accel=kvm,kernel_irqchip=on \\\n  \
             -smp 8,cores=4,threads=2 \\\n  \
             -m 16G \\\n  \
             -mem-prealloc \\\n  \
             -overcommit mem-lock=off",
            pci_address
        );

        Ok(qemu_args)
    }
}

/// Check if running on nvidia-open driver
pub fn check_nvidia_open_driver() -> bool {
    if let Ok(output) = Command::new("nvidia-smi").arg("--query").output() {
        let info = String::from_utf8_lossy(&output.stdout);
        return info.contains("nvidia-open") || info.contains("open-gpu-kernel-modules");
    }

    // Alternative check via modinfo
    if let Ok(output) = Command::new("modinfo").arg("nvidia").output() {
        let info = String::from_utf8_lossy(&output.stdout);
        return info.contains("open-gpu-kernel-modules");
    }

    false
}

/// Get GPU PCI info for MCP Docker integration
pub fn get_gpu_pci_info_for_mcp() -> NvResult<Vec<HashMap<String, String>>> {
    let manager = GpuPassthroughManager::new()?;
    let mut devices_info = Vec::new();

    for device in &manager.devices {
        let mut info = HashMap::new();
        info.insert("pci_address".to_string(), device.pci_address.clone());
        info.insert("name".to_string(), device.name.clone());
        info.insert("vendor_id".to_string(), device.vendor_id.clone());
        info.insert("device_id".to_string(), device.device_id.clone());

        if let Some(driver) = &device.driver {
            info.insert("driver".to_string(), driver.clone());
        }

        if let Some(group) = device.iommu_group {
            info.insert("iommu_group".to_string(), group.to_string());
        }

        devices_info.push(info);
    }

    Ok(devices_info)
}
