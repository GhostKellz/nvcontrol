/// MSI Afterburner / GPU Tweak Style Tuner GUI
///
/// Professional GPU tuning interface with real-time graphs and controls

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Tuner widget state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunerState {
    pub gpu_id: u32,

    // Overclocking sliders
    pub core_clock_offset: i32,      // -500 to +500 MHz
    pub memory_clock_offset: i32,    // -1000 to +1500 MHz
    pub power_limit: u32,             // % of TDP
    pub temp_limit: i32,              // °C
    pub voltage_offset: i32,          // mV (if supported)

    // Fan control
    pub fan_mode: FanControlMode,
    pub fan_speed_manual: u32,        // % if manual mode
    pub fan_curve: Vec<(i32, u32)>,   // (temp, speed) points

    // Monitoring
    pub gpu_clock: u32,
    pub memory_clock: u32,
    pub temperature: i32,
    pub gpu_load: u32,
    pub memory_load: u32,
    pub vram_used: u64,
    pub vram_total: u64,
    pub power_draw: f32,
    pub fan_speed: u32,

    // Graph history (for live charts)
    pub temp_history: VecDeque<i32>,
    pub load_history: VecDeque<u32>,
    pub clock_history: VecDeque<u32>,
    pub power_history: VecDeque<f32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FanControlMode {
    Auto,
    Manual,
    Curve,
}

impl TunerState {
    pub fn new(gpu_id: u32) -> Self {
        Self {
            gpu_id,
            core_clock_offset: 0,
            memory_clock_offset: 0,
            power_limit: 100,
            temp_limit: 83,
            voltage_offset: 0,
            fan_mode: FanControlMode::Auto,
            fan_speed_manual: 50,
            fan_curve: vec![
                (40, 30),
                (55, 50),
                (70, 70),
                (80, 90),
                (90, 100),
            ],
            gpu_clock: 0,
            memory_clock: 0,
            temperature: 0,
            gpu_load: 0,
            memory_load: 0,
            vram_used: 0,
            vram_total: 0,
            power_draw: 0.0,
            fan_speed: 0,
            temp_history: VecDeque::with_capacity(300),
            load_history: VecDeque::with_capacity(300),
            clock_history: VecDeque::with_capacity(300),
            power_history: VecDeque::with_capacity(300),
        }
    }

    /// Update monitoring data
    pub fn update_monitoring(&mut self) -> NvResult<()> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let device = nvml.device_by_index(self.gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        self.gpu_clock = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
            .unwrap_or(0);
        self.memory_clock = device
            .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
            .unwrap_or(0);

        self.temperature = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0) as i32;

        if let Ok(util) = device.utilization_rates() {
            self.gpu_load = util.gpu;
            self.memory_load = util.memory;
        }

        if let Ok(mem_info) = device.memory_info() {
            self.vram_used = mem_info.used / 1024 / 1024;
            self.vram_total = mem_info.total / 1024 / 1024;
        }

        self.power_draw = device.power_usage().unwrap_or(0) as f32 / 1000.0;
        self.fan_speed = device.fan_speed(0).unwrap_or(0);

        // Update history (store values to avoid borrow conflicts)
        let temp = self.temperature;
        let load = self.gpu_load;
        let clock = self.gpu_clock;
        let power = self.power_draw;

        Self::add_to_history(temp, &mut self.temp_history);
        Self::add_to_history(load, &mut self.load_history);
        Self::add_to_history(clock, &mut self.clock_history);
        Self::add_to_history(power, &mut self.power_history);

        Ok(())
    }

    fn add_to_history<T>(value: T, history: &mut VecDeque<T>) {
        history.push_back(value);
        if history.len() > 300 {
            history.pop_front();
        }
    }

    /// Apply overclocking settings
    pub fn apply_overclock(&self) -> NvResult<()> {
        use std::process::Command;

        // Apply GPU clock offset
        if self.core_clock_offset != 0 {
            Command::new("nvidia-smi")
                .args(&[
                    "-i",
                    &self.gpu_id.to_string(),
                    "-lgc",
                    &format!("{:+}", self.core_clock_offset),
                ])
                .output()
                .ok();
        }

        // Apply memory clock offset
        if self.memory_clock_offset != 0 {
            Command::new("nvidia-smi")
                .args(&[
                    "-i",
                    &self.gpu_id.to_string(),
                    "-lmc",
                    &format!("{:+}", self.memory_clock_offset),
                ])
                .output()
                .ok();
        }

        // Apply power limit
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let mut device = nvml.device_by_index(self.gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        let default_power = device.power_management_limit_default().ok();
        if let Some(default) = default_power {
            let target_power = (default as f32 * self.power_limit as f32 / 100.0) as u32;
            device.set_power_management_limit(target_power).ok();
        }

        println!("Applied tuner settings:");
        println!("  Core Clock: {:+} MHz", self.core_clock_offset);
        println!("  Memory Clock: {:+} MHz", self.memory_clock_offset);
        println!("  Power Limit: {}%", self.power_limit);

        Ok(())
    }

    /// Apply fan settings
    pub fn apply_fan_control(&self) -> NvResult<()> {
        use std::process::Command;

        match self.fan_mode {
            FanControlMode::Auto => {
                // Reset to auto
                Command::new("nvidia-settings")
                    .args(&[
                        "-a",
                        &format!("[gpu:{}]/GPUFanControlState=0", self.gpu_id),
                    ])
                    .output()
                    .ok();

                println!("Fan control set to: Auto");
            }
            FanControlMode::Manual => {
                // Enable manual control
                Command::new("nvidia-settings")
                    .args(&[
                        "-a",
                        &format!("[gpu:{}]/GPUFanControlState=1", self.gpu_id),
                    ])
                    .output()
                    .ok();

                // Set speed
                Command::new("nvidia-settings")
                    .args(&[
                        "-a",
                        &format!("[fan:0]/GPUTargetFanSpeed={}", self.fan_speed_manual),
                    ])
                    .output()
                    .ok();

                println!("Fan control set to: Manual ({}%)", self.fan_speed_manual);
            }
            FanControlMode::Curve => {
                // Enable manual control
                Command::new("nvidia-settings")
                    .args(&[
                        "-a",
                        &format!("[gpu:{}]/GPUFanControlState=1", self.gpu_id),
                    ])
                    .output()
                    .ok();

                // Calculate speed from curve
                let target_speed = self.calculate_fan_speed_from_curve();

                Command::new("nvidia-settings")
                    .args(&[
                        "-a",
                        &format!("[fan:0]/GPUTargetFanSpeed={}", target_speed),
                    ])
                    .output()
                    .ok();

                println!("Fan control set to: Curve ({}% at {}°C)", target_speed, self.temperature);
            }
        }

        Ok(())
    }

    fn calculate_fan_speed_from_curve(&self) -> u32 {
        // Linear interpolation
        for i in 0..self.fan_curve.len() - 1 {
            let (temp1, speed1) = self.fan_curve[i];
            let (temp2, speed2) = self.fan_curve[i + 1];

            if self.temperature >= temp1 && self.temperature <= temp2 {
                let temp_range = temp2 - temp1;
                let speed_range = speed2 as i32 - speed1 as i32;
                let temp_offset = self.temperature - temp1;

                let speed = speed1 as i32 + (speed_range * temp_offset / temp_range);
                return speed.max(0).min(100) as u32;
            }
        }

        // Out of range
        if self.temperature < self.fan_curve[0].0 {
            self.fan_curve[0].1
        } else {
            self.fan_curve[self.fan_curve.len() - 1].1
        }
    }

    /// Reset all settings to default
    pub fn reset_to_defaults(&mut self) -> NvResult<()> {
        self.core_clock_offset = 0;
        self.memory_clock_offset = 0;
        self.power_limit = 100;
        self.fan_mode = FanControlMode::Auto;

        // Reset hardware
        use std::process::Command;

        Command::new("nvidia-smi")
            .args(&["-i", &self.gpu_id.to_string(), "-rgc"])
            .output()
            .ok();

        Command::new("nvidia-smi")
            .args(&["-i", &self.gpu_id.to_string(), "-rmc"])
            .output()
            .ok();

        Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!("[gpu:{}]/GPUFanControlState=0", self.gpu_id),
            ])
            .output()
            .ok();

        println!("Reset to default settings");

        Ok(())
    }
}

