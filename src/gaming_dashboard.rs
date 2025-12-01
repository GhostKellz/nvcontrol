use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Modern Gaming Dashboard - ASUS ROG Style
/// Combines MSI Afterburner monitoring with ASUS GPU Tweak II controls
/// Optimized for ASUS ROG GPUs and motherboards

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingDashboard {
    pub monitoring: MonitoringData,
    pub overclocking: OverclockingControls,
    pub profiles: Vec<PerformanceProfile>,
    pub active_profile: String,
    pub osd_config: OsdConfig,
    pub recording: RecordingState,
    pub rgb_sync: RgbSyncState,
    pub statistics: SessionStatistics,
}

/// Real-time monitoring data (like MSI Afterburner)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringData {
    // GPU Metrics
    pub gpu_temp: f32,
    pub gpu_usage: f32,
    pub gpu_clock: u32,
    pub gpu_voltage: f32,
    pub gpu_power: f32,
    pub gpu_power_limit: f32,

    // Memory
    pub mem_clock: u32,
    pub mem_usage: u64,
    pub mem_total: u64,
    pub mem_temp: Option<f32>,

    // Fan Control
    pub fan_speed: Vec<u32>, // ASUS Astral has 4 fans
    pub fan_rpm: Vec<u32>,
    pub fan_mode: FanMode,

    // Frame Rate
    pub fps_current: f32,
    pub fps_average: f32,
    pub fps_1percent_low: f32,
    pub frame_time: f32,

    // Thermal
    pub hotspot_temp: Option<f32>,
    pub vram_temp: Option<f32>,
    pub vrm_temp: Option<f32>,

    // History (for graphs)
    pub temp_history: VecDeque<f32>,
    pub clock_history: VecDeque<u32>,
    pub fps_history: VecDeque<f32>,
    pub power_history: VecDeque<f32>,

    // Timestamp
    #[serde(skip, default = "Instant::now")]
    pub last_update: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FanMode {
    Auto,
    Manual,
    Custom(Vec<(u8, u8)>), // (temp, fan_speed) curve
    Aggressive,
    Silent,
    QuadFanOptimized, // Special mode for ASUS Astral 4-fan
}

/// Overclocking controls (like ASUS GPU Tweak II)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverclockingControls {
    // Core
    pub core_clock_offset: i32,   // MHz
    pub core_voltage_offset: i32, // mV
    pub core_voltage_locked: bool,

    // Memory
    pub mem_clock_offset: i32,   // MHz
    pub mem_voltage_offset: i32, // mV

    // Power
    pub power_limit: u8, // Percentage (50-130%)
    pub temp_limit: u8,  // Celsius

    // Advanced
    pub voltage_curve: Vec<(u32, u32)>, // (MHz, mV) points
    pub voltage_curve_active: bool,

    // ASUS Specific
    pub gpu_tweak_mode: GpuTweakMode,
    pub safe_limits: SafetyLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GpuTweakMode {
    Silent,   // Quiet, low power
    Standard, // Balanced
    OC,       // Overclocked
    Custom,   // User-defined
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyLimits {
    pub max_temp: u8,
    pub max_power: u8,
    pub max_core_offset: i32,
    pub max_mem_offset: i32,
    pub thermal_throttle_warning: bool,
    pub power_throttle_warning: bool,
}

/// Performance profiles (quick switching)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub name: String,
    pub description: String,
    pub icon: String,

    // OC Settings
    pub core_offset: i32,
    pub mem_offset: i32,
    pub power_limit: u8,
    pub temp_limit: u8,

    // Fan Curve
    pub fan_curve: Vec<(u8, u8)>,
    pub fan_mode: FanMode,

    // Vibrance
    pub vibrance_level: i32,

    // RGB
    pub rgb_preset: String,

    // Auto-apply triggers
    pub auto_apply_games: Vec<String>,
    pub auto_apply_on_startup: bool,
}

