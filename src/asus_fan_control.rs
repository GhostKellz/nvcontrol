/// ASUS ROG Fan Control
///
/// Advanced fan control for ASUS ROG graphics cards with multi-fan support
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ASUS fan operating mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AsusFanMode {
    Silent,      // Minimum noise, 0 RPM mode enabled
    Performance, // Balanced cooling
    Turbo,       // Maximum cooling
    Manual,      // Custom fan curve
}

/// ASUS fan curve preset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsusFanCurve {
    pub name: String,
    pub mode: AsusFanMode,
    pub curve_points: Vec<(i32, u32)>, // (temp_c, fan_percent)
    pub zero_rpm_temp: Option<i32>,
    pub hysteresis: i32,
}

impl AsusFanCurve {
    /// Silent mode - ROG emphasis on quiet operation
    pub fn silent() -> Self {
        Self {
            name: "Silent".to_string(),
            mode: AsusFanMode::Silent,
            curve_points: vec![
                (40, 0), // 0 RPM below 40°C
                (50, 25),
                (60, 35),
                (70, 50),
                (80, 70),
                (90, 100),
            ],
            zero_rpm_temp: Some(40),
            hysteresis: 5,
        }
    }

    /// Performance mode - Balanced for gaming
    pub fn performance() -> Self {
        Self {
            name: "Performance".to_string(),
            mode: AsusFanMode::Performance,
            curve_points: vec![(35, 0), (50, 40), (65, 60), (75, 80), (85, 100)],
            zero_rpm_temp: Some(35),
            hysteresis: 3,
        }
    }

    /// Turbo mode - Maximum cooling for OC
    pub fn turbo() -> Self {
        Self {
            name: "Turbo".to_string(),
            mode: AsusFanMode::Turbo,
            curve_points: vec![
                (30, 40), // Always spinning
                (50, 60),
                (65, 75),
                (75, 90),
                (85, 100),
            ],
            zero_rpm_temp: None,
            hysteresis: 2,
        }
    }

    /// ROG Astral 5090 optimized curve
    pub fn rog_astral_5090() -> Self {
        Self {
            name: "ROG Astral 5090 OC".to_string(),
            mode: AsusFanMode::Performance,
            curve_points: vec![(30, 0), (45, 35), (60, 50), (70, 65), (80, 85), (90, 100)],
            zero_rpm_temp: Some(30),
            hysteresis: 4,
        }
    }

    /// Calculate fan speed for given temperature
    pub fn get_fan_speed(&self, current_temp: i32, previous_speed: u32) -> u32 {
        // Check zero RPM
        if let Some(zero_rpm_temp) = self.zero_rpm_temp {
            let adjusted_temp = if previous_speed > 0 {
                current_temp - self.hysteresis
            } else {
                current_temp
            };

            if adjusted_temp < zero_rpm_temp {
                return 0;
            }
        }

        // Linear interpolation between curve points
        for i in 0..self.curve_points.len() - 1 {
            let (temp1, speed1) = self.curve_points[i];
            let (temp2, speed2) = self.curve_points[i + 1];

            if current_temp >= temp1 && current_temp <= temp2 {
                let temp_range = temp2 - temp1;
                let speed_range = speed2 as i32 - speed1 as i32;
                let temp_offset = current_temp - temp1;

                let interpolated = speed1 as i32 + (speed_range * temp_offset / temp_range);
                return interpolated.max(0).min(100) as u32;
            }
        }

        // Beyond curve range
        if current_temp < self.curve_points[0].0 {
            self.curve_points[0].1
        } else {
            self.curve_points[self.curve_points.len() - 1].1
        }
    }
}

/// ASUS multi-fan controller
pub struct AsusMultiFanController {
    gpu_id: u32,
    num_fans: u32,
    current_mode: AsusFanMode,
    fan_curves: HashMap<String, AsusFanCurve>,
    per_fan_speeds: Vec<u32>,
}

impl AsusMultiFanController {
    pub fn new(gpu_id: u32) -> NvResult<Self> {
        let num_fans = Self::detect_fan_count(gpu_id)?;

        let mut fan_curves = HashMap::new();
        fan_curves.insert("Silent".to_string(), AsusFanCurve::silent());
        fan_curves.insert("Performance".to_string(), AsusFanCurve::performance());
        fan_curves.insert("Turbo".to_string(), AsusFanCurve::turbo());
        fan_curves.insert(
            "ROG Astral 5090".to_string(),
            AsusFanCurve::rog_astral_5090(),
        );

        Ok(Self {
            gpu_id,
            num_fans,
            current_mode: AsusFanMode::Performance,
            fan_curves,
            per_fan_speeds: vec![0; num_fans as usize],
        })
    }

    fn detect_fan_count(gpu_id: u32) -> NvResult<u32> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let device = nvml
            .device_by_index(gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        // Try to detect number of fans
        // Most ASUS ROG cards have 2-3 fans
        let mut count = 0;
        for i in 0..3 {
            if device.fan_speed(i).is_ok() {
                count += 1;
            }
        }

        if count == 0 {
            count = 1; // Assume at least 1 fan
        }

        Ok(count)
    }

