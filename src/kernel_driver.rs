use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

/// Kernel driver information and optimization for open-gpu-kernel-modules
/// Specific features for NVIDIA driver 580.105.08 with Blackwell/GB202 support

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelDriverInfo {
    pub driver_version: String,
    pub kernel_version: String,
    pub driver_type: DriverType,
    pub gsp_firmware_version: Option<String>,
    pub loaded_modules: Vec<String>,
    pub architecture_support: Vec<String>,
    pub features: DriverFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DriverType {
    Proprietary, // nvidia.ko
    OpenKernel,  // nvidia-open.ko
    Nouveau,     // nouveau.ko
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverFeatures {
    pub gsp_firmware: bool, // GSP (GPU System Processor) firmware
    pub resizable_bar: bool,
    pub confidential_compute: bool, // GB202 feature
    pub nvlink: bool,
    pub display_port_2_1a: bool,  // Blackwell feature
    pub multi_instance_gpu: bool, // MIG support
}

impl KernelDriverInfo {
    /// Detect kernel driver information
    pub fn detect() -> NvResult<Self> {
        let driver_version = Self::get_driver_version();
        let kernel_version = Self::get_kernel_version();
        let driver_type = Self::detect_driver_type();
        let gsp_firmware_version = Self::get_gsp_firmware_version();
        let loaded_modules = Self::get_loaded_modules();
        let architecture_support = Self::get_architecture_support();
        let features = Self::detect_features(&driver_type);

        Ok(KernelDriverInfo {
            driver_version,
            kernel_version,
            driver_type,
            gsp_firmware_version,
            loaded_modules,
            architecture_support,
            features,
        })
    }

    /// Get NVIDIA driver version
    fn get_driver_version() -> String {
        if let Ok(output) = Command::new("nvidia-smi")
            .args(["--query-gpu=driver_version"])
            .arg("--format=csv,noheader")
            .output()
        {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
        "Unknown".to_string()
    }

    /// Get kernel version
    fn get_kernel_version() -> String {
        if let Ok(output) = Command::new("uname").arg("-r").output() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
        "Unknown".to_string()
    }

    /// Detect which NVIDIA driver type is loaded
    fn detect_driver_type() -> DriverType {
        if let Ok(output) = Command::new("lsmod").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);

            if output_str.contains("nvidia_open") {
                return DriverType::OpenKernel;
            } else if output_str.contains("nvidia ") && !output_str.contains("nouveau") {
                return DriverType::Proprietary;
            } else if output_str.contains("nouveau") {
                return DriverType::Nouveau;
            }
        }
        DriverType::Unknown
    }

    /// Get GSP firmware version
    fn get_gsp_firmware_version() -> Option<String> {
        // Check dmesg for GSP firmware version
        if let Ok(output) = Command::new("dmesg").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("NVRM: loading GSP firmware") {
                    // Extract version from log
                    if let Some(version) = line.split("version").nth(1) {
                        return Some(version.trim().to_string());
                    }
                }
            }
        }

        // Fallback: Check /sys/module/nvidia/version
        if let Ok(version) = fs::read_to_string("/sys/module/nvidia/version") {
            return Some(version.trim().to_string());
        }

        None
    }

    /// Get loaded NVIDIA kernel modules
    fn get_loaded_modules() -> Vec<String> {
        let mut modules = Vec::new();

        if let Ok(output) = Command::new("lsmod").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("nvidia") {
                    if let Some(module_name) = line.split_whitespace().next() {
                        modules.push(module_name.to_string());
                    }
                }
            }
        }

        modules
    }

    /// Get supported GPU architectures
    fn get_architecture_support() -> Vec<String> {
        let mut architectures = Vec::new();

        // Check dmesg for architecture support
        if let Ok(output) = Command::new("dmesg").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("NVRM") && (line.contains("GB202") || line.contains("Blackwell")) {
                    if !architectures.contains(&"Blackwell (GB202)".to_string()) {
                        architectures.push("Blackwell (GB202)".to_string());
                    }
                }
                if line.contains("AD102") || line.contains("Ada Lovelace") {
                    if !architectures.contains(&"Ada Lovelace (AD10x)".to_string()) {
                        architectures.push("Ada Lovelace (AD10x)".to_string());
                    }
                }
            }
        }

        if architectures.is_empty() {
            architectures.push("Unknown".to_string());
        }

        architectures
    }

    /// Detect driver features
    fn detect_features(driver_type: &DriverType) -> DriverFeatures {
        let mut features = DriverFeatures {
            gsp_firmware: false,
            resizable_bar: false,
            confidential_compute: false,
            nvlink: false,
            display_port_2_1a: false,
            multi_instance_gpu: false,
        };

        // Open kernel modules require GSP firmware
        if *driver_type == DriverType::OpenKernel {
            features.gsp_firmware = true;
        }

        // Check for ReBAR in dmesg
        if let Ok(output) = Command::new("dmesg").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);

            if output_str.contains("BAR") && output_str.contains("resizable") {
                features.resizable_bar = true;
            }

            if output_str.contains("confidential") || output_str.contains("CC") {
                features.confidential_compute = true;
            }

            if output_str.contains("NVLink") {
                features.nvlink = true;
            }

            if output_str.contains("DP 2.1") || output_str.contains("DisplayPort 2.1") {
                features.display_port_2_1a = true;
            }

            if output_str.contains("MIG") || output_str.contains("Multi-Instance GPU") {
                features.multi_instance_gpu = true;
            }
        }

        features
    }

    /// Check if driver supports RTX 50-series
    pub fn supports_rtx_50_series(&self) -> bool {
        // Driver 550+ required for RTX 50-series
        if let Some(major) = self.driver_version.split('.').next() {
            if let Ok(version) = major.parse::<u32>() {
                return version >= 550;
            }
        }
        false
    }

    /// Check if GSP firmware is properly loaded
    pub fn check_gsp_firmware(&self) -> bool {
        self.gsp_firmware_version.is_some() && self.features.gsp_firmware
    }

    /// Print driver information
    pub fn print_info(&self) {
        println!("\n=== NVIDIA Kernel Driver Information ===\n");

        println!("Driver Version: {}", self.driver_version);
        println!("Kernel Version: {}", self.kernel_version);
        println!("Driver Type: {:?}", self.driver_type);

        if let Some(gsp_version) = &self.gsp_firmware_version {
            println!("GSP Firmware: {}", gsp_version);
        } else if self.driver_type == DriverType::OpenKernel {
            println!("GSP Firmware: Required but version unknown");
        }

        println!("\nLoaded Modules:");
        for module in &self.loaded_modules {
            println!("  • {}", module);
        }

        println!("\nSupported Architectures:");
        for arch in &self.architecture_support {
            println!("  • {}", arch);
        }

        println!("\nFeatures:");
        println!(
            "  GSP Firmware: {}",
            if self.features.gsp_firmware {
                "✅"
            } else {
                "❌"
            }
        );
        println!(
            "  Resizable BAR: {}",
            if self.features.resizable_bar {
                "✅"
            } else {
                "❌"
            }
        );
        println!(
            "  Confidential Compute: {}",
            if self.features.confidential_compute {
                "✅"
            } else {
                "❌"
            }
        );
        println!(
            "  NVLink: {}",
            if self.features.nvlink { "✅" } else { "❌" }
        );
        println!(
            "  DisplayPort 2.1a: {}",
            if self.features.display_port_2_1a {
                "✅"
            } else {
                "❌"
            }
        );
        println!(
            "  Multi-Instance GPU: {}",
            if self.features.multi_instance_gpu {
                "✅"
            } else {
                "❌"
            }
        );

        println!(
            "\nRTX 50-Series Support: {}",
            if self.supports_rtx_50_series() {
                "✅ Yes"
            } else {
                "❌ No (requires driver 550+)"
            }
        );
    }

    /// Get kernel module parameters
    pub fn get_module_params() -> Vec<(String, String)> {
        let mut params = Vec::new();

        // Read nvidia module parameters
        if let Ok(entries) = fs::read_dir("/sys/module/nvidia/parameters") {
            for entry in entries.flatten() {
                let param_name = entry.file_name().to_string_lossy().to_string();
                if let Ok(value) = fs::read_to_string(entry.path()) {
                    params.push((param_name, value.trim().to_string()));
                }
            }
        }

        params
    }

    /// Recommend optimizations for RTX 50-series
    pub fn recommend_optimizations() -> Vec<String> {
        let mut recommendations = Vec::new();

        // Check current modprobe settings
        if let Ok(modprobe) = fs::read_to_string("/etc/modprobe.d/nvidia.conf") {
            if !modprobe.contains("NVreg_EnableGpuFirmware") {
                recommendations.push(
                    "Add 'options nvidia NVreg_EnableGpuFirmware=1' to /etc/modprobe.d/nvidia.conf for GSP firmware".to_string()
                );
            }

            if !modprobe.contains("NVreg_EnableResizableBar") {
                recommendations.push(
                    "Add 'options nvidia NVreg_EnableResizableBar=1' to /etc/modprobe.d/nvidia.conf for ReBAR".to_string()
                );
            }

            if !modprobe.contains("NVreg_DynamicPowerManagement") {
                recommendations.push(
                    "Consider 'options nvidia NVreg_DynamicPowerManagement=0x02' for better power management".to_string()
                );
            }
        } else {
            recommendations.push(
                "Create /etc/modprobe.d/nvidia.conf for kernel module optimization".to_string(),
            );
        }

        recommendations
    }
}

