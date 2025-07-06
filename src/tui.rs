use crate::{NvResult, vrr};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use nvml_wrapper::{Device, Nvml};
use nvml_wrapper::enums::device::UsedGpuMemory;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Sparkline, Tabs},
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
    vrr_enabled: bool,
    gaming_mode_enabled: bool,
    status_message: Option<String>,
    status_message_time: Option<Instant>,
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
        vec![
            "Overview",
            "Performance",
            "Memory",
            "Temperature",
            "Power",
            "Processes",
        ]
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

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
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

        // Check initial VRR state
        let vrr_enabled = vrr::detect_vrr_displays()
            .map(|displays| displays.iter().any(|d| d.current_settings.enabled))
            .unwrap_or(false);

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
            vrr_enabled,
            gaming_mode_enabled: false,
            status_message: None,
            status_message_time: None,
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
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            break;
                        }
                        KeyCode::Tab => self.next_tab(),
                        KeyCode::BackTab => self.prev_tab(),
                        KeyCode::Left => self.prev_gpu(),
                        KeyCode::Right => self.next_gpu(),
                        KeyCode::Up => self.prev_gpu(),
                        KeyCode::Down => self.next_gpu(),
                        KeyCode::Char('h') => self.show_help = !self.show_help,
                        KeyCode::Char('s') => self.show_settings = !self.show_settings,
                        KeyCode::Char(' ') => self.paused = !self.paused, // Space for pause
                        KeyCode::Char('p') => self.paused = !self.paused, // Also 'p' for pause
                        KeyCode::Char('r') => self.reset_metrics(),
                        // Direct tab navigation
                        KeyCode::Char('1') => self.current_tab = 0,
                        KeyCode::Char('2') => self.current_tab = 1,
                        KeyCode::Char('3') => self.current_tab = 2,
                        KeyCode::Char('4') => self.current_tab = 3,
                        KeyCode::Char('5') => self.current_tab = 4,
                        KeyCode::Char('6') => self.current_tab = 5,
                        // Future features (planned)
                        KeyCode::Char('e') => {
                            // TODO: Export data functionality
                        },
                        KeyCode::Char('f') => {
                            // TODO: Fan control
                        },
                        KeyCode::Char('o') => {
                            // TODO: Overclocking controls
                        },
                        KeyCode::Char('v') => {
                            // VRR toggle
                            self.toggle_vrr();
                        }
                        KeyCode::Char('g') => {
                            // Gaming mode toggle
                            self.toggle_gaming_mode();
                        },
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
        let temperature = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0) as f64;

        let utilization = device
            .utilization_rates()
            .map(|u| (u.gpu as f64, u.memory as f64))
            .unwrap_or((0.0, 0.0));

        let power_draw = device
            .power_usage()
            .map(|p| p as f64 / 1000.0) // mW to W
            .unwrap_or(0.0);

        let fan_speed = device.fan_speed(0).unwrap_or(0) as f64;

        let clocks = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .map(|g| {
                let mem = device
                    .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
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
        let titles: Vec<String> = Tab::titles().into_iter().map(|s| s.to_string()).collect();

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
                let metrics = self
                    .metrics_history
                    .get(self.selected_gpu)
                    .and_then(|h| h.back());

                // Split into sections
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(6), // GPU info
                        Constraint::Length(8), // Quick stats
                        Constraint::Min(0),    // Mini graphs
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
            format!(
                "Memory: {}",
                if let Some(mem) = memory {
                    format!(
                        "{:.1} GB ({:.1} GB used)",
                        mem.total as f64 / 1e9,
                        mem.used as f64 / 1e9
                    )
                } else {
                    "Unknown".to_string()
                }
            ),
            format!(
                "Power Limit: {}",
                if let Some(limit) = power_limit {
                    format!("{:.0} W", limit as f64 / 1000.0)
                } else {
                    "Unknown".to_string()
                }
            ),
        ];

        let list = List::new(info.into_iter().map(ListItem::new).collect::<Vec<_>>())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("GPU Information"),
            )
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
            let gpu_data: Vec<u64> = history.iter().map(|m| m.gpu_utilization as u64).collect();

            let gpu_sparkline = Sparkline::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("GPU Utilization History"),
                )
                .data(&gpu_data)
                .style(Style::default().fg(Color::Green));
            f.render_widget(gpu_sparkline, chunks[0]);

            // Temperature sparkline
            let temp_data: Vec<u64> = history.iter().map(|m| m.temperature as u64).collect();

            let temp_sparkline = Sparkline::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Temperature History"),
                )
                .data(&temp_data)
                .style(Style::default().fg(Color::Red));
            f.render_widget(temp_sparkline, chunks[1]);
        }
    }

    // Additional draw methods for other tabs...
    fn draw_performance(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("Real-time Performance Metrics")
            .block(Block::default().borders(Borders::ALL).title("Performance"))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Green));
        f.render_widget(header, chunks[0]);

        // Performance metrics
        if let Some(ref _nvml) = self.nvml {
            if let Some(history) = self.metrics_history.get(self.selected_gpu as usize) {
                if let Some(latest) = history.back() {
                    let perf_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Percentage(50),
                            Constraint::Percentage(50),
                        ])
                        .split(chunks[1]);

                    // Left side - Current stats
                    let current_stats = vec![
                        format!("GPU Utilization: {:.1}%", latest.gpu_utilization),
                        format!("Memory Utilization: {:.1}%", latest.memory_utilization),
                        format!("Graphics Clock: {} MHz", latest.gpu_clock),
                        format!("Memory Clock: {} MHz", latest.memory_clock),
                        format!("Power Usage: {:.1}W", latest.power_draw),
                        format!("Temperature: {:.1}°C", latest.temperature),
                        format!("Fan Speed: {}%", latest.fan_speed),
                    ];

                    let stats_list: Vec<ListItem> = current_stats.into_iter().map(ListItem::new).collect();
                    let stats_widget = List::new(stats_list)
                        .block(Block::default().borders(Borders::ALL).title("Current Stats"))
                        .style(Style::default().fg(Color::White));
                    f.render_widget(stats_widget, perf_chunks[0]);

                    // Right side - Mini graph
                    if history.len() > 1 {
                        let gpu_data: Vec<u64> = history.iter()
                            .map(|m| m.gpu_utilization as u64)
                            .collect();
                        
                        let sparkline = Sparkline::default()
                            .block(Block::default().borders(Borders::ALL).title("GPU Usage History"))
                            .data(&gpu_data)
                            .style(Style::default().fg(Color::Yellow));
                        f.render_widget(sparkline, perf_chunks[1]);
                    } else {
                        let placeholder = Paragraph::new("Collecting data...")
                            .block(Block::default().borders(Borders::ALL).title("GPU Usage History"))
                            .alignment(Alignment::Center);
                        f.render_widget(placeholder, perf_chunks[1]);
                    }
                } else {
                    let no_data = Paragraph::new("No performance data available")
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL));
                    f.render_widget(no_data, chunks[1]);
                }
            }
        } else {
            let no_nvml = Paragraph::new("NVML not available - install NVIDIA drivers")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Red));
            f.render_widget(no_nvml, chunks[1]);
        }
    }

    fn draw_memory(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("GPU Memory Analysis")
            .block(Block::default().borders(Borders::ALL).title("Memory"))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(header, chunks[0]);

        if let Some(ref nvml) = self.nvml {
            if self.selected_gpu < self.device_count as usize {
                if let Ok(device) = nvml.device_by_index(self.selected_gpu as u32) {
                    let memory_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Percentage(60),
                            Constraint::Percentage(40),
                        ])
                        .split(chunks[1]);

                    // Left side - Memory stats
                    let mut memory_info = Vec::new();
                    
                    if let Ok(mem_info) = device.memory_info() {
                        let used_gb = mem_info.used as f64 / 1024.0 / 1024.0 / 1024.0;
                        let total_gb = mem_info.total as f64 / 1024.0 / 1024.0 / 1024.0;
                        let free_gb = total_gb - used_gb;
                        let usage_percent = (used_gb / total_gb) * 100.0;

                        memory_info.push(format!("Total VRAM: {:.2} GB", total_gb));
                        memory_info.push(format!("Used VRAM: {:.2} GB ({:.1}%)", used_gb, usage_percent));
                        memory_info.push(format!("Free VRAM: {:.2} GB", free_gb));
                    }

                    if let Ok(processes) = device.running_graphics_processes() {
                        memory_info.push(String::new());
                        memory_info.push(format!("Active Processes: {}", processes.len()));
                        
                        for (i, process) in processes.iter().take(5).enumerate() {
                            let mem_mb = match process.used_gpu_memory {
                                UsedGpuMemory::Used(bytes) => bytes as f64 / 1024.0 / 1024.0,
                                UsedGpuMemory::Unavailable => 0.0,
                            };
                            memory_info.push(format!("  Process {}: {:.1} MB", i + 1, mem_mb));
                        }
                        
                        if processes.len() > 5 {
                            memory_info.push(format!("  ... and {} more", processes.len() - 5));
                        }
                    }

                    // Add memory clock info
                    if let Some(history) = self.metrics_history.get(self.selected_gpu as usize) {
                        if let Some(latest) = history.back() {
                            memory_info.push(String::new());
                            memory_info.push(format!("Memory Clock: {} MHz", latest.memory_clock));
                            memory_info.push(format!("Memory Utilization: {:.1}%", latest.memory_utilization));
                        }
                    }

                    let memory_list: Vec<ListItem> = memory_info.into_iter().map(ListItem::new).collect();
                    let memory_widget = List::new(memory_list)
                        .block(Block::default().borders(Borders::ALL).title("Memory Statistics"))
                        .style(Style::default().fg(Color::White));
                    f.render_widget(memory_widget, memory_chunks[0]);

                    // Right side - Memory usage gauge
                    if let Ok(mem_info) = device.memory_info() {
                        let usage_ratio = mem_info.used as f64 / mem_info.total as f64;
                        let gauge = Gauge::default()
                            .block(Block::default().borders(Borders::ALL).title("VRAM Usage"))
                            .gauge_style(
                                Style::default()
                                    .fg(if usage_ratio > 0.9 { Color::Red } 
                                        else if usage_ratio > 0.7 { Color::Yellow } 
                                        else { Color::Green })
                            )
                            .ratio(usage_ratio)
                            .label(format!("{:.1}%", usage_ratio * 100.0));
                        f.render_widget(gauge, memory_chunks[1]);
                    }
                } else {
                    let error = Paragraph::new("Failed to access GPU memory information")
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL))
                        .style(Style::default().fg(Color::Red));
                    f.render_widget(error, chunks[1]);
                }
            }
        } else {
            let no_nvml = Paragraph::new("NVML not available - install NVIDIA drivers")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Red));
            f.render_widget(no_nvml, chunks[1]);
        }
    }

    fn draw_temperature(&self, f: &mut Frame, area: Rect) {
        if self.metrics_history.is_empty() {
            let placeholder = Paragraph::new("No GPU detected")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Temperature"));
            f.render_widget(placeholder, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),   // Current temps
                Constraint::Length(8),   // Temperature history
                Constraint::Min(6),      // Temperature details
            ])
            .split(area);

        // Current temperatures for all GPUs
        let current_temps: Vec<ListItem> = (0..self.metrics_history.len())
            .map(|i| {
                let temp = if let Some(latest) = self.metrics_history[i].back() {
                    latest.temperature
                } else {
                    0.0
                };
                
                let color = if temp > 80.0 {
                    Color::Red
                } else if temp > 70.0 {
                    Color::Yellow
                } else {
                    Color::Green
                };
                
                ListItem::new(format!("GPU {}: {:.1}°C", i, temp))
                    .style(Style::default().fg(color))
            })
            .collect();

        let current_temps_list = List::new(current_temps)
            .block(Block::default().borders(Borders::ALL).title("Current Temperatures"));
        f.render_widget(current_temps_list, chunks[0]);

        // Temperature history sparkline for selected GPU
        if let Some(metrics) = self.metrics_history.get(self.selected_gpu) {
            let temps: Vec<u64> = metrics
                .iter()
                .map(|m| m.temperature as u64)
                .collect();
            
            let max_temp = temps.iter().max().copied().unwrap_or(100).max(100);
            let sparkline = Sparkline::default()
                .block(Block::default().borders(Borders::ALL)
                    .title(format!("GPU {} Temperature History", self.selected_gpu)))
                .data(&temps)
                .max(max_temp)
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(sparkline, chunks[1]);
        }

        // Temperature details and thresholds
        if let Some(latest) = self.metrics_history.get(self.selected_gpu).and_then(|h| h.back()) {
            let temp = latest.temperature;
            let temp_info = format!(
                "GPU {} Temperature Details:\n\n\
                Current: {:.1}°C\n\
                Status: {}\n\
                Thermal Throttling: {}\n\
                Critical Threshold: ~95°C\n\
                Target Threshold: ~83°C\n\n\
                Temperature Guide:\n\
                • < 60°C: Excellent\n\
                • 60-70°C: Good\n\
                • 70-80°C: Warm (normal under load)\n\
                • 80-90°C: Hot (check cooling)\n\
                • > 90°C: Critical (throttling may occur)",
                self.selected_gpu,
                temp,
                if temp < 60.0 { "Excellent" }
                else if temp < 70.0 { "Good" }
                else if temp < 80.0 { "Warm" }
                else if temp < 90.0 { "Hot" }
                else { "Critical" },
                if temp > 83.0 { "Possible" } else { "None" }
            );

            let temp_details = Paragraph::new(temp_info)
                .block(Block::default().borders(Borders::ALL).title("Temperature Analysis"))
                .style(Style::default().fg(
                    if temp > 80.0 { Color::Red }
                    else if temp > 70.0 { Color::Yellow }
                    else { Color::Green }
                ));
            f.render_widget(temp_details, chunks[2]);
        }
    }

    fn draw_power(&self, f: &mut Frame, area: Rect) {
        if self.metrics_history.is_empty() {
            let placeholder = Paragraph::new("No GPU detected")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Power"));
            f.render_widget(placeholder, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),   // Current power usage
                Constraint::Length(8),   // Power history
                Constraint::Min(6),      // Power details
            ])
            .split(area);

        // Current power usage for all GPUs
        let power_items: Vec<ListItem> = (0..self.metrics_history.len())
            .map(|i| {
                let (power, max_power) = if let Some(latest) = self.metrics_history[i].back() {
                    (latest.power_draw, 300.0) // Assuming ~300W typical max
                } else {
                    (0.0, 300.0)
                };
                
                let percentage = (power / max_power * 100.0).min(100.0);
                let color = if percentage > 90.0 {
                    Color::Red
                } else if percentage > 70.0 {
                    Color::Yellow
                } else {
                    Color::Green
                };
                
                ListItem::new(format!("GPU {}: {:.1}W ({:.1}%)", i, power, percentage))
                    .style(Style::default().fg(color))
            })
            .collect();

        let power_list = List::new(power_items)
            .block(Block::default().borders(Borders::ALL).title("Current Power Usage"));
        f.render_widget(power_list, chunks[0]);

        // Power history sparkline for selected GPU
        if let Some(metrics) = self.metrics_history.get(self.selected_gpu) {
            let power_data: Vec<u64> = metrics
                .iter()
                .map(|m| m.power_draw as u64)
                .collect();
            
            let max_power = power_data.iter().max().copied().unwrap_or(300).max(100);
            let sparkline = Sparkline::default()
                .block(Block::default().borders(Borders::ALL)
                    .title(format!("GPU {} Power History", self.selected_gpu)))
                .data(&power_data)
                .max(max_power)
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(sparkline, chunks[1]);
        }

        // Power details and efficiency
        if let Some(latest) = self.metrics_history.get(self.selected_gpu).and_then(|h| h.back()) {
            let power = latest.power_draw;
            let gpu_util = latest.gpu_utilization;
            let efficiency = if power > 0.0 { gpu_util / power * 100.0 } else { 0.0 };
            
            // Calculate average power over last minute
            let recent_power: f64 = self.metrics_history[self.selected_gpu]
                .iter()
                .rev()
                .take(60)
                .map(|m| m.power_draw)
                .sum::<f64>() / (60.0_f64).min(self.metrics_history[self.selected_gpu].len() as f64);

            let power_info = format!(
                "GPU {} Power Analysis:\n\n\
                Current Draw: {:.1}W\n\
                Average (1m): {:.1}W\n\
                Efficiency: {:.2} util/W\n\
                Power State: {}\n\n\
                Power Management:\n\
                • Idle: < 50W\n\
                • Light Load: 50-150W\n\
                • Gaming: 150-250W\n\
                • Heavy Compute: 250W+\n\n\
                Tips:\n\
                • Lower power limits for efficiency\n\
                • Monitor for power throttling\n\
                • Check cooling if power is limited",
                self.selected_gpu,
                power,
                recent_power,
                efficiency,
                if power < 50.0 { "Idle" }
                else if power < 150.0 { "Light Load" }
                else if power < 250.0 { "Gaming" }
                else { "Heavy Compute" }
            );

            let power_details = Paragraph::new(power_info)
                .block(Block::default().borders(Borders::ALL).title("Power Management"))
                .style(Style::default());
            f.render_widget(power_details, chunks[2]);
        }
    }

    fn draw_processes(&self, f: &mut Frame, area: Rect) {
        if self.nvml.is_none() {
            let placeholder = Paragraph::new("NVML not available")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Processes"));
            f.render_widget(placeholder, area);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Header
                Constraint::Min(8),      // Process list
                Constraint::Length(5),   // Summary
            ])
            .split(area);

        // Header with GPU selection
        let header = Paragraph::new(format!("GPU {} Running Processes", self.selected_gpu))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Try to get processes for the selected GPU
        let processes = if let Some(ref nvml) = self.nvml {
            match nvml.device_by_index(self.selected_gpu as u32) {
                Ok(device) => {
                    match device.running_graphics_processes() {
                        Ok(graphics_procs) => {
                            let mut all_processes = Vec::new();
                            
                            // Add graphics processes
                            for proc in graphics_procs {
                                let memory_mb = match proc.used_gpu_memory {
                                    UsedGpuMemory::Used(bytes) => bytes / 1024 / 1024,
                                    UsedGpuMemory::Unavailable => 0,
                                };
                                all_processes.push(format!(
                                    "GFX  PID: {:>6} | VRAM: {:>8} MB | Name: {}",
                                    proc.pid,
                                    memory_mb,
                                    format!("Process {}", proc.pid) // We could enhance this with actual process names
                                ));
                            }
                            
                            // Try to add compute processes
                            if let Ok(compute_procs) = device.running_compute_processes() {
                                for proc in compute_procs {
                                    let memory_mb = match proc.used_gpu_memory {
                                        UsedGpuMemory::Used(bytes) => bytes / 1024 / 1024,
                                        UsedGpuMemory::Unavailable => 0,
                                    };
                                    all_processes.push(format!(
                                        "COMP PID: {:>6} | VRAM: {:>8} MB | Name: {}",
                                        proc.pid,
                                        memory_mb,
                                        format!("Process {}", proc.pid)
                                    ));
                                }
                            }
                            
                            all_processes
                        }
                        Err(_) => vec!["No graphics processes found or permission denied".to_string()],
                    }
                }
                Err(_) => vec!["Failed to access GPU device".to_string()],
            }
        } else {
            vec!["NVML not initialized".to_string()]
        };

        // Process list
        let process_items: Vec<ListItem> = if processes.is_empty() || processes[0].contains("No graphics processes") {
            vec![ListItem::new("No active GPU processes detected")]
        } else {
            processes.into_iter().map(ListItem::new).collect()
        };

        let process_list = List::new(process_items)
            .block(Block::default().borders(Borders::ALL).title("Active Processes"));
        f.render_widget(process_list, chunks[1]);

        // Summary
        let total_procs = if let Some(ref nvml) = self.nvml {
            match nvml.device_by_index(self.selected_gpu as u32) {
                Ok(device) => {
                    let graphics_count = device.running_graphics_processes().map(|p| p.len()).unwrap_or(0);
                    let compute_count = device.running_compute_processes().map(|p| p.len()).unwrap_or(0);
                    graphics_count + compute_count
                }
                Err(_) => 0,
            }
        } else {
            0
        };

        let summary_text = format!(
            "Process Summary:\n\
            Total GPU processes: {}\n\
            Note: Process names require additional permissions\n\
            Use 'nvidia-smi' for detailed process information",
            total_procs
        );

        let summary = Paragraph::new(summary_text)
            .block(Block::default().borders(Borders::ALL).title("Summary"));
        f.render_widget(summary, chunks[2]);
    }

    fn draw_help_popup(&self, f: &mut Frame) {
        let area = centered_rect(70, 80, f.area());
        f.render_widget(Clear, area);

        let help_text = vec![
            "nvcontrol TUI - Complete Keybindings",
            "",
            "Essential Controls:",
            "  q, Esc     - Quit application",
            "  h, F1      - Toggle this help",
            "  s          - Settings panel",
            "  Space, p   - Pause/Resume updates",
            "  r          - Reset metrics history",
            "",
            "Tab Navigation:",
            "  Tab        - Next tab",
            "  Shift+Tab  - Previous tab",
            "  1          - Overview",
            "  2          - Performance (GPU usage history)",
            "  3          - Memory (VRAM usage)",
            "  4          - Temperature (thermal monitoring)",
            "  5          - Power (power consumption)",
            "  6          - Processes (GPU processes)",
            "",
            "GPU Selection (Multi-GPU):",
            "  ←/→        - Previous/Next GPU",
            "  ↑/↓        - Previous/Next GPU",
            "",
            "Advanced Features:",
            "  v          - Toggle VRR/G-Sync",
            "  g          - Toggle Gaming Mode",
            "  f          - Fan controls (planned)",
            "  o          - Overclocking (planned)",
            "  e          - Export current data (planned)",
            "",
            "System:",
            "  Ctrl+C     - Force quit",
            "",
            "Pro Tips:",
            "• Use pause to freeze data for analysis",
            "• Temperature tab shows thermal thresholds",
            "• Power tab shows efficiency metrics",
            "• Each tab shows real-time history",
        ];

        let help = Paragraph::new(help_text.join("\n"))
            .block(Block::default().borders(Borders::ALL).title("Help & Keybindings"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);

        f.render_widget(help, area);
    }

    fn draw_settings_popup(&self, f: &mut Frame) {
        let area = centered_rect(70, 80, f.area());
        f.render_widget(Clear, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Title
                Constraint::Min(10),     // Settings content
                Constraint::Length(3),   // Controls
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Settings & Configuration")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        // Settings content
        let settings_text = format!(
            "Current Configuration:\n\n\
            Selected GPU: {}\n\
            Update Interval: {:.1}s\n\
            Display Mode: {}\n\
            History Length: {} samples\n\n\
            Display Settings:\n\
            • Temperature Units: Celsius\n\
            • Power Units: Watts\n\
            • Memory Units: MB/GB\n\
            • Graph Style: Sparklines\n\n\
            Monitoring:\n\
            • Real-time Updates: {}\n\
            • Data Collection: {}\n\
            • Auto-refresh: Enabled\n\n\
            Controls:\n\
            • Tab Navigation: ←/→ or 1-6\n\
            • GPU Selection: ↑/↓\n\
            • Pause/Resume: Space\n\
            • Refresh Rate: +/- (planned)\n\n\
            Advanced:\n\
            • Export Data: E (planned)\n\
            • Save Profile: S (planned)\n\
            • Load Profile: L (planned)\n\
            • Screenshot: P (planned)",
            self.selected_gpu,
            self.update_interval.as_secs_f64(),
            if self.paused { "Paused" } else { "Live" },
            MAX_HISTORY,
            if self.paused { "Paused" } else { "Active" },
            if self.paused { "Paused" } else { "Active" }
        );

        let settings_content = Paragraph::new(settings_text)
            .block(Block::default().borders(Borders::ALL).title("Configuration Details"))
            .style(Style::default())
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(settings_content, chunks[1]);

        // Controls
        let controls = Paragraph::new("Press 's' again to close settings, 'r' to reset to defaults")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(controls, chunks[2]);
    }

    fn draw_status_bar(&self, f: &mut Frame, area: Rect) {
        // Split status bar into multiple sections
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ])
            .split(area);

        // Left section: main status
        let main_status = if self.paused {
            "PAUSED - Press Space to resume"
        } else if let Some(msg) = self.get_status_message() {
            msg
        } else {
            "Press 'h' for help, 'q' to quit, 'v' for VRR, 'g' for gaming mode"
        };

        let status_bar = Paragraph::new(main_status)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left);
        f.render_widget(status_bar, chunks[0]);

        // Middle section: feature status
        let vrr_status = if self.vrr_enabled { "VRR: ON" } else { "VRR: OFF" };
        let gaming_status = if self.gaming_mode_enabled { "Gaming: ON" } else { "Gaming: OFF" };
        let feature_status = format!("{} | {}", vrr_status, gaming_status);
        
        let feature_bar = Paragraph::new(feature_status)
            .style(Style::default().fg(if self.vrr_enabled || self.gaming_mode_enabled {
                Color::Green
            } else {
                Color::Gray
            }))
            .alignment(Alignment::Center);
        f.render_widget(feature_bar, chunks[1]);

        // Right section: GPU selection
        let gpu_info = if self.device_count > 1 {
            format!("GPU {}/{}", self.selected_gpu + 1, self.device_count)
        } else {
            "GPU 1/1".to_string()
        };
        
        let gpu_bar = Paragraph::new(gpu_info)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Right);
        f.render_widget(gpu_bar, chunks[2]);
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
    
    fn toggle_vrr(&mut self) {
        match vrr::detect_vrr_displays() {
            Ok(displays) => {
                if let Some(display) = displays.first() {
                    let new_settings = vrr::VrrSettings {
                        enabled: !self.vrr_enabled,
                        ..display.current_settings.clone()
                    };
                    
                    match vrr::apply_vrr_settings(&display.display_name, &new_settings) {
                        Ok(()) => {
                            self.vrr_enabled = !self.vrr_enabled;
                            let status = if self.vrr_enabled {
                                "VRR enabled"
                            } else {
                                "VRR disabled"
                            };
                            self.set_status_message(status.to_string());
                        }
                        Err(e) => {
                            self.set_status_message(format!("VRR toggle failed: {}", e));
                        }
                    }
                } else {
                    self.set_status_message("No VRR-capable displays found".to_string());
                }
            }
            Err(e) => {
                self.set_status_message(format!("VRR detection failed: {}", e));
            }
        }
    }
    
    fn toggle_gaming_mode(&mut self) {
        self.gaming_mode_enabled = !self.gaming_mode_enabled;
        
        if self.gaming_mode_enabled {
            // Apply gaming optimizations
            match self.apply_gaming_optimizations() {
                Ok(()) => {
                    self.set_status_message("Gaming mode enabled".to_string());
                }
                Err(e) => {
                    self.gaming_mode_enabled = false;
                    self.set_status_message(format!("Gaming mode failed: {}", e));
                }
            }
        } else {
            self.set_status_message("Gaming mode disabled".to_string());
        }
    }
    
    fn apply_gaming_optimizations(&self) -> NvResult<()> {
        // Apply latency optimizations
        crate::latency::optimize_latency()?;
        
        // Enable performance fan profile if available
        let profiles = crate::fan::load_fan_profiles()?;
        if let Some(perf_profile) = profiles.iter().find(|p| p.name == "Performance") {
            // Apply performance fan profile
            for (fan_id, curve) in &perf_profile.curves {
                // Get current temperature for fan curve application
                if let Some(ref nvml) = self.nvml {
                    if let Ok(device) = nvml.device_by_index(0) {
                        if let Ok(temp) = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
                            let _ = crate::fan::apply_fan_curve(*fan_id, curve, temp as u8, None);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_message_time = Some(Instant::now());
    }
    
    fn get_status_message(&self) -> Option<&str> {
        if let Some(ref time) = self.status_message_time {
            if time.elapsed() < Duration::from_secs(3) {
                self.status_message.as_deref()
            } else {
                None
            }
        } else {
            None
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
