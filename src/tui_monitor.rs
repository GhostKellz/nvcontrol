/// TUI Live Monitor
///
/// Real-time terminal UI for monitoring GPU speeds and feeds
/// Similar to nvidia-smi but with live graphs and detailed metrics

use crate::{NvControlError, NvResult};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// TUI Monitor data
#[derive(Debug, Clone)]
pub struct MonitorData {
    pub gpu_id: u32,
    pub gpu_name: String,
    pub driver_version: String,

    // Current values
    pub gpu_clock_mhz: u32,
    pub memory_clock_mhz: u32,
    pub temperature_c: i32,
    pub gpu_load_percent: u32,
    pub memory_load_percent: u32,
    pub vram_used_mb: u64,
    pub vram_total_mb: u64,
    pub fan_speed_percent: u32,
    pub power_draw_watts: f32,
    pub power_limit_watts: u32,

    // History for graphs (last 60 samples)
    pub temp_history: VecDeque<i32>,
    pub load_history: VecDeque<u32>,
    pub power_history: VecDeque<f32>,
    pub clock_history: VecDeque<u32>,
}

impl MonitorData {
    pub fn new(gpu_id: u32) -> NvResult<Self> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let device = nvml.device_by_index(gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        let gpu_name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());
        let driver_version = nvml.sys_driver_version().unwrap_or_else(|_| "Unknown".to_string());

        Ok(Self {
            gpu_id,
            gpu_name,
            driver_version,
            gpu_clock_mhz: 0,
            memory_clock_mhz: 0,
            temperature_c: 0,
            gpu_load_percent: 0,
            memory_load_percent: 0,
            vram_used_mb: 0,
            vram_total_mb: 0,
            fan_speed_percent: 0,
            power_draw_watts: 0.0,
            power_limit_watts: 0,
            temp_history: VecDeque::with_capacity(60),
            load_history: VecDeque::with_capacity(60),
            power_history: VecDeque::with_capacity(60),
            clock_history: VecDeque::with_capacity(60),
        })
    }

    /// Update monitor data
    pub fn update(&mut self) -> NvResult<()> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let device = nvml.device_by_index(self.gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        // Clocks
        self.gpu_clock_mhz = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .unwrap_or(0);
        self.memory_clock_mhz = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
            .unwrap_or(0);

        // Temperature
        self.temperature_c = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0) as i32;

        // Utilization
        if let Ok(util) = device.utilization_rates() {
            self.gpu_load_percent = util.gpu;
            self.memory_load_percent = util.memory;
        }

        // Memory
        if let Ok(mem_info) = device.memory_info() {
            self.vram_used_mb = mem_info.used / 1024 / 1024;
            self.vram_total_mb = mem_info.total / 1024 / 1024;
        }

        // Fan
        self.fan_speed_percent = device.fan_speed(0).unwrap_or(0);

        // Power
        self.power_draw_watts = device.power_usage().unwrap_or(0) as f32 / 1000.0;
        self.power_limit_watts = device.power_management_limit().unwrap_or(0) / 1000;

        // Update history (store values to avoid borrow conflicts)
        let temp = self.temperature_c;
        let load = self.gpu_load_percent;
        let power = self.power_draw_watts;
        let clock = self.gpu_clock_mhz;

        Self::add_to_history(temp, &mut self.temp_history);
        Self::add_to_history(load, &mut self.load_history);
        Self::add_to_history(power, &mut self.power_history);
        Self::add_to_history(clock, &mut self.clock_history);

        Ok(())
    }

    fn add_to_history<T>(value: T, history: &mut VecDeque<T>) {
        history.push_back(value);
        if history.len() > 60 {
            history.pop_front();
        }
    }

    /// Render as ASCII art (for TUI)
    pub fn render_ascii(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!("\n╔════════════════════════════════════════════════════════════════╗\n"));
        output.push_str(&format!("║  NVIDIA GPU Monitor - {} (GPU {})  ║\n", self.gpu_name, self.gpu_id));
        output.push_str(&format!("║  Driver: {}                                        ║\n", self.driver_version));
        output.push_str(&format!("╚════════════════════════════════════════════════════════════════╝\n\n"));

        // Current stats
        output.push_str(&format!("┌─ GPU Clocks ────────────────────────────────────────────────┐\n"));
        output.push_str(&format!("│  GPU Clock:    {:>6} MHz  {:>40} │\n", self.gpu_clock_mhz, self.render_bar(self.gpu_clock_mhz, 3000)));
        output.push_str(&format!("│  Memory Clock: {:>6} MHz  {:>40} │\n", self.memory_clock_mhz, self.render_bar(self.memory_clock_mhz, 12000)));
        output.push_str(&format!("└─────────────────────────────────────────────────────────────┘\n\n"));

        output.push_str(&format!("┌─ Temperature & Fan ─────────────────────────────────────────┐\n"));
        output.push_str(&format!("│  Temperature:  {:>3}°C      {:>40} │\n", self.temperature_c, self.render_bar(self.temperature_c as u32, 100)));
        output.push_str(&format!("│  Fan Speed:    {:>3}%       {:>40} │\n", self.fan_speed_percent, self.render_bar(self.fan_speed_percent, 100)));
        output.push_str(&format!("└─────────────────────────────────────────────────────────────┘\n\n"));

        output.push_str(&format!("┌─ Utilization ───────────────────────────────────────────────┐\n"));
        output.push_str(&format!("│  GPU Load:     {:>3}%       {:>40} │\n", self.gpu_load_percent, self.render_bar(self.gpu_load_percent, 100)));
        output.push_str(&format!("│  Memory Load:  {:>3}%       {:>40} │\n", self.memory_load_percent, self.render_bar(self.memory_load_percent, 100)));
        output.push_str(&format!("└─────────────────────────────────────────────────────────────┘\n\n"));

        output.push_str(&format!("┌─ Memory ────────────────────────────────────────────────────┐\n"));
        output.push_str(&format!("│  VRAM Used:    {:>5} MB / {:>5} MB ({:>3}%)           │\n",
            self.vram_used_mb,
            self.vram_total_mb,
            (self.vram_used_mb as f32 / self.vram_total_mb as f32 * 100.0) as u32
        ));
        output.push_str(&format!("│  {:>58} │\n", self.render_bar(self.vram_used_mb as u32, self.vram_total_mb as u32)));
        output.push_str(&format!("└─────────────────────────────────────────────────────────────┘\n\n"));

        output.push_str(&format!("┌─ Power ─────────────────────────────────────────────────────┐\n"));
        output.push_str(&format!("│  Power Draw:   {:>6.1} W / {:>4} W ({:>3}%)         │\n",
            self.power_draw_watts,
            self.power_limit_watts,
            (self.power_draw_watts / self.power_limit_watts as f32 * 100.0) as u32
        ));
        output.push_str(&format!("│  {:>58} │\n", self.render_bar(self.power_draw_watts as u32, self.power_limit_watts)));
        output.push_str(&format!("└─────────────────────────────────────────────────────────────┘\n\n"));

        // Temperature graph
        output.push_str(&format!("┌─ Temperature History (last 60s) ────────────────────────────┐\n"));
        output.push_str(&self.render_graph(&self.temp_history, 0, 100));
        output.push_str(&format!("└─────────────────────────────────────────────────────────────┘\n\n"));

        output.push_str(&format!("Press Ctrl+C to exit\n"));

        output
    }

    fn render_bar(&self, value: u32, max: u32) -> String {
        let bar_width = 40;
        let filled = ((value as f32 / max as f32) * bar_width as f32) as usize;
        let filled = filled.min(bar_width);

        let mut bar = String::from("[");
        for i in 0..bar_width {
            if i < filled {
                bar.push('█');
            } else {
                bar.push('░');
            }
        }
        bar.push(']');
        bar
    }

    fn render_graph<T: Clone + Into<i32>>(&self, history: &VecDeque<T>, min: i32, max: i32) -> String {
        let height = 8;
        let width = 60;

        if history.is_empty() {
            return format!("│{:^60}│\n", "No data yet...");
        }

        let mut output = String::new();

        for row in (0..height).rev() {
            output.push_str("│ ");

            let threshold = min + ((max - min) * row / height);

            for value in history.iter() {
                let val: i32 = (*value).clone().into();
                if val >= threshold {
                    output.push('█');
                } else {
                    output.push(' ');
                }
            }

            // Padding
            for _ in history.len()..width {
                output.push(' ');
            }

            output.push_str(&format!(" │ {:>3}\n", threshold));
        }

        output
    }
}

