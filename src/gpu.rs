use crate::{NvControlError, NvResult, nvml_backend::SharedNvmlBackend};
use clap::ValueEnum;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub driver_version: String,
    pub memory_total: u64,
    pub memory_used: u64,
    pub temperature: u32,
    pub power_draw: f32,
    pub fan_speed: u32,
    pub gpu_utilization: u32,
    pub memory_utilization: u32,
    // Enhanced info fields
    pub cuda_compute: Option<String>,
    pub pcie_gen: Option<u32>,
    pub pcie_width: Option<u32>,
    pub power_limit: Option<f32>,
    pub power_limit_min: Option<f32>,
    pub power_limit_max: Option<f32>,
    pub gpu_clock: Option<u32>,
    pub memory_clock: Option<u32>,
    pub architecture: Option<String>,
    pub throttle_reason: Option<String>,
}

/// Check if NVIDIA GPU is available on the system
pub fn is_nvidia_available(backend: &SharedNvmlBackend) -> bool {
    backend
        .device_count()
        .map(|count| count > 0)
        .unwrap_or(false)
}

pub fn get_gpu_info(backend: &SharedNvmlBackend) -> NvResult<GpuInfo> {
    let device_count = backend.device_count()?;
    if device_count == 0 {
        return Err(NvControlError::GpuQueryFailed(
            "No NVIDIA GPUs found".to_string(),
        ));
    }

    let info = backend.get_device_info(0)?;
    let metrics = backend.get_metrics(0)?;
    let (mem_used, mem_total) = backend.get_memory_info(0)?;
    let driver = backend
        .get_name(0)
        .unwrap_or_else(|_| "Unknown".to_string());
    let cuda_compute = None; // Not available via backend yet
    let pcie_gen = None; // Not available via backend yet
    let pcie_width = None; // Not available via backend yet
    let power_limit = None; // Not available via backend yet
    let power_limit_min = None;
    let power_limit_max = None;
    let gpu_clock = backend.get_gpu_clock(0).ok();
    let memory_clock = backend.get_memory_clock(0).ok();
    let architecture = detect_architecture(&info.name);
    let throttle_reason = None; // Not available via backend yet

    Ok(GpuInfo {
        name: info.name,
        driver_version: driver,
        memory_total: mem_total / (1024 * 1024),
        memory_used: mem_used / (1024 * 1024),
        temperature: metrics.temperature,
        power_draw: metrics.power_draw_mw as f32 / 1000.0,
        fan_speed: metrics.fan_speed,
        gpu_utilization: metrics.gpu_utilization,
        memory_utilization: metrics.memory_utilization,
        cuda_compute,
        pcie_gen,
        pcie_width,
        power_limit,
        power_limit_min,
        power_limit_max,
        gpu_clock,
        memory_clock,
        architecture,
        throttle_reason,
    })
}

/// Detect GPU architecture from name
fn detect_architecture(name: &str) -> Option<String> {
    let name_lower = name.to_lowercase();
    if name_lower.contains("5090")
        || name_lower.contains("5080")
        || name_lower.contains("5070")
        || name_lower.contains("5060")
    {
        Some("Blackwell".to_string())
    } else if name_lower.contains("4090")
        || name_lower.contains("4080")
        || name_lower.contains("4070")
        || name_lower.contains("4060")
    {
        Some("Ada Lovelace".to_string())
    } else if name_lower.contains("3090")
        || name_lower.contains("3080")
        || name_lower.contains("3070")
        || name_lower.contains("3060")
    {
        Some("Ampere".to_string())
    } else if name_lower.contains("2080")
        || name_lower.contains("2070")
        || name_lower.contains("2060")
    {
        Some("Turing".to_string())
    } else if name_lower.contains("1080")
        || name_lower.contains("1070")
        || name_lower.contains("1060")
    {
        Some("Pascal".to_string())
    } else if name_lower.contains("1660") || name_lower.contains("1650") {
        Some("Turing (GTX)".to_string())
    } else {
        None
    }
}

/// Get current throttle reason from backend metrics
#[allow(dead_code)]
fn get_throttle_reason_from_metrics(
    backend: &SharedNvmlBackend,
    index: u32,
    power_limit_mw: Option<u32>,
) -> Option<String> {
    let mut reasons = Vec::new();

    // Check temperature-based throttling
    if let Ok(temp) = backend.get_temperature(index) {
        if temp > 83 {
            reasons.push("Thermal");
        }
    }

    // Check power-based throttling
    if let (Ok(power_mw), Some(limit_mw)) = (backend.get_power_usage(index), power_limit_mw) {
        if power_mw > limit_mw {
            reasons.push("Power");
        }
    }

    if reasons.is_empty() {
        None
    } else {
        Some(reasons.join(", "))
    }
}