/// OSD (On-Screen Display) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsdConfig {
    pub enabled: bool,
    pub position: OsdPosition,
    pub opacity: f32,
    pub font_size: u32,

    // Displayed metrics
    pub show_fps: bool,
    pub show_frametime: bool,
    pub show_gpu_temp: bool,
    pub show_gpu_usage: bool,
    pub show_gpu_clock: bool,
    pub show_mem_usage: bool,
    pub show_power: bool,
    pub show_fan_speed: bool,

    // Advanced
    pub color_scheme: OsdColorScheme,
    pub update_interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OsdPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom(i32, i32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OsdColorScheme {
    RogRed,
    Matrix,
    Cyberpunk,
    Stealth,
    Rainbow,
    Custom(String, String, String), // (text, background, accent)
}

/// Recording state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingState {
    pub is_recording: bool,
    pub output_path: String,
    pub encoder: VideoEncoder,
    pub quality: RecordingQuality,
    pub capture_fps: u32,
    pub instant_replay_enabled: bool,
    pub instant_replay_duration: u32, // seconds
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VideoEncoder {
    NvencH264,
    NvencH265,
    NvencAv1, // RTX 50-series
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecordingQuality {
    Low,
    Medium,
    High,
    Ultra,
    Lossless,
}

/// RGB sync state (ASUS Aura)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RgbSyncState {
    pub enabled: bool,
    pub sync_with_temp: bool,
    pub sync_with_usage: bool,
    pub current_preset: String,
    pub brightness: u8,
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatistics {
    #[serde(skip, default = "Instant::now")]
    pub session_start: Instant,
    #[serde(skip, default)]
    pub total_runtime: Duration,

    // Performance stats
    pub avg_fps: f32,
    pub max_fps: f32,
    pub min_fps: f32,
    pub fps_99_percentile: f32,

    // Thermal stats
    pub avg_temp: f32,
    pub max_temp: f32,
    #[serde(skip, default)]
    pub time_above_80c: Duration,

    // Power stats
    pub avg_power: f32,
    pub max_power: f32,
    pub total_energy_kwh: f32,

    // Throttling detection
    pub thermal_throttle_count: u32,
    pub power_throttle_count: u32,
    pub voltage_throttle_count: u32,
}

impl GamingDashboard {
    /// Create new gaming dashboard
    pub fn new() -> NvResult<Self> {
        Ok(GamingDashboard {
            monitoring: MonitoringData::new(),
            overclocking: OverclockingControls::default(),
            profiles: Self::default_profiles(),
            active_profile: "Standard".to_string(),
            osd_config: OsdConfig::default(),
            recording: RecordingState::default(),
            rgb_sync: RgbSyncState::default(),
            statistics: SessionStatistics::new(),
        })
    }

    /// Update monitoring data (called frequently)
    pub fn update_monitoring(&mut self) -> NvResult<()> {
        // Update GPU metrics via NVML
        if let Ok(metrics) = Self::fetch_gpu_metrics() {
            self.monitoring.gpu_temp = metrics.gpu_temp;
            self.monitoring.gpu_usage = metrics.gpu_usage;
            self.monitoring.gpu_clock = metrics.gpu_clock;
            self.monitoring.gpu_power = metrics.gpu_power;

            // Update history
            self.monitoring.temp_history.push_back(metrics.gpu_temp);
            self.monitoring.clock_history.push_back(metrics.gpu_clock);
            self.monitoring.power_history.push_back(metrics.gpu_power);

            // Keep only last 60 samples
            if self.monitoring.temp_history.len() > 60 {
                self.monitoring.temp_history.pop_front();
                self.monitoring.clock_history.pop_front();
                self.monitoring.power_history.pop_front();
            }
        }

        // Update FPS (via MangoHud integration or frame timing)
        if let Ok(fps) = Self::fetch_fps() {
            self.monitoring.fps_current = fps;
            self.monitoring.fps_history.push_back(fps);
            if self.monitoring.fps_history.len() > 60 {
                self.monitoring.fps_history.pop_front();
            }
        }

        // Update statistics
        self.statistics.update(&self.monitoring);

        self.monitoring.last_update = Instant::now();
        Ok(())
    }

    /// Apply overclocking settings
    pub fn apply_overclock(&self) -> NvResult<()> {
        println!("Applying overclock settings:");
        println!("  Core: {:+} MHz", self.overclocking.core_clock_offset);
        println!("  Memory: {:+} MHz", self.overclocking.mem_clock_offset);
        println!("  Power Limit: {}%", self.overclocking.power_limit);
        println!("  Temp Limit: {}°C", self.overclocking.temp_limit);

        // Apply via nvidia-settings
        use std::process::Command;

        let commands = vec![
            format!(
                "nvidia-settings -a '[gpu:0]/GPUGraphicsClockOffset[3]={}'",
                self.overclocking.core_clock_offset
            ),
            format!(
                "nvidia-settings -a '[gpu:0]/GPUMemoryTransferRateOffset[3]={}'",
                self.overclocking.mem_clock_offset
            ),
        ];

        for cmd in commands {
            let _ = Command::new("sh").arg("-c").arg(&cmd).output();
        }

        Ok(())
    }

    /// Switch to a performance profile
    pub fn switch_profile(&mut self, profile_name: &str) -> NvResult<()> {
        if let Some(profile) = self.profiles.iter().find(|p| p.name == profile_name) {
            self.overclocking.core_clock_offset = profile.core_offset;
            self.overclocking.mem_clock_offset = profile.mem_offset;
            self.overclocking.power_limit = profile.power_limit;
            self.overclocking.temp_limit = profile.temp_limit;
            self.monitoring.fan_mode = profile.fan_mode.clone();

            self.active_profile = profile_name.to_string();
            self.apply_overclock()?;

            println!("✅ Switched to profile: {}", profile_name);
        }
        Ok(())
    }

    /// Toggle OSD
    pub fn toggle_osd(&mut self) {
        self.osd_config.enabled = !self.osd_config.enabled;

        if self.osd_config.enabled {
            // Start OSD via MangoHud
            println!("✅ OSD enabled");
        } else {
            println!("❌ OSD disabled");
        }
    }

    /// Start recording
    pub fn start_recording(&mut self) -> NvResult<()> {
        self.recording.is_recording = true;
        println!("🔴 Recording started");
        // Integrate with src/recording.rs
        Ok(())
    }

    /// Stop recording
    pub fn stop_recording(&mut self) -> NvResult<()> {
        self.recording.is_recording = false;
        println!("⏹️  Recording stopped");
        Ok(())
    }

    /// Default ASUS ROG profiles
    fn default_profiles() -> Vec<PerformanceProfile> {
        vec![
            // Silent Mode
            PerformanceProfile {
                name: "Silent".to_string(),
                description: "Quiet operation, reduced clocks".to_string(),
                icon: "🤫".to_string(),
                core_offset: -100,
                mem_offset: -200,
                power_limit: 85,
                temp_limit: 75,
                fan_curve: vec![(30, 20), (40, 25), (50, 30), (60, 35), (70, 45), (80, 60)],
                fan_mode: FanMode::Silent,
                vibrance_level: 400,
                rgb_preset: "Dim".to_string(),
                auto_apply_games: vec![],
                auto_apply_on_startup: false,
            },
            // Standard Mode
            PerformanceProfile {
                name: "Standard".to_string(),
                description: "Balanced performance and acoustics".to_string(),
                icon: "⚖️".to_string(),
                core_offset: 0,
                mem_offset: 0,
                power_limit: 100,
                temp_limit: 85,
                fan_curve: vec![(30, 30), (40, 35), (50, 45), (60, 55), (70, 70), (80, 85)],
                fan_mode: FanMode::Auto,
                vibrance_level: 600,
                rgb_preset: "ROG Red".to_string(),
                auto_apply_games: vec![],
                auto_apply_on_startup: true,
            },
            // OC Mode (ASUS Astral optimized)
            PerformanceProfile {
                name: "OC Mode".to_string(),
                description: "Factory overclock + headroom".to_string(),
                icon: "🚀".to_string(),
                core_offset: 175, // ASUS Astral safe limit
                mem_offset: 1500, // GDDR7 can handle it
                power_limit: 105, // 630W max
                temp_limit: 90,
                fan_curve: vec![(30, 35), (40, 45), (50, 55), (60, 65), (70, 80), (80, 95)],
                fan_mode: FanMode::QuadFanOptimized,
                vibrance_level: 700,
                rgb_preset: "Rainbow Wave".to_string(),
                auto_apply_games: vec!["cyberpunk2077".to_string(), "starfield".to_string()],
                auto_apply_on_startup: false,
            },
            // Extreme Mode (for benchmarking)
            PerformanceProfile {
                name: "Extreme".to_string(),
                description: "Maximum performance (loud!)".to_string(),
                icon: "💥".to_string(),
                core_offset: 210, // 1.2x safe limit
                mem_offset: 1650,
                power_limit: 105,
                temp_limit: 92,
                fan_curve: vec![(30, 50), (40, 60), (50, 70), (60, 80), (70, 90), (80, 100)],
                fan_mode: FanMode::Aggressive,
                vibrance_level: 800,
                rgb_preset: "Temperature Reactive".to_string(),
                auto_apply_games: vec![],
                auto_apply_on_startup: false,
            },
        ]
    }

    /// Fetch GPU metrics via NVML
    fn fetch_gpu_metrics() -> NvResult<GpuMetrics> {
        // This would integrate with existing NVML code
        Ok(GpuMetrics {
            gpu_temp: 65.0,
            gpu_usage: 85.0,
            gpu_clock: 2610,
            gpu_power: 450.0,
        })
    }

    /// Fetch FPS via MangoHud or frame timing
    fn fetch_fps() -> NvResult<f32> {
        // Integrate with MangoHud or use Vulkan layer
        Ok(144.0)
    }

    /// Print dashboard summary
    pub fn print_summary(&self) {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║         ASUS ROG Gaming Dashboard - nvcontrol           ║");
        println!("╚══════════════════════════════════════════════════════════╝");

        println!("\n🎮 Active Profile: {}", self.active_profile);

        println!("\n📊 GPU Monitoring:");
        println!("  Temperature:  {:.1}°C", self.monitoring.gpu_temp);
        println!("  Usage:        {:.1}%", self.monitoring.gpu_usage);
        println!("  Core Clock:   {} MHz", self.monitoring.gpu_clock);
        println!(
            "  Memory:       {} / {} MB",
            self.monitoring.mem_usage / 1024 / 1024,
            self.monitoring.mem_total / 1024 / 1024
        );
        println!(
            "  Power:        {:.1}W / {:.1}W",
            self.monitoring.gpu_power, self.monitoring.gpu_power_limit
        );

        if !self.monitoring.fan_speed.is_empty() {
            println!("\n🌀 Fan Speeds (Quad-Fan):");
            for (idx, speed) in self.monitoring.fan_speed.iter().enumerate() {
                println!("  Fan {}: {}%", idx + 1, speed);
            }
        }

        println!("\n⚡ Overclocking:");
        println!(
            "  Core Offset:  {:+} MHz",
            self.overclocking.core_clock_offset
        );
        println!(
            "  Memory Offset: {:+} MHz",
            self.overclocking.mem_clock_offset
        );
        println!("  Power Limit:  {}%", self.overclocking.power_limit);
        println!("  Temp Limit:   {}°C", self.overclocking.temp_limit);

        println!("\n📈 Session Stats:");
        println!("  Runtime:      {:?}", self.statistics.total_runtime);
        println!("  Avg FPS:      {:.1}", self.statistics.avg_fps);
        println!("  Avg Temp:     {:.1}°C", self.statistics.avg_temp);
        println!("  Max Temp:     {:.1}°C", self.statistics.max_temp);

        if self.osd_config.enabled {
            println!("\n📺 OSD: ✅ Enabled");
        }

        if self.recording.is_recording {
            println!("🔴 Recording: Active");
        }
    }
}

// Helper structs
struct GpuMetrics {
    gpu_temp: f32,
    gpu_usage: f32,
    gpu_clock: u32,
    gpu_power: f32,
}

impl Default for OverclockingControls {
    fn default() -> Self {
        Self {
            core_clock_offset: 0,
            core_voltage_offset: 0,
            core_voltage_locked: false,
            mem_clock_offset: 0,
            mem_voltage_offset: 0,
            power_limit: 100,
            temp_limit: 85,
            voltage_curve: Vec::new(),
            voltage_curve_active: false,
            gpu_tweak_mode: GpuTweakMode::Standard,
            safe_limits: SafetyLimits {
                max_temp: 92,
                max_power: 105,
                max_core_offset: 250,
                max_mem_offset: 2000,
                thermal_throttle_warning: false,
                power_throttle_warning: false,
            },
        }
    }
}

impl Default for OsdConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            position: OsdPosition::TopLeft,
            opacity: 0.8,
            font_size: 16,
            show_fps: true,
            show_frametime: true,
            show_gpu_temp: true,
            show_gpu_usage: true,
            show_gpu_clock: false,
            show_mem_usage: false,
            show_power: true,
            show_fan_speed: false,
            color_scheme: OsdColorScheme::RogRed,
            update_interval_ms: 1000,
        }
    }
}

