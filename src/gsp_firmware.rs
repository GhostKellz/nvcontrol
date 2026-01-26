// NVIDIA GSP Firmware Manager (nvidia-open specific)
// Monitor, control, and update GSP firmware
//
// GSP (GPU System Processor) offloads GPU management from CPU to a dedicated
// RISC-V processor on the GPU. Required for nvidia-open driver on Turing+.

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ==================== Data Structures ====================

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

/// Comprehensive GSP status for deep diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GspStatus {
    /// Whether GSP is configured to be enabled (modprobe/sysfs)
    pub enabled: bool,
    /// GSP state: "active", "loaded", "not_loaded", "unknown"
    pub state: String,
    /// Firmware version (from driver or firmware file)
    pub firmware_version: Option<String>,
    /// Path to firmware file(s)
    pub firmware_path: Option<String>,
    /// GSP init result from dmesg
    pub init_result: Option<String>,
    /// Init timestamp from dmesg
    pub init_timestamp: Option<String>,
    /// Count of GSP-RM warnings/errors since boot
    pub error_count: u32,
    /// Count of GSP-RM messages total
    pub message_count: u32,
    /// Detected GPU architecture (for firmware selection)
    pub gpu_arch: Option<String>,
    /// Whether nvidia-open driver is in use
    pub is_nvidia_open: bool,
}

/// GSP health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GspHealthCheck {
    pub passed: bool,
    pub checks: Vec<GspCheck>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GspCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

// ==================== GspManager ====================

pub struct GspManager {
    firmware_dir: PathBuf,
}

impl Default for GspManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GspManager {
    pub fn new() -> Self {
        Self {
            firmware_dir: PathBuf::from("/lib/firmware/nvidia"),
        }
    }

    // ==================== Core Detection ====================

    /// Check if GSP firmware is enabled (best-effort, multiple sources)
    pub fn is_gsp_enabled() -> NvResult<bool> {
        // 1. Check sysfs first (runtime state, most accurate)
        if let Ok(content) =
            fs::read_to_string("/sys/module/nvidia/parameters/NVreg_EnableGpuFirmware")
        {
            let val = content.trim();
            if val == "1" || val == "Y" {
                return Ok(true);
            }
            if val == "0" || val == "N" {
                return Ok(false);
            }
        }

        // 2. Check modprobe configuration (boot-time intent)
        for conf_file in &[
            "/etc/modprobe.d/nvidia.conf",
            "/etc/modprobe.d/nvidia-gsp.conf",
            "/etc/modprobe.d/zz-nvidia.conf",
        ] {
            if let Ok(content) = fs::read_to_string(conf_file) {
                if content.contains("NVreg_EnableGpuFirmware=1") {
                    return Ok(true);
                }
                if content.contains("NVreg_EnableGpuFirmware=0") {
                    return Ok(false);
                }
            }
        }

        // 3. Default: GSP is enabled by default with nvidia-open on Turing+
        Ok(Self::is_nvidia_open())
    }

    /// Check if nvidia-open driver is in use
    pub fn is_nvidia_open() -> bool {
        // Primary detection: /proc/driver/nvidia/version
        if let Ok(version_info) = fs::read_to_string("/proc/driver/nvidia/version") {
            if version_info.contains("Open Kernel Module") {
                return true;
            }
        }

        // Secondary: modinfo license check (Dual MIT/GPL = open)
        if let Ok(output) = Command::new("modinfo")
            .args(["-F", "license", "nvidia"])
            .output()
        {
            if output.status.success() {
                let license = String::from_utf8_lossy(&output.stdout);
                if license.contains("MIT") || license.contains("GPL") {
                    return true;
                }
            }
        }

        false
    }

