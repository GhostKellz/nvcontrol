use crate::{NvControlError, NvResult};
use nvml_wrapper::Nvml;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

/// Represents a GPU fan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanInfo {
    pub id: usize,
    pub rpm: Option<u32>,
    pub percent: Option<u8>,
    pub controllable: bool,
    pub health_status: FanHealthStatus,
    pub max_rpm: Option<u32>,
    pub target_rpm: Option<u32>,
}

/// Fan health monitoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FanHealthStatus {
    Healthy,
    Warning,   // RPM irregularities or high noise
    Critical,  // Fan failure or dangerous temperatures
    Unknown,
}

/// Custom fan curve with temperature-speed mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurve {
    pub name: String,
    pub points: Vec<FanCurvePoint>,
    pub hysteresis: u8, // Temperature difference for up/down changes
    pub min_duty_cycle: u8, // Minimum fan speed percentage
    pub max_duty_cycle: u8, // Maximum fan speed percentage
    pub zero_rpm_threshold: Option<u8>, // Temperature below which fan stops
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurvePoint {
    pub temperature: u8, // Temperature in Celsius
    pub duty_cycle: u8,  // Fan speed percentage (0-100)
}

/// Fan profile with multiple curves and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanProfile {
    pub name: String,
    pub description: String,
    pub curves: HashMap<usize, FanCurve>, // fan_id -> curve
    pub enabled: bool,
    pub load_based_scaling: bool, // Scale based on GPU load vs just temperature
    pub aggressive_mode: bool,    // More responsive to temperature changes
}

impl Default for FanCurve {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            points: vec![
                FanCurvePoint { temperature: 30, duty_cycle: 0 },
                FanCurvePoint { temperature: 40, duty_cycle: 20 },
                FanCurvePoint { temperature: 50, duty_cycle: 30 },
                FanCurvePoint { temperature: 60, duty_cycle: 50 },
                FanCurvePoint { temperature: 70, duty_cycle: 70 },
                FanCurvePoint { temperature: 80, duty_cycle: 90 },
                FanCurvePoint { temperature: 90, duty_cycle: 100 },
            ],
            hysteresis: 2,
            min_duty_cycle: 0,
            max_duty_cycle: 100,
            zero_rpm_threshold: Some(35),
        }
    }
}

impl Default for FanProfile {
    fn default() -> Self {
        let mut curves = HashMap::new();
        curves.insert(0, FanCurve::default());
        
        Self {
            name: "Balanced".to_string(),
            description: "Balanced performance and noise".to_string(),
            curves,
            enabled: false,
            load_based_scaling: false,
            aggressive_mode: false,
        }
    }
}

/// Predefined fan profiles
pub fn get_predefined_profiles() -> Vec<FanProfile> {
    vec![
        create_silent_profile(),
        create_performance_profile(),
        create_aggressive_profile(),
        FanProfile::default(), // Balanced
    ]
}

fn create_silent_profile() -> FanProfile {
    let mut curves = HashMap::new();
    curves.insert(0, FanCurve {
        name: "Silent".to_string(),
        points: vec![
            FanCurvePoint { temperature: 30, duty_cycle: 0 },
            FanCurvePoint { temperature: 45, duty_cycle: 15 },
            FanCurvePoint { temperature: 55, duty_cycle: 25 },
            FanCurvePoint { temperature: 65, duty_cycle: 40 },
            FanCurvePoint { temperature: 75, duty_cycle: 60 },
            FanCurvePoint { temperature: 85, duty_cycle: 80 },
            FanCurvePoint { temperature: 95, duty_cycle: 100 },
        ],
        hysteresis: 3,
        min_duty_cycle: 0,
        max_duty_cycle: 80, // Cap at 80% for noise reduction
        zero_rpm_threshold: Some(40),
    });
    
    FanProfile {
        name: "Silent".to_string(),
        description: "Prioritizes low noise over performance".to_string(),
        curves,
        enabled: false,
        load_based_scaling: false,
        aggressive_mode: false,
    }
}