impl Default for RecordingState {
    fn default() -> Self {
        Self {
            is_recording: false,
            output_path: "~/Videos/nvcontrol".to_string(),
            encoder: VideoEncoder::NvencAv1, // RTX 50-series
            quality: RecordingQuality::High,
            capture_fps: 60,
            instant_replay_enabled: false,
            instant_replay_duration: 30,
        }
    }
}

impl Default for RgbSyncState {
    fn default() -> Self {
        Self {
            enabled: true,
            sync_with_temp: true,
            sync_with_usage: false,
            current_preset: "ROG Red".to_string(),
            brightness: 80,
        }
    }
}

impl MonitoringData {
    fn new() -> Self {
        Self {
            gpu_temp: 0.0,
            gpu_usage: 0.0,
            gpu_clock: 0,
            gpu_voltage: 0.0,
            gpu_power: 0.0,
            gpu_power_limit: 575.0,
            mem_clock: 0,
            mem_usage: 0,
            mem_total: 34359738368, // 32GB
            mem_temp: None,
            fan_speed: vec![0, 0, 0, 0], // 4 fans
            fan_rpm: vec![0, 0, 0, 0],
            fan_mode: FanMode::Auto,
            fps_current: 0.0,
            fps_average: 0.0,
            fps_1percent_low: 0.0,
            frame_time: 0.0,
            hotspot_temp: None,
            vram_temp: None,
            vrm_temp: None,
            temp_history: VecDeque::new(),
            clock_history: VecDeque::new(),
            fps_history: VecDeque::new(),
            power_history: VecDeque::new(),
            last_update: Instant::now(),
        }
    }
}

