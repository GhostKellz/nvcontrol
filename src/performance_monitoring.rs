/// Phase 4.2: Performance Monitoring
///
/// FPS overlay, frame time analysis, 1%/0.1% lows, latency monitoring, performance regression detection
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// FPS counter and frame time tracker
pub struct FpsCounter {
    frame_times: VecDeque<Duration>,
    last_frame: Instant,
    max_samples: usize,
}

impl FpsCounter {
    pub fn new(max_samples: usize) -> Self {
        Self {
            frame_times: VecDeque::with_capacity(max_samples),
            last_frame: Instant::now(),
            max_samples,
        }
    }

    /// Record a frame
    pub fn record_frame(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame);

        self.frame_times.push_back(frame_time);

        if self.frame_times.len() > self.max_samples {
            self.frame_times.pop_front();
        }

        self.last_frame = now;
    }

    /// Get current FPS
    pub fn current_fps(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total_time: Duration = self.frame_times.iter().sum();
        let avg_frame_time = total_time.as_secs_f32() / self.frame_times.len() as f32;

        if avg_frame_time > 0.0 {
            1.0 / avg_frame_time
        } else {
            0.0
        }
    }

    /// Get average frame time in milliseconds
    pub fn avg_frame_time_ms(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total_time: Duration = self.frame_times.iter().sum();
        (total_time.as_secs_f64() / self.frame_times.len() as f64 * 1000.0) as f32
    }

    /// Get 1% low FPS (99th percentile worst frame time)
    pub fn one_percent_low(&self) -> f32 {
        self.percentile_fps(0.99)
    }

    /// Get 0.1% low FPS (99.9th percentile worst frame time)
    pub fn zero_one_percent_low(&self) -> f32 {
        self.percentile_fps(0.999)
    }

    fn percentile_fps(&self, percentile: f64) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let mut sorted: Vec<Duration> = self.frame_times.iter().copied().collect();
        sorted.sort();

        let index = ((sorted.len() as f64 * percentile) as usize).min(sorted.len() - 1);
        let frame_time = sorted[index].as_secs_f64();

        if frame_time > 0.0 {
            (1.0 / frame_time) as f32
        } else {
            0.0
        }
    }

    /// Get frame time statistics
    pub fn get_stats(&self) -> FrameTimeStats {
        if self.frame_times.is_empty() {
            return FrameTimeStats::default();
        }

        let mut sorted: Vec<f32> = self
            .frame_times
            .iter()
            .map(|d| d.as_secs_f32() * 1000.0)
            .collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        FrameTimeStats {
            avg_fps: self.current_fps(),
            avg_frame_time_ms: self.avg_frame_time_ms(),
            one_percent_low: self.one_percent_low(),
            zero_one_percent_low: self.zero_one_percent_low(),
            min_frame_time_ms: sorted[0],
            max_frame_time_ms: sorted[sorted.len() - 1],
            median_frame_time_ms: sorted[sorted.len() / 2],
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FrameTimeStats {
    pub avg_fps: f32,
    pub avg_frame_time_ms: f32,
    pub one_percent_low: f32,
    pub zero_one_percent_low: f32,
    pub min_frame_time_ms: f32,
    pub max_frame_time_ms: f32,
    pub median_frame_time_ms: f32,
}

/// Performance overlay manager (MangoHud integration)
pub struct PerformanceOverlay {
    enabled: bool,
    config: OverlayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayConfig {
    pub fps: bool,
    pub frame_time: bool,
    pub gpu_temp: bool,
    pub gpu_power: bool,
    pub gpu_load: bool,
    pub vram_usage: bool,
    pub cpu_temp: bool,
    pub cpu_load: bool,
    pub position: OverlayPosition,
    pub font_size: u32,
    pub opacity: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OverlayPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            fps: true,
            frame_time: true,
            gpu_temp: true,
            gpu_power: true,
            gpu_load: true,
            vram_usage: true,
            cpu_temp: false,
            cpu_load: false,
            position: OverlayPosition::TopLeft,
            font_size: 24,
            opacity: 0.9,
        }
    }
}

impl PerformanceOverlay {
    pub fn new() -> Self {
        Self {
            enabled: false,
            config: OverlayConfig::default(),
        }
    }

    /// Enable overlay with MangoHud
    pub fn enable(&mut self) -> NvResult<()> {
        self.write_mangohud_config()?;
        self.enabled = true;

        println!("Performance overlay enabled");

        Ok(())
    }

    /// Disable overlay
    pub fn disable(&mut self) -> NvResult<()> {
        self.enabled = false;
        println!("Performance overlay disabled");

        Ok(())
    }

    fn write_mangohud_config(&self) -> NvResult<()> {
        let home = std::env::var("HOME")
            .map_err(|_| NvControlError::ConfigError("HOME not set".to_string()))?;

        let config_dir = std::path::PathBuf::from(home).join(".config/MangoHud");
        std::fs::create_dir_all(&config_dir).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create MangoHud config dir: {}", e))
        })?;

        let config_path = config_dir.join("MangoHud.conf");

        let mut config_lines = Vec::new();

        // Position
        match self.config.position {
            OverlayPosition::TopLeft => config_lines.push("position=top-left".to_string()),
            OverlayPosition::TopRight => config_lines.push("position=top-right".to_string()),
            OverlayPosition::BottomLeft => config_lines.push("position=bottom-left".to_string()),
            OverlayPosition::BottomRight => config_lines.push("position=bottom-right".to_string()),
        }

        // Font size
        config_lines.push(format!("font_size={}", self.config.font_size));

        // Opacity
        config_lines.push(format!("background_alpha={:.2}", self.config.opacity));

        // Metrics
        if self.config.fps {
            config_lines.push("fps".to_string());
        }
        if self.config.frame_time {
            config_lines.push("frametime".to_string());
        }
        if self.config.gpu_temp {
            config_lines.push("gpu_temp".to_string());
        }
        if self.config.gpu_power {
            config_lines.push("gpu_power".to_string());
        }
        if self.config.gpu_load {
            config_lines.push("gpu_stats".to_string());
        }
        if self.config.vram_usage {
            config_lines.push("vram".to_string());
        }
        if self.config.cpu_temp {
            config_lines.push("cpu_temp".to_string());
        }
        if self.config.cpu_load {
            config_lines.push("cpu_stats".to_string());
        }

        let config_content = config_lines.join("\n");

        std::fs::write(&config_path, config_content).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to write MangoHud config: {}", e))
        })?;

        Ok(())
    }

    pub fn set_config(&mut self, config: OverlayConfig) {
        self.config = config;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for PerformanceOverlay {
    fn default() -> Self {
        Self::new()
    }
}

/// Latency monitor (input to display)
pub struct LatencyMonitor {
    samples: VecDeque<Duration>,
    max_samples: usize,
}

impl LatencyMonitor {
    pub fn new(max_samples: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    /// Record latency sample
    pub fn record_latency(&mut self, latency: Duration) {
        self.samples.push_back(latency);

        if self.samples.len() > self.max_samples {
            self.samples.pop_front();
        }
    }

    /// Get average latency
    pub fn avg_latency_ms(&self) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }

        let total: Duration = self.samples.iter().sum();
        (total.as_secs_f64() / self.samples.len() as f64 * 1000.0) as f32
    }

    /// Get percentile latency
    pub fn percentile_latency_ms(&self, percentile: f64) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }

        let mut sorted: Vec<Duration> = self.samples.iter().copied().collect();
        sorted.sort();

        let index = ((sorted.len() as f64 * percentile) as usize).min(sorted.len() - 1);
        (sorted[index].as_secs_f64() * 1000.0) as f32
    }
}

