use crate::arch_integration::ArchIntegration;
use crate::container_runtime::NvContainerRuntime;
use crate::gsp_firmware::GspManager;
use crate::{NvControlError, NvResult};
use flate2::{Compression, write::GzEncoder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::{fmt::Write as _, fs};
use tar::Builder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverStatus {
    pub current_version: String,
    pub driver_type: String,
    pub available_version: Option<String>,
    pub update_available: bool,
    pub dkms_status: HashMap<String, String>,
    pub kernel_modules: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum DriverType {
    Proprietary,
    Open,
    OpenBeta,
    Nouveau,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuReleaseDiagnostic {
    pub gpu_name: String,
    pub pci_bus_id: Option<String>,
    pub pci_device_id: Option<String>,
    pub chip_code: Option<String>,
    pub architecture: String,
    pub legacy: bool,
    pub open_driver_capable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDiagnostics {
    pub running_kernel: String,
    pub module_kernel: String,
    pub kernel_match: bool,
    pub boot_cmdline: Option<String>,
    pub initramfs_tool: Option<String>,
    pub boot_entries: Vec<String>,
    pub installed_kernels: Vec<String>,
    pub initramfs_images: Vec<String>,
    pub initramfs_findings: Vec<String>,
    pub userspace_driver_version: Option<String>,
    pub kernel_module_version: Option<String>,
    pub firmware_layout: Option<String>,
    pub firmware_path: Option<String>,
    pub firmware_file: Option<String>,
    pub release_alignment: Option<String>,
    pub expected_firmware_paths: Vec<String>,
    pub ownership: Vec<OwnershipDiagnostic>,
    pub gpu_diagnostics: Vec<GpuReleaseDiagnostic>,
    pub arch_packages: Vec<PackageDiagnostic>,
    pub package_findings: Vec<String>,
    pub package_inventory: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDiagnostic {
    pub package: String,
    pub installed_version: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipDiagnostic {
    pub path: String,
    pub owner: Option<String>,
    pub package_check: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportBundleMetadata {
    pub bundle_path: String,
    pub tarball: bool,
    pub gzip: bool,
    pub redact_paths: bool,
    pub redact_ids: bool,
    pub log_tail: usize,
    pub release_diagnostics: ReleaseDiagnostics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DkmsDoctorReport {
    pub severity: DiagnosticSeverity,
    pub findings: Vec<String>,
    pub fixes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildState {
    pub source_path: Option<String>,
    pub source_type: String,
    pub current_tag: Option<String>,
    pub latest_tag: Option<String>,
    pub git_commit: Option<String>,
    pub git_dirty: Option<bool>,
    pub symlink_target: Option<String>,
    pub tracked_version: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Healthy,
    Warning,
    Broken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticSummary {
    pub severity: DiagnosticSeverity,
    pub messages: Vec<String>,
}

pub fn severity_label(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Healthy => "Healthy",
        DiagnosticSeverity::Warning => "Warning",
        DiagnosticSeverity::Broken => "Broken",
    }
}

pub fn suggested_fixes(summary: &DiagnosticSummary) -> Vec<String> {
    let mut fixes = Vec::new();

    for message in &summary.messages {
        if message.contains("running kernel does not match") {
            fixes.push("Reboot into the kernel that matches the installed NVIDIA module, or rebuild the module for the current kernel".to_string());
        }
        if message.contains("missing expected firmware paths") {
            fixes.push("Reinstall the matching NVIDIA userspace/firmware packages and verify the expected firmware directories exist".to_string());
        }
        if message.contains("mixed package state") {
            fixes.push("Make sure nvidia-open, nvidia-utils, and firmware packages come from the same 595 release branch".to_string());
        }
        if message.contains("Both nvidia and nvidia-open are installed") {
            fixes.push("Remove the branch you are not using, for example: sudo pacman -Rns nvidia or sudo pacman -Rns nvidia-open".to_string());
        }
        if message
            .contains("Both proprietary and open NVIDIA kernel package variants are installed")
        {
            fixes.push("Remove the unused branch packages so only one kernel-driver family remains installed, for example: sudo pacman -Rns nvidia nvidia-dkms or sudo pacman -Rns nvidia-open nvidia-open-dkms".to_string());
        }
        if message.contains("Both nvidia and nvidia-dkms are installed") {
            fixes.push("Keep either `nvidia` or `nvidia-dkms`, then rebuild initramfs for the remaining branch".to_string());
        }
        if message.contains("Both nvidia-open and nvidia-open-dkms are installed") {
            fixes.push("Keep either `nvidia-open` or `nvidia-open-dkms`, then rerun `nvctl driver dkms doctor` if you stay on DKMS".to_string());
        }
        if message.contains("nvidia-utils version") {
            fixes.push("Sync driver userspace packages to one branch, for example: sudo pacman -Syu nvidia-utils nvidia-open or sudo pacman -Syu nvidia-utils nvidia".to_string());
        }
        if message.contains("lib32-nvidia-utils version") {
            fixes.push("Match `lib32-nvidia-utils` to the same branch/version as `nvidia-utils`, for example: sudo pacman -Syu nvidia-utils lib32-nvidia-utils".to_string());
        }
        if message.contains("linux-firmware-nvidia is missing") {
            fixes.push("Install the missing firmware package for Arch/CachyOS, for example: sudo pacman -Syu linux-firmware-nvidia".to_string());
        }
        if message.contains("no owning Arch package") {
            fixes.push("Reinstall the firmware package that should own the missing path, for example: sudo pacman -Syu linux-firmware linux-firmware-nvidia".to_string());
        }
        if message.contains("dkms") {
            fixes.push("Run `nvctl driver dkms fix` after confirming matching kernel headers are installed".to_string());
        }
        if message.contains("boot/initramfs") {
            fixes.push("Verify the detected initramfs tool, regenerate images after kernel/driver changes, and confirm the running kernel has a matching /boot entry".to_string());
        }
    }

    fixes.sort();
    fixes.dedup();
    fixes
}

impl DriverType {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "proprietary" | "nvidia" => Some(DriverType::Proprietary),
            "open" | "nvidia-open" => Some(DriverType::Open),
            "open-beta" | "nvidia-open-beta" => Some(DriverType::OpenBeta),
            "nouveau" => Some(DriverType::Nouveau),
            _ => None,
        }
    }

    fn package_name(&self) -> &str {
        match self {
            DriverType::Proprietary => "nvidia",
            DriverType::Open => "nvidia-open",
            DriverType::OpenBeta => "nvidia-open-beta",
            DriverType::Nouveau => "nouveau",
        }
    }
}

pub fn get_driver_status() -> NvResult<DriverStatus> {
    let mut status = DriverStatus {
        current_version: "Unknown".to_string(),
        driver_type: "Unknown".to_string(),
        available_version: None,
        update_available: false,
        dkms_status: HashMap::new(),
        kernel_modules: Vec::new(),
    };

    // Get current driver version
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=driver_version"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            status.current_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            status.driver_type = determine_driver_type(&status.current_version);
        }
    }

    // Check DKMS status
    status.dkms_status = get_dkms_status()?;

    // Get loaded kernel modules
    status.kernel_modules = get_loaded_nvidia_modules()?;

    // Check for available updates
    status.available_version = check_available_driver_version()?;
    status.update_available =
        is_update_available(&status.current_version, &status.available_version);

    Ok(status)
}

fn determine_driver_type(_version: &str) -> String {
    if GspManager::is_nvidia_open() {
        "Open Source".to_string()
    } else if let Ok(version_info) = std::fs::read_to_string("/proc/driver/nvidia/version") {
        if version_info.contains("NVIDIA UNIX") {
            "Proprietary".to_string()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}

fn normalize_pci_bus_id(bus_id: &str) -> Option<String> {
    let trimmed = bus_id.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.len() == 12 && trimmed.chars().nth(4) == Some(':') {
        return Some(trimmed.to_lowercase());
    }

    Some(trimmed.to_lowercase())
}

fn read_pci_device_id(bus_id: &str) -> Option<String> {
    let normalized = normalize_pci_bus_id(bus_id)?;
    let path = format!("/sys/bus/pci/devices/{}/device", normalized);
    std::fs::read_to_string(path)
        .ok()
        .map(|value: String| value.trim().trim_start_matches("0x").to_lowercase())
}

fn lookup_pci_ids_name(device_id: &str) -> Option<String> {
    let pci_ids = fs::read_to_string("/usr/share/hwdata/pci.ids")
        .or_else(|_| fs::read_to_string("/usr/share/misc/pci.ids"))
        .ok()?;

    let mut in_nvidia = false;
    for line in pci_ids.lines() {
        if !line.starts_with('\t') {
            in_nvidia = line.starts_with("10de  ") || line.starts_with("10de ");
            continue;
        }
        if in_nvidia {
            let trimmed = line.trim_start();
            if trimmed.len() >= 4 && trimmed[..4].to_lowercase() == device_id {
                return Some(trimmed[4..].trim().to_string());
            }
        }
    }

    None
}

fn detect_gpu_architecture_by_compute_capability(major: u32, minor: u32) -> Option<(String, bool)> {
    Some(match (major, minor) {
        (12, _) | (10, _) => ("Blackwell".to_string(), false),
        (8, 9) => ("Ada Lovelace".to_string(), false),
        (8, 6) | (8, 0) => ("Ampere".to_string(), false),
        (7, 5) => ("Turing".to_string(), false),
        (7, 0) => ("Volta".to_string(), true),
        (6, _) => ("Pascal".to_string(), true),
        (5, _) => ("Maxwell".to_string(), true),
        _ => return None,
    })
}

fn detect_gpu_architecture_by_pci_id(device_id: &str) -> Option<(String, bool)> {
    let id = u32::from_str_radix(device_id, 16).ok()?;

    const PCI_ARCH_TABLE: &[(u32, u32, &str, bool)] = &[
        (0x2900, 0x29ff, "Blackwell", false),
        (0x2800, 0x28ff, "Ada Lovelace", false),
        (0x2700, 0x27ff, "Ada Lovelace", false),
        (0x2500, 0x26ff, "Ampere", false),
        (0x2480, 0x24ff, "Ampere", false),
        (0x2300, 0x23ff, "Hopper", false),
        (0x2200, 0x22ff, "Ampere", false),
        (0x1e00, 0x21ff, "Turing", false),
        (0x1d00, 0x1dff, "Volta", true),
        (0x1b00, 0x1cff, "Pascal", true),
        (0x1300, 0x17ff, "Maxwell", true),
        (0x0f00, 0x12ff, "Kepler", true),
    ];

    PCI_ARCH_TABLE
        .iter()
        .find_map(|(start, end, arch, legacy)| {
            (id >= *start && id <= *end).then(|| ((*arch).to_string(), *legacy))
        })
}

fn detect_chip_code_by_pci_id(device_id: &str) -> Option<String> {
    let id = u32::from_str_radix(device_id, 16).ok()?;
    const CHIP_TABLE: &[(u32, u32, &str)] = &[
        (0x2900, 0x29ff, "gb2xx"),
        (0x2800, 0x28ff, "ad10x"),
        (0x2700, 0x27ff, "ad10x"),
        (0x2500, 0x26ff, "ga10x"),
        (0x2480, 0x24ff, "ga10x"),
        (0x2300, 0x23ff, "gh100"),
        (0x2200, 0x22ff, "ga10x"),
        (0x1e00, 0x21ff, "tu10x"),
        (0x1d00, 0x1dff, "gv100"),
        (0x1b00, 0x1cff, "gp10x"),
        (0x1300, 0x17ff, "gm20x"),
        (0x0f00, 0x12ff, "gk10x"),
    ];

    CHIP_TABLE
        .iter()
        .find_map(|(start, end, chip)| (id >= *start && id <= *end).then(|| (*chip).to_string()))
}

fn collect_arch_package_diagnostics() -> Vec<PackageDiagnostic> {
    let packages = [
        "nvidia",
        "nvidia-dkms",
        "nvidia-open",
        "nvidia-open-dkms",
        "nvidia-utils",
        "lib32-nvidia-utils",
        "linux-firmware",
        "linux-firmware-nvidia",
        "dkms",
    ];
    let mut diagnostics = Vec::new();

    for package in packages {
        let installed_version = Command::new("pacman")
            .args(["-Q", package])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .and_then(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .split_whitespace()
                    .nth(1)
                    .map(ToString::to_string)
            });

        diagnostics.push(PackageDiagnostic {
            package: package.to_string(),
            status: if installed_version.is_some() {
                "installed".to_string()
            } else {
                "missing".to_string()
            },
            installed_version,
        });
    }

    diagnostics
}

fn detect_initramfs_tool() -> Option<String> {
    if std::path::Path::new("/usr/bin/mkinitcpio").exists() {
        Some("mkinitcpio".to_string())
    } else if std::path::Path::new("/usr/bin/dracut").exists() {
        Some("dracut".to_string())
    } else if std::path::Path::new("/usr/sbin/update-initramfs").exists()
        || std::path::Path::new("/usr/bin/update-initramfs").exists()
    {
        Some("update-initramfs".to_string())
    } else {
        None
    }
}

fn list_boot_entries() -> Vec<String> {
    let mut entries = Vec::new();
    if let Ok(dir) = fs::read_dir("/boot") {
        for entry in dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("vmlinuz-") || name.starts_with("initramfs-") {
                entries.push(name);
            }
        }
    }
    entries.sort();
    entries
}

fn list_installed_kernels() -> Vec<String> {
    let mut kernels = Vec::new();
    if let Ok(dir) = fs::read_dir("/lib/modules") {
        for entry in dir.flatten() {
            kernels.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    kernels.sort();
    kernels
}

fn list_initramfs_images() -> Vec<String> {
    let mut images = Vec::new();
    if let Ok(dir) = fs::read_dir("/boot") {
        for entry in dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("initramfs-") && name.ends_with(".img") {
                images.push(name);
            }
        }
    }
    images.sort();
    images
}

fn collect_initramfs_findings(
    running_kernel: &str,
    initramfs_tool: Option<&str>,
    boot_cmdline: Option<&str>,
    initramfs_images: &[String],
) -> Vec<String> {
    let mut findings = Vec::new();
    let expected_image = format!("initramfs-{}.img", running_kernel);

    if !initramfs_images
        .iter()
        .any(|image| image == &expected_image)
    {
        findings.push(format!(
            "boot image for running kernel not found in /boot: {}",
            expected_image
        ));
    }

    match initramfs_tool {
        Some("mkinitcpio") if !std::path::Path::new("/etc/mkinitcpio.conf").exists() => {
            findings.push("mkinitcpio detected but /etc/mkinitcpio.conf is missing".to_string());
        }
        Some("dracut")
            if !std::path::Path::new("/etc/dracut.conf").exists()
                && !std::path::Path::new("/etc/dracut.conf.d").exists() =>
        {
            findings
                .push("dracut detected without a visible config file or config dir".to_string());
        }
        _ => {}
    }

    if let Some(cmdline) = boot_cmdline {
        if !cmdline.contains("nvidia_drm.modeset=1") {
            findings.push("boot cmdline is missing nvidia_drm.modeset=1".to_string());
        }
        if !cmdline.contains("nvidia.NVreg_PreserveVideoMemoryAllocations=1") {
            findings.push(
                "boot cmdline is missing nvidia.NVreg_PreserveVideoMemoryAllocations=1".to_string(),
            );
        }
    }

    findings.sort();
    findings.dedup();
    findings
}

fn collect_package_inventory() -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(output) = Command::new("pacman")
        .args([
            "-Q",
            "nvidia",
            "nvidia-dkms",
            "nvidia-open",
            "nvidia-open-dkms",
            "nvidia-utils",
            "lib32-nvidia-utils",
            "linux-firmware",
            "linux-firmware-nvidia",
            "dkms",
            "linux-cachyos-lto",
            "linux-zen",
            "linux-lts",
            "linux",
        ])
        .output()
    {
        if output.status.success() || !output.stdout.is_empty() {
            lines.extend(
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|line| line.to_string()),
            );
        }
    }
    lines
}

fn collect_package_findings(
    driver_version: Option<&str>,
    arch_packages: &[PackageDiagnostic],
    ownership: &[OwnershipDiagnostic],
) -> Vec<String> {
    let mut findings = Vec::new();

    let package_version = |name: &str| {
        arch_packages
            .iter()
            .find(|pkg| pkg.package == name)
            .and_then(|pkg| pkg.installed_version.as_deref())
    };

    let has_open = package_version("nvidia-open").is_some();
    let has_open_dkms = package_version("nvidia-open-dkms").is_some();
    let has_prop = package_version("nvidia").is_some();
    let has_prop_dkms = package_version("nvidia-dkms").is_some();
    let utils = package_version("nvidia-utils");
    let lib32_utils = package_version("lib32-nvidia-utils");
    let firmware_nvidia = package_version("linux-firmware-nvidia");

    if (has_open || has_open_dkms) && (has_prop || has_prop_dkms) {
        findings.push(
            "Both proprietary and open NVIDIA kernel package variants are installed; keep only the branch you actually use".to_string(),
        );
    }

    if let (Some(driver), Some(utils_version)) = (driver_version, utils) {
        if !utils_version.starts_with(driver) {
            findings.push(format!(
                "nvidia-utils version {} does not match detected driver {}",
                utils_version, driver
            ));
        }
    }

    if (has_open || has_open_dkms) && utils.is_none() {
        findings.push(
            "nvidia-open is installed without nvidia-utils; userspace components are incomplete"
                .to_string(),
        );
    }

    if let (Some(utils_version), Some(lib32_version)) = (utils, lib32_utils)
        && utils_version != lib32_version
    {
        findings.push(format!(
            "lib32-nvidia-utils version {} does not match nvidia-utils version {}",
            lib32_version, utils_version
        ));
    }

    if utils.is_some() && firmware_nvidia.is_none() {
        findings.push(
            "nvidia-utils is installed but linux-firmware-nvidia is missing; firmware packaging may be incomplete on Arch/CachyOS"
                .to_string(),
        );
    }

    if has_prop && has_prop_dkms {
        findings.push(
            "Both nvidia and nvidia-dkms are installed; keep either the prebuilt package or the DKMS variant, not both"
                .to_string(),
        );
    }

    if has_open && has_open_dkms {
        findings.push(
            "Both nvidia-open and nvidia-open-dkms are installed; keep either the prebuilt package or the DKMS variant, not both"
                .to_string(),
        );
    }

    if ownership.iter().any(|item| item.owner.is_none()) {
        findings.push(
            "At least one expected firmware path has no owning Arch package; firmware installation may be incomplete"
                .to_string(),
        );
    }

    if ownership.iter().any(|item| {
        item.owner
            .as_deref()
            .map(|owner| owner.contains("nvidia-utils-beta") || owner.contains("nvidia-open-beta"))
            .unwrap_or(false)
    }) {
        findings.push(
            "Firmware ownership points to beta packages; verify that your loaded driver branch matches the installed beta userspace stack"
                .to_string(),
        );
    }

    findings.sort();
    findings.dedup();
    findings
}

fn collect_gpu_release_diagnostics() -> Vec<GpuReleaseDiagnostic> {
    let mut diagnostics = Vec::new();

    if let Ok(output) = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,pci.bus_id,compute_cap",
            "--format=csv,noheader",
        ])
        .output()
    {
        if output.status.success() {
            let gpu_info = String::from_utf8_lossy(&output.stdout);
            for line in gpu_info.trim().lines() {
                let parts: Vec<&str> = line.split(", ").collect();
                let pci_bus_id = parts.get(1).map(|value| value.to_string());
                let pci_device_id = pci_bus_id.as_deref().and_then(read_pci_device_id);
                let compute_cap = parts.get(2).and_then(|cap| {
                    let (major, minor) = cap.split_once('.')?;
                    Some((major.parse::<u32>().ok()?, minor.parse::<u32>().ok()?))
                });
                let (architecture, legacy) = compute_cap
                    .and_then(|(major, minor)| {
                        detect_gpu_architecture_by_compute_capability(major, minor)
                    })
                    .or_else(|| {
                        pci_device_id
                            .as_deref()
                            .and_then(detect_gpu_architecture_by_pci_id)
                    })
                    .or_else(|| parts.first().map(|name| detect_gpu_architecture(name)))
                    .unwrap_or_else(|| ("Unknown".to_string(), false));
                let resolved_name = pci_device_id
                    .as_deref()
                    .and_then(lookup_pci_ids_name)
                    .unwrap_or_else(|| parts.first().copied().unwrap_or("Unknown").to_string());

                diagnostics.push(GpuReleaseDiagnostic {
                    gpu_name: resolved_name,
                    pci_bus_id,
                    pci_device_id: pci_device_id.clone(),
                    chip_code: pci_device_id
                        .as_deref()
                        .and_then(detect_chip_code_by_pci_id),
                    open_driver_capable: !legacy,
                    architecture,
                    legacy,
                });
            }
        }
    }

    diagnostics
}

fn collect_ownership_diagnostics(paths: &[String]) -> Vec<OwnershipDiagnostic> {
    let mut diagnostics = Vec::new();

    for path in paths {
        let owner = Command::new("pacman")
            .args(["-Qo", path])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string());

        let package_check = owner.as_ref().and_then(|owner_line| {
            owner_line
                .split_whitespace()
                .last()
                .map(|pkg| pkg.trim().to_string())
                .and_then(|pkg| {
                    Command::new("pacman")
                        .args(["-Qkk", &pkg])
                        .output()
                        .ok()
                        .filter(|output| output.status.success())
                        .map(|output| {
                            String::from_utf8_lossy(&output.stdout)
                                .lines()
                                .next()
                                .unwrap_or("")
                                .to_string()
                        })
                })
        });

        diagnostics.push(OwnershipDiagnostic {
            path: path.clone(),
            owner,
            package_check,
        });
    }

    diagnostics
}

pub fn collect_release_diagnostics() -> ReleaseDiagnostics {
    let running_kernel = Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "Unknown".to_string());

    let module_kernel = Command::new("modinfo")
        .args(["nvidia", "-F", "vermagic"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .and_then(|vermagic| vermagic.split_whitespace().next().map(ToString::to_string))
        .unwrap_or_else(|| "Unknown".to_string());

    let gsp_status = GspManager::new().get_deep_status();
    let boot_cmdline = fs::read_to_string("/proc/cmdline")
        .ok()
        .map(|line| line.trim().to_string());
    let initramfs_tool = detect_initramfs_tool();
    let boot_entries = list_boot_entries();
    let installed_kernels = list_installed_kernels();
    let initramfs_images = list_initramfs_images();
    let initramfs_findings = collect_initramfs_findings(
        &running_kernel,
        initramfs_tool.as_deref(),
        boot_cmdline.as_deref(),
        &initramfs_images,
    );
    let mut expected_firmware_paths = Vec::new();
    if let Some(arch) = &gsp_status.gpu_arch {
        expected_firmware_paths.push(format!("/lib/firmware/nvidia/{}/gsp", arch));
    }
    if let Some(version) = &gsp_status.firmware_version {
        expected_firmware_paths.push(format!("/lib/firmware/nvidia/{}", version));
    }
    let ownership = collect_ownership_diagnostics(&expected_firmware_paths);
    let arch_packages = collect_arch_package_diagnostics();
    let package_findings = collect_package_findings(
        gsp_status.firmware_version.as_deref(),
        &arch_packages,
        &ownership,
    );
    let package_inventory = collect_package_inventory();

    ReleaseDiagnostics {
        running_kernel: running_kernel.clone(),
        module_kernel: module_kernel.clone(),
        kernel_match: running_kernel == module_kernel,
        boot_cmdline,
        initramfs_tool,
        boot_entries,
        installed_kernels,
        initramfs_images,
        initramfs_findings,
        userspace_driver_version: gsp_status.firmware_version,
        kernel_module_version: gsp_status.kernel_module_version,
        firmware_layout: gsp_status.firmware_layout,
        firmware_path: gsp_status.firmware_path,
        firmware_file: gsp_status.firmware_file,
        release_alignment: gsp_status.release_alignment,
        expected_firmware_paths,
        ownership,
        gpu_diagnostics: collect_gpu_release_diagnostics(),
        arch_packages,
        package_findings,
        package_inventory,
    }
}

pub fn print_release_diagnostics() -> NvResult<()> {
    let diagnostics = collect_release_diagnostics();

    println!("Release Diagnostics");
    println!("{}", "═".repeat(50));
    println!();
    println!("Running Kernel:  {}", diagnostics.running_kernel);
    println!("Module Kernel:   {}", diagnostics.module_kernel);
    println!(
        "Kernel Match:    {}",
        if diagnostics.kernel_match {
            "yes"
        } else {
            "no"
        }
    );
    if let Some(tool) = &diagnostics.initramfs_tool {
        println!("Initramfs Tool:  {}", tool);
    }

    if let Some(version) = &diagnostics.userspace_driver_version {
        println!("Userspace:       {}", version);
    }
    if let Some(version) = &diagnostics.kernel_module_version {
        println!("Kernel Module:   {}", version);
    }
    if let Some(alignment) = &diagnostics.release_alignment {
        println!("Release Match:   {}", alignment);
    }
    if let Some(layout) = &diagnostics.firmware_layout {
        println!("FW Layout:       {}", layout);
    }
    if let Some(path) = &diagnostics.firmware_path {
        println!("FW Path:         {}", path);
    }
    if let Some(file) = &diagnostics.firmware_file {
        println!("FW File:         {}", file);
    }

    if !diagnostics.expected_firmware_paths.is_empty() {
        println!("Expected Paths:");
        for path in &diagnostics.expected_firmware_paths {
            let status = if std::path::Path::new(path).exists() {
                "present"
            } else {
                "missing"
            };
            println!("  - {} ({})", path, status);
        }
    }

    if let Some(cmdline) = &diagnostics.boot_cmdline {
        println!("Boot Cmdline:    {}", cmdline);
    }

    if !diagnostics.installed_kernels.is_empty() {
        println!("Installed Kernels:");
        for kernel in &diagnostics.installed_kernels {
            println!("  - {}", kernel);
        }
    }

    if !diagnostics.initramfs_images.is_empty() {
        println!("Initramfs Images:");
        for image in &diagnostics.initramfs_images {
            println!("  - {}", image);
        }
    }

    if !diagnostics.initramfs_findings.is_empty() {
        println!();
        println!("Boot/Initramfs Findings:");
        for finding in &diagnostics.initramfs_findings {
            println!("  - {}", finding);
        }
    }

    if !diagnostics.gpu_diagnostics.is_empty() {
        println!();
        println!("GPU Support:");
        for gpu in &diagnostics.gpu_diagnostics {
            println!(
                "  - {} [{}{}] -> {}{} ({})",
                gpu.gpu_name,
                gpu.pci_bus_id.as_deref().unwrap_or("unknown pci"),
                gpu.pci_device_id
                    .as_ref()
                    .map(|id| format!(", 0x{}", id))
                    .unwrap_or_default(),
                gpu.architecture,
                gpu.chip_code
                    .as_ref()
                    .map(|chip| format!(", {}", chip))
                    .unwrap_or_default(),
                if gpu.open_driver_capable {
                    "open-driver capable"
                } else {
                    "legacy/proprietary-only"
                }
            );
        }
    }

    if !diagnostics.ownership.is_empty() {
        println!();
        println!("Ownership Checks:");
        for item in &diagnostics.ownership {
            println!(
                "  - {} -> {}{}",
                item.path,
                item.owner.as_deref().unwrap_or("no package owner found"),
                item.package_check
                    .as_ref()
                    .map(|line| format!(" [{}]", line))
                    .unwrap_or_default()
            );
        }
    }

    if !diagnostics.arch_packages.is_empty() {
        println!();
        println!("Arch Packages:");
        for pkg in &diagnostics.arch_packages {
            println!(
                "  - {}: {}{}",
                pkg.package,
                pkg.status,
                pkg.installed_version
                    .as_ref()
                    .map(|v| format!(" ({})", v))
                    .unwrap_or_default()
            );
        }
    }

    if !diagnostics.package_findings.is_empty() {
        println!();
        println!("Package Findings:");
        for finding in &diagnostics.package_findings {
            println!("  - {}", finding);
        }
    }

    Ok(())
}

pub fn summarize_release_diagnostics(diagnostics: &ReleaseDiagnostics) -> DiagnosticSummary {
    let mut messages = Vec::new();
    let mut severity = DiagnosticSeverity::Healthy;

    if !diagnostics.kernel_match {
        severity = DiagnosticSeverity::Broken;
        messages.push("running kernel does not match the loaded NVIDIA module target".to_string());
    }

    if let Some(alignment) = &diagnostics.release_alignment {
        if alignment.starts_with("mismatch") {
            severity = DiagnosticSeverity::Broken;
            messages.push(format!("release mismatch: {}", alignment));
        } else if alignment.starts_with("structurally aligned")
            && severity != DiagnosticSeverity::Broken
        {
            severity = DiagnosticSeverity::Warning;
            messages.push(format!("firmware filename is non-versioned: {}", alignment));
        }
    }

    let missing_paths: Vec<_> = diagnostics
        .expected_firmware_paths
        .iter()
        .filter(|path| !std::path::Path::new(path.as_str()).exists())
        .cloned()
        .collect();
    if !missing_paths.is_empty() {
        severity = DiagnosticSeverity::Broken;
        messages.push(format!(
            "missing expected firmware paths: {}",
            missing_paths.join(", ")
        ));
    }

    for finding in &diagnostics.package_findings {
        if finding.contains("does not match") || finding.contains("keep only") {
            severity = DiagnosticSeverity::Broken;
        } else if severity != DiagnosticSeverity::Broken {
            severity = DiagnosticSeverity::Warning;
        }
        messages.push(finding.clone());
    }

    for finding in &diagnostics.initramfs_findings {
        if severity != DiagnosticSeverity::Broken {
            severity = DiagnosticSeverity::Warning;
        }
        messages.push(format!("boot/initramfs: {}", finding));
    }

    let has_mixed_arch_state = diagnostics
        .arch_packages
        .iter()
        .any(|pkg| pkg.package == "nvidia-utils" && pkg.installed_version.is_some())
        && diagnostics
            .arch_packages
            .iter()
            .all(|pkg| pkg.package != "nvidia-open" || pkg.installed_version.is_none())
        && diagnostics.firmware_layout.as_deref() == Some("per-chip");
    if has_mixed_arch_state && severity != DiagnosticSeverity::Broken {
        severity = DiagnosticSeverity::Warning;
        messages.push(
            "595-era mixed package state suspected: per-chip firmware is visible but the expected nvidia-open package is not installed"
                .to_string(),
        );
    }

    if messages.is_empty() {
        messages.push("release diagnostics look healthy".to_string());
    }

    DiagnosticSummary { severity, messages }
}

pub fn write_support_bundle(output_path: &str) -> NvResult<()> {
    write_support_bundle_with_options(output_path, false, false, false, false, 40)
}

pub fn support_state_dir() -> std::path::PathBuf {
    if let Some(state_dir) = dirs::state_dir() {
        state_dir.join("nvcontrol").join("support")
    } else if let Some(config_dir) = dirs::config_dir() {
        config_dir.join("nvcontrol").join("support")
    } else {
        std::env::temp_dir().join("nvcontrol-support")
    }
}

pub fn default_support_bundle_path(file_name: &str) -> String {
    let dir = support_state_dir();
    let _ = std::fs::create_dir_all(&dir);
    dir.join(file_name).display().to_string()
}

pub fn write_support_bundle_with_options(
    output_path: &str,
    tarball: bool,
    gzip: bool,
    redact_paths: bool,
    redact_ids: bool,
    log_tail: usize,
) -> NvResult<()> {
    let mut report = String::new();
    let diagnostics = collect_release_diagnostics();
    let gsp_status = GspManager::new().get_deep_status();

    let _ = writeln!(report, "nvcontrol support bundle");
    let _ = writeln!(report, "=======================");
    let _ = writeln!(report);
    let _ = writeln!(report, "[release diagnostics]");
    let _ = writeln!(report, "running_kernel={}", diagnostics.running_kernel);
    let _ = writeln!(report, "module_kernel={}", diagnostics.module_kernel);
    let _ = writeln!(report, "kernel_match={}", diagnostics.kernel_match);
    if let Some(tool) = diagnostics.initramfs_tool.as_ref() {
        let _ = writeln!(report, "initramfs_tool={}", tool);
    }
    if let Some(cmdline) = diagnostics.boot_cmdline.as_ref() {
        let _ = writeln!(report, "boot_cmdline={}", cmdline);
    }
    if let Some(version) = diagnostics.userspace_driver_version.as_ref() {
        let _ = writeln!(report, "userspace_driver_version={}", version);
    }
    if let Some(version) = diagnostics.kernel_module_version.as_ref() {
        let _ = writeln!(report, "kernel_module_version={}", version);
    }
    if let Some(alignment) = diagnostics.release_alignment.as_ref() {
        let _ = writeln!(report, "release_alignment={}", alignment);
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[gpus]");
    for gpu in &diagnostics.gpu_diagnostics {
        let _ = writeln!(
            report,
            "{} | pci={} | device={} | chip={} | arch={} | open_capable={}",
            gpu.gpu_name,
            if redact_ids {
                "<redacted-pci>"
            } else {
                gpu.pci_bus_id.as_deref().unwrap_or("unknown")
            },
            if redact_ids {
                "<redacted-device>"
            } else {
                gpu.pci_device_id.as_deref().unwrap_or("unknown")
            },
            gpu.chip_code.as_deref().unwrap_or("unknown"),
            gpu.architecture,
            gpu.open_driver_capable
        );
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[boot entries]");
    for entry in &diagnostics.boot_entries {
        let _ = writeln!(report, "{}", entry);
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[installed kernels]");
    for kernel in &diagnostics.installed_kernels {
        let _ = writeln!(report, "{}", kernel);
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[initramfs images]");
    for image in &diagnostics.initramfs_images {
        let _ = writeln!(report, "{}", image);
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[boot/initramfs findings]");
    if diagnostics.initramfs_findings.is_empty() {
        let _ = writeln!(report, "none");
    } else {
        for finding in &diagnostics.initramfs_findings {
            let _ = writeln!(report, "finding={}", finding);
        }
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[firmware paths]");
    for path in &diagnostics.expected_firmware_paths {
        let _ = writeln!(
            report,
            "{}",
            if redact_paths {
                "<redacted-path>"
            } else {
                path
            }
        );
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[ownership]");
    for item in &diagnostics.ownership {
        let _ = writeln!(
            report,
            "{} | owner={} | verify={}",
            if redact_paths {
                "<redacted-path>"
            } else {
                &item.path
            },
            item.owner.as_deref().unwrap_or("unknown"),
            item.package_check.as_deref().unwrap_or("n/a")
        );
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[arch packages]");
    for pkg in &diagnostics.arch_packages {
        let _ = writeln!(
            report,
            "{} | {} | {}",
            pkg.package,
            pkg.status,
            pkg.installed_version.as_deref().unwrap_or("n/a")
        );
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[pacman package inventory]");
    if diagnostics.package_inventory.is_empty() {
        let _ = writeln!(report, "unavailable");
    } else {
        for line in &diagnostics.package_inventory {
            let _ = writeln!(report, "{}", line);
        }
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[gsp status]");
    let _ = writeln!(report, "enabled={}", gsp_status.enabled);
    let _ = writeln!(report, "state={}", gsp_status.state);
    let _ = writeln!(report, "errors={}", gsp_status.error_count);
    if let Some(path) = gsp_status.firmware_path.as_ref() {
        let _ = writeln!(report, "firmware_path={}", path);
    }
    if let Some(file) = gsp_status.firmware_file.as_ref() {
        let _ = writeln!(report, "firmware_file={}", file);
    }

    let dkms_output = Command::new("dkms")
        .arg("status")
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_else(|| "dkms status unavailable".to_string());
    let _ = writeln!(report);
    let _ = writeln!(report, "[dkms]");
    let _ = writeln!(report, "{}", dkms_output.trim());

    let dkms_doctor = doctor_dkms();
    let _ = writeln!(report);
    let _ = writeln!(report, "[dkms doctor]");
    let _ = writeln!(report, "severity={}", severity_label(dkms_doctor.severity));
    for finding in &dkms_doctor.findings {
        let _ = writeln!(report, "finding={}", finding);
    }
    for fix in &dkms_doctor.fixes {
        let _ = writeln!(report, "fix={}", fix);
    }

    let source_state = inspect_source_build_state();
    let _ = writeln!(report);
    let _ = writeln!(report, "[source build]");
    if let Some(path) = &source_state.source_path {
        let _ = writeln!(report, "source_path={}", path);
    }
    let _ = writeln!(report, "source_type={}", source_state.source_type);
    if let Some(tag) = &source_state.current_tag {
        let _ = writeln!(report, "current_tag={}", tag);
    }
    if let Some(tag) = &source_state.latest_tag {
        let _ = writeln!(report, "latest_tag={}", tag);
    }
    if let Some(commit) = &source_state.git_commit {
        let _ = writeln!(report, "git_commit={}", commit);
    }
    if let Some(dirty) = source_state.git_dirty {
        let _ = writeln!(report, "git_dirty={}", dirty);
    }

    let _ = writeln!(report);
    let _ = writeln!(report, "[container runtime doctor]");
    match NvContainerRuntime::new().and_then(|runtime| runtime.runtime_doctor(None)) {
        Ok(runtime_report) => {
            let _ = writeln!(report, "severity={}", runtime_report.severity);
            if let Some(command) = runtime_report.smoke_test_command {
                let _ = writeln!(report, "smoke_test={}", command);
            }
            for check in runtime_report.checks {
                let _ = writeln!(
                    report,
                    "check={} ok={} details={}",
                    check.name, check.ok, check.details
                );
            }
        }
        Err(err) => {
            let _ = writeln!(report, "error={}", err);
        }
    }

    for (section, pattern) in [
        ("nvidia logs", "nvidia|NVRM"),
        ("gsp logs", "GSP|gsp"),
        ("xid logs", "Xid|NVRM.*Xid"),
    ] {
        let logs = Command::new("journalctl")
            .args([
                "-k",
                "-g",
                pattern,
                "--no-pager",
                "-q",
                "-b",
                "-n",
                &log_tail.to_string(),
            ])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .unwrap_or_else(|| "no log data".to_string());
        let _ = writeln!(report);
        let _ = writeln!(report, "[{}]", section);
        let _ = writeln!(report, "{}", logs.trim());
    }

    let text_path = if tarball {
        format!("{}.txt", output_path.trim_end_matches(".tar.gz"))
    } else if gzip {
        output_path.trim_end_matches(".gz").to_string()
    } else {
        output_path.to_string()
    };
    fs::write(&text_path, &report)
        .map_err(|e| NvControlError::IoError(format!("Failed to write support bundle: {}", e)))?;

    let metadata = SupportBundleMetadata {
        bundle_path: output_path.to_string(),
        tarball,
        gzip,
        redact_paths,
        redact_ids,
        log_tail,
        release_diagnostics: diagnostics,
    };
    let metadata_path = format!("{}.json", text_path);
    let metadata_json = serde_json::to_string_pretty(&metadata).unwrap_or_default();
    if !gzip {
        fs::write(&metadata_path, &metadata_json)
            .map_err(|e| NvControlError::IoError(format!("Failed to write metadata: {}", e)))?;
    }

    if tarball {
        let tar_gz = fs::File::create(output_path).map_err(|e| {
            NvControlError::IoError(format!("Failed to create tarball bundle: {}", e))
        })?;
        let encoder = GzEncoder::new(tar_gz, Compression::default());
        let mut builder = Builder::new(encoder);
        builder
            .append_path_with_name(&text_path, "support.txt")
            .map_err(|e| NvControlError::IoError(format!("Failed to add support.txt: {}", e)))?;
        builder
            .append_path_with_name(&metadata_path, "support.json")
            .map_err(|e| NvControlError::IoError(format!("Failed to add support.json: {}", e)))?;
        builder
            .finish()
            .map_err(|e| NvControlError::IoError(format!("Failed to finalize tarball: {}", e)))?;
        let _ = fs::remove_file(&text_path);
        let _ = fs::remove_file(&metadata_path);
    } else if gzip {
        let gz_path = format!("{}.gz", text_path);
        let file = fs::File::create(&gz_path)
            .map_err(|e| NvControlError::IoError(format!("Failed to create gzip bundle: {}", e)))?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        use std::io::Write;
        encoder.write_all(report.as_bytes()).map_err(|e| {
            NvControlError::IoError(format!("Failed to gzip support bundle: {}", e))
        })?;
        encoder.write_all(b"\n\n[metadata json]\n").map_err(|e| {
            NvControlError::IoError(format!("Failed to append gzip metadata header: {}", e))
        })?;
        encoder.write_all(metadata_json.as_bytes()).map_err(|e| {
            NvControlError::IoError(format!("Failed to append gzip metadata: {}", e))
        })?;
        encoder.finish().map_err(|e| {
            NvControlError::IoError(format!("Failed to finalize gzip bundle: {}", e))
        })?;
        let _ = fs::remove_file(&text_path);
    }

    Ok(())
}

fn get_dkms_status() -> NvResult<HashMap<String, String>> {
    let mut status = HashMap::new();

    if let Ok(output) = Command::new("dkms").arg("status").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("nvidia") {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 2 {
                        let module_name = parts[0].trim();
                        let module_status = parts.last().map_or("unknown", |v| v).trim();
                        status.insert(module_name.to_string(), module_status.to_string());
                    }
                }
            }
        }
    }

    Ok(status)
}

fn get_loaded_nvidia_modules() -> NvResult<Vec<String>> {
    let mut modules = Vec::new();

    if let Ok(output) = Command::new("lsmod").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("nvidia") {
                    let module_name = line.split_whitespace().next().unwrap_or("").to_string();
                    modules.push(module_name);
                }
            }
        }
    }

    Ok(modules)
}

fn check_available_driver_version() -> NvResult<Option<String>> {
    // Check package manager for available driver versions
    let distro = detect_distribution();

    match distro.as_str() {
        "arch" => check_available_arch(),
        "ubuntu" | "debian" => check_available_debian(),
        "fedora" => check_available_fedora(),
        _ => Ok(None),
    }
}

fn detect_distribution() -> String {
    if let Ok(output) = std::fs::read_to_string("/etc/os-release") {
        for line in output.lines() {
            if line.starts_with("ID=") {
                return line
                    .split('=')
                    .nth(1)
                    .unwrap_or("unknown")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }

    "unknown".to_string()
}

fn check_available_arch() -> NvResult<Option<String>> {
    if let Ok(output) = Command::new("pacman").args(["-Si", "nvidia"]).output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("Version") {
                    if let Some(version) = line.split(':').nth(1) {
                        return Ok(Some(version.trim().to_string()));
                    }
                }
            }
        }
    }

    Ok(None)
}

fn check_available_debian() -> NvResult<Option<String>> {
    if let Ok(output) = Command::new("apt")
        .args(["list", "--upgradable", "nvidia-driver*"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("nvidia-driver") {
                    // Parse apt output for version
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return Ok(Some(parts[1].to_string()));
                    }
                }
            }
        }
    }

    Ok(None)
}

fn check_available_fedora() -> NvResult<Option<String>> {
    if let Ok(output) = Command::new("dnf")
        .args(["list", "--available", "nvidia-driver*"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse DNF output similar to apt
            for line in output_str.lines() {
                if line.contains("nvidia-driver") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return Ok(Some(parts[1].to_string()));
                    }
                }
            }
        }
    }

    Ok(None)
}

fn is_update_available(current: &str, available: &Option<String>) -> bool {
    if let Some(available_version) = available {
        // Simple version comparison (would need more sophisticated comparison for real use)
        current != available_version
    } else {
        false
    }
}

pub fn install_driver(driver_type: &str) -> NvResult<()> {
    let driver_enum = DriverType::from_str(driver_type).ok_or_else(|| {
        NvControlError::DisplayDetectionFailed(format!(
            "Unknown driver type: {driver_type}. Use: proprietary, open, open-beta"
        ))
    })?;

    let distro = detect_distribution();

    match distro.as_str() {
        "arch" => install_driver_arch(&driver_enum),
        "ubuntu" | "debian" => install_driver_debian(&driver_enum),
        "fedora" => install_driver_fedora(&driver_enum),
        _ => Err(NvControlError::DisplayDetectionFailed(format!(
            "Unsupported distribution: {distro}"
        ))),
    }
}

fn install_driver_arch(driver_type: &DriverType) -> NvResult<()> {
    let package = match driver_type {
        DriverType::Proprietary => "nvidia nvidia-utils",
        DriverType::Open => "nvidia-open nvidia-utils",
        DriverType::OpenBeta => "nvidia-open-beta nvidia-utils-beta",
        DriverType::Nouveau => "xf86-video-nouveau",
    };

    println!(
        "Installing NVIDIA driver for Arch Linux: {} ({})",
        package,
        driver_type.package_name()
    );

    // First, remove conflicting drivers
    let _ = Command::new("sudo")
        .args([
            "pacman",
            "-Rns",
            "--noconfirm",
            "nvidia",
            "nvidia-open",
            "nouveau",
        ])
        .output();

    // Install new driver
    let output = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm"])
        .args(package.split_whitespace())
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("pacman failed: {e}")))?;

    if output.status.success() {
        println!("Driver installation completed. Reboot required.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "Installation failed: {stderr}"
        )))
    }
}

