use crate::NvResult;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use nvml_wrapper::{Nvml, Device};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, 
        Paragraph, Sparkline, Tabs
    },
    Frame, Terminal,
};
use std::collections::VecDeque;
use std::io;
use std::time::{Duration, Instant};

const MAX_HISTORY: usize = 120; // 2 minutes at 1Hz

#[derive(Clone)]
pub struct GpuMetrics {
    pub timestamp: Instant,
    pub temperature: f64,
    pub gpu_utilization: f64,
    pub memory_utilization: f64,
    pub power_draw: f64,
    pub fan_speed: f64,
    pub gpu_clock: f64,
    pub memory_clock: f64,
}

pub struct TuiApp {
    nvml: Option<Nvml>,
    device_count: u32,
    metrics_history: Vec<VecDeque<GpuMetrics>>,
    current_tab: usize,
    selected_gpu: usize,
    show_help: bool,
    show_settings: bool,
    paused: bool,
    update_interval: Duration,
    start_time: Instant,
}

#[derive(Clone, Copy)]
enum Tab {
    Overview,
    Performance,
    Memory,
    Temperature,
    Power,
    Processes,
}

impl Tab {
    fn titles() -> Vec<&'static str> {
        vec!["Overview", "Performance", "Memory", "Temperature", "Power", "Processes"]
    }
    
    fn from_index(index: usize) -> Self {
        match index {
            0 => Tab::Overview,
            1 => Tab::Performance,
            2 => Tab::Memory,
            3 => Tab::Temperature,
            4 => Tab::Power,
            5 => Tab::Processes,
            _ => Tab::Overview,
        }
    }
}

impl TuiApp {
    pub fn new() -> Self {
        let nvml = Nvml::init().ok();
        let device_count = if let Some(ref nvml) = nvml {
            nvml.device_count().unwrap_or(0)
        } else {
            0
        };
        
        let metrics_history = (0..device_count)
            .map(|_| VecDeque::with_capacity(MAX_HISTORY))
            .collect();

        Self {
            nvml,
            device_count,
            metrics_history,
            current_tab: 0,
            selected_gpu: 0,
            show_help: false,
            show_settings: false,
            paused: false,
            update_interval: Duration::from_secs(1),
            start_time: Instant::now(),
        }
    }

    pub fn run(&mut self) -> NvResult<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut last_update = Instant::now();
        