    /// Detect GPU architecture code (e.g., "gb202", "ad102", "ga102")
    pub fn detect_gpu_arch() -> Option<String> {
        // Get GPU name from nvidia-smi
        let output = Command::new("nvidia-smi")
            .args(["--query-gpu=name", "--format=csv,noheader"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let gpu_name = String::from_utf8_lossy(&output.stdout).to_lowercase();

        // Map GPU series to architecture code
        // Blackwell (RTX 50 series)
        if gpu_name.contains("5090") || gpu_name.contains("5080") {
            return Some("gb202".to_string());
        }
        if gpu_name.contains("5070") || gpu_name.contains("5060") {
            return Some("gb205".to_string());
        }

        // Ada Lovelace (RTX 40 series)
        if gpu_name.contains("4090") || gpu_name.contains("4080") {
            return Some("ad102".to_string());
        }
        if gpu_name.contains("4070") || gpu_name.contains("4060") {
            return Some("ad104".to_string());
        }

        // Ampere (RTX 30 series)
        if gpu_name.contains("3090") || gpu_name.contains("3080") || gpu_name.contains("3070") {
            return Some("ga102".to_string());
        }
        if gpu_name.contains("3060") || gpu_name.contains("3050") {
            return Some("ga106".to_string());
        }

        // Turing (RTX 20 / GTX 16 series)
        if gpu_name.contains("2080") || gpu_name.contains("2070") {
            return Some("tu102".to_string());
        }
        if gpu_name.contains("2060") || gpu_name.contains("1660") || gpu_name.contains("1650") {
            return Some("tu106".to_string());
        }

        None
    }

    // ==================== Deep GSP Status ====================

    /// Get comprehensive GSP status (for `nvctl driver gsp status`)
    pub fn get_deep_status(&self) -> GspStatus {
        let enabled = Self::is_gsp_enabled().unwrap_or(false);
        let is_nvidia_open = Self::is_nvidia_open();
        let gpu_arch = Self::detect_gpu_arch();

        // Determine GSP state from multiple sources
        let (state, init_result, init_timestamp, error_count, message_count) =
            self.parse_gsp_dmesg();

        // Find firmware path
        let firmware_path = self.find_firmware_path(&gpu_arch);
        let firmware_version = self.get_driver_version();

        GspStatus {
            enabled,
            state,
            firmware_version,
            firmware_path,
            init_result,
            init_timestamp,
            error_count,
            message_count,
            gpu_arch,
            is_nvidia_open,
        }
    }

    /// Parse dmesg/journalctl for GSP init messages
    fn parse_gsp_dmesg(&self) -> (String, Option<String>, Option<String>, u32, u32) {
        let mut state = "unknown".to_string();
        let mut init_result = None;
        let mut init_timestamp = None;
        let mut error_count = 0u32;
        let mut message_count = 0u32;

        // Try journalctl first (more reliable, doesn't need root)
        let dmesg_output = Command::new("journalctl")
            .args(["-k", "-g", "nvidia|NVRM|GSP", "--no-pager", "-q"])
            .output()
            .or_else(|_| Command::new("dmesg").output());

        if let Ok(output) = dmesg_output {
            let log = String::from_utf8_lossy(&output.stdout);

            for line in log.lines() {
                let line_lower = line.to_lowercase();

                // Count GSP-related messages
                if line_lower.contains("gsp") {
                    message_count += 1;
                }

                // Check for GSP init success
                if line_lower.contains("gsp") && line_lower.contains("init") {
                    if line_lower.contains("success") || line_lower.contains("complete") {
                        state = "active".to_string();
                        init_result = Some("init ok".to_string());
                    } else if line_lower.contains("fail") || line_lower.contains("error") {
                        state = "failed".to_string();
                        init_result = Some("init failed".to_string());
                        error_count += 1;
                    }

                    // Extract timestamp (format varies by system)
                    if let Some(ts) = Self::extract_timestamp(line) {
                        init_timestamp = Some(ts);
                    }
                }

                // Count errors/warnings
                if line_lower.contains("gsp")
                    && (line_lower.contains("error")
                        || line_lower.contains("warn")
                        || line_lower.contains("fail")
                        || line_lower.contains("timeout"))
                {
                    error_count += 1;
                }
            }

            // If we found GSP messages but no explicit init, assume loaded
            if state == "unknown" && message_count > 0 {
                state = "loaded".to_string();
            }
        }

        // Fallback: check if nvidia module is loaded
        if state == "unknown" {
            if let Ok(output) = Command::new("lsmod").output() {
                let modules = String::from_utf8_lossy(&output.stdout);
                if modules.contains("nvidia") {
                    state = "loaded".to_string();
                } else {
                    state = "not_loaded".to_string();
                }
            }
        }

        (
            state,
            init_result,
            init_timestamp,
            error_count,
            message_count,
        )
    }

    /// Extract timestamp from dmesg line
    fn extract_timestamp(line: &str) -> Option<String> {
        // dmesg format: "[   12.345678] ..." - check this first
        if line.starts_with('[') {
            if let Some(end) = line.find(']') {
                return Some(line[1..end].trim().to_string());
            }
        }

        // journalctl format: "Dec 19 15:30:42 hostname kernel: ..."
        if line.len() > 15 {
            let parts: Vec<&str> = line.splitn(4, ' ').collect();
            // First part should be a month abbreviation (3 letters)
            if parts.len() >= 3
                && parts[0].len() == 3
                && parts[0].chars().all(|c| c.is_alphabetic())
            {
                return Some(format!("{} {} {}", parts[0], parts[1], parts[2]));
            }
        }

        None
    }

    /// Find firmware path (supports new per-chip layout)
    fn find_firmware_path(&self, gpu_arch: &Option<String>) -> Option<String> {
        let driver_version = self.get_driver_version()?;

        // New layout: /lib/firmware/nvidia/<arch>/gsp/gsp-<ver>.bin.zst
        if let Some(arch) = gpu_arch {
            let new_path = self.firmware_dir.join(arch).join("gsp");
            if new_path.exists() {
                return Some(new_path.display().to_string());
            }
        }

        // Legacy layout: /lib/firmware/nvidia/<version>/gsp_*.bin
        let legacy_path = self.firmware_dir.join(&driver_version);
        if legacy_path.exists() {
            // Find actual firmware file
            if let Ok(entries) = fs::read_dir(&legacy_path) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with("gsp_") && name_str.ends_with(".bin") {
                        return Some(entry.path().display().to_string());
                    }
                }
            }
            return Some(legacy_path.display().to_string());
        }

        None
    }

