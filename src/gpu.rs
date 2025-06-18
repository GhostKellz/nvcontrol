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
use std::time::{Duration, Instant};

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

pub fn get_gpu_info() {
    match Nvml::init() {
        Ok(nvml) => match nvml.device_count() {
            Ok(count) => {
                println!("Found {count} NVIDIA GPU(s):");
                for idx in 0..count {
                    if let Ok(device) = nvml.device_by_index(idx) {
                        let name = device.name().unwrap_or("Unknown".to_string());
                        let driver = nvml.sys_driver_version().unwrap_or("Unknown".to_string());
                        let mem = device.memory_info().ok();
                        let mem_str = if let Some(m) = mem {
                            format!("{:.1} GB", m.total as f64 / 1e9)
                        } else {
                            "Unknown".to_string()
                        };
                        let temp = device
                            .temperature(
                                nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu,
                            )
                            .unwrap_or(0);
                        let power_state = device
                            .performance_state()
                            .map(|p| format!("{p:?}"))
                            .unwrap_or("Unknown".to_string());

                        println!("  GPU {idx}: {name}");
                        println!("    Driver: {driver}");
                        println!("    VRAM: {mem_str}");
                        println!("    Temperature: {temp}°C");
                        println!("    Power State: {power_state}");
                        println!();
                    }
                }
            }
            Err(e) => eprintln!("Failed to get GPU count: {e}"),
        },
        Err(e) => {
            eprintln!("NVML not available: {e}");
            eprintln!("Falling back to nvidia-smi...");

            // Fallback to nvidia-smi
            match std::process::Command::new("nvidia-smi")
                .arg("--query-gpu=name,driver_version,memory.total,temperature.gpu,power.state")
                .arg("--format=csv,noheader,nounits")
                .output()
            {
                Ok(output) => {
                    if output.status.success() {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        println!("GPU Information (via nvidia-smi):");
                        for (idx, line) in output_str.lines().enumerate() {
                            let fields: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                            if fields.len() >= 5 {
                                println!("  GPU {}: {}", idx, fields[0]);
                                println!("    Driver: {}", fields[1]);
                                println!("    VRAM: {} MB", fields[2]);
                                println!("    Temperature: {}°C", fields[3]);
                                println!("    Power State: {}", fields[4]);
                                println!();
                            }
                        }
                    } else {
                        eprintln!("nvidia-smi failed");
                    }
                }
                Err(_) => eprintln!("nvidia-smi not found. Please install NVIDIA drivers."),
            }
        }
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