/// Performance regression detector
pub struct RegressionDetector {
    baseline: Option<FrameTimeStats>,
    current_sessions: Vec<SessionStats>,
    threshold_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    pub timestamp: u64,
    pub game: String,
    pub stats: FrameTimeStats,
}

impl RegressionDetector {
    pub fn new(threshold_percent: f32) -> Self {
        Self {
            baseline: None,
            current_sessions: Vec::new(),
            threshold_percent,
        }
    }

    /// Set baseline performance
    pub fn set_baseline(&mut self, stats: FrameTimeStats) {
        self.baseline = Some(stats);
    }

    /// Record session
    pub fn record_session(&mut self, game: String, stats: FrameTimeStats) {
        self.current_sessions.push(SessionStats {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            game,
            stats,
        });
    }

    /// Check for regression
    pub fn check_regression(&self, current: &FrameTimeStats) -> Option<RegressionReport> {
        let baseline = self.baseline.as_ref()?;

        let fps_diff_percent = ((baseline.avg_fps - current.avg_fps) / baseline.avg_fps) * 100.0;
        let frame_time_diff_percent = ((current.avg_frame_time_ms - baseline.avg_frame_time_ms)
            / baseline.avg_frame_time_ms)
            * 100.0;

        if fps_diff_percent > self.threshold_percent {
            Some(RegressionReport {
                severity: RegressionSeverity::Critical,
                fps_drop_percent: fps_diff_percent,
                frame_time_increase_percent: frame_time_diff_percent,
                baseline: baseline.clone(),
                current: current.clone(),
            })
        } else if fps_diff_percent > self.threshold_percent / 2.0 {
            Some(RegressionReport {
                severity: RegressionSeverity::Warning,
                fps_drop_percent: fps_diff_percent,
                frame_time_increase_percent: frame_time_diff_percent,
                baseline: baseline.clone(),
                current: current.clone(),
            })
        } else {
            None
        }
    }

