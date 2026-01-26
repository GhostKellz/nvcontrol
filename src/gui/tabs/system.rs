//! System Information Tab
//!
//! Displays system information like neofetch - OS, kernel, CPU, GPU, DE, memory, etc.
//! Also shows driver, GSP, and DKMS status.

use eframe::egui;
use std::fs;
use std::process::Command;

use crate::drivers;
use crate::gsp_firmware::GspManager;
use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;

/// Cached system information
#[derive(Default, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub uptime: String,
    pub shell: String,
    pub de: String,
    pub wm: String,
    pub cpu: String,
    pub memory_used: String,
    pub memory_total: String,
    pub gpus: Vec<String>,
}

impl SystemInfo {
    /// Gather system information
    pub fn gather() -> Self {
        Self {
            hostname: get_hostname(),
            os: get_os(),
            kernel: get_kernel(),
            uptime: get_uptime(),
            shell: get_shell(),
            de: get_de(),
            wm: get_wm(),
            cpu: get_cpu(),
            memory_used: get_memory_used(),
            memory_total: get_memory_total(),
            gpus: get_gpus(),
        }
    }
}

/// Driver, GSP, and DKMS information
#[derive(Default, Clone)]
pub struct DriverInfo {
    pub driver_version: String,
    pub driver_type: String,
    pub gsp_enabled: bool,
    pub gsp_state: String,
    pub gsp_arch: Option<String>,
    pub dkms_registered: bool,
    pub dkms_status: String,
    pub kernels_count: usize,
    pub kernels_with_nvidia: usize,
}

impl DriverInfo {
    /// Gather driver information
    pub fn gather() -> Self {
        let mut info = Self::default();

        // Get driver version from nvidia-smi
        if let Ok(output) = Command::new("nvidia-smi")
            .args(["--query-gpu=driver_version", "--format=csv,noheader"])
            .output()
        {
            if output.status.success() {
                info.driver_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            }
        }

        // Determine driver type
        info.driver_type = if GspManager::is_nvidia_open() {
            "nvidia-open".to_string()
        } else {
            "proprietary".to_string()
        };

        // Get GSP status
        let gsp_mgr = GspManager::new();
        let gsp_status = gsp_mgr.get_deep_status();
        info.gsp_enabled = gsp_status.enabled;
        info.gsp_state = gsp_status.state;
        info.gsp_arch = gsp_status.gpu_arch;

        // Get DKMS status
        let dkms_info = drivers::get_dkms_setup_info();
        info.dkms_registered = dkms_info.nvidia_registered;
        info.dkms_status = if !dkms_info.dkms_installed {
            "not installed".to_string()
        } else if dkms_info.nvidia_registered {
            "registered".to_string()
        } else {
            "not registered".to_string()
        };

        // Count kernels
        if let Ok(entries) = std::fs::read_dir("/lib/modules") {
            for entry in entries.flatten() {
                info.kernels_count += 1;
                let kernel = entry.file_name().to_string_lossy().to_string();
                let paths = [
                    format!("/lib/modules/{}/kernel/drivers/video/nvidia.ko.zst", kernel),
                    format!("/lib/modules/{}/kernel/drivers/video/nvidia.ko", kernel),
                    format!("/lib/modules/{}/extramodules/nvidia.ko.zst", kernel),
                    format!("/lib/modules/{}/extramodules/nvidia.ko", kernel),
                ];
                if paths.iter().any(|p| std::path::Path::new(p).exists()) {
                    info.kernels_with_nvidia += 1;
                }
            }
        }

        info
    }
}

fn get_hostname() -> String {
    fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

fn get_os() -> String {
    // Try os-release first
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line
                    .trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    "Linux".to_string()
}

fn get_kernel() -> String {
    Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn get_uptime() -> String {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(secs_str) = content.split_whitespace().next() {
            if let Ok(secs) = secs_str.parse::<f64>() {
                let total_secs = secs as u64;
                let days = total_secs / 86400;
                let hours = (total_secs % 86400) / 3600;
                let mins = (total_secs % 3600) / 60;

                if days > 0 {
                    return format!(
                        "{} day{}, {} hr{}, {} min",
                        days,
                        if days != 1 { "s" } else { "" },
                        hours,
                        if hours != 1 { "s" } else { "" },
                        mins
                    );
                } else if hours > 0 {
                    return format!(
                        "{} hr{}, {} min",
                        hours,
                        if hours != 1 { "s" } else { "" },
                        mins
                    );
                } else {
                    return format!("{} min", mins);
                }
            }
        }
    }
    "unknown".to_string()
}

fn get_shell() -> String {
    std::env::var("SHELL")
        .ok()
        .and_then(|s| s.rsplit('/').next().map(String::from))
        .unwrap_or_else(|| "unknown".to_string())
}

fn get_de() -> String {
    // Try XDG_CURRENT_DESKTOP first
    if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
        if !de.is_empty() {
            return de;
        }
    }
    // Try DESKTOP_SESSION
    if let Ok(de) = std::env::var("DESKTOP_SESSION") {
        if !de.is_empty() {
            return de;
        }
    }
    "unknown".to_string()
}

