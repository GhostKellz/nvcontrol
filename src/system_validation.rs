use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// System validation for RTX 50-series GPUs
/// Checks for Resizable BAR, 4G Decoding, PCIe Gen 5, and other requirements

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemValidation {
    pub rebar_enabled: bool,
    pub rebar_size_gb: u32,
    pub above_4g_decoding: bool,
    pub pcie_generation: u8,
    pub pcie_lanes: u8,
    pub pcie_speed_gts: f32,
    pub iommu_enabled: bool,
    pub secure_boot_enabled: bool,
    pub gpu_bar_regions: Vec<BarRegion>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarRegion {
    pub region_number: u8,
    pub address: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub prefetchable: bool,
    pub bits: u8, // 32 or 64
}

impl SystemValidation {
    /// Validate system for RTX 50-series installation
    pub fn validate_for_rtx_50() -> NvResult<Self> {
        let mut validation = Self::detect_system_config()?;

        // RTX 50-series specific validation
        validation.validate_rebar_for_blackwell();
        validation.validate_pcie_for_blackwell();
        validation.validate_boot_config();

        Ok(validation)
    }

    /// Detect current system configuration
    fn detect_system_config() -> NvResult<Self> {
        let mut validation = SystemValidation {
            rebar_enabled: false,
            rebar_size_gb: 0,
            above_4g_decoding: false,
            pcie_generation: 0,
            pcie_lanes: 0,
            pcie_speed_gts: 0.0,
            iommu_enabled: false,
            secure_boot_enabled: false,
            gpu_bar_regions: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        // Detect GPU PCIe address
        if let Some(pcie_addr) = Self::find_nvidia_gpu() {
            validation.detect_bar_configuration(&pcie_addr);
            validation.detect_pcie_config(&pcie_addr);
        }

        validation.detect_iommu();
        validation.detect_secure_boot();

        Ok(validation)
    }

    /// Find NVIDIA GPU PCIe address
    fn find_nvidia_gpu() -> Option<String> {
        if let Ok(output) = Command::new("lspci").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("NVIDIA") && line.contains("VGA") {
                    if let Some(addr) = line.split_whitespace().next() {
                        return Some(addr.to_string());
                    }
                }
            }
        }
        None
    }

    /// Detect BAR configuration and Resizable BAR
    fn detect_bar_configuration(&mut self, pcie_addr: &str) {
        if let Ok(output) = Command::new("lspci")
            .args(["-vv", "-s", pcie_addr])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);

            for line in output_str.lines() {
                if line.contains("Region") {
                    if let Some(bar) = Self::parse_bar_region(line) {
                        // Check for Resizable BAR (large prefetchable region)
                        if bar.prefetchable && bar.bits == 64 && bar.size_bytes >= 8_589_934_592 {
                            self.rebar_enabled = true;
                            self.rebar_size_gb = (bar.size_bytes / 1_073_741_824) as u32;
                            self.above_4g_decoding = true; // Required for large BARs
                        }
                        self.gpu_bar_regions.push(bar);
                    }
                }
            }
        }
    }

    /// Parse BAR region from lspci output
    fn parse_bar_region(line: &str) -> Option<BarRegion> {
        // Example: "Region 1: Memory at f000000000 (64-bit, prefetchable) [size=32G]"
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 5 {
            return None;
        }

        let region_num = parts[1].trim_end_matches(':').parse::<u8>().ok()?;
        let address = parts[3].to_string();
        let prefetchable = line.contains("prefetchable");
        let bits = if line.contains("64-bit") { 64 } else { 32 };

        // Parse size
        let size_human = if let Some(size_part) = parts.iter().find(|s| s.starts_with("[size=")) {
            size_part
                .trim_start_matches("[size=")
                .trim_end_matches(']')
                .to_string()
        } else {
            "Unknown".to_string()
        };

        let size_bytes = Self::parse_size_to_bytes(&size_human);

        Some(BarRegion {
            region_number: region_num,
            address,
            size_bytes,
            size_human,
            prefetchable,
            bits,
        })
    }

    /// Convert human-readable size to bytes
    fn parse_size_to_bytes(size_str: &str) -> u64 {
        let size_str = size_str.to_uppercase();
        let number: f64 = size_str
            .chars()
            .take_while(|c| c.is_numeric() || *c == '.')
            .collect::<String>()
            .parse()
            .unwrap_or(0.0);

        if size_str.contains('T') {
            (number * 1_099_511_627_776.0) as u64
        } else if size_str.contains('G') {
            (number * 1_073_741_824.0) as u64
        } else if size_str.contains('M') {
            (number * 1_048_576.0) as u64
        } else if size_str.contains('K') {
            (number * 1024.0) as u64
        } else {
            number as u64
        }
    }

    /// Detect PCIe configuration
    fn detect_pcie_config(&mut self, pcie_addr: &str) {
        if let Ok(output) = Command::new("lspci")
            .args(["-vv", "-s", pcie_addr])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);

            for line in output_str.lines() {
                // LnkCap: Port #0, Speed 16GT/s, Width x16
                if line.contains("LnkCap:") {
                    if let Some(speed_part) = line.split("Speed ").nth(1) {
                        if let Some(speed_str) = speed_part.split(',').next() {
                            self.pcie_speed_gts =
                                speed_str.trim_end_matches("GT/s").parse().unwrap_or(0.0);
                            self.pcie_generation = Self::speed_to_generation(self.pcie_speed_gts);
                        }
                    }
                    if let Some(width_part) = line.split("Width x").nth(1) {
                        if let Some(width_str) = width_part.split(',').next() {
                            self.pcie_lanes = width_str.trim().parse().unwrap_or(0);
                        }
                    }
                }
                // LnkSta: Speed 16GT/s, Width x16 (actual link status)
                else if line.contains("LnkSta:") {
                    if let Some(speed_part) = line.split("Speed ").nth(1) {
                        if let Some(speed_str) = speed_part.split(',').next() {
                            let actual_speed: f32 =
                                speed_str.trim_end_matches("GT/s").parse().unwrap_or(0.0);
                            self.pcie_speed_gts = actual_speed;
                            self.pcie_generation = Self::speed_to_generation(actual_speed);
                        }
                    }
                }
            }
        }
    }

    /// Convert PCIe speed (GT/s) to generation number
    fn speed_to_generation(speed_gts: f32) -> u8 {
        if speed_gts >= 32.0 {
            5 // PCIe Gen 5: 32 GT/s
        } else if speed_gts >= 16.0 {
            4 // PCIe Gen 4: 16 GT/s
        } else if speed_gts >= 8.0 {
            3 // PCIe Gen 3: 8 GT/s
        } else if speed_gts >= 5.0 {
            2 // PCIe Gen 2: 5 GT/s
        } else if speed_gts >= 2.5 {
            1 // PCIe Gen 1: 2.5 GT/s
        } else {
            0
        }
    }

    /// Detect IOMMU status
    fn detect_iommu(&mut self) {
        // Check kernel command line for IOMMU
        if let Ok(cmdline) = std::fs::read_to_string("/proc/cmdline") {
            self.iommu_enabled = cmdline.contains("iommu=on")
                || cmdline.contains("intel_iommu=on")
                || cmdline.contains("amd_iommu=on");
        }

        // Also check dmesg
        if let Ok(output) = Command::new("dmesg").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.contains("IOMMU enabled")
                || output_str.contains("AMD-Vi: ")
                || output_str.contains("DMAR: ")
            {
                self.iommu_enabled = true;
            }
        }
    }

    /// Detect Secure Boot status
    fn detect_secure_boot(&mut self) {
        if let Ok(output) = Command::new("mokutil").arg("--sb-state").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            self.secure_boot_enabled = output_str.contains("SecureBoot enabled");
        }
    }

    /// Validate Resizable BAR for Blackwell GPUs
    fn validate_rebar_for_blackwell(&mut self) {
        if !self.rebar_enabled {
            self.errors.push(
                "RTX 50-series REQUIRES Resizable BAR! Enable in BIOS: Above 4G Decoding + Resizable BAR".to_string()
            );
            self.errors.push(
                "Without ReBAR, RTX 5090 may fail to boot or perform very poorly".to_string(),
            );
        } else if self.rebar_size_gb < 16 {
            self.warnings.push(format!(
                "Resizable BAR size ({} GB) is smaller than VRAM. Recommend 32GB+ BAR for RTX 5090",
                self.rebar_size_gb
            ));
        }

        if !self.above_4g_decoding {
            self.errors.push(
                "Above 4G Decoding NOT detected! This is REQUIRED for RTX 50-series".to_string(),
            );
        }
    }

    /// Validate PCIe configuration for Blackwell GPUs
    fn validate_pcie_for_blackwell(&mut self) {
        if self.pcie_generation < 4 {
            self.warnings.push(format!(
                "PCIe Gen {} detected. RTX 5090 supports PCIe Gen 5 for optimal performance",
                self.pcie_generation
            ));
        } else if self.pcie_generation == 4 {
            self.warnings.push(
                "PCIe Gen 4 detected. RTX 5090 supports Gen 5 but Gen 4 will work fine (<2% perf loss)".to_string()
            );
        }

        if self.pcie_lanes < 16 {
            self.warnings.push(format!(
                "Only x{} PCIe lanes detected. Recommend x16 for RTX 5090",
                self.pcie_lanes
            ));
        }
    }

    /// Validate boot configuration
    fn validate_boot_config(&mut self) {
        // Check for known problematic configurations
        if self.secure_boot_enabled && self.iommu_enabled {
            self.warnings.push(
                "Secure Boot + IOMMU detected. Some RTX 5090 users report boot failures with this combo. Monitor for BAR allocation errors.".to_string()
            );
            self.warnings.push(
                "See: https://forums.developer.nvidia.com/t/linux-fails-to-boot-with-secure-boot-tpm-enabled-nvidia-rtx-5090-bar-allocation-errors/343254".to_string()
            );
        }

        // Check for kernel parameters that may help
        if let Ok(cmdline) = std::fs::read_to_string("/proc/cmdline") {
            if !cmdline.contains("pci=realloc") && !self.rebar_enabled {
                self.warnings.push(
                    "Consider adding 'pci=realloc' to kernel cmdline if you encounter boot issues"
                        .to_string(),
                );
            }
        }
    }

    /// Print validation report
    pub fn print_report(&self) {
        println!("\n=== RTX 50-Series System Validation Report ===\n");

        // Resizable BAR Status
        println!("Resizable BAR:");
        if self.rebar_enabled {
            println!("  ✅ Enabled: {} GB", self.rebar_size_gb);
        } else {
            println!("  ❌ DISABLED - REQUIRED FOR RTX 50-SERIES!");
        }

        // Above 4G Decoding
        println!("\nAbove 4G Decoding:");
        if self.above_4g_decoding {
            println!("  ✅ Enabled");
        } else {
            println!("  ❌ DISABLED - REQUIRED FOR RTX 50-SERIES!");
        }

        // PCIe Configuration
        println!("\nPCIe Configuration:");
        println!("  Generation: PCIe Gen {}", self.pcie_generation);
        println!("  Speed: {} GT/s", self.pcie_speed_gts);
        println!("  Lanes: x{}", self.pcie_lanes);

        // BAR Regions
        if !self.gpu_bar_regions.is_empty() {
            println!("\nGPU BAR Regions:");
            for bar in &self.gpu_bar_regions {
                println!(
                    "  Region {}: {} ({}-bit, {}) - {}",
                    bar.region_number,
                    bar.size_human,
                    bar.bits,
                    if bar.prefetchable {
                        "prefetchable"
                    } else {
                        "non-prefetchable"
                    },
                    bar.address
                );
            }
        }

        // IOMMU & Secure Boot
        println!("\nBoot Configuration:");
        println!(
            "  IOMMU: {}",
            if self.iommu_enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        );
        println!(
            "  Secure Boot: {}",
            if self.secure_boot_enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        );

        // Warnings
        if !self.warnings.is_empty() {
            println!("\n⚠️  WARNINGS:");
            for warning in &self.warnings {
                println!("  • {}", warning);
            }
        }

        // Errors
        if !self.errors.is_empty() {
            println!("\n❌ ERRORS:");
            for error in &self.errors {
                println!("  • {}", error);
            }
        }

        // Overall status
        println!("\n=== Overall Status ===");
        if self.errors.is_empty() {
            println!("✅ System ready for RTX 50-series installation!");
        } else {
            println!("❌ System NOT ready - Fix errors above before installing RTX 5090");
            println!("\nFix these in BIOS:");
            println!("  1. Enable 'Above 4G Decoding'");
            println!("  2. Enable 'Resizable BAR Support'");
            println!("  3. Save and reboot");
        }
    }

    /// Get BIOS recommendations
    pub fn get_bios_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !self.rebar_enabled || !self.above_4g_decoding {
            recommendations.push("Enable 'Above 4G Decoding' in BIOS (required)".to_string());
            recommendations.push("Enable 'Resizable BAR Support' in BIOS (required)".to_string());
            recommendations.push("Some motherboards list it as 'Re-Size BAR Support'".to_string());
        }

        if self.secure_boot_enabled && self.iommu_enabled {
            recommendations
                .push("If experiencing boot issues, try disabling Secure Boot or TPM".to_string());
        }

        if self.pcie_generation < 4 {
            recommendations
                .push("Ensure GPU is in the primary PCIe x16 slot (Gen 4/5 capable)".to_string());
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size() {
        assert_eq!(SystemValidation::parse_size_to_bytes("16M"), 16_777_216);
        assert_eq!(SystemValidation::parse_size_to_bytes("32G"), 34_359_738_368);
        assert_eq!(
            SystemValidation::parse_size_to_bytes("1T"),
            1_099_511_627_776
        );
    }

    #[test]
    fn test_pcie_gen_detection() {
        assert_eq!(SystemValidation::speed_to_generation(32.0), 5);
        assert_eq!(SystemValidation::speed_to_generation(16.0), 4);
        assert_eq!(SystemValidation::speed_to_generation(8.0), 3);
    }
}