/// Guard to ensure terminal state is restored even on panic
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
    }
}

pub fn monitor_gpu_stat(backend: &SharedNvmlBackend) {
    if let Err(e) = monitor_gpu_stat_inner(backend) {
        eprintln!("Monitor error: {}", e);
    }
}

fn monitor_gpu_stat_inner(backend: &SharedNvmlBackend) -> NvResult<()> {
    // Setup terminal with proper error handling
    enable_raw_mode()
        .map_err(|e| NvControlError::RuntimeError(format!("Failed to enable raw mode: {}", e)))?;
    let _guard = TerminalGuard; // Ensures cleanup on exit or panic

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|e| {
        NvControlError::RuntimeError(format!("Failed to enter alternate screen: {}", e))
    })?;

    let term_backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(term_backend)
        .map_err(|e| NvControlError::RuntimeError(format!("Failed to create terminal: {}", e)))?;

    let mut last_update = Instant::now();
    let mut gpu_rows: Vec<Vec<String>> = Vec::new();
    let mut uptime = 0u64;
    let mut spinner_idx = 0;
    let spinner = ["|", "/", "-", "\\"];

    loop {
        // Poll for keypress - break on error rather than panic
        match event::poll(Duration::from_millis(200)) {
            Ok(true) => {
                if let Ok(Event::Key(key)) = event::read() {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
            Ok(false) => {}
            Err(_) => break, // Exit gracefully on poll error
        }

        // Refresh stats every second
        if last_update.elapsed() >= Duration::from_secs(1) {
            gpu_rows.clear();
            match backend.device_count() {
                Ok(count) if count > 0 => {
                    for idx in 0..count {
                        let row = build_gpu_row(backend, idx);
                        gpu_rows.push(row);
                    }
                }
                Ok(_) => gpu_rows.push(vec!["No NVIDIA GPU found.".to_string(); 6]),
                Err(_) => gpu_rows.push(vec!["NVML not available.".to_string(); 6]),
            }
            last_update = Instant::now();
            uptime += 1;
            spinner_idx = (spinner_idx + 1) % spinner.len();
        }

        // Draw UI - break on error rather than panic
        if terminal
            .draw(|f| {
                let area = f.area();
                let block = Block::default()
                    .title("nvcontrol GPU Monitor (q to quit)")
                    .borders(Borders::ALL);
                let header = Row::new(vec!["GPU", "Temp", "Fan", "VRAM", "Util", "Power"])
                    .style(Style::default().fg(Color::Yellow));
                let rows = gpu_rows.iter().map(|r| Row::new(r.clone()));
                let column_widths = vec![
                    Constraint::Length(18),
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Length(16),
                    Constraint::Length(8),
                    Constraint::Length(10),
                ];
                let table = Table::new(rows, column_widths).header(header).block(block);
                f.render_widget(table, area);
                // Footer spinner/uptime
                let footer = format!("{} Uptime: {}s", spinner[spinner_idx], uptime);
                let footer_paragraph = Paragraph::new(footer)
                    .alignment(Alignment::Right)
                    .style(Style::default().fg(Color::Gray));
                let footer_rect = ratatui::layout::Rect {
                    x: area.x,
                    y: area.y + area.height.saturating_sub(2),
                    width: area.width,
                    height: 1,
                };
                f.render_widget(footer_paragraph, footer_rect);
            })
            .is_err()
        {
            break;
        }
    }

    // Show cursor before guard cleanup
    let _ = terminal.show_cursor();
    Ok(())
    // TerminalGuard drops here, restoring terminal state
}

/// Build a row of GPU stats for the monitor table
fn build_gpu_row(backend: &SharedNvmlBackend, idx: u32) -> Vec<String> {
    let name = backend
        .get_name(idx)
        .unwrap_or_else(|_| "Unknown".to_string());
    let temp = backend.get_temperature(idx).unwrap_or(0);
    let fan = backend.get_fan_speed(idx, 0).unwrap_or(0);
    let mem_str = match backend.get_memory_info(idx) {
        Ok((used, total)) => format!("{:.1}/{:.1} GB", used as f64 / 1e9, total as f64 / 1e9),
        Err(_) => "N/A".to_string(),
    };
    let util = backend.get_utilization(idx).map(|(g, _)| g).unwrap_or(0);
    let power = backend
        .get_power_usage(idx)
        .map(|p| p as f64 / 1000.0)
        .unwrap_or(0.0);

    vec![
        name,
        format!("{temp}°C"),
        format!("{fan}%"),
        mem_str,
        format!("{util}%"),
        format!("{:.1}W", power),
    ]
}

/// Get GPU info with specified format
pub fn get_gpu_info_with_format(format: OutputFormat, backend: &SharedNvmlBackend) -> NvResult<()> {
    let gpu_info = get_gpu_info(backend)?;

    match format {
        OutputFormat::Human => {
            println!("🖥️  GPU Information:");
            println!("   Name: {}", gpu_info.name);
            if let Some(ref arch) = gpu_info.architecture {
                println!("   Architecture: {}", arch);
            }
            println!("   Driver: {}", gpu_info.driver_version);
            if let Some(ref cuda) = gpu_info.cuda_compute {
                println!("   CUDA Compute: {}", cuda);
            }
            println!(
                "   Memory: {} MB ({} MB used)",
                gpu_info.memory_total, gpu_info.memory_used
            );
            if let (Some(pcie_gen), Some(pcie_width)) = (gpu_info.pcie_gen, gpu_info.pcie_width) {
                println!("   PCIe: Gen{} x{}", pcie_gen, pcie_width);
            }
            println!("   Temperature: {}°C", gpu_info.temperature);
            println!("   Power Draw: {:.1} W", gpu_info.power_draw);
            if let (Some(limit), Some(max)) = (gpu_info.power_limit, gpu_info.power_limit_max) {
                println!("   Power Limit: {:.0} W (max: {:.0} W)", limit, max);
            }
            println!("   Fan Speed: {}%", gpu_info.fan_speed);
            println!("   GPU Utilization: {}%", gpu_info.gpu_utilization);
            println!("   Memory Utilization: {}%", gpu_info.memory_utilization);
            if let (Some(gpu_clk), Some(mem_clk)) = (gpu_info.gpu_clock, gpu_info.memory_clock) {
                println!("   Clocks: {} MHz (GPU) / {} MHz (Mem)", gpu_clk, mem_clk);
            }
            if let Some(ref reason) = gpu_info.throttle_reason {
                println!("   Throttling: {}", reason);
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&gpu_info).unwrap());
        }
        OutputFormat::Table => {
            println!("┌──────────────────────┬──────────────────────────────┐");
            println!("│ Property             │ Value                        │");
            println!("├──────────────────────┼──────────────────────────────┤");
            println!("│ Name                 │ {:<28} │", gpu_info.name);
            if let Some(ref arch) = gpu_info.architecture {
                println!("│ Architecture         │ {:<28} │", arch);
            }
            println!("│ Driver               │ {:<28} │", gpu_info.driver_version);
            if let Some(ref cuda) = gpu_info.cuda_compute {
                println!("│ CUDA Compute         │ {:<28} │", cuda);
            }
            println!(
                "│ Memory               │ {:<28} │",
                format!("{} / {} MB", gpu_info.memory_used, gpu_info.memory_total)
            );
            if let (Some(pcie_gen), Some(pcie_width)) = (gpu_info.pcie_gen, gpu_info.pcie_width) {
                println!(
                    "│ PCIe                 │ {:<28} │",
                    format!("Gen{} x{}", pcie_gen, pcie_width)
                );
            }
            println!(
                "│ Temperature          │ {:<28} │",
                format!("{}°C", gpu_info.temperature)
            );
            println!(
                "│ Power Draw           │ {:<28} │",
                format!("{:.1} W", gpu_info.power_draw)
            );
            if let (Some(limit), Some(max)) = (gpu_info.power_limit, gpu_info.power_limit_max) {
                println!(
                    "│ Power Limit          │ {:<28} │",
                    format!("{:.0} W (max: {:.0} W)", limit, max)
                );
            }
            println!(
                "│ Fan Speed            │ {:<28} │",
                format!("{}%", gpu_info.fan_speed)
            );
            println!(
                "│ GPU Utilization      │ {:<28} │",
                format!("{}%", gpu_info.gpu_utilization)
            );
            println!(
                "│ Memory Utilization   │ {:<28} │",
                format!("{}%", gpu_info.memory_utilization)
            );
            if let (Some(gpu_clk), Some(mem_clk)) = (gpu_info.gpu_clock, gpu_info.memory_clock) {
                println!(
                    "│ Clocks               │ {:<28} │",
                    format!("{} / {} MHz", gpu_clk, mem_clk)
                );
            }
            if let Some(ref reason) = gpu_info.throttle_reason {
                println!("│ Throttling           │ {:<28} │", reason);
            }
            println!("└──────────────────────┴──────────────────────────────┘");
        }
    }

    Ok(())
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
    Table,
}
