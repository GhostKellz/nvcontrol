use crate::fan;
use crate::nvml_backend::SharedNvmlBackend;
#[cfg(feature = "tray")]
use std::sync::mpsc;
#[cfg(feature = "tray")]
use std::time::{Duration, Instant};
#[cfg(feature = "tray")]
use tray_icon::{
    TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem},
};

#[derive(Debug, Clone)]
pub enum TrayEvent {
    ShowGui,
    ShowTui,
    QuickVibrance(i16),
    ToggleVrr,
    ToggleGamingMode,
    FanProfile(String),
    LatencyMode(String),
    Exit,
}

#[derive(Debug, Clone)]
pub struct GpuStats {
    pub name: String,
    pub temperature: u32,
    pub utilization: u32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub power_draw: f32,
    pub fan_speed: u32,
    pub fan_health: fan::FanHealthStatus,
}

#[cfg(feature = "tray")]
pub struct SystemTray {
    tray_icon: TrayIcon,
    event_receiver: mpsc::Receiver<TrayEvent>,
    last_update: Instant,
    gaming_mode_enabled: bool,
    vrr_enabled: bool,
    backend: SharedNvmlBackend,
}

#[cfg(feature = "tray")]
impl SystemTray {
    /// Create tray with shared backend (preferred)
    pub fn with_backend(backend: SharedNvmlBackend) -> Result<Self, Box<dyn std::error::Error>> {
        let (_sender, receiver) = mpsc::channel();

        // Create simplified menu for now
        let menu = Menu::new();
        let _show_gui_item = MenuItem::new("🎮 Show GUI", true, None);
        let _show_tui_item = MenuItem::new("📊 Show TUI Monitor", true, None);
        let _vrr_toggle = MenuItem::new("🔄 Toggle VRR", true, None);
        let _gaming_toggle = MenuItem::new("🎯 Gaming Mode", true, None);
        let _exit_item = MenuItem::new("❌ Exit", true, None);

        // Get initial GPU stats for tooltip using backend
        let initial_tooltip = Self::generate_tooltip_with_backend(&backend);

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip(&initial_tooltip)
            .build()?;

        Ok(Self {
            tray_icon,
            event_receiver: receiver,
            last_update: Instant::now(),
            gaming_mode_enabled: false,
            vrr_enabled: false,
            backend,
        })
    }

    /// Create tray (legacy - creates own backend)
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_backend(crate::nvml_backend::create_real_backend())
    }

    pub fn try_recv(&self) -> Option<TrayEvent> {
        self.event_receiver.try_recv().ok()
    }

    /// Update tooltip with current GPU stats (call periodically)
    pub fn update_tooltip(&mut self) {
        // Update every 5 seconds to avoid excessive polling
        if self.last_update.elapsed() > Duration::from_secs(5) {
            let tooltip = Self::generate_tooltip_with_backend(&self.backend);
            let _ = self.tray_icon.set_tooltip(Some(&tooltip));
            self.last_update = Instant::now();
        }
    }

    /// Generate rich tooltip with current GPU stats using backend
    fn generate_tooltip_with_backend(backend: &SharedNvmlBackend) -> String {
        match get_gpu_stats_with_backend(backend) {
            Ok(stats) => {
                format!(
                    "nvcontrol - NVIDIA GPU Control\n\
                    \n\
                    🎯 GPU: {}\n\
                    🌡️  Temp: {}°C\n\
                    📈 Usage: {}%\n\
                    💾 VRAM: {:.1}GB / {:.1}GB\n\
                    ⚡ Power: {:.1}W\n\
                    🌀 Fan: {} RPM ({})\n\
                    \n\
                    🎮 Gaming Mode: {}\n\
                    🔄 VRR: {}\n\
                    \n\
                    Right-click for controls",
                    stats.name,
                    stats.temperature,
                    stats.utilization,
                    stats.memory_used as f64 / 1024.0 / 1024.0 / 1024.0,
                    stats.memory_total as f64 / 1024.0 / 1024.0 / 1024.0,
                    stats.power_draw,
                    stats.fan_speed,
                    format_health_status(&stats.fan_health),
                    "Unknown", // Gaming mode state would require config file
                    "Unknown"  // VRR state would require display query
                )
            }
            Err(_) => "nvcontrol - NVIDIA GPU Control\n\
                \n\
                ⚠️  Unable to get GPU stats\n\
                \n\
                Right-click for controls"
                .to_string(),
        }
    }

    pub fn set_gaming_mode(&mut self, enabled: bool) {
        self.gaming_mode_enabled = enabled;
    }

    pub fn set_vrr_enabled(&mut self, enabled: bool) {
        self.vrr_enabled = enabled;
    }
}

#[cfg(not(feature = "tray"))]
pub struct SystemTray;

#[cfg(not(feature = "tray"))]
impl SystemTray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(SystemTray)
    }

    pub fn try_recv(&self) -> Option<TrayEvent> {
        None
    }

    pub fn update_tooltip(&mut self) {
        // No-op for non-tray builds
    }

    pub fn set_gaming_mode(&mut self, _enabled: bool) {
        // No-op for non-tray builds
    }

    pub fn set_vrr_enabled(&mut self, _enabled: bool) {
        // No-op for non-tray builds
    }
}

/// Get current GPU statistics for tray display using backend
#[allow(dead_code)]
fn get_gpu_stats_with_backend(
    backend: &SharedNvmlBackend,
) -> Result<GpuStats, Box<dyn std::error::Error>> {
    let name = backend
        .get_name(0)
        .unwrap_or_else(|_| "Unknown GPU".to_string());
    let temperature = backend.get_temperature(0).unwrap_or(0);
    let (gpu_util, _mem_util) = backend.get_utilization(0).unwrap_or((0, 0));
    let (memory_used, memory_total) = backend.get_memory_info(0).unwrap_or((0, 0));
    let power_draw = backend
        .get_power_usage(0)
        .map(|p| p as f32 / 1000.0)
        .unwrap_or(0.0);

    // Get fan info using backend
    let fans = fan::list_fans_with_backend(backend);
    let (fan_speed, fan_health) = if let Some(fan) = fans.first() {
        (fan.rpm.unwrap_or(0), fan.health_status.clone())
    } else {
        (0, fan::FanHealthStatus::Unknown)
    };

    Ok(GpuStats {
        name,
        temperature,
        utilization: gpu_util,
        memory_used,
        memory_total,
        power_draw,
        fan_speed,
        fan_health,
    })
}

/// Get current GPU statistics for tray display (legacy - creates own backend)
#[allow(dead_code)]
fn get_gpu_stats() -> Result<GpuStats, Box<dyn std::error::Error>> {
    let backend = crate::nvml_backend::create_real_backend();
    get_gpu_stats_with_backend(&backend)
}

#[allow(dead_code)]
fn format_health_status(status: &fan::FanHealthStatus) -> &'static str {
    match status {
        fan::FanHealthStatus::Healthy => "OK",
        fan::FanHealthStatus::Warning => "WARN",
        fan::FanHealthStatus::Critical => "CRIT",
        fan::FanHealthStatus::Unknown => "?",
    }
}
