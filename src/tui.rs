use crate::{
    NvControlError, NvResult, asus_power_detector, gui_tuner, nvidia_profiler, themes, vrr,
};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEvent,
        MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::{Device, Nvml};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Sparkline, Tabs},
};
use std::collections::VecDeque;
use std::io::{self, IsTerminal};
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
    fan_control_mode: bool,
    oc_control_mode: bool,
    #[allow(dead_code)]
    fan_speed_target: u32,
    current_theme: themes::ThemeVariant,
    theme: themes::ColorPalette,
    // OC controls
    gpu_offset: i32,          // -200 to +200 MHz
    memory_offset: i32,       // -1000 to +1000 MHz
    power_limit_percent: u32, // 50 to 100%
    oc_preset: OcPreset,
    // Fan curve editing
    fan_curve_points: Vec<(u32, u32)>, // (temp°C, fan%)
    selected_curve_point: usize,
    // Tuner state (MSI Afterburner-style)
    tuner_states: Vec<gui_tuner::TunerState>,
    // Profiler state (radeon-profile equivalent)
    profiler: Option<nvidia_profiler::NvidiaProfiler>,
    profiler_recording: bool,
    // OSD/MangoHud state (reserved for future interactivity)
    #[allow(dead_code)]
    osd_enabled: bool,
    #[allow(dead_code)]
    osd_selected_metric: usize,
}