fn get_wm() -> String {
    // Check session type
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();

    if session_type == "wayland" {
        // Check for common Wayland compositors
        if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
            return "Hyprland".to_string();
        }
        if std::env::var("SWAYSOCK").is_ok() {
            return "Sway".to_string();
        }
        // KDE on Wayland
        if std::env::var("KDE_FULL_SESSION").is_ok() {
            return "KWin (Wayland)".to_string();
        }
        // GNOME on Wayland
        if std::env::var("GNOME_SETUP_DISPLAY").is_ok() {
            return "Mutter (Wayland)".to_string();
        }
        return format!(
            "Wayland ({})",
            std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
        );
    }

    // X11 - try to detect WM
    if let Ok(wm) = std::env::var("XDG_CURRENT_DESKTOP") {
        match wm.to_lowercase().as_str() {
            "kde" => return "KWin".to_string(),
            "gnome" => return "Mutter".to_string(),
            "xfce" => return "Xfwm4".to_string(),
            _ => {}
        }
    }

    session_type
}

fn get_cpu() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some(name) = line.split(':').nth(1) {
                    let name = name.trim();
                    // Count cores
                    let cores = content
                        .lines()
                        .filter(|l| l.starts_with("processor"))
                        .count();
                    return format!("{} ({})", name, cores);
                }
            }
        }
    }
    "unknown".to_string()
}

fn get_memory_used() -> String {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        let mut total: u64 = 0;
        let mut available: u64 = 0;

        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                total = parse_meminfo_value(line);
            } else if line.starts_with("MemAvailable:") {
                available = parse_meminfo_value(line);
            }
        }

        if total > 0 {
            let used = total.saturating_sub(available);
            return format!("{} MiB", used / 1024);
        }
    }
    "unknown".to_string()
}

fn get_memory_total() -> String {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                let kb = parse_meminfo_value(line);
                return format!("{} MiB", kb / 1024);
            }
        }
    }
    "unknown".to_string()
}

fn parse_meminfo_value(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn get_gpus() -> Vec<String> {
    let mut gpus = Vec::new();

    // Try lspci for GPU info
    if let Ok(output) = Command::new("lspci").output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines() {
                if line.contains("VGA") || line.contains("3D controller") {
                    // Extract GPU name after the colon
                    if let Some(idx) = line.find(": ") {
                        let gpu_name = line[idx + 2..].trim();
                        gpus.push(gpu_name.to_string());
                    }
                }
            }
        }
    }

    if gpus.is_empty() {
        gpus.push("Unknown GPU".to_string());
    }

    gpus
}