    /// Get driver version from nvidia-smi
    fn get_driver_version(&self) -> Option<String> {
        let output = Command::new("nvidia-smi")
            .args(["--query-gpu=driver_version", "--format=csv,noheader"])
            .output()
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }

    // ==================== Health Checks ====================

    /// Run GSP health checks (for `nvctl driver check` GSP section)
    pub fn run_health_checks(&self) -> GspHealthCheck {
        let mut checks = Vec::new();
        let mut recommendations = Vec::new();
        let mut all_passed = true;

        let is_open = Self::is_nvidia_open();
        let enabled = Self::is_gsp_enabled().unwrap_or(false);
        let status = self.get_deep_status();

        // Check 1: nvidia-open driver
        checks.push(GspCheck {
            name: "nvidia-open driver".to_string(),
            passed: is_open,
            message: if is_open {
                "Using nvidia-open kernel module".to_string()
            } else {
                "Using proprietary driver (GSP optional)".to_string()
            },
        });

        if !is_open {
            recommendations.push("GSP is only available with nvidia-open driver".to_string());
        }

        // Check 2: GSP enabled
        if is_open {
            checks.push(GspCheck {
                name: "GSP enabled".to_string(),
                passed: enabled,
                message: if enabled {
                    "GSP firmware is enabled".to_string()
                } else {
                    "GSP firmware is disabled".to_string()
                },
            });

            if !enabled {
                all_passed = false;
                recommendations
                    .push("Enable GSP with: nvctl driver gsp enable (then reboot)".to_string());
            }
        }

        // Check 3: GSP init status
        if enabled && is_open {
            let init_ok = status.state == "active" || status.state == "loaded";
            checks.push(GspCheck {
                name: "GSP initialization".to_string(),
                passed: init_ok,
                message: match status.state.as_str() {
                    "active" => "GSP initialized successfully".to_string(),
                    "loaded" => "GSP firmware loaded".to_string(),
                    "failed" => "GSP initialization failed".to_string(),
                    "not_loaded" => "nvidia module not loaded".to_string(),
                    _ => format!("GSP state: {}", status.state),
                },
            });

            if !init_ok {
                all_passed = false;
                recommendations.push("Check logs: nvctl driver logs --gsp".to_string());
            }
        }

        // Check 4: GSP errors
        if status.error_count > 0 {
            checks.push(GspCheck {
                name: "GSP errors".to_string(),
                passed: false,
                message: format!("{} GSP errors/warnings in kernel log", status.error_count),
            });
            all_passed = false;
            recommendations.push("Review GSP errors: nvctl driver logs --gsp".to_string());
        } else if status.message_count > 0 {
            checks.push(GspCheck {
                name: "GSP errors".to_string(),
                passed: true,
                message: "No GSP errors in kernel log".to_string(),
            });
        }

        // Check 5: Firmware path exists
        if is_open {
            let fw_exists = status.firmware_path.is_some();
            checks.push(GspCheck {
                name: "GSP firmware files".to_string(),
                passed: fw_exists,
                message: if let Some(ref path) = status.firmware_path {
                    format!("Found at {}", path)
                } else {
                    "Firmware not found".to_string()
                },
            });

            if !fw_exists {
                all_passed = false;
                recommendations
                    .push("Reinstall nvidia-open package to restore firmware".to_string());
            }
        }

        GspHealthCheck {
            passed: all_passed,
            checks,
            recommendations,
        }
    }