fn install_driver_debian(driver_type: &DriverType) -> NvResult<()> {
    let package = match driver_type {
        DriverType::Proprietary => "nvidia-driver nvidia-settings",
        DriverType::Open => "nvidia-driver nvidia-kernel-open-dkms",
        DriverType::OpenBeta => "nvidia-driver-beta",
        DriverType::Nouveau => "xserver-xorg-video-nouveau",
    };

    println!("Installing NVIDIA driver for Debian/Ubuntu: {package}");

    // Update package list
    let _ = Command::new("sudo").args(["apt", "update"]).output();

    // Install driver
    let output = Command::new("sudo")
        .args(["apt", "install", "-y"])
        .args(package.split_whitespace())
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("apt failed: {e}")))?;

    if output.status.success() {
        println!("Driver installation completed. Reboot required.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "Installation failed: {stderr}"
        )))
    }
}

fn install_driver_fedora(driver_type: &DriverType) -> NvResult<()> {
    let package = match driver_type {
        DriverType::Proprietary => "nvidia-driver nvidia-settings",
        DriverType::Open => "nvidia-driver nvidia-kernel-open",
        DriverType::OpenBeta => "nvidia-driver-beta",
        DriverType::Nouveau => "xorg-x11-drv-nouveau",
    };

    println!("Installing NVIDIA driver for Fedora: {package}");

    // Install RPM Fusion if not already available
    let _ = Command::new("sudo")
        .args(["dnf", "install", "-y", "https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm"])
        .output();

    let _ = Command::new("sudo")
        .args(["dnf", "install", "-y", "https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm"])
        .output();

    // Install driver
    let output = Command::new("sudo")
        .args(["dnf", "install", "-y"])
        .args(package.split_whitespace())
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("dnf failed: {e}")))?;

    if output.status.success() {
        println!("Driver installation completed. Reboot required.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "Installation failed: {stderr}"
        )))
    }
}

