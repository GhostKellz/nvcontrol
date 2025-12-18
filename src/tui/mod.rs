//! TUI module for nvcontrol

mod event;
mod terminal;

pub use event::{Event, EventHandler};
pub use terminal::Tui;

use crate::config::TuiSessionState;
use crate::nvml_backend::GuiBackendContext;
use crate::{NvResult, gui_tuner, nvidia_profiler, themes};
use crossterm::event::{KeyCode, KeyModifiers, MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{
        Block, Borders, Cell, Clear, Gauge, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Sparkline, Table, TableState, Tabs,
    },
};
use std::collections::VecDeque;
use std::fs;
use std::time::{Duration, Instant};

const MAX_HISTORY: usize = 120; // 2 minutes at 1Hz

/// GPU metrics snapshot
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

/// TUI view mode
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ViewMode {
    /// Main menu with options
    Menu,
    /// nvtop-style monitoring (htop for GPU)
    Nvtop,
    /// Full dashboard with all tabs
    Dashboard,
}

/// Tab in dashboard view
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tab {
    Overview,
    Performance,
    Memory,
    Temperature,
    Power,
    Processes,
    Overclocking,
    FanControl,
    Profiles,
    Tuner,
    Profiler,
    Osd,
    Drivers,
    Settings,
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
            "Drivers",
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
            12 => Tab::Drivers,
            _ => Tab::Settings,
        }
    }

    fn count() -> usize {
        14
    }
}

/// OC preset levels
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OcPreset {
    Stock,
    MildOc,
    Performance,
    Extreme,
}

/// Input mode for the TUI
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InputMode {
    /// Normal navigation mode
    Normal,
    /// Filter input mode (typing filter text)
    Filter,
    /// Sort selection mode
    Sort,
}

/// Sort column for process table
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SortColumn {
    /// No sorting
    None,
    /// Sort by PID ascending
    PidAsc,
    /// Sort by PID descending
    PidDesc,
    /// Sort by name ascending
    NameAsc,
    /// Sort by name descending
    NameDesc,
    /// Sort by VRAM ascending
    VramAsc,
    /// Sort by VRAM descending
    VramDesc,
    /// Sort by type ascending
    TypeAsc,
    /// Sort by type descending
    TypeDesc,
}

impl SortColumn {
    fn cycle_for_column(col: usize, current: Self) -> Self {
        match col {
            0 => match current {
                SortColumn::PidAsc => SortColumn::PidDesc,
                SortColumn::PidDesc => SortColumn::None,
                _ => SortColumn::PidAsc,
            },
            1 => match current {
                SortColumn::NameAsc => SortColumn::NameDesc,
                SortColumn::NameDesc => SortColumn::None,
                _ => SortColumn::NameAsc,
            },
            2 => match current {
                SortColumn::TypeAsc => SortColumn::TypeDesc,
                SortColumn::TypeDesc => SortColumn::None,
                _ => SortColumn::TypeAsc,
            },
            3 => match current {
                SortColumn::VramAsc => SortColumn::VramDesc,
                SortColumn::VramDesc => SortColumn::None,
                _ => SortColumn::VramAsc,
            },
            _ => SortColumn::None,
        }
    }

    fn column_index(&self) -> Option<usize> {
        match self {
            SortColumn::None => None,
            SortColumn::PidAsc | SortColumn::PidDesc => Some(0),
            SortColumn::NameAsc | SortColumn::NameDesc => Some(1),
            SortColumn::TypeAsc | SortColumn::TypeDesc => Some(2),
            SortColumn::VramAsc | SortColumn::VramDesc => Some(3),
        }
    }

    fn is_descending(&self) -> bool {
        matches!(
            self,
            SortColumn::PidDesc
                | SortColumn::NameDesc
                | SortColumn::TypeDesc
                | SortColumn::VramDesc
        )
    }
}

/// GPU process entry for display
#[derive(Clone, Debug)]
pub struct ProcessEntry {
    pub pid: u32,
    pub name: String,
    pub process_type: ProcessType,
    pub vram_mb: u64,
}

/// Type of GPU process
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessType {
    Graphics,
    Compute,
}

impl ProcessType {
    fn as_str(&self) -> &'static str {
        match self {
            ProcessType::Graphics => "Graphics",
            ProcessType::Compute => "Compute",
        }
    }
}

/// Main TUI application state
pub struct TuiApp {
    /// Running flag
    pub running: bool,
    /// Current view mode
    pub view_mode: ViewMode,
    /// Backend context for GPU operations (lazy loaded)
    backend_ctx: Option<GuiBackendContext>,
    /// Backend initialized flag
    backend_initialized: bool,
    /// Device count
    device_count: u32,
    /// Metrics history per GPU
    metrics_history: Vec<VecDeque<GpuMetrics>>,
    /// Current tab (dashboard mode)
    current_tab: usize,
    /// Selected GPU
    selected_gpu: usize,
    /// Show help overlay
    show_help: bool,
    /// Show settings overlay
    show_settings: bool,
    /// Paused state
    paused: bool,
    /// Update interval
    update_interval: Duration,
    /// Start time
    start_time: Instant,
    /// VRR enabled (placeholder for future feature)
    _vrr_enabled: bool,
    /// Gaming mode (placeholder for future feature)
    _gaming_mode_enabled: bool,
    /// Status message
    status_message: Option<String>,
    status_message_time: Option<Instant>,
    /// Fan control mode active
    fan_control_mode: bool,
    /// OC control mode active
    oc_control_mode: bool,
    /// Fan speed target
    fan_speed_target: u32,
    /// Current theme
    current_theme: themes::ThemeVariant,
    theme: themes::ColorPalette,
    /// OC settings
    gpu_offset: i32,
    memory_offset: i32,
    power_limit_percent: u32,
    oc_preset: OcPreset,
    /// Fan curve
    fan_curve_points: Vec<(u32, u32)>,
    selected_curve_point: usize,
    /// Tuner states
    tuner_states: Vec<gui_tuner::TunerState>,
    /// Profiler
    profiler: Option<nvidia_profiler::NvidiaProfiler>,
    profiler_recording: bool,
    /// OSD
    osd_enabled: bool,
    osd_selected_metric: usize,
    /// Driver state
    driver_validation: Option<crate::state::DriverValidationState>,
    driver_capabilities: Option<crate::drivers::DriverCapabilities>,
    /// Menu selection (for menu mode)
    menu_selection: usize,
    // === Process table state (v0.8.0) ===
    /// Current input mode
    input_mode: InputMode,
    /// Process list (cached)
    processes: Vec<ProcessEntry>,
    /// Process list last update time
    processes_last_update: Instant,
    /// Process table state (selection)
    process_table_state: TableState,
    /// Scrollbar state for process table
    process_scrollbar_state: ScrollbarState,
    /// Filter string for processes
    filter_text: String,
    /// Sort column for processes
    sort_column: SortColumn,
    /// Selected column in sort mode
    sort_selected_column: usize,
    /// Show sparkline graphs (toggle with 'g')
    show_graphs: bool,
}

impl TuiApp {
    /// Create new TUI app with default view mode (instant startup)
    pub fn new() -> Self {
        Self::with_view(ViewMode::Nvtop)
    }