/// Generate optimized modprobe configuration for RTX 50-series
pub fn generate_modprobe_config() -> String {
    r#"# NVIDIA Kernel Module Configuration for RTX 50-Series (Blackwell)
# Generated by nvcontrol

# Enable GSP firmware (required for open kernel modules)
options nvidia NVreg_EnableGpuFirmware=1

# Enable Resizable BAR
options nvidia NVreg_EnableResizableBar=1

# Dynamic Power Management (D3Cold)
options nvidia NVreg_DynamicPowerManagement=0x02

# Preserve video memory allocations across suspend
options nvidia NVreg_PreserveVideoMemoryAllocations=1

# Enable HDMI 2.1 fixed rate link
options nvidia NVreg_EnableHDMI20=1

# Temporal dithering for better image quality
options nvidia NVreg_TemporalDithering=1

# Enable DisplayPort 2.1a support (Blackwell feature)
options nvidia NVreg_EnableDP21=1

# Improve performance with open kernel modules
options nvidia_drm modeset=1

# Enable NVIDIA runtime power management
options nvidia_drm fbdev=1
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_type_detection() {
        let info = KernelDriverInfo::detect();
        assert!(info.is_ok());
    }

    #[test]
    fn test_modprobe_generation() {
        let config = generate_modprobe_config();
        assert!(config.contains("NVreg_EnableGpuFirmware"));
        assert!(config.contains("NVreg_EnableResizableBar"));
    }
}