/// Render the System tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} System Information", icons::SYSTEM));
    ui.add_space(4.0);

    // Use cached system info (rate-limited to avoid subprocess spawns every frame)
    state.refresh_system_info();
    let info = match state.get_system_info() {
        Some(info) => info.clone(),
        None => SystemInfo::default(), // Fallback while loading
    };

    // System Overview Card
    Card::new(&colors)
        .title("System Overview")
        .icon(icons::SYSTEM)
        .show(ui, |ui| {
            egui::Grid::new("system_info_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    // Host
                    ui.label(
                        egui::RichText::new("Host:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(colors.fg.to_egui(), &info.hostname);
                    ui.end_row();

                    // OS
                    ui.label(
                        egui::RichText::new("OS:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(colors.fg.to_egui(), &info.os);
                    ui.end_row();

                    // Kernel
                    ui.label(
                        egui::RichText::new("Kernel:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(colors.cyan.to_egui(), &info.kernel);
                    ui.end_row();

                    // Uptime
                    ui.label(
                        egui::RichText::new("Uptime:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(colors.fg.to_egui(), &info.uptime);
                    ui.end_row();

                    // Shell
                    ui.label(
                        egui::RichText::new("Shell:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(colors.fg.to_egui(), &info.shell);
                    ui.end_row();

                    // DE
                    ui.label(
                        egui::RichText::new("DE:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(colors.purple.to_egui(), &info.de);
                    ui.end_row();

                    // WM
                    ui.label(
                        egui::RichText::new("WM:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(colors.fg.to_egui(), &info.wm);
                    ui.end_row();
                });
        });

    ui.add_space(8.0);

    // Hardware Card
    Card::new(&colors)
        .title("Hardware")
        .icon(icons::GPU)
        .show(ui, |ui| {
            egui::Grid::new("hardware_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    // CPU
                    ui.label(
                        egui::RichText::new("CPU:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(colors.green.to_egui(), &info.cpu);
                    ui.end_row();

                    // GPUs
                    for (i, gpu) in info.gpus.iter().enumerate() {
                        ui.label(
                            egui::RichText::new(format!(
                                "GPU{}:",
                                if info.gpus.len() > 1 {
                                    format!(" {}", i)
                                } else {
                                    String::new()
                                }
                            ))
                            .strong()
                            .color(colors.comment.to_egui()),
                        );
                        ui.colored_label(colors.teal.to_egui(), gpu);
                        ui.end_row();
                    }

                    // Memory
                    ui.label(
                        egui::RichText::new("Memory:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(
                        colors.fg.to_egui(),
                        format!("{} / {}", info.memory_used, info.memory_total),
                    );
                    ui.end_row();
                });
        });

    ui.add_space(8.0);

    // Session Info Card
    Card::new(&colors)
        .title("Session")
        .icon(icons::DISPLAY)
        .show(ui, |ui| {
            egui::Grid::new("session_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    // Session Type
                    ui.label(
                        egui::RichText::new("Session:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    let session =
                        std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".to_string());
                    let session_color = if session == "wayland" {
                        colors.green.to_egui()
                    } else {
                        colors.yellow.to_egui()
                    };
                    ui.colored_label(session_color, &session);
                    ui.end_row();

                    // Display
                    if let Ok(display) = std::env::var("DISPLAY") {
                        ui.label(
                            egui::RichText::new("Display:")
                                .strong()
                                .color(colors.comment.to_egui()),
                        );
                        ui.colored_label(colors.fg.to_egui(), &display);
                        ui.end_row();
                    }

                    // Wayland Display
                    if let Ok(wayland) = std::env::var("WAYLAND_DISPLAY") {
                        ui.label(
                            egui::RichText::new("Wayland:")
                                .strong()
                                .color(colors.comment.to_egui()),
                        );
                        ui.colored_label(colors.fg.to_egui(), &wayland);
                        ui.end_row();
                    }

                    // nvcontrol version
                    ui.label(
                        egui::RichText::new("nvcontrol:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(
                        colors.teal.to_egui(),
                        format!("v{}", env!("CARGO_PKG_VERSION")),
                    );
                    ui.end_row();
                });
        });

    ui.add_space(8.0);

    // Driver & GSP Card (use cached data)
    let driver_info = match state.get_driver_info() {
        Some(info) => info.clone(),
        None => DriverInfo::default(), // Fallback while loading
    };

    Card::new(&colors)
        .title("NVIDIA Driver")
        .icon(icons::GPU)
        .show(ui, |ui| {
            egui::Grid::new("driver_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    // Driver version
                    ui.label(
                        egui::RichText::new("Driver:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    ui.colored_label(
                        colors.green.to_egui(),
                        format!(
                            "{} ({})",
                            driver_info.driver_version, driver_info.driver_type
                        ),
                    );
                    ui.end_row();

                    // GSP Status (only for nvidia-open)
                    if driver_info.driver_type == "nvidia-open" {
                        ui.label(
                            egui::RichText::new("GSP:")
                                .strong()
                                .color(colors.comment.to_egui()),
                        );
                        let gsp_text = if driver_info.gsp_enabled {
                            format!("enabled ({})", driver_info.gsp_state)
                        } else {
                            "disabled".to_string()
                        };
                        let gsp_color =
                            if driver_info.gsp_enabled && driver_info.gsp_state != "failed" {
                                colors.green.to_egui()
                            } else if driver_info.gsp_enabled {
                                colors.red.to_egui()
                            } else {
                                colors.yellow.to_egui()
                            };
                        ui.colored_label(gsp_color, gsp_text);
                        ui.end_row();

                        // GPU Architecture
                        if let Some(ref arch) = driver_info.gsp_arch {
                            ui.label(
                                egui::RichText::new("GPU Arch:")
                                    .strong()
                                    .color(colors.comment.to_egui()),
                            );
                            ui.colored_label(colors.cyan.to_egui(), arch);
                            ui.end_row();
                        }
                    }

                    // DKMS Status
                    ui.label(
                        egui::RichText::new("DKMS:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    let dkms_color = if driver_info.dkms_registered {
                        colors.green.to_egui()
                    } else if driver_info.dkms_status == "not installed" {
                        colors.comment.to_egui()
                    } else {
                        colors.yellow.to_egui()
                    };
                    ui.colored_label(dkms_color, &driver_info.dkms_status);
                    ui.end_row();

                    // Kernel coverage
                    ui.label(
                        egui::RichText::new("Kernels:")
                            .strong()
                            .color(colors.comment.to_egui()),
                    );
                    let kernels_text = format!(
                        "{}/{} have nvidia",
                        driver_info.kernels_with_nvidia, driver_info.kernels_count
                    );
                    let kernels_color =
                        if driver_info.kernels_with_nvidia == driver_info.kernels_count {
                            colors.green.to_egui()
                        } else {
                            colors.yellow.to_egui()
                        };
                    ui.colored_label(kernels_color, kernels_text);
                    ui.end_row();
                });

            // Hint for CLI
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Run 'nvctl driver check' for detailed diagnostics")
                    .small()
                    .color(colors.comment.to_egui()),
            );
        });
}
