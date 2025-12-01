use crate::{NvControlError, NvResult};
use clap::ValueEnum;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use nvml_wrapper::Nvml;
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
pub fn is_nvidia_available() -> bool {
    match Nvml::init() {
        Ok(nvml) => match nvml.device_count() {
            Ok(count) => count > 0,
            Err(_) => false,
        },
        Err(_) => false,
    }
}

pub fn get_gpu_info() -> NvResult<GpuInfo> {
    match Nvml::init() {
        Ok(nvml) => match nvml.device_count() {
            Ok(count) => {
                if count == 0 {
                    return Err(NvControlError::GpuQueryFailed(
                        "No NVIDIA GPUs found".to_string(),
                    ));
                }

                // Get info for first GPU
                if let Ok(device) = nvml.device_by_index(0) {
                    let name = device.name().unwrap_or("Unknown".to_string());
                    let driver = nvml.sys_driver_version().unwrap_or("Unknown".to_string());
                    let mem = device.memory_info().unwrap_or_else(|_| {
                        nvml_wrapper::struct_wrappers::device::MemoryInfo {
                            total: 0,
                            free: 0,
                            used: 0,
                            reserved: 0,
                            version: 0,
                        }
                    });
                    let temp = device
                        .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                        .unwrap_or(0);

                    // Get utilization if available
                    let utilization = device.utilization_rates().unwrap_or_else(|_| {
                        nvml_wrapper::struct_wrappers::device::Utilization { gpu: 0, memory: 0 }
                    });

                    // Get CUDA compute capability
                    let cuda_compute = device
                        .cuda_compute_capability()
                        .ok()
                        .map(|cc| format!("{}.{}", cc.major, cc.minor));

                    // Get PCIe info
                    let pcie_gen = device.current_pcie_link_gen().ok();
                    let pcie_width = device.current_pcie_link_width().ok();

                    // Get power limits
                    let power_limit = device
                        .power_management_limit()
                        .ok()
                        .map(|p| p as f32 / 1000.0);
                    let power_limit_constraints = device.power_management_limit_constraints().ok();
                    let power_limit_min = power_limit_constraints
                        .as_ref()
                        .map(|c| c.min_limit as f32 / 1000.0);
                    let power_limit_max = power_limit_constraints
                        .as_ref()
                        .map(|c| c.max_limit as f32 / 1000.0);

                    // Get clock speeds
                    let gpu_clock = device
                        .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
                        .ok();
                    let memory_clock = device
                        .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
                        .ok();

                    // Detect architecture from name
                    let architecture = detect_architecture(&name);

                    // Get throttle reason
                    let throttle_reason = get_throttle_reason(&device);

                    Ok(GpuInfo {
                        name,
                        driver_version: driver,
                        memory_total: mem.total / (1024 * 1024), // Convert to MB
                        memory_used: mem.used / (1024 * 1024),
                        temperature: temp,
                        power_draw: device.power_usage().unwrap_or(0) as f32 / 1000.0, // Convert to watts
                        fan_speed: device.fan_speed(0).unwrap_or(0),
                        gpu_utilization: utilization.gpu,
                        memory_utilization: utilization.memory,
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
                } else {
                    Err(NvControlError::GpuQueryFailed(
                        "Failed to access GPU".to_string(),
                    ))
                }
            }
            Err(e) => Err(NvControlError::GpuQueryFailed(format!(
                "Failed to get device count: {}",
                e
            ))),
        },
        Err(e) => Err(NvControlError::GpuQueryFailed(format!(
            "Failed to initialize NVML: {}",
            e
        ))),
    }
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

/// Get current throttle reason from device
fn get_throttle_reason(device: &nvml_wrapper::Device) -> Option<String> {
    // Check various throttle reasons
    let mut reasons = Vec::new();

    // Try to get current throttle reasons via supported clocks
    if let Ok(temp) =
        device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
    {
        if temp > 83 {
            reasons.push("Thermal");
        }
    }

    if let Ok(power) = device.power_usage() {
        if let Ok(limit) = device.power_management_limit() {
            if power > limit {
                reasons.push("Power");
            }
        }
    }

    if reasons.is_empty() {
        None
    } else {
        Some(reasons.join(", "))
    }
}

pub fn monitor_gpu_stat() {
    // Setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Try to init NVML
    let nvml = Nvml::init();
    let mut last_update = Instant::now();
    let mut gpu_rows: Vec<Vec<String>> = Vec::new();
    let mut uptime = 0u64;
    let mut spinner_idx = 0;
    let spinner = ["|", "/", "-", "\\"];

    loop {
        // Poll for keypress
        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Refresh stats every second
        if last_update.elapsed() >= Duration::from_secs(1) {
            gpu_rows.clear();
            match &nvml {
                Ok(nvml) => match nvml.device_count() {
                    Ok(count) => {
                        for idx in 0..count {
                            let row = match nvml.device_by_index(idx) {
                                Ok(device) => {
                                    let name = device.name().unwrap_or("Unknown".to_string());
                                    let temp = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).unwrap_or(0);
                                    let fan = device.fan_speed(0).unwrap_or(0);
                                    let mem = device.memory_info().ok();
                                    let mem_str = if let Some(m) = mem {
                                        format!(
                                            "{:.1}/{:.1} GB",
                                            m.used as f64 / 1e9,
                                            m.total as f64 / 1e9
                                        )
                                    } else {
                                        "N/A".to_string()
                                    };
                                    let util =
                                        device.utilization_rates().map(|u| u.gpu).unwrap_or(0);
                                    let power = device
                                        .power_usage()
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
                                Err(_) => vec!["No NVIDIA GPU found.".to_string(); 6],
                            };
                            gpu_rows.push(row);
                        }
                    }
                    Err(_) => gpu_rows.push(vec!["No NVIDIA GPU found.".to_string(); 6]),
                },
                Err(_) => gpu_rows.push(vec!["NVML not available.".to_string(); 6]),
            }
            last_update = Instant::now();
            uptime += 1;
            spinner_idx = (spinner_idx + 1) % spinner.len();
        }

        // Draw UI
        terminal
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
            .unwrap();
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}

/// Get GPU info with specified format
pub fn get_gpu_info_with_format(format: OutputFormat) -> NvResult<()> {
    let gpu_info = get_gpu_info()?;

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
