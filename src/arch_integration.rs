// Arch Linux Package Management Integration for NVIDIA
// Handles nvidia-open vs nvidia-dkms, kernel compatibility, and pacman hooks

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelInfo {
    pub version: String,
    pub nvidia_compatible: bool,
    pub dkms_built: bool,
}

pub struct ArchIntegration {
    hooks_dir: PathBuf,
}

impl ArchIntegration {
    pub fn pacman_kernel_targets() -> Vec<String> {
        let mut targets = vec![
            "linux".to_string(),
            "linux-lts".to_string(),
            "linux-zen".to_string(),
            "linux-hardened".to_string(),
            "linux-cachyos".to_string(),
            "linux-cachyos-lto".to_string(),
        ];

        if let Ok(entries) = fs::read_dir("/boot") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if let Some(kernel_name) = name.strip_prefix("vmlinuz-") {
                    let package = format!("linux-{}", kernel_name);
                    targets.push(package);
                }
            }
        }

        targets.push("linux-tkg-*".to_string());
        targets.sort();
        targets.dedup();
        targets
    }

    fn pacman_kernel_targets_block() -> String {
        Self::pacman_kernel_targets()
            .into_iter()
            .map(|target| format!("Target = {}", target))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn new() -> Self {
        Self {
            hooks_dir: PathBuf::from("/etc/pacman.d/hooks"),
        }
    }

    /// Detect NVIDIA package installation
    pub fn detect_nvidia_packages() -> NvResult<HashMap<String, PackageInfo>> {
        let mut packages = HashMap::new();

        let nvidia_pkgs = vec![
            "nvidia",
            "nvidia-dkms",
            "nvidia-open",
            "nvidia-open-dkms",
            "nvidia-utils",
            "nvidia-settings",
            "lib32-nvidia-utils",
            "nvidia-container-toolkit",
            "cuda",
        ];

        for pkg in nvidia_pkgs {
            if let Ok(info) = Self::query_package(pkg) {
                packages.insert(pkg.to_string(), info);
            }
        }

        Ok(packages)
    }

    /// Query pacman for package info
    fn query_package(name: &str) -> NvResult<PackageInfo> {
        let output = Command::new("pacman").args(&["-Qi", name]).output();

        let installed = output.as_ref().map(|o| o.status.success()).unwrap_or(false);

        let version = if installed {
            if let Ok(out) = output {
                let info = String::from_utf8_lossy(&out.stdout);
                info.lines()
                    .find(|l| l.starts_with("Version"))
                    .and_then(|l| l.split(':').nth(1))
                    .map(|v| v.trim().to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            } else {
                "unknown".to_string()
            }
        } else {
            "not installed".to_string()
        };

        Ok(PackageInfo {
            name: name.to_string(),
            version,
            installed,
        })
    }

    /// Get current kernel version
    pub fn get_kernel_version() -> NvResult<String> {
        let output = Command::new("uname").arg("-r").output().map_err(|e| {
            NvControlError::CommandFailed(format!("Failed to get kernel version: {}", e))
        })?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Check if current kernel is compatible with NVIDIA drivers
    pub fn check_kernel_compatibility() -> NvResult<KernelInfo> {
        let version = Self::get_kernel_version()?;

        // Check if DKMS modules are built for this kernel
        let dkms_status = Command::new("dkms")
            .args(&["status", "-m", "nvidia"])
            .output();

        let dkms_built = if let Ok(output) = dkms_status {
            let status = String::from_utf8_lossy(&output.stdout);
            status.contains(&version) && status.contains("installed")
        } else {
            false
        };

        // Assume compatible if modules are built or nvidia-open is used
        let nvidia_compatible = dkms_built || Self::check_nvidia_module_loaded();

        Ok(KernelInfo {
            version,
            nvidia_compatible,
            dkms_built,
        })
    }

    /// Check if NVIDIA kernel module is loaded
    fn check_nvidia_module_loaded() -> bool {
        Command::new("lsmod")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains("nvidia"))
            .unwrap_or(false)
    }

    /// Install pacman hook for automatic mkinitcpio regeneration
    pub fn install_mkinitcpio_hook(&self) -> NvResult<()> {
        println!("🪝 Installing mkinitcpio regeneration hook...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to install hooks".to_string(),
            ));
        }

        fs::create_dir_all(&self.hooks_dir).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create hooks dir: {}", e))
        })?;

        let hook_content = format!(
            r#"[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = nvidia
Target = nvidia-dkms
Target = nvidia-open
Target = nvidia-open-dkms
{}

[Action]
Description = Updating NVIDIA module in initcpio
Depends = mkinitcpio
When = PostTransaction
NeedsTargets
Exec = /bin/sh -c 'while read -r trg; do case $trg in linux*) exit 0; esac; done; /usr/bin/mkinitcpio -P'
"#,
            Self::pacman_kernel_targets_block()
        );

        let hook_path = self.hooks_dir.join("nvidia-mkinitcpio.hook");
        fs::write(&hook_path, hook_content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write hook: {}", e)))?;

        println!("✅ Hook installed: {}", hook_path.display());
        Ok(())
    }

    /// Install pacman hook for DKMS rebuild
    pub fn install_dkms_hook(&self) -> NvResult<()> {
        println!("🪝 Installing DKMS rebuild hook...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to install hooks".to_string(),
            ));
        }

        let hook_content = r#"[Trigger]