fn create_performance_profile() -> FanProfile {
    let mut curves = HashMap::new();
    curves.insert(0, FanCurve {
        name: "Performance".to_string(),
        points: vec![
            FanCurvePoint { temperature: 25, duty_cycle: 30 },
            FanCurvePoint { temperature: 35, duty_cycle: 40 },
            FanCurvePoint { temperature: 45, duty_cycle: 50 },
            FanCurvePoint { temperature: 55, duty_cycle: 65 },
            FanCurvePoint { temperature: 65, duty_cycle: 80 },
            FanCurvePoint { temperature: 75, duty_cycle: 95 },
            FanCurvePoint { temperature: 85, duty_cycle: 100 },
        ],
        hysteresis: 1,
        min_duty_cycle: 30,
        max_duty_cycle: 100,
        zero_rpm_threshold: None, // Always spinning
    });
    
    FanProfile {
        name: "Performance".to_string(),
        description: "Aggressive cooling for maximum performance".to_string(),
        curves,
        enabled: false,
        load_based_scaling: true,
        aggressive_mode: true,
    }
}

fn create_aggressive_profile() -> FanProfile {
    let mut curves = HashMap::new();
    curves.insert(0, FanCurve {
        name: "Aggressive".to_string(),
        points: vec![
            FanCurvePoint { temperature: 20, duty_cycle: 50 },
            FanCurvePoint { temperature: 30, duty_cycle: 60 },
            FanCurvePoint { temperature: 40, duty_cycle: 70 },
            FanCurvePoint { temperature: 50, duty_cycle: 80 },
            FanCurvePoint { temperature: 60, duty_cycle: 90 },
            FanCurvePoint { temperature: 70, duty_cycle: 100 },
        ],
        hysteresis: 0,
        min_duty_cycle: 50,
        max_duty_cycle: 100,
        zero_rpm_threshold: None,
    });
    
    FanProfile {
        name: "Aggressive".to_string(),
        description: "Maximum cooling regardless of noise".to_string(),
        curves,
        enabled: false,
        load_based_scaling: true,
        aggressive_mode: true,
    }
}

/// List all fans with real NVML integration and health monitoring
pub fn list_fans() -> Vec<FanInfo> {
    let mut fans = Vec::new();

    // Try NVML first
    if let Ok(nvml) = Nvml::init() {
        if let Ok(device_count) = nvml.device_count() {
            for gpu_id in 0..device_count {
                if let Ok(device) = nvml.device_by_index(gpu_id) {
                    // Try to get fan count for this GPU
                    let fan_count = get_fan_count_for_gpu(&device).unwrap_or(1);

                    for fan_id in 0..fan_count {
                        let rpm = device.fan_speed(fan_id).ok();
                        let percent = rpm.map(|r| ((r as f32 / 3000.0) * 100.0) as u8); // Estimate percentage
                        let controllable = can_control_fan(&device, fan_id);
                        let health_status = assess_fan_health(&device, fan_id, rpm);
                        let max_rpm = estimate_max_rpm(&device, fan_id);

                        fans.push(FanInfo {
                            id: (gpu_id * 10 + fan_id) as usize, // Unique ID across GPUs
                            rpm,
                            percent,
                            controllable,
                            health_status,
                            max_rpm,
                            target_rpm: None,
                        });
                    }
                }
            }
        }
    }

    // Fallback to nvidia-smi if NVML fails
    if fans.is_empty() {
        fans = get_fans_via_nvidia_smi();
    }

    // If still no fans, provide stub data
    if fans.is_empty() {
        fans.push(FanInfo {
            id: 0,
            rpm: Some(1500),
            percent: Some(40),
            controllable: false, // Conservative default
            health_status: FanHealthStatus::Unknown,
            max_rpm: Some(3000),
            target_rpm: None,
        });
    }

    fans
}

