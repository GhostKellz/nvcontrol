/// Enhanced Error Messages with Solutions
///
/// Provides user-friendly error messages with actionable solutions
use crate::NvControlError;
use console::style;

pub trait ErrorWithSolution {
    fn with_solution(&self) -> String;
}

impl ErrorWithSolution for NvControlError {
    fn with_solution(&self) -> String {
        match self {
            NvControlError::NvmlNotAvailable(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n  {}\n\n{}\n  {}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("NVML not available").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• NVIDIA drivers not installed",
                    "• nvidia-smi not in PATH",
                    "• Permissions issue",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Install NVIDIA drivers: sudo pacman -S nvidia-utils",
                        style("1.").cyan().bold()
                    ),
                    format!("{} Reboot your system", style("2.").cyan().bold()),
                    format!(
                        "{} Run: nvctl doctor (to diagnose issues)",
                        style("3.").cyan().bold()
                    )
                )
            }
            NvControlError::DisplayDetectionFailed(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Display detection failed").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• Wayland compositor not providing display info",
                    "• X11 server not running",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Check if Wayland/X11 is running: echo $XDG_SESSION_TYPE",
                        style("1.").cyan().bold()
                    ),
                    format!("{} Restart your compositor", style("2.").cyan().bold()),
                    format!(
                        "{} Try running with X11 if on Wayland",
                        style("3.").cyan().bold()
                    )
                )
            }
            NvControlError::VibranceControlFailed(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Vibrance control failed").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• nvidia-settings not installed",
                    "• Compositor doesn't support vibrance",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Install nvidia-settings: sudo pacman -S nvidia-settings",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} For Wayland: Use nvcontrol's native vibrance (no X11 needed)",
                        style("2.").cyan().bold()
                    ),
                    format!(
                        "{} For KDE: nvctl will use kwriteconfig6 automatically",
                        style("3.").cyan().bold()
                    )
                )
            }
            NvControlError::FanControlNotSupported => {
                format!(
                    "{} {}\n\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Fan control not supported").bold(),
                    style("Possible causes:").yellow(),
                    "• GPU doesn't support manual fan control",
                    "• nvidia-settings not configured",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Enable CoolBits in X11: nvidia-xconfig --cool-bits=12",
                        style("1.").cyan().bold()
                    ),
                    format!("{} Reboot or restart X11", style("2.").cyan().bold()),
                    format!(
                        "{} Check GPU model supports fan control (most modern GPUs do)",
                        style("3.").cyan().bold()
                    )
                )
            }
            NvControlError::PowerManagementFailed(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Power management failed").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• Insufficient permissions",
                    "• GPU locked by another process",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Run with sudo for power limit changes",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} Check if any GPU monitoring tools are running",
                        style("2.").cyan().bold()
                    ),
                    format!(
                        "{} Ensure nvidia-smi works: nvidia-smi -pl <power_limit>",
                        style("3.").cyan().bold()
                    )
                )
            }
            NvControlError::LatencyOptimizationFailed(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Latency optimization failed").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• GameMode not installed",
                    "• CPU governor not accessible",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Install GameMode: sudo pacman -S gamemode",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} Enable user permissions: usermod -aG gamemode $USER",
                        style("2.").cyan().bold()
                    ),
                    format!(
                        "{} Reboot to apply group changes",
                        style("3.").cyan().bold()
                    )
                )
            }
            NvControlError::ContainerOperationFailed(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n  {}\n\n{}\n  {}\n  {}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Container operation failed").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• Docker/Podman not running",
                    "• NVIDIA Container Toolkit not installed",
                    "• Insufficient permissions",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Install nvidia-container-toolkit",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} Start Docker: sudo systemctl start docker",
                        style("2.").cyan().bold()
                    ),
                    format!(
                        "{} Add user to docker group: sudo usermod -aG docker $USER",
                        style("3.").cyan().bold()
                    ),
                    format!(
                        "{} Test with: docker run --rm --gpus all nvidia/cuda:12.0-base nvidia-smi",
                        style("4.").cyan().bold()
                    )
                )
            }
            NvControlError::GpuQueryFailed(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("GPU query failed").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• GPU not detected",
                    "• Driver not loaded",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Check GPU is detected: lspci | grep -i nvidia",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} Load nvidia driver: sudo modprobe nvidia",
                        style("2.").cyan().bold()
                    ),
                    format!(
                        "{} Verify driver version: nvidia-smi",
                        style("3.").cyan().bold()
                    )
                )
            }
            NvControlError::CommandFailed(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Command execution failed").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• Command not found in PATH",
                    "• Insufficient permissions",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Ensure all NVIDIA tools are installed",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} Check command exists: which <command>",
                        style("2.").cyan().bold()
                    )
                )
            }
            NvControlError::ConfigError(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Configuration error").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• Invalid configuration file",
                    "• Missing required fields",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Check config file: ~/.config/nvcontrol/config.toml",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} Reset to defaults: nvctl config reset",
                        style("2.").cyan().bold()
                    )
                )
            }
            NvControlError::UnsupportedFeature(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n  {}\n  {}\n\n{}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Unsupported feature").bold(),
                    style("Details:").yellow(),
                    msg,
                    "• Feature not available on this GPU",
                    "• Feature requires newer drivers",
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Update NVIDIA drivers to the latest version",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} Check GPU capabilities: nvctl gpu info",
                        style("2.").cyan().bold()
                    )
                )
            }
            NvControlError::RuntimeError(msg) => {
                format!(
                    "{} {}\n\n{}\n{}\n\n{}\n  {}\n  {}",
                    style("❌ Error:").red().bold(),
                    style("Runtime error").bold(),
                    style("Details:").yellow(),
                    msg,
                    style("💡 Solutions:").green().bold(),
                    format!(
                        "{} Check system logs: journalctl -xe",
                        style("1.").cyan().bold()
                    ),
                    format!(
                        "{} Report issue: https://github.com/ghostkellz/nvcontrol/issues",
                        style("2.").cyan().bold()
                    )
                )
            }
        }
    }
}