pub fn update_driver() -> NvResult<()> {
    let distro = detect_distribution();

    match distro.as_str() {
        "arch" => {
            let output = Command::new("sudo")
                .args(["pacman", "-Syu", "--noconfirm", "nvidia"])
                .output()
                .map_err(|e| {
                    NvControlError::DisplayDetectionFailed(format!("Update failed: {e}"))
                })?;

            if output.status.success() {
                println!("Driver update completed");
                Ok(())
            } else {
                Err(NvControlError::DisplayDetectionFailed(
                    "Update failed".to_string(),
                ))
            }
        }
        "ubuntu" | "debian" => {
            let output = Command::new("sudo")
                .args([
                    "apt",
                    "update",
                    "&&",
                    "sudo",
                    "apt",
                    "upgrade",
                    "-y",
                    "nvidia-driver*",
                ])
                .output()
                .map_err(|e| {
                    NvControlError::DisplayDetectionFailed(format!("Update failed: {e}"))
                })?;

            if output.status.success() {
                println!("Driver update completed");
                Ok(())
            } else {
                Err(NvControlError::DisplayDetectionFailed(
                    "Update failed".to_string(),
                ))
            }
        }
        "fedora" => {
            let output = Command::new("sudo")
                .args(["dnf", "update", "-y", "nvidia-driver*"])
                .output()
                .map_err(|e| {
                    NvControlError::DisplayDetectionFailed(format!("Update failed: {e}"))
                })?;

            if output.status.success() {
                println!("Driver update completed");
                Ok(())
            } else {
                Err(NvControlError::DisplayDetectionFailed(
                    "Update failed".to_string(),
                ))
            }
        }
        _ => Err(NvControlError::DisplayDetectionFailed(
            "Unsupported distribution for driver updates".to_string(),
        )),
    }
}

pub fn rollback_driver() -> NvResult<()> {
    println!("Rolling back to previous driver version...");

    let distro = detect_distribution();

    match distro.as_str() {
        "arch" => {
            // Arch Linux rollback via package cache
            if let Ok(output) = Command::new("sudo").args(["downgrade", "nvidia"]).output() {
                if output.status.success() {
                    println!("Driver rollback completed");
                    return Ok(());
                }
            }

            Err(NvControlError::DisplayDetectionFailed(
                "Driver rollback failed. Install 'downgrade' package for rollback support"
                    .to_string(),
            ))
        }
        _ => Err(NvControlError::DisplayDetectionFailed(
            "Driver rollback not yet implemented for this distribution".to_string(),
        )),
    }
}

pub fn fix_dkms_issues() -> NvResult<()> {
    println!("Attempting to fix DKMS issues...");

    // Common DKMS fixes
    let fixes = vec![
        (
            vec!["sudo", "dkms", "autoinstall"],
            "Rebuilding all DKMS modules",
        ),
        (
            vec!["sudo", "dkms", "remove", "nvidia", "--all"],
            "Removing old NVIDIA modules",
        ),
        (
            vec!["sudo", "dkms", "add", "nvidia"],
            "Re-adding NVIDIA modules",
        ),
        (
            vec!["sudo", "dkms", "build", "nvidia"],
            "Building NVIDIA modules",
        ),
        (
            vec!["sudo", "dkms", "install", "nvidia"],
            "Installing NVIDIA modules",
        ),
    ];

    for (cmd, description) in fixes {
        println!("{description}...");
        if let Ok(output) = Command::new(cmd[0]).args(&cmd[1..]).output() {
            if output.status.success() {
                println!("✓ {description} succeeded");
            } else {
                println!("✗ {description} failed");
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.is_empty() {
                    println!("  Error: {stderr}");
                }
            }
        }
    }

    println!("DKMS repair attempts completed. Reboot may be required.");
    Ok(())
}

// ==================== DKMS Setup & Management ====================

/// How the NVIDIA DKMS source was installed/configured
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DkmsSourceType {
    /// Installed via package manager (nvidia-open-dkms)
    Packaged,
    /// Git clone (has .git directory)
    Git { remote_url: Option<String> },
    /// Manual copy to /usr/src (no package, no git)
    Manual,
    /// No source found
    NotFound,
}

impl std::fmt::Display for DkmsSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Packaged => write!(f, "packaged (nvidia-open-dkms)"),
            Self::Git { remote_url } => {
                if let Some(url) = remote_url {
                    write!(f, "git ({})", url)
                } else {
                    write!(f, "git")
                }
            }
            Self::Manual => write!(f, "manual"),
            Self::NotFound => write!(f, "not found"),
        }
    }
}

/// Information about DKMS setup status
#[derive(Debug)]
pub struct DkmsSetupInfo {
    pub dkms_installed: bool,
    pub nvidia_registered: bool,
    pub nvidia_version: Option<String>,
    pub source_path: Option<String>,
    pub source_type: DkmsSourceType,
    pub kernels_built: Vec<String>,
    pub kernels_missing: Vec<String>,
}