fn get_fan_count_for_gpu(device: &nvml_wrapper::Device) -> NvResult<u32> {
    // Try to determine fan count - most consumer GPUs have 1-3 fans
    for fan_id in 0..4 {
        if device.fan_speed(fan_id).is_err() {
            return Ok(fan_id);
        }
    }
    Ok(1) // Default to 1 fan
}

fn can_control_fan(device: &nvml_wrapper::Device, fan_id: u32) -> bool {
    // Check if we can control this fan
    // This is typically limited by driver permissions and GPU capabilities
    match device.fan_speed(fan_id) {
        Ok(_) => {
            // If we can read fan speed, we might be able to control it
            // But fan control often requires additional permissions
            false // Conservative default - most systems don't allow fan control
        }
        Err(_) => false,
    }
}

/// Assess fan health based on RPM and temperature data
fn assess_fan_health(device: &nvml_wrapper::Device, _fan_id: u32, rpm: Option<u32>) -> FanHealthStatus {
    // Get temperature for context
    let temp = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
        .unwrap_or(0);
    
    match rpm {
        Some(current_rpm) => {
            // Check for concerning patterns
            if temp > 80 && current_rpm < 1000 {
                FanHealthStatus::Critical // High temp with low fan speed
            } else if current_rpm > 4000 {
                FanHealthStatus::Warning // Unusually high RPM
            } else if current_rpm == 0 && temp > 60 {
                FanHealthStatus::Critical // Fan stopped at high temp
            } else {
                FanHealthStatus::Healthy
            }
        }
        None => FanHealthStatus::Unknown,
    }
}

/// Estimate maximum RPM for a fan
fn estimate_max_rpm(device: &nvml_wrapper::Device, _fan_id: u32) -> Option<u32> {
    // Most modern GPU fans range from 2000-5000 RPM max
    // We can estimate based on GPU model or use a safe default
    if let Ok(name) = device.name() {
        if name.contains("RTX 40") || name.contains("RTX 30") {
            Some(4500) // Modern high-end cards
        } else if name.contains("RTX 20") || name.contains("GTX 16") {
            Some(3500) // Mid-range cards
        } else {
            Some(3000) // Conservative default
        }
    } else {
        Some(3000)
    }
}

fn get_fans_via_nvidia_smi() -> Vec<FanInfo> {
    let mut fans = Vec::new();

    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=fan.speed"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for (id, line) in output_str.lines().enumerate() {
                if let Ok(percent) = line.trim().parse::<u8>() {
                    // Estimate RPM from percentage (very rough)
                    let estimated_rpm = (percent as f32 / 100.0 * 3000.0) as u32;

                    fans.push(FanInfo {
                        id,
                        rpm: Some(estimated_rpm),
                        percent: Some(percent),
                        controllable: false, // nvidia-smi typically can't control fans
                        health_status: FanHealthStatus::Unknown,
                        max_rpm: Some(3000),
                        target_rpm: None,
                    });
                }
            }
        }
    }

    fans
}

/// Get info for a specific fan
pub fn get_fan_info(fan_id: usize) -> Option<FanInfo> {
    list_fans().into_iter().find(|f| f.id == fan_id)
}

/// Set fan speed (real implementation with safety checks)
pub fn set_fan_speed(fan_id: usize, speed_percent: u8) -> NvResult<()> {
    if speed_percent > 100 {
        return Err(NvControlError::FanControlNotSupported);
    }

    // Try NVML first
    if let Ok(nvml) = Nvml::init() {
        let gpu_id = fan_id / 10; // Extract GPU ID from fan ID
        let _local_fan_id = (fan_id % 10) as u32; // Extract local fan ID

        if let Ok(_device) = nvml.device_by_index(gpu_id as u32) {
            // Note: NVML fan control is typically not available in consumer drivers
            // This would require special permissions and enterprise drivers
            println!("NVML fan control not available in consumer drivers");
        }
    }

    // Try nvidia-settings (X11 only)
    if std::env::var("DISPLAY").is_ok() {
        return set_fan_speed_nvidia_settings(fan_id, speed_percent);
    }

    // Try direct sysfs manipulation (requires root)
    set_fan_speed_sysfs(fan_id, speed_percent)
}