    // ==================== Legacy Compatibility ====================

    /// Get GSP firmware information (legacy API)
    pub fn get_firmware_info(&self) -> NvResult<GspFirmwareInfo> {
        let enabled = Self::is_gsp_enabled()?;
        let gpu_arch = Self::detect_gpu_arch();

        // Try new layout first
        if let Some(ref arch) = gpu_arch {
            let arch_path = self.firmware_dir.join(arch).join("gsp");
            if arch_path.exists() {
                if let Ok(entries) = fs::read_dir(&arch_path) {
                    for entry in entries.flatten() {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        if name_str.contains("gsp") && name_str.ends_with(".bin.zst") {
                            let metadata = fs::metadata(entry.path()).ok();
                            return Ok(GspFirmwareInfo {
                                enabled,
                                version: self
                                    .get_driver_version()
                                    .unwrap_or_else(|| "unknown".to_string()),
                                path: entry.path(),
                                file_size: metadata.map(|m| m.len()).unwrap_or(0),
                                checksum: None, // Skip for zst files
                            });
                        }
                    }
                }
            }
        }

        // Try legacy layout
        if let Some(version) = self.get_driver_version() {
            let version_path = self.firmware_dir.join(&version);
            let firmware_files = [
                version_path.join("gsp_ga10x.bin"),
                version_path.join("gsp_tu10x.bin"),
                version_path.join("gsp_ad10x.bin"),
                version_path.join("gsp.bin"),
            ];

            for fw_path in &firmware_files {
                if fw_path.exists() {
                    let metadata = fs::metadata(fw_path).map_err(|e| {
                        NvControlError::ConfigError(format!("Failed to read firmware: {}", e))
                    })?;

                    return Ok(GspFirmwareInfo {
                        enabled,
                        version,
                        path: fw_path.clone(),
                        file_size: metadata.len(),
                        checksum: Self::calculate_checksum(fw_path).ok(),
                    });
                }
            }
        }

        Err(NvControlError::ConfigError(
            "GSP firmware not found".to_string(),
        ))
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

    // ==================== GSP Control ====================

    /// Enable GSP firmware
    pub fn enable_gsp(&self) -> NvResult<()> {
        println!("Enabling GSP firmware...");

        if !self.is_root() {
            println!("\nTo enable GSP, add to /etc/modprobe.d/nvidia.conf:");
            println!("  options nvidia NVreg_EnableGpuFirmware=1");
            println!("\nThen rebuild initramfs and reboot:");
            println!("  sudo mkinitcpio -P  # Arch");
            println!("  sudo update-initramfs -u  # Debian/Ubuntu");
            return Ok(());
        }

        self.set_gsp_parameter("1")?;
        self.rebuild_initramfs();

        println!("\nGSP firmware enabled");
        println!("Reboot required for changes to take effect");

        Ok(())
    }

    /// Disable GSP firmware (fallback mode)
    pub fn disable_gsp(&self) -> NvResult<()> {
        println!("Disabling GSP firmware (fallback to legacy mode)...");

        if !self.is_root() {
            println!("\nTo disable GSP, add to /etc/modprobe.d/nvidia.conf:");
            println!("  options nvidia NVreg_EnableGpuFirmware=0");
            println!("\nThen rebuild initramfs and reboot");
            return Ok(());
        }

        self.set_gsp_parameter("0")?;
        self.rebuild_initramfs();

        println!("\nGSP firmware disabled");
        println!("Reboot required for changes to take effect");

        Ok(())
    }

    /// Set GSP parameter in modprobe config
    fn set_gsp_parameter(&self, value: &str) -> NvResult<()> {
        let config_path = PathBuf::from("/etc/modprobe.d/nvidia.conf");

        let mut content = fs::read_to_string(&config_path).unwrap_or_else(|_| String::new());

        // Remove existing GSP parameter lines
        content = content
            .lines()
            .filter(|l| !l.contains("NVreg_EnableGpuFirmware"))
            .collect::<Vec<_>>()
            .join("\n");

        // Add new parameter
        if !content.ends_with('\n') && !content.is_empty() {
            content.push('\n');
        }
        content.push_str(&format!(
            "options nvidia NVreg_EnableGpuFirmware={}\n",
            value
        ));

        fs::write(&config_path, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    /// Rebuild initramfs (best effort)
    fn rebuild_initramfs(&self) {
        // Try Arch
        if Command::new("mkinitcpio").args(["-P"]).status().is_ok() {
            println!("Initramfs rebuilt (mkinitcpio)");
            return;
        }

        // Try Debian/Ubuntu
        if Command::new("update-initramfs")
            .args(["-u"])
            .status()
            .is_ok()
        {
            println!("Initramfs rebuilt (update-initramfs)");
            return;
        }

        // Try Fedora
        if Command::new("dracut").args(["--force"]).status().is_ok() {
            println!("Initramfs rebuilt (dracut)");
            return;
        }

        println!("Note: Please rebuild initramfs manually");
    }

    // ==================== Telemetry ====================

    /// Get GSP telemetry (if available)
    pub fn get_telemetry(&self) -> NvResult<GspTelemetry> {
        let output = Command::new("nvidia-smi")
            .args([
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
            errors: 0,
        })
    }

    /// Check GSP error log
    pub fn check_gsp_errors(&self) -> NvResult<Vec<String>> {
        // Try journalctl first (doesn't need root)
        let output = Command::new("journalctl")
            .args(["-k", "-g", "GSP|gsp", "--no-pager", "-q"])
            .output()
            .or_else(|_| Command::new("dmesg").output())
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to read logs: {}", e)))?;

        let log = String::from_utf8_lossy(&output.stdout);

        let errors: Vec<String> = log
            .lines()
            .filter(|l| {
                let lower = l.to_lowercase();
                (lower.contains("gsp") || lower.contains("nvidia"))
                    && (lower.contains("error")
                        || lower.contains("warn")
                        || lower.contains("fail")
                        || lower.contains("timeout"))
            })
            .map(|s| s.to_string())
            .collect();

        Ok(errors)
    }

    // ==================== Update Management ====================

    /// Check if firmware update is available
    pub fn check_for_updates(&self) -> NvResult<bool> {
        println!("Checking for GSP firmware updates...");

        let output = Command::new("checkupdates").output();

        if let Ok(out) = output {
            let updates = String::from_utf8_lossy(&out.stdout);
            if updates.contains("nvidia-open") || updates.contains("nvidia-utils") {
                println!("  Update available!");
                return Ok(true);
            }
        }

        println!("  No updates available");
        Ok(false)
    }

    /// Update firmware (via package manager)
    pub fn update_firmware(&self) -> NvResult<()> {
        println!("Updating NVIDIA packages...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to update packages".to_string(),
            ));
        }

        let status = Command::new("pacman")
            .args(["-Syu", "--noconfirm", "nvidia-open", "nvidia-utils"])
            .status()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("Failed to update packages: {}", e))
            })?;

        if !status.success() {
            return Err(NvControlError::CommandFailed(
                "Package update failed".to_string(),
            ));
        }

        println!("Firmware updated successfully");
        println!("Reboot recommended");

        Ok(())
    }

    // ==================== Diagnostics ====================

    /// Run GSP diagnostics (legacy)
    pub fn run_diagnostics(&self) -> NvResult<()> {
        println!("Running GSP Firmware Diagnostics\n");

        let status = self.get_deep_status();

        // Basic info
        println!(
            "GSP Mode:       {}",
            if status.enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
        println!("GSP State:      {}", status.state);
        println!(
            "Driver Type:    {}",
            if status.is_nvidia_open {
                "nvidia-open"
            } else {
                "proprietary"
            }
        );

        if let Some(ref arch) = status.gpu_arch {
            println!("GPU Arch:       {}", arch);
        }

        if let Some(ref ver) = status.firmware_version {
            println!("Driver Version: {}", ver);
        }

        if let Some(ref path) = status.firmware_path {
            println!("Firmware Path:  {}", path);
        }

        println!();

        // Init info
        if let Some(ref result) = status.init_result {
            println!("Init Result:    {}", result);
        }
        if let Some(ref ts) = status.init_timestamp {
            println!("Init Time:      {}", ts);
        }

        // Error counts
        println!();
        println!("GSP Messages:   {}", status.message_count);
        println!("GSP Errors:     {}", status.error_count);

        // Telemetry
        if let Ok(telemetry) = self.get_telemetry() {
            println!();
            println!("Current Telemetry:");
            println!("  Power:        {:.1} W", telemetry.power_usage);
            println!("  Temperature:  {:.0} C", telemetry.temperature);
            println!("  Clock:        {} MHz", telemetry.clock_speed);
        }

        // Errors
        if let Ok(errors) = self.check_gsp_errors() {
            if errors.is_empty() {
                println!("\nNo GSP errors found in kernel log");
            } else {
                println!("\nGSP Errors Found:");
                for (i, error) in errors.iter().take(5).enumerate() {
                    println!("  {}. {}", i + 1, error);
                }
                if errors.len() > 5 {
                    println!("  ... and {} more", errors.len() - 5);
                }
            }
        }

        Ok(())
    }

    // ==================== Print Methods ====================

    /// Print GSP status (simple view)
    pub fn print_status(&self) -> NvResult<()> {
        println!("NVIDIA GSP Firmware Status\n");

        let status = self.get_deep_status();

        println!(
            "GSP Enabled:    {}",
            if status.enabled { "Yes" } else { "No" }
        );
        println!("GSP State:      {}", status.state);

        if !status.is_nvidia_open {
            println!("\nnvidia-open driver not detected");
            println!("GSP firmware is only available with nvidia-open");
            return Ok(());
        }

        if let Some(ref ver) = status.firmware_version {
            println!("Version:        {}", ver);
        }

        if let Some(ref path) = status.firmware_path {
            println!("Firmware:       {}", path);
        }

        if let Some(ref arch) = status.gpu_arch {
            println!("GPU Arch:       {}", arch);
        }

        if status.error_count > 0 {
            println!("\nWarning: {} GSP errors in kernel log", status.error_count);
            println!("Run 'nvctl driver logs --gsp' for details");
        }

        Ok(())
    }

    /// Print GSP explanation (what it is and why it matters)
    pub fn print_explain() {
        println!("NVIDIA GSP Firmware Explained");
        println!("{}", "=".repeat(50));
        println!();

        println!("What is GSP?");
        println!("  GSP (GPU System Processor) is a dedicated RISC-V processor");
        println!("  embedded in NVIDIA GPUs (Turing and newer). It handles GPU");
        println!("  management tasks like power, thermals, and initialization.");
        println!();

        println!("Why does it matter?");
        println!("  - Required for nvidia-open driver on Turing+ GPUs");
        println!("  - Offloads management from CPU to GPU (lower latency)");
        println!("  - Enables advanced power management features");
        println!("  - Required for full feature support on RTX 40/50 series");
        println!();

        println!("Common issues:");
        println!("  1. GSP init fails after kernel update");
        println!("     -> Rebuild DKMS: nvctl driver dkms fix");
        println!();
        println!("  2. GSP errors on resume from suspend");
        println!("     -> Check NVreg_PreserveVideoMemoryAllocations=1");
        println!();
        println!("  3. GSP firmware load timeout");
        println!("     -> Usually kernel/driver version mismatch");
        println!("     -> Ensure DKMS rebuilt for current kernel");
        println!();
        println!("  4. Performance issues with GSP enabled");
        println!("     -> Try disabling: nvctl driver gsp disable");
        println!("     -> (Not recommended for RTX 40/50 series)");
        println!();

        println!("Relevant logs:");
        println!("  nvctl driver logs --gsp     # GSP-specific messages");
        println!("  nvctl driver logs --nvidia  # All NVIDIA messages");
        println!("  nvctl driver logs --xid     # Xid errors (GPU faults)");
        println!();

        println!("Learn more:");
        println!("  https://wiki.archlinux.org/title/NVIDIA#GSP_Firmware");
        println!("  https://github.com/NVIDIA/open-gpu-kernel-modules");
    }

    fn is_root(&self) -> bool {
        nix::unistd::geteuid().is_root()
    }
}

// ==================== Tests ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gsp_manager_creation() {
        let manager = GspManager::new();
        assert_eq!(manager.firmware_dir, PathBuf::from("/lib/firmware/nvidia"));
    }

    #[test]
    fn test_timestamp_extraction() {
        // journalctl format
        let ts = GspManager::extract_timestamp("Dec 19 15:30:42 myhost kernel: nvidia");
        assert_eq!(ts, Some("Dec 19 15:30:42".to_string()));

        // dmesg format
        let ts = GspManager::extract_timestamp("[   12.345678] nvidia: GSP init");
        assert_eq!(ts, Some("12.345678".to_string()));

        // No timestamp
        let ts = GspManager::extract_timestamp("random log line");
        assert!(ts.is_none());
    }

    #[test]
    fn test_deep_status_structure() {
        let manager = GspManager::new();
        let status = manager.get_deep_status();

        // Should have valid structure regardless of whether nvidia is installed
        assert!(!status.state.is_empty());
    }

    #[test]
    fn test_health_checks_structure() {
        let manager = GspManager::new();
        let health = manager.run_health_checks();

        // Should have at least one check
        assert!(!health.checks.is_empty());
    }
}