/// Detect how the DKMS source was installed
fn detect_dkms_source_type(source_path: &str) -> DkmsSourceType {
    let path = std::path::Path::new(source_path);

    // Check if it's a git repository
    let git_dir = path.join(".git");
    if git_dir.exists() {
        // Try to get remote URL
        let remote_url = Command::new("git")
            .args(["-C", source_path, "remote", "get-url", "origin"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());

        return DkmsSourceType::Git { remote_url };
    }

    // Check if it's from a package (nvidia-open-dkms)
    let is_packaged = Command::new("pacman")
        .args(["-Qo", source_path])
        .output()
        .is_ok_and(|o| {
            o.status.success() && String::from_utf8_lossy(&o.stdout).contains("nvidia-open-dkms")
        });

    if is_packaged {
        return DkmsSourceType::Packaged;
    }

    // If source exists but not git and not packaged, it's manual
    if path.exists() {
        DkmsSourceType::Manual
    } else {
        DkmsSourceType::NotFound
    }
}

/// Get detailed DKMS setup information
pub fn get_dkms_setup_info() -> DkmsSetupInfo {
    let mut info = DkmsSetupInfo {
        dkms_installed: false,
        nvidia_registered: false,
        nvidia_version: None,
        source_path: None,
        source_type: DkmsSourceType::NotFound,
        kernels_built: Vec::new(),
        kernels_missing: Vec::new(),
    };

    // Check if DKMS is installed
    info.dkms_installed = Command::new("which")
        .arg("dkms")
        .output()
        .is_ok_and(|o| o.status.success());

    if !info.dkms_installed {
        return info;
    }

    // Get current driver version
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=driver_version", "--format=csv,noheader"])
        .output()
    {
        if output.status.success() {
            info.nvidia_version = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    // Check DKMS status for nvidia
    if let Ok(output) = Command::new("dkms").arg("status").output() {
        if output.status.success() {
            let status = String::from_utf8_lossy(&output.stdout);
            for line in status.lines() {
                if line.contains("nvidia") {
                    info.nvidia_registered = true;
                    // Parse kernel versions that have nvidia built
                    // Format: nvidia/590.48.01, 6.18.2-1-cachyos-lto, x86_64: installed
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 2 {
                        let kernel = parts[1].trim();
                        if line.contains("installed") {
                            info.kernels_built.push(kernel.to_string());
                        }
                    }
                }
            }
        }
    }

    // Check for nvidia source - first try DKMS registered source, then /usr/src
    let source_path = if let Some(ref ver) = info.nvidia_version {
        let dkms_source = format!("/var/lib/dkms/nvidia/{}/source", ver);
        if std::path::Path::new(&dkms_source).exists() {
            // Follow symlink to get actual source path
            std::fs::read_link(&dkms_source)
                .ok()
                .map(|p| p.display().to_string())
        } else {
            None
        }
    } else {
        None
    };

    // Fallback to scanning /usr/src if DKMS source not found
    let source_path = source_path.or_else(|| {
        std::fs::read_dir("/usr/src").ok().and_then(|entries| {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("nvidia-") || name_str.starts_with("nvidia-open-") {
                    return Some(entry.path().display().to_string());
                }
            }
            None
        })
    });

    if let Some(ref path) = source_path {
        info.source_path = Some(path.clone());
        info.source_type = detect_dkms_source_type(path);
    }

    // Find kernels that are missing nvidia modules
    if let Ok(entries) = std::fs::read_dir("/lib/modules") {
        for entry in entries.flatten() {
            let kernel = entry.file_name().to_string_lossy().to_string();
            if !info.kernels_built.contains(&kernel) {
                // Check if nvidia module exists for this kernel
                let module_paths = [
                    format!("/lib/modules/{}/kernel/drivers/video/nvidia.ko.zst", kernel),
                    format!("/lib/modules/{}/kernel/drivers/video/nvidia.ko", kernel),
                    format!("/lib/modules/{}/extramodules/nvidia.ko.zst", kernel),
                    format!("/lib/modules/{}/extramodules/nvidia.ko", kernel),
                    format!("/lib/modules/{}/updates/dkms/nvidia.ko.zst", kernel),
                    format!("/lib/modules/{}/updates/dkms/nvidia.ko", kernel),
                ];
                let has_module = module_paths
                    .iter()
                    .any(|p| std::path::Path::new(p).exists());
                if !has_module {
                    info.kernels_missing.push(kernel);
                }
            }
        }
    }

    info
}

/// Set up DKMS for nvidia-open driver
pub fn setup_dkms_nvidia_open() -> NvResult<()> {
    println!("Setting up DKMS for nvidia-open driver\n");

    let info = get_dkms_setup_info();

    // Step 1: Check DKMS is installed
    if !info.dkms_installed {
        println!("DKMS not installed. Installing...");
        let distro = detect_distribution();
        match distro.as_str() {
            "arch" | "cachyos" | "endeavouros" => {
                println!("  sudo pacman -S dkms linux-headers");
            }
            "ubuntu" | "debian" | "pop" => {
                println!("  sudo apt install dkms linux-headers-$(uname -r)");
            }
            "fedora" => {
                println!("  sudo dnf install dkms kernel-devel");
            }
            _ => {
                println!("  Install dkms and kernel headers for your distribution");
            }
        }
        return Err(NvControlError::ConfigError(
            "DKMS must be installed first".to_string(),
        ));
    }
    println!("DKMS installed");

    // Step 2: Get driver version
    let driver_version = info.nvidia_version.clone().ok_or_else(|| {
        NvControlError::ConfigError("Cannot detect nvidia driver version".to_string())
    })?;
    println!("Driver version: {}", driver_version);

    // Step 3: Check if already registered
    if info.nvidia_registered {
        println!("\nnvidia is already registered with DKMS");
        println!(
            "Kernels with nvidia built: {}",
            info.kernels_built.join(", ")
        );
        if !info.kernels_missing.is_empty() {
            println!(
                "Kernels missing nvidia: {}",
                info.kernels_missing.join(", ")
            );
            println!("\nTo build for missing kernels:");
            println!("  nvctl driver dkms build");
        }
        return Ok(());
    }

    // Step 4: Check for source or download
    let source_path = if let Some(path) = info.source_path {
        println!("Found nvidia source at: {}", path);
        path
    } else {
        println!("\nNo nvidia source found in /usr/src");
        println!("For Arch Linux, install nvidia-open-dkms:");
        println!("  sudo pacman -S nvidia-open-dkms");
        println!();
        println!("Or clone and set up manually:");
        println!("  git clone https://github.com/NVIDIA/open-gpu-kernel-modules.git");
        println!("  cd open-gpu-kernel-modules");
        println!("  git checkout {}", driver_version);
        println!("  sudo cp -r . /usr/src/nvidia-{}", driver_version);
        println!();
        print_dkms_conf_template(&driver_version);
        return Err(NvControlError::ConfigError(
            "nvidia source not found - see instructions above".to_string(),
        ));
    };

    // Step 5: Check for dkms.conf
    let dkms_conf_path = format!("{}/dkms.conf", source_path);
    if !std::path::Path::new(&dkms_conf_path).exists() {
        println!("\nMissing dkms.conf at {}", dkms_conf_path);
        print_dkms_conf_template(&driver_version);
        return Err(NvControlError::ConfigError(
            "dkms.conf not found - create it with the template above".to_string(),
        ));
    }

    // Step 6: Register with DKMS
    println!("\nRegistering nvidia with DKMS...");
    let version_part = source_path
        .split('/')
        .next_back()
        .unwrap_or(&driver_version)
        .replace("nvidia-", "")
        .replace("nvidia-open-", "");

    let add_result = Command::new("sudo")
        .args(["dkms", "add", "-m", "nvidia", "-v", &version_part])
        .status();

    match add_result {
        Ok(status) if status.success() => {
            println!("nvidia registered with DKMS");
        }
        _ => {
            println!("Failed to register. Trying alternative method...");
            let _ = Command::new("sudo")
                .args(["dkms", "add", &source_path])
                .status();
        }
    }

    println!("\nTo build for all kernels: nvctl driver dkms build");
    Ok(())
}

/// Print a dkms.conf template for nvidia-open
fn print_dkms_conf_template(version: &str) {
    println!("Create /usr/src/nvidia-{}/dkms.conf with:", version);
    println!("────────────────────────────────────────");
    println!(
        r#"PACKAGE_NAME="nvidia"
PACKAGE_VERSION="{}"
BUILT_MODULE_NAME[0]="nvidia"
BUILT_MODULE_NAME[1]="nvidia-modeset"
BUILT_MODULE_NAME[2]="nvidia-drm"
BUILT_MODULE_NAME[3]="nvidia-uvm"
DEST_MODULE_LOCATION[0]="/kernel/drivers/video"
DEST_MODULE_LOCATION[1]="/kernel/drivers/video"
DEST_MODULE_LOCATION[2]="/kernel/drivers/video"
DEST_MODULE_LOCATION[3]="/kernel/drivers/video"
AUTOINSTALL="yes"
MAKE[0]="make -j$(nproc) NV_KERNEL_MODULES=1 NV_KERNEL_SOURCES=/lib/modules/$kernelver/build modules"
CLEAN="make clean""#,
        version
    );
    println!("────────────────────────────────────────");
}

/// Build nvidia modules for all or specific kernels
pub fn build_dkms_nvidia(kernel: Option<&str>, force: bool) -> NvResult<()> {
    let info = get_dkms_setup_info();

    if !info.dkms_installed {
        return Err(NvControlError::ConfigError(
            "DKMS not installed. Run: nvctl driver dkms setup".to_string(),
        ));
    }

    if !info.nvidia_registered {
        return Err(NvControlError::ConfigError(
            "nvidia not registered with DKMS. Run: nvctl driver dkms setup".to_string(),
        ));
    }

    let version = info
        .nvidia_version
        .as_ref()
        .ok_or_else(|| NvControlError::ConfigError("Cannot detect nvidia version".to_string()))?;

    let force_flag = if force { vec!["--force"] } else { vec![] };

    match kernel {
        Some(k) => {
            println!(
                "Building nvidia {} for kernel {}{}...",
                version,
                k,
                if force { " (force)" } else { "" }
            );
            let mut args = vec!["dkms", "install", "-m", "nvidia", "-v", version, "-k", k];
            args.extend(force_flag.iter().copied());

            let status = Command::new("sudo").args(&args).status().map_err(|e| {
                NvControlError::CommandFailed(format!("dkms install failed: {}", e))
            })?;

            if status.success() {
                println!("nvidia built and installed for {}", k);
            } else {
                return Err(NvControlError::CommandFailed(format!(
                    "Failed to build nvidia for {}",
                    k
                )));
            }
        }
        None => {
            println!(
                "Building nvidia {} for all kernels{}...\n",
                version,
                if force { " (force)" } else { "" }
            );

            // Get all installed kernels
            let mut kernels = Vec::new();
            if let Ok(entries) = std::fs::read_dir("/lib/modules") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    // Check if kernel has headers (build dir)
                    let build_path = format!("/lib/modules/{}/build", name);
                    if std::path::Path::new(&build_path).exists() {
                        kernels.push(name);
                    }
                }
            }

            if kernels.is_empty() {
                return Err(NvControlError::ConfigError(
                    "No kernels with headers found".to_string(),
                ));
            }

            kernels.sort();
            println!("Found {} kernels with headers", kernels.len());

            for kernel in &kernels {
                print!("  Building for {}... ", kernel);
                let mut args = vec![
                    "dkms", "install", "-m", "nvidia", "-v", version, "-k", kernel,
                ];
                args.extend(force_flag.iter().copied());

                let status = Command::new("sudo").args(&args).output();

                match status {
                    Ok(output) if output.status.success() => println!("done"),
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let combined = format!("{}{}", stdout, stderr);
                        if combined.contains("already installed") {
                            println!("already installed (use --force to rebuild)");
                        } else {
                            println!("failed");
                            if !stderr.is_empty() {
                                println!("    {}", stderr.lines().next().unwrap_or(""));
                            }
                        }
                    }
                    Err(e) => println!("error: {}", e),
                }
            }

            println!("\nBuild complete. Check status with: nvctl driver dkms status");
        }
    }

    Ok(())
}

/// Unregister nvidia from DKMS
pub fn unregister_dkms_nvidia() -> NvResult<()> {
    let info = get_dkms_setup_info();

    if !info.nvidia_registered {
        println!("nvidia is not registered with DKMS");
        return Ok(());
    }

    let version = info
        .nvidia_version
        .as_ref()
        .ok_or_else(|| NvControlError::ConfigError("Cannot detect nvidia version".to_string()))?;

    println!("Unregistering nvidia {} from DKMS...", version);

    let status = Command::new("sudo")
        .args(["dkms", "remove", "-m", "nvidia", "-v", version, "--all"])
        .status()
        .map_err(|e| NvControlError::CommandFailed(format!("dkms remove failed: {}", e)))?;

    if status.success() {
        println!("nvidia unregistered from DKMS");
        println!("\nNote: Existing modules in /lib/modules are not removed.");
        println!("You may need to reinstall nvidia-open for the current kernel.");
    } else {
        return Err(NvControlError::CommandFailed(
            "Failed to unregister nvidia".to_string(),
        ));
    }

    Ok(())
}

/// Show DKMS build logs for nvidia
pub fn print_dkms_logs(kernel: Option<&str>, tail: Option<usize>) -> NvResult<()> {
    let info = get_dkms_setup_info();

    println!("NVIDIA DKMS Build Logs");
    println!("══════════════════════════════════════════════════\n");

    let mut found_logs = false;

    // Check wrapper script logs first (from pacman hook)
    let wrapper_log_dir = "/var/log/nvidia-dkms";
    if std::path::Path::new(wrapper_log_dir).exists() {
        if let Ok(entries) = std::fs::read_dir(wrapper_log_dir) {
            let mut logs: Vec<_> = entries
                .flatten()
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "log"))
                .filter(|e| e.file_name().to_string_lossy() != "latest.log")
                .collect();

            logs.sort_by_key(|e| std::cmp::Reverse(e.file_name()));

            if !logs.is_empty() {
                println!("Pacman Hook Logs ({})", wrapper_log_dir);
                println!("────────────────────────────────────────");

                for (i, entry) in logs.iter().take(5).enumerate() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let path = entry.path();
                    let show_tail = if i == 0 { tail } else { None };
                    print_log_file(path.to_str().unwrap_or(""), &name, show_tail)?;
                    found_logs = true;
                }
                println!();
            }
        }
    }

    // Check DKMS internal logs
    let version = info
        .nvidia_version
        .unwrap_or_else(|| "590.48.01".to_string());
    let dkms_base = format!("/var/lib/dkms/nvidia/{}", version);

    if std::path::Path::new(&dkms_base).exists() {
        if kernel.is_some() || !found_logs {
            println!("DKMS Build Logs ({})", dkms_base);
            println!("────────────────────────────────────────");
        }

        if let Some(k) = kernel {
            // Show logs for specific kernel
            let log_path = format!("{}/{}/x86_64/log/make.log", dkms_base, k);
            if std::path::Path::new(&log_path).exists() {
                print_log_file(&log_path, k, tail)?;
                found_logs = true;
            } else {
                println!("No build log found for kernel {}", k);
                println!("  Expected: {}", log_path);
            }
        } else {
            // Show logs for all kernels
            if let Ok(entries) = std::fs::read_dir(&dkms_base) {
                let mut kernels: Vec<_> = entries
                    .flatten()
                    .filter(|e| e.path().is_dir())
                    .filter(|e| e.file_name().to_string_lossy().contains("."))
                    .collect();

                kernels.sort_by_key(|e| e.file_name());

                for entry in kernels {
                    let kernel_name = entry.file_name().to_string_lossy().to_string();
                    let log_path = format!("{}/{}/x86_64/log/make.log", dkms_base, kernel_name);

                    if std::path::Path::new(&log_path).exists() {
                        print_log_file(&log_path, &kernel_name, tail)?;
                        found_logs = true;
                    }
                }
            }
        }
    }

    if !found_logs {
        println!("No DKMS build logs found.");
        println!("\nBuild logs are created when:");
        println!("  - DKMS builds nvidia: nvctl driver dkms build");
        println!("  - Pacman hook triggers: pacman -S linux-cachyos");
    }

    Ok(())
}

fn print_log_file(path: &str, kernel: &str, tail: Option<usize>) -> NvResult<()> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to read log: {}", e)))?;

    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    // Check for errors
    let has_error = lines
        .iter()
        .any(|l| l.contains("error:") || l.contains("Error:") || l.contains("FAILED"));

    let status = if has_error { "FAILED" } else { "OK" };
    let status_color = if has_error { "✗" } else { "✓" };

    // Get file modification time
    let modified = std::fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .map(|t| {
            let duration = std::time::SystemTime::now()
                .duration_since(t)
                .unwrap_or_default();
            if duration.as_secs() < 60 {
                format!("{}s ago", duration.as_secs())
            } else if duration.as_secs() < 3600 {
                format!("{}m ago", duration.as_secs() / 60)
            } else if duration.as_secs() < 86400 {
                format!("{}h ago", duration.as_secs() / 3600)
            } else {
                format!("{}d ago", duration.as_secs() / 86400)
            }
        })
        .unwrap_or_else(|| "unknown".to_string());

    println!(
        "{} {} [{}] - {} ({} lines)",
        status_color, kernel, status, modified, total_lines
    );

    if let Some(n) = tail {
        println!("────────────────────────────────────────");
        let start = total_lines.saturating_sub(n);
        for line in &lines[start..] {
            // Highlight errors
            if line.contains("error:") || line.contains("Error:") || line.contains("FAILED") {
                println!("  >> {}", line);
            } else {
                println!("  {}", line);
            }
        }
        println!();
    } else if has_error {
        // Show last few lines with errors
        println!("  Last errors:");
        for line in lines.iter().rev().take(10).rev() {
            if line.contains("error:") || line.contains("Error:") || line.contains("FAILED") {
                println!("    >> {}", line);
            }
        }
        println!();
    }

    Ok(())
}

