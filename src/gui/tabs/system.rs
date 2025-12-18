//! System Information Tab
//!
//! Displays system information like neofetch - OS, kernel, CPU, GPU, DE, memory, etc.

use eframe::egui;
use std::fs;
use std::process::Command;

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

    // Gather info (could cache this with a timer)
    let info = SystemInfo::gather();

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
}
