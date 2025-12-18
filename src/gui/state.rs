//! GUI State Management
//!
//! Unified state for the GUI application, integrating with the
//! shared state.rs and config.rs modules.

use crate::config::Config;
use crate::fan::FanInfo;
use crate::gui_widgets::{FanCurve, MonitoringDashboard, VoltageCurve};
use crate::multi_gpu::GpuInfo;
use crate::overclocking::OverclockProfile;
use crate::state::AppState;
use crate::themes::ThemeVariant;

use super::tabs::Tab;
use super::toast::ToastManager;

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{Receiver, Sender};

/// GPU statistics snapshot
#[derive(Debug, Clone, Default)]
pub struct GpuStats {
    pub name: String,
    pub architecture: String,
    pub driver_version: String,
    pub cuda_cores: u32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub temperature: f32,
    pub utilization: f32,
    pub fan_speed: u32,
    pub power_draw: f32,
    pub power_limit: f32,
    pub core_clock: u32,
    pub memory_clock: u32,
    pub pci_bus: String,
    pub compute_capability: String,
}

/// Container information for the containers tab
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub gpu_usage: String,
}

/// Overclock preset
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OcPreset {
    Stock,       // 0/0/100%
    MildOc,      // +75/+500/105%
    Performance, // +150/+1000/110%
    Extreme,     // +200/+1500/115%
}

impl OcPreset {
    pub fn name(&self) -> &'static str {
        match self {
            OcPreset::Stock => "Stock",
            OcPreset::MildOc => "Mild OC",
            OcPreset::Performance => "Performance",
            OcPreset::Extreme => "Extreme",
        }
    }

    pub fn values(&self) -> (i32, i32, u32) {
        match self {
            OcPreset::Stock => (0, 0, 100),
            OcPreset::MildOc => (75, 500, 105),
            OcPreset::Performance => (150, 1000, 110),
            OcPreset::Extreme => (200, 1500, 115),
        }
    }
}

/// Fan control mode
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FanMode {
    Auto,
    Manual,
    Curve,
}

impl FanMode {
    pub fn name(&self) -> &'static str {
        match self {
            FanMode::Auto => "Auto",
            FanMode::Manual => "Manual",
            FanMode::Curve => "Custom Curve",
        }
    }
}

/// Unified GUI state
pub struct GuiState {
    // === Navigation ===
    pub tab: Tab,

    // === Theme ===
    pub current_theme: ThemeVariant,

    // === GPU Data ===
    pub gpu_stats: Option<GpuStats>,
    pub available_gpus: Vec<GpuInfo>,
    pub selected_gpu_index: u32,

    // Shared atomic for background thread to read current GPU selection
    selected_gpu_atomic: Arc<AtomicU32>,
    // Shutdown signal for background thread
    shutdown_signal: Arc<AtomicBool>,

    // Async GPU stats channel
    gpu_stats_rx: Receiver<GpuStats>,
    #[allow(dead_code)]
    gpu_stats_tx: Sender<GpuStats>,
    pub last_stats_update: std::time::Instant,

    // === UI Settings ===
    pub ui_scale: f32,

    // === Configuration ===
    pub config: Config,
    pub app_state: Option<AppState>,

    // === Toast Notifications ===
    pub toasts: ToastManager,

    // === Overclock Settings ===
    pub overclock_profile: OverclockProfile,
    pub oc_preset: OcPreset,
    pub gpu_offset: i32,          // -200 to +200 MHz
    pub memory_offset: i32,       // -1000 to +1000 MHz
    pub power_limit_percent: u32, // 50 to 115%

    // === Auto-Overclock Wizard ===
    pub auto_oc_running: bool,
    pub auto_oc_target: String,
    pub auto_oc_safety: String,
    pub auto_oc_max_temp: f32,
    pub auto_oc_max_power: u32,

    // === Fan Control ===
    pub fan_curve: FanCurve,
    pub fan_mode: FanMode,
    pub fan_speeds: HashMap<usize, u8>,
    pub manual_fan_speed: u32,

    // === Voltage Curve ===
    pub voltage_curve: VoltageCurve,