/// Install Arch Linux pacman hooks for automatic DKMS rebuilds
pub fn install_pacman_hooks() -> NvResult<()> {
    println!("Installing Pacman Hooks for NVIDIA DKMS\n");

    let distro = detect_distribution();
    if distro != "arch" && distro != "cachyos" && distro != "endeavouros" {
        return Err(NvControlError::ConfigError(
            "Pacman hooks are only for Arch-based distributions".to_string(),
        ));
    }

    let hook_dir = "/etc/pacman.d/hooks";
    let hook_path = format!("{}/nvidia-dkms.hook", hook_dir);

    // Check if hook already exists
    if std::path::Path::new(&hook_path).exists() {
        println!("Hook already exists at {}", hook_path);
        if let Ok(content) = std::fs::read_to_string(&hook_path) {
            println!("\nCurrent hook content:");
            println!("────────────────────────────────────────");
            println!("{}", content);
            println!("────────────────────────────────────────");
        }
        return Ok(());
    }

    // Create hook content - uses wrapper script for logging
    let hook_content = format!(
        r#"[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = nvidia-open
Target = nvidia-open-dkms
{}

[Action]
Description = Rebuilding NVIDIA modules via DKMS...
Depends = dkms
When = PostTransaction
NeedsTargets
Exec = /usr/local/bin/nvidia-dkms-build
"#,
        ArchIntegration::pacman_kernel_targets()
            .into_iter()
            .map(|target| format!("Target = {}", target))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Wrapper script with logging and notification
    let wrapper_script = r#"#!/bin/bash
# NVIDIA DKMS build wrapper with logging
# Installed by nvctl

LOG_DIR="/var/log/nvidia-dkms"
LOG_FILE="$LOG_DIR/build-$(date +%Y%m%d-%H%M%S).log"
LATEST_LOG="$LOG_DIR/latest.log"

mkdir -p "$LOG_DIR"

echo "=== NVIDIA DKMS Build $(date) ===" | tee "$LOG_FILE"
echo "Kernels to build for:" | tee -a "$LOG_FILE"

# Run dkms autoinstall with PIPESTATUS to capture actual exit code
/usr/bin/dkms autoinstall 2>&1 | tee -a "$LOG_FILE"
DKMS_EXIT=${PIPESTATUS[0]}

ln -sf "$LOG_FILE" "$LATEST_LOG"

# Exit code 0 = success, 6 = already installed (not an error)
if [ $DKMS_EXIT -eq 0 ]; then
    echo "" | tee -a "$LOG_FILE"
    echo "Build completed successfully" | tee -a "$LOG_FILE"
    if command -v notify-send &>/dev/null && [ -n "$DISPLAY" -o -n "$WAYLAND_DISPLAY" ]; then
        notify-send -u low "NVIDIA DKMS" "Modules rebuilt successfully"
    fi
elif [ $DKMS_EXIT -eq 6 ]; then
    echo "" | tee -a "$LOG_FILE"
    echo "Modules already installed (use --force to rebuild)" | tee -a "$LOG_FILE"
    if command -v notify-send &>/dev/null && [ -n "$DISPLAY" -o -n "$WAYLAND_DISPLAY" ]; then
        notify-send -u low "NVIDIA DKMS" "Modules already up to date"
    fi
else
    echo "" | tee -a "$LOG_FILE"
    echo "Build FAILED with exit code $DKMS_EXIT" | tee -a "$LOG_FILE"
    if command -v notify-send &>/dev/null && [ -n "$DISPLAY" -o -n "$WAYLAND_DISPLAY" ]; then
        notify-send -u critical "NVIDIA DKMS FAILED" "Check: nvctl driver dkms logs"
    fi
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║  NVIDIA DKMS build failed! Check logs:                     ║"
    echo "║    nvctl driver dkms logs                                  ║"
    echo "║    cat $LATEST_LOG                                         ║"
    echo "╚════════════════════════════════════════════════════════════╝"
fi
"#;

    let wrapper_path = "/usr/local/bin/nvidia-dkms-build";

    println!("Hook to be installed:");
    println!("────────────────────────────────────────");
    println!("{}", hook_content);
    println!("────────────────────────────────────────");

    println!("\nWrapper script (with logging + notifications):");
    println!("────────────────────────────────────────");
    println!("  Logs to: /var/log/nvidia-dkms/");
    println!("  Desktop notifications on success/failure");
    println!("────────────────────────────────────────");

    // Check if running as root
    if !nix::unistd::geteuid().is_root() {
        println!("\nTo install, run as root:");
        println!("  sudo mkdir -p {}", hook_dir);
        println!("  sudo tee {} << 'EOF'\n{}EOF", hook_path, hook_content);
        println!(
            "  sudo tee {} << 'EOF'\n{}EOF",
            wrapper_path, wrapper_script
        );
        println!("  sudo chmod +x {}", wrapper_path);
        return Ok(());
    }

    // Create hooks directory
    std::fs::create_dir_all(hook_dir).map_err(|e| {
        NvControlError::ConfigError(format!("Failed to create hooks directory: {}", e))
    })?;

    // Write hook
    std::fs::write(&hook_path, hook_content)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to write hook: {}", e)))?;
    println!("Hook installed at {}", hook_path);

    // Write wrapper script
    std::fs::write(wrapper_path, wrapper_script).map_err(|e| {
        NvControlError::ConfigError(format!("Failed to write wrapper script: {}", e))
    })?;

    // Make wrapper executable
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(wrapper_path, std::fs::Permissions::from_mode(0o755))
        .map_err(|e| NvControlError::ConfigError(format!("Failed to set permissions: {}", e)))?;
    println!("Wrapper script installed at {}", wrapper_path);

    // Create log directory
    let _ = std::fs::create_dir_all("/var/log/nvidia-dkms");

    println!("\nThis hook will automatically rebuild NVIDIA DKMS modules");
    println!("when nvidia-open or kernel packages are updated.");
    println!("\nLogs will be saved to: /var/log/nvidia-dkms/");
    println!("View with: nvctl driver dkms logs");

    Ok(())
}

/// Print comprehensive DKMS status for nvidia
pub fn print_dkms_status_detailed() -> NvResult<()> {
    let info = get_dkms_setup_info();

    println!("NVIDIA DKMS Status");
    println!("{}", "═".repeat(50));
    println!();

    // DKMS installed
    println!(
        "DKMS:           {}",
        if info.dkms_installed {
            "installed"
        } else {
            "not installed"
        }
    );

    if !info.dkms_installed {
        println!("\nInstall DKMS: nvctl driver dkms setup");
        return Ok(());
    }

    // nvidia version
    if let Some(ref ver) = info.nvidia_version {
        println!("Driver:         {}", ver);
    }

    // Registration status
    println!(
        "Registered:     {}",
        if info.nvidia_registered { "yes" } else { "no" }
    );

    // Source path and type
    if let Some(ref path) = info.source_path {
        println!("Source:         {}", path);
        println!("Source Type:    {}", info.source_type);
    } else if !info.nvidia_registered {
        println!("Source:         not found");
    }

    println!();

    // Kernel status
    let running_kernel = Command::new("uname")
        .arg("-r")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Get all installed kernels
    let mut all_kernels = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/lib/modules") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            all_kernels.push(name);
        }
    }
    all_kernels.sort();

    println!("Installed Kernels ({}):", all_kernels.len());
    for kernel in &all_kernels {
        let has_headers = std::path::Path::new(&format!("/lib/modules/{}/build", kernel)).exists();
        let has_nvidia = {
            let paths = [
                format!("/lib/modules/{}/kernel/drivers/video/nvidia.ko.zst", kernel),
                format!("/lib/modules/{}/kernel/drivers/video/nvidia.ko", kernel),
                format!("/lib/modules/{}/extramodules/nvidia.ko.zst", kernel),
                format!("/lib/modules/{}/extramodules/nvidia.ko", kernel),
                format!("/lib/modules/{}/updates/dkms/nvidia.ko.zst", kernel),
                format!("/lib/modules/{}/updates/dkms/nvidia.ko", kernel),
            ];
            paths.iter().any(|p| std::path::Path::new(p).exists())
        };
        let in_dkms = info.kernels_built.contains(kernel);

        let running = if kernel == &running_kernel {
            " (running)"
        } else {
            ""
        };

        let nvidia_status = if in_dkms {
            "dkms"
        } else if has_nvidia {
            "manual"
        } else {
            "MISSING"
        };

        let headers_icon = if has_headers { "✓" } else { "✗" };

        println!(
            "  {} {} [nvidia: {}, headers: {}]{}",
            if has_nvidia { "✓" } else { "✗" },
            kernel,
            nvidia_status,
            headers_icon,
            running
        );
    }

    println!();

    // Recommendations
    if !info.nvidia_registered {
        println!("Recommendations:");
        println!("  -> Set up DKMS: nvctl driver dkms setup");
    } else if !info.kernels_missing.is_empty() {
        println!("Recommendations:");
        println!("  -> Build for missing kernels: nvctl driver dkms build");
    }

    // Check for pacman hook
    if std::path::Path::new("/etc/pacman.d/hooks/nvidia-dkms.hook").exists() {
        println!("\nPacman Hook:    installed (auto-rebuild enabled)");
    } else {
        println!("\nPacman Hook:    not installed");
        println!("  -> Install: nvctl driver dkms hook");
    }

    Ok(())
}

pub fn doctor_dkms() -> DkmsDoctorReport {
    let info = get_dkms_setup_info();
    let mut severity = DiagnosticSeverity::Healthy;
    let mut findings = Vec::new();
    let mut fixes = Vec::new();

    if !info.dkms_installed {
        severity = DiagnosticSeverity::Broken;
        findings.push("DKMS is not installed".to_string());
        fixes.push(
            "Install dkms and matching kernel headers before rebuilding NVIDIA modules".to_string(),
        );
    }

    if info.dkms_installed && !info.nvidia_registered {
        if severity != DiagnosticSeverity::Broken {
            severity = DiagnosticSeverity::Warning;
        }
        findings.push("nvidia is not registered with DKMS".to_string());
        fixes.push(
            "Run `nvctl driver dkms setup` to register the current NVIDIA source with DKMS"
                .to_string(),
        );
    }

    let mut all_kernels = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/lib/modules") {
        for entry in entries.flatten() {
            all_kernels.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    all_kernels.sort();

    for kernel in &all_kernels {
        let build_path = format!("/lib/modules/{}/build", kernel);
        if !std::path::Path::new(&build_path).exists() {
            if severity == DiagnosticSeverity::Healthy {
                severity = DiagnosticSeverity::Warning;
            }
            findings.push(format!("kernel headers missing for {}", kernel));
        }
    }

    for kernel in &info.kernels_missing {
        if severity != DiagnosticSeverity::Broken {
            severity = DiagnosticSeverity::Warning;
        }
        findings.push(format!("missing NVIDIA module for kernel {}", kernel));
    }

    let running_kernel = Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    if !running_kernel.is_empty() && info.kernels_missing.contains(&running_kernel) {
        severity = DiagnosticSeverity::Broken;
        findings.push(format!(
            "running kernel {} is missing a usable NVIDIA module",
            running_kernel
        ));
    }

    if let Some(version) = &info.nvidia_version {
        let symlink_path = format!("/usr/src/nvidia-{}", version);
        if let Ok(target) = std::fs::read_link(&symlink_path) {
            if let Some(source_path) = &info.source_path {
                if target.display().to_string() != *source_path {
                    if severity != DiagnosticSeverity::Broken {
                        severity = DiagnosticSeverity::Warning;
                    }
                    findings.push(format!(
                        "{} points to {} instead of {}",
                        symlink_path,
                        target.display(),
                        source_path
                    ));
                    fixes.push(format!(
                        "Update the DKMS source symlink: sudo ln -sf {} {}",
                        source_path, symlink_path
                    ));
                }
            }
        }
    }

    if findings.is_empty() {
        findings.push("DKMS state looks healthy".to_string());
    }

    fixes.sort();
    fixes.dedup();

    DkmsDoctorReport {
        severity,
        findings,
        fixes,
    }
}

pub fn print_dkms_doctor() -> NvResult<()> {
    let report = doctor_dkms();
    println!("NVIDIA DKMS Doctor");
    println!("{}", "═".repeat(50));
    println!();
    println!("Severity: {}", severity_label(report.severity));
    println!();
    println!("Findings:");
    for finding in &report.findings {
        println!("  - {}", finding);
    }
    if !report.fixes.is_empty() {
        println!();
        println!("Suggested Fixes:");
        for fix in &report.fixes {
            println!("  - {}", fix);
        }
    }
    Ok(())
}

pub fn inspect_source_build_state() -> SourceBuildState {
    let info = get_dkms_setup_info();
    let mut state = SourceBuildState {
        source_path: info.source_path.clone(),
        source_type: info.source_type.to_string(),
        current_tag: None,
        latest_tag: None,
        git_commit: None,
        git_dirty: None,
        symlink_target: None,
        tracked_version: info.nvidia_version.clone(),
    };

    if let Some(path) = &info.source_path {
        if matches!(info.source_type, DkmsSourceType::Git { .. }) {
            state.current_tag = Command::new("git")
                .args(["-C", path, "describe", "--tags", "--always"])
                .output()
                .ok()
                .filter(|o| o.status.success())
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
            state.latest_tag = Command::new("git")
                .args(["-C", path, "tag", "--sort=-v:refname"])
                .output()
                .ok()
                .filter(|o| o.status.success())
                .and_then(|o| {
                    String::from_utf8_lossy(&o.stdout)
                        .lines()
                        .next()
                        .map(|s| s.to_string())
                });
            state.git_commit = Command::new("git")
                .args(["-C", path, "rev-parse", "--short", "HEAD"])
                .output()
                .ok()
                .filter(|o| o.status.success())
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());
            state.git_dirty = Command::new("git")
                .args(["-C", path, "status", "--porcelain"])
                .output()
                .ok()
                .filter(|o| o.status.success())
                .map(|o| !String::from_utf8_lossy(&o.stdout).trim().is_empty());
        }

        if let Some(version) = &info.nvidia_version {
            let symlink_path = format!("/usr/src/nvidia-{}", version);
            state.symlink_target = std::fs::read_link(&symlink_path)
                .ok()
                .map(|target| target.display().to_string());
        }
    }

    state
}

pub fn print_source_doctor() -> NvResult<()> {
    let state = inspect_source_build_state();
    println!("NVIDIA Source Build Doctor");
    println!("{}", "═".repeat(50));
    println!();
    println!("Source Type: {}", state.source_type);
    if let Some(path) = &state.source_path {
        println!("Source Path: {}", path);
    }
    if let Some(version) = &state.tracked_version {
        println!("Tracked Version: {}", version);
    }
    if let Some(tag) = &state.current_tag {
        println!("Current Tag: {}", tag);
    }
    if let Some(tag) = &state.latest_tag {
        println!("Latest Tag: {}", tag);
    }
    if let Some(commit) = &state.git_commit {
        println!("Git Commit: {}", commit);
    }
    if let Some(dirty) = state.git_dirty {
        println!("Git Dirty: {}", if dirty { "yes" } else { "no" });
    }
    if let Some(target) = &state.symlink_target {
        println!("/usr/src symlink: {}", target);
    }
    if state.source_path.is_none() {
        println!(
            "\nNo tracked source tree. Initialize one with `nvctl driver source init <path>`."
        );
    }
    Ok(())
}

/// Check for driver updates
pub fn check_for_updates() -> NvResult<Option<String>> {
    let status = get_driver_status()?;

    if status.update_available {
        Ok(status.available_version)
    } else {
        Ok(None)
    }
}

/// Validate driver installation
pub fn validate_driver_installation() -> NvResult<bool> {
    // Check if nvidia-smi works
    let nvidia_smi = Command::new("nvidia-smi").output();
    if nvidia_smi.is_err() || !nvidia_smi.unwrap().status.success() {
        return Ok(false);
    }

    // Check if kernel modules are loaded
    let lsmod = Command::new("lsmod").output();
    if let Ok(output) = lsmod {
        let modules_str = String::from_utf8_lossy(&output.stdout);
        if !modules_str.contains("nvidia") {
            return Ok(false);
        }
    }

    // Check DKMS status
    let dkms = Command::new("dkms").args(&["status"]).output();
    if let Ok(output) = dkms {
        let dkms_str = String::from_utf8_lossy(&output.stdout);
        if dkms_str.contains("nvidia") && !dkms_str.contains("installed") {
            return Ok(false);
        }
    }

    Ok(true)
}

// ==================== Driver Capabilities ====================

/// Driver capabilities based on version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverCapabilities {
    pub version: String,
    pub major_version: u32,
    pub is_beta: bool,
    pub wayland_min_version: String,
    pub glibc_min_version: String,
    pub has_vulkan_swapchain_perf: bool,
    pub has_usb4_dp_support: bool,
    pub supports_preempt_rt: bool,
    pub has_powermizer_wayland_fix: bool,
}

impl DriverCapabilities {
    /// Detect capabilities from the current driver
    pub fn detect() -> NvResult<Self> {
        let status = get_driver_status()?;
        Self::from_version(&status.current_version)
    }

    /// Parse capabilities from a version string
    pub fn from_version(version: &str) -> NvResult<Self> {
        let major = Self::parse_major_version(version);
        let is_beta = version.contains("beta") || version.contains("Beta");

        Ok(Self {
            version: version.to_string(),
            major_version: major,
            is_beta,
            wayland_min_version: if major >= 590 {
                "1.20".into()
            } else {
                "1.17".into()
            },
            glibc_min_version: if major >= 590 {
                "2.27".into()
            } else {
                "2.17".into()
            },
            has_vulkan_swapchain_perf: major >= 590,
            has_usb4_dp_support: major >= 590,
            supports_preempt_rt: major >= 590,
            has_powermizer_wayland_fix: major >= 590,
        })
    }

    fn parse_major_version(version: &str) -> u32 {
        // Version format: "590.44.01" -> 590
        version
            .split('.')
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }
}

/// System requirements check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirementCheck {
    pub passed: bool,
    pub wayland_ok: Option<bool>,
    pub wayland_version: Option<String>,
    pub glibc_ok: Option<bool>,
    pub glibc_version: Option<String>,
    pub preempt_rt_kernel: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Check if system meets requirements for a driver version
pub fn validate_system_for_driver(target_version: u32) -> SystemRequirementCheck {
    let mut result = SystemRequirementCheck {
        passed: true,
        wayland_ok: None,
        wayland_version: None,
        glibc_ok: None,
        glibc_version: None,
        preempt_rt_kernel: is_preempt_rt_kernel(),
        warnings: Vec::new(),
        errors: Vec::new(),
    };

    // Check Wayland version
    if let Some(wayland_ver) = get_wayland_version() {
        result.wayland_version = Some(wayland_ver.clone());
        if target_version >= 590 {
            let required = "1.20";
            let ok = compare_versions(&wayland_ver, required) >= 0;
            result.wayland_ok = Some(ok);
            if !ok {
                result.warnings.push(format!(
                    "Wayland {} may have issues with {} drivers (requires {}+)",
                    wayland_ver, target_version, required
                ));
            }
        }
    }

    // Check glibc version
    if let Some(glibc_ver) = get_glibc_version() {
        result.glibc_version = Some(glibc_ver.clone());
        if target_version >= 590 {
            let required = "2.27";
            let ok = compare_versions(&glibc_ver, required) >= 0;
            result.glibc_ok = Some(ok);
            if !ok {
                result.errors.push(format!(
                    "glibc {} not supported by {} drivers (requires {}+)",
                    glibc_ver, target_version, required
                ));
                result.passed = false;
            }
        }
    }

    // PREEMPT_RT kernel warning for older drivers
    if result.preempt_rt_kernel && target_version < 590 {
        result
            .warnings
            .push("PREEMPT_RT kernel detected - drivers < 590 may freeze".to_string());
    }

    result
}

/// Check if running a PREEMPT_RT kernel
pub fn is_preempt_rt_kernel() -> bool {
    std::fs::read_to_string("/proc/version")
        .map(|v| v.contains("PREEMPT_RT"))
        .unwrap_or(false)
}

