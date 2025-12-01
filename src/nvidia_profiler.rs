/// NVIDIA GPU Profiler
///
/// Equivalent to Radeon GPU Profiler - comprehensive GPU profiling and monitoring
/// Provides detailed telemetry, power metrics, and performance analysis
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

fn instant_now() -> Instant {
    Instant::now()
}

/// GPU profiling data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDataPoint {
    #[serde(skip, default = "instant_now")]
    pub timestamp: Instant,
    pub timestamp_ms: u64, // Milliseconds since session start
    pub gpu_clock_mhz: u32,
    pub memory_clock_mhz: u32,
    pub gpu_voltage_mv: Option<u32>,
    pub memory_voltage_mv: Option<u32>,
    pub temperature_c: i32,
    pub gpu_load_percent: u32,
    pub memory_load_percent: u32,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub fan_speed_percent: u32,
    pub fan_speed_rpm: Option<u32>,
    pub power_draw_watts: f32,
    pub power_limit_watts: u32,
    pub pcie_link_speed: Option<u32>,
    pub pcie_link_width: Option<u32>,
}

/// Profiling session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingSession {
    pub name: String,
    pub gpu_id: u32,
    pub start_time: std::time::SystemTime,
    pub duration: Duration,
    pub sample_interval_ms: u64,
    pub data_points: Vec<ProfileDataPoint>,
}

/// GPU Profiler
pub struct NvidiaProfiler {
    gpu_id: u32,
    sample_interval: Duration,
    max_samples: usize,
    data_buffer: VecDeque<ProfileDataPoint>,
    is_recording: bool,
    session_start: Option<Instant>,
}

impl NvidiaProfiler {
    pub fn new(gpu_id: u32, sample_interval_ms: u64, max_samples: usize) -> Self {
        Self {
            gpu_id,
            sample_interval: Duration::from_millis(sample_interval_ms),
            max_samples,
            data_buffer: VecDeque::with_capacity(max_samples),
            is_recording: false,
            session_start: None,
        }
    }

    /// Start profiling session
    pub fn start_recording(&mut self) {
        self.is_recording = true;
        self.session_start = Some(Instant::now());
        self.data_buffer.clear();
        println!("Started GPU profiling session");
    }

    /// Stop profiling session
    pub fn stop_recording(&mut self) -> Option<ProfilingSession> {
        if !self.is_recording {
            return None;
        }

        self.is_recording = false;

        let duration = self.session_start?.elapsed();
        let data_points: Vec<ProfileDataPoint> = self.data_buffer.iter().cloned().collect();

        Some(ProfilingSession {
            name: format!("GPU {} Profile", self.gpu_id),
            gpu_id: self.gpu_id,
            start_time: std::time::SystemTime::now() - duration,
            duration,
            sample_interval_ms: self.sample_interval.as_millis() as u64,
            data_points,
        })
    }

    /// Sample current GPU state
    pub fn sample(&mut self) -> NvResult<ProfileDataPoint> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let device = nvml
            .device_by_index(self.gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        // Clock speeds
        let gpu_clock = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .unwrap_or(0);
        let mem_clock = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
            .unwrap_or(0);

        // Temperature
        let temp = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0) as i32;

        // Utilization
        let utilization = device.utilization_rates().ok();
        let gpu_load = utilization.as_ref().map(|u| u.gpu).unwrap_or(0);
        let mem_load = utilization.as_ref().map(|u| u.memory).unwrap_or(0);

        // Memory
        let memory_info = device.memory_info().ok();
        let vram_used = memory_info
            .as_ref()
            .map(|m| m.used / 1024 / 1024)
            .unwrap_or(0);
        let vram_total = memory_info
            .as_ref()
            .map(|m| m.total / 1024 / 1024)
            .unwrap_or(0);

        // Fan
        let fan_speed = device.fan_speed(0).unwrap_or(0);

        // Power
        let power_draw = device.power_usage().unwrap_or(0) as f32 / 1000.0; // mW to W
        let power_limit = device.power_management_limit().unwrap_or(0) / 1000;

        // PCIe (these methods may not be available in all nvml_wrapper versions)
        let pcie_link = None; // Placeholder - would use device.curr_pcie_link_generation()
        let pcie_width = None; // Placeholder - would use device.curr_pcie_link_width()

        let timestamp_now = Instant::now();
        let timestamp_ms = if let Some(start) = self.session_start {
            start.elapsed().as_millis() as u64
        } else {
            0
        };

        let data_point = ProfileDataPoint {
            timestamp: timestamp_now,
            timestamp_ms,
            gpu_clock_mhz: gpu_clock,
            memory_clock_mhz: mem_clock,
            gpu_voltage_mv: None, // NVML doesn't expose voltage directly
            memory_voltage_mv: None,
            temperature_c: temp,
            gpu_load_percent: gpu_load,
            memory_load_percent: mem_load,
            vram_used_mb: vram_used,
            vram_total_mb: vram_total,
            fan_speed_percent: fan_speed,
            fan_speed_rpm: None,
            power_draw_watts: power_draw,
            power_limit_watts: power_limit,
            pcie_link_speed: pcie_link,
            pcie_link_width: pcie_width,
        };

