/// Phase 3.2: Intelligent Fan Control
///
/// ML-based fan curve optimization, acoustic optimization, per-fan control, zero RPM mode

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Fan control mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FanMode {
    Auto,
    Manual,
    Curve,
    Acoustic,
    Performance,
    Silent,
}

/// Advanced fan curve with hysteresis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFanCurve {
    pub points: Vec<FanCurvePoint>,
    pub hysteresis_temp: i32,
    pub zero_rpm_temp: Option<i32>,
    pub min_fan_speed: u32,
    pub max_fan_speed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurvePoint {
    pub temp: i32,
    pub speed: u32,
}

impl AdvancedFanCurve {
    pub fn performance() -> Self {
        Self {
            points: vec![
                FanCurvePoint { temp: 30, speed: 30 },
                FanCurvePoint { temp: 50, speed: 50 },
                FanCurvePoint { temp: 65, speed: 70 },
                FanCurvePoint { temp: 75, speed: 90 },
                FanCurvePoint { temp: 85, speed: 100 },
            ],
            hysteresis_temp: 3,
            zero_rpm_temp: None,
            min_fan_speed: 30,
            max_fan_speed: 100,
        }
    }

    pub fn silent() -> Self {
        Self {
            points: vec![
                FanCurvePoint { temp: 40, speed: 0 },
                FanCurvePoint { temp: 55, speed: 35 },
                FanCurvePoint { temp: 70, speed: 55 },
                FanCurvePoint { temp: 80, speed: 75 },
                FanCurvePoint { temp: 90, speed: 100 },
            ],
            hysteresis_temp: 5,
            zero_rpm_temp: Some(40),
            min_fan_speed: 0,
            max_fan_speed: 100,
        }
    }

    pub fn balanced() -> Self {
        Self {
            points: vec![
                FanCurvePoint { temp: 35, speed: 0 },
                FanCurvePoint { temp: 50, speed: 40 },
                FanCurvePoint { temp: 65, speed: 60 },
                FanCurvePoint { temp: 75, speed: 80 },
                FanCurvePoint { temp: 85, speed: 100 },
            ],
            hysteresis_temp: 3,
            zero_rpm_temp: Some(35),
            min_fan_speed: 0,
            max_fan_speed: 100,
        }
    }

    /// Calculate fan speed for given temperature
    pub fn get_fan_speed(&self, current_temp: i32, previous_speed: u32) -> u32 {
        // Apply hysteresis to prevent rapid fan speed changes
        let adjusted_temp = if previous_speed > 0 {
            current_temp - self.hysteresis_temp
        } else {
            current_temp
        };

        // Handle zero RPM mode
        if let Some(zero_rpm_temp) = self.zero_rpm_temp {
            if adjusted_temp < zero_rpm_temp {
                return 0;
            }
        }

        // Linear interpolation between curve points
        for i in 0..self.points.len() - 1 {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];

            if adjusted_temp >= p1.temp && adjusted_temp <= p2.temp {
                let temp_range = p2.temp - p1.temp;
                let speed_range = p2.speed as i32 - p1.speed as i32;
                let temp_offset = adjusted_temp - p1.temp;

                let interpolated_speed =
                    p1.speed as i32 + (speed_range * temp_offset / temp_range);
                return interpolated_speed
                    .max(self.min_fan_speed as i32)
                    .min(self.max_fan_speed as i32) as u32;
            }
        }

        // Temperature beyond curve range
        if adjusted_temp < self.points[0].temp {
            self.points[0].speed
        } else {
            self.points[self.points.len() - 1].speed
        }
    }
}

/// Acoustic profile for noise optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticProfile {
    pub max_noise_db: f32,
    pub target_temp: i32,
    pub aggressive_temp: i32,
}

impl AcousticProfile {
    pub fn quiet() -> Self {
        Self {
            max_noise_db: 35.0,
            target_temp: 75,
            aggressive_temp: 85,
        }
    }