    // === Power Curves ===
    pub power_config: crate::power_curves::PowerManagementConfig,

    // === Display Settings ===
    pub vibrance_levels: Vec<i16>,
    pub selected_icc_profile_idx: usize,
    pub available_icc_profiles: Vec<String>,

    // === HDR ===
    pub hdr_enabled: bool,
    pub hdr_config: crate::hdr::HdrConfig,

    // === VRR ===
    pub vrr_enabled: bool,
    pub vrr_displays: Vec<crate::vrr::DisplayVrrCapability>,

    // === Recording ===
    pub recording_active: bool,
    pub recording_path: String,
    pub recording_settings: crate::recording::RecordingSettings,

    // === OSD (On-Screen Display) ===
    pub osd_enabled: bool,
    pub osd_position: String,
    pub osd_metrics: Vec<String>,
    pub osd_font_size: u32,
    pub osd_opacity: f32,
    pub mangohud_installed: bool,

    // === RGB Control ===
    pub rgb_mode: String,
    pub rgb_color: [f32; 3],

    // === Container Management ===
    pub container_runtime: Option<crate::container_runtime::NvContainerRuntime>,
    pub containers: Vec<ContainerInfo>,

    // === Game Profiles ===
    pub game_auto_config: crate::game_profile_auto::AutoProfileConfig,

    // === Performance History (Ring Buffers) ===
    pub temp_history: VecDeque<f32>,
    pub util_history: VecDeque<f32>,
    pub power_history: VecDeque<f32>,
    pub memory_history: VecDeque<f32>,
    pub history_max_len: usize,

    // === Monitoring Dashboard ===
    pub monitoring_dashboard: MonitoringDashboard,

    // === Driver Validation ===
    pub driver_validation: Option<crate::state::DriverValidationState>,
    pub driver_capabilities: Option<crate::drivers::DriverCapabilities>,

    // === ASUS Power Monitor+ ===
    pub asus_power_detector: Option<crate::asus_power_detector::AsusPowerDetector>,
    pub asus_power_status: Option<crate::asus_power_detector::PowerConnectorStatus>,
    pub asus_power_last_update: std::time::Instant,

    // === Latency Settings ===
    pub latency_mode: String,
    pub reflex_enabled: bool,

    // === Gamescope ===
    pub gamescope_config: Option<crate::gamescope::GamescopeConfig>,

    // === Cached Fan Data (to avoid per-frame queries) ===
    pub cached_fans: Vec<FanInfo>,
    pub fans_last_update: std::time::Instant,

    // === Cached Display Data (to avoid subprocess spawns per frame) ===
    pub cached_displays: Vec<crate::display::DisplayInfo>,
    pub displays_last_update: std::time::Instant,
    pub cached_icc_profiles: Vec<String>,
    pub icc_profiles_last_update: std::time::Instant,

    // === Cached Recording Data (to avoid NVML init and file I/O per frame) ===
    pub cached_nvenc_caps: Option<crate::recording::NvencCapabilities>,
    pub nvenc_caps_last_update: std::time::Instant,
    pub cached_is_recording: bool,
    pub recording_status_last_update: std::time::Instant,
}