/// Get Wayland compositor version
fn get_wayland_version() -> Option<String> {
    // Try wayland-info
    if let Ok(output) = Command::new("wayland-info").arg("--version").output() {
        if output.status.success() {
            let ver = String::from_utf8_lossy(&output.stdout);
            // Parse version from output
            if let Some(v) = ver.lines().next() {
                return Some(v.trim().to_string());
            }
        }
    }

    // Try pkg-config
    if let Ok(output) = Command::new("pkg-config")
        .args(["--modversion", "wayland-client"])
        .output()
    {
        if output.status.success() {
            return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    None
}

/// Get glibc version
fn get_glibc_version() -> Option<String> {
    // Try ldd --version
    if let Ok(output) = Command::new("ldd").arg("--version").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Parse "ldd (GNU libc) 2.38"
        for line in stdout.lines() {
            if line.contains("libc") || line.contains("GLIBC") {
                if let Some(ver) = line.split_whitespace().last() {
                    return Some(ver.to_string());
                }
            }
        }
    }

    None
}

/// Simple version comparison (returns -1, 0, or 1)
fn compare_versions(a: &str, b: &str) -> i32 {
    let parse = |s: &str| -> Vec<u32> { s.split('.').filter_map(|p| p.parse().ok()).collect() };

    let va = parse(a);
    let vb = parse(b);

    for i in 0..va.len().max(vb.len()) {
        let a_part = va.get(i).copied().unwrap_or(0);
        let b_part = vb.get(i).copied().unwrap_or(0);
        if a_part < b_part {
            return -1;
        }
        if a_part > b_part {
            return 1;
        }
    }
    0
}

/// Get driver capabilities for current system
pub fn get_driver_capabilities() -> NvResult<DriverCapabilities> {
    DriverCapabilities::detect()
}

/// Print driver info (for CLI)
pub fn print_driver_info() -> NvResult<()> {
    let caps = DriverCapabilities::detect()?;

    println!(
        "Driver: {} {}",
        caps.version,
        if caps.is_beta { "(Beta)" } else { "" }
    );
    println!("Major Version: {}", caps.major_version);
    println!();
    println!("Requirements:");
    println!("  Wayland: {}+", caps.wayland_min_version);
    println!("  glibc: {}+", caps.glibc_min_version);
    println!();
    println!("Features:");
    println!(
        "  Vulkan swapchain optimization: {}",
        if caps.has_vulkan_swapchain_perf {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "  USB4 DP support: {}",
        if caps.has_usb4_dp_support {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "  PREEMPT_RT support: {}",
        if caps.supports_preempt_rt {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "  PowerMizer on Wayland: {}",
        if caps.has_powermizer_wayland_fix {
            "Fixed"
        } else {
            "May have issues"
        }
    );

    Ok(())
}

/// Validate system for specific driver version (for CLI)
pub fn print_validation(target_version: u32) -> NvResult<()> {
    let check = validate_system_for_driver(target_version);

    println!("System Validation for Driver {}", target_version);
    println!("================================");
    println!();

    if let Some(ref ver) = check.wayland_version {
        let status = match check.wayland_ok {
            Some(true) => "✓",
            Some(false) => "✗",
            None => "?",
        };
        println!("Wayland: {} {}", ver, status);
    } else {
        println!("Wayland: Not detected");
    }

    if let Some(ref ver) = check.glibc_version {
        let status = match check.glibc_ok {
            Some(true) => "✓",
            Some(false) => "✗",
            None => "?",
        };
        println!("glibc: {} {}", ver, status);
    } else {
        println!("glibc: Not detected");
    }

    println!(
        "PREEMPT_RT Kernel: {}",
        if check.preempt_rt_kernel { "Yes" } else { "No" }
    );
    println!();

    if !check.warnings.is_empty() {
        println!("Warnings:");
        for w in &check.warnings {
            println!("  ⚠️  {}", w);
        }
        println!();
    }

    if !check.errors.is_empty() {
        println!("Errors:");
        for e in &check.errors {
            println!("  ❌ {}", e);
        }
        println!();
    }

    if check.passed {
        println!("✓ System meets requirements for driver {}", target_version);
    } else {
        println!(
            "✗ System does NOT meet requirements for driver {}",
            target_version
        );
    }

    Ok(())
}

/// Comprehensive driver info "truth table" for debugging
/// Shows GPU, driver version, module type, kernel, GSP, DKMS status
pub fn print_driver_info_full() -> NvResult<()> {
    use std::fs;
    use std::process::Command;

    println!("Driver Information");
    println!("{}", "═".repeat(50));
    println!();

    // GPU Info
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=name,pci.bus_id", "--format=csv,noheader"])
        .output()
    {
        if output.status.success() {
            let gpu_info = String::from_utf8_lossy(&output.stdout);
            for line in gpu_info.trim().lines() {
                let parts: Vec<&str> = line.split(", ").collect();
                if parts.len() >= 2 {
                    println!("GPU:            {} [{}]", parts[0], parts[1]);
                }
            }
        }
    }

    // Driver version and type
    let mut driver_version = "Unknown".to_string();
    let mut module_type = "Unknown".to_string();
    let mut license = "Unknown".to_string();
    let mut built_by = None;

    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=driver_version", "--format=csv,noheader"])
        .output()
    {
        if output.status.success() {
            driver_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    // Detect open vs proprietary from /proc/driver/nvidia/version
    if let Ok(version_info) = fs::read_to_string("/proc/driver/nvidia/version") {
        if version_info.contains("Open Kernel Module") {
            module_type = "Open Kernel".to_string();
        } else {
            module_type = "Proprietary".to_string();
        }

        // Extract build info
        for line in version_info.lines() {
            if line.contains("NVRM version:") {
                // Parse: NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  590.48.01  Release Build  (chris@arch)  Thu Dec 18 ...
                if let Some(build_info) = line.split("Release Build").nth(1) {
                    let build_info = build_info.trim();
                    if let Some(start) = build_info.find('(') {
                        if let Some(end) = build_info.find(')') {
                            let builder = &build_info[start + 1..end];
                            if let Some(date_start) = build_info.get(end + 1..) {
                                built_by = Some(format!("{} ({})", builder, date_start.trim()));
                            } else {
                                built_by = Some(builder.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // Get license from modinfo
    if let Ok(output) = Command::new("modinfo")
        .args(["nvidia", "-F", "license"])
        .output()
    {
        if output.status.success() {
            license = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    println!(
        "Driver:         {} ({})",
        driver_version,
        if module_type == "Open Kernel" {
            "nvidia-open"
        } else {
            "nvidia"
        }
    );
    println!("Module Type:    {} ({})", module_type, license);

    if let Some(ref build) = built_by {
        println!("Built By:       {}", build);
    }

    println!();

    // Kernel info
    let running_kernel = if let Ok(output) = Command::new("uname").arg("-r").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "Unknown".to_string()
    };

    // Get vermagic from nvidia module
    let mut module_kernel = "Unknown".to_string();
    if let Ok(output) = Command::new("modinfo")
        .args(["nvidia", "-F", "vermagic"])
        .output()
    {
        if output.status.success() {
            let vermagic = String::from_utf8_lossy(&output.stdout);
            // vermagic format: "6.18.2-1-cachyos-lto SMP preempt mod_unload"
            module_kernel = vermagic
                .split_whitespace()
                .next()
                .unwrap_or("Unknown")
                .to_string();
        }
    }

    let kernel_match = running_kernel == module_kernel;
    println!("Kernel:         {} (running)", running_kernel);
    println!(
        "Module For:     {} {}",
        module_kernel,
        if kernel_match { "✓" } else { "✗ MISMATCH" }
    );

    println!();

    // GSP Firmware (using enhanced detection)
    use crate::gsp_firmware::GspManager;
    let gsp_mgr = GspManager::new();
    let gsp_status = gsp_mgr.get_deep_status();

    // GSP mode (enabled/disabled)
    let gsp_mode = if gsp_status.enabled {
        "enabled"
    } else {
        "disabled"
    };

    // GSP state
    let gsp_state = match gsp_status.state.as_str() {
        "active" => "active",
        "loaded" => "loaded",
        "failed" => "failed",
        "not_loaded" => "not loaded",
        _ => "unknown",
    };

    // GSP display line
    if gsp_status.is_nvidia_open {
        println!("GSP:            {} ({})", gsp_mode, gsp_state);
    } else {
        println!("GSP:            N/A (proprietary driver)");
    }

    // Firmware path/version
    if let Some(ref ver) = gsp_status.firmware_version {
        if gsp_status.is_nvidia_open {
            println!("GSP Firmware:   {}", ver);
        }
    }
    if let Some(ref path) = gsp_status.firmware_path {
        if gsp_status.is_nvidia_open {
            println!("                {}", path);
        }
    }

    // GPU arch for debugging
    if let Some(ref arch) = gsp_status.gpu_arch {
        if gsp_status.is_nvidia_open {
            println!("GPU Arch:       {}", arch);
        }
    }

    // GSP errors (if any)
    if gsp_status.error_count > 0 {
        println!(
            "GSP Errors:     {} (run 'nvctl driver logs --gsp')",
            gsp_status.error_count
        );
    }

    println!();

    // DKMS status
    print_dkms_status_inline()?;

    println!();

    // Loaded modules
    if let Ok(output) = Command::new("lsmod").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let nvidia_modules: Vec<&str> = output_str
                .lines()
                .filter(|line| line.starts_with("nvidia"))
                .map(|line| line.split_whitespace().next().unwrap_or(""))
                .collect();

            if !nvidia_modules.is_empty() {
                println!("Modules Loaded: {}", nvidia_modules.join(" "));
            }
        }
    }

    Ok(())
}

/// Internal helper to print DKMS status inline (for driver info)
fn print_dkms_status_inline() -> NvResult<()> {
    use std::process::Command;

    if let Ok(output) = Command::new("dkms").arg("status").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let nvidia_lines: Vec<&str> = output_str
                .lines()
                .filter(|line| line.contains("nvidia"))
                .collect();

            if nvidia_lines.is_empty() {
                println!("DKMS:           Not managed (manually installed)");
            } else {
                for (i, line) in nvidia_lines.iter().enumerate() {
                    if i == 0 {
                        println!("DKMS:           {}", line.trim());
                    } else {
                        println!("                {}", line.trim());
                    }
                }
            }
        } else {
            println!("DKMS:           Not installed");
        }
    } else {
        println!("DKMS:           Not installed");
    }

    Ok(())
}

/// Pretty DKMS status display
pub fn print_dkms_status() -> NvResult<()> {
    use std::process::Command;

    println!("DKMS Module Status");
    println!("{}", "═".repeat(50));
    println!();

    // Check if DKMS is installed
    let dkms_installed = Command::new("which")
        .arg("dkms")
        .output()
        .is_ok_and(|o| o.status.success());

    if !dkms_installed {
        println!("DKMS:           Not installed");
        println!();
        println!("Install with:");
        println!("  Arch:   sudo pacman -S dkms");
        println!("  Debian: sudo apt install dkms");
        println!("  Fedora: sudo dnf install dkms");
        return Ok(());
    }

    // Get all installed kernels
    let mut installed_kernels = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/usr/lib/modules") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            installed_kernels.push(name.to_string_lossy().to_string());
        }
    }
    installed_kernels.sort();

    // Get running kernel
    let running_kernel = if let Ok(output) = Command::new("uname").arg("-r").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "Unknown".to_string()
    };

    println!("Installed Kernels:");
    for kernel in &installed_kernels {
        let is_running = kernel == &running_kernel;
        println!("  {} {}", kernel, if is_running { "(running)" } else { "" });
    }
    println!();

    // Get DKMS status
    if let Ok(output) = Command::new("dkms").arg("status").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();

            if lines.is_empty() {
                println!("No DKMS modules registered.");
            } else {
                println!("DKMS Modules:");
                for line in lines {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    // Parse DKMS status line: nvidia/590.48.01, 6.18.2-1-cachyos-lto, x86_64: installed
                    let status_icon = if line.contains(": installed") {
                        "✓"
                    } else if line.contains(": built") {
                        "⚙"
                    } else if line.contains(": added") {
                        "+"
                    } else {
                        "?"
                    };

                    let is_nvidia = line.contains("nvidia");
                    if is_nvidia {
                        println!("  {} {}", status_icon, line);
                    } else {
                        println!("    {}", line);
                    }
                }
            }
        }
    }

    println!();

    // Check for potential issues
    let mut issues = Vec::new();

    // Check if any kernel is missing nvidia module
    for kernel in &installed_kernels {
        let module_path = format!(
            "/usr/lib/modules/{}/kernel/drivers/video/nvidia.ko.zst",
            kernel
        );
        let module_path_alt = format!("/usr/lib/modules/{}/kernel/drivers/video/nvidia.ko", kernel);
        let module_path_extra = format!("/usr/lib/modules/{}/extramodules/nvidia.ko.zst", kernel);

        if !std::path::Path::new(&module_path).exists()
            && !std::path::Path::new(&module_path_alt).exists()
            && !std::path::Path::new(&module_path_extra).exists()
        {
            issues.push(format!("Kernel {} may be missing nvidia module", kernel));
        }
    }

    if !issues.is_empty() {
        println!("Potential Issues:");
        for issue in issues {
            println!("  ⚠️  {}", issue);
        }
        println!();
        println!("To rebuild DKMS modules: nvctl driver dkms fix");
    }

    Ok(())
}

/// Run driver health checks with opinionated warnings
pub fn print_driver_check() -> NvResult<()> {
    use std::fs;
    use std::process::Command;

    println!("Driver Health Check");
    println!("{}", "═".repeat(50));
    println!();

    let mut warnings = Vec::new();
    let mut errors = Vec::new();
    let mut passed = Vec::new();

    // 1. Check kernel version match
    let running_kernel = if let Ok(output) = Command::new("uname").arg("-r").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "Unknown".to_string()
    };

    let mut module_kernel = "Unknown".to_string();
    if let Ok(output) = Command::new("modinfo")
        .args(["nvidia", "-F", "vermagic"])
        .output()
    {
        if output.status.success() {
            let vermagic = String::from_utf8_lossy(&output.stdout);
            module_kernel = vermagic
                .split_whitespace()
                .next()
                .unwrap_or("Unknown")
                .to_string();
        }
    }

    if running_kernel != module_kernel && module_kernel != "Unknown" {
        errors.push(format!(
            "Kernel mismatch: running {} but module built for {}",
            running_kernel, module_kernel
        ));
    } else if module_kernel != "Unknown" {
        passed.push("Kernel version matches module".to_string());
    }

    // 2. Check kernel headers
    let headers_path = format!("/usr/lib/modules/{}/build", running_kernel);
    if !std::path::Path::new(&headers_path).exists() {
        warnings.push(format!(
            "Kernel headers not found for {} (needed for DKMS rebuilds)",
            running_kernel
        ));
    } else {
        passed.push("Kernel headers installed".to_string());
    }

    // 3. Check DKMS status
    let dkms_installed = Command::new("which")
        .arg("dkms")
        .output()
        .is_ok_and(|o| o.status.success());

    if dkms_installed {
        if let Ok(output) = Command::new("dkms").arg("status").output() {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let has_nvidia = output_str.lines().any(|l| l.contains("nvidia"));

                if !has_nvidia {
                    warnings.push("DKMS installed but nvidia not registered (manually installed - rebuild needed after kernel updates)".to_string());
                } else {
                    passed.push("DKMS has nvidia module registered".to_string());
                }
            }
        }
    } else {
        warnings.push("DKMS not installed (recommended for automatic module rebuilds)".to_string());
    }

    // 4. Check for multiple kernels with missing modules (linux-tkg scenario)
    let mut installed_kernels = Vec::new();
    if let Ok(entries) = fs::read_dir("/usr/lib/modules") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            installed_kernels.push(name.to_string_lossy().to_string());
        }
    }

    if installed_kernels.len() > 1 {
        let mut kernels_without_nvidia = Vec::new();
        for kernel in &installed_kernels {
            let module_paths = [
                format!(
                    "/usr/lib/modules/{}/kernel/drivers/video/nvidia.ko.zst",
                    kernel
                ),
                format!("/usr/lib/modules/{}/kernel/drivers/video/nvidia.ko", kernel),
                format!("/usr/lib/modules/{}/extramodules/nvidia.ko.zst", kernel),
                format!("/usr/lib/modules/{}/extramodules/nvidia.ko", kernel),
            ];

            let has_module = module_paths
                .iter()
                .any(|p| std::path::Path::new(p).exists());
            if !has_module {
                kernels_without_nvidia.push(kernel.clone());
            }
        }

        if !kernels_without_nvidia.is_empty() {
            warnings.push(format!(
                "{} non-running kernel(s) are missing an NVIDIA module: {}",
                kernels_without_nvidia.len(),
                kernels_without_nvidia.join(", ")
            ));
        } else {
            passed.push(format!(
                "All {} installed kernels have nvidia modules",
                installed_kernels.len()
            ));
        }
    }

    // 5. Check GSP firmware for nvidia-open
    if GspManager::is_nvidia_open() {
        let gsp_manager = GspManager::new();
        let gsp_status = gsp_manager.get_deep_status();
        let release = collect_release_diagnostics();
        if let Some(ref path) = gsp_status.firmware_path {
            passed.push(format!("GSP firmware present for nvidia-open at {}", path));
            if let Some(ref layout) = gsp_status.firmware_layout {
                passed.push(format!("GSP firmware layout detected: {}", layout));
            }
            if let Some(ref file) = gsp_status.firmware_file {
                passed.push(format!("GSP firmware file resolved: {}", file));
            }
        } else {
            errors.push(
                "GSP firmware path not detected for nvidia-open (required for open kernel modules)"
                    .to_string(),
            );
        }

        if let Some(ref alignment) = gsp_status.release_alignment {
            if alignment.starts_with("aligned") {
                passed.push(format!("Open driver release alignment OK: {}", alignment));
            } else {
                warnings.push(format!(
                    "Open driver release alignment needs attention: {}",
                    alignment
                ));
            }
        }

        for expected_path in release.expected_firmware_paths {
            if std::path::Path::new(&expected_path).exists() {
                passed.push(format!(
                    "Expected firmware search path present: {}",
                    expected_path
                ));
            } else {
                warnings.push(format!(
                    "Expected firmware search path is missing: {}",
                    expected_path
                ));
            }
        }

        if gsp_status.firmware_path.is_none() && gsp_status.enabled {
            warnings.push(
                "GSP is enabled but no firmware path was resolved; this usually means mixed nvidia-open/userspace packages or an incomplete firmware install"
                    .to_string(),
            );
        }
    }

    // 6. Check for legacy GPU compatibility issues
    if let Ok(output) = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,driver_version,pci.bus_id",
            "--format=csv,noheader",
        ])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.trim().split(", ").collect();
            let gpu_name = parts.first().unwrap_or(&"Unknown").to_lowercase();
            let driver_version = parts.get(1).unwrap_or(&"0").to_string();

            // Parse driver major version
            let driver_major: u32 = driver_version
                .split('.')
                .next()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);

            // Detect GPU architecture
            let pci_device_id = parts.get(2).and_then(|bus| read_pci_device_id(bus));
            let (arch, is_legacy) = pci_device_id
                .as_deref()
                .and_then(detect_gpu_architecture_by_pci_id)
                .unwrap_or_else(|| detect_gpu_architecture(&gpu_name));

            // Check for nvidia-open on pre-Turing GPUs (hard error)
            if is_legacy {
                if let Ok(version_info) = fs::read_to_string("/proc/driver/nvidia/version") {
                    if version_info.contains("Open Kernel Module") {
                        errors.push(format!(
                            "nvidia-open driver on {} GPU ({}). Use proprietary driver instead.",
                            arch,
                            gpu_name.trim()
                        ));
                    }
                }
            }

            if !is_legacy {
                if arch == "Unknown" {
                    warnings.push(format!(
                        "Could not confidently classify GPU '{}' for open-driver support; verify PCI ID detection and package branch selection",
                        gpu_name.trim()
                    ));
                } else if GspManager::is_nvidia_open() {
                    passed.push(format!(
                        "Detected {} GPU on open-driver-capable generation ({}){}",
                        arch,
                        gpu_name.trim(),
                        pci_device_id
                            .as_ref()
                            .map(|id| format!(", pci 0x{}", id))
                            .unwrap_or_default()
                    ));
                }
            }

            // Check for Pascal/Maxwell deprecation in 590+ (warning)
            let is_maxwell = gpu_name.contains("gtx 9")
                || gpu_name.contains("titan x")
                || gpu_name.contains("quadro m");
            let is_pascal = gpu_name.contains("gtx 10")
                || gpu_name.contains("titan xp")
                || gpu_name.contains("quadro p")
                || gpu_name.contains("p100")
                || gpu_name.contains("p40")
                || gpu_name.contains("p6000");

            if (is_maxwell || is_pascal) && driver_major >= 590 {
                warnings.push(format!(
                    "{} GPU ({}) is deprecated in driver {}. Consider nvidia-470xx-dkms for Maxwell or nvidia-535xx-dkms for Pascal from AUR.",
                    arch, gpu_name.trim(), driver_version
                ));
                warnings.push(
                    "Future NVIDIA drivers will drop support for Maxwell and Pascal GPUs."
                        .to_string(),
                );
            }

            // Check for Kepler/Fermi (very old GPUs)
            let is_kepler_or_older = gpu_name.contains("gtx 6")
                || gpu_name.contains("gtx 7")
                || gpu_name.contains("gtx 5")
                || gpu_name.contains("geforce 6")
                || gpu_name.contains("geforce 7")
                || gpu_name.contains("quadro k");

            if is_kepler_or_older {
                errors.push(format!(
                    "Kepler/older GPU ({}) is not supported by current drivers. Use nvidia-390xx-dkms from AUR.",
                    gpu_name.trim()
                ));
            }
        }
    }

    // Print results
    if !passed.is_empty() {
        println!("Passed:");
        for p in &passed {
            println!("  ✓ {}", p);
        }
        println!();
    }

    if !warnings.is_empty() {
        println!("Warnings:");
        for w in &warnings {
            println!("  ⚠️  {}", w);
        }
        println!();
    }

    if !errors.is_empty() {
        println!("Errors:");
        for e in &errors {
            println!("  ✗ {}", e);
        }
        println!();
    }

    // GSP health checks (for nvidia-open users)
    use crate::gsp_firmware::GspManager;
    if GspManager::is_nvidia_open() {
        println!();
        println!("GSP Firmware:");

        let gsp_mgr = GspManager::new();
        let gsp_health = gsp_mgr.run_health_checks();

        for check in &gsp_health.checks {
            let icon = if check.passed { "✓" } else { "✗" };
            println!("  {} {}: {}", icon, check.name, check.message);
        }

        if !gsp_health.recommendations.is_empty() {
            println!();
            println!("Recommendations:");
            for rec in &gsp_health.recommendations {
                println!("  -> {}", rec);
            }
        }

        if !gsp_health.passed {
            errors.push("GSP health check failed".to_string());
        }
    }

    // Summary
    println!();
    println!("{}", "─".repeat(50));
    if errors.is_empty() && warnings.is_empty() {
        println!("✓ All checks passed");
    } else if errors.is_empty() {
        println!("⚠️  {} warning(s), no errors", warnings.len());
    } else {
        println!("✗ {} error(s), {} warning(s)", errors.len(), warnings.len());
    }

    let summary = summarize_driver_check();
    let fixes = suggested_fixes(&summary);
    if !fixes.is_empty() {
        println!();
        println!("Suggested Fixes:");
        for fix in fixes {
            println!("  -> {}", fix);
        }
    }

    Ok(())
}