        loop {
            // Handle input events
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                        KeyCode::Tab => self.next_tab(),
                        KeyCode::BackTab => self.prev_tab(),
                        KeyCode::Left => self.prev_gpu(),
                        KeyCode::Right => self.next_gpu(),
                        KeyCode::Char('h') => self.show_help = !self.show_help,
                        KeyCode::Char('s') => self.show_settings = !self.show_settings,
                        KeyCode::Char('p') => self.paused = !self.paused,
                        KeyCode::Char('r') => self.reset_metrics(),
                        _ => {}
                    }
                }
            }

            // Update metrics
            if !self.paused && last_update.elapsed() >= self.update_interval {
                self.update_metrics();
                last_update = Instant::now();
            }

            // Draw UI
            terminal.draw(|f| self.draw(f))?;
        }

        // Cleanup
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn update_metrics(&mut self) {
        if let Some(ref nvml) = self.nvml {
            for gpu_id in 0..self.device_count {
                if let Ok(device) = nvml.device_by_index(gpu_id) {
                    if let Ok(metrics) = self.get_device_metrics(&device) {
                        if let Some(history) = self.metrics_history.get_mut(gpu_id as usize) {
                            history.push_back(metrics);
                            if history.len() > MAX_HISTORY {
                                history.pop_front();
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_device_metrics(&self, device: &Device) -> NvResult<GpuMetrics> {
        let temperature = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0) as f64;
        
        let utilization = device.utilization_rates()
            .map(|u| (u.gpu as f64, u.memory as f64))
            .unwrap_or((0.0, 0.0));
        
        let power_draw = device.power_usage()
            .map(|p| p as f64 / 1000.0) // mW to W
            .unwrap_or(0.0);
        
        let fan_speed = device.fan_speed(0)
            .unwrap_or(0) as f64;
        
        let clocks = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .map(|g| {
                let mem = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
                    .unwrap_or(0);
                (g as f64, mem as f64)
            })
            .unwrap_or((0.0, 0.0));

        Ok(GpuMetrics {
            timestamp: Instant::now(),
            temperature,
            gpu_utilization: utilization.0,
            memory_utilization: utilization.1,
            power_draw,
            fan_speed,
            gpu_clock: clocks.0,
            memory_clock: clocks.1,
        })
    }

    fn draw(&self, f: &mut Frame) {
        let size = f.area();

        // Show help overlay
        if self.show_help {
            self.draw_help_popup(f);
            return;
        }

        // Show settings overlay
        if self.show_settings {
            self.draw_settings_popup(f);
            return;
        }

        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Tabs
                Constraint::Min(0),    // Content
                Constraint::Length(1), // Status bar
            ])
            .split(size);

        // Header
        self.draw_header(f, chunks[0]);
        
        // Tabs
        self.draw_tabs(f, chunks[1]);
        
        // Content based on current tab
        match Tab::from_index(self.current_tab) {
            Tab::Overview => self.draw_overview(f, chunks[2]),
            Tab::Performance => self.draw_performance(f, chunks[2]),
            Tab::Memory => self.draw_memory(f, chunks[2]),
            Tab::Temperature => self.draw_temperature(f, chunks[2]),
            Tab::Power => self.draw_power(f, chunks[2]),
            Tab::Processes => self.draw_processes(f, chunks[2]),
        }
        
        // Status bar
        self.draw_status_bar(f, chunks[3]);
    }

    fn draw_header(&self, f: &mut Frame, area: Rect) {
        let gpu_count = self.device_count;
        let uptime = self.start_time.elapsed().as_secs();
        let status = if self.paused { "PAUSED" } else { "LIVE" };
        
        let title = format!(
            "nvcontrol GPU Monitor - {} GPU(s) | {} | Uptime: {}s",
            gpu_count, status, uptime
        );

        let header = Paragraph::new(title)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Cyan));
        
        f.render_widget(header, area);
    }

    fn draw_tabs(&self, f: &mut Frame, area: Rect) {
        let titles: Vec<String> = Tab::titles()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.current_tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow));

        f.render_widget(tabs, area);
    }

    fn draw_overview(&self, f: &mut Frame, area: Rect) {
        if self.device_count == 0 {
            let placeholder = Paragraph::new("No NVIDIA GPUs detected")
                .block(Block::default().borders(Borders::ALL).title("GPU Overview"));
            f.render_widget(placeholder, area);
            return;
        }

        if let Some(ref nvml) = self.nvml {
            if let Ok(device) = nvml.device_by_index(self.selected_gpu as u32) {
                let metrics = self.metrics_history.get(self.selected_gpu).and_then(|h| h.back());

                // Split into sections
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(6),  // GPU info
                        Constraint::Length(8),  // Quick stats
                        Constraint::Min(0),     // Mini graphs
                    ])
                    .split(area);

                // GPU Information
                self.draw_gpu_info(f, chunks[0], &device);

                // Quick Stats Gauges
                if let Some(metrics) = metrics {
                    self.draw_quick_stats(f, chunks[1], metrics);
                }

                // Mini graphs
                self.draw_mini_graphs(f, chunks[2]);
            }
        }
    }

    fn draw_gpu_info(&self, f: &mut Frame, area: Rect, device: &Device) {
        let name = device.name().unwrap_or("Unknown GPU".to_string());
        let memory = device.memory_info().ok();
        let power_limit = device.power_management_limit_default().ok();
        
        let info = vec![
            format!("Name: {}", name),
            format!("Memory: {}", 
                if let Some(mem) = memory {
                    format!("{:.1} GB ({:.1} GB used)", 
                        mem.total as f64 / 1e9,
                        mem.used as f64 / 1e9)
                } else {
                    "Unknown".to_string()
                }
            ),
            format!("Power Limit: {}", 
                if let Some(limit) = power_limit {
                    format!("{:.0} W", limit as f64 / 1000.0)
                } else {
                    "Unknown".to_string()
                }
            ),
        ];

        let list = List::new(
            info.into_iter()
                .map(|line| ListItem::new(line))
                .collect::<Vec<_>>()
        )
        .block(Block::default().borders(Borders::ALL).title("GPU Information"))
        .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    fn draw_quick_stats(&self, f: &mut Frame, area: Rect, metrics: &GpuMetrics) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        // GPU Utilization
        let gpu_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("GPU"))
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(metrics.gpu_utilization / 100.0)
            .label(format!("{:.0}%", metrics.gpu_utilization));
        f.render_widget(gpu_gauge, chunks[0]);

        // Memory Utilization
        let mem_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("VRAM"))
            .gauge_style(Style::default().fg(Color::Blue))
            .ratio(metrics.memory_utilization / 100.0)
            .label(format!("{:.0}%", metrics.memory_utilization));
        f.render_widget(mem_gauge, chunks[1]);

        // Temperature
        let temp_color = if metrics.temperature > 80.0 {
            Color::Red
        } else if metrics.temperature > 70.0 {
            Color::Yellow
        } else {
            Color::Green
        };
        let temp_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Temp"))
            .gauge_style(Style::default().fg(temp_color))
            .ratio((metrics.temperature / 100.0).min(1.0))
            .label(format!("{:.0}°C", metrics.temperature));
        f.render_widget(temp_gauge, chunks[2]);

        // Power
        let power_gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Power"))
            .gauge_style(Style::default().fg(Color::Magenta))
            .ratio((metrics.power_draw / 400.0).min(1.0)) // Assume 400W max
            .label(format!("{:.0}W", metrics.power_draw));
        f.render_widget(power_gauge, chunks[3]);
    }

    fn draw_mini_graphs(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // GPU Utilization sparkline
        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            let gpu_data: Vec<u64> = history.iter()
                .map(|m| m.gpu_utilization as u64)
                .collect();
            
            let gpu_sparkline = Sparkline::default()
                .block(Block::default().borders(Borders::ALL).title("GPU Utilization History"))
                .data(&gpu_data)
                .style(Style::default().fg(Color::Green));
            f.render_widget(gpu_sparkline, chunks[0]);

            // Temperature sparkline
            let temp_data: Vec<u64> = history.iter()
                .map(|m| m.temperature as u64)
                .collect();
            
            let temp_sparkline = Sparkline::default()
                .block(Block::default().borders(Borders::ALL).title("Temperature History"))
                .data(&temp_data)
                .style(Style::default().fg(Color::Red));
            f.render_widget(temp_sparkline, chunks[1]);
        }
    }

    // Additional draw methods for other tabs...
    fn draw_performance(&self, f: &mut Frame, area: Rect) {
        // TODO: Implement detailed performance graphs
        let placeholder = Paragraph::new("Performance graphs - Coming soon!")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Performance"));
        f.render_widget(placeholder, area);
    }

    fn draw_memory(&self, f: &mut Frame, area: Rect) {
        // TODO: Implement memory usage breakdown
        let placeholder = Paragraph::new("Memory analysis - Coming soon!")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Memory"));
        f.render_widget(placeholder, area);
    }

    fn draw_temperature(&self, f: &mut Frame, area: Rect) {
        // TODO: Implement temperature monitoring
        let placeholder = Paragraph::new("Temperature monitoring - Coming soon!")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Temperature"));
        f.render_widget(placeholder, area);
    }

    fn draw_power(&self, f: &mut Frame, area: Rect) {
        // TODO: Implement power analysis
        let placeholder = Paragraph::new("Power analysis - Coming soon!")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Power"));
        f.render_widget(placeholder, area);
    }

    fn draw_processes(&self, f: &mut Frame, area: Rect) {
        // TODO: Implement process monitoring
        let placeholder = Paragraph::new("GPU processes - Coming soon!")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Processes"));
        f.render_widget(placeholder, area);
    }

    fn draw_help_popup(&self, f: &mut Frame) {
        let area = centered_rect(60, 70, f.area());
        f.render_widget(Clear, area);

        let help_text = vec![
            "nvcontrol TUI - Keybindings",
            "",
            "Navigation:",
            "  q, Esc     - Quit application",
            "  h, F1      - Toggle this help",
            "  s          - Settings",
            "  Space      - Pause/Resume updates",
            "  r          - Reset metrics history",
            "",
            "Tabs:",
            "  Tab        - Next tab",
            "  Shift+Tab  - Previous tab",
            "  1-6        - Jump to specific tab",
            "",
            "GPU Selection:",
            "  ←/→        - Previous/Next GPU",
            "",
            "Other:",
            "  Ctrl+C     - Force quit",
        ];

        let help = Paragraph::new(help_text.join("\n"))
            .block(Block::default().borders(Borders::ALL).title("Help"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);

        f.render_widget(help, area);
    }

    fn draw_settings_popup(&self, f: &mut Frame) {
        let area = centered_rect(50, 50, f.area());
        f.render_widget(Clear, area);

        let settings = Paragraph::new("Settings panel - Coming soon!")
            .block(Block::default().borders(Borders::ALL).title("Settings"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        f.render_widget(settings, area);
    }

    fn draw_status_bar(&self, f: &mut Frame, area: Rect) {
        let status = if self.paused {
            "PAUSED - Press Space to resume"
        } else {
            "Press 'h' for help, 'q' to quit"
        };

        let status_bar = Paragraph::new(status)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);

        f.render_widget(status_bar, area);
    }

    // Navigation methods
    fn next_tab(&mut self) {
        let tab_count = Tab::titles().len();
        self.current_tab = (self.current_tab + 1) % tab_count;
    }

    fn prev_tab(&mut self) {
        let tab_count = Tab::titles().len();
        self.current_tab = if self.current_tab == 0 {
            tab_count - 1
        } else {
            self.current_tab - 1
        };
    }

    fn next_gpu(&mut self) {
        if self.device_count > 0 {
            self.selected_gpu = (self.selected_gpu + 1) % self.device_count as usize;
        }
    }

    fn prev_gpu(&mut self) {
        if self.device_count > 0 {
            self.selected_gpu = if self.selected_gpu == 0 {
                self.device_count as usize - 1
            } else {
                self.selected_gpu - 1
            };
        }
    }

    fn reset_metrics(&mut self) {
        for history in &mut self.metrics_history {
            history.clear();
        }
    }
}

// Helper function to center a rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// Error conversion
impl From<io::Error> for crate::NvControlError {
    fn from(error: io::Error) -> Self {
        crate::NvControlError::DisplayDetectionFailed(format!("IO error: {}", error))
    }
}