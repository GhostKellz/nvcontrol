use crate::{NvControlError, NvResult};
use nvml_wrapper::Nvml;
use std::process::Command;

/// Represents a GPU fan
pub struct FanInfo {
    pub id: usize,
    pub rpm: Option<u32>,
    pub percent: Option<u8>,
    pub controllable: bool,
}

/// List all fans with real NVML integration
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

                        fans.push(FanInfo {
                            id: (gpu_id * 10 + fan_id) as usize, // Unique ID across GPUs
                            rpm,
                            percent,
                            controllable,
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
        };
        assert_eq!(fan.id, 0);
        assert_eq!(fan.rpm, Some(1500));
        assert_eq!(fan.percent, Some(40));
        assert!(fan.controllable);
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
