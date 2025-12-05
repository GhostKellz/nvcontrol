// NVIDIA GSP Firmware Manager (nvidia-open specific)
// Monitor, control, and update GSP firmware

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GspFirmwareInfo {
    pub enabled: bool,
    pub version: String,
    pub path: PathBuf,
    pub file_size: u64,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GspTelemetry {
    pub power_usage: f32,
    pub temperature: f32,
    pub clock_speed: u32,
    pub errors: u32,
}

pub struct GspManager {
    firmware_dir: PathBuf,
}

impl GspManager {
    pub fn new() -> Self {
        Self {
            firmware_dir: PathBuf::from("/usr/lib/firmware/nvidia"),
        }
    }

    /// Check if GSP firmware is enabled
    pub fn is_gsp_enabled() -> NvResult<bool> {
        // Check modprobe configuration
        if let Ok(content) = fs::read_to_string("/etc/modprobe.d/nvidia.conf") {
            if content.contains("NVreg_EnableGpuFirmware=1") {
                return Ok(true);
            }
            if content.contains("NVreg_EnableGpuFirmware=0") {
                return Ok(false);
            }
        }

        // Check via sysfs (runtime)
        if let Ok(content) =
            fs::read_to_string("/sys/module/nvidia/parameters/NVreg_EnableGpuFirmware")
        {
            return Ok(content.trim() == "1");
        }

        // Default: GSP is enabled with nvidia-open
        Ok(Self::is_nvidia_open())
    }

    /// Check if nvidia-open driver is in use
    fn is_nvidia_open() -> bool {
        // Check if nvidia-open package is installed
        Command::new("pacman")
            .args(&["-Q", "nvidia-open"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get GSP firmware information
    pub fn get_firmware_info(&self) -> NvResult<GspFirmwareInfo> {
        let enabled = Self::is_gsp_enabled()?;

        // Find firmware file
        let firmware_files = [
            self.firmware_dir.join("gsp.bin"),
            self.firmware_dir.join("gsp_tu10x.bin"),
            self.firmware_dir.join("gsp_ga10x.bin"),
            self.firmware_dir.join("gsp_ad10x.bin"), // RTX 40 series
        ];

        let firmware_path = firmware_files
            .iter()
            .find(|p| p.exists())
            .ok_or_else(|| NvControlError::ConfigError("GSP firmware not found".to_string()))?;

        let metadata = fs::metadata(firmware_path).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to read firmware metadata: {}", e))
        })?;

        let version = Self::get_firmware_version(firmware_path)?;

        Ok(GspFirmwareInfo {
            enabled,
            version,
            path: firmware_path.clone(),
            file_size: metadata.len(),
            checksum: Self::calculate_checksum(firmware_path).ok(),
        })
    }

    /// Get firmware version
    fn get_firmware_version(path: &Path) -> NvResult<String> {
        // Try to extract version from filename or file metadata
        if let Some(filename) = path.file_name() {
            if let Some(name) = filename.to_str() {
                // Version is often embedded in the filename
                return Ok(name.to_string());
            }
        }

        // Fallback: check driver version
        if let Ok(output) = Command::new("nvidia-smi")
            .arg("--query-gpu=driver_version")
            .arg("--format=csv,noheader")
            .output()
        {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }

        Ok("unknown".to_string())
    }

    /// Calculate firmware checksum
    fn calculate_checksum(path: &Path) -> NvResult<String> {
        let output = Command::new("sha256sum").arg(path).output().map_err(|e| {
            NvControlError::CommandFailed(format!("Failed to calculate checksum: {}", e))
        })?;

        let checksum = String::from_utf8_lossy(&output.stdout);
        let hash = checksum
            .split_whitespace()
            .next()
            .ok_or_else(|| NvControlError::ConfigError("Invalid checksum output".to_string()))?;

        Ok(hash.to_string())
    }

    /// Enable GSP firmware
    pub fn enable_gsp(&self) -> NvResult<()> {
        println!("✅ Enabling GSP firmware...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to enable GSP".to_string(),
            ));
        }

        self.set_gsp_parameter("1")?;

        println!("✅ GSP firmware enabled");
        println!("⚠️  Reboot required for changes to take effect");

        Ok(())
    }

    /// Disable GSP firmware (fallback mode)
    pub fn disable_gsp(&self) -> NvResult<()> {
        println!("⚠️  Disabling GSP firmware (fallback to legacy mode)...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to disable GSP".to_string(),
            ));
        }

        self.set_gsp_parameter("0")?;

        println!("✅ GSP firmware disabled");
        println!("⚠️  Reboot required for changes to take effect");

        Ok(())
    }

    /// Set GSP parameter in modprobe config
    fn set_gsp_parameter(&self, value: &str) -> NvResult<()> {
        let config_path = PathBuf::from("/etc/modprobe.d/nvidia.conf");

        let mut content = fs::read_to_string(&config_path).unwrap_or_else(|_| String::new());

        // Remove existing GSP parameter
        content = content
            .lines()
            .filter(|l| !l.contains("NVreg_EnableGpuFirmware"))
            .collect::<Vec<_>>()
            .join("\n");

        // Add new parameter
        content.push_str(&format!(
            "\noptions nvidia NVreg_EnableGpuFirmware={}\n",
            value
        ));

        fs::write(&config_path, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    /// Get GSP telemetry (if available)
    pub fn get_telemetry(&self) -> NvResult<GspTelemetry> {
        // Query GPU stats via nvidia-smi
        let output = Command::new("nvidia-smi")
            .args(&[
                "--query-gpu=power.draw,temperature.gpu,clocks.current.graphics",
                "--format=csv,noheader,nounits",
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to query GPU: {}", e)))?;

        let data = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = data.trim().split(',').collect();

        if parts.len() < 3 {
            return Err(NvControlError::GpuQueryFailed(
                "Invalid telemetry data".to_string(),
            ));
        }

        Ok(GspTelemetry {
            power_usage: parts[0].trim().parse().unwrap_or(0.0),
            temperature: parts[1].trim().parse().unwrap_or(0.0),
            clock_speed: parts[2].trim().parse().unwrap_or(0),
            errors: 0, // Would need to check dmesg for GSP errors
        })
    }

    /// Check GSP error log
    pub fn check_gsp_errors(&self) -> NvResult<Vec<String>> {
        let output = Command::new("dmesg")
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to read dmesg: {}", e)))?;

        let log = String::from_utf8_lossy(&output.stdout);

        let errors: Vec<String> = log
            .lines()
            .filter(|l| l.contains("nvidia") && (l.contains("GSP") || l.contains("firmware")))
            .filter(|l| l.contains("error") || l.contains("warn") || l.contains("fail"))
            .map(|s| s.to_string())
            .collect();

        Ok(errors)
    }

    /// Check if firmware update is available
    pub fn check_for_updates(&self) -> NvResult<bool> {
        println!("🔍 Checking for GSP firmware updates...");

        // Check if nvidia-open package has updates
        let output = Command::new("checkupdates").output();

        if let Ok(out) = output {
            let updates = String::from_utf8_lossy(&out.stdout);
            if updates.contains("nvidia-open") || updates.contains("nvidia-utils") {
                println!("   ✅ Update available!");
                return Ok(true);
            }
        }

        println!("   ℹ️  No updates available");
        Ok(false)
    }

    /// Update firmware (via package manager)
    pub fn update_firmware(&self) -> NvResult<()> {
        println!("📦 Updating NVIDIA packages...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to update packages".to_string(),
            ));
        }

        let status = Command::new("pacman")
            .args(&["-Syu", "--noconfirm", "nvidia-open", "nvidia-utils"])
            .status()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("Failed to update packages: {}", e))
            })?;

        if !status.success() {
            return Err(NvControlError::CommandFailed(
                "Package update failed".to_string(),
            ));
        }

        println!("✅ Firmware updated successfully");
        println!("⚠️  Reboot recommended");

        Ok(())
    }

    /// Run GSP diagnostics
    pub fn run_diagnostics(&self) -> NvResult<()> {
        println!("🔬 Running GSP Firmware Diagnostics\n");

        // Check if GSP is enabled
        let enabled = Self::is_gsp_enabled()?;
        println!(
            "GSP Status: {}",
            if enabled {
                "✅ Enabled"
            } else {
                "❌ Disabled"
            }
        );

        // Get firmware info
        if let Ok(info) = self.get_firmware_info() {
            println!("\nFirmware Information:");
            println!("   Version: {}", info.version);
            println!("   Path: {}", info.path.display());
            println!("   Size: {} bytes", info.file_size);
            if let Some(checksum) = info.checksum {
                println!("   SHA256: {}...", &checksum[..16]);
            }
        } else {
            println!("\n⚠️  Firmware file not found!");
        }

        // Get telemetry
        if let Ok(telemetry) = self.get_telemetry() {
            println!("\nCurrent Telemetry:");
            println!("   Power: {:.1} W", telemetry.power_usage);
            println!("   Temperature: {:.0} °C", telemetry.temperature);
            println!("   Clock Speed: {} MHz", telemetry.clock_speed);
        }

        // Check for errors
        let errors = self.check_gsp_errors()?;
        if errors.is_empty() {
            println!("\n✅ No GSP errors found in kernel log");
        } else {
            println!("\n⚠️  GSP Errors Found:");
            for (i, error) in errors.iter().take(5).enumerate() {
                println!("   {}. {}", i + 1, error);
            }
            if errors.len() > 5 {
                println!("   ... and {} more", errors.len() - 5);
            }
        }

        Ok(())
    }

    fn is_root(&self) -> bool {
        unsafe { libc::geteuid() == 0 }
    }

    /// Print GSP status
    pub fn print_status(&self) -> NvResult<()> {
        println!("🔧 NVIDIA GSP Firmware Status\n");

        let enabled = Self::is_gsp_enabled()?;
        println!("GSP Enabled: {}", if enabled { "✅ Yes" } else { "❌ No" });

        if !Self::is_nvidia_open() {
            println!("\n⚠️  nvidia-open driver not detected");
            println!("   GSP firmware is only available with nvidia-open");
            return Ok(());
        }

        if let Ok(info) = self.get_firmware_info() {
            println!("\nFirmware:");
            println!("   Version: {}", info.version);
            println!("   Location: {}", info.path.display());
            println!("   Size: {:.2} MB", info.file_size as f64 / 1024.0 / 1024.0);
        }

        println!("\nBenefits of GSP Firmware:");
        println!("   • Offloads GPU management to GPU");
        println!("   • Reduces CPU overhead");
        println!("   • Required for advanced features");
        println!("   • Better power management");

        if !enabled {
            println!("\n💡 Tip: Enable GSP with 'nvctl gsp enable'");
        }

        Ok(())
    }
}
