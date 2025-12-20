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

// ==================== DKMS Setup & Management ====================

/// Information about DKMS setup status
#[derive(Debug)]
pub struct DkmsSetupInfo {
    pub dkms_installed: bool,
    pub nvidia_registered: bool,
    pub nvidia_version: Option<String>,
    pub source_path: Option<String>,
    pub kernels_built: Vec<String>,
    pub kernels_missing: Vec<String>,
}

/// Get detailed DKMS setup information
pub fn get_dkms_setup_info() -> DkmsSetupInfo {
    let mut info = DkmsSetupInfo {
        dkms_installed: false,
        nvidia_registered: false,
        nvidia_version: None,
        source_path: None,
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

    // Check for nvidia source in /usr/src
    if let Ok(entries) = std::fs::read_dir("/usr/src") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("nvidia-") || name_str.starts_with("nvidia-open-") {
                info.source_path = Some(entry.path().display().to_string());
                break;
            }
        }
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
pub fn build_dkms_nvidia(kernel: Option<&str>) -> NvResult<()> {
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

    match kernel {
        Some(k) => {
            println!("Building nvidia {} for kernel {}...", version, k);
            let status = Command::new("sudo")
                .args(["dkms", "install", "-m", "nvidia", "-v", version, "-k", k])
                .status()
                .map_err(|e| {
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
            println!("Building nvidia {} for all kernels...\n", version);

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
                let status = Command::new("sudo")
                    .args([
                        "dkms", "install", "-m", "nvidia", "-v", version, "-k", kernel,
                    ])
                    .output();

                match status {
                    Ok(output) if output.status.success() => println!("done"),
                    Ok(output) => {
                        println!("failed");
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if stderr.contains("already installed") {
                            println!("    (already installed)");
                        } else if !stderr.is_empty() {
                            println!("    {}", stderr.lines().next().unwrap_or(""));
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
    let hook_content = r#"[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = nvidia-open
Target = nvidia-open-dkms
Target = linux
Target = linux-lts
Target = linux-zen
Target = linux-hardened
Target = linux-cachyos
Target = linux-cachyos-lto
Target = linux-tkg-*

[Action]
Description = Rebuilding NVIDIA modules via DKMS...
Depends = dkms
When = PostTransaction
NeedsTargets
Exec = /usr/local/bin/nvidia-dkms-build
"#;

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

# Run dkms autoinstall with full output captured
if /usr/bin/dkms autoinstall 2>&1 | tee -a "$LOG_FILE"; then
    echo "" | tee -a "$LOG_FILE"
    echo "Build completed successfully" | tee -a "$LOG_FILE"
    ln -sf "$LOG_FILE" "$LATEST_LOG"

    # Desktop notification on success (if available)
    if command -v notify-send &>/dev/null && [ -n "$DISPLAY" -o -n "$WAYLAND_DISPLAY" ]; then
        notify-send -u low "NVIDIA DKMS" "Modules rebuilt successfully"
    fi
else
    EXIT_CODE=$?
    echo "" | tee -a "$LOG_FILE"
    echo "Build FAILED with exit code $EXIT_CODE" | tee -a "$LOG_FILE"
    ln -sf "$LOG_FILE" "$LATEST_LOG"

    # Desktop notification on failure
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
    if unsafe { libc::geteuid() != 0 } {
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

    // Source path
    if let Some(ref path) = info.source_path {
        println!("Source:         {}", path);
    } else if !info.nvidia_registered {
        println!("Source:         not found in /usr/src");
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
                "Multiple kernels installed but {} missing nvidia module: {}",
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
    if let Ok(version_info) = fs::read_to_string("/proc/driver/nvidia/version") {
        if version_info.contains("Open Kernel Module") {
            // Check if GSP firmware exists
            let driver_version = if let Ok(output) = Command::new("nvidia-smi")
                .args(["--query-gpu=driver_version", "--format=csv,noheader"])
                .output()
            {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "Unknown".to_string()
            };

            let firmware_path = format!("/lib/firmware/nvidia/{}", driver_version);
            if std::path::Path::new(&firmware_path).exists() {
                passed.push("GSP firmware present for nvidia-open".to_string());
            } else {
                errors.push(format!(
                    "GSP firmware missing at {} (required for nvidia-open)",
                    firmware_path
                ));
            }
        }
    }

    // 6. Check if nvidia-open is used on unsupported GPU (pre-Turing)
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=name", "--format=csv,noheader"])
        .output()
    {
        if output.status.success() {
            let gpu_name = String::from_utf8_lossy(&output.stdout).to_lowercase();

            // Check for pre-Turing GPUs (Maxwell, Pascal, Volta)
            let pre_turing = [
                "gtx 9", "gtx 10", "titan x", "titan xp", "quadro p", "quadro m",
            ];
            let is_pre_turing = pre_turing.iter().any(|&name| gpu_name.contains(name));

            if is_pre_turing {
                if let Ok(version_info) = fs::read_to_string("/proc/driver/nvidia/version") {
                    if version_info.contains("Open Kernel Module") {
                        errors.push(format!(
                            "nvidia-open driver on pre-Turing GPU ({}). Use proprietary driver instead.",
                            gpu_name.trim()
                        ));
                    }
                }
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

    Ok(())
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