        // Add to buffer if recording
        if self.is_recording {
            self.data_buffer.push_back(data_point.clone());

            if self.data_buffer.len() > self.max_samples {
                self.data_buffer.pop_front();
            }
        }

        Ok(data_point)
    }

    /// Export session to JSON
    pub fn export_session(&self, session: &ProfilingSession, path: &str) -> NvResult<()> {
        let json = serde_json::to_string_pretty(session).map_err(|e| {
            NvControlError::RuntimeError(format!("JSON serialization failed: {}", e))
        })?;

        std::fs::write(path, json)
            .map_err(|e| NvControlError::RuntimeError(format!("Failed to write file: {}", e)))?;

        println!("Exported profiling session to: {}", path);

        Ok(())
    }

    /// Get profiling statistics
    pub fn get_statistics(&self) -> Option<ProfileStatistics> {
        if self.data_buffer.is_empty() {
            return None;
        }

        let gpu_clocks: Vec<u32> = self.data_buffer.iter().map(|d| d.gpu_clock_mhz).collect();
        let mem_clocks: Vec<u32> = self
            .data_buffer
            .iter()
            .map(|d| d.memory_clock_mhz)
            .collect();
        let temps: Vec<i32> = self.data_buffer.iter().map(|d| d.temperature_c).collect();
        let gpu_loads: Vec<u32> = self
            .data_buffer
            .iter()
            .map(|d| d.gpu_load_percent)
            .collect();
        let powers: Vec<f32> = self
            .data_buffer
            .iter()
            .map(|d| d.power_draw_watts)
            .collect();

        Some(ProfileStatistics {
            avg_gpu_clock: avg(&gpu_clocks),
            max_gpu_clock: *gpu_clocks.iter().max().unwrap_or(&0),
            min_gpu_clock: *gpu_clocks.iter().min().unwrap_or(&0),
            avg_memory_clock: avg(&mem_clocks),
            avg_temperature: avg_i32(&temps),
            max_temperature: *temps.iter().max().unwrap_or(&0),
            min_temperature: *temps.iter().min().unwrap_or(&0),
            avg_gpu_load: avg(&gpu_loads),
            avg_power_draw: avg_f32(&powers),
            max_power_draw: powers.iter().fold(0.0f32, |a, &b| a.max(b)),
            total_samples: self.data_buffer.len(),
        })
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    pub fn current_samples(&self) -> usize {
        self.data_buffer.len()
    }
}

fn avg(values: &[u32]) -> u32 {
    if values.is_empty() {
        return 0;
    }
    values.iter().sum::<u32>() / values.len() as u32
}

fn avg_i32(values: &[i32]) -> i32 {
    if values.is_empty() {
        return 0;
    }
    values.iter().sum::<i32>() / values.len() as i32
}

fn avg_f32(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f32>() / values.len() as f32
}

/// Profiling statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStatistics {
    pub avg_gpu_clock: u32,
    pub max_gpu_clock: u32,
    pub min_gpu_clock: u32,
    pub avg_memory_clock: u32,
    pub avg_temperature: i32,
    pub max_temperature: i32,
    pub min_temperature: i32,
    pub avg_gpu_load: u32,
    pub avg_power_draw: f32,
    pub max_power_draw: f32,
    pub total_samples: usize,
}

/// Performance capture for specific workload
#[allow(dead_code)]
pub struct WorkloadCapture {
    profiler: NvidiaProfiler,
    workload_name: String,
}

impl WorkloadCapture {
    pub fn new(gpu_id: u32, workload_name: String) -> Self {
        Self {
            profiler: NvidiaProfiler::new(gpu_id, 100, 10000), // 100ms, 10k samples
            workload_name,
        }
    }

    /// Capture workload performance
    pub fn capture<F>(&mut self, workload: F) -> NvResult<ProfilingSession>
    where
        F: FnOnce() -> NvResult<()>,
    {
        self.profiler.start_recording();

        // Run workload
        let workload_result = workload();

        // Wait a bit for metrics to settle
        std::thread::sleep(Duration::from_millis(500));

        let session = self
            .profiler
            .stop_recording()
            .ok_or_else(|| NvControlError::RuntimeError("Failed to capture session".to_string()))?;

        workload_result?;

        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_creation() {
        let profiler = NvidiaProfiler::new(0, 100, 1000);
        assert_eq!(profiler.gpu_id, 0);
        assert!(!profiler.is_recording());
    }

    #[test]
    fn test_start_stop_recording() {
        let mut profiler = NvidiaProfiler::new(0, 100, 1000);

        profiler.start_recording();
        assert!(profiler.is_recording());

        profiler.stop_recording();
        assert!(!profiler.is_recording());
    }

    #[test]
    fn test_statistics() {
        let mut profiler = NvidiaProfiler::new(0, 100, 100);

        // Simulate some data
        for _ in 0..10 {
            if let Ok(_) = profiler.sample() {
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        if let Some(stats) = profiler.get_statistics() {
            println!("Average GPU clock: {} MHz", stats.avg_gpu_clock);
            println!("Average temperature: {}°C", stats.avg_temperature);
            println!("Total samples: {}", stats.total_samples);
        }
    }
}