/// Tuner preset profiles
pub struct TunerPresets;

impl TunerPresets {
    pub fn silent(gpu_id: u32) -> TunerState {
        let mut state = TunerState::new(gpu_id);
        state.core_clock_offset = -100;
        state.memory_clock_offset = 0;
        state.power_limit = 85;
        state.temp_limit = 75;
        state.fan_mode = FanControlMode::Curve;
        state.fan_curve = vec![
            (40, 0),
            (50, 25),
            (60, 40),
            (70, 60),
            (80, 80),
        ];
        state
    }

    pub fn gaming(gpu_id: u32) -> TunerState {
        let mut state = TunerState::new(gpu_id);
        state.core_clock_offset = 0;
        state.memory_clock_offset = 0;
        state.power_limit = 100;
        state.temp_limit = 83;
        state.fan_mode = FanControlMode::Auto;
        state
    }

    pub fn overclocking(gpu_id: u32) -> TunerState {
        let mut state = TunerState::new(gpu_id);
        state.core_clock_offset = 150;
        state.memory_clock_offset = 800;
        state.power_limit = 115;
        state.temp_limit = 85;
        state.fan_mode = FanControlMode::Curve;
        state.fan_curve = vec![
            (30, 40),
            (50, 60),
            (70, 80),
            (80, 100),
        ];
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuner_state_creation() {
        let state = TunerState::new(0);
        assert_eq!(state.gpu_id, 0);
        assert_eq!(state.core_clock_offset, 0);
        assert_eq!(state.fan_mode, FanControlMode::Auto);
    }

    #[test]
    fn test_fan_curve_calculation() {
        let mut state = TunerState::new(0);
        state.temperature = 60;
        state.fan_curve = vec![
            (40, 30),
            (60, 50),
            (80, 80),
        ];

        let speed = state.calculate_fan_speed_from_curve();
        assert_eq!(speed, 50);
    }

    #[test]
    fn test_presets() {
        let silent = TunerPresets::silent(0);
        assert!(silent.core_clock_offset < 0);
        assert!(silent.power_limit < 100);

        let oc = TunerPresets::overclocking(0);
        assert!(oc.core_clock_offset > 0);
        assert!(oc.power_limit > 100);
    }
}