/// TUI Monitor runner
pub struct TuiMonitor {
    data: MonitorData,
    refresh_rate: Duration,
}

impl TuiMonitor {
    pub fn new(gpu_id: u32, refresh_hz: u32) -> NvResult<Self> {
        Ok(Self {
            data: MonitorData::new(gpu_id)?,
            refresh_rate: Duration::from_millis(1000 / refresh_hz as u64),
        })
    }

    /// Run monitor loop
    pub fn run(&mut self) -> NvResult<()> {
        loop {
            // Clear screen (ANSI escape code)
            print!("\x1B[2J\x1B[1;1H");

            // Update data
            self.data.update()?;

            // Render
            println!("{}", self.data.render_ascii());

            // Sleep
            std::thread::sleep(self.refresh_rate);
        }
    }

    /// Run for limited time
    pub fn run_for(&mut self, duration: Duration) -> NvResult<()> {
        let start = Instant::now();

        while start.elapsed() < duration {
            print!("\x1B[2J\x1B[1;1H");
            self.data.update()?;
            println!("{}", self.data.render_ascii());
            std::thread::sleep(self.refresh_rate);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_data_creation() {
        let data = MonitorData::new(0);
        if let Ok(d) = data {
            println!("GPU: {}", d.gpu_name);
            println!("Driver: {}", d.driver_version);
        }
    }

    #[test]
    fn test_render_bar() {
        let data = MonitorData::new(0).unwrap_or_else(|_| {
            let mut d = MonitorData {
                gpu_id: 0,
                gpu_name: "Test".to_string(),
                driver_version: "1.0".to_string(),
                gpu_clock_mhz: 0,
                memory_clock_mhz: 0,
                temperature_c: 0,
                gpu_load_percent: 0,
                memory_load_percent: 0,
                vram_used_mb: 0,
                vram_total_mb: 0,
                fan_speed_percent: 0,
                power_draw_watts: 0.0,
                power_limit_watts: 0,
                temp_history: VecDeque::new(),
                load_history: VecDeque::new(),
                power_history: VecDeque::new(),
                clock_history: VecDeque::new(),
            };
            d
        });

        let bar = data.render_bar(50, 100);
        assert!(bar.contains('['));
        assert!(bar.contains(']'));
    }
}