impl Default for GuiState {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GuiState {
    fn drop(&mut self) {
        // Signal background thread to stop
        self.shutdown_signal.store(true, Ordering::Relaxed);
    }
}

impl GuiState {
    /// Create new GUI state with defaults
    pub fn new() -> Self {
        let config = Config::load();
        let app_state = AppState::load().ok();

        // Detect theme from config
        let theme =
            ThemeVariant::from_config_key(&config.theme).unwrap_or(ThemeVariant::TokyoNightMoon);

        // Detect GPUs
        let available_gpus = crate::multi_gpu::detect_gpus().unwrap_or_default();

        // Load vibrance from config
        let display_count = crate::display::get_display_count();
        let vibrance_levels = if config.vibrance_levels.is_empty() {
            vec![0; display_count]
        } else {
            config.vibrance_levels.clone()
        };

        // Initialize ASUS Power Monitor+ if applicable
        let asus_power_detector = {
            let gpus = crate::asus_power_detector::detect_asus_gpus();
            gpus.into_iter()
                .find(|(_, model)| model.supports_power_detector())
                .and_then(|(pci_id, _)| {
                    crate::asus_power_detector::AsusPowerDetector::new(&pci_id).ok()
                })
                .filter(|d| d.is_supported())
        };

        // Create channel for async GPU stats
        let (tx, rx) = std::sync::mpsc::channel();

        // Shared atomic for GPU index - background thread reads this
        let selected_gpu_atomic = Arc::new(AtomicU32::new(0));
        let gpu_index_for_thread = Arc::clone(&selected_gpu_atomic);

        // Shutdown signal for clean exit
        let shutdown_signal = Arc::new(AtomicBool::new(false));
        let shutdown_for_thread = Arc::clone(&shutdown_signal);

        // Spawn background thread for GPU monitoring
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            while !shutdown_for_thread.load(Ordering::Relaxed) {
                // Read the currently selected GPU index atomically
                let gpu_index = gpu_index_for_thread.load(Ordering::Relaxed);

                if let Ok(nvml) = nvml_wrapper::Nvml::init() {
                    if let Ok(device) = nvml.device_by_index(gpu_index) {
                        let name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());
                        let temperature = device
                            .temperature(
                                nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu,
                            )
                            .unwrap_or(0) as f32;
                        let power_draw = device
                            .power_usage()
                            .map(|p| p as f32 / 1000.0)
                            .unwrap_or(0.0);
                        let power_limit = device
                            .power_management_limit()
                            .map(|p| p as f32 / 1000.0)
                            .unwrap_or(0.0);
                        let utilization = device
                            .utilization_rates()
                            .map(|u| u.gpu as f32)
                            .unwrap_or(0.0);
                        let mem_info = device.memory_info().ok();
                        let memory_used = mem_info.as_ref().map(|m| m.used).unwrap_or(0);
                        let memory_total = mem_info.as_ref().map(|m| m.total).unwrap_or(0);
                        let fan_speed = device.fan_speed(0).unwrap_or(0);
                        let core_clock = device
                            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
                            .unwrap_or(0);
                        let memory_clock = device
                            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
                            .unwrap_or(0);
                        let driver_version = nvml
                            .sys_driver_version()
                            .unwrap_or_else(|_| "Unknown".to_string());
                        let pci_info = device.pci_info().ok();
                        let pci_bus = pci_info
                            .map(|p| format!("{:04x}:{:02x}:{:02x}.0", p.domain, p.bus, p.device))
                            .unwrap_or_else(|| "Unknown".to_string());

                        // Detect architecture from compute capability
                        let compute_cap = device.cuda_compute_capability().ok();
                        let (architecture, compute_capability) = compute_cap
                            .map(|cc| {
                                let arch = match (cc.major, cc.minor) {
                                    (12, _) => "Blackwell",      // RTX 50 series (SM 12.0)
                                    (10, _) => "Blackwell",      // Blackwell alternate
                                    (8, 9) => "Ada Lovelace",    // RTX 40 series
                                    (8, 6) | (8, 0) => "Ampere", // RTX 30 series
                                    (7, 5) => "Turing",          // RTX 20 series
                                    (7, 0) => "Volta",           // Titan V, Tesla V100
                                    (6, _) => "Pascal",          // GTX 10 series
                                    (5, _) => "Maxwell",         // GTX 9 series
                                    _ => "Unknown",
                                };
                                (arch.to_string(), format!("SM {}.{}", cc.major, cc.minor))
                            })
                            .unwrap_or_else(|| ("Unknown".to_string(), "N/A".to_string()));

                        let cuda_cores = device.num_cores().unwrap_or(0);

                        let stats = GpuStats {
                            name,
                            architecture,
                            driver_version,
                            cuda_cores,
                            memory_total,
                            memory_used,
                            temperature,
                            utilization,
                            fan_speed,
                            power_draw,
                            power_limit,
                            core_clock,
                            memory_clock,
                            pci_bus,
                            compute_capability,
                        };

                        let _ = tx_clone.send(stats);
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });

        // Initialize container runtime
        let container_runtime = crate::container_runtime::NvContainerRuntime::new().ok();

        // Check for MangoHud
        let mangohud_installed = std::process::Command::new("which")
            .arg("mangohud")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        // Load HDR config
        let hdr_config = crate::hdr::HdrConfig::load().unwrap_or_default();
        let hdr_enabled = config.hdr_enabled;

        // Load power config
        let power_config = crate::power_curves::load_power_config().unwrap_or_default();

        // Load game auto config
        let game_auto_config = crate::game_profile_auto::AutoProfileConfig::default();

        // Save OSD settings before moving config
        let osd_enabled = config.osd_enabled;
        let osd_position = config.osd_position.clone();

        Self {
            tab: Tab::Gpu,
            current_theme: theme,
            gpu_stats: None,
            available_gpus,
            selected_gpu_index: 0,
            selected_gpu_atomic,
            shutdown_signal,
            gpu_stats_rx: rx,
            gpu_stats_tx: tx,
            last_stats_update: std::time::Instant::now(),
            ui_scale: 1.0,
            config,
            app_state,
            toasts: ToastManager::new(),
            overclock_profile: OverclockProfile::default(),
            oc_preset: OcPreset::Stock,
            gpu_offset: 0,
            memory_offset: 0,
            power_limit_percent: 100,
            auto_oc_running: false,
            auto_oc_target: "balanced".to_string(),
            auto_oc_safety: "conservative".to_string(),
            auto_oc_max_temp: 85.0,
            auto_oc_max_power: 100,
            fan_curve: FanCurve::default(),
            fan_mode: FanMode::Auto,
            fan_speeds: HashMap::new(),
            manual_fan_speed: 50,
            voltage_curve: VoltageCurve::default(),
            power_config,
            vibrance_levels,
            selected_icc_profile_idx: 0,
            available_icc_profiles: Vec::new(),
            hdr_enabled,
            hdr_config,
            vrr_enabled: false,
            vrr_displays: crate::vrr::detect_vrr_displays().unwrap_or_default(),
            recording_active: false,
            recording_path: String::new(),
            recording_settings: crate::recording::create_shadowplay_preset(),
            osd_enabled,
            osd_position,
            osd_metrics: vec![
                "fps".to_string(),
                "frametime".to_string(),
                "cpu".to_string(),
                "gpu".to_string(),
            ],
            osd_font_size: 24,
            osd_opacity: 0.8,
            mangohud_installed,
            rgb_mode: "static".to_string(),
            rgb_color: [0.0, 1.0, 0.5], // Default green
            container_runtime,
            containers: Vec::new(),
            game_auto_config,
            temp_history: VecDeque::with_capacity(120),
            util_history: VecDeque::with_capacity(120),
            power_history: VecDeque::with_capacity(120),
            memory_history: VecDeque::with_capacity(120),
            history_max_len: 120,
            monitoring_dashboard: MonitoringDashboard::new(120),
            driver_validation: crate::state::DriverValidationState::load(),
            driver_capabilities: crate::drivers::DriverCapabilities::detect().ok(),
            asus_power_detector,
            asus_power_status: None,
            asus_power_last_update: std::time::Instant::now(),
            latency_mode: "normal".to_string(),
            reflex_enabled: false,
            gamescope_config: None,
            cached_fans: Vec::new(),
            fans_last_update: std::time::Instant::now()
                .checked_sub(std::time::Duration::from_secs(10))
                .unwrap_or_else(std::time::Instant::now), // Force initial refresh
            cached_displays: Vec::new(),
            displays_last_update: std::time::Instant::now()
                .checked_sub(std::time::Duration::from_secs(60))
                .unwrap_or_else(std::time::Instant::now), // Force initial refresh
            cached_icc_profiles: Vec::new(),
            icc_profiles_last_update: std::time::Instant::now()
                .checked_sub(std::time::Duration::from_secs(60))
                .unwrap_or_else(std::time::Instant::now), // Force initial refresh
            cached_nvenc_caps: None,
            nvenc_caps_last_update: std::time::Instant::now()
                .checked_sub(std::time::Duration::from_secs(60))
                .unwrap_or_else(std::time::Instant::now), // Force initial refresh
            cached_is_recording: false,
            recording_status_last_update: std::time::Instant::now()
                .checked_sub(std::time::Duration::from_secs(10))
                .unwrap_or_else(std::time::Instant::now), // Force initial refresh
        }
    }