    pub fn balanced() -> Self {
        Self {
            max_noise_db: 40.0,
            target_temp: 70,
            aggressive_temp: 80,
        }
    }

    pub fn performance() -> Self {
        Self {
            max_noise_db: 50.0,
            target_temp: 65,
            aggressive_temp: 75,
        }
    }
}

/// ML-based fan optimizer using historical temperature data
#[allow(dead_code)]
pub struct FanOptimizer {
    gpu_id: u32,
    temp_history: VecDeque<TempSample>,
    max_history: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TempSample {
    timestamp: std::time::Instant,
    temperature: i32,
    fan_speed: u32,
    gpu_load: u32,
}

impl FanOptimizer {
    pub fn new(gpu_id: u32) -> Self {
        Self {
            gpu_id,
            temp_history: VecDeque::with_capacity(300), // 5 minutes at 1 sample/sec
            max_history: 300,
        }
    }

    /// Record temperature sample
    pub fn record_sample(&mut self, temp: i32, fan_speed: u32, gpu_load: u32) {
        let sample = TempSample {
            timestamp: std::time::Instant::now(),
            temperature: temp,
            fan_speed,
            gpu_load,
        };

        self.temp_history.push_back(sample);

        if self.temp_history.len() > self.max_history {
            self.temp_history.pop_front();
        }
    }

    /// Predict temperature trend
    pub fn predict_temp_trend(&self) -> TempTrend {
        if self.temp_history.len() < 10 {
            return TempTrend::Stable;
        }

        let recent_samples: Vec<_> = self.temp_history.iter().rev().take(10).collect();

        let avg_recent: f32 = recent_samples.iter().map(|s| s.temperature as f32).sum::<f32>()
            / recent_samples.len() as f32;

        let older_samples: Vec<_> = self
            .temp_history
            .iter()
            .rev()
            .skip(10)
            .take(10)
            .collect();

        if older_samples.is_empty() {
            return TempTrend::Stable;
        }

        let avg_older: f32 = older_samples.iter().map(|s| s.temperature as f32).sum::<f32>()
            / older_samples.len() as f32;

        let diff = avg_recent - avg_older;

        if diff > 3.0 {
            TempTrend::Rising
        } else if diff < -3.0 {
            TempTrend::Falling
        } else {
            TempTrend::Stable
        }
    }

    /// Optimize fan curve based on usage patterns
    pub fn optimize_curve(&self) -> AdvancedFanCurve {
        if self.temp_history.is_empty() {
            return AdvancedFanCurve::balanced();
        }

        // Analyze temperature distribution
        let temps: Vec<i32> = self.temp_history.iter().map(|s| s.temperature).collect();
        let avg_temp: f32 = temps.iter().sum::<i32>() as f32 / temps.len() as f32;
        let max_temp = *temps.iter().max().unwrap_or(&75);

        // Create optimized curve based on thermal behavior
        if avg_temp > 75.0 || max_temp > 85 {
            AdvancedFanCurve::performance()
        } else if avg_temp < 60.0 && max_temp < 75 {
            AdvancedFanCurve::silent()
        } else {
            AdvancedFanCurve::balanced()
        }
    }