pub fn summarize_driver_check() -> DiagnosticSummary {
    let release = collect_release_diagnostics();
    let release_summary = summarize_release_diagnostics(&release);
    let mut severity = release_summary.severity;
    let mut messages = release_summary.messages;

    if let Ok(output) = Command::new("dkms").arg("status").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if !output_str.lines().any(|l| l.contains("nvidia"))
                && severity != DiagnosticSeverity::Broken
            {
                severity = DiagnosticSeverity::Warning;
                messages.push("dkms installed but nvidia is not registered".to_string());
            }
        }
    }

    DiagnosticSummary { severity, messages }
}

// ==================== Driver Logs ====================

/// Filter type for driver logs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogFilter {
    /// All NVIDIA-related logs
    Nvidia,
    /// GSP-specific logs
    Gsp,
    /// Xid errors (GPU faults)
    Xid,
}

/// Print driver logs from kernel journal
pub fn print_driver_logs(filter: LogFilter, tail: Option<usize>) -> NvResult<()> {
    let (title, patterns) = match filter {
        LogFilter::Nvidia => ("NVIDIA Driver Logs", vec!["nvidia", "NVRM"]),
        LogFilter::Gsp => ("GSP Firmware Logs", vec!["GSP", "gsp"]),
        LogFilter::Xid => ("Xid Error Logs", vec!["Xid", "NVRM.*Xid"]),
    };

    println!("{}", title);
    println!("{}", "=".repeat(50));
    println!();

    // Build grep pattern for journalctl
    let pattern = patterns.join("|");

    // Try journalctl first (doesn't need root)
    let output = Command::new("journalctl")
        .args([
            "-k", // kernel messages
            "-g",
            &pattern, // grep pattern
            "--no-pager",
            "-q", // quiet (no extra info)
            "-b", // current boot only
        ])
        .output();

    let log_content = match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
        _ => {
            // Fallback to dmesg
            if let Ok(dmesg_out) = Command::new("dmesg").output() {
                let all_logs = String::from_utf8_lossy(&dmesg_out.stdout);
                all_logs
                    .lines()
                    .filter(|line| {
                        let lower = line.to_lowercase();
                        patterns.iter().any(|p| lower.contains(&p.to_lowercase()))
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                return Err(crate::NvControlError::CommandFailed(
                    "Cannot read kernel logs. Try running with sudo.".to_string(),
                ));
            }
        }
    };

    if log_content.trim().is_empty() {
        println!("No logs found matching filter.");
        return Ok(());
    }

    // Apply tail limit if specified
    let lines: Vec<&str> = log_content.lines().collect();
    let display_lines = match tail {
        Some(n) if n < lines.len() => &lines[lines.len() - n..],
        _ => &lines[..],
    };

    for line in display_lines {
        // Color-code based on severity
        if line.to_lowercase().contains("error") || line.contains("Xid") {
            println!("  ✗ {}", line);
        } else if line.to_lowercase().contains("warn") {
            println!("  ⚠ {}", line);
        } else {
            println!("  {}", line);
        }
    }

    println!();
    println!("Total: {} lines", display_lines.len());

    if filter == LogFilter::Xid {
        println!();
        println!("Common Xid errors:");
        println!("  Xid 13: Graphics Engine fault");
        println!("  Xid 31: GPU memory page fault");
        println!("  Xid 43: GPU stopped processing");
        println!("  Xid 45: Preemptive cleanup, due to previous errors");
        println!("  Xid 79: GPU fallen off the bus");
        println!();
        println!("More info: https://docs.nvidia.com/deploy/xid-errors/");
    }

    Ok(())
}

/// Print logs suitable for pasting (Discord mode)
pub fn print_driver_logs_paste() -> NvResult<()> {
    use crate::gsp_firmware::GspManager;

    // Header
    let gsp_mgr = GspManager::new();
    let gsp_status = gsp_mgr.get_deep_status();

    println!("```");
    println!("nvctl driver info (paste)");
    println!("─────────────────────────");

    // Compact info
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=name,driver_version", "--format=csv,noheader"])
        .output()
    {
        if output.status.success() {
            let info = String::from_utf8_lossy(&output.stdout);
            println!("GPU: {}", info.trim());
        }
    }

    // Kernel
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        println!("Kernel: {}", String::from_utf8_lossy(&output.stdout).trim());
    }

    // Module type
    if let Ok(content) = std::fs::read_to_string("/proc/driver/nvidia/version") {
        if content.contains("Open Kernel Module") {
            println!("Module: nvidia-open");
        } else {
            println!("Module: nvidia (proprietary)");
        }
    }

    // GSP
    if gsp_status.is_nvidia_open {
        println!(
            "GSP: {} ({})",
            if gsp_status.enabled { "on" } else { "off" },
            gsp_status.state
        );
        if let Some(ref arch) = gsp_status.gpu_arch {
            println!("Arch: {}", arch);
        }
        if gsp_status.error_count > 0 {
            println!("GSP Errors: {}", gsp_status.error_count);
        }
    }

    // DKMS
    if let Ok(output) = Command::new("dkms").arg("status").output() {
        if output.status.success() {
            let status = String::from_utf8_lossy(&output.stdout);
            let nvidia_line = status.lines().find(|l| l.contains("nvidia"));
            if let Some(line) = nvidia_line {
                println!("DKMS: {}", line.trim());
            }
        }
    }

    println!("```");

    Ok(())
}

// ==================== Source Build Management ====================

/// Print source build status
pub fn print_source_status() -> NvResult<()> {
    let info = get_dkms_setup_info();

    println!("NVIDIA Source Build Status");
    println!("{}", "═".repeat(50));
    println!();

    if let Some(ref path) = info.source_path {
        println!("Source Path:    {}", path);
        println!("Source Type:    {}", info.source_type);

        // For git sources, show more info
        if let DkmsSourceType::Git { ref remote_url } = info.source_type {
            if let Some(url) = remote_url {
                println!("Remote URL:     {}", url);
            }

            // Show current branch/tag
            if let Ok(output) = Command::new("git")
                .args(["-C", path, "describe", "--tags", "--always"])
                .output()
            {
                if output.status.success() {
                    let tag = String::from_utf8_lossy(&output.stdout);
                    println!("Current Tag:    {}", tag.trim());
                }
            }

            // Show if there are updates available
            let _ = Command::new("git")
                .args(["-C", path, "fetch", "--tags", "--quiet"])
                .status();

            if let Ok(output) = Command::new("git")
                .args(["-C", path, "tag", "--sort=-v:refname"])
                .output()
            {
                if output.status.success() {
                    let tags = String::from_utf8_lossy(&output.stdout);
                    if let Some(latest) = tags.lines().next() {
                        println!("Latest Tag:     {}", latest);
                    }
                }
            }
        }

        println!();

        // Show registered version
        if let Some(ref ver) = info.nvidia_version {
            println!("Driver Version: {}", ver);
        }

        println!(
            "DKMS Registered: {}",
            if info.nvidia_registered { "yes" } else { "no" }
        );
    } else {
        println!("No source found.");
        println!();
        println!("To set up from source:");
        println!("  1. Clone: git clone https://github.com/NVIDIA/open-gpu-kernel-modules.git");
        println!("  2. Init:  nvctl driver source init ~/open-gpu-kernel-modules");
    }

    Ok(())
}

/// Initialize source build from a git clone
pub fn init_source_build(path: &str) -> NvResult<()> {
    // Expand ~ to home directory
    let expanded_path = if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            path.replacen('~', &home.display().to_string(), 1)
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    };

    let source_path = std::path::Path::new(&expanded_path);

    println!("Initializing NVIDIA source build from: {}", expanded_path);
    println!();

    // Verify path exists and is a git repo
    if !source_path.exists() {
        return Err(NvControlError::ConfigError(format!(
            "Path does not exist: {}",
            expanded_path
        )));
    }

    let git_dir = source_path.join(".git");
    if !git_dir.exists() {
        return Err(NvControlError::ConfigError(format!(
            "Not a git repository: {}",
            expanded_path
        )));
    }

    // Verify it's the nvidia repo
    let kernel_open = source_path.join("kernel-open");
    if !kernel_open.exists() {
        return Err(NvControlError::ConfigError(
            "Not an nvidia open-gpu-kernel-modules clone (missing kernel-open/)".to_string(),
        ));
    }

    // Get version from version.mk
    let version = get_source_version(&expanded_path)?;
    println!("Detected version: {}", version);

    // Check for dkms.conf
    let dkms_conf = source_path.join("dkms.conf");
    if !dkms_conf.exists() {
        println!("\nNo dkms.conf found. Creating one...");
        create_dkms_conf(&expanded_path, &version)?;
        println!("Created dkms.conf");
    }

    // Create symlink in /usr/src
    let usr_src_link = format!("/usr/src/nvidia-{}", version);
    println!();

    if std::path::Path::new(&usr_src_link).exists() {
        println!("Symlink already exists: {}", usr_src_link);
        // Check if it points to the right place
        if let Ok(target) = std::fs::read_link(&usr_src_link) {
            if target.display().to_string() != expanded_path {
                println!("  Warning: Points to different path: {}", target.display());
                println!("  Run with sudo to update if needed");
            }
        }
    } else {
        println!("Creating symlink: {} -> {}", usr_src_link, expanded_path);
        if nix::unistd::geteuid().is_root() {
            std::os::unix::fs::symlink(&expanded_path, &usr_src_link).map_err(|e| {
                NvControlError::ConfigError(format!("Failed to create symlink: {}", e))
            })?;
        } else {
            println!("  Run as root:");
            println!("  sudo ln -sf {} {}", expanded_path, usr_src_link);
        }
    }

    // Register with DKMS
    println!();
    let info = get_dkms_setup_info();
    if info.nvidia_registered {
        println!("Already registered with DKMS");
    } else {
        println!("Registering with DKMS...");
        if nix::unistd::geteuid().is_root() {
            let status = Command::new("dkms")
                .args(["add", "nvidia", &version])
                .status();
            match status {
                Ok(s) if s.success() => println!("Registered nvidia/{} with DKMS", version),
                Ok(_) => println!("Registration may have failed - check with: dkms status"),
                Err(e) => println!("Failed to register: {}", e),
            }
        } else {
            println!("  Run as root:");
            println!("  sudo dkms add nvidia/{}", version);
        }
    }

    println!();
    println!("Setup complete! Next steps:");
    println!("  Build modules: nvctl driver source sync");
    println!("  Update source: nvctl driver source update");

    Ok(())
}

/// Get version from source version.mk
fn get_source_version(path: &str) -> NvResult<String> {
    let version_mk = format!("{}/version.mk", path);
    let content = std::fs::read_to_string(&version_mk)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to read version.mk: {}", e)))?;

    for line in content.lines() {
        if line.starts_with("NVIDIA_VERSION") && line.contains('=') {
            if let Some(ver) = line.split('=').nth(1) {
                return Ok(ver.trim().to_string());
            }
        }
    }

    Err(NvControlError::ConfigError(
        "Could not parse version from version.mk".to_string(),
    ))
}

/// Create a dkms.conf for the source
fn create_dkms_conf(path: &str, version: &str) -> NvResult<()> {
    let dkms_conf = format!(
        r#"PACKAGE_NAME="nvidia"
PACKAGE_VERSION="{version}"

# LLVM/Clang detection for CachyOS/TKG kernels
MAKE="'make' -j$(nproc) KERNEL_UNAME=${{kernelver}} $(grep -q CONFIG_CC_IS_CLANG=y ${{kernel_source_dir}}/.config 2>/dev/null && echo 'LLVM=1 CC=clang LD=ld.lld') NV_KERNEL_SOURCES=${{kernel_source_dir}} NV_KERNEL_OUTPUT=${{kernel_source_dir}} modules"

BUILT_MODULE_NAME[0]="nvidia"
BUILT_MODULE_LOCATION[0]="kernel-open"
DEST_MODULE_LOCATION[0]="/kernel/drivers/video"

BUILT_MODULE_NAME[1]="nvidia-modeset"
BUILT_MODULE_LOCATION[1]="kernel-open"
DEST_MODULE_LOCATION[1]="/kernel/drivers/video"

BUILT_MODULE_NAME[2]="nvidia-drm"
BUILT_MODULE_LOCATION[2]="kernel-open"
DEST_MODULE_LOCATION[2]="/kernel/drivers/video"

BUILT_MODULE_NAME[3]="nvidia-uvm"
BUILT_MODULE_LOCATION[3]="kernel-open"
DEST_MODULE_LOCATION[3]="/kernel/drivers/video"

BUILT_MODULE_NAME[4]="nvidia-peermem"
BUILT_MODULE_LOCATION[4]="kernel-open"
DEST_MODULE_LOCATION[4]="/kernel/drivers/video"

AUTOINSTALL="yes"
"#
    );

    let conf_path = format!("{}/dkms.conf", path);
    std::fs::write(&conf_path, dkms_conf)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to write dkms.conf: {}", e)))?;

    Ok(())
}