    /// Create with a specific theme
    pub fn with_theme(theme: ThemeVariant) -> Self {
        let mut state = Self::new();
        state.current_theme = theme;
        state
    }

    /// Get current theme colors
    pub fn theme_colors(&self) -> crate::themes::ColorPalette {
        crate::themes::ColorPalette::from_variant(self.current_theme)
    }

    /// Set selected GPU index (updates background polling thread)
    pub fn set_selected_gpu(&mut self, index: u32) {
        self.selected_gpu_index = index;
        self.selected_gpu_atomic.store(index, Ordering::Relaxed);
        // Clear history when switching GPUs
        self.temp_history.clear();
        self.util_history.clear();
        self.power_history.clear();
        self.memory_history.clear();
    }

    /// Save configuration
    pub fn save_config(&mut self) {
        self.config.theme = self.current_theme.config_key().to_string();
        self.config.vibrance_levels = self.vibrance_levels.clone();
        self.config.hdr_enabled = self.hdr_enabled;
        self.config.osd_enabled = self.osd_enabled;
        self.config.osd_position = self.osd_position.clone();
        self.config.save();
        self.toasts.success("Configuration saved");
    }

    /// Update GPU stats from background channel
    pub fn refresh_gpu_stats(&mut self) {
        // Non-blocking receive from channel
        if let Ok(stats) = self.gpu_stats_rx.try_recv() {
            // Update monitoring dashboard
            let gui_stats = crate::gui_widgets::GpuStats {
                temperature: stats.temperature,
                power_draw: stats.power_draw,
                utilization: stats.utilization,
                fan_speed: stats.fan_speed,
            };
            self.monitoring_dashboard.update(&gui_stats);

            // Update ring buffer histories (O(1) operations)
            if self.temp_history.len() >= self.history_max_len {
                self.temp_history.pop_front();
            }
            self.temp_history.push_back(stats.temperature);

            if self.util_history.len() >= self.history_max_len {
                self.util_history.pop_front();
            }
            self.util_history.push_back(stats.utilization);

            if self.power_history.len() >= self.history_max_len {
                self.power_history.pop_front();
            }
            self.power_history.push_back(stats.power_draw);

            if stats.memory_total > 0 {
                let mem_percent = stats.memory_used as f32 / stats.memory_total as f32 * 100.0;
                if self.memory_history.len() >= self.history_max_len {
                    self.memory_history.pop_front();
                }
                self.memory_history.push_back(mem_percent);
            }

            self.gpu_stats = Some(stats);
            self.last_stats_update = std::time::Instant::now();
        }
    }