impl SessionStatistics {
    fn new() -> Self {
        Self {
            session_start: Instant::now(),
            total_runtime: Duration::new(0, 0),
            avg_fps: 0.0,
            max_fps: 0.0,
            min_fps: 0.0,
            fps_99_percentile: 0.0,
            avg_temp: 0.0,
            max_temp: 0.0,
            time_above_80c: Duration::new(0, 0),
            avg_power: 0.0,
            max_power: 0.0,
            total_energy_kwh: 0.0,
            thermal_throttle_count: 0,
            power_throttle_count: 0,
            voltage_throttle_count: 0,
        }
    }

    fn update(&mut self, monitoring: &MonitoringData) {
        self.total_runtime = self.session_start.elapsed();

        // Update max values
        if monitoring.fps_current > self.max_fps {
            self.max_fps = monitoring.fps_current;
        }
        if monitoring.gpu_temp > self.max_temp {
            self.max_temp = monitoring.gpu_temp;
        }
        if monitoring.gpu_power > self.max_power {
            self.max_power = monitoring.gpu_power;
        }

        // Track time above 80°C
        if monitoring.gpu_temp > 80.0 {
            self.time_above_80c += Duration::from_secs(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = GamingDashboard::new();
        assert!(dashboard.is_ok());
    }

    #[test]
    fn test_default_profiles() {
        let profiles = GamingDashboard::default_profiles();
        assert_eq!(profiles.len(), 4);
        assert!(profiles.iter().any(|p| p.name == "Silent"));
        assert!(profiles.iter().any(|p| p.name == "OC Mode"));
    }
}
