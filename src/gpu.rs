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
            println!("   Driver: {}", gpu_info.driver_version);
            println!("   Memory: {} MB", gpu_info.memory_total);
            println!("   Temperature: {}°C", gpu_info.temperature);
            println!("   Power Draw: {} W", gpu_info.power_draw);
            println!("   Fan Speed: {}%", gpu_info.fan_speed);
            println!("   GPU Utilization: {}%", gpu_info.gpu_utilization);
            println!("   Memory Utilization: {}%", gpu_info.memory_utilization);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&gpu_info).unwrap());
        }
        OutputFormat::Table => {
            println!("┌─────────────────────┬─────────────────────────┐");
            println!("│ Property            │ Value                   │");
            println!("├─────────────────────┼─────────────────────────┤");
            println!("│ Name                │ {:<23} │", gpu_info.name);
            println!("│ Driver              │ {:<23} │", gpu_info.driver_version);
            println!(
                "│ Memory              │ {:<23} │",
                format!("{} MB", gpu_info.memory_total)
            );
            println!(
                "│ Temperature         │ {:<23} │",
                format!("{}°C", gpu_info.temperature)
            );
            println!(
                "│ Power Draw          │ {:<23} │",
                format!("{} W", gpu_info.power_draw)
            );
            println!(
                "│ Fan Speed           │ {:<23} │",
                format!("{}%", gpu_info.fan_speed)
            );
            println!(
                "│ GPU Utilization     │ {:<23} │",
                format!("{}%", gpu_info.gpu_utilization)
            );
            println!(
                "│ Memory Utilization  │ {:<23} │",
                format!("{}%", gpu_info.memory_utilization)
            );
            println!("└─────────────────────┴─────────────────────────┘");
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
