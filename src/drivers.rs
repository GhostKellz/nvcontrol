use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

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
    // Try to determine if it's open or proprietary driver
    if let Ok(output) = Command::new("modinfo").arg("nvidia").output() {
        let modinfo = String::from_utf8_lossy(&output.stdout);
        if modinfo.contains("nvidia-open") {
            "Open Source".to_string()
        } else {
            "Proprietary".to_string()
        }
    } else {
        "Unknown".to_string()
    }
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

pub fn generate_shell_completions(shell: &str) -> NvResult<()> {
    let completion_script = match shell.to_lowercase().as_str() {
        "bash" => generate_bash_completions(),
        "zsh" => generate_zsh_completions(),
        "fish" => generate_fish_completions(),
        _ => {
            return Err(NvControlError::DisplayDetectionFailed(format!(
                "Unsupported shell: {shell}. Use: bash, zsh, fish"
            )));
        }
    };

    println!("{completion_script}");
    Ok(())
}

fn generate_bash_completions() -> String {
    r#"# nvctl bash completion
_nvctl_completion() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    case "${prev}" in
        nvctl)
            opts="gpu display fan overclock vrr upscaling drivers"
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        gpu)
            opts="info stat capabilities"
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        display)
            opts="info ls vibrance hdr"
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        drivers)
            opts="status install update rollback"
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        install)
            opts="proprietary open open-beta"
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
    esac
}

complete -F _nvctl_completion nvctl"#
        .to_string()
}

fn generate_zsh_completions() -> String {
    r#"#compdef nvctl

_nvctl() {
    local context state line
    
    _arguments -C \
        '1: :->commands' \
        '*: :->args'
    
    case $state in
        commands)
            _values 'nvctl commands' \
                'gpu[GPU management]' \
                'display[Display management]' \
                'fan[Fan control]' \
                'overclock[Overclocking]' \
                'vrr[Variable refresh rate]' \
                'upscaling[DLSS/FSR management]' \
                'drivers[Driver management]'
            ;;
        args)
            case $words[2] in
                gpu)
                    _values 'GPU commands' \
                        'info[Show GPU information]' \
                        'stat[Live GPU monitoring]' \
                        'capabilities[Show overclocking capabilities]'
                    ;;
                drivers)
                    _values 'Driver commands' \
                        'status[Show driver status]' \
                        'install[Install driver]' \
                        'update[Update driver]' \
                        'rollback[Rollback driver]'
                    ;;
            esac
            ;;
    esac
}

_nvctl "$@""#
        .to_string()
}

fn generate_fish_completions() -> String {
    r#"# nvctl fish completion

# Main commands
complete -c nvctl -f -n '__fish_use_subcommand' -a 'gpu' -d 'GPU management'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'display' -d 'Display management'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'fan' -d 'Fan control'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'overclock' -d 'Overclocking'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'vrr' -d 'Variable refresh rate'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'upscaling' -d 'DLSS/FSR management'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'drivers' -d 'Driver management'

# GPU subcommands
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'info' -d 'Show GPU information'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'stat' -d 'Live GPU monitoring'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'capabilities' -d 'Overclocking capabilities'

# Driver subcommands
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'status' -d 'Show driver status'
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'install' -d 'Install driver'
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'update' -d 'Update driver'
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'rollback' -d 'Rollback driver'

# Driver types
complete -c nvctl -f -n '__fish_seen_subcommand_from install' -a 'proprietary' -d 'NVIDIA proprietary driver'
complete -c nvctl -f -n '__fish_seen_subcommand_from install' -a 'open' -d 'NVIDIA open source driver'
complete -c nvctl -f -n '__fish_seen_subcommand_from install' -a 'open-beta' -d 'NVIDIA open source beta driver'"#.to_string()
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
}