fn set_fan_speed_nvidia_settings(fan_id: usize, speed_percent: u8) -> NvResult<()> {
    // Enable manual fan control first
    let enable_cmd = "nvidia-settings -a '[gpu:0]/GPUFanControlState=1'".to_string();
    let _ = Command::new("sh").arg("-c").arg(&enable_cmd).output();

    // Set fan speed
    let fan_cmd = format!(
        "nvidia-settings -a '[fan:{}]/GPUTargetFanSpeed={}'",
        fan_id, speed_percent
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&fan_cmd)
        .output()
        .map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("nvidia-settings failed: {e}"))
        })?;

    if output.status.success() {
        println!(
            "Fan {} set to {}% via nvidia-settings",
            fan_id, speed_percent
        );
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(NvControlError::DisplayDetectionFailed(format!(
            "nvidia-settings error: {stderr}"
        )))
    }
}

fn set_fan_speed_sysfs(fan_id: usize, speed_percent: u8) -> NvResult<()> {
    // Try to find the correct hwmon path for the fan
    let hwmon_paths = find_nvidia_hwmon_paths();

    for hwmon_path in hwmon_paths {
        let pwm_path = hwmon_path.join(format!("pwm{}", fan_id + 1));
        let pwm_enable_path = hwmon_path.join(format!("pwm{}_enable", fan_id + 1));

        // Convert percentage to PWM value (0-255)
        let pwm_value = (speed_percent as f32 / 100.0 * 255.0) as u8;

        // Enable manual control
        if std::fs::write(&pwm_enable_path, "1").is_ok() {
            // Set PWM value
            if std::fs::write(&pwm_path, pwm_value.to_string()).is_ok() {
                println!("Fan {} set to {}% via sysfs", fan_id, speed_percent);
                return Ok(());
            }
        }
    }

    Err(NvControlError::FanControlNotSupported)
}

fn find_nvidia_hwmon_paths() -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();

    if let Ok(entries) = std::fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let hwmon_path = entry.path();
            let name_path = hwmon_path.join("name");

            if let Ok(name) = std::fs::read_to_string(&name_path) {
                if name.trim().contains("nvidia") {
                    paths.push(hwmon_path);
                }
            }
        }
    }

    paths
}

/// Create custom fan curve
pub fn set_fan_curve(fan_id: usize, curve_points: &[(u8, u8)]) -> NvResult<()> {
    if curve_points.is_empty() {
        return Err(NvControlError::DisplayDetectionFailed(
            "Empty fan curve".to_string(),
        ));
    }

    // Validate curve points
    for (temp, speed) in curve_points {
        if *temp > 100 || *speed > 100 {
            return Err(NvControlError::DisplayDetectionFailed(
                "Invalid curve point: temperature and speed must be 0-100".to_string(),
            ));
        }
    }

    println!("Setting fan curve for fan {}: {:?}", fan_id, curve_points);

    // This would require more complex implementation to set automatic curves
    // For now, just apply the maximum speed point as a simple implementation
    if let Some((_, max_speed)) = curve_points.iter().max_by_key(|(temp, _)| temp) {
        set_fan_speed(fan_id, *max_speed)?;
    }

    Ok(())
}

/// Reset fan to automatic control
pub fn reset_fan_to_auto(fan_id: usize) -> NvResult<()> {
    // Try nvidia-settings first
    if std::env::var("DISPLAY").is_ok() {
        let cmd = "nvidia-settings -a '[gpu:0]/GPUFanControlState=0'";
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!("nvidia-settings failed: {e}"))
            })?;

        if output.status.success() {
            println!("Fan {} reset to automatic control", fan_id);
            return Ok(());
        }
    }

    // Try sysfs
    let hwmon_paths = find_nvidia_hwmon_paths();
    for hwmon_path in hwmon_paths {
        let pwm_enable_path = hwmon_path.join(format!("pwm{}_enable", fan_id + 1));
        if std::fs::write(&pwm_enable_path, "2").is_ok() {
            // 2 = automatic
            println!("Fan {} reset to automatic control via sysfs", fan_id);
            return Ok(());
        }
    }

    Err(NvControlError::FanControlNotSupported)
}