    /// Apply fan mode
    pub fn apply_mode(&mut self, mode: AsusFanMode) -> NvResult<()> {
        let curve_name = match mode {
            AsusFanMode::Silent => "Silent",
            AsusFanMode::Performance => "Performance",
            AsusFanMode::Turbo => "Turbo",
            AsusFanMode::Manual => {
                self.current_mode = mode;
                return Ok(());
            }
        };

        self.apply_curve(curve_name)?;
        self.current_mode = mode;

        println!("Applied ASUS fan mode: {:?}", mode);

        Ok(())
    }

    /// Apply fan curve
    pub fn apply_curve(&mut self, curve_name: &str) -> NvResult<()> {
        let curve = self.fan_curves.get(curve_name).ok_or_else(|| {
            NvControlError::ConfigError(format!("Fan curve not found: {}", curve_name))
        })?;

        // Get current temperature
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let device = nvml
            .device_by_index(self.gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        let temp = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(50) as i32;

        // Calculate target fan speed
        let target_speed = curve.get_fan_speed(temp, self.per_fan_speeds[0]);

        // Apply to all fans
        self.set_all_fans(target_speed)?;

        println!(
            "Applied fan curve: {} ({}% at {}°C)",
            curve_name, target_speed, temp
        );

        Ok(())
    }

    /// Set individual fan speed
    pub fn set_fan(&mut self, fan_index: u32, speed_percent: u32) -> NvResult<()> {
        if fan_index >= self.num_fans {
            return Err(NvControlError::RuntimeError(format!(
                "Invalid fan index: {} (GPU has {} fans)",
                fan_index, self.num_fans
            )));
        }

        use std::process::Command;

        // Enable manual fan control
        let output = Command::new("nvidia-settings")
            .args(&["-a", &format!("[gpu:{}]/GPUFanControlState=1", self.gpu_id)])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("nvidia-settings failed: {}", e)))?;

        if !output.status.success() {
            return Err(NvControlError::FanControlNotSupported);
        }

        // Set fan speed
        let output = Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!("[fan:{}]/GPUTargetFanSpeed={}", fan_index, speed_percent),
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("nvidia-settings failed: {}", e)))?;

        if output.status.success() {
            self.per_fan_speeds[fan_index as usize] = speed_percent;
            println!("Fan {} set to {}%", fan_index, speed_percent);
        }

        Ok(())
    }

    /// Set all fans to same speed
    pub fn set_all_fans(&mut self, speed_percent: u32) -> NvResult<()> {
        for i in 0..self.num_fans {
            self.set_fan(i, speed_percent)?;
        }
        Ok(())
    }

    /// Get fan speeds
    pub fn get_fan_speeds(&self) -> NvResult<Vec<u32>> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let device = nvml
            .device_by_index(self.gpu_id)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        let mut speeds = Vec::new();

        for i in 0..self.num_fans {
            let speed = device.fan_speed(i).unwrap_or(0);
            speeds.push(speed);
        }

        Ok(speeds)
    }

    /// Reset to auto fan control
    pub fn reset_to_auto(&mut self) -> NvResult<()> {
        use std::process::Command;

        let output = Command::new("nvidia-settings")
            .args(&["-a", &format!("[gpu:{}]/GPUFanControlState=0", self.gpu_id)])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("nvidia-settings failed: {}", e)))?;

        if output.status.success() {
            println!("Reset to automatic fan control");
        }

        Ok(())
    }

    pub fn num_fans(&self) -> u32 {
        self.num_fans
    }

    pub fn current_mode(&self) -> AsusFanMode {
        self.current_mode
    }

    /// Add custom fan curve
    pub fn add_custom_curve(&mut self, curve: AsusFanCurve) {
        self.fan_curves.insert(curve.name.clone(), curve);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_curve_interpolation() {
        let curve = AsusFanCurve::performance();

        let speed_at_55 = curve.get_fan_speed(55, 0);
        assert!(speed_at_55 > 30 && speed_at_55 < 50);

        let speed_at_70 = curve.get_fan_speed(70, 0);
        assert!(speed_at_70 > 60 && speed_at_70 < 85);
    }

    #[test]
    fn test_zero_rpm_mode() {
        let curve = AsusFanCurve::silent();

        let speed_low_temp = curve.get_fan_speed(35, 0);
        assert_eq!(speed_low_temp, 0);

        let speed_high_temp = curve.get_fan_speed(60, 0);
        assert!(speed_high_temp > 0);
    }

    #[test]
    fn test_fan_modes() {
        let silent = AsusFanCurve::silent();
        let performance = AsusFanCurve::performance();
        let turbo = AsusFanCurve::turbo();

        // Turbo should be most aggressive
        assert!(turbo.get_fan_speed(70, 0) > performance.get_fan_speed(70, 0));
        assert!(performance.get_fan_speed(70, 0) > silent.get_fan_speed(70, 0));
    }

    #[test]
    fn test_rog_astral_5090_curve() {
        let curve = AsusFanCurve::rog_astral_5090();

        assert_eq!(curve.mode, AsusFanMode::Performance);
        assert!(curve.zero_rpm_temp.is_some());
        assert_eq!(curve.zero_rpm_temp.unwrap(), 30);
    }
}