    /// Update ASUS Power Monitor+ status
    pub fn refresh_asus_power(&mut self) {
        if self.asus_power_last_update.elapsed() > std::time::Duration::from_secs(2) {
            if let Some(ref detector) = self.asus_power_detector {
                self.asus_power_status = detector.read_power_rails().ok();
                self.asus_power_last_update = std::time::Instant::now();
            }
        }
    }

    /// Refresh cached fan data (rate-limited to avoid blocking main thread)
    pub fn refresh_fans(&mut self) {
        // Only refresh every 1 second to avoid hammering NVML
        if self.fans_last_update.elapsed() > std::time::Duration::from_secs(1) {
            self.cached_fans = crate::fan::list_fans();
            self.fans_last_update = std::time::Instant::now();
        }
    }

    /// Get cached fan list (call refresh_fans() first if data may be stale)
    pub fn get_fans(&self) -> &[FanInfo] {
        &self.cached_fans
    }

    /// Refresh cached display data (rate-limited to avoid subprocess spawns)
    pub fn refresh_displays(&mut self) {
        // Only refresh every 5 seconds - displays rarely change
        if self.displays_last_update.elapsed() > std::time::Duration::from_secs(5) {
            self.cached_displays = crate::display::list_displays();
            self.displays_last_update = std::time::Instant::now();
        }
    }