    /// Create TUI app with specific view mode (instant startup)
    pub fn with_view(view_mode: ViewMode) -> Self {
        // Load theme immediately (fast - just file read)
        let config = crate::config::Config::load();
        let current_theme = themes::ThemeVariant::from_config_key(&config.theme)
            .unwrap_or(themes::ThemeVariant::TokyoNightStorm);
        let theme = themes::ColorPalette::from_variant(current_theme);

        // Load saved session (fast - just file read)
        let saved_state = TuiSessionState::load();

        let fan_curve_points = if !saved_state.fan_curve_points.is_empty() {
            saved_state
                .fan_curve_points
                .iter()
                .map(|(t, f)| (*t as u32, *f as u32))
                .collect()
        } else {
            vec![(30, 20), (50, 40), (70, 60), (80, 80), (90, 100)]
        };

        let oc_preset = match saved_state.oc_preset.as_str() {
            "MildOc" => OcPreset::MildOc,
            "Performance" => OcPreset::Performance,
            "Extreme" => OcPreset::Extreme,
            _ => OcPreset::Stock,
        };

        // Return immediately - backend loaded on first tick
        Self {
            running: true,
            view_mode,
            backend_ctx: None,
            backend_initialized: false,
            device_count: 0,
            metrics_history: Vec::new(),
            current_tab: saved_state.current_tab.min(Tab::count() - 1),
            selected_gpu: saved_state.selected_gpu,
            show_help: false,
            show_settings: false,
            paused: false,
            update_interval: Duration::from_secs(1),
            start_time: Instant::now(),
            _vrr_enabled: false,
            _gaming_mode_enabled: false,
            status_message: None,
            status_message_time: None,
            fan_control_mode: false,
            oc_control_mode: false,
            fan_speed_target: 50,
            current_theme,
            theme,
            gpu_offset: saved_state.gpu_offset,
            memory_offset: saved_state.memory_offset,
            power_limit_percent: u32::from(saved_state.power_limit_percent),
            oc_preset,
            fan_curve_points,
            selected_curve_point: 0,
            tuner_states: Vec::new(),
            profiler: None,
            profiler_recording: false,
            osd_enabled: false,
            osd_selected_metric: 0,
            driver_validation: None,
            driver_capabilities: None,
            menu_selection: 0,
            // Process table state (v0.8.0)
            input_mode: InputMode::Normal,
            processes: Vec::new(),
            processes_last_update: Instant::now()
                .checked_sub(Duration::from_secs(10))
                .unwrap_or_else(Instant::now),
            process_table_state: TableState::default(),
            process_scrollbar_state: ScrollbarState::default(),
            filter_text: String::new(),
            sort_column: SortColumn::VramDesc,
            sort_selected_column: 0,
            show_graphs: true,
        }
    }

    /// Initialize backend (called on first tick)
    fn ensure_initialized(&mut self) {
        if self.backend_initialized {
            return;
        }
        self.backend_initialized = true;

        // Initialize NVML backend (this is the slow part)
        let backend_ctx = GuiBackendContext::new();
        self.device_count = backend_ctx.device_count;

        // Setup metrics history for each GPU
        self.metrics_history = (0..self.device_count)
            .map(|_| VecDeque::with_capacity(MAX_HISTORY))
            .collect();

        // Validate selected GPU
        if self.selected_gpu >= self.device_count as usize {
            self.selected_gpu = 0;
        }

        // Initialize tuner states
        self.tuner_states = (0..self.device_count)
            .map(gui_tuner::TunerState::new)
            .collect();

        // Initialize profiler
        if self.device_count > 0 {
            self.profiler = Some(nvidia_profiler::NvidiaProfiler::new(
                self.selected_gpu as u32,
                100,
                10000,
            ));
        }

        // Load driver info (fast after NVML init)
        self.driver_validation = crate::state::DriverValidationState::load();
        self.driver_capabilities = crate::drivers::get_driver_capabilities().ok();
        self.osd_enabled = crate::osd::OsdManager::check_mangohud_installed();

        self.backend_ctx = Some(backend_ctx);
    }