#[derive(Clone, Copy)]
enum Tab {
    Overview,
    Performance,
    Memory,
    Temperature,
    Power,
    Processes,
    Overclocking,
    FanControl,
    Profiles,
    Tuner,    // MSI Afterburner-style tuner
    Profiler, // GPU profiler (radeon-profile equivalent)
    Osd,      // MangoHud OSD configuration
    Settings, // Settings panel
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum OcPreset {
    Stock,
    MildOc,
    Performance,
    Extreme,
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
            "Overclock",
            "Fan Control",
            "Profiles",
            "Tuner",
            "Profiler",
            "OSD",
            "Settings",
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
            6 => Tab::Overclocking,
            7 => Tab::FanControl,
            8 => Tab::Profiles,
            9 => Tab::Tuner,
            10 => Tab::Profiler,
            11 => Tab::Osd,
            12 => Tab::Settings,
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

        // Auto-detect theme based on GPU vendor
        let current_theme = Self::detect_gpu_vendor_theme(&nvml);
        let theme = themes::ColorPalette::from_variant(current_theme);

        // Default fan curve (performance)
        let fan_curve_points = vec![
            (30, 20),  // 30°C -> 20%
            (50, 40),  // 50°C -> 40%
            (70, 60),  // 70°C -> 60%
            (80, 80),  // 80°C -> 80%
            (90, 100), // 90°C -> 100%
        ];

        // Initialize tuner states for each GPU
        let tuner_states = (0..device_count)
            .map(|gpu_id| gui_tuner::TunerState::new(gpu_id))
            .collect();

        // Initialize profiler (for selected GPU initially)
        let profiler = if device_count > 0 {
            Some(nvidia_profiler::NvidiaProfiler::new(0, 100, 10000))
        } else {
            None
        };

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
            fan_control_mode: false,
            oc_control_mode: false,
            fan_speed_target: 50,
            current_theme,
            theme,
            gpu_offset: 0,
            memory_offset: 0,
            power_limit_percent: 80,
            oc_preset: OcPreset::Stock,
            fan_curve_points,
            selected_curve_point: 0,
            tuner_states,
            profiler,
            profiler_recording: false,
            osd_enabled: crate::osd::OsdManager::check_mangohud_installed(),
            osd_selected_metric: 0,
        }
    }

    pub fn run(&mut self) -> NvResult<()> {
        // Check if we have a real terminal
        if !io::stdout().is_terminal() {
            return Err(NvControlError::RuntimeError(
                "TUI requires a terminal (TTY). Run this command in a terminal emulator like Ghostty, Kitty, Alacritty, etc.".to_string()
            ));
        }

        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut last_update = Instant::now();

        loop {
            // Handle input events
            if event::poll(Duration::from_millis(100))? {
                let event = event::read()?;

                // Handle mouse events
                if let Event::Mouse(mouse) = event {
                    self.handle_mouse_event(mouse);
                    continue;
                }

                // Handle keyboard events
                if let Event::Key(key) = event {
                    // Handle OC mode controls first
                    if self.oc_control_mode && self.current_tab == 6 {
                        // Tab 6 is Overclocking
                        match key.code {
                            KeyCode::Left => {
                                self.gpu_offset = (self.gpu_offset - 10).max(-200);
                                self.set_status_message(format!(
                                    "GPU Offset: {:+} MHz",
                                    self.gpu_offset
                                ));
                                continue;
                            }
                            KeyCode::Right => {
                                self.gpu_offset = (self.gpu_offset + 10).min(200);
                                self.set_status_message(format!(
                                    "GPU Offset: {:+} MHz",
                                    self.gpu_offset
                                ));
                                continue;
                            }
                            KeyCode::Up => {
                                self.memory_offset = (self.memory_offset + 50).min(1000);
                                self.set_status_message(format!(
                                    "Memory Offset: {:+} MHz",
                                    self.memory_offset
                                ));
                                continue;
                            }
                            KeyCode::Down => {
                                self.memory_offset = (self.memory_offset - 50).max(-1000);
                                self.set_status_message(format!(
                                    "Memory Offset: {:+} MHz",
                                    self.memory_offset
                                ));
                                continue;
                            }
                            KeyCode::Char('+') | KeyCode::Char('=') => {
                                self.power_limit_percent = (self.power_limit_percent + 5).min(100);
                                self.set_status_message(format!(
                                    "Power Limit: {}%",
                                    self.power_limit_percent
                                ));
                                continue;
                            }
                            KeyCode::Char('-') | KeyCode::Char('_') => {
                                self.power_limit_percent = (self.power_limit_percent - 5).max(50);
                                self.set_status_message(format!(
                                    "Power Limit: {}%",
                                    self.power_limit_percent
                                ));
                                continue;
                            }
                            KeyCode::Char('1') => {
                                self.apply_oc_preset(OcPreset::Stock);
                                continue;
                            }
                            KeyCode::Char('2') => {
                                self.apply_oc_preset(OcPreset::MildOc);
                                continue;
                            }
                            KeyCode::Char('3') => {
                                self.apply_oc_preset(OcPreset::Performance);
                                continue;
                            }
                            KeyCode::Char('4') => {
                                self.apply_oc_preset(OcPreset::Extreme);
                                continue;
                            }
                            KeyCode::Enter => {
                                self.apply_overclock();
                                continue;
                            }
                            KeyCode::Char('o') | KeyCode::Esc => {
                                self.oc_control_mode = false;
                                self.set_status_message("OC Mode disabled".to_string());
                                continue;
                            }
                            _ => {}
                        }
                    }

                    // Handle fan control mode
                    if self.fan_control_mode && self.current_tab == 7 {
                        // Tab 7 is Fan Control
                        match key.code {
                            KeyCode::Left => {
                                if self.selected_curve_point > 0 {
                                    self.selected_curve_point -= 1;
                                }
                                continue;
                            }
                            KeyCode::Right => {
                                if self.selected_curve_point < self.fan_curve_points.len() - 1 {
                                    self.selected_curve_point += 1;
                                }
                                continue;
                            }
                            KeyCode::Up => {
                                let idx = self.selected_curve_point;
                                if let Some(point) = self.fan_curve_points.get_mut(idx) {
                                    point.1 = (point.1 + 5).min(100);
                                    let temp = point.0;
                                    let fan = point.1;
                                    self.set_status_message(format!(
                                        "Fan curve point: {}°C -> {}%",
                                        temp, fan
                                    ));
                                }
                                continue;
                            }
                            KeyCode::Down => {
                                let idx = self.selected_curve_point;
                                if let Some(point) = self.fan_curve_points.get_mut(idx) {
                                    point.1 = (point.1.saturating_sub(5)).max(0);
                                    let temp = point.0;
                                    let fan = point.1;
                                    self.set_status_message(format!(
                                        "Fan curve point: {}°C -> {}%",
                                        temp, fan
                                    ));
                                }
                                continue;
                            }
                            KeyCode::Enter => {
                                self.apply_fan_curve();
                                continue;
                            }
                            KeyCode::Char('f') | KeyCode::Esc => {
                                self.fan_control_mode = false;
                                self.set_status_message("Fan Control Mode disabled".to_string());
                                continue;
                            }
                            _ => {}
                        }
                    }

                    // Normal key handling
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
                        KeyCode::Char('7') => self.current_tab = 6,
                        KeyCode::Char('8') => self.current_tab = 7,
                        KeyCode::Char('9') => self.current_tab = 8,
                        // Feature hotkeys
                        KeyCode::Char('e') => {
                            // Export data to JSON
                            self.export_metrics();
                        }
                        KeyCode::Char('f') => {
                            // Open fan control mode
                            self.fan_control_mode = !self.fan_control_mode;
                        }
                        KeyCode::Char('o') => {
                            // Open overclocking controls
                            self.oc_control_mode = !self.oc_control_mode;
                            if self.oc_control_mode {
                                self.set_status_message("OC Mode: Use ←/→ GPU offset, ↑/↓ Memory offset, +/- Power, 1-4 Presets, Enter to Apply".to_string());
                            } else {
                                self.set_status_message("OC Mode disabled".to_string());
                            }
                        }
                        KeyCode::Char('v') => {
                            // VRR toggle
                            self.toggle_vrr();
                        }
                        KeyCode::Char('g') => {
                            // Gaming mode toggle
                            self.toggle_gaming_mode();
                        }
                        KeyCode::Char('t') => {
                            // Cycle through themes
                            self.cycle_theme();
                        }
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
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
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
            Tab::Overclocking => self.draw_overclocking(f, chunks[2]),
            Tab::FanControl => self.draw_fan_control(f, chunks[2]),
            Tab::Profiles => self.draw_profiles(f, chunks[2]),
            Tab::Tuner => self.draw_tuner(f, chunks[2]),
            Tab::Profiler => self.draw_profiler(f, chunks[2]),
            Tab::Osd => self.draw_osd(f, chunks[2]),
            Tab::Settings => self.draw_settings(f, chunks[2]),
        }

        // Status bar
        self.draw_status_bar(f, chunks[3]);
    }

    fn draw_header(&self, f: &mut Frame, area: Rect) {
        let uptime = self.start_time.elapsed().as_secs();
        let status = if self.paused {
            "󰏤 PAUSED"
        } else {
            "󰐊 LIVE"
        };

        // Get current GPU stats for header display
        let gpu_stats = self
            .metrics_history
            .get(self.selected_gpu)
            .and_then(|h| h.back());

        let stats_str = if let Some(m) = gpu_stats {
            format!(
                "{}°C | {}% | {:.0}W",
                m.temperature as i32, m.gpu_utilization as i32, m.power_draw
            )
        } else {
            "-- | -- | --".to_string()
        };

        let title = format!(
            "{} nvcontrol v0.7.1 │ GPU {} │ {} │ {} │ {} │ {}",
            themes::icons::GPU,
            self.selected_gpu,
            stats_str,
            status,
            format!("{}m {}s", uptime / 60, uptime % 60),
            self.current_theme.name()
        );

        let header = Paragraph::new(title)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(self.theme.primary().to_ratatui())
                    .add_modifier(Modifier::BOLD),
            );

        f.render_widget(header, area);
    }

    fn draw_tabs(&self, f: &mut Frame, area: Rect) {
        // Tab titles with icons for better visual identification
        let tab_icons = [
            "󰍹 Overview",    // 0
            "󰓅 Performance", // 1
            "󰍛 Memory",      // 2
            "󱃂 Temperature", // 3
            "󰚥 Power",       // 4
            "󰕮 Processes",   // 5
            "󰓸 Overclock",   // 6
            "󰈐 Fan Control", // 7
            "󰆼 Profiles",    // 8
            "󰔎 Tuner",       // 9
            "󰄪 Profiler",    // 10
            "󰕧 OSD",         // 11
            "󰒓 Settings",    // 12
        ];

        let titles: Vec<String> = tab_icons.iter().map(|s| s.to_string()).collect();

        // Use different divider style for better visual separation
        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} Navigation ", themes::icons::TAB))
                    .title_style(
                        Style::default()
                            .fg(self.theme.primary().to_ratatui())
                            .add_modifier(Modifier::BOLD),
                    )
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .select(self.current_tab)
            .style(Style::default().fg(self.theme.text().to_ratatui()))
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(self.theme.accent().to_ratatui())
                    .add_modifier(Modifier::BOLD),
            )
            .divider("│");

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

        // GPU Utilization with usage-based color
        let gpu_color = if metrics.gpu_utilization > 90.0 {
            self.theme.usage_high.to_ratatui()
        } else if metrics.gpu_utilization > 50.0 {
            self.theme.usage_medium.to_ratatui()
        } else {
            self.theme.usage_low.to_ratatui()
        };
        let gpu_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} GPU", themes::icons::GPU))
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .gauge_style(Style::default().fg(gpu_color))
            .ratio(metrics.gpu_utilization / 100.0)
            .label(format!("{:.0}%", metrics.gpu_utilization));
        f.render_widget(gpu_gauge, chunks[0]);

        // Memory Utilization
        let mem_color = if metrics.memory_utilization > 90.0 {
            self.theme.usage_high.to_ratatui()
        } else if metrics.memory_utilization > 70.0 {
            self.theme.usage_medium.to_ratatui()
        } else {
            self.theme.usage_low.to_ratatui()
        };
        let mem_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} VRAM", themes::icons::MEMORY))
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .gauge_style(Style::default().fg(mem_color))
            .ratio(metrics.memory_utilization / 100.0)
            .label(format!("{:.0}%", metrics.memory_utilization));
        f.render_widget(mem_gauge, chunks[1]);

        // Temperature with themed colors
        let temp_color = if metrics.temperature > 80.0 {
            self.theme.temp_hot.to_ratatui()
        } else if metrics.temperature > 70.0 {
            self.theme.temp_warm.to_ratatui()
        } else if metrics.temperature > 50.0 {
            self.theme.temp_normal.to_ratatui()
        } else {
            self.theme.temp_cold.to_ratatui()
        };
        let temp_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} Temp", themes::icons::TEMP))
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .gauge_style(Style::default().fg(temp_color))
            .ratio((metrics.temperature / 100.0).min(1.0))
            .label(format!("{:.0}°C", metrics.temperature));
        f.render_widget(temp_gauge, chunks[2]);

        // Power with themed colors
        let power_color = if metrics.power_draw > 500.0 {
            self.theme.power_high.to_ratatui()
        } else if metrics.power_draw > 250.0 {
            self.theme.power_normal.to_ratatui()
        } else {
            self.theme.power_efficient.to_ratatui()
        };
        let power_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} Power", themes::icons::POWER))
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .gauge_style(Style::default().fg(power_color))
            .ratio((metrics.power_draw / 600.0).min(1.0)) // RTX 5090 max
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
            .constraints([Constraint::Length(3), Constraint::Min(0)])
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
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
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

                    let stats_list: Vec<ListItem> =
                        current_stats.into_iter().map(ListItem::new).collect();
                    let stats_widget = List::new(stats_list)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title("Current Stats"),
                        )
                        .style(Style::default().fg(Color::White));
                    f.render_widget(stats_widget, perf_chunks[0]);

                    // Right side - Mini graph
                    if history.len() > 1 {
                        let gpu_data: Vec<u64> =
                            history.iter().map(|m| m.gpu_utilization as u64).collect();

                        let sparkline = Sparkline::default()
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("GPU Usage History"),
                            )
                            .data(&gpu_data)
                            .style(Style::default().fg(Color::Yellow));
                        f.render_widget(sparkline, perf_chunks[1]);
                    } else {
                        let placeholder = Paragraph::new("Collecting data...")
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("GPU Usage History"),
                            )
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
            .constraints([Constraint::Length(3), Constraint::Min(0)])
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
                        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                        .split(chunks[1]);

                    // Left side - Memory stats
                    let mut memory_info = Vec::new();

                    if let Ok(mem_info) = device.memory_info() {
                        let used_gb = mem_info.used as f64 / 1024.0 / 1024.0 / 1024.0;
                        let total_gb = mem_info.total as f64 / 1024.0 / 1024.0 / 1024.0;
                        let free_gb = total_gb - used_gb;
                        let usage_percent = (used_gb / total_gb) * 100.0;

                        memory_info.push(format!("Total VRAM: {:.2} GB", total_gb));
                        memory_info.push(format!(
                            "Used VRAM: {:.2} GB ({:.1}%)",
                            used_gb, usage_percent
                        ));
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
                            memory_info.push(format!(
                                "Memory Utilization: {:.1}%",
                                latest.memory_utilization
                            ));
                        }
                    }

                    let memory_list: Vec<ListItem> =
                        memory_info.into_iter().map(ListItem::new).collect();
                    let memory_widget = List::new(memory_list)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title("Memory Statistics"),
                        )
                        .style(Style::default().fg(Color::White));
                    f.render_widget(memory_widget, memory_chunks[0]);

                    // Right side - Memory usage gauge
                    if let Ok(mem_info) = device.memory_info() {
                        let usage_ratio = mem_info.used as f64 / mem_info.total as f64;
                        let gauge = Gauge::default()
                            .block(Block::default().borders(Borders::ALL).title("VRAM Usage"))
                            .gauge_style(Style::default().fg(if usage_ratio > 0.9 {
                                Color::Red
                            } else if usage_ratio > 0.7 {
                                Color::Yellow
                            } else {
                                Color::Green
                            }))
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
                Constraint::Length(8), // Current temps
                Constraint::Length(8), // Temperature history
                Constraint::Min(6),    // Temperature details
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

                ListItem::new(format!("GPU {}: {:.1}°C", i, temp)).style(Style::default().fg(color))
            })
            .collect();

        let current_temps_list = List::new(current_temps).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Temperatures"),
        );
        f.render_widget(current_temps_list, chunks[0]);

        // Temperature history sparkline for selected GPU
        if let Some(metrics) = self.metrics_history.get(self.selected_gpu) {
            let temps: Vec<u64> = metrics.iter().map(|m| m.temperature as u64).collect();

            let max_temp = temps.iter().max().copied().unwrap_or(100).max(100);
            let sparkline = Sparkline::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("GPU {} Temperature History", self.selected_gpu)),
                )
                .data(&temps)
                .max(max_temp)
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(sparkline, chunks[1]);
        }

        // Temperature details and thresholds
        if let Some(latest) = self
            .metrics_history
            .get(self.selected_gpu)
            .and_then(|h| h.back())
        {
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
                if temp < 60.0 {
                    "Excellent"
                } else if temp < 70.0 {
                    "Good"
                } else if temp < 80.0 {
                    "Warm"
                } else if temp < 90.0 {
                    "Hot"
                } else {
                    "Critical"
                },
                if temp > 83.0 { "Possible" } else { "None" }
            );

            let temp_details = Paragraph::new(temp_info)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Temperature Analysis"),
                )
                .style(Style::default().fg(if temp > 80.0 {
                    Color::Red
                } else if temp > 70.0 {
                    Color::Yellow
                } else {
                    Color::Green
                }));
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
                Constraint::Length(8),  // Current power usage
                Constraint::Length(8),  // Power history
                Constraint::Length(10), // Power details
                Constraint::Min(6),     // ASUS Power Detector (if available)
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

        let power_list = List::new(power_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Current Power Usage"),
        );
        f.render_widget(power_list, chunks[0]);

        // Power history sparkline for selected GPU
        if let Some(metrics) = self.metrics_history.get(self.selected_gpu) {
            let power_data: Vec<u64> = metrics.iter().map(|m| m.power_draw as u64).collect();

            let max_power = power_data.iter().max().copied().unwrap_or(300).max(100);
            let sparkline = Sparkline::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("GPU {} Power History", self.selected_gpu)),
                )
                .data(&power_data)
                .max(max_power)
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(sparkline, chunks[1]);
        }

        // Power details and efficiency
        if let Some(latest) = self
            .metrics_history
            .get(self.selected_gpu)
            .and_then(|h| h.back())
        {
            let power = latest.power_draw;
            let gpu_util = latest.gpu_utilization;
            let efficiency = if power > 0.0 {
                gpu_util / power * 100.0
            } else {
                0.0
            };

            // Calculate average power over last minute
            let recent_power: f64 = self.metrics_history[self.selected_gpu]
                .iter()
                .rev()
                .take(60)
                .map(|m| m.power_draw)
                .sum::<f64>()
                / (60.0_f64).min(self.metrics_history[self.selected_gpu].len() as f64);

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
                if power < 50.0 {
                    "Idle"
                } else if power < 150.0 {
                    "Light Load"
                } else if power < 250.0 {
                    "Gaming"
                } else {
                    "Heavy Compute"
                }
            );

            let power_details = Paragraph::new(power_info)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Power Management"),
                )
                .style(Style::default());
            f.render_widget(power_details, chunks[2]);
        }

        // ASUS Power Detector+ (ROG GPUs only)
        self.draw_asus_power_detector(f, chunks[3]);
    }

    fn draw_asus_power_detector(&self, f: &mut Frame, area: Rect) {
        // Try to detect ASUS ROG GPUs
        let asus_gpus = asus_power_detector::detect_asus_gpus();

        if asus_gpus.is_empty() {
            let placeholder = Paragraph::new("No ASUS ROG GPU detected")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::DarkGray))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("ASUS Power Detector+"),
                );
            f.render_widget(placeholder, area);
            return;
        }

        // Get first ASUS GPU
        let (pci_id, model) = &asus_gpus[0];

        // Try to create detector and read power rails
        let power_info = match asus_power_detector::AsusPowerDetector::new(pci_id) {
            Ok(detector) => {
                if !detector.is_supported() {
                    format!("{}: Power Detector+ not available", model.name())
                } else {
                    match detector.read_power_rails() {
                        Ok(status) => {
                            let health_str = match status.health {
                                asus_power_detector::PowerHealth::Good => "✓ GOOD",
                                asus_power_detector::PowerHealth::Warning => "⚠ WARNING",
                                asus_power_detector::PowerHealth::Critical => "✗ CRITICAL",
                                asus_power_detector::PowerHealth::Unknown => "? UNKNOWN",
                            };

                            let rails_str: Vec<String> = status
                                .rails
                                .iter()
                                .map(|r| {
                                    let current = r
                                        .current_ma
                                        .map(|c| format!("{:.2}A", c as f32 / 1000.0))
                                        .unwrap_or_else(|| "N/A".to_string());
                                    format!("Rail {}: {}", r.rail_id, current)
                                })
                                .collect();

                            format!(
                                "12V-2x6 Connector Health: {}\n\n{}\n\nTotal Power: {:.1}W",
                                health_str,
                                rails_str.join("  │  "),
                                status.total_power_w.unwrap_or(0.0)
                            )
                        }
                        Err(e) => format!("Read error: {}", e),
                    }
                }
            }
            Err(e) => format!("Detector init failed: {}", e),
        };

        // Color based on health
        let title_color = if power_info.contains("✓ GOOD") {
            Color::Green
        } else if power_info.contains("⚠ WARNING") {
            Color::Yellow
        } else if power_info.contains("✗ CRITICAL") {
            Color::Red
        } else {
            Color::Gray
        };

        let asus_widget = Paragraph::new(power_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("ASUS Power Detector+ - {}", asus_gpus[0].1.name()))
                    .border_style(Style::default().fg(title_color)),
            )
            .style(Style::default());
        f.render_widget(asus_widget, area);
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
                Constraint::Length(3), // Header
                Constraint::Min(8),    // Process list
                Constraint::Length(5), // Summary
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
                        Err(_) => {
                            vec!["No graphics processes found or permission denied".to_string()]
                        }
                    }
                }
                Err(_) => vec!["Failed to access GPU device".to_string()],
            }
        } else {
            vec!["NVML not initialized".to_string()]
        };

        // Process list
        let process_items: Vec<ListItem> =
            if processes.is_empty() || processes[0].contains("No graphics processes") {
                vec![ListItem::new("No active GPU processes detected")]
            } else {
                processes.into_iter().map(ListItem::new).collect()
            };

        let process_list = List::new(process_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Active Processes"),
        );
        f.render_widget(process_list, chunks[1]);

        // Summary
        let total_procs = if let Some(ref nvml) = self.nvml {
            match nvml.device_by_index(self.selected_gpu as u32) {
                Ok(device) => {
                    let graphics_count = device
                        .running_graphics_processes()
                        .map(|p| p.len())
                        .unwrap_or(0);
                    let compute_count = device
                        .running_compute_processes()
                        .map(|p| p.len())
                        .unwrap_or(0);
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

    fn draw_overclocking(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(12), // Current OC status
                Constraint::Min(10),    // OC controls
                Constraint::Length(5),  // Warning/Info
            ])
            .split(area);

        // Header
        let header = Paragraph::new(format!(
            "{} Overclocking & Performance Tuning",
            themes::icons::OC
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(self.theme.border.to_ratatui())),
        )
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(self.theme.warning().to_ratatui())
                .add_modifier(Modifier::BOLD),
        );
        f.render_widget(header, chunks[0]);

        // Current OC status
        if let Some(ref nvml) = self.nvml {
            if let Ok(device) = nvml.device_by_index(self.selected_gpu as u32) {
                let mut oc_info = Vec::new();

                oc_info.push(format!(
                    "{} Current Overclocking Status:",
                    themes::icons::GPU
                ));
                oc_info.push(String::new());

                // Current clocks
                if let Ok(gpu_clock) =
                    device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
                {
                    if let Ok(max_clock) =
                        device.max_clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
                    {
                        oc_info.push(format!(
                            "  {} GPU Clock: {} MHz (Max: {} MHz)",
                            themes::icons::CLOCK,
                            gpu_clock,
                            max_clock
                        ));
                    }
                }

                if let Ok(mem_clock) =
                    device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
                {
                    if let Ok(max_mem) =
                        device.max_clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
                    {
                        oc_info.push(format!(
                            "  {} Memory Clock: {} MHz (Max: {} MHz)",
                            themes::icons::MEMORY,
                            mem_clock,
                            max_mem
                        ));
                    }
                }

                // Power limit
                if let Ok(power_limit) = device.power_management_limit() {
                    if let Ok(max_power) = device.power_management_limit_constraints() {
                        oc_info.push(format!(
                            "  {} Power Limit: {:.0}W / {:.0}W",
                            themes::icons::POWER,
                            power_limit as f32 / 1000.0,
                            max_power.max_limit as f32 / 1000.0
                        ));
                    }
                }

                oc_info.push(String::new());
                oc_info.push(format!("{} Current OC Settings:", themes::icons::PROFILE));
                oc_info.push(format!("  GPU Offset: {:+} MHz", self.gpu_offset));
                oc_info.push(format!("  Memory Offset: {:+} MHz", self.memory_offset));
                oc_info.push(format!("  Power Limit: {}%", self.power_limit_percent));
                oc_info.push(format!("  Active Preset: {:?}", self.oc_preset));

                let oc_list: Vec<ListItem> = oc_info.into_iter().map(ListItem::new).collect();
                let oc_widget = List::new(oc_list)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Overclocking Status")
                            .style(Style::default().fg(self.theme.border.to_ratatui())),
                    )
                    .style(Style::default().fg(self.theme.text().to_ratatui()));
                f.render_widget(oc_widget, chunks[1]);
            }
        }

        // OC Controls with live sliders
        let gpu_slider = self.render_slider(self.gpu_offset, -200, 200, 40);
        let mem_slider = self.render_slider(self.memory_offset, -1000, 1000, 40);
        let power_slider = self.render_slider(self.power_limit_percent as i32, 50, 100, 40);

        let mode_indicator = if self.oc_control_mode {
            format!(
                "{} OC MODE ACTIVE - Use arrow keys to adjust",
                themes::icons::WARNING
            )
        } else {
            "Press 'o' to enter OC mode".to_string()
        };

        let controls_text = format!(
            "{} Interactive Overclocking Controls\n\n\
            GPU Offset:    [-200] {} [+200] {:+} MHz\n\
            Memory Offset: [-1000] {} [+1000] {:+} MHz\n\
            Power Limit:   [50%] {} [100%] {}%\n\n\
            {} Presets: [1] Stock  [2] Mild OC  [3] Performance  [4] Extreme\n\n\
            {}\n\
            ←/→: GPU offset  ↑/↓: Memory offset  +/-: Power  Enter: Apply\n\n\
            {} For Arch + RTX 5090:\n\
            • GDDR7 safe: +1500 MHz memory\n\
            • GPU boost: +150-200 MHz typical\n\
            • Power: 600W TDP (630W max for ASUS Astral)",
            themes::icons::OC,
            gpu_slider,
            self.gpu_offset,
            mem_slider,
            self.memory_offset,
            power_slider,
            self.power_limit_percent,
            themes::icons::PROFILE,
            mode_indicator,
            themes::icons::INFO
        );

        let controls = Paragraph::new(controls_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Overclock Controls")
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .style(Style::default().fg(self.theme.text().to_ratatui()))
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(controls, chunks[2]);

        // Warning
        let warning = Paragraph::new(format!(
            "{} CAUTION: Overclocking may void warranty and cause instability. Monitor temps carefully!",
            themes::icons::WARNING
        ))
            .block(Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(self.theme.border.to_ratatui())))
            .style(Style::default().fg(self.theme.error().to_ratatui()))
            .alignment(Alignment::Center);
        f.render_widget(warning, chunks[3]);
    }

    fn draw_fan_control(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(8), // Current fan status
                Constraint::Min(10),   // Fan curve editor
                Constraint::Length(4), // Presets
            ])
            .split(area);

        // Header
        let header = Paragraph::new(format!("{} Fan Control & Curves", themes::icons::FAN))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(self.theme.primary().to_ratatui())
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(header, chunks[0]);

        // Current fan status
        if let Some(ref nvml) = self.nvml {
            if let Ok(device) = nvml.device_by_index(self.selected_gpu as u32) {
                let mut fan_info = Vec::new();

                // Try to get fan speed for multiple fans (ASUS Astral has 4 fans)
                for fan_id in 0..4 {
                    if let Ok(fan_speed) = device.fan_speed(fan_id) {
                        let fan_icon = if fan_speed > 80 {
                            "󰈐" // High speed
                        } else if fan_speed > 50 {
                            "󰈐" // Medium speed
                        } else {
                            "󰈐" // Low speed
                        };
                        fan_info.push(format!(
                            "  {} Fan {}: {}% ({} RPM est.)",
                            fan_icon,
                            fan_id,
                            fan_speed,
                            fan_speed * 25
                        ));
                    }
                }

                if fan_info.is_empty() {
                    fan_info.push("  Fan control not available on this GPU".to_string());
                }

                fan_info.push(String::new());
                if let Some(latest) = self
                    .metrics_history
                    .get(self.selected_gpu)
                    .and_then(|h| h.back())
                {
                    let temp_icon = if latest.temperature > 80.0 { "" } else { "" };
                    fan_info.push(format!(
                        "  {} Current Temperature: {:.1}°C",
                        temp_icon, latest.temperature
                    ));
                }

                let fan_list: Vec<ListItem> = fan_info.into_iter().map(ListItem::new).collect();
                let fan_widget = List::new(fan_list)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Current Fan Status")
                            .style(Style::default().fg(self.theme.border.to_ratatui())),
                    )
                    .style(Style::default().fg(self.theme.text().to_ratatui()));
                f.render_widget(fan_widget, chunks[1]);
            }
        }

        // Fan curve editor with actual curve points
        let mut curve_display = Vec::new();
        curve_display.push(format!("{} Fan Curve Editor (Live)", themes::icons::CHART));
        curve_display.push(String::new());

        // Display current curve points
        for (i, (temp, fan)) in self.fan_curve_points.iter().enumerate() {
            let marker = if i == self.selected_curve_point && self.fan_control_mode {
                "►"
            } else {
                " "
            };
            curve_display.push(format!("{}  {}°C -> {}%", marker, temp, fan));
        }

        curve_display.push(String::new());
        let mode_indicator = if self.fan_control_mode {
            format!(
                "{} FAN MODE ACTIVE - Use arrow keys to adjust selected point",
                themes::icons::WARNING
            )
        } else {
            "Press 'f' to enter fan curve mode".to_string()
        };
        curve_display.push(mode_indicator);
        curve_display.push("←/→: Select point  ↑/↓: Adjust fan %  Enter: Apply".to_string());
        curve_display.push(String::new());
        curve_display.push(format!("{} ASUS ROG Astral Quad-Fan:", themes::icons::INFO));
        curve_display.push("• 4 independent fans for optimal cooling".to_string());
        curve_display.push("• Per-fan curve support".to_string());
        curve_display.push("• 0 RPM mode available (fans stop when cool)".to_string());

        let curve_text = curve_display.join("\n");

        let curve = Paragraph::new(curve_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Fan Curve Configuration")
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .style(Style::default().fg(self.theme.text().to_ratatui()))
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(curve, chunks[2]);

        // Fan presets
        let presets = Paragraph::new(format!(
            "{} Quick Presets: [Silent] [Auto] [Performance] [Aggressive] [0 RPM Mode]",
            themes::icons::PROFILE
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(self.theme.border.to_ratatui())),
        )
        .style(Style::default().fg(self.theme.accent().to_ratatui()))
        .alignment(Alignment::Center);
        f.render_widget(presets, chunks[3]);
    }

    fn draw_profiles(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(12),   // Profile list
                Constraint::Length(8), // Current profile details
                Constraint::Length(4), // Actions
            ])
            .split(area);

        // Header
        let header = Paragraph::new(format!("{} Performance Profiles", themes::icons::PROFILE))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(self.theme.primary().to_ratatui())
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(header, chunks[0]);

        // Profile list
        let profiles = vec![
            format!(
                "{} Silent - Low power, quiet operation (50% power, stock clocks)",
                themes::icons::SUCCESS
            ),
            format!(
                "{} Balanced - Default balanced performance",
                themes::icons::INFO
            ),
            format!(
                "{} Performance - Higher clocks, increased power limit",
                themes::icons::OC
            ),
            format!(
                "{} Extreme - Maximum overclock for RTX 5090 (+200 GPU, +1500 MEM, 100% power)",
                themes::icons::WARNING
            ),
            String::new(),
            "Game-Specific Profiles:".to_string(),
            "  Cyberpunk 2077 - DLSS 4 enabled, 4K Ultra preset".to_string(),
            "  Counter-Strike 2 - Competitive mode, low latency".to_string(),
            "  Stable Diffusion - Power limit 90%, memory OC".to_string(),
            String::new(),
            format!(
                "{} Press Enter to apply selected profile",
                themes::icons::POWER
            ),
            format!("{} Press 'n' to create new profile", themes::icons::PROFILE),
            format!("{} Press 'd' to delete profile", themes::icons::WARNING),
        ];

        let profile_items: Vec<ListItem> = profiles.into_iter().map(ListItem::new).collect();
        let profile_list = List::new(profile_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Available Profiles")
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .style(Style::default().fg(self.theme.text().to_ratatui()))
            .highlight_style(
                Style::default()
                    .fg(self.theme.accent().to_ratatui())
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(profile_list, chunks[1]);

        // Current profile details
        let details_text = format!(
            "{} Currently Active: Balanced\n\n\
            • GPU Offset: +0 MHz\n\
            • Memory Offset: +0 MHz\n\
            • Power Limit: 80%\n\
            • Fan Curve: Auto\n\
            • Digital Vibrance: 130% (Gaming preset)",
            themes::icons::INFO
        );

        let details = Paragraph::new(details_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Active Profile Settings")
                    .style(Style::default().fg(self.theme.border.to_ratatui())),
            )
            .style(Style::default().fg(self.theme.text().to_ratatui()));
        f.render_widget(details, chunks[2]);

        // Actions
        let actions = Paragraph::new(format!(
            "{} Auto-apply profiles per game | {} Save current settings as profile | {} Export/Import profiles",
            themes::icons::GAMING,
            themes::icons::PROFILE,
            themes::icons::SETTINGS
        ))
            .block(Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(self.theme.border.to_ratatui())))
            .style(Style::default().fg(self.theme.success().to_ratatui()))
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(actions, chunks[3]);
    }

    fn draw_help_popup(&self, f: &mut Frame) {
        let area = centered_rect(75, 90, f.area());
        f.render_widget(Clear, area);

        let help_text = vec![
            "╔═══════════════════════════════════════════════════════════════════╗",
            "║          󰢮 nvcontrol v0.7.0 - TUI Keyboard Reference              ║",
            "╚═══════════════════════════════════════════════════════════════════╝",
            "",
            "═══ ESSENTIAL ═══════════════════════════════════════════════════════",
            "  q / Ctrl+C    Exit application",
            "  h / F1 / ?    Toggle this help screen",
            "  Space / p     Pause/Resume real-time updates",
            "  r             Reset metrics history",
            "  t             Cycle through themes",
            "  s             Open settings panel",
            "",
            "═══ NAVIGATION ══════════════════════════════════════════════════════",
            "  Tab / →       Next tab              Shift+Tab / ←  Previous tab",
            "  ↑/↓           Select GPU (multi-GPU systems)",
            "  Home / End    Jump to first/last tab",
            "",
            "  ┌──────────────────────────────────────────────────────────────┐",
            "  │  1 󰍹 Overview     5 󰚥 Power       9  󰆼 Profiles   13 󰒓 Settings│",
            "  │  2 󰓅 Performance  6 󰕮 Processes  10 󰔎 Tuner                   │",
            "  │  3 󰍛 Memory       7 󰓸 Overclock  11 󰄪 Profiler                │",
            "  │  4 󱃂 Temperature  8 󰈐 Fan        12 󰕧 OSD                     │",
            "  └──────────────────────────────────────────────────────────────┘",
            "",
            "═══ FEATURES ════════════════════════════════════════════════════════",
            "  v             Toggle VRR/G-Sync (display sync)",
            "  g             Toggle Gaming Mode (performance profile)",
            "  o             Enter Overclock editing (Tab 7)",
            "  f             Enter Fan curve editing (Tab 8)",
            "  e             Export current metrics to JSON file",
            "",
            "═══ OVERCLOCK MODE (Tab 7 → press 'o') ══════════════════════════════",
            "  ←/→           Adjust GPU clock offset ±10 MHz",
            "  ↑/↓           Adjust Memory clock offset ±50 MHz",
            "  +/-           Adjust Power limit ±5%",
            "  1-4           Quick presets: 1=Stock 2=Mild 3=Perf 4=Extreme",
            "  Enter         Apply overclock settings",
            "  Escape        Cancel and exit overclock mode",
            "",
            "═══ FAN CONTROL MODE (Tab 8 → press 'f') ════════════════════════════",
            "  ←/→           Select curve point (temperature)",
            "  ↑/↓           Adjust fan speed at selected point",
            "  Enter         Apply custom fan curve",
            "  Escape        Cancel and exit fan control mode",
            "",
            "═══ THEMES ══════════════════════════════════════════════════════════",
            "  Tokyo Night (Storm/Moon) │ Dracula │ ROG Red │ Matrix │ Cyberpunk",
            "",
            "                      Press any key to close this help",
        ];

        let help = Paragraph::new(help_text.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.theme.accent().to_ratatui()))
                    .title(" 󰋖 Help & Keybindings ")
                    .title_style(
                        Style::default()
                            .fg(self.theme.primary().to_ratatui())
                            .add_modifier(Modifier::BOLD),
                    ),
            )
            .style(Style::default().fg(self.theme.text().to_ratatui()))
            .alignment(Alignment::Left);

        f.render_widget(help, area);
    }

    fn draw_settings_popup(&self, f: &mut Frame) {
        let area = centered_rect(70, 80, f.area());
        f.render_widget(Clear, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(10),   // Settings content
                Constraint::Length(3), // Controls
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
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Configuration Details"),
            )
            .style(Style::default())
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(settings_content, chunks[1]);

        // Controls
        let controls =
            Paragraph::new("Press 's' again to close settings, 'r' to reset to defaults")
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
            "󰏤 PAUSED - Press Space to resume"
        } else if let Some(msg) = self.get_status_message() {
            msg
        } else {
            "Press 'h' for help, 'q' to quit, 't' for themes, 'v' for VRR, 'g' for gaming"
        };

        let status_bar = Paragraph::new(main_status)
            .style(Style::default().fg(self.theme.warning().to_ratatui()))
            .alignment(Alignment::Left);
        f.render_widget(status_bar, chunks[0]);

        // Middle section: feature status
        let vrr_icon = if self.vrr_enabled {
            themes::icons::VRR
        } else {
            ""
        };
        let gaming_icon = if self.gaming_mode_enabled {
            themes::icons::GAMING
        } else {
            ""
        };

        let vrr_status = if self.vrr_enabled {
            format!("{} VRR", vrr_icon)
        } else {
            "VRR: OFF".to_string()
        };
        let gaming_status = if self.gaming_mode_enabled {
            format!("{} Gaming", gaming_icon)
        } else {
            "Gaming: OFF".to_string()
        };
        let feature_status = format!("{} | {}", vrr_status, gaming_status);

        let feature_color = if self.vrr_enabled || self.gaming_mode_enabled {
            self.theme.success().to_ratatui()
        } else {
            self.theme.text_dim().to_ratatui()
        };

        let feature_bar = Paragraph::new(feature_status)
            .style(Style::default().fg(feature_color))
            .alignment(Alignment::Center);
        f.render_widget(feature_bar, chunks[1]);

        // Right section: GPU selection
        let gpu_info = if self.device_count > 1 {
            format!(
                "{} GPU {}/{}",
                themes::icons::GPU,
                self.selected_gpu + 1,
                self.device_count
            )
        } else {
            format!("{} GPU 1/1", themes::icons::GPU)
        };

        let gpu_bar = Paragraph::new(gpu_info)
            .style(Style::default().fg(self.theme.primary().to_ratatui()))
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
                        if let Ok(temp) = device.temperature(
                            nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu,
                        ) {
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

    /// Handle mouse events (clicks, scrolls)
    fn handle_mouse_event(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::Down(_button) => {
                let row = mouse.row;
                let col = mouse.column;

                // Tab bar is at row 3-5 (after header)
                if row >= 3 && row < 6 {
                    // Calculate which tab was clicked
                    // Each tab title is ~12 chars wide
                    let tab_index = (col / 12) as usize;
                    let total_tabs = Tab::titles().len();

                    if tab_index < total_tabs {
                        self.current_tab = tab_index;
                        let tab_name = Tab::titles()[tab_index];
                        self.set_status_message(format!("🖱️  Switched to {} tab", tab_name));
                    }
                }

                // TODO: Add click handlers for buttons, sliders, etc.
                // For now, clicking anywhere else shows coordinates
                if row > 6 {
                    self.set_status_message(format!("🖱️  Clicked at ({}, {})", col, row));
                }
            }
            MouseEventKind::ScrollUp => {
                // Scroll up - previous tab
                if self.current_tab > 0 {
                    self.current_tab -= 1;
                    let tab_name = Tab::titles()[self.current_tab];
                    self.set_status_message(format!("🖱️  Scrolled to {} tab", tab_name));
                }
            }
            MouseEventKind::ScrollDown => {
                // Scroll down - next tab
                let total_tabs = Tab::titles().len();
                if self.current_tab < total_tabs - 1 {
                    self.current_tab += 1;
                    let tab_name = Tab::titles()[self.current_tab];
                    self.set_status_message(format!("🖱️  Scrolled to {} tab", tab_name));
                }
            }
            _ => {}
        }
    }

    /// Render a visual slider for values
    fn render_slider(&self, value: i32, min: i32, max: i32, width: usize) -> String {
        let range = max - min;
        let position = ((value - min) as f32 / range as f32 * width as f32) as usize;
        let position = position.min(width);

        let mut slider = String::new();
        for i in 0..width {
            if i == position {
                slider.push('●'); // Current position marker
            } else {
                slider.push('─'); // Slider bar
            }
        }
        slider
    }

    fn apply_oc_preset(&mut self, preset: OcPreset) {
        self.oc_preset = preset;
        match preset {
            OcPreset::Stock => {
                self.gpu_offset = 0;
                self.memory_offset = 0;
                self.power_limit_percent = 80;
                self.set_status_message("Applied Stock preset".to_string());
            }
            OcPreset::MildOc => {
                self.gpu_offset = 75;
                self.memory_offset = 500;
                self.power_limit_percent = 90;
                self.set_status_message(
                    "Applied Mild OC preset (+75 GPU, +500 MEM, 90% power)".to_string(),
                );
            }
            OcPreset::Performance => {
                self.gpu_offset = 150;
                self.memory_offset = 1000;
                self.power_limit_percent = 95;
                self.set_status_message(
                    "Applied Performance preset (+150 GPU, +1000 MEM, 95% power)".to_string(),
                );
            }
            OcPreset::Extreme => {
                self.gpu_offset = 200;
                self.memory_offset = 1500; // GDDR7 safe for RTX 5090
                self.power_limit_percent = 100;
                self.set_status_message("Applied Extreme preset - RTX 5090 Max OC!".to_string());
            }
        }
    }

    fn apply_overclock(&mut self) {
        // Use TunerState for consistent OC implementation with GUI
        if let Some(tuner) = self.tuner_states.get_mut(self.selected_gpu) {
            // Sync TUI values to tuner state
            tuner.core_clock_offset = self.gpu_offset;
            tuner.memory_clock_offset = self.memory_offset;
            tuner.power_limit = self.power_limit_percent;

            // Apply via tuner (uses nvidia-smi + NVML like GUI)
            match tuner.apply_overclock() {
                Ok(()) => {
                    self.set_status_message(format!(
                        "✅ Applied OC: GPU {:+}MHz, MEM {:+}MHz, Power {}%",
                        self.gpu_offset, self.memory_offset, self.power_limit_percent
                    ));
                }
                Err(e) => {
                    self.set_status_message(format!("❌ OC failed: {}", e));
                }
            }
        } else {
            // Fallback: create tuner state and apply
            let mut tuner = gui_tuner::TunerState::new(self.selected_gpu as u32);
            tuner.core_clock_offset = self.gpu_offset;
            tuner.memory_clock_offset = self.memory_offset;
            tuner.power_limit = self.power_limit_percent;

            match tuner.apply_overclock() {
                Ok(()) => {
                    self.set_status_message(format!(
                        "✅ Applied OC: GPU {:+}MHz, MEM {:+}MHz, Power {}%",
                        self.gpu_offset, self.memory_offset, self.power_limit_percent
                    ));
                }
                Err(e) => {
                    self.set_status_message(format!("❌ OC failed: {}", e));
                }
            }
        }
    }

    fn apply_fan_curve(&mut self) {
        // Use TunerState for consistent fan control implementation with GUI
        if let Some(tuner) = self.tuner_states.get_mut(self.selected_gpu) {
            // Sync TUI curve to tuner state
            tuner.fan_curve = self.fan_curve_points
                .iter()
                .map(|(t, s)| (*t as i32, *s))
                .collect();
            tuner.fan_mode = gui_tuner::FanControlMode::Curve;

            // Apply via tuner (uses nvidia-settings like GUI)
            match tuner.apply_fan_control() {
                Ok(()) => {
                    self.set_status_message(format!(
                        "✅ Fan curve applied ({} points)",
                        self.fan_curve_points.len()
                    ));
                }
                Err(e) => {
                    // Try saving to fan profile as fallback
                    use crate::fan::{FanCurve, FanCurvePoint, FanProfile, save_fan_profiles};

                    let curve = FanCurve {
                        name: "TUI Custom".to_string(),
                        points: self.fan_curve_points.iter().map(|(t, s)| FanCurvePoint {
                            temperature: *t as u8,
                            duty_cycle: *s as u8,
                        }).collect(),
                        hysteresis: 2,
                        min_duty_cycle: self.fan_curve_points.first().map(|(_, s)| *s as u8).unwrap_or(0),
                        max_duty_cycle: 100,
                        zero_rpm_threshold: if self.fan_curve_points.first().map(|(_, s)| *s).unwrap_or(0) == 0 {
                            Some(self.fan_curve_points.first().map(|(t, _)| *t as u8).unwrap_or(30))
                        } else {
                            None
                        },
                    };

                    let mut curves = std::collections::HashMap::new();
                    curves.insert(self.selected_gpu, curve);

                    let profile = FanProfile {
                        name: "TUI Custom".to_string(),
                        description: "Custom curve from TUI".to_string(),
                        curves,
                        enabled: true,
                        load_based_scaling: false,
                        aggressive_mode: false,
                    };

                    if save_fan_profiles(&[profile]).is_ok() {
                        self.set_status_message(format!(
                            "⚠️ Fan curve saved (direct control: {})", e
                        ));
                    } else {
                        self.set_status_message(format!("❌ Fan control failed: {}", e));
                    }
                }
            }
        } else {
            // No tuner state, try direct fan module
            use crate::fan::set_fan_curve;

            let curve_points: Vec<(u8, u8)> = self.fan_curve_points
                .iter()
                .map(|(t, s)| (*t as u8, *s as u8))
                .collect();

            let fan_id = self.selected_gpu * 10;

            match set_fan_curve(fan_id, &curve_points) {
                Ok(()) => {
                    self.set_status_message(format!(
                        "✅ Fan curve applied ({} points)",
                        curve_points.len()
                    ));
                }
                Err(e) => {
                    self.set_status_message(format!("❌ Fan control failed: {}", e));
                }
            }
        }
    }

    /// Export metrics to JSON file
    fn cycle_theme(&mut self) {
        use themes::ThemeVariant;

        self.current_theme = match self.current_theme {
            ThemeVariant::TokyoNightNight => ThemeVariant::TokyoNightStorm,
            ThemeVariant::TokyoNightStorm => ThemeVariant::TokyoNightMoon,
            ThemeVariant::TokyoNightMoon => ThemeVariant::Dracula,
            ThemeVariant::Dracula => ThemeVariant::RogRed,
            ThemeVariant::RogRed => ThemeVariant::MatrixGreen,
            ThemeVariant::MatrixGreen => ThemeVariant::Cyberpunk,
            ThemeVariant::Cyberpunk => ThemeVariant::TokyoNightNight,
        };

        self.theme = themes::ColorPalette::from_variant(self.current_theme);
        self.set_status_message(format!("Theme: {}", self.current_theme.name()));
    }

    fn export_metrics(&self) {
        use std::fs::File;
        use std::io::Write;

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("nvcontrol_metrics_{}.json", timestamp);

        let mut export_data = serde_json::Map::new();

        // Export all GPU metrics
        for (gpu_id, history) in self.metrics_history.iter().enumerate() {
            let gpu_data: Vec<_> = history
                .iter()
                .map(|m| {
                    serde_json::json!({
                        "temperature": m.temperature,
                        "gpu_utilization": m.gpu_utilization,
                        "memory_utilization": m.memory_utilization,
                        "power_draw": m.power_draw,
                        "fan_speed": m.fan_speed,
                        "gpu_clock": m.gpu_clock,
                        "memory_clock": m.memory_clock,
                    })
                })
                .collect();

            export_data.insert(
                format!("gpu_{}", gpu_id),
                serde_json::Value::Array(gpu_data),
            );
        }

        match File::create(&filename) {
            Ok(mut file) => {
                if let Ok(json) = serde_json::to_string_pretty(&export_data) {
                    if file.write_all(json.as_bytes()).is_ok() {
                        println!("✅ Metrics exported to: {}", filename);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to export metrics: {}", e);
            }
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

impl TuiApp {
    /// Draw Tuner tab (MSI Afterburner-style)
    fn draw_tuner(&self, f: &mut Frame, area: Rect) {
        let tuner_state = &self.tuner_states[self.selected_gpu];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Content
            ])
            .split(area);

        // Title
        let title = Paragraph::new(format!(
            "🎛️  GPU Tuner - GPU {} (MSI Afterburner Style)",
            self.selected_gpu
        ))
        .style(Style::default().fg(self.theme.cyan.to_ratatui()))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Content
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Controls
                Constraint::Percentage(50), // Monitoring
            ])
            .split(chunks[1]);

        // Controls
        let controls_text = vec![
            format!("┌─ Overclocking ────────────────────┐"),
            format!(
                "│ Core Clock:   {:+4} MHz  [±500]  │",
                tuner_state.core_clock_offset
            ),
            format!(
                "│ Memory Clock: {:+4} MHz  [±1500] │",
                tuner_state.memory_clock_offset
            ),
            format!(
                "│ Power Limit:  {:3}%  TDP         │",
                tuner_state.power_limit
            ),
            format!("│ Temp Limit:   {}°C             │", tuner_state.temp_limit),
            format!("└───────────────────────────────────┘"),
            format!(""),
            format!("┌─ Fan Control ─────────────────────┐"),
            format!("│ Mode: {:?}                 │", tuner_state.fan_mode),
            format!(
                "│ Speed: {}%                    │",
                tuner_state.fan_speed_manual
            ),
            format!("└───────────────────────────────────┘"),
            format!(""),
            format!("📋 Presets:"),
            format!("  [S]ilent  [G]aming  [O]verclocking"),
            format!(""),
            format!("🎮 Controls:"),
            format!("  [A]pply  [R]eset  [E]xport"),
        ];

        let controls = Paragraph::new(controls_text.join("\n"))
            .style(Style::default().fg(self.theme.fg.to_ratatui()))
            .block(Block::default().borders(Borders::ALL).title("Controls"));
        f.render_widget(controls, content_chunks[0]);

        // Monitoring (real-time stats)
        let monitoring_text = vec![
            format!("┌─ Live Monitoring ─────────────────┐"),
            format!("│ GPU Clock:    {} MHz           │", tuner_state.gpu_clock),
            format!(
                "│ Memory Clock: {} MHz           │",
                tuner_state.memory_clock
            ),
            format!(
                "│ Temperature:  {}°C             │",
                tuner_state.temperature
            ),
            format!("│ GPU Load:     {}%              │", tuner_state.gpu_load),
            format!(
                "│ Memory Load:  {}%              │",
                tuner_state.memory_load
            ),
            format!(
                "│ VRAM Used:    {} / {} MB      │",
                tuner_state.vram_used, tuner_state.vram_total
            ),
            format!(
                "│ Power Draw:   {:.1} W            │",
                tuner_state.power_draw
            ),
            format!("│ Fan Speed:    {}%              │", tuner_state.fan_speed),
            format!("└───────────────────────────────────┘"),
            format!(""),
            format!("📊 History: {} samples", tuner_state.temp_history.len()),
            format!(""),
            format!("💡 Tip: Use ← → to adjust values"),
            format!("       Use ↑ ↓ to select setting"),
        ];

        let monitoring = Paragraph::new(monitoring_text.join("\n"))
            .style(Style::default().fg(self.theme.fg.to_ratatui()))
            .block(Block::default().borders(Borders::ALL).title("Monitoring"));
        f.render_widget(monitoring, content_chunks[1]);
    }

    /// Draw Profiler tab (radeon-profile equivalent)
    fn draw_profiler(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Content
            ])
            .split(area);

        // Title
        let title_text = if self.profiler_recording {
            format!("🔴 GPU Profiler - RECORDING (GPU {})", self.selected_gpu)
        } else {
            format!("📊 GPU Profiler - Ready (GPU {})", self.selected_gpu)
        };
        let title = Paragraph::new(title_text)
            .style(Style::default().fg(if self.profiler_recording {
                Color::Red
            } else {
                self.theme.cyan.to_ratatui()
            }))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Content
        let content_text = if let Some(ref profiler) = self.profiler {
            let stats = profiler.get_statistics();
            let sample_count = profiler.current_samples();
            let recording_status = if profiler.is_recording() {
                "RECORDING"
            } else {
                "STOPPED"
            };

            let mut lines = vec![
                format!("┌─ Session Status ──────────────────────────────────────┐"),
                format!(
                    "│ Status: {}                                    │",
                    recording_status
                ),
                format!(
                    "│ Samples: {}                                         │",
                    sample_count
                ),
                format!("│ Sample Interval: 100ms                              │"),
                format!("│ Max Samples: 10,000                                  │"),
                format!("└─────────────────────────────────────────────────────┘"),
                format!(""),
            ];

            if let Some(stats) = stats {
                lines.extend(vec![
                    format!("┌─ Statistics ──────────────────────────────────────────┐"),
                    format!(
                        "│ GPU Clock:    Avg {}  Max {}  Min {} MHz    │",
                        stats.avg_gpu_clock, stats.max_gpu_clock, stats.min_gpu_clock
                    ),
                    format!(
                        "│ Memory Clock: Avg {} MHz                        │",
                        stats.avg_memory_clock
                    ),
                    format!(
                        "│ Temperature:  Avg {}°C  Max {}°C  Min {}°C    │",
                        stats.avg_temperature, stats.max_temperature, stats.min_temperature
                    ),
                    format!(
                        "│ GPU Load:     Avg {}%                            │",
                        stats.avg_gpu_load
                    ),
                    format!(
                        "│ Power Draw:   Avg {:.1}W  Max {:.1}W           │",
                        stats.avg_power_draw, stats.max_power_draw
                    ),
                    format!("└─────────────────────────────────────────────────────┘"),
                ]);
            } else {
                lines.push(format!("No statistics available yet - start recording"));
            }

            lines.extend(vec![
                format!(""),
                format!("🎮 Controls:"),
                format!("  [Space] Start/Stop Recording"),
                format!("  [E]xport Session to JSON"),
                format!("  [C]lear Data"),
                format!(""),
                format!("💡 This profiler captures comprehensive GPU telemetry"),
                format!("   similar to Radeon GPU Profiler but for NVIDIA GPUs."),
            ]);

            lines.join("\n")
        } else {
            format!(
                "❌ No profiler available\n\nNo GPU detected or profiler initialization failed."
            )
        };

        let content = Paragraph::new(content_text)
            .style(Style::default().fg(self.theme.fg.to_ratatui()))
            .block(Block::default().borders(Borders::ALL).title("Profiler"));
        f.render_widget(content, chunks[1]);
    }

    /// Auto-detect theme based on GPU vendor
    fn detect_gpu_vendor_theme(nvml: &Option<Nvml>) -> themes::ThemeVariant {
        if let Some(nvml) = nvml {
            if let Ok(device) = nvml.device_by_index(0) {
                if let Ok(name) = device.name() {
                    let name_lower = name.to_lowercase();

                    // ASUS ROG cards
                    if name_lower.contains("asus") || name_lower.contains("rog") {
                        println!("🎨 Detected ASUS GPU - using ROG Red theme");
                        return themes::ThemeVariant::RogRed;
                    }

                    // MSI cards
                    if name_lower.contains("msi") {
                        println!("🎨 Detected MSI GPU - using Dracula theme");
                        return themes::ThemeVariant::Dracula;
                    }

                    // EVGA cards
                    if name_lower.contains("evga") {
                        println!("🎨 Detected EVGA GPU - using Cyberpunk theme");
                        return themes::ThemeVariant::Cyberpunk;
                    }

                    // Gigabyte cards
                    if name_lower.contains("gigabyte") || name_lower.contains("aorus") {
                        println!("🎨 Detected Gigabyte GPU - using Matrix Green theme");
                        return themes::ThemeVariant::MatrixGreen;
                    }

                    // Founders Edition or generic NVIDIA
                    if name_lower.contains("founders") || name_lower.contains("nvidia") {
                        println!("🎨 Detected NVIDIA GPU - using Tokyo Night Moon theme");
                        return themes::ThemeVariant::TokyoNightMoon;
                    }
                }
            }
        }

        // Default theme - Tokyo Night Moon
        println!("🎨 Using default Tokyo Night Moon theme");
        themes::ThemeVariant::TokyoNightMoon
    }

    /// Draw OSD/MangoHud configuration tab
    fn draw_osd(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Left panel
                Constraint::Percentage(50), // Right panel
            ])
            .split(area);

        // Left panel - Status and Current Config
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),  // Status
                Constraint::Length(10), // Current metrics
                Constraint::Min(0),     // Presets
            ])
            .split(chunks[0]);

        // Status section
        let mangohud_installed = crate::osd::OsdManager::check_mangohud_installed();
        let status_icon = if mangohud_installed { "✅" } else { "❌" };
        let status_text = format!(
            " MangoHud: {} {}\n\
             Config: ~/.config/MangoHud/MangoHud.conf\n\n\
             Launch: mangohud %command%\n\
             Steam:  MANGOHUD=1 %command%",
            status_icon,
            if mangohud_installed {
                "Installed"
            } else {
                "Not Found"
            }
        );
        let status = Paragraph::new(status_text)
            .style(Style::default().fg(self.theme.fg.to_ratatui()))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📊 MangoHud Status")
                    .border_style(Style::default().fg(self.theme.cyan.to_ratatui())),
            );
        f.render_widget(status, left_chunks[0]);

        // Current metrics (read from config if exists)
        let current_metrics = " Default metrics:\n\
                               • fps (FPS Counter)\n\
                               • frametime (Frame Graph)\n\
                               • gpu_temp (Temperature)\n\
                               • gpu_load (Utilization)\n\
                               • vram (VRAM Usage)";
        let current = Paragraph::new(current_metrics)
            .style(Style::default().fg(self.theme.fg.to_ratatui()))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Active Metrics")
                    .border_style(Style::default().fg(self.theme.green.to_ratatui())),
            );
        f.render_widget(current, left_chunks[1]);

        // Presets
        let presets_text = " Quick Presets (via CLI):\n\n\
                            [Minimal]   fps only\n\
                            [Standard]  fps, frametime, gpu_temp,\n\
                                        gpu_load, vram\n\
                            [Full]      All metrics\n\
                            [Benchmark] fps, frametime, temps,\n\
                                        clocks, power";
        let presets = Paragraph::new(presets_text)
            .style(Style::default().fg(self.theme.fg.to_ratatui()))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("⚡ Presets")
                    .border_style(Style::default().fg(self.theme.yellow.to_ratatui())),
            );
        f.render_widget(presets, left_chunks[2]);

        // Right panel - Available Metrics and Commands
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12), // Available metrics
                Constraint::Min(0),     // Commands
            ])
            .split(chunks[1]);

        // Available metrics list
        let metrics_items: Vec<ListItem> = vec![
            ListItem::new(" fps          FPS Counter"),
            ListItem::new(" frametime    Frame Time Graph"),
            ListItem::new(" gpu_temp     GPU Temperature"),
            ListItem::new(" gpu_load     GPU Utilization"),
            ListItem::new(" gpu_core_clock GPU Clock Speed"),
            ListItem::new(" gpu_power    Power Draw"),
            ListItem::new(" vram         VRAM Usage"),
            ListItem::new(" fan          Fan Speed"),
            ListItem::new(" cpu_temp     CPU Temperature"),
            ListItem::new(" cpu_load     CPU Utilization"),
        ];
        let metrics_list = List::new(metrics_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("📈 Available Metrics")
                    .border_style(Style::default().fg(self.theme.magenta.to_ratatui())),
            )
            .style(Style::default().fg(self.theme.fg.to_ratatui()));
        f.render_widget(metrics_list, right_chunks[0]);

        // Commands
        let commands_text = " CLI Commands:\n\n\
                             nvctl osd enable     Enable OSD\n\
                             nvctl osd disable    Disable OSD\n\
                             nvctl osd status     Show config\n\
                             nvctl osd add <m>    Add metric\n\
                             nvctl osd remove <m> Remove metric\n\
                             nvctl osd metrics    List metrics\n\n\
                             💡 For GUI config: nvcontrol";
        let commands = Paragraph::new(commands_text)
            .style(Style::default().fg(self.theme.fg.to_ratatui()))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🔧 Commands")
                    .border_style(Style::default().fg(self.theme.border.to_ratatui())),
            );
        f.render_widget(commands, right_chunks[1]);
    }

    /// Draw Settings tab
    fn draw_settings(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Content
            ])
            .split(area);

        // Title
        let title = Paragraph::new("⚙️  Settings")
            .style(Style::default().fg(self.theme.cyan.to_ratatui()))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Settings content
        let settings_text = vec![
            format!("┌─ Display Settings ────────────────────────────────────┐"),
            format!(
                "│ Theme: {:?}                               │",
                self.current_theme
            ),
            format!(
                "│ Update Interval: {}s                               │",
                self.update_interval.as_secs()
            ),
            format!(
                "│ Selected GPU: {}                                    │",
                self.selected_gpu
            ),
            format!("└─────────────────────────────────────────────────────┘"),
            format!(""),
            format!("┌─ Features ────────────────────────────────────────────┐"),
            format!(
                "│ VRR Enabled: {}                                    │",
                if self.vrr_enabled { "Yes" } else { "No" }
            ),
            format!(
                "│ Gaming Mode: {}                                    │",
                if self.gaming_mode_enabled {
                    "Yes"
                } else {
                    "No"
                }
            ),
            format!(
                "│ Fan Control: {}                                    │",
                if self.fan_control_mode { "Yes" } else { "No" }
            ),
            format!(
                "│ OC Control:  {}                                    │",
                if self.oc_control_mode { "Yes" } else { "No" }
            ),
            format!("└─────────────────────────────────────────────────────┘"),
            format!(""),
            format!("┌─ System ──────────────────────────────────────────────┐"),
            format!(
                "│ GPUs Detected: {}                                   │",
                self.device_count
            ),
            format!(
                "│ NVML Available: {}                                 │",
                if self.nvml.is_some() { "Yes" } else { "No" }
            ),
            format!(
                "│ Uptime: {:.0}s                                      │",
                self.start_time.elapsed().as_secs()
            ),
            format!("└─────────────────────────────────────────────────────┘"),
            format!(""),
            format!("🎨 Themes Available:"),
            format!("  [T] Cycle themes"),
            format!("  Available: TokyoNight, Dracula, RogRed, Cyberpunk, etc."),
            format!(""),
            format!("⚙️  Configuration file: ~/.config/nvcontrol/config.toml"),
        ];

        let settings = Paragraph::new(settings_text.join("\n"))
            .style(Style::default().fg(self.theme.fg.to_ratatui()))
            .block(Block::default().borders(Borders::ALL).title("Settings"));
        f.render_widget(settings, chunks[1]);
    }
}

// Error conversion
impl From<io::Error> for crate::NvControlError {
    fn from(error: io::Error) -> Self {
        crate::NvControlError::DisplayDetectionFailed(format!("IO error: {}", error))
    }
}