    /// Get cached display list
    pub fn get_displays(&self) -> &[crate::display::DisplayInfo] {
        &self.cached_displays
    }

    /// Refresh cached ICC profiles (rate-limited to avoid filesystem scans)
    pub fn refresh_icc_profiles(&mut self) {
        // Only refresh every 30 seconds - ICC profiles rarely change
        if self.icc_profiles_last_update.elapsed() > std::time::Duration::from_secs(30) {
            self.cached_icc_profiles = crate::display::list_icc_profiles();
            self.icc_profiles_last_update = std::time::Instant::now();
        }
    }

    /// Get cached ICC profiles
    pub fn get_icc_profiles(&self) -> &[String] {
        &self.cached_icc_profiles
    }

    /// Refresh cached NVENC capabilities (rate-limited - NVML init is expensive)
    pub fn refresh_nvenc_caps(&mut self) {
        // Only refresh every 30 seconds - capabilities don't change
        if self.nvenc_caps_last_update.elapsed() > std::time::Duration::from_secs(30) {
            self.cached_nvenc_caps = crate::recording::get_nvenc_capabilities().ok();
            self.nvenc_caps_last_update = std::time::Instant::now();
        }
    }

    /// Get cached NVENC capabilities
    pub fn get_nvenc_caps(&self) -> Option<&crate::recording::NvencCapabilities> {
        self.cached_nvenc_caps.as_ref()
    }

    /// Refresh cached recording status (rate-limited - file I/O)
    pub fn refresh_recording_status(&mut self) {
        // Only refresh every 1 second
        if self.recording_status_last_update.elapsed() > std::time::Duration::from_secs(1) {
            self.cached_is_recording = crate::recording::is_recording();
            self.recording_status_last_update = std::time::Instant::now();
        }
    }

    /// Get cached recording status
    pub fn is_recording(&self) -> bool {
        self.cached_is_recording
    }

    /// Cycle to the next theme
    pub fn cycle_theme(&mut self) {
        self.current_theme = super::theme::next_theme(self.current_theme);
        self.config.theme = self.current_theme.config_key().to_string();
        self.config.save();
    }

    /// Get theme icon
    pub fn theme_icon(&self) -> &'static str {
        super::theme::theme_icon(self.current_theme)
    }

    /// Apply overclock preset
    pub fn apply_oc_preset(&mut self, preset: OcPreset) {
        self.oc_preset = preset;
        let (gpu, mem, power) = preset.values();
        self.gpu_offset = gpu;
        self.memory_offset = mem;
        self.power_limit_percent = power;
    }

    /// Apply current overclock settings
    pub fn apply_overclock(&self) -> Result<(), String> {
        let profile = OverclockProfile {
            name: "GUI Custom".to_string(),
            gpu_clock_offset: self.gpu_offset,
            memory_clock_offset: self.memory_offset,
            power_limit: self.power_limit_percent as u8,
            voltage_offset: 0,
            temp_limit: 90,
            fan_curve: vec![],
        };

        crate::overclocking::apply_overclock_profile(&profile).map_err(|e| e.to_string())
    }

    /// Reset overclock to stock
    pub fn reset_overclock(&mut self) {
        self.apply_oc_preset(OcPreset::Stock);
        if let Err(e) = self.apply_overclock() {
            self.toasts.error(format!("Failed to reset OC: {}", e));
        } else {
            self.toasts.success("Overclock reset to stock");
        }
    }

    /// Set fan mode
    pub fn set_fan_mode(&mut self, mode: FanMode) {
        self.fan_mode = mode;
        match mode {
            FanMode::Auto => {
                // Reset fan 0 to auto
                if let Err(e) = crate::fan::reset_fan_to_auto(0) {
                    self.toasts.error(format!("Failed to set auto fan: {}", e));
                } else {
                    self.toasts.success("Fan set to automatic control");
                }
            }
            FanMode::Manual => {
                // Will be set with set_manual_fan_speed
            }
            FanMode::Curve => {
                // Apply fan curve points - convert (u32, u32) to (u8, u8)
                let points: Vec<(u8, u8)> = self
                    .fan_curve
                    .to_nvcontrol_format()
                    .iter()
                    .map(|(t, s)| (*t as u8, *s as u8))
                    .collect();
                if let Err(e) = crate::fan::set_fan_curve(0, &points) {
                    self.toasts
                        .error(format!("Failed to apply fan curve: {}", e));
                } else {
                    self.toasts.success("Custom fan curve applied");
                }
            }
        }
    }

