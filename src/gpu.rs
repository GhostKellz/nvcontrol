use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph, Table, Row},
    layout::{Alignment, Constraint},
    style::{Style, Color},
};
use nvml_wrapper::Nvml;

pub fn get_gpu_info() {
    // TODO: Query NVIDIA GPU info
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
                Ok(nvml) => {
                    match nvml.device_count() {
                        Ok(count) => {
                            for idx in 0..count {
                                let row = match nvml.device_by_index(idx) {
                                    Ok(device) => {
                                        let name = device.name().unwrap_or("Unknown".to_string());
                                        let temp = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).unwrap_or(0);
                                        let fan = device.fan_speed(0).unwrap_or(0);
                                        let mem = device.memory_info().ok();
                                        let mem_str = if let Some(m) = mem {
                                            format!("{:.1}/{:.1} GB", m.used as f64 / 1e9, m.total as f64 / 1e9)
                                        } else {
                                            "N/A".to_string()
                                        };
                                        let util = device.utilization_rates().map(|u| u.gpu).unwrap_or(0);
                                        let power = device.power_usage().map(|p| p as f64 / 1000.0).unwrap_or(0.0);
                                        vec![name, format!("{temp}°C"), format!("{fan}%"), mem_str, format!("{util}%"), format!("{:.1}W", power)]
                                    }
                                    Err(_) => vec!["No NVIDIA GPU found.".to_string(); 6],
                                };
                                gpu_rows.push(row);
                            }
                        }
                        Err(_) => gpu_rows.push(vec!["No NVIDIA GPU found.".to_string(); 6]),
                    }
                }
                Err(_) => gpu_rows.push(vec!["NVML not available.".to_string(); 6]),
            }
            last_update = Instant::now();
            uptime += 1;
            spinner_idx = (spinner_idx + 1) % spinner.len();
        }

        // Draw UI
        terminal.draw(|f| {
            let area = f.area();
            let block = Block::default().title("nvcontrol GPU Monitor (q to quit)").borders(Borders::ALL);
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
            let table = Table::new(rows, column_widths)
                .header(header)
                .block(block);
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
        }).unwrap();
    }

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}