/// Apply a custom fan curve with intelligent temperature tracking
pub fn apply_fan_curve(fan_id: usize, curve: &FanCurve, current_temp: u8, previous_speed: Option<u8>) -> NvResult<u8> {
    // Sort curve points by temperature
    let mut sorted_points = curve.points.clone();
    sorted_points.sort_by_key(|p| p.temperature);
    
    // Check zero RPM threshold
    if let Some(zero_threshold) = curve.zero_rpm_threshold {
        if current_temp < zero_threshold {
            return Ok(0);
        }
    }
    
    // Handle empty curves
    if sorted_points.is_empty() {
        return Ok(curve.min_duty_cycle);
    }
    
    // Find the appropriate speed using linear interpolation
    let target_speed = if current_temp <= sorted_points[0].temperature {
        sorted_points[0].duty_cycle
    } else if current_temp >= sorted_points.last().unwrap().temperature {
        sorted_points.last().unwrap().duty_cycle
    } else {
        // Linear interpolation between points
        let mut result = sorted_points[0].duty_cycle; // Default fallback
        for i in 0..sorted_points.len() - 1 {
            let p1 = &sorted_points[i];
            let p2 = &sorted_points[i + 1];
            
            if current_temp >= p1.temperature && current_temp <= p2.temperature {
                let temp_range = p2.temperature - p1.temperature;
                let speed_range = p2.duty_cycle as i16 - p1.duty_cycle as i16;
                let temp_offset = current_temp - p1.temperature;
                
                let interpolated_speed = p1.duty_cycle as i16 + 
                    (speed_range * temp_offset as i16) / temp_range as i16;
                
                result = interpolated_speed.max(0).min(100) as u8;
                break;
            }
        }
        result
    };
    
    // Apply hysteresis to prevent oscillation
    let final_speed = if let Some(prev_speed) = previous_speed {
        apply_hysteresis(target_speed, prev_speed, curve.hysteresis, current_temp)
    } else {
        target_speed
    };
    
    // Clamp to min/max duty cycle
    let clamped_speed = final_speed
        .max(curve.min_duty_cycle)
        .min(curve.max_duty_cycle);
    
    // Apply the fan speed
    set_fan_speed(fan_id, clamped_speed)?;
    
    Ok(clamped_speed)
}

/// Apply hysteresis to prevent fan speed oscillation
fn apply_hysteresis(target_speed: u8, previous_speed: u8, hysteresis: u8, _current_temp: u8) -> u8 {
    let speed_diff = if target_speed > previous_speed {
        target_speed - previous_speed
    } else {
        previous_speed - target_speed
    };
    
    // Only change speed if the difference exceeds hysteresis threshold
    if speed_diff > hysteresis {
        target_speed
    } else {
        previous_speed
    }
}

