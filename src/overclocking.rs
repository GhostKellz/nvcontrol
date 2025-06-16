use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverclockProfile {
    pub name: String,
    pub gpu_clock_offset: i32,    // MHz offset from base
    pub memory_clock_offset: i32, // MHz offset from base
    pub voltage_offset: i32,      // mV offset (if supported)
    pub power_limit: u8,          // Percentage (50-120%)
    pub temp_limit: u8,           // Temperature limit in Celsius
    pub fan_curve: Vec<(u8, u8)>, // (temp, fan_speed) pairs
}

impl Default for OverclockProfile {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            gpu_clock_offset: 0,
            memory_clock_offset: 0,
            voltage_offset: 0,
            power_limit: 100,
            temp_limit: 83,
            fan_curve: vec![
                (30, 20),
                (40, 30),
                (50, 40),
                (60, 50),
                (70, 70),
                (80, 85),
                (85, 100),
            ],
        }
    }
}

#[derive(Debug)]
pub struct GpuCapabilities {
    pub max_gpu_clock_offset: i32,
    pub min_gpu_clock_offset: i32,
    pub max_memory_clock_offset: i32,
    pub min_memory_clock_offset: i32,
    pub supports_voltage_control: bool,
    pub max_power_limit: u8,
    pub min_power_limit: u8,
    pub default_temp_limit: u8,
}

pub fn get_gpu_capabilities() -> NvResult<GpuCapabilities> {
    // Try to get capabilities via nvidia-ml
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=power.max_limit,power.min_limit,temperature.gpu"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.trim().split(',').collect();

            if parts.len() >= 3 {
                return Ok(GpuCapabilities {
                    max_gpu_clock_offset: 200, // Conservative defaults
                    min_gpu_clock_offset: -200,
                    max_memory_clock_offset: 1000,
                    min_memory_clock_offset: -500,
                    supports_voltage_control: false, // Most GPUs don't allow this
                    max_power_limit: parts[0].trim().parse().unwrap_or(120),
                    min_power_limit: parts[1].trim().parse().unwrap_or(50),
                    default_temp_limit: 83,
                });
            }
        }
    }

    // Fallback to safe defaults
    Ok(GpuCapabilities {
        max_gpu_clock_offset: 150,
        min_gpu_clock_offset: -150,
        max_memory_clock_offset: 500,
        min_memory_clock_offset: -200,
        supports_voltage_control: false,
        max_power_limit: 120,
        min_power_limit: 50,
        default_temp_limit: 83,
    })
}

pub fn apply_overclock_profile(profile: &OverclockProfile) -> NvResult<()> {
    println!("Applying overclock profile: {}", profile.name);

    // Try nvidia-settings first (X11)
    if std::env::var("DISPLAY").is_ok() {
        apply_overclock_x11(profile)?;
    } else {
        // Try direct sysfs manipulation for Wayland
        apply_overclock_sysfs(profile)?;
    }

    Ok(())
}

fn apply_overclock_x11(profile: &OverclockProfile) -> NvResult<()> {
    let commands = vec![
        format!(
            "nvidia-settings -a '[gpu:0]/GPUGraphicsClockOffset[3]={}'",
            profile.gpu_clock_offset
        ),
        format!(
            "nvidia-settings -a '[gpu:0]/GPUMemoryTransferRateOffset[3]={}'",
            profile.memory_clock_offset
        ),
        format!("nvidia-settings -a '[gpu:0]/GPUPowerMizerMode=1'"), // Performance mode
        format!(
            "nvidia-settings -a '[gpu:0]/GPUTargetFanSpeed={}'",
            profile
                .fan_curve
                .last()
                .map(|(_, speed)| *speed)
                .unwrap_or(50)
        ),
    ];

    for cmd in commands {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!(
                    "Failed to execute nvidia-settings: {e}"
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Warning: nvidia-settings command failed: {stderr}");
        }
    }

    Ok(())
}

fn apply_overclock_sysfs(profile: &OverclockProfile) -> NvResult<()> {
    // For Wayland, try direct sysfs manipulation
    let _sysfs_paths = [
        "/sys/class/drm/card0/device/hwmon/hwmon0/power1_cap",
        "/sys/class/drm/card0/device/hwmon/hwmon0/pwm1_enable",
        "/sys/kernel/debug/dri/0/amdgpu_pm_info", // For testing
    ];

    // Note: This is a placeholder - actual implementation would need
    // proper detection of GPU vendor and appropriate sysfs paths
    println!("Wayland overclock via sysfs not yet implemented");
    println!(
        "Would apply: GPU +{}MHz, Memory +{}MHz, Power {}%",
        profile.gpu_clock_offset, profile.memory_clock_offset, profile.power_limit
    );

    Ok(())
}

pub fn create_stress_test(duration_minutes: u32) -> NvResult<()> {
    println!("Starting GPU stress test for {} minutes", duration_minutes);

    // Pre-format the duration string to avoid borrowing issues
    let duration_arg = format!("--time={}", duration_minutes * 60);

    // Try different stress testing tools
    let stress_tools = vec![
        ("glxgears", vec!["-fullscreen"]),
        ("glmark2", vec!["--fullscreen"]),
        ("vkmark", vec!["--fullscreen"]),
        ("furmark", vec!["--fullscreen", &duration_arg]),
    ];

    for (tool, args) in stress_tools {
        if std::process::Command::new(tool)
            .arg("--help")
            .output()
            .is_ok()
        {
            println!("Using {} for stress testing", tool);
            let mut cmd = std::process::Command::new(tool);
            cmd.args(args);

            match cmd.spawn() {
                Ok(mut child) => {
                    println!("Stress test started. Monitor temperatures!");
                    // Don't wait for completion in this example
                    let _ = child.wait();
                    return Ok(());
                }
                Err(e) => eprintln!("Failed to start {}: {}", tool, e),
            }
        }
    }

    Err(NvControlError::DisplayDetectionFailed(
        "No stress testing tools found. Install glmark2, vkmark, or furmark".to_string(),
    ))
}

pub fn get_memory_timings() -> NvResult<HashMap<String, String>> {
    let mut timings = HashMap::new();

    // Try to read memory timings via nvidia-ml or sysfs
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["--query-gpu=memory.total,memory.used,clocks.mem"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.trim().split(',').collect();

            if parts.len() >= 3 {
                timings.insert(
                    "Total Memory".to_string(),
                    format!("{} MB", parts[0].trim()),
                );
                timings.insert("Used Memory".to_string(), format!("{} MB", parts[1].trim()));
                timings.insert(
                    "Memory Clock".to_string(),
                    format!("{} MHz", parts[2].trim()),
                );
            }
        }
    }

    // Add placeholder advanced timings (would need specialized tools to read)
    timings.insert("CAS Latency".to_string(), "Unknown".to_string());
    timings.insert("tRCD".to_string(), "Unknown".to_string());
    timings.insert("tRP".to_string(), "Unknown".to_string());
    timings.insert("tRAS".to_string(), "Unknown".to_string());

    Ok(timings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile() {
        let profile = OverclockProfile::default();
        assert_eq!(profile.name, "Default");
        assert_eq!(profile.gpu_clock_offset, 0);
        assert_eq!(profile.power_limit, 100);
    }

    #[test]
    fn test_capabilities() {
        let caps = get_gpu_capabilities().unwrap();
        assert!(caps.max_gpu_clock_offset > 0);
        assert!(caps.min_gpu_clock_offset < 0);
    }
}