    /// Get recommended fan speed with predictive adjustment
    pub fn get_recommended_speed(
        &self,
        current_temp: i32,
        current_speed: u32,
        curve: &AdvancedFanCurve,
    ) -> u32 {
        let base_speed = curve.get_fan_speed(current_temp, current_speed);

        // Adjust based on temperature trend
        match self.predict_temp_trend() {
            TempTrend::Rising => {
                // Increase fan speed proactively
                (base_speed + 10).min(100)
            }
            TempTrend::Falling => {
                // Allow fan speed to decrease more slowly
                if base_speed < current_speed {
                    current_speed.saturating_sub(5)
                } else {
                    base_speed
                }
            }
            TempTrend::Stable => base_speed,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempTrend {
    Rising,
    Falling,
    Stable,
}

/// Multi-fan controller for GPUs with multiple fans
pub struct MultiFanController {
    gpu_id: u32,
    num_fans: u32,
}

impl MultiFanController {
    pub fn new(gpu_id: u32) -> NvResult<Self> {
        let num_fans = Self::detect_num_fans(gpu_id)?;

        Ok(Self { gpu_id, num_fans })
    }

    fn detect_num_fans(gpu_id: u32) -> NvResult<u32> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let device = nvml.device_by_index(gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        // Try to get number of fans
        // Note: NVML doesn't expose fan count directly, typically 1-3 fans
        // We'll detect by trying to read each fan
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

    /// Set individual fan speed
    pub fn set_fan_speed(&self, fan_index: u32, speed_percent: u32) -> NvResult<()> {
        if fan_index >= self.num_fans {
            return Err(NvControlError::RuntimeError(format!(
                "Invalid fan index: {} (GPU has {} fans)",
                fan_index, self.num_fans
            )));
        }

        // Use nvidia-settings for per-fan control
        use std::process::Command;

        let output = Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!(
                    "[gpu:{}]/GPUFanControlState=1",
                    self.gpu_id
                ),
            ])
            .output()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("nvidia-settings failed: {}", e))
            })?;

        if !output.status.success() {
            return Err(NvControlError::FanControlNotSupported);
        }

        let output = Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!(
                    "[fan:{}]/GPUTargetFanSpeed={}",
                    fan_index, speed_percent
                ),
            ])
            .output()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("nvidia-settings failed: {}", e))
            })?;

        if !output.status.success() {
            return Err(NvControlError::FanControlNotSupported);
        }

        Ok(())
    }

    /// Get individual fan speed
    pub fn get_fan_speed(&self, fan_index: u32) -> NvResult<u32> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let device = nvml.device_by_index(self.gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        device.fan_speed(fan_index).map_err(|_| {
            NvControlError::FanControlNotSupported
        })
    }

    /// Set all fans to same speed
    pub fn set_all_fans(&self, speed_percent: u32) -> NvResult<()> {
        for i in 0..self.num_fans {
            self.set_fan_speed(i, speed_percent)?;
        }
        Ok(())
    }

    pub fn num_fans(&self) -> u32 {
        self.num_fans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_curve_interpolation() {
        let curve = AdvancedFanCurve::balanced();

        let speed = curve.get_fan_speed(60, 0);
        assert!(speed > 30 && speed < 70);
    }

    #[test]
    fn test_zero_rpm_mode() {
        let curve = AdvancedFanCurve::silent();

        let speed = curve.get_fan_speed(35, 0);
        assert_eq!(speed, 0);
    }

    #[test]
    fn test_hysteresis() {
        let curve = AdvancedFanCurve::balanced();

        let speed1 = curve.get_fan_speed(50, 0);
        let speed2 = curve.get_fan_speed(50, 50);

        // Speed should be different due to hysteresis
        assert_ne!(speed1, speed2);
    }

    #[test]
    fn test_preset_curves() {
        let perf = AdvancedFanCurve::performance();
        let silent = AdvancedFanCurve::silent();
        let balanced = AdvancedFanCurve::balanced();

        // Performance should be more aggressive
        assert!(perf.get_fan_speed(70, 0) > balanced.get_fan_speed(70, 0));
        assert!(balanced.get_fan_speed(70, 0) > silent.get_fan_speed(70, 0));
    }

    #[test]
    fn test_optimizer_trend_detection() {
        let mut optimizer = FanOptimizer::new(0);

        // Simulate rising temperature
        for i in 40..60 {
            optimizer.record_sample(i, 50, 80);
        }

        assert_eq!(optimizer.predict_temp_trend(), TempTrend::Rising);
    }

    #[test]
    fn test_acoustic_profiles() {
        let quiet = AcousticProfile::quiet();
        let perf = AcousticProfile::performance();

        assert!(quiet.max_noise_db < perf.max_noise_db);
        assert!(quiet.target_temp > perf.target_temp);
    }
}