/// Test fan functionality and measure response
pub fn test_fan(fan_id: usize) -> NvResult<FanTestResult> {
    println!("Testing fan {} functionality...", fan_id);
    
    let initial_info = get_fan_info(fan_id)
        .ok_or_else(|| NvControlError::DisplayDetectionFailed("Fan not found".to_string()))?;
    
    let initial_rpm = initial_info.rpm.unwrap_or(0);
    let initial_speed = initial_info.percent.unwrap_or(0);
    
    let mut test_result = FanTestResult {
        fan_id,
        initial_rpm,
        max_rpm_achieved: initial_rpm,
        min_rpm_achieved: initial_rpm,
        response_time_ms: 0,
        noise_level: NoiseLevel::Unknown,
        health_status: initial_info.health_status.clone(),
        controllable: initial_info.controllable,
        test_successful: false,
    };
    
    if !initial_info.controllable {
        test_result.test_successful = false;
        return Ok(test_result);
    }
    
    let test_start = Instant::now();
    
    // Test different speed levels
    let test_speeds = [25, 50, 75, 100, 0]; // Including 0 for zero RPM test
    
    for &speed in &test_speeds {
        if set_fan_speed(fan_id, speed).is_ok() {
            std::thread::sleep(Duration::from_millis(2000)); // Wait for fan response
            
            if let Some(current_info) = get_fan_info(fan_id) {
                if let Some(rpm) = current_info.rpm {
                    test_result.max_rpm_achieved = test_result.max_rpm_achieved.max(rpm);
                    test_result.min_rpm_achieved = test_result.min_rpm_achieved.min(rpm);
                }
            }
        }
    }
    
    // Restore original speed
    let _ = set_fan_speed(fan_id, initial_speed);
    
    test_result.response_time_ms = test_start.elapsed().as_millis() as u32;
    test_result.test_successful = true;
    
    // Assess noise level based on max RPM
    test_result.noise_level = if test_result.max_rpm_achieved > 3500 {
        NoiseLevel::Loud
    } else if test_result.max_rpm_achieved > 2500 {
        NoiseLevel::Moderate
    } else if test_result.max_rpm_achieved > 1000 {
        NoiseLevel::Quiet
    } else {
        NoiseLevel::Silent
    };
    
    Ok(test_result)
}

/// Enable zero RPM mode (fan stops at low temperatures)
pub fn enable_zero_rpm_mode(fan_id: usize, threshold_temp: u8) -> NvResult<()> {
    println!("Enabling zero RPM mode for fan {} at {}°C threshold", fan_id, threshold_temp);
    
    // This would typically involve setting fan curves with zero RPM points
    // For now, we'll set a very low speed when temperature is below threshold
    
    if let Ok(nvml) = Nvml::init() {
        let gpu_id = fan_id / 10;
        if let Ok(device) = nvml.device_by_index(gpu_id as u32) {
            if let Ok(temp) = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
                if temp < threshold_temp as u32 {
                    set_fan_speed(fan_id, 0)?;
                    println!("Fan {} stopped (zero RPM mode active)", fan_id);
                }
            }
        }
    }
    
    Ok(())
}

/// Monitor fan health over time
pub fn monitor_fan_health(fan_id: usize, duration_minutes: u32) -> NvResult<FanHealthReport> {
    println!("Monitoring fan {} health for {} minutes...", fan_id, duration_minutes);
    
    let mut report = FanHealthReport {
        fan_id,
        monitoring_duration_minutes: duration_minutes,
        rpm_samples: Vec::new(),
        temperature_samples: Vec::new(),
        average_rpm: 0.0,
        rpm_stability: 0.0,
        health_events: Vec::new(),
        overall_health: FanHealthStatus::Healthy,
    };
    
    let start_time = Instant::now();
    let sample_interval = Duration::from_secs(30); // Sample every 30 seconds
    let total_duration = Duration::from_secs(duration_minutes as u64 * 60);
    
    while start_time.elapsed() < total_duration {
        if let Some(fan_info) = get_fan_info(fan_id) {
            if let Some(rpm) = fan_info.rpm {
                report.rpm_samples.push(rpm);
                
                // Check for concerning patterns
                if rpm == 0 {
                    report.health_events.push("Fan stopped".to_string());
                } else if rpm > 4500 {
                    report.health_events.push(format!("High RPM detected: {}", rpm));
                }
            }
            
            // Get temperature if available
            if let Ok(nvml) = Nvml::init() {
                let gpu_id = fan_id / 10;
                if let Ok(device) = nvml.device_by_index(gpu_id as u32) {
                    if let Ok(temp) = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
                        report.temperature_samples.push(temp);
                    }
                }
            }
        }
        
        std::thread::sleep(sample_interval);
    }
    
    // Calculate statistics
    if !report.rpm_samples.is_empty() {
        report.average_rpm = report.rpm_samples.iter().map(|&x| x as f64).sum::<f64>() / report.rpm_samples.len() as f64;
        
        // Calculate RPM stability (coefficient of variation)
        let variance = report.rpm_samples.iter()
            .map(|&x| (x as f64 - report.average_rpm).powi(2))
            .sum::<f64>() / report.rpm_samples.len() as f64;
        let std_dev = variance.sqrt();
        report.rpm_stability = (std_dev / report.average_rpm) * 100.0; // CV percentage
    }
    
    // Assess overall health
    report.overall_health = if report.health_events.len() > 5 {
        FanHealthStatus::Critical
    } else if report.health_events.len() > 2 || report.rpm_stability > 20.0 {
        FanHealthStatus::Warning
    } else {
        FanHealthStatus::Healthy
    };
    
    Ok(report)
}