    /// Get session history
    pub fn get_history(&self) -> &[SessionStats] {
        &self.current_sessions
    }
}

#[derive(Debug, Clone)]
pub struct RegressionReport {
    pub severity: RegressionSeverity,
    pub fps_drop_percent: f32,
    pub frame_time_increase_percent: f32,
    pub baseline: FrameTimeStats,
    pub current: FrameTimeStats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegressionSeverity {
    Warning,
    Critical,
}

/// Real-time performance tracker
pub struct PerformanceTracker {
    fps_counter: FpsCounter,
    latency_monitor: LatencyMonitor,
    regression_detector: RegressionDetector,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            fps_counter: FpsCounter::new(1000),
            latency_monitor: LatencyMonitor::new(100),
            regression_detector: RegressionDetector::new(10.0), // 10% threshold
        }
    }

    /// Record frame
    pub fn record_frame(&mut self) {
        self.fps_counter.record_frame();
    }

    /// Record latency
    pub fn record_latency(&mut self, latency: Duration) {
        self.latency_monitor.record_latency(latency);
    }

    /// Get current stats
    pub fn get_stats(&self) -> FrameTimeStats {
        self.fps_counter.get_stats()
    }

    /// Get latency
    pub fn get_avg_latency_ms(&self) -> f32 {
        self.latency_monitor.avg_latency_ms()
    }

    /// Check for regression
    pub fn check_regression(&self) -> Option<RegressionReport> {
        let current_stats = self.get_stats();
        self.regression_detector.check_regression(&current_stats)
    }

    /// Set baseline
    pub fn set_baseline(&mut self) {
        let stats = self.get_stats();
        self.regression_detector.set_baseline(stats);
    }
}

impl Default for PerformanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fps_counter() {
        let mut counter = FpsCounter::new(100);

        // Simulate 60 FPS
        for _ in 0..60 {
            counter.record_frame();
            std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
        }

        let fps = counter.current_fps();
        assert!(fps > 50.0 && fps < 70.0);
    }

    #[test]
    fn test_frame_time_stats() {
        let mut counter = FpsCounter::new(100);

        for _ in 0..100 {
            counter.record_frame();
            std::thread::sleep(Duration::from_millis(16));
        }

        let stats = counter.get_stats();
        assert!(stats.avg_fps > 0.0);
        assert!(stats.one_percent_low > 0.0);
    }

    #[test]
    fn test_latency_monitor() {
        let mut monitor = LatencyMonitor::new(100);

        for i in 0..100 {
            monitor.record_latency(Duration::from_millis(10 + i % 5));
        }

        let avg = monitor.avg_latency_ms();
        assert!(avg > 9.0 && avg < 15.0);
    }

    #[test]
    fn test_regression_detection() {
        let mut detector = RegressionDetector::new(10.0);

        let baseline = FrameTimeStats {
            avg_fps: 100.0,
            avg_frame_time_ms: 10.0,
            one_percent_low: 90.0,
            zero_one_percent_low: 80.0,
            min_frame_time_ms: 8.0,
            max_frame_time_ms: 15.0,
            median_frame_time_ms: 10.0,
        };

        detector.set_baseline(baseline.clone());

        let regressed = FrameTimeStats {
            avg_fps: 80.0, // 20% drop
            ..baseline
        };

        let report = detector.check_regression(&regressed);
        assert!(report.is_some());
        assert_eq!(report.unwrap().severity, RegressionSeverity::Critical);
    }

    #[test]
    fn test_overlay_config() {
        let config = OverlayConfig::default();
        assert!(config.fps);
        assert_eq!(config.font_size, 24);
    }
}