    /// Set manual fan speed
    pub fn set_manual_fan_speed(&mut self, speed: u32) {
        self.manual_fan_speed = speed.clamp(0, 100);
        if self.fan_mode == FanMode::Manual {
            if let Err(e) = crate::fan::set_fan_speed(0, self.manual_fan_speed as u8) {
                self.toasts.error(format!("Failed to set fan speed: {}", e));
            }
        }
    }

    /// Shutdown background threads cleanly
    pub fn shutdown(&self) {
        self.shutdown_signal.store(true, Ordering::Relaxed);
    }

    /// Set UI scale (1.0 = normal, 1.5 = 150%, 2.0 = 200%)
    pub fn set_ui_scale(&mut self, scale: f32) {
        self.ui_scale = scale.clamp(0.75, 3.0);
    }

    /// Refresh VRR displays
    pub fn refresh_vrr_displays(&mut self) {
        self.vrr_displays = crate::vrr::detect_vrr_displays().unwrap_or_default();
        // Check if any display has VRR enabled via current_settings
        self.vrr_enabled = self.vrr_displays.iter().any(|d| d.current_settings.enabled);
    }

    /// Apply VRR settings to a display
    pub fn apply_vrr_to_display(&mut self, display_name: &str, enabled: bool) {
        let settings = crate::vrr::VrrSettings {
            enabled,
            min_refresh_rate: 30,
            max_refresh_rate: 144,
            adaptive_sync: true,
            low_framerate_compensation: true,
        };
        match crate::vrr::apply_vrr_settings(display_name, &settings) {
            Ok(_) => {
                self.toasts.success(format!(
                    "VRR {} on {}",
                    if enabled { "enabled" } else { "disabled" },
                    display_name
                ));
                self.refresh_vrr_displays();
            }
            Err(e) => {
                self.toasts.error(format!("Failed to set VRR: {}", e));
            }
        }
    }

    /// Toggle HDR (CLI method)
    pub fn toggle_hdr(&mut self) {
        let new_state = !self.hdr_enabled;
        let result = if new_state {
            crate::hdr::enable_hdr_cli()
        } else {
            crate::hdr::disable_hdr_cli()
        };
        match result {
            Ok(_) => {
                self.hdr_enabled = new_state;
                self.config.hdr_enabled = new_state;
                self.config.save();
                self.toasts.success(format!(
                    "HDR {}",
                    if new_state { "enabled" } else { "disabled" }
                ));
            }
            Err(e) => {
                self.toasts.error(format!("Failed to toggle HDR: {}", e));
            }
        }
    }

    /// Set vibrance for a display
    pub fn set_vibrance(&mut self, display_idx: usize, level: i16) {
        if display_idx < self.vibrance_levels.len() {
            self.vibrance_levels[display_idx] = level.clamp(-100, 100);
            // set_vibrance takes a slice of (display_idx, vibrance_value)
            let values = [(display_idx, level as i32)];
            if let Err(e) = crate::vibrance::set_vibrance(&values) {
                self.toasts.error(format!("Failed to set vibrance: {}", e));
            }
        }
    }

    /// Refresh container list
    pub fn refresh_containers(&mut self) {
        if let Some(ref runtime) = self.container_runtime {
            match runtime.monitor_gpu_containers() {
                Ok(containers) => {
                    self.containers = containers
                        .into_iter()
                        .map(|c: crate::container::ContainerGpuInfo| ContainerInfo {
                            id: c.container_id,
                            name: c.container_name,
                            image: c.image,
                            status: format!("{:?}", c.status),
                            gpu_usage: format!("{:.1}%", c.gpu_utilization),
                        })
                        .collect();
                }
                Err(e) => {
                    self.toasts
                        .error(format!("Failed to list containers: {}", e));
                }
            }
        }
    }
}