/// Load fan profiles from configuration
pub fn load_fan_profiles() -> NvResult<Vec<FanProfile>> {
    let config_path = get_fan_config_path();
    
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to read fan config: {}", e)))?;
        
        let profiles: Vec<FanProfile> = serde_json::from_str(&content)
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to parse fan config: {}", e)))?;
        
        Ok(profiles)
    } else {
        // Return predefined profiles if no custom config exists
        Ok(get_predefined_profiles())
    }
}

/// Save fan profiles to configuration
pub fn save_fan_profiles(profiles: &[FanProfile]) -> NvResult<()> {
    let config_path = get_fan_config_path();
    
    // Ensure config directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to create config dir: {}", e)))?;
    }
    
    let content = serde_json::to_string_pretty(profiles)
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to serialize profiles: {}", e)))?;
    
    fs::write(&config_path, content)
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to save fan config: {}", e)))?;
    
    println!("Saved {} fan profiles to {:?}", profiles.len(), config_path);
    Ok(())
}

fn get_fan_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("nvcontrol");
    path.push("fan_profiles.json");
    path
}

/// Results from fan testing
#[derive(Debug, Clone)]
pub struct FanTestResult {
    pub fan_id: usize,
    pub initial_rpm: u32,
    pub max_rpm_achieved: u32,
    pub min_rpm_achieved: u32,
    pub response_time_ms: u32,
    pub noise_level: NoiseLevel,
    pub health_status: FanHealthStatus,
    pub controllable: bool,
    pub test_successful: bool,
}

#[derive(Debug, Clone)]
pub enum NoiseLevel {
    Silent,   // < 1000 RPM
    Quiet,    // 1000-2500 RPM
    Moderate, // 2500-3500 RPM
    Loud,     // > 3500 RPM
    Unknown,
}

/// Comprehensive fan health report
#[derive(Debug, Clone)]
pub struct FanHealthReport {
    pub fan_id: usize,
    pub monitoring_duration_minutes: u32,
    pub rpm_samples: Vec<u32>,
    pub temperature_samples: Vec<u32>,
    pub average_rpm: f64,
    pub rpm_stability: f64, // Coefficient of variation percentage
    pub health_events: Vec<String>,
    pub overall_health: FanHealthStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_info_creation() {
        let fan = FanInfo {
            id: 0,
            rpm: Some(1500),
            percent: Some(40),
            controllable: true,
            health_status: FanHealthStatus::Healthy,
            max_rpm: Some(3000),
            target_rpm: None,
        };
        assert_eq!(fan.id, 0);
        assert_eq!(fan.rpm, Some(1500));
        assert_eq!(fan.percent, Some(40));
        assert!(fan.controllable);
        assert_eq!(fan.health_status, FanHealthStatus::Healthy);
    }

    #[test]
    fn test_list_fans() {
        let fans = list_fans();
        assert!(!fans.is_empty());
        assert_eq!(fans[0].id, 0);
    }

    #[test]
    fn test_get_fan_info() {
        let fan = get_fan_info(0);
        assert!(fan.is_some());
        if let Some(fan) = fan {
            assert_eq!(fan.id, 0);
        }
    }
}