Operation = Install
Operation = Upgrade
Type = Package
Target = nvidia-dkms
Target = nvidia-open-dkms

[Action]
Description = Rebuilding NVIDIA DKMS modules
Depends = dkms
When = PostTransaction
Exec = /usr/bin/dkms autoinstall
"#;

        let hook_path = self.hooks_dir.join("nvidia-dkms-rebuild.hook");
        fs::write(&hook_path, hook_content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write hook: {}", e)))?;

        println!("✅ Hook installed: {}", hook_path.display());
        Ok(())
    }

    /// Install pacman hook to prevent kernel updates breaking NVIDIA
    pub fn install_kernel_hold_hook(&self) -> NvResult<()> {
        println!("🪝 Installing kernel compatibility check hook...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to install hooks".to_string(),
            ));
        }

        let hook_content = format!(
            r#"[Trigger]
Operation = Upgrade
Type = Package
{}

[Action]
Description = Checking NVIDIA compatibility before kernel upgrade
When = PreTransaction
Exec = /usr/bin/sh -c 'echo "⚠️  Kernel upgrade detected. NVIDIA drivers may need rebuilding after upgrade."'
"#,
            Self::pacman_kernel_targets_block()
        );

        let hook_path = self.hooks_dir.join("nvidia-kernel-warn.hook");
        fs::write(&hook_path, hook_content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write hook: {}", e)))?;

        println!("✅ Hook installed: {}", hook_path.display());
        Ok(())
    }

    /// Install all recommended hooks
    pub fn install_all_hooks(&self) -> NvResult<()> {
        println!("🪝 Installing all NVIDIA pacman hooks...\n");

        self.install_mkinitcpio_hook()?;
        self.install_dkms_hook()?;
        self.install_kernel_hold_hook()?;

        println!("\n✅ All hooks installed successfully!");
        println!("   These hooks will automatically:");
        println!("   • Regenerate initramfs after NVIDIA/kernel updates");
        println!("   • Rebuild DKMS modules after updates");
        println!("   • Warn before kernel upgrades");

        Ok(())
    }

    /// Remove nvcontrol-installed hooks
    pub fn remove_hooks(&self) -> NvResult<()> {
        println!("🗑️  Removing nvcontrol pacman hooks...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to remove hooks".to_string(),
            ));
        }

        let hooks = vec![
            "nvidia-mkinitcpio.hook",
            "nvidia-dkms-rebuild.hook",
            "nvidia-kernel-warn.hook",
        ];

        for hook in hooks {
            let path = self.hooks_dir.join(hook);
            if path.exists() {
                fs::remove_file(&path).map_err(|e| {
                    NvControlError::ConfigError(format!("Failed to remove hook: {}", e))
                })?;
                println!("   Removed: {}", hook);
            }
        }

        println!("✅ Hooks removed");
        Ok(())
    }

    /// Check AUR packages and suggest optimizations
    pub fn suggest_aur_optimizations() -> Vec<String> {
        let mut suggestions = Vec::new();

        // Check for nvidia-patch (removes streaming restrictions)
        if Self::query_package("nvidia-patch")
            .map(|p| !p.installed)
            .unwrap_or(true)
        {
            suggestions
                .push("📦 nvidia-patch - Removes NVENC/NVDEC stream restrictions".to_string());
        }

        // Check for nvidia-tweaks
        if Self::query_package("nvidia-tweaks")
            .map(|p| !p.installed)
            .unwrap_or(true)
        {
            suggestions.push("📦 nvidia-tweaks - Additional performance tweaks".to_string());
        }

        // Check for optimus-manager (for laptops)
        if Self::is_laptop() {
            if Self::query_package("optimus-manager")
                .map(|p| !p.installed)
                .unwrap_or(true)
            {
                suggestions
                    .push("📦 optimus-manager - Hybrid GPU management for laptops".to_string());
            }
        }

        suggestions
    }

    /// Check if system is a laptop
    fn is_laptop() -> bool {
        Path::new("/sys/class/power_supply/BAT0").exists()
            || Path::new("/sys/class/power_supply/BAT1").exists()
    }

    /// Rebuild DKMS modules manually
    pub fn rebuild_dkms_modules(&self) -> NvResult<()> {
        println!("🔧 Rebuilding NVIDIA DKMS modules...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to rebuild DKMS modules".to_string(),
            ));
        }

        let kernel = Self::get_kernel_version()?;

        println!("   Kernel: {}", kernel);
        println!("   Module: nvidia");

        let status = Command::new("dkms")
            .args(&["autoinstall"])
            .status()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to rebuild DKMS: {}", e)))?;

        if !status.success() {
            return Err(NvControlError::CommandFailed(
                "DKMS rebuild failed".to_string(),
            ));
        }

        println!("✅ DKMS modules rebuilt successfully");
        Ok(())
    }

    /// Regenerate initramfs
    pub fn regenerate_initramfs(&self) -> NvResult<()> {
        println!("🔧 Regenerating initramfs...");

        if !self.is_root() {
            return Err(NvControlError::ConfigError(
                "Root privileges required to regenerate initramfs".to_string(),
            ));
        }

        let status = Command::new("mkinitcpio").arg("-P").status().map_err(|e| {
            NvControlError::CommandFailed(format!("Failed to run mkinitcpio: {}", e))
        })?;

        if !status.success() {
            return Err(NvControlError::CommandFailed(
                "mkinitcpio failed".to_string(),
            ));
        }

        println!("✅ Initramfs regenerated successfully");
        println!("⚠️  Reboot required for changes to take effect");

        Ok(())
    }

    /// Check for pending kernel/driver updates
    pub fn check_pending_updates() -> NvResult<Vec<String>> {
        let output = Command::new("checkupdates").output();

        let mut nvidia_updates = Vec::new();

        if let Ok(out) = output {
            let updates = String::from_utf8_lossy(&out.stdout);

            for line in updates.lines() {
                if line.contains("nvidia") || line.contains("linux") {
                    nvidia_updates.push(line.to_string());
                }
            }
        }

        Ok(nvidia_updates)
    }

    fn is_root(&self) -> bool {
        nix::unistd::geteuid().is_root()
    }

    /// Print comprehensive system status
    pub fn print_status(&self) -> NvResult<()> {
        println!("🐧 Arch Linux NVIDIA Integration Status\n");

        // Kernel info
        let kernel_info = Self::check_kernel_compatibility()?;
        println!("Kernel Information:");
        println!("   Version: {}", kernel_info.version);
        println!(
            "   NVIDIA Compatible: {}",
            if kernel_info.nvidia_compatible {
                "✅ Yes"
            } else {
                "❌ No"
            }
        );
        println!(
            "   DKMS Built: {}",
            if kernel_info.dkms_built {
                "✅ Yes"
            } else {
                "❌ No (using nvidia-open?)"
            }
        );

        // Package info
        println!("\nInstalled NVIDIA Packages:");
        let packages = Self::detect_nvidia_packages()?;

        for (name, info) in packages.iter() {
            if info.installed {
                println!("   ✅ {} ({})", name, info.version);
            }
        }

        // Hooks status
        println!("\nPacman Hooks:");
        let hooks = vec![
            "nvidia-mkinitcpio.hook",
            "nvidia-dkms-rebuild.hook",
            "nvidia-kernel-warn.hook",
        ];

        for hook in hooks {
            let exists = self.hooks_dir.join(hook).exists();
            println!("   {} {}", if exists { "✅" } else { "❌" }, hook);
        }

        // Check for pending updates
        if let Ok(updates) = Self::check_pending_updates() {
            if !updates.is_empty() {
                println!("\n⚠️  Pending Updates:");
                for update in updates {
                    println!("   {}", update);
                }
            }
        }

        // AUR suggestions
        let suggestions = Self::suggest_aur_optimizations();
        if !suggestions.is_empty() {
            println!("\n💡 Recommended AUR Packages:");
            for suggestion in suggestions {
                println!("   {}", suggestion);
            }
        }

        Ok(())
    }
}