    /// Get process name from PID by reading /proc/{pid}/comm
    fn get_process_name(pid: u32) -> String {
        fs::read_to_string(format!("/proc/{}/comm", pid))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| format!("pid:{}", pid))
    }

    /// Refresh process list from backend (rate-limited)
    fn refresh_processes(&mut self) {
        // Rate limit to 1 second
        if self.processes_last_update.elapsed() < Duration::from_secs(1) {
            return;
        }
        self.processes_last_update = Instant::now();

        let mut entries = Vec::new();

        if let Some(ref ctx) = self.backend_ctx {
            // Get graphics processes
            if let Ok(procs) = ctx
                .nvml
                .get_running_graphics_processes(self.selected_gpu as u32)
            {
                for p in procs {
                    entries.push(ProcessEntry {
                        pid: p.pid,
                        name: Self::get_process_name(p.pid),
                        process_type: ProcessType::Graphics,
                        vram_mb: p.used_gpu_memory_bytes.unwrap_or(0) / (1024 * 1024),
                    });
                }
            }

            // Get compute processes
            if let Ok(procs) = ctx
                .nvml
                .get_running_compute_processes(self.selected_gpu as u32)
            {
                for p in procs {
                    // Avoid duplicates (some processes may be both)
                    if !entries.iter().any(|e| e.pid == p.pid) {
                        entries.push(ProcessEntry {
                            pid: p.pid,
                            name: Self::get_process_name(p.pid),
                            process_type: ProcessType::Compute,
                            vram_mb: p.used_gpu_memory_bytes.unwrap_or(0) / (1024 * 1024),
                        });
                    }
                }
            }
        }

        // Apply filter
        if !self.filter_text.is_empty() {
            let filter_lower = self.filter_text.to_lowercase();
            entries.retain(|e| {
                e.name.to_lowercase().contains(&filter_lower)
                    || e.pid.to_string().contains(&filter_lower)
                    || e.process_type
                        .as_str()
                        .to_lowercase()
                        .contains(&filter_lower)
            });
        }

        // Apply sort
        match self.sort_column {
            SortColumn::None => {}
            SortColumn::PidAsc => entries.sort_by_key(|e| e.pid),
            SortColumn::PidDesc => entries.sort_by_key(|e| std::cmp::Reverse(e.pid)),
            SortColumn::NameAsc => entries.sort_by(|a, b| a.name.cmp(&b.name)),
            SortColumn::NameDesc => entries.sort_by(|a, b| b.name.cmp(&a.name)),
            SortColumn::TypeAsc => {
                entries.sort_by(|a, b| a.process_type.as_str().cmp(b.process_type.as_str()))
            }
            SortColumn::TypeDesc => {
                entries.sort_by(|a, b| b.process_type.as_str().cmp(a.process_type.as_str()))
            }
            SortColumn::VramAsc => entries.sort_by_key(|e| e.vram_mb),
            SortColumn::VramDesc => entries.sort_by_key(|e| std::cmp::Reverse(e.vram_mb)),
        }

        self.processes = entries;

        // Update scrollbar state
        self.process_scrollbar_state = self
            .process_scrollbar_state
            .content_length(self.processes.len());
    }

    /// Select next process in table
    fn select_next_process(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        let i = match self.process_table_state.selected() {
            Some(i) => (i + 1).min(self.processes.len() - 1),
            None => 0,
        };
        self.process_table_state.select(Some(i));
        self.process_scrollbar_state = self.process_scrollbar_state.position(i);
    }

    /// Select previous process in table
    fn select_prev_process(&mut self) {
        if self.processes.is_empty() {
            return;
        }
        let i = match self.process_table_state.selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.process_table_state.select(Some(i));
        self.process_scrollbar_state = self.process_scrollbar_state.position(i);
    }

    /// Run the TUI main loop
    pub fn run(&mut self) -> NvResult<()> {
        // Initialize terminal FIRST (enters alternate screen immediately)
        let mut tui = Tui::init()?;
        let events = EventHandler::new(100); // 100ms tick rate

        // Show loading indicator
        tui.terminal().draw(|f| {
            let area = f.area();
            let accent = self.theme.teal.to_ratatui();
            let block = Block::default()
                .title(" nvctl ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent));
            let loading = Paragraph::new("Loading...")
                .alignment(Alignment::Center)
                .block(block);
            f.render_widget(loading, area);
        })?;

        // Main loop
        while self.running {
            // Handle events
            if let Some(event) = events.try_next() {
                match event {
                    Event::Key(key) => self.handle_key(key.code, key.modifiers),
                    Event::Mouse(mouse) => self.handle_mouse(mouse),
                    Event::Tick => {
                        if !self.paused {
                            self.update_metrics();
                        }
                    }
                    Event::Resize(_, _) => {}
                }
            }

            // Draw UI
            tui.terminal().draw(|f| self.draw(f))?;

            // Small sleep to prevent busy loop
            std::thread::sleep(Duration::from_millis(16)); // ~60fps
        }

        // Save state and cleanup
        self.save_session_state();
        tui.exit()?;

        Ok(())
    }

    /// Handle keyboard input
    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        // Handle input modes first
        match self.input_mode {
            InputMode::Filter => {
                self.handle_filter_input(code);
                return;
            }
            InputMode::Sort => {
                self.handle_sort_input(code);
                return;
            }
            InputMode::Normal => {}
        }

        // Global keys (only in Normal mode)
        match code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.running = false;
            }
            KeyCode::Char('?') | KeyCode::F(1) => self.show_help = !self.show_help,
            _ => {}
        }

        if self.show_help {
            return;
        }

        // View-specific handling
        match self.view_mode {
            ViewMode::Menu => self.handle_menu_key(code),
            ViewMode::Nvtop => self.handle_nvtop_key(code, modifiers),
            ViewMode::Dashboard => self.handle_dashboard_key(code, modifiers),
        }
    }

    /// Handle filter input mode
    fn handle_filter_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc | KeyCode::Enter => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                self.filter_text.pop();
                // Force process refresh
                self.processes_last_update = Instant::now()
                    .checked_sub(Duration::from_secs(10))
                    .unwrap_or_else(Instant::now);
            }
            KeyCode::Char(c) => {
                self.filter_text.push(c);
                // Force process refresh
                self.processes_last_update = Instant::now()
                    .checked_sub(Duration::from_secs(10))
                    .unwrap_or_else(Instant::now);
            }
            _ => {}
        }
    }

    /// Handle sort selection mode
    fn handle_sort_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Enter => {
                // Cycle sort for current column
                self.sort_column =
                    SortColumn::cycle_for_column(self.sort_selected_column, self.sort_column);
                // Force process refresh
                self.processes_last_update = Instant::now()
                    .checked_sub(Duration::from_secs(10))
                    .unwrap_or_else(Instant::now);
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.sort_selected_column > 0 {
                    self.sort_selected_column -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.sort_selected_column < 3 {
                    self.sort_selected_column += 1;
                }
            }
            KeyCode::Up | KeyCode::Down => {
                // Cycle sort direction for current column
                self.sort_column =
                    SortColumn::cycle_for_column(self.sort_selected_column, self.sort_column);
                // Force process refresh
                self.processes_last_update = Instant::now()
                    .checked_sub(Duration::from_secs(10))
                    .unwrap_or_else(Instant::now);
            }
            _ => {}
        }
    }

    fn handle_menu_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_selection > 0 {
                    self.menu_selection -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_selection < 3 {
                    self.menu_selection += 1;
                }
            }
            KeyCode::Enter => {
                match self.menu_selection {
                    0 => self.view_mode = ViewMode::Nvtop,
                    1 => self.view_mode = ViewMode::Dashboard,
                    2 => { /* Settings */ }
                    3 => self.running = false,
                    _ => {}
                }
            }
            KeyCode::Char('1') => self.view_mode = ViewMode::Nvtop,
            KeyCode::Char('2') => self.view_mode = ViewMode::Dashboard,
            _ => {}
        }
    }

    fn handle_nvtop_key(&mut self, code: KeyCode, _modifiers: KeyModifiers) {
        match code {
            KeyCode::Tab => self.next_gpu(),
            KeyCode::BackTab => self.prev_gpu(),
            KeyCode::Left => self.prev_gpu(),
            KeyCode::Right => self.next_gpu(),
            KeyCode::Char(' ') | KeyCode::Char('p') => self.paused = !self.paused,
            KeyCode::Char('m') => self.view_mode = ViewMode::Menu,
            KeyCode::Char('d') => self.view_mode = ViewMode::Dashboard,
            KeyCode::Char('t') => self.cycle_theme(),
            // Process navigation
            KeyCode::Up | KeyCode::Char('k') => self.select_prev_process(),
            KeyCode::Down | KeyCode::Char('j') => self.select_next_process(),
            // Filter mode
            KeyCode::Char('f') | KeyCode::Char('/') => {
                self.input_mode = InputMode::Filter;
            }
            // Sort mode
            KeyCode::Char('s') => {
                self.input_mode = InputMode::Sort;
                // Set sort column to current sort if any
                self.sort_selected_column = self.sort_column.column_index().unwrap_or(0);
            }
            // Clear filter
            KeyCode::Esc => {
                if !self.filter_text.is_empty() {
                    self.filter_text.clear();
                    self.processes_last_update = Instant::now()
                        .checked_sub(Duration::from_secs(10))
                        .unwrap_or_else(Instant::now);
                }
            }
            // Toggle sparkline graphs
            KeyCode::Char('g') => self.show_graphs = !self.show_graphs,
            _ => {}
        }
    }

    fn handle_dashboard_key(&mut self, code: KeyCode, _modifiers: KeyModifiers) {
        // Check if we're on Processes tab (index 5)
        let on_processes_tab = self.current_tab == 5;

        match code {
            KeyCode::Tab => self.next_tab(),
            KeyCode::BackTab => self.prev_tab(),
            KeyCode::Left if !on_processes_tab => self.prev_gpu(),
            KeyCode::Right if !on_processes_tab => self.next_gpu(),
            KeyCode::Char(' ') | KeyCode::Char('p') => self.paused = !self.paused,
            KeyCode::Char('m') => self.view_mode = ViewMode::Menu,
            KeyCode::Char('n') => self.view_mode = ViewMode::Nvtop,
            KeyCode::Char('t') => self.cycle_theme(),
            // Number keys to switch tabs (except in filter/sort mode or on processes tab with filter)
            KeyCode::Char('1') if !on_processes_tab => self.current_tab = 0,
            KeyCode::Char('2') if !on_processes_tab => self.current_tab = 1,
            KeyCode::Char('3') if !on_processes_tab => self.current_tab = 2,
            KeyCode::Char('4') if !on_processes_tab => self.current_tab = 3,
            KeyCode::Char('5') if !on_processes_tab => self.current_tab = 4,
            KeyCode::Char('6') if !on_processes_tab => self.current_tab = 5,
            KeyCode::Char('7') if !on_processes_tab => self.current_tab = 6,
            KeyCode::Char('8') if !on_processes_tab => self.current_tab = 7,
            KeyCode::Char('9') if !on_processes_tab => self.current_tab = 8,
            // Processes tab navigation
            KeyCode::Up | KeyCode::Char('k') if on_processes_tab => self.select_prev_process(),
            KeyCode::Down | KeyCode::Char('j') if on_processes_tab => self.select_next_process(),
            KeyCode::Char('f') | KeyCode::Char('/') if on_processes_tab => {
                self.input_mode = InputMode::Filter;
            }
            KeyCode::Char('s') if on_processes_tab => {
                self.input_mode = InputMode::Sort;
                self.sort_selected_column = self.sort_column.column_index().unwrap_or(0);
            }
            KeyCode::Esc if on_processes_tab && !self.filter_text.is_empty() => {
                self.filter_text.clear();
                self.processes_last_update = Instant::now()
                    .checked_sub(Duration::from_secs(10))
                    .unwrap_or_else(Instant::now);
            }
            // Settings toggle (not on processes tab to avoid conflict with sort)
            KeyCode::Char('s') if !on_processes_tab => self.show_settings = !self.show_settings,
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                // Scroll down in process list
                self.select_next_process();
            }
            MouseEventKind::ScrollUp => {
                // Scroll up in process list
                self.select_prev_process();
            }
            MouseEventKind::Down(_) => {
                // Handle mouse clicks on tabs, etc.
                // Future: click on process row to select
            }
            _ => {}
        }
    }

    fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % Tab::count();
    }

    fn prev_tab(&mut self) {
        self.current_tab = if self.current_tab == 0 {
            Tab::count() - 1
        } else {
            self.current_tab - 1
        };
    }

    fn next_gpu(&mut self) {
        if self.device_count > 1 {
            self.selected_gpu = (self.selected_gpu + 1) % self.device_count as usize;
        }
    }

    fn prev_gpu(&mut self) {
        if self.device_count > 1 {
            self.selected_gpu = if self.selected_gpu == 0 {
                self.device_count as usize - 1
            } else {
                self.selected_gpu - 1
            };
        }
    }

    fn cycle_theme(&mut self) {
        self.current_theme = crate::gui::theme::next_theme(self.current_theme);
        self.theme = themes::ColorPalette::from_variant(self.current_theme);
        self.set_status_message(format!("Theme: {}", self.current_theme.name()));
    }

    fn set_status_message(&mut self, msg: String) {
        self.status_message = Some(msg);
        self.status_message_time = Some(Instant::now());
    }

    fn update_metrics(&mut self) {
        // Lazy initialization on first tick
        self.ensure_initialized();

        let backend_ctx = match &self.backend_ctx {
            Some(ctx) => ctx,
            None => return,
        };

        if !backend_ctx.is_nvml_available() {
            return;
        }

        for gpu_id in 0..self.device_count {
            if let Ok(metrics) = backend_ctx.get_metrics(gpu_id) {
                let gpu_metrics = GpuMetrics {
                    timestamp: Instant::now(),
                    temperature: metrics.temperature as f64,
                    gpu_utilization: metrics.gpu_utilization as f64,
                    memory_utilization: metrics.memory_utilization as f64,
                    power_draw: metrics.power_draw_mw as f64 / 1000.0,
                    fan_speed: metrics.fan_speed as f64,
                    gpu_clock: metrics.gpu_clock_mhz as f64,
                    memory_clock: metrics.memory_clock_mhz as f64,
                };

                if let Some(history) = self.metrics_history.get_mut(gpu_id as usize) {
                    history.push_back(gpu_metrics);
                    if history.len() > MAX_HISTORY {
                        history.pop_front();
                    }
                }
            }
        }

        // Refresh process list (rate-limited internally)
        self.refresh_processes();
    }

    fn save_session_state(&self) {
        let state = TuiSessionState {
            version: 1,
            current_tab: self.current_tab,
            selected_gpu: self.selected_gpu,
            gpu_offset: self.gpu_offset,
            memory_offset: self.memory_offset,
            power_limit_percent: self.power_limit_percent as u8,
            oc_preset: format!("{:?}", self.oc_preset),
            fan_curve_points: self
                .fan_curve_points
                .iter()
                .map(|(t, f)| (*t as u8, *f as u8))
                .collect(),
        };
        state.save();
    }

    /// Draw the UI
    fn draw(&self, f: &mut Frame) {
        match self.view_mode {
            ViewMode::Menu => self.draw_menu(f),
            ViewMode::Nvtop => self.draw_nvtop(f),
            ViewMode::Dashboard => self.draw_dashboard(f),
        }

        // Draw help overlay if active
        if self.show_help {
            self.draw_help(f);
        }
    }

    fn draw_menu(&self, f: &mut Frame) {
        let area = f.area();

        // Use theme colors
        let accent = self.theme.teal.to_ratatui();
        let fg = self.theme.fg.to_ratatui();
        let bg = self.theme.bg.to_ratatui();
        let bg_dark = self.theme.bg_dark.to_ratatui();

        let block = Block::default()
            .title(" nvctl ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent))
            .style(Style::default().bg(bg));

        f.render_widget(block, area);

        let inner = area.inner(ratatui::layout::Margin {
            vertical: 2,
            horizontal: 4,
        });

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Min(10),
                Constraint::Length(2),
            ])
            .split(inner);

        // Title
        let title = Paragraph::new("Select an option:")
            .style(Style::default().fg(fg).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        // Menu items
        let menu_items = [
            ("1", "nvtop", "realtime monitoring"),
            ("2", "nvctl dashboard", "full control panel"),
            ("3", "settings", "configuration"),
            ("q", "quit", "exit"),
        ];

        let menu_area = chunks[2];
        let item_height = 2;

        for (i, (key, label, desc)) in menu_items.iter().enumerate() {
            let y = menu_area.y + (i as u16 * item_height);
            if y >= menu_area.y + menu_area.height {
                break;
            }

            let style = if i == self.menu_selection {
                Style::default()
                    .fg(bg_dark)
                    .bg(accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(fg)
            };

            let item = Paragraph::new(format!(" [{}] {} - {}", key, label, desc)).style(style);

            let item_area = Rect::new(menu_area.x, y, menu_area.width, 1);
            f.render_widget(item, item_area);
        }

        // Footer
        let footer = Paragraph::new("↑↓/jk navigate · enter select · q quit")
            .style(Style::default().fg(self.theme.comment.to_ratatui()))
            .alignment(Alignment::Center);
        f.render_widget(footer, chunks[3]);
    }

    fn draw_nvtop(&self, f: &mut Frame) {
        let area = f.area();

        // Layout: header, gauges, processes, footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(8), // Gauges
                Constraint::Min(8),    // Process table
                Constraint::Length(2), // Footer (2 lines for keybinds + filter status)
            ])
            .split(area);

        // Header with GPU info
        self.draw_nvtop_header(f, chunks[0]);

        // Gauges section
        self.draw_nvtop_gauges(f, chunks[1]);

        // Process table section
        self.draw_nvtop_processes(f, chunks[2]);

        // Footer with keybinds and status
        self.draw_nvtop_footer(f, chunks[3]);
    }

    fn draw_nvtop_footer(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();
        let yellow = self.theme.yellow.to_ratatui();
        let comment = self.theme.comment.to_ratatui();
        let bg_dark = self.theme.bg_dark.to_ratatui();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(area);

        // Line 1: Keybinds
        let uptime = self.start_time.elapsed().as_secs();
        let uptime_str = format!("{}:{:02}", uptime / 60, uptime % 60);
        let pause_str = if self.paused { " [PAUSED]" } else { "" };

        let keybinds = format!(
            " nvcontrol v{} · q:quit · ↑↓:select · f:filter · s:sort · g:graph · ?:help · up:{}{}",
            env!("CARGO_PKG_VERSION"),
            uptime_str,
            pause_str
        );
        let keybind_line = Paragraph::new(keybinds).style(Style::default().fg(comment).bg(bg_dark));
        f.render_widget(keybind_line, chunks[0]);

        // Line 2: Filter/sort status or process count
        let status_text = match self.input_mode {
            InputMode::Filter => {
                format!(" FILTER: /{}▏", self.filter_text)
            }
            InputMode::Sort => {
                let cols = ["PID", "Name", "Type", "VRAM"];
                let sort_display: String = cols
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == self.sort_selected_column {
                            format!("[{}]", c)
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(" SORT: {} (←/→ select, Enter confirm)", sort_display)
            }
            InputMode::Normal => {
                let filter_info = if self.filter_text.is_empty() {
                    String::new()
                } else {
                    format!(" | filter: \"{}\"", self.filter_text)
                };
                let sort_info = match self.sort_column {
                    SortColumn::None => String::new(),
                    _ => {
                        let col = match self.sort_column {
                            SortColumn::PidAsc | SortColumn::PidDesc => "PID",
                            SortColumn::NameAsc | SortColumn::NameDesc => "Name",
                            SortColumn::TypeAsc | SortColumn::TypeDesc => "Type",
                            SortColumn::VramAsc | SortColumn::VramDesc => "VRAM",
                            SortColumn::None => "",
                        };
                        let dir = if self.sort_column.is_descending() {
                            "▼"
                        } else {
                            "▲"
                        };
                        format!(" | sort: {}{}", col, dir)
                    }
                };
                format!(
                    " {} processes{}{}",
                    self.processes.len(),
                    filter_info,
                    sort_info
                )
            }
        };

        let status_style = match self.input_mode {
            InputMode::Filter | InputMode::Sort => Style::default().fg(yellow).bg(bg_dark),
            InputMode::Normal => Style::default().fg(accent).bg(bg_dark),
        };
        let status_line = Paragraph::new(status_text).style(status_style);
        f.render_widget(status_line, chunks[1]);
    }

    fn draw_nvtop_processes(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();
        let fg = self.theme.fg.to_ratatui();
        let green = self.theme.green.to_ratatui();
        let yellow = self.theme.yellow.to_ratatui();
        let bg_dark = self.theme.bg_dark.to_ratatui();

        // Process table block
        let block = Block::default()
            .title(" Processes ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent));

        let inner = block.inner(area);
        f.render_widget(block, area);

        if self.processes.is_empty() {
            let msg = if self.filter_text.is_empty() {
                "No GPU processes running"
            } else {
                "No processes match filter"
            };
            let para = Paragraph::new(msg)
                .style(Style::default().fg(self.theme.comment.to_ratatui()))
                .alignment(Alignment::Center);
            f.render_widget(para, inner);
            return;
        }

        // Build header with sort indicators
        let headers = ["PID", "Name", "Type", "VRAM"];
        let header_cells: Vec<Cell> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let indicator = if self.sort_column.column_index() == Some(i) {
                    if self.sort_column.is_descending() {
                        "▼"
                    } else {
                        "▲"
                    }
                } else {
                    ""
                };
                let style = if self.sort_column.column_index() == Some(i) {
                    Style::default().fg(green)
                } else {
                    Style::default().fg(accent)
                };
                Cell::from(format!("{}{}", h, indicator)).style(style)
            })
            .collect();

        let header = Row::new(header_cells).height(1);

        // Build rows
        let rows: Vec<Row> = self
            .processes
            .iter()
            .map(|p| {
                let type_style = match p.process_type {
                    ProcessType::Graphics => Style::default().fg(green),
                    ProcessType::Compute => Style::default().fg(yellow),
                };
                Row::new(vec![
                    Cell::from(format!("{:>6}", p.pid)).style(Style::default().fg(fg)),
                    Cell::from(p.name.clone()).style(Style::default().fg(fg)),
                    Cell::from(p.process_type.as_str()).style(type_style),
                    Cell::from(format!("{:>6} MB", p.vram_mb)).style(Style::default().fg(fg)),
                ])
            })
            .collect();

        let widths = [
            Constraint::Length(8),
            Constraint::Min(15),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(Style::default().bg(bg_dark).add_modifier(Modifier::BOLD))
            .highlight_symbol("▶ ");

        f.render_stateful_widget(table, inner, &mut self.process_table_state.clone());

        // Scrollbar if needed
        let visible_rows = inner.height.saturating_sub(2) as usize; // -2 for header
        if self.processes.len() > visible_rows {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));
            f.render_stateful_widget(scrollbar, inner, &mut self.process_scrollbar_state.clone());
        }
    }

    fn draw_nvtop_header(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();

        let (gpu_name, driver) = if let Some(ref ctx) = self.backend_ctx {
            let name = ctx
                .nvml
                .get_name(self.selected_gpu as u32)
                .unwrap_or_else(|_| "Unknown GPU".to_string());
            (name, ctx.driver_version.clone())
        } else {
            ("Initializing...".to_string(), "...".to_string())
        };

        let header_text = format!(
            " {} | Driver: {} | GPU {}/{} ",
            gpu_name,
            driver,
            self.selected_gpu + 1,
            self.device_count.max(1)
        );

        let block = Block::default()
            .title(header_text)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent));

        // Get current metrics
        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            if let Some(metrics) = history.back() {
                let status = format!(
                    "Temp: {}°C | GPU: {}% | Mem: {}% | Power: {:.0}W | Fan: {}% | Clock: {} MHz",
                    metrics.temperature as u32,
                    metrics.gpu_utilization as u32,
                    metrics.memory_utilization as u32,
                    metrics.power_draw,
                    metrics.fan_speed as u32,
                    metrics.gpu_clock as u32,
                );
                let para = Paragraph::new(status)
                    .block(block)
                    .alignment(Alignment::Center);
                f.render_widget(para, area);
                return;
            }
        }

        let para = Paragraph::new("Loading metrics...")
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(para, area);
    }

    fn draw_nvtop_gauges(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();
        let green = self.theme.green.to_ratatui();
        let yellow = self.theme.yellow.to_ratatui();
        let red = self.theme.red.to_ratatui();
        let cyan = self.theme.cyan.to_ratatui();
        let purple = self.theme.purple.to_ratatui();
        let fg = self.theme.fg.to_ratatui();

        // Two-column layout like htop
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            if let Some(m) = history.back() {
                // Left column: GPU, Memory, Power
                let left_gauges = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Min(0),
                    ])
                    .split(cols[0]);

                // GPU utilization
                let gpu_label = format!("GPU [{:>3}%]", m.gpu_utilization as u32);
                let gpu_gauge = Gauge::default()
                    .block(Block::default().title(gpu_label))
                    .gauge_style(
                        Style::default()
                            .fg(green)
                            .bg(self.theme.bg_dark.to_ratatui()),
                    )
                    .percent(m.gpu_utilization as u16)
                    .label("");
                f.render_widget(gpu_gauge, left_gauges[0]);

                // Memory utilization
                let mem_label = format!("Mem [{:>3}%]", m.memory_utilization as u32);
                let mem_gauge = Gauge::default()
                    .block(Block::default().title(mem_label))
                    .gauge_style(
                        Style::default()
                            .fg(yellow)
                            .bg(self.theme.bg_dark.to_ratatui()),
                    )
                    .percent(m.memory_utilization as u16)
                    .label("");
                f.render_widget(mem_gauge, left_gauges[1]);

                // Power
                let power_pct = (m.power_draw / 450.0 * 100.0).min(100.0) as u16;
                let pwr_label = format!("Pwr [{:>3.0}W]", m.power_draw);
                let pwr_gauge = Gauge::default()
                    .block(Block::default().title(pwr_label))
                    .gauge_style(
                        Style::default()
                            .fg(purple)
                            .bg(self.theme.bg_dark.to_ratatui()),
                    )
                    .percent(power_pct)
                    .label("");
                f.render_widget(pwr_gauge, left_gauges[2]);

                // Right column: Temp, Fan, Clocks
                let right_gauges = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Length(2),
                        Constraint::Min(0),
                    ])
                    .split(cols[1]);

                // Temperature
                let temp_pct = (m.temperature / 100.0 * 100.0).min(100.0) as u16;
                let temp_color = if m.temperature > 80.0 {
                    red
                } else if m.temperature > 60.0 {
                    yellow
                } else {
                    green
                };
                let tmp_label = format!("Tmp [{:>3}°C]", m.temperature as u32);
                let tmp_gauge = Gauge::default()
                    .block(Block::default().title(tmp_label))
                    .gauge_style(
                        Style::default()
                            .fg(temp_color)
                            .bg(self.theme.bg_dark.to_ratatui()),
                    )
                    .percent(temp_pct)
                    .label("");
                f.render_widget(tmp_gauge, right_gauges[0]);

                // Fan
                let fan_label = format!("Fan [{:>3}%]", m.fan_speed as u32);
                let fan_gauge = Gauge::default()
                    .block(Block::default().title(fan_label))
                    .gauge_style(
                        Style::default()
                            .fg(cyan)
                            .bg(self.theme.bg_dark.to_ratatui()),
                    )
                    .percent(m.fan_speed as u16)
                    .label("");
                f.render_widget(fan_gauge, right_gauges[1]);

                // Clocks info (text, not gauge)
                let clocks_text = format!(
                    " Core: {} MHz  |  Mem: {} MHz",
                    m.gpu_clock as u32, m.memory_clock as u32
                );
                let clocks = Paragraph::new(clocks_text).style(Style::default().fg(fg));
                f.render_widget(clocks, right_gauges[2]);
            }
        } else {
            // No data yet
            let loading = Paragraph::new("Waiting for GPU data...")
                .style(Style::default().fg(accent))
                .alignment(Alignment::Center);
            f.render_widget(loading, area);
        }
    }

    fn draw_dashboard(&self, f: &mut Frame) {
        let area = f.area();

        // Use theme colors
        let accent = self.theme.teal.to_ratatui();
        let fg = self.theme.fg.to_ratatui();
        let comment = self.theme.comment.to_ratatui();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Tabs
                Constraint::Min(10),   // Content
                Constraint::Length(1), // Footer
            ])
            .split(area);

        // Tabs
        let titles: Vec<&str> = Tab::titles();
        let tabs = Tabs::new(titles.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" nvctl dashboard ")
                    .border_style(Style::default().fg(accent)),
            )
            .select(self.current_tab)
            .style(Style::default().fg(fg))
            .highlight_style(Style::default().fg(accent).add_modifier(Modifier::BOLD));
        f.render_widget(tabs, chunks[0]);

        // Content based on current tab
        let current_tab = Tab::from_index(self.current_tab);
        self.draw_tab_content(f, chunks[1], current_tab);

        // Footer
        let footer = Paragraph::new(" tab:next · 1-9:jump · n:nvtop · m:menu · q:quit · ?:help ")
            .style(Style::default().fg(comment));
        f.render_widget(footer, chunks[2]);

        // Settings overlay
        if self.show_settings {
            self.draw_settings_overlay(f);
        }
    }

    fn draw_tab_content(&self, f: &mut Frame, area: Rect, tab: Tab) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", Tab::titles()[self.current_tab]));

        let inner = block.inner(area);
        f.render_widget(block, area);

        match tab {
            Tab::Overview => self.draw_overview_tab(f, inner),
            Tab::Performance => self.draw_performance_tab(f, inner),
            Tab::Memory => self.draw_memory_tab(f, inner),
            Tab::Temperature => self.draw_temperature_tab(f, inner),
            Tab::Power => self.draw_power_tab(f, inner),
            Tab::Processes => self.draw_processes_tab(f, inner),
            Tab::FanControl => self.draw_fan_tab(f, inner),
            Tab::Overclocking => self.draw_oc_tab(f, inner),
            Tab::Tuner => self.draw_tuner_tab(f, inner),
            Tab::Profiler => self.draw_profiler_tab(f, inner),
            Tab::Osd => self.draw_osd_tab(f, inner),
            Tab::Drivers => self.draw_drivers_tab(f, inner),
            Tab::Settings => self.draw_settings_tab(f, inner),
            _ => self.draw_generic_tab(f, inner, tab),
        }
    }

    fn draw_overview_tab(&self, f: &mut Frame, area: Rect) {
        let mut lines = vec![];

        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            if let Some(m) = history.back() {
                lines.push(format!("GPU Utilization:    {}%", m.gpu_utilization as u32));
                lines.push(format!(
                    "Memory Utilization: {}%",
                    m.memory_utilization as u32
                ));
                lines.push(format!("Temperature:        {}°C", m.temperature as u32));
                lines.push(format!("Fan Speed:          {}%", m.fan_speed as u32));
                lines.push(format!("Power Draw:         {:.1}W", m.power_draw));
                lines.push(format!("GPU Clock:          {} MHz", m.gpu_clock as u32));
                lines.push(format!("Memory Clock:       {} MHz", m.memory_clock as u32));
            }
        }

        let text = lines.join("\n");
        let para = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(para, area);
    }

    fn draw_performance_tab(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();
        let green = self.theme.green.to_ratatui();
        let yellow = self.theme.yellow.to_ratatui();
        let purple = self.theme.purple.to_ratatui();
        let fg = self.theme.fg.to_ratatui();
        let bg_dark = self.theme.bg_dark.to_ratatui();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // GPU Clock gauge
                Constraint::Length(3), // Memory Clock gauge
                Constraint::Length(3), // GPU Utilization gauge
                Constraint::Min(3),    // Info section
            ])
            .split(area);

        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            if let Some(m) = history.back() {
                // GPU Clock (assume max ~3000 MHz for modern GPUs)
                let gpu_clock_pct = ((m.gpu_clock / 3000.0) * 100.0).min(100.0) as u16;
                let gpu_clock_gauge = Gauge::default()
                    .block(
                        Block::default().title(format!(" GPU Clock: {} MHz ", m.gpu_clock as u32)),
                    )
                    .gauge_style(Style::default().fg(green).bg(bg_dark))
                    .percent(gpu_clock_pct)
                    .label("");
                f.render_widget(gpu_clock_gauge, chunks[0]);

                // Memory Clock (assume max ~12000 MHz for GDDR6X)
                let mem_clock_pct = ((m.memory_clock / 12000.0) * 100.0).min(100.0) as u16;
                let mem_clock_gauge = Gauge::default()
                    .block(
                        Block::default()
                            .title(format!(" Memory Clock: {} MHz ", m.memory_clock as u32)),
                    )
                    .gauge_style(Style::default().fg(yellow).bg(bg_dark))
                    .percent(mem_clock_pct)
                    .label("");
                f.render_widget(mem_clock_gauge, chunks[1]);

                // GPU Utilization
                let gpu_util_gauge = Gauge::default()
                    .block(
                        Block::default()
                            .title(format!(" GPU Utilization: {}% ", m.gpu_utilization as u32)),
                    )
                    .gauge_style(Style::default().fg(purple).bg(bg_dark))
                    .percent(m.gpu_utilization as u16)
                    .label("");
                f.render_widget(gpu_util_gauge, chunks[2]);

                // Info section with sparkline history (toggle with 'g')
                if self.show_graphs {
                    let info_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(chunks[3]);

                    // GPU utilization history sparkline
                    let gpu_history: Vec<u64> =
                        history.iter().map(|m| m.gpu_utilization as u64).collect();
                    let sparkline = Sparkline::default()
                        .block(
                            Block::default()
                                .title(" GPU History ")
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(accent)),
                        )
                        .data(&gpu_history)
                        .style(Style::default().fg(green));
                    f.render_widget(sparkline, info_chunks[0]);

                    // Clock history sparkline
                    let clock_history: Vec<u64> =
                        history.iter().map(|m| m.gpu_clock as u64).collect();
                    let clock_sparkline = Sparkline::default()
                        .block(
                            Block::default()
                                .title(" Clock History ")
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(accent)),
                        )
                        .data(&clock_history)
                        .style(Style::default().fg(yellow));
                    f.render_widget(clock_sparkline, info_chunks[1]);
                }
            }
        } else {
            let para = Paragraph::new("No performance data available")
                .style(Style::default().fg(fg))
                .alignment(Alignment::Center);
            f.render_widget(para, area);
        }
    }

    fn draw_memory_tab(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();
        let cyan = self.theme.cyan.to_ratatui();
        let fg = self.theme.fg.to_ratatui();
        let bg_dark = self.theme.bg_dark.to_ratatui();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Memory utilization gauge
                Constraint::Length(3), // Spacer
                Constraint::Min(5),    // Memory breakdown
            ])
            .split(area);

        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            if let Some(m) = history.back() {
                // Memory utilization gauge
                let mem_gauge = Gauge::default()
                    .block(Block::default().title(format!(
                        " Memory Utilization: {}% ",
                        m.memory_utilization as u32
                    )))
                    .gauge_style(Style::default().fg(cyan).bg(bg_dark))
                    .percent(m.memory_utilization as u16)
                    .label("");
                f.render_widget(mem_gauge, chunks[0]);

                // Memory breakdown info
                if let Some(ref ctx) = self.backend_ctx {
                    if let Ok(metrics) = ctx.get_metrics(self.selected_gpu as u32) {
                        let used_mb = metrics.memory_used_bytes / (1024 * 1024);
                        let total_mb = metrics.memory_total_bytes / (1024 * 1024);
                        let free_mb = total_mb.saturating_sub(used_mb);

                        let info = format!(
                            "VRAM Usage:\n\n  Used:  {:>6} MB\n  Free:  {:>6} MB\n  Total: {:>6} MB\n\n  Memory Bus: {} MHz",
                            used_mb, free_mb, total_mb, m.memory_clock as u32
                        );
                        let info_para = Paragraph::new(info)
                            .block(
                                Block::default()
                                    .title(" Memory Details ")
                                    .borders(Borders::ALL)
                                    .border_style(Style::default().fg(accent)),
                            )
                            .style(Style::default().fg(fg));
                        f.render_widget(info_para, chunks[2]);
                    }
                }
            }
        } else {
            let para = Paragraph::new("No memory data available")
                .style(Style::default().fg(fg))
                .alignment(Alignment::Center);
            f.render_widget(para, area);
        }
    }

    fn draw_temperature_tab(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();
        let green = self.theme.green.to_ratatui();
        let yellow = self.theme.yellow.to_ratatui();
        let red = self.theme.red.to_ratatui();
        let fg = self.theme.fg.to_ratatui();
        let bg_dark = self.theme.bg_dark.to_ratatui();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Temperature gauge
                Constraint::Length(3), // Fan gauge
                Constraint::Min(5),    // History graph
            ])
            .split(area);

        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            if let Some(m) = history.back() {
                // Temperature gauge with color coding
                let temp = m.temperature as u32;
                let temp_color = if temp >= 80 {
                    red
                } else if temp >= 65 {
                    yellow
                } else {
                    green
                };
                let temp_pct = ((temp as f64 / 100.0) * 100.0).min(100.0) as u16;
                let temp_status = if temp >= 80 {
                    "HOT!"
                } else if temp >= 65 {
                    "Warm"
                } else {
                    "OK"
                };

                let temp_gauge = Gauge::default()
                    .block(
                        Block::default()
                            .title(format!(" Temperature: {}°C [{}] ", temp, temp_status)),
                    )
                    .gauge_style(Style::default().fg(temp_color).bg(bg_dark))
                    .percent(temp_pct)
                    .label("");
                f.render_widget(temp_gauge, chunks[0]);

                // Fan speed gauge
                let fan_gauge = Gauge::default()
                    .block(Block::default().title(format!(" Fan Speed: {}% ", m.fan_speed as u32)))
                    .gauge_style(Style::default().fg(accent).bg(bg_dark))
                    .percent(m.fan_speed as u16)
                    .label("");
                f.render_widget(fan_gauge, chunks[1]);

                // Temperature history sparkline (toggle with 'g')
                if self.show_graphs {
                    let temp_history: Vec<u64> =
                        history.iter().map(|m| m.temperature as u64).collect();
                    let sparkline = Sparkline::default()
                        .block(
                            Block::default()
                                .title(" Temperature History ")
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(accent)),
                        )
                        .data(&temp_history)
                        .style(Style::default().fg(temp_color));
                    f.render_widget(sparkline, chunks[2]);
                }
            }
        } else {
            let para = Paragraph::new("No temperature data available")
                .style(Style::default().fg(fg))
                .alignment(Alignment::Center);
            f.render_widget(para, area);
        }
    }

    fn draw_power_tab(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();
        let green = self.theme.green.to_ratatui();
        let yellow = self.theme.yellow.to_ratatui();
        let orange = self.theme.orange.to_ratatui();
        let fg = self.theme.fg.to_ratatui();
        let bg_dark = self.theme.bg_dark.to_ratatui();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Power draw gauge
                Constraint::Length(5), // Power info
                Constraint::Min(5),    // History graph
            ])
            .split(area);

        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            if let Some(m) = history.back() {
                // Estimate power limit (assume 450W max for high-end cards)
                let power_limit = 450.0;
                let power_pct = ((m.power_draw / power_limit) * 100.0).min(100.0) as u16;
                let power_color = if power_pct >= 90 {
                    orange
                } else if power_pct >= 70 {
                    yellow
                } else {
                    green
                };

                let power_gauge = Gauge::default()
                    .block(Block::default().title(format!(" Power Draw: {:.1}W ", m.power_draw)))
                    .gauge_style(Style::default().fg(power_color).bg(bg_dark))
                    .percent(power_pct)
                    .label("");
                f.render_widget(power_gauge, chunks[0]);

                // Power info
                let power_info = format!(
                    "  Current Draw:  {:.1}W\n  Est. TDP:      ~{}W\n  Efficiency:    {}%",
                    m.power_draw, power_limit as u32, power_pct
                );
                let info_para = Paragraph::new(power_info)
                    .block(
                        Block::default()
                            .title(" Power Details ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(accent)),
                    )
                    .style(Style::default().fg(fg));
                f.render_widget(info_para, chunks[1]);

                // Power history sparkline (toggle with 'g')
                if self.show_graphs {
                    let power_history: Vec<u64> =
                        history.iter().map(|m| m.power_draw as u64).collect();
                    let sparkline = Sparkline::default()
                        .block(
                            Block::default()
                                .title(" Power History ")
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(accent)),
                        )
                        .data(&power_history)
                        .style(Style::default().fg(power_color));
                    f.render_widget(sparkline, chunks[2]);
                }
            }
        } else {
            let para = Paragraph::new("No power data available")
                .style(Style::default().fg(fg))
                .alignment(Alignment::Center);
            f.render_widget(para, area);
        }
    }

    fn draw_processes_tab(&self, f: &mut Frame, area: Rect) {
        let accent = self.theme.teal.to_ratatui();
        let fg = self.theme.fg.to_ratatui();
        let yellow = self.theme.yellow.to_ratatui();
        let green = self.theme.green.to_ratatui();
        let bg_dark = self.theme.bg_dark.to_ratatui();

        // Layout: process table with optional filter/sort footer
        let footer_height = match self.input_mode {
            InputMode::Filter | InputMode::Sort => 3,
            InputMode::Normal => 2,
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(footer_height)])
            .split(area);

        // Build table headers with sort indicators
        let headers = ["PID", "Name", "Type", "VRAM (MB)"];
        let header_cells: Vec<Cell> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let style = if self.input_mode == InputMode::Sort && self.sort_selected_column == i
                {
                    Style::default().fg(yellow).add_modifier(Modifier::BOLD)
                } else if self.sort_column.column_index() == Some(i) {
                    Style::default().fg(green)
                } else {
                    Style::default().fg(accent)
                };

                // Add sort indicator
                let indicator = if self.sort_column.column_index() == Some(i) {
                    if self.sort_column.is_descending() {
                        " ▼"
                    } else {
                        " ▲"
                    }
                } else {
                    ""
                };

                Cell::from(format!("{}{}", h, indicator)).style(style)
            })
            .collect();

        let header = Row::new(header_cells).height(1).bottom_margin(1);

        // Build rows from process entries
        let rows: Vec<Row> = self
            .processes
            .iter()
            .map(|p| {
                let type_style = match p.process_type {
                    ProcessType::Graphics => Style::default().fg(green),
                    ProcessType::Compute => Style::default().fg(yellow),
                };

                Row::new(vec![
                    Cell::from(p.pid.to_string()).style(Style::default().fg(fg)),
                    Cell::from(p.name.clone()).style(Style::default().fg(fg)),
                    Cell::from(p.process_type.as_str()).style(type_style),
                    Cell::from(p.vram_mb.to_string()).style(Style::default().fg(fg)),
                ])
            })
            .collect();

        let widths = [
            Constraint::Length(8),
            Constraint::Min(20),
            Constraint::Length(10),
            Constraint::Length(12),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .row_highlight_style(Style::default().bg(bg_dark).add_modifier(Modifier::BOLD))
            .highlight_symbol("▶ ");

        // Render table with scrollbar
        let table_area = chunks[0];
        f.render_stateful_widget(table, table_area, &mut self.process_table_state.clone());

        // Scrollbar
        if self.processes.len() > (table_area.height as usize).saturating_sub(3) {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));
            f.render_stateful_widget(
                scrollbar,
                table_area,
                &mut self.process_scrollbar_state.clone(),
            );
        }

        // Footer with keybinds / filter / sort
        let footer_area = chunks[1];
        match self.input_mode {
            InputMode::Filter => {
                let filter_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(yellow))
                    .title(" Filter (Enter/Esc to exit) ");
                let filter_text =
                    Paragraph::new(format!("/{}", self.filter_text)).block(filter_block);
                f.render_widget(filter_text, footer_area);
            }
            InputMode::Sort => {
                let sort_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(yellow))
                    .title(" Sort (←/→ select, Enter confirm, Esc cancel) ");
                let col_names = ["PID", "Name", "Type", "VRAM"];
                let sort_text = col_names
                    .iter()
                    .enumerate()
                    .map(|(i, name)| {
                        if i == self.sort_selected_column {
                            format!("[{}]", name)
                        } else {
                            name.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("  ");
                let sort_para = Paragraph::new(sort_text)
                    .block(sort_block)
                    .alignment(Alignment::Center);
                f.render_widget(sort_para, footer_area);
            }
            InputMode::Normal => {
                let filter_indicator = if self.filter_text.is_empty() {
                    String::new()
                } else {
                    format!(" | Filter: \"{}\" (Esc to clear)", self.filter_text)
                };
                let footer = format!(
                    " (f)ilter | (s)ort | (↑/↓) navigate | {} processes{}",
                    self.processes.len(),
                    filter_indicator
                );
                let footer_para = Paragraph::new(footer)
                    .style(Style::default().fg(accent))
                    .alignment(Alignment::Center);
                f.render_widget(footer_para, footer_area);
            }
        }
    }

    fn draw_fan_tab(&self, f: &mut Frame, area: Rect) {
        let mode_str = if self.fan_control_mode {
            "MANUAL"
        } else {
            "AUTO"
        };
        let mut lines = vec![
            format!("Fan Mode: {}", mode_str),
            format!("Target Speed: {}%", self.fan_speed_target),
            String::new(),
            "Fan Curve:".to_string(),
        ];

        for (i, (temp, speed)) in self.fan_curve_points.iter().enumerate() {
            let marker = if i == self.selected_curve_point {
                ">"
            } else {
                " "
            };
            lines.push(format!("{} {}°C -> {}%", marker, temp, speed));
        }

        let text = lines.join("\n");
        let para = Paragraph::new(text);
        f.render_widget(para, area);
    }

    fn draw_oc_tab(&self, f: &mut Frame, area: Rect) {
        let mode_str = if self.oc_control_mode {
            "ACTIVE"
        } else {
            "INACTIVE"
        };
        let lines = [
            format!("Overclock Mode: {}", mode_str),
            format!("Preset: {:?}", self.oc_preset),
            String::new(),
            format!("GPU Clock Offset:    {:+} MHz", self.gpu_offset),
            format!("Memory Clock Offset: {:+} MHz", self.memory_offset),
            format!("Power Limit:         {}%", self.power_limit_percent),
        ];

        let text = lines.join("\n");
        let para = Paragraph::new(text);
        f.render_widget(para, area);
    }

    fn draw_tuner_tab(&self, f: &mut Frame, area: Rect) {
        let mut lines = vec![format!("Tuner States: {} GPUs", self.tuner_states.len())];

        for (i, state) in self.tuner_states.iter().enumerate() {
            lines.push(format!(
                "GPU {}: Core {:+}MHz, Mem {:+}MHz, Power {}%, Temp {}°C",
                i,
                state.core_clock_offset,
                state.memory_clock_offset,
                state.power_limit,
                state.temp_limit
            ));
        }

        let text = lines.join("\n");
        let para = Paragraph::new(text);
        f.render_widget(para, area);
    }

    fn draw_profiler_tab(&self, f: &mut Frame, area: Rect) {
        let status = if self.profiler.is_some() {
            if self.profiler_recording {
                "Recording..."
            } else {
                "Ready"
            }
        } else {
            "Not available"
        };

        let lines = [
            format!("Profiler Status: {}", status),
            String::new(),
            "Press 'r' to start/stop recording".to_string(),
        ];

        let text = lines.join("\n");
        let para = Paragraph::new(text);
        f.render_widget(para, area);
    }

    fn draw_osd_tab(&self, f: &mut Frame, area: Rect) {
        let osd_status = if self.osd_enabled {
            "Enabled"
        } else {
            "Disabled"
        };
        let metrics = ["FPS", "GPU Usage", "VRAM", "Temperature", "Power"];
        let selected = metrics.get(self.osd_selected_metric).unwrap_or(&"None");

        let lines = [
            format!("OSD Status: {}", osd_status),
            format!("Selected Metric: {}", selected),
            String::new(),
            "MangoHud integration for on-screen display".to_string(),
        ];

        let text = lines.join("\n");
        let para = Paragraph::new(text);
        f.render_widget(para, area);
    }

    fn draw_drivers_tab(&self, f: &mut Frame, area: Rect) {
        let mut lines = vec![];

        if let Some(ref caps) = self.driver_capabilities {
            lines.push(format!("Driver Version: {}", caps.version));
            lines.push(format!("Major Version: {}", caps.major_version));
            lines.push(format!(
                "Beta Driver: {}",
                if caps.is_beta { "Yes" } else { "No" }
            ));
            lines.push(format!(
                "Vulkan Swapchain Perf: {}",
                caps.has_vulkan_swapchain_perf
            ));
            lines.push(format!("USB4 DP Support: {}", caps.has_usb4_dp_support));
            lines.push(format!("PREEMPT_RT Support: {}", caps.supports_preempt_rt));
        } else {
            lines.push("Driver capabilities not available".to_string());
        }

        lines.push(String::new());

        if let Some(ref validation) = self.driver_validation {
            lines.push(format!("Validation Passed: {}", validation.passed));
            lines.push(format!("Wayland OK: {:?}", validation.wayland_ok));
            lines.push(format!("glibc OK: {:?}", validation.glibc_ok));
            if !validation.warnings.is_empty() {
                lines.push(format!("Warnings: {}", validation.warnings.len()));
            }
        }

        let text = lines.join("\n");
        let para = Paragraph::new(text);
        f.render_widget(para, area);
    }

    fn draw_settings_tab(&self, f: &mut Frame, area: Rect) {
        let lines = [
            format!("Theme: {}", self.current_theme.name()),
            format!("Update Interval: {}ms", self.update_interval.as_millis()),
            format!("Show Settings Overlay: {}", self.show_settings),
            String::new(),
            "Press 't' to cycle themes".to_string(),
            "Press 's' to toggle settings overlay".to_string(),
        ];

        let text = lines.join("\n");
        let para = Paragraph::new(text);
        f.render_widget(para, area);
    }

    fn draw_generic_tab(&self, f: &mut Frame, area: Rect, tab: Tab) {
        let mut lines = vec![];

        if let Some(history) = self.metrics_history.get(self.selected_gpu) {
            if let Some(m) = history.back() {
                match tab {
                    Tab::Performance => {
                        lines.push(format!("GPU Clock: {} MHz", m.gpu_clock as u32));
                        lines.push(format!("Memory Clock: {} MHz", m.memory_clock as u32));
                        lines.push(format!("GPU Utilization: {}%", m.gpu_utilization as u32));
                    }
                    Tab::Memory => {
                        lines.push(format!(
                            "Memory Utilization: {}%",
                            m.memory_utilization as u32
                        ));
                    }
                    Tab::Temperature => {
                        lines.push(format!("Temperature: {}°C", m.temperature as u32));
                    }
                    Tab::Power => {
                        lines.push(format!("Power Draw: {:.1}W", m.power_draw));
                    }
                    Tab::Processes => {
                        lines.push("GPU Processes: (requires nvidia-smi)".to_string());
                    }
                    Tab::Profiles => {
                        lines.push("Saved Profiles:".to_string());
                        lines.push("  - Default".to_string());
                        lines.push("  - Gaming".to_string());
                        lines.push("  - Quiet".to_string());
                    }
                    _ => {}
                }
            }
        }

        if lines.is_empty() {
            lines.push("No data available".to_string());
        }

        let text = lines.join("\n");
        let para = Paragraph::new(text);
        f.render_widget(para, area);
    }

    fn draw_settings_overlay(&self, f: &mut Frame) {
        let area = f.area();
        let popup_width = 50.min(area.width - 4);
        let popup_height = 15.min(area.height - 4);
        let popup_x = (area.width - popup_width) / 2;
        let popup_y = (area.height - popup_height) / 2;
        let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

        f.render_widget(Clear, popup_area);

        let settings_text = format!(
            "Settings\n\nTheme: {}\nRefresh: {}ms\n\nPress 's' to close",
            self.current_theme.name(),
            self.update_interval.as_millis()
        );

        let accent = self.theme.teal.to_ratatui();
        let popup = Paragraph::new(settings_text).block(
            Block::default()
                .title(" Settings ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent)),
        );
        f.render_widget(popup, popup_area);
    }

    fn draw_help(&self, f: &mut Frame) {
        let area = f.area();
        let accent = self.theme.teal.to_ratatui();
        let fg = self.theme.fg.to_ratatui();

        // Center popup
        let popup_width = 60.min(area.width - 4);
        let popup_height = 28.min(area.height - 4);
        let popup_x = (area.width - popup_width) / 2;
        let popup_y = (area.height - popup_height) / 2;
        let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

        // Clear background
        f.render_widget(Clear, popup_area);

        let help_text = r#"
 Keyboard Shortcuts

 Global:
   q, Ctrl+C    Quit
   ?/F1         Toggle help
   t            Cycle theme

 Navigation:
   Tab          Next tab/GPU
   Shift+Tab    Previous tab/GPU
   1-9          Jump to tab
   m            Main menu
   n            Nvtop view
   d            Dashboard view

 Controls:
   Space/p      Pause updates

 Process Table (nvtop/Processes tab):
   j/↓          Select next process
   k/↑          Select previous process
   Mouse wheel  Scroll process list
   f or /       Filter processes
   s            Sort by column
   Esc          Clear filter
   g            Toggle graphs
"#;

        let help = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title(" Help ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(accent)),
            )
            .style(Style::default().fg(fg));

        f.render_widget(help, popup_area);
    }
}

impl Default for TuiApp {
    fn default() -> Self {
        Self::new()
    }
}

// Public launch functions for CLI
/// Launch TUI with menu
pub fn launch_menu() -> NvResult<()> {
    let mut app = TuiApp::with_view(ViewMode::Menu);
    app.run()
}

/// Launch nvtop-style monitor
pub fn launch_nvtop() -> NvResult<()> {
    let mut app = TuiApp::with_view(ViewMode::Nvtop);
    app.run()
}

/// Launch full dashboard
pub fn launch_dashboard() -> NvResult<()> {
    let mut app = TuiApp::with_view(ViewMode::Dashboard);
    app.run()
}