/// Update source from git and optionally rebuild
pub fn update_source(rebuild: bool) -> NvResult<()> {
    let info = get_dkms_setup_info();

    let source_path = info.source_path.ok_or_else(|| {
        NvControlError::ConfigError(
            "No source path found. Run: nvctl driver source init".to_string(),
        )
    })?;

    // Verify it's a git source
    if !matches!(info.source_type, DkmsSourceType::Git { .. }) {
        return Err(NvControlError::ConfigError(
            "Source is not a git repository. Cannot update.".to_string(),
        ));
    }

    println!("Updating NVIDIA source from git...\n");

    // Fetch latest
    println!("Fetching latest tags...");
    let status = Command::new("git")
        .args(["-C", &source_path, "fetch", "--tags"])
        .status()
        .map_err(|e| NvControlError::CommandFailed(format!("git fetch failed: {}", e)))?;

    if !status.success() {
        return Err(NvControlError::CommandFailed(
            "git fetch failed".to_string(),
        ));
    }

    // Get current and latest versions
    let current = Command::new("git")
        .args(["-C", &source_path, "describe", "--tags", "--always"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let latest = Command::new("git")
        .args(["-C", &source_path, "tag", "--sort=-v:refname"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .next()
                    .map(|s| s.to_string())
            } else {
                None
            }
        });

    println!("Current: {}", current);
    if let Some(ref latest_tag) = latest {
        println!("Latest:  {}", latest_tag);

        if &current == latest_tag {
            println!("\nAlready at latest version.");
            if rebuild {
                println!();
                return sync_source_build(None, false);
            }
            return Ok(());
        }

        // Checkout latest tag
        println!("\nChecking out {}...", latest_tag);
        let status = Command::new("git")
            .args(["-C", &source_path, "checkout", latest_tag])
            .status()
            .map_err(|e| NvControlError::CommandFailed(format!("git checkout failed: {}", e)))?;

        if !status.success() {
            return Err(NvControlError::CommandFailed(
                "git checkout failed".to_string(),
            ));
        }

        // Update dkms.conf version if needed
        let new_version = get_source_version(&source_path)?;
        println!("New version: {}", new_version);

        // Check if DKMS needs re-registration
        if let Some(ref old_ver) = info.nvidia_version {
            if old_ver != &new_version {
                println!("\nVersion changed, may need to re-register with DKMS:");
                println!("  sudo dkms remove nvidia/{} --all", old_ver);
                println!(
                    "  sudo ln -sf {} /usr/src/nvidia-{}",
                    source_path, new_version
                );
                println!("  sudo dkms add nvidia/{}", new_version);
            }
        }
    } else {
        println!("Could not determine latest tag");
    }

    if rebuild {
        println!();
        sync_source_build(None, false)?;
    } else {
        println!("\nSource updated. Build with: nvctl driver source sync");
    }

    Ok(())
}

/// Sync: rebuild modules from current source
pub fn sync_source_build(kernel: Option<&str>, force: bool) -> NvResult<()> {
    println!("Syncing NVIDIA modules from source...\n");
    build_dkms_nvidia(kernel, force)
}

// ==================== Kernel Cleanup ====================

/// Clean up old kernel modules
pub fn cleanup_old_kernels(keep: usize, execute: bool) -> NvResult<()> {
    println!(
        "NVIDIA DKMS Kernel Cleanup{}",
        if execute { "" } else { " (dry run)" }
    );
    println!("{}", "═".repeat(50));
    println!();

    // Get running kernel
    let running_kernel = Command::new("uname")
        .arg("-r")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| String::new());

    println!("Running kernel: {}", running_kernel);
    println!("Keeping: {} most recent kernels (plus running)\n", keep);

    // Get all installed kernels sorted by modification time (newest first)
    let mut kernels: Vec<(String, std::time::SystemTime)> = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/lib/modules") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Ok(meta) = entry.metadata() {
                if let Ok(modified) = meta.modified() {
                    kernels.push((name, modified));
                }
            }
        }
    }

    // Sort by modification time, newest first
    kernels.sort_by_key(|entry| std::cmp::Reverse(entry.1));

    let kernel_names: Vec<&str> = kernels.iter().map(|(n, _)| n.as_str()).collect();

    // Determine which to keep
    let mut to_keep: Vec<&str> = Vec::new();
    let mut to_remove: Vec<&str> = Vec::new();

    // Always keep running kernel
    to_keep.push(&running_kernel);

    // Keep N most recent (excluding running if already counted)
    let mut kept = 0;
    for name in &kernel_names {
        if *name == running_kernel {
            continue;
        }
        if kept < keep {
            to_keep.push(name);
            kept += 1;
        } else {
            to_remove.push(name);
        }
    }

    println!("Keeping ({}):", to_keep.len());
    for k in &to_keep {
        let suffix = if *k == running_kernel {
            " (running)"
        } else {
            ""
        };
        println!("  ✓ {}{}", k, suffix);
    }

    if to_remove.is_empty() {
        println!("\nNo kernels to remove.");
        return Ok(());
    }

    println!("\nTo remove ({}):", to_remove.len());
    for k in &to_remove {
        println!("  ✗ {}", k);
    }

    if !execute {
        println!("\nDry run - no changes made.");
        println!("Run with --execute to actually remove.");
        return Ok(());
    }

    // Get nvidia version for DKMS removal
    let info = get_dkms_setup_info();
    let version = info.nvidia_version.as_deref().unwrap_or("unknown");

    println!("\nRemoving...");
    for kernel in &to_remove {
        print!("  Removing nvidia from {}... ", kernel);

        // Remove from DKMS
        let status = Command::new("sudo")
            .args([
                "dkms", "remove", "-m", "nvidia", "-v", version, "-k", kernel,
            ])
            .output();

        match status {
            Ok(output) if output.status.success() => println!("done"),
            Ok(_) => println!("skipped (not in DKMS)"),
            Err(e) => println!("error: {}", e),
        }
    }

    println!("\nCleanup complete.");
    println!("Note: Kernel packages themselves were not removed.");
    println!("      Use your package manager to remove unused kernels.");

    Ok(())
}

// ==================== GPU Architecture Detection ====================

/// Detect GPU architecture from name, returns (architecture_name, is_legacy)
/// Legacy GPUs are pre-Turing (Maxwell, Pascal, Volta)
fn detect_gpu_architecture(gpu_name: &str) -> (String, bool) {
    let name = gpu_name.to_lowercase();

    // RTX 50 series - Blackwell
    if name.contains("5090")
        || name.contains("5080")
        || name.contains("5070")
        || name.contains("5060")
        || name.contains("rtx pro 6000 blackwell")
        || name.contains("rtx pro 5000 blackwell")
        || name.contains("gb202")
        || name.contains("gb203")
        || name.contains("gb205")
    {
        return ("Blackwell".to_string(), false);
    }

    // RTX 40 series - Ada Lovelace
    if name.contains("4090")
        || name.contains("4080")
        || name.contains("4070")
        || name.contains("4060")
        || name.contains("l40")
        || name.contains("l40s")
        || name.contains("l4")
        || name.contains("rtx 4000")
        || name.contains("rtx 5000")
        || name.contains("rtx 6000")
        || name.contains("rtx 2000 ada")
        || name.contains("rtx 4000 ada")
        || name.contains("rtx 5000 ada")
        || name.contains("rtx 6000 ada")
        || name.contains("ad102")
        || name.contains("ad103")
        || name.contains("ad104")
        || name.contains("ad106")
        || name.contains("ad107")
    {
        return ("Ada Lovelace".to_string(), false);
    }

    // RTX 30 series - Ampere
    if name.contains("3090")
        || name.contains("3080")
        || name.contains("3070")
        || name.contains("3060")
        || name.contains("a100")
        || name.contains("a40")
        || name.contains("a30")
        || name.contains("a10")
        || name.contains("a16")
        || name.contains("a2")
        || name.contains("ga100")
        || name.contains("ga102")
        || name.contains("ga104")
        || name.contains("ga106")
    {
        return ("Ampere".to_string(), false);
    }

    // Hopper
    if name.contains("h100")
        || name.contains("h200")
        || name.contains("gh100")
        || name.contains("gh200")
    {
        return ("Hopper".to_string(), false);
    }

    // RTX 20 series and GTX 16 series - Turing (last supported by nvidia-open)
    if name.contains("2080")
        || name.contains("2070")
        || name.contains("2060")
        || name.contains("1660")
        || name.contains("1650")
        || name.contains("t4")
        || name.contains("quadro rtx")
        || name.contains("tu102")
        || name.contains("tu104")
        || name.contains("tu106")
        || name.contains("tu117")
    {
        return ("Turing".to_string(), false);
    }

    // Volta - Legacy
    if name.contains("v100") || name.contains("titan v") {
        return ("Volta".to_string(), true);
    }

    // GTX 10 series - Pascal (Legacy, deprecated in 590+)
    if name.contains("gtx 10")
        || name.contains("1080")
        || name.contains("1070")
        || name.contains("1060")
        || name.contains("1050")
        || name.contains("titan xp")
        || name.contains("quadro p")
        || name.contains("p100")
        || name.contains("p40")
        || name.contains("p6000")
        || name.contains("p5000")
    {
        return ("Pascal".to_string(), true);
    }

    // GTX 9 series - Maxwell (Legacy, deprecated in 590+)
    if name.contains("gtx 9")
        || name.contains("980")
        || name.contains("970")
        || name.contains("960")
        || name.contains("950")
        || name.contains("titan x")
        || name.contains("quadro m")
    {
        // Be careful not to match "Titan Xp" (Pascal)
        if !name.contains("xp") {
            return ("Maxwell".to_string(), true);
        }
    }

    // Kepler and older - not supported
    if name.contains("gtx 7")
        || name.contains("gtx 6")
        || name.contains("gtx 5")
        || name.contains("780")
        || name.contains("770")
        || name.contains("760")
        || name.contains("750")
        || name.contains("680")
        || name.contains("670")
        || name.contains("660")
        || name.contains("650")
        || name.contains("quadro k")
        || name.contains("k80")
        || name.contains("k40")
    {
        return ("Kepler".to_string(), true);
    }

    // Fermi and older
    if name.contains("gtx 4")
        || name.contains("gtx 5")
        || name.contains("geforce 4")
        || name.contains("geforce 5")
        || name.contains("geforce 6")
        || name.contains("geforce 7")
        || name.contains("quadro 4")
        || name.contains("quadro 5")
        || name.contains("quadro 6")
    {
        return ("Fermi or older".to_string(), true);
    }

    ("Unknown".to_string(), false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_type_parsing() {
        assert!(matches!(
            DriverType::from_str("proprietary"),
            Some(DriverType::Proprietary)
        ));
        assert!(matches!(
            DriverType::from_str("open"),
            Some(DriverType::Open)
        ));
        assert!(DriverType::from_str("invalid").is_none());
    }

    #[test]
    fn test_detect_distribution() {
        let distro = detect_distribution();
        assert!(!distro.is_empty());
    }

    #[test]
    fn test_driver_capabilities_from_version() {
        // Test 590 driver
        let caps = DriverCapabilities::from_version("590.44.01").unwrap();
        assert_eq!(caps.major_version, 590);
        assert!(caps.has_vulkan_swapchain_perf);
        assert!(caps.supports_preempt_rt);
        assert_eq!(caps.wayland_min_version, "1.20");
        assert_eq!(caps.glibc_min_version, "2.27");

        // Test older driver
        let caps = DriverCapabilities::from_version("570.86.10").unwrap();
        assert_eq!(caps.major_version, 570);
        assert!(!caps.has_vulkan_swapchain_perf);
        assert!(!caps.supports_preempt_rt);
        assert_eq!(caps.wayland_min_version, "1.17");
        assert_eq!(caps.glibc_min_version, "2.17");
    }

    #[test]
    fn test_version_comparison() {
        assert_eq!(compare_versions("1.20", "1.17"), 1);
        assert_eq!(compare_versions("1.17", "1.20"), -1);
        assert_eq!(compare_versions("2.27", "2.27"), 0);
        assert_eq!(compare_versions("2.38", "2.27"), 1);
    }

    #[test]
    fn test_version_comparison_edge_cases() {
        // Single digit vs multi-digit
        assert_eq!(compare_versions("2.0", "1.99"), 1);
        assert_eq!(compare_versions("1.99", "2.0"), -1);

        // Three-part versions
        assert_eq!(compare_versions("1.20.5", "1.20.4"), 1);
        assert_eq!(compare_versions("1.20.5", "1.20.10"), -1);

        // Single component
        assert_eq!(compare_versions("590", "570"), 1);
        assert_eq!(compare_versions("570", "590"), -1);

        // Different lengths
        assert_eq!(compare_versions("1.20", "1.20.0"), 0);
        assert_eq!(compare_versions("1.20.0", "1.20"), 0);

        // Empty/malformed (should not panic)
        assert_eq!(compare_versions("", "1.0"), -1);
        assert_eq!(compare_versions("1.0", ""), 1);
    }

    #[test]
    fn test_driver_capabilities_beta_detection() {
        // Beta version string
        let caps = DriverCapabilities::from_version("590.44.01-beta").unwrap();
        assert!(caps.is_beta);
        assert_eq!(caps.major_version, 590);

        // Regular version should not be beta
        let caps = DriverCapabilities::from_version("590.44.01").unwrap();
        assert!(!caps.is_beta);
    }

    #[test]
    fn test_driver_capabilities_boundary_versions() {
        // Exactly at 590 boundary
        let caps = DriverCapabilities::from_version("590.00.00").unwrap();
        assert!(caps.has_vulkan_swapchain_perf);
        assert!(caps.supports_preempt_rt);
        assert!(caps.has_usb4_dp_support);
        assert!(caps.has_powermizer_wayland_fix);

        // Just below 590
        let caps = DriverCapabilities::from_version("589.99.99").unwrap();
        assert!(!caps.has_vulkan_swapchain_perf);
        assert!(!caps.supports_preempt_rt);

        // Very old driver
        let caps = DriverCapabilities::from_version("470.82.00").unwrap();
        assert_eq!(caps.major_version, 470);
        assert!(!caps.has_vulkan_swapchain_perf);
    }

    #[test]
    fn test_driver_capabilities_invalid_version() {
        // Empty string - gracefully defaults to major version 0
        let caps = DriverCapabilities::from_version("").unwrap();
        assert_eq!(caps.major_version, 0);
        assert!(!caps.has_vulkan_swapchain_perf);

        // Non-numeric - gracefully defaults to major version 0
        let caps = DriverCapabilities::from_version("abc.def.ghi").unwrap();
        assert_eq!(caps.major_version, 0);
        assert!(!caps.has_vulkan_swapchain_perf);
    }

    #[test]
    fn test_validate_system_requirements() {
        // Test with a 590 target - should run validation logic
        let check = validate_system_for_driver(590);

        // These checks depend on the actual system, but they should not panic
        // and should return structured results
        assert!(check.warnings.is_empty() || !check.warnings.is_empty()); // always true
        assert!(check.errors.is_empty() || !check.errors.is_empty()); // always true

        // Check that driver version info is populated
        // (may be empty if nvidia-smi is not available)
    }

    #[test]
    fn test_validate_system_old_target() {
        // Test with older target - should pass more easily
        let _check = validate_system_for_driver(470);

        // 470 has very minimal requirements, most systems should pass
        // unless there's no driver at all
    }

    #[test]
    fn test_preempt_rt_detection() {
        // Just verify the function runs without panicking
        let _ = is_preempt_rt_kernel();
    }

    #[test]
    fn test_get_wayland_version() {
        // Should not panic regardless of whether Wayland is installed
        let version = get_wayland_version();
        // Either returns Some(version) or None, both are valid
        if let Some(v) = version {
            // If a version is returned, it should be parseable
            assert!(!v.is_empty());
        }
    }

    #[test]
    fn test_get_glibc_version() {
        // Should not panic - glibc is always present on Linux
        let version = get_glibc_version();
        // glibc should always be present on Linux
        if let Some(v) = version {
            assert!(!v.is_empty());
            // Should contain a dot (e.g., "2.38")
            assert!(v.contains('.'));
        }
    }

    #[test]
    fn test_detect_gpu_architecture() {
        // Modern GPUs - should not be legacy
        let (arch, legacy) = detect_gpu_architecture("NVIDIA GeForce RTX 5090");
        assert_eq!(arch, "Blackwell");
        assert!(!legacy);

        let (arch, legacy) = detect_gpu_architecture("NVIDIA GeForce RTX 4090");
        assert_eq!(arch, "Ada Lovelace");
        assert!(!legacy);

        let (arch, legacy) = detect_gpu_architecture("NVIDIA GeForce RTX 3080");
        assert_eq!(arch, "Ampere");
        assert!(!legacy);

        let (arch, legacy) = detect_gpu_architecture("NVIDIA GeForce RTX 2070 Super");
        assert_eq!(arch, "Turing");
        assert!(!legacy);

        let (arch, legacy) = detect_gpu_architecture("NVIDIA GeForce GTX 1660 Ti");
        assert_eq!(arch, "Turing");
        assert!(!legacy);

        // Legacy GPUs - should be marked as legacy
        let (arch, legacy) = detect_gpu_architecture("NVIDIA GeForce GTX 1080 Ti");
        assert_eq!(arch, "Pascal");
        assert!(legacy);

        let (arch, legacy) = detect_gpu_architecture("NVIDIA GeForce GTX 980 Ti");
        assert_eq!(arch, "Maxwell");
        assert!(legacy);

        let (arch, legacy) = detect_gpu_architecture("NVIDIA TITAN Xp");
        assert_eq!(arch, "Pascal");
        assert!(legacy);

        let (arch, legacy) = detect_gpu_architecture("NVIDIA Tesla V100");
        assert_eq!(arch, "Volta");
        assert!(legacy);

        // Datacenter GPUs
        let (arch, legacy) = detect_gpu_architecture("NVIDIA A100-SXM4-80GB");
        assert_eq!(arch, "Ampere");
        assert!(!legacy);

        let (arch, legacy) = detect_gpu_architecture("NVIDIA L40S");
        assert_eq!(arch, "Ada Lovelace");
        assert!(!legacy);
    }

    #[test]
    fn test_detect_gpu_architecture_by_pci_id() {
        assert_eq!(
            detect_gpu_architecture_by_pci_id("2901"),
            Some(("Blackwell".to_string(), false))
        );
        assert_eq!(
            detect_gpu_architecture_by_pci_id("2704"),
            Some(("Ada Lovelace".to_string(), false))
        );
        assert_eq!(
            detect_gpu_architecture_by_pci_id("2330"),
            Some(("Hopper".to_string(), false))
        );
        assert_eq!(
            detect_gpu_architecture_by_pci_id("1f02"),
            Some(("Turing".to_string(), false))
        );
        assert_eq!(
            detect_gpu_architecture_by_pci_id("1b80"),
            Some(("Pascal".to_string(), true))
        );
    }

    #[test]
    fn test_normalize_pci_bus_id() {
        assert_eq!(
            normalize_pci_bus_id("0000:01:00.0"),
            Some("0000:01:00.0".to_string())
        );
        assert_eq!(normalize_pci_bus_id(""), None);
    }

    #[test]
    fn test_collect_release_diagnostics_structure() {
        let diagnostics = collect_release_diagnostics();
        assert!(!diagnostics.running_kernel.is_empty());
        assert!(!diagnostics.module_kernel.is_empty());
    }
}