/// Print error with solution
pub fn print_error_with_solution(error: &NvControlError) {
    eprintln!("\n{}\n", error.with_solution());
}

/// Detect if running in headless/CI environment
fn is_headless() -> bool {
    // No display session
    std::env::var("XDG_SESSION_TYPE").is_err()
        && std::env::var("DISPLAY").is_err()
        && std::env::var("WAYLAND_DISPLAY").is_err()
}

/// Quick diagnostic helper
pub fn run_diagnostics() -> String {
    let mut output = String::new();
    let headless = is_headless();

    if headless {
        output.push_str(&format!(
            "\n{}\n\n",
            style("🔍 nvcontrol Diagnostics (headless/CI mode)")
                .cyan()
                .bold()
        ));
    } else {
        output.push_str(&format!(
            "\n{}\n\n",
            style("🔍 nvcontrol Diagnostics").cyan().bold()
        ));
    }

    // Check NVIDIA drivers
    output.push_str(&format!(
        "{}\n",
        style("1. NVIDIA Drivers:").yellow().bold()
    ));
    let nvidia_smi_check = std::process::Command::new("nvidia-smi").output();
    match nvidia_smi_check {
        Ok(result) if result.status.success() => {
            output.push_str("   ✅ nvidia-smi found and working\n");
        }
        _ => {
            output.push_str("   ❌ nvidia-smi not found or not working\n");
            output.push_str("   💡 Install: sudo pacman -S nvidia-utils\n");
        }
    }

    // Skip display-related checks in headless mode
    if !headless {
        // Check nvidia-settings
        output.push_str(&format!(
            "\n{}\n",
            style("2. NVIDIA Settings:").yellow().bold()
        ));
        let settings_check = std::process::Command::new("nvidia-settings")
            .arg("--version")
            .output();
        match settings_check {
            Ok(result) if result.status.success() => {
                output.push_str("   ✅ nvidia-settings found\n");
            }
            _ => {
                output.push_str("   ❌ nvidia-settings not found\n");
                output.push_str("   💡 Install: sudo pacman -S nvidia-settings\n");
            }
        }

        // Check display server
        output.push_str(&format!(
            "\n{}\n",
            style("3. Display Server:").yellow().bold()
        ));
        if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
            output.push_str(&format!("   ✅ Running on: {}\n", session_type));
        } else {
            output.push_str("   ⚠️  Could not detect session type\n");
        }

        // Check GameMode
        output.push_str(&format!("\n{}\n", style("4. GameMode:").yellow().bold()));
        let gamemode_check = std::process::Command::new("gamemoded")
            .arg("--version")
            .output();
        match gamemode_check {
            Ok(result) if result.status.success() => {
                output.push_str("   ✅ GameMode installed\n");
            }
            _ => {
                output.push_str("   ❌ GameMode not found\n");
                output.push_str("   💡 Install: sudo pacman -S gamemode\n");
            }
        }

        // Check Docker/Podman
        output.push_str(&format!(
            "\n{}\n",
            style("5. Container Runtimes:").yellow().bold()
        ));
        let docker_check = std::process::Command::new("docker")
            .arg("--version")
            .output();
        match docker_check {
            Ok(result) if result.status.success() => {
                output.push_str("   ✅ Docker installed\n");
            }
            _ => {
                output.push_str("   ❌ Docker not found\n");
            }
        }
    } else {
        output.push_str("\n   ℹ️  Skipping display/desktop checks (headless environment)\n");
    }

    output.push_str(&format!(
        "\n{}\n",
        style("━━━━━━━━━━━━━━━━━━━━━━━━━━━━").cyan()
    ));

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_with_solution() {
        let error = NvControlError::NvmlNotAvailable("Test".to_string());
        let solution = error.with_solution();
        assert!(solution.contains("Solutions"));
        assert!(solution.contains("nvidia-utils"));
    }

    #[test]
    fn test_diagnostics() {
        let diag = run_diagnostics();
        assert!(diag.contains("Diagnostics"));
        assert!(diag.contains("NVIDIA Drivers"));
    }
}
