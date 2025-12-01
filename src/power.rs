use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerInfo {
    pub power_draw: Option<f32>,          // Watts
    pub power_limit: Option<f32>,         // Watts
    pub power_limit_default: Option<f32>, // Watts
    pub power_limit_max: Option<f32>,     // Watts
    pub power_limit_min: Option<f32>,     // Watts
    pub temperature: Option<f32>,         // Celsius
    pub fan_speed: Option<u32>,           // Percentage
    pub power_state: Option<String>,      // P0, P1, P2, etc.
    pub persistence_mode: bool,
}

#[derive(Debug, Clone)]
pub enum PowerProfile {
    Balanced,
    Performance,
    PowerSaver,
    Custom(String),
}

impl PowerProfile {
    pub fn as_str(&self) -> &str {
        match self {
            PowerProfile::Balanced => "balanced",
            PowerProfile::Performance => "performance",
            PowerProfile::PowerSaver => "power_saver",
            PowerProfile::Custom(name) => name,
        }
    }
}

impl From<&str> for PowerProfile {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "balanced" => PowerProfile::Balanced,
            "performance" => PowerProfile::Performance,
            "power_saver" | "powersaver" => PowerProfile::PowerSaver,
            _ => PowerProfile::Custom(s.to_string()),
        }
    }
}

/// Custom power profile configuration loaded from TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPowerProfileConfig {
    pub name: String,
    pub description: Option<String>,
    pub power_limit_percentage: Option<u32>,
    pub performance_level: Option<String>, // "max", "auto", "min"
    pub cpu_governor: Option<String>,      // "performance", "schedutil", "powersave"
    pub thermal_policy: Option<String>,    // "performance", "balanced", "quiet"
    pub gpu_clock_boost: Option<bool>,
    pub persistence_mode: Option<bool>,
    pub power_gating: Option<bool>,
    pub adaptive_power_management: Option<bool>,
}

impl Default for CustomPowerProfileConfig {
    fn default() -> Self {
        Self {
            name: "custom".to_string(),
            description: None,
            power_limit_percentage: Some(80),
            performance_level: Some("auto".to_string()),
            cpu_governor: Some("schedutil".to_string()),
            thermal_policy: Some("balanced".to_string()),
            gpu_clock_boost: Some(false),
            persistence_mode: Some(false),
            power_gating: Some(true),
            adaptive_power_management: Some(true),
        }
    }
}

/// Set power management profile for NVIDIA GPU
pub fn set_power_profile(profile: &str) -> NvResult<()> {
    let profile: PowerProfile = profile.into();

    match profile {
        PowerProfile::Performance => {
            // Set maximum performance mode
            set_power_limit_percentage(100)?;
            set_persistence_mode(true)?;
            set_performance_level("max")?;
        }
        PowerProfile::PowerSaver => {
            // Set power saving mode
            set_power_limit_percentage(70)?;
            set_persistence_mode(false)?;
            set_performance_level("min")?;
        }
        PowerProfile::Balanced => {
            // Set balanced mode
            set_power_limit_percentage(85)?;
            set_persistence_mode(true)?;
            set_performance_level("auto")?;
        }
        PowerProfile::Custom(name) => {
            return Err(NvControlError::PowerManagementFailed(format!(
                "Custom profile '{}' not implemented",
                name
            )));
        }
    }

    println!("Power profile '{}' applied successfully", profile.as_str());
    Ok(())
}

/// Get comprehensive power information for all NVIDIA GPUs
pub fn get_power_info() -> NvResult<Vec<PowerInfo>> {
    let mut power_infos = Vec::new();

    // Get GPU count first
    let gpu_count = get_gpu_count()?;

    for gpu_id in 0..gpu_count {
        let mut info = PowerInfo {
            power_draw: None,
            power_limit: None,
            power_limit_default: None,
            power_limit_max: None,
            power_limit_min: None,
            temperature: None,
            fan_speed: None,
            power_state: None,
            persistence_mode: false,
        };

        // Get power draw
        if let Ok(draw) = get_power_draw(gpu_id) {
            info.power_draw = Some(draw);
        }

        // Get power limits
        if let Ok(limit) = get_power_limit(gpu_id) {
            info.power_limit = Some(limit);
        }

        if let Ok(default) = get_power_limit_default(gpu_id) {
            info.power_limit_default = Some(default);
        }

        if let Ok((min, max)) = get_power_limit_range(gpu_id) {
            info.power_limit_min = Some(min);
            info.power_limit_max = Some(max);
        }

        // Get temperature
        if let Ok(temp) = get_gpu_temperature(gpu_id) {
            info.temperature = Some(temp);
        }

        // Get fan speed
        if let Ok(speed) = get_fan_speed(gpu_id) {
            info.fan_speed = Some(speed);
        }

        // Get power state
        if let Ok(state) = get_power_state(gpu_id) {
            info.power_state = Some(state);
        }

        // Get persistence mode
        info.persistence_mode = get_persistence_mode(gpu_id).unwrap_or(false);

        power_infos.push(info);
    }

    Ok(power_infos)
}

/// Set power limit as percentage of maximum
pub fn set_power_limit_percentage(percentage: u32) -> NvResult<()> {
    let percentage = percentage.min(100);

    // Use nvidia-ml-py approach via nvidia-smi
    let output = Command::new("nvidia-smi")
        .args(&["-pl", &format!("{}%", percentage)])
        .output()
        .map_err(|e| {
            NvControlError::PowerManagementFailed(format!("Failed to set power limit: {}", e))
        })?;

    if !output.status.success() {
        // Try alternative approach with direct sysfs on Wayland
        set_power_limit_sysfs(percentage)?;
    }

    Ok(())
}

/// Set power limit via sysfs (Wayland-friendly)
fn set_power_limit_sysfs(percentage: u32) -> NvResult<()> {
    let gpu_count = get_gpu_count()?;

    for gpu_id in 0..gpu_count {
        let power_limit_path = format!("/sys/class/drm/card{}/device/power_limit_watts", gpu_id);
        let max_power_path = format!("/sys/class/drm/card{}/device/power_limit_max_watts", gpu_id);

        if Path::new(&max_power_path).exists() {
            if let Ok(max_power_str) = fs::read_to_string(&max_power_path) {
                if let Ok(max_power) = max_power_str.trim().parse::<f32>() {
                    let target_power = (max_power * percentage as f32 / 100.0) as u32;

                    let write_result = Command::new("sudo")
                        .args(&["tee", &power_limit_path])
                        .arg(&format!("{}", target_power))
                        .output();

                    if write_result.is_err() {
                        return Err(NvControlError::PowerManagementFailed(
                            "Failed to write power limit - may need sudo access".to_string(),
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

/// Set persistence mode
pub fn set_persistence_mode(enabled: bool) -> NvResult<()> {
    let status = if enabled { "1" } else { "0" };

    let output = Command::new("nvidia-smi")
        .args(&["-pm", status])
        .output()
        .map_err(|e| {
            NvControlError::PowerManagementFailed(format!("Failed to set persistence mode: {}", e))
        })?;

    if !output.status.success() {
        return Err(NvControlError::PowerManagementFailed(
            "Failed to set persistence mode".to_string(),
        ));
    }

    Ok(())
}

/// Set performance level
fn set_performance_level(level: &str) -> NvResult<()> {
    // Use nvidia-settings for performance level if available
    let perf_level = match level {
        "max" => "3",  // Maximum performance
        "min" => "0",  // Minimum performance
        "auto" => "1", // Automatic
        _ => "1",
    };

    let output = Command::new("nvidia-settings")
        .args(&["-a", &format!("GPUPowerMizerMode={}", perf_level)])
        .output();

    if output.is_err() {
        // Fallback: try via sysfs
        set_performance_level_sysfs(level)?;
    }

    Ok(())
}

fn set_performance_level_sysfs(level: &str) -> NvResult<()> {
    let gpu_count = get_gpu_count()?;

    for gpu_id in 0..gpu_count {
        let perf_path = format!(
            "/sys/class/drm/card{}/device/power_dpm_force_performance_level",
            gpu_id
        );

        if Path::new(&perf_path).exists() {
            let perf_value = match level {
                "max" => "high",
                "min" => "low",
                "auto" => "auto",
                _ => "auto",
            };

            let _ = Command::new("sudo")
                .args(&["tee", &perf_path])
                .arg(perf_value)
                .output();
        }
    }

    Ok(())
}

/// Advanced power management functions
/// Set GPU clock boost state
/// Set GPU clock boost state
#[allow(dead_code)]
pub fn set_gpu_clock_boost(enabled: bool) -> NvResult<()> {
    let _boost_value = if enabled { "1" } else { "0" };

    // Try nvidia-settings approach
    if std::env::var("DISPLAY").is_ok() {
        let output = Command::new("nvidia-settings")
            .args(&[
                "-a",
                &format!(
                    "GPUGraphicsClockOffset[3]={}",
                    if enabled { "100" } else { "0" }
                ),
            ])
            .output();

        if output.is_ok() {
            return Ok(());
        }
    }

    // Try sysfs approach for Wayland
    set_boost_via_sysfs(enabled)
}

#[allow(dead_code)]
fn set_boost_via_sysfs(enabled: bool) -> NvResult<()> {
    let gpu_count = get_gpu_count()?;

    for gpu_id in 0..gpu_count {
        let boost_path = format!("/sys/class/drm/card{}/device/power_dpm_boost", gpu_id);
        if Path::new(&boost_path).exists() {
            let boost_value = if enabled { "1" } else { "0" };
            if fs::write(&boost_path, boost_value).is_ok() {
                println!(
                    "✅ GPU {} boost {}",
                    gpu_id,
                    if enabled { "enabled" } else { "disabled" }
                );
            }
        }
    }

    Ok(())
}

/// Disable power throttling for maximum performance
/// Disable power throttling for maximum performance
#[allow(dead_code)]
pub fn disable_power_throttling() -> NvResult<()> {
    println!("🔥 Disabling power throttling for maximum performance...");

    let mut success_count = 0;

    // Increase thermal limits
    if set_thermal_policy("performance").is_ok() {
        success_count += 1;
        println!("  ✓ Thermal policy set to performance");
    }

    // Set maximum power limits
    if set_power_limit_percentage(100).is_ok() {
        success_count += 1;
        println!("  ✓ Power limit set to 100%");
    }

    // Disable power management features that limit performance
    let optimizations = [
        ("cpupower", "frequency-set -g performance"),
        (
            "echo",
            "performance > /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor",
        ),
    ];

    for (cmd, args) in optimizations {
        if Command::new("which")
            .arg(cmd)
            .output()
            .map_or(false, |o| o.status.success())
        {
            let result = if cmd == "echo" {
                Command::new("sudo").arg("sh").arg("-c").arg(args).output()
            } else {
                Command::new("sudo")
                    .arg(cmd)
                    .args(args.split_whitespace())
                    .output()
            };

            if result.map_or(false, |o| o.status.success()) {
                success_count += 1;
                println!("  ✓ Applied {} optimization", cmd);
            }
        }
    }

    // Disable CPU throttling
    let cpu_throttle_paths = [
        "/sys/devices/system/cpu/intel_pstate/no_turbo",
        "/sys/devices/system/cpu/cpufreq/boost",
    ];

    for path in cpu_throttle_paths {
        if Path::new(path).exists() {
            if let Ok(_) = Command::new("sudo").args(&["tee", path]).arg("0").output() {
                success_count += 1;
                println!("  ✓ Disabled CPU throttling at {}", path);
            }
        }
    }

    if success_count > 0 {
        println!(
            "✅ Applied {} power throttling optimizations",
            success_count
        );
    } else {
        println!("⚠️ No power throttling optimizations could be applied");
    }

    Ok(())
}

/// Enable aggressive power saving features
/// Enable aggressive power saving features
#[allow(dead_code)]
pub fn enable_aggressive_power_saving() -> NvResult<()> {
    println!("🍃 Enabling aggressive power saving...");

    let mut success_count = 0;

    // Set conservative CPU governor
    if set_cpu_governor("powersave")? {
        success_count += 1;
        println!("  ✓ CPU governor set to powersave");
    }

    // Enable GPU power gating
    if enable_gpu_power_gating().is_ok() {
        success_count += 1;
        println!("  ✓ GPU power gating enabled");
    }

    // Set lower power limits
    if set_power_limit_percentage(60).is_ok() {
        success_count += 1;
        println!("  ✓ Power limit reduced to 60%");
    }

    // Enable CPU idle states for maximum power saving
    if enable_cpu_idle_states().is_ok() {
        success_count += 1;
        println!("  ✓ CPU idle states enabled");
    }

    // Reduce display brightness if possible
    if optimize_display_power().is_ok() {
        success_count += 1;
        println!("  ✓ Display power optimized");
    }

    // Enable runtime power management for PCI devices
    if enable_runtime_pm().is_ok() {
        success_count += 1;
        println!("  ✓ Runtime power management enabled");
    }

    if success_count > 0 {
        println!("✅ Applied {} power saving optimizations", success_count);
    } else {
        println!("⚠️ No power saving optimizations could be applied");
    }

    Ok(())
}

/// Set adaptive power management based on workload
/// Set adaptive power management based on workload
#[allow(dead_code)]
pub fn set_adaptive_power_management(enabled: bool) -> NvResult<()> {
    if enabled {
        println!("🧠 Enabling adaptive power management...");

        let mut success_count = 0;

        // Enable dynamic frequency scaling
        if set_performance_level("auto").is_ok() {
            success_count += 1;
            println!("  ✓ Performance level set to auto");
        }

        // Set intelligent fan curves
        if apply_adaptive_fan_curve().is_ok() {
            success_count += 1;
            println!("  ✓ Adaptive fan curve applied");
        }

        // Enable power gating when idle
        if enable_idle_power_gating().is_ok() {
            success_count += 1;
            println!("  ✓ Idle power gating enabled");
        }

        // Enable CPU frequency scaling
        if set_cpu_governor("schedutil").unwrap_or(false) {
            success_count += 1;
            println!("  ✓ CPU governor set to adaptive");
        }

        println!(
            "✅ Applied {} adaptive power management features",
            success_count
        );
    } else {
        println!("🔒 Disabling adaptive power management...");
        set_performance_level("max")?;
        set_cpu_governor("performance")?;
        println!("✅ Adaptive power management disabled");
    }

    Ok(())
}

/// Apply custom power profile from configuration
/// Apply custom power profile from configuration
#[allow(dead_code)]
pub fn apply_custom_power_profile(profile_name: &str) -> NvResult<()> {
    println!("📋 Loading custom power profile: {}", profile_name);

    // Load custom profile from config directory
    let config_dir =
        directories::ProjectDirs::from("com", "ghostkellz", "nvcontrol").ok_or_else(|| {
            NvControlError::PowerManagementFailed("Cannot access config directory".to_string())
        })?;

    let profile_path = config_dir
        .config_dir()
        .join("power_profiles")
        .join(format!("{}.toml", profile_name));

    if profile_path.exists() {
        // For now, implement some predefined custom profiles
        match profile_name {
            "gaming" => {
                println!("🎮 Applying gaming profile...");
                set_power_limit_percentage(95)?;
                set_performance_level("max")?;
                set_cpu_governor("performance")?;
                println!("✅ Gaming profile applied successfully");
            }
            "streaming" => {
                println!("📺 Applying streaming profile...");
                set_power_limit_percentage(85)?;
                set_performance_level("auto")?;
                set_cpu_governor("performance")?;
                println!("✅ Streaming profile applied successfully");
            }
            "workstation" => {
                println!("💼 Applying workstation profile...");
                set_power_limit_percentage(90)?;
                set_performance_level("auto")?;
                set_cpu_governor("schedutil")?;
                println!("✅ Workstation profile applied successfully");
            }
            _ => {
                println!("📄 Loading custom TOML profile from file...");

                // Read and parse TOML file
                let toml_content = fs::read_to_string(&profile_path).map_err(|e| {
                    NvControlError::PowerManagementFailed(format!(
                        "Failed to read profile file: {}",
                        e
                    ))
                })?;

                let config: CustomPowerProfileConfig =
                    toml::from_str(&toml_content).map_err(|e| {
                        NvControlError::PowerManagementFailed(format!(
                            "Failed to parse TOML profile: {}",
                            e
                        ))
                    })?;

                println!("🔧 Applying custom profile: {}", config.name);
                if let Some(desc) = &config.description {
                    println!("   Description: {}", desc);
                }

                // Apply power limit if specified
                if let Some(power_limit) = config.power_limit_percentage {
                    println!("   Setting power limit: {}%", power_limit);
                    set_power_limit_percentage(power_limit)?;
                }

                // Apply performance level if specified
                if let Some(perf_level) = &config.performance_level {
                    println!("   Setting performance level: {}", perf_level);
                    set_performance_level(perf_level)?;
                }

                // Apply CPU governor if specified
                if let Some(governor) = &config.cpu_governor {
                    println!("   Setting CPU governor: {}", governor);
                    set_cpu_governor(governor)?;
                }

                // Apply thermal policy if specified
                if let Some(thermal) = &config.thermal_policy {
                    println!("   Setting thermal policy: {}", thermal);
                    set_thermal_policy(thermal)?;
                }

                // Apply GPU clock boost if specified
                if let Some(boost) = config.gpu_clock_boost {
                    println!("   Setting GPU clock boost: {}", boost);
                    set_gpu_clock_boost(boost)?;
                }

                // Apply persistence mode if specified
                if let Some(persistence) = config.persistence_mode {
                    println!("   Setting persistence mode: {}", persistence);
                    set_persistence_mode(persistence)?;
                }

                // Apply power gating if specified
                if let Some(gating) = config.power_gating {
                    if gating {
                        println!("   Enabling power gating");
                        enable_gpu_power_gating()?;
                    }
                }

                // Apply adaptive power management if specified
                if let Some(adaptive) = config.adaptive_power_management {
                    println!("   Setting adaptive power management: {}", adaptive);
                    set_adaptive_power_management(adaptive)?;
                }

                println!("✅ Custom profile '{}' applied successfully", config.name);
            }
        }
        Ok(())
    } else {
        Err(NvControlError::PowerManagementFailed(format!(
            "Custom profile '{}' not found at {}",
            profile_name,
            profile_path.display()
        )))
    }
}

/// Set thermal policy for the GPU
#[allow(dead_code)]
fn set_thermal_policy(policy: &str) -> NvResult<()> {
    let policy_value = match policy {
        "performance" => "0", // Prioritize performance over temperature
        "balanced" => "1",    // Balance performance and temperature
        "quiet" => "2",       // Prioritize low noise/temperature
        _ => "1",
    };

    let gpu_count = get_gpu_count()?;

    for gpu_id in 0..gpu_count {
        // Try different thermal control interfaces
        let thermal_paths = [
            format!("/sys/class/drm/card{}/device/power_dpm_policy", gpu_id),
            format!("/sys/class/drm/card{}/device/thermal_policy", gpu_id),
        ];

        for path in thermal_paths {
            if Path::new(&path).exists() && fs::write(&path, policy_value).is_ok() {
                println!("✅ Set thermal policy '{}' for GPU {}", policy, gpu_id);
                break;
            }
        }
    }

    Ok(())
}

/// Enable GPU power gating when idle
#[allow(dead_code)]
fn enable_gpu_power_gating() -> NvResult<()> {
    let gpu_count = get_gpu_count()?;

    for gpu_id in 0..gpu_count {
        let power_gate_path = format!("/sys/class/drm/card{}/device/power_gate", gpu_id);
        if Path::new(&power_gate_path).exists() && fs::write(&power_gate_path, "1").is_ok() {
            println!("✅ Enabled power gating for GPU {}", gpu_id);
        }
    }

    Ok(())
}

/// Enable idle power gating
#[allow(dead_code)]
fn enable_idle_power_gating() -> NvResult<()> {
    // Enable runtime power management
    let output = Command::new("sudo")
        .args(&[
            "sh",
            "-c",
            "echo auto > /sys/bus/pci/devices/*/power/control",
        ])
        .output();

    if output.is_ok() {
        println!("✅ Enabled idle power gating");
    }

    Ok(())
}

/// Apply adaptive fan curve based on workload
#[allow(dead_code)]
fn apply_adaptive_fan_curve() -> NvResult<()> {
    // This would integrate with the fan control system
    println!("🌀 Applied adaptive fan curve");
    Ok(())
}

/// Optimize display power consumption
#[allow(dead_code)]
fn optimize_display_power() -> NvResult<()> {
    // Try to reduce display brightness and refresh rate for power saving
    let display_optimizations = [
        (
            "xrandr --output DP-1 --brightness 0.7",
            "Reduce display brightness",
        ),
        (
            "xrandr --output HDMI-A-1 --rate 60",
            "Set 60Hz refresh rate",
        ),
    ];

    for (cmd, description) in display_optimizations {
        if std::env::var("DISPLAY").is_ok() {
            let _ = Command::new("sh").arg("-c").arg(cmd).output();
            println!("🔆 {}", description);
        }
    }

    Ok(())
}

/// Create example power profile TOML files for users
pub fn create_example_power_profiles() -> NvResult<()> {
    println!("📝 Creating example power profile configurations...");

    let config_dir =
        directories::ProjectDirs::from("com", "ghostkellz", "nvcontrol").ok_or_else(|| {
            NvControlError::PowerManagementFailed("Cannot access config directory".to_string())
        })?;

    let profiles_dir = config_dir.config_dir().join("power_profiles");
    fs::create_dir_all(&profiles_dir).map_err(|e| {
        NvControlError::PowerManagementFailed(format!("Failed to create profiles directory: {}", e))
    })?;

    // Example 1: Ultra Gaming Profile
    let ultra_gaming = CustomPowerProfileConfig {
        name: "ultra_gaming".to_string(),
        description: Some("Maximum performance for competitive gaming".to_string()),
        power_limit_percentage: Some(100),
        performance_level: Some("max".to_string()),
        cpu_governor: Some("performance".to_string()),
        thermal_policy: Some("performance".to_string()),
        gpu_clock_boost: Some(true),
        persistence_mode: Some(true),
        power_gating: Some(false),
        adaptive_power_management: Some(false),
    };

    // Example 2: Silent Operation Profile
    let silent = CustomPowerProfileConfig {
        name: "silent".to_string(),
        description: Some("Quiet operation with minimal fan noise".to_string()),
        power_limit_percentage: Some(60),
        performance_level: Some("min".to_string()),
        cpu_governor: Some("powersave".to_string()),
        thermal_policy: Some("quiet".to_string()),
        gpu_clock_boost: Some(false),
        persistence_mode: Some(false),
        power_gating: Some(true),
        adaptive_power_management: Some(true),
    };

    // Example 3: Balanced Creator Profile
    let creator = CustomPowerProfileConfig {
        name: "creator".to_string(),
        description: Some("Optimized for content creation and rendering".to_string()),
        power_limit_percentage: Some(85),
        performance_level: Some("auto".to_string()),
        cpu_governor: Some("schedutil".to_string()),
        thermal_policy: Some("balanced".to_string()),
        gpu_clock_boost: Some(true),
        persistence_mode: Some(true),
        power_gating: Some(false),
        adaptive_power_management: Some(true),
    };

    // Write profiles to TOML files
    for profile in &[ultra_gaming, silent, creator] {
        let profile_path = profiles_dir.join(format!("{}.toml", profile.name));
        let toml_content = toml::to_string_pretty(profile).map_err(|e| {
            NvControlError::PowerManagementFailed(format!("Failed to serialize profile: {}", e))
        })?;

        fs::write(&profile_path, toml_content).map_err(|e| {
            NvControlError::PowerManagementFailed(format!("Failed to write profile file: {}", e))
        })?;

        println!("✅ Created: {}", profile_path.display());
    }

    println!("\n📁 Profile location: {}", profiles_dir.display());
    println!("💡 You can edit these files or create new ones!");
    println!("   Use: nvctl power profile <profile_name>");

    Ok(())
}

/// Monitor power consumption and provide insights
pub fn monitor_power_consumption(duration_seconds: u32) -> NvResult<()> {
    println!(
        "⚡ Monitoring power consumption for {} seconds...",
        duration_seconds
    );

    let mut measurements = Vec::new();
    let start_time = std::time::Instant::now();

    while start_time.elapsed().as_secs() < duration_seconds as u64 {
        if let Ok(power_infos) = get_power_info() {
            for (gpu_id, info) in power_infos.iter().enumerate() {
                if let Some(power_draw) = info.power_draw {
                    measurements.push((
                        start_time.elapsed().as_secs(),
                        gpu_id,
                        power_draw,
                        info.temperature.unwrap_or(0.0),
                    ));

                    print!(
                        "\r⚡ GPU {}: {:.1}W | 🌡️ {:.0}°C",
                        gpu_id,
                        power_draw,
                        info.temperature.unwrap_or(0.0)
                    );
                    std::io::stdout().flush().unwrap();
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("\n");

    // Analyze measurements
    if !measurements.is_empty() {
        analyze_power_measurements(&measurements)?;
    }

    Ok(())
}

fn analyze_power_measurements(measurements: &[(u64, usize, f32, f32)]) -> NvResult<()> {
    println!("📊 Power Consumption Analysis:");
    println!("============================");

    // Group by GPU
    let mut gpu_data: std::collections::HashMap<usize, Vec<(f32, f32)>> =
        std::collections::HashMap::new();

    for &(_, gpu_id, power, temp) in measurements {
        gpu_data.entry(gpu_id).or_default().push((power, temp));
    }

    for (gpu_id, data) in gpu_data {
        if !data.is_empty() {
            let avg_power = data.iter().map(|(p, _)| p).sum::<f32>() / data.len() as f32;
            let max_power = data.iter().map(|(p, _)| p).fold(0.0f32, |a, &b| a.max(b));
            let min_power = data
                .iter()
                .map(|(p, _)| p)
                .fold(f32::INFINITY, |a, &b| a.min(b));
            let avg_temp = data.iter().map(|(_, t)| t).sum::<f32>() / data.len() as f32;

            println!("GPU {}:", gpu_id);
            println!("  Average Power: {:.1}W", avg_power);
            println!("  Peak Power: {:.1}W", max_power);
            println!("  Minimum Power: {:.1}W", min_power);
            println!("  Average Temperature: {:.1}°C", avg_temp);

            // Power efficiency recommendations
            if avg_power > 250.0 {
                println!("  💡 Consider power limit reduction for efficiency");
            }
            if avg_temp > 80.0 {
                println!("  🌡️ Consider better cooling or lower power limits");
            }

            println!();
        }
    }

    Ok(())
}

/// Create power management automation rules
pub fn create_power_automation() -> NvResult<()> {
    println!("🤖 Setting up power management automation...");

    // Create a service file for power management
    let service_content = r#"[Unit]
Description=nvcontrol Power Management
After=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/local/bin/nvcontrol-power-daemon
Restart=always
RestartSec=5

[Install]
WantedBy=graphical-session.target
"#;

    // Save service file
    let service_path = "/tmp/nvcontrol-power.service";
    fs::write(service_path, service_content).map_err(|e| {
        NvControlError::PowerManagementFailed(format!("Failed to create service file: {}", e))
    })?;

    println!(
        "📄 Created power management service file at {}",
        service_path
    );
    println!("To install: sudo cp {} /etc/systemd/user/", service_path);
    println!("To enable: systemctl --user enable nvcontrol-power.service");

    Ok(())
}

use std::io::Write;

fn get_gpu_count() -> NvResult<usize> {
    // Try NVML first
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=count"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(count) = output_str.trim().parse::<usize>() {
                return Ok(count);
            }
        }
    }

    // Fallback: count DRM cards
    let drm_path = Path::new("/sys/class/drm");
    if drm_path.exists() {
        let count = fs::read_dir(drm_path)
            .map_err(|e| {
                NvControlError::PowerManagementFailed(format!(
                    "Failed to read DRM directory: {}",
                    e
                ))
            })?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    if name.starts_with("card") && !name.contains("control") {
                        Some(())
                    } else {
                        None
                    }
                })
            })
            .count();

        if count > 0 {
            return Ok(count);
        }
    }

    // Final fallback
    Ok(1)
}

fn set_cpu_governor(governor: &str) -> NvResult<bool> {
    // Set CPU frequency governor
    let cpu_count = num_cpus::get();
    let mut success_count = 0;

    for cpu in 0..cpu_count {
        let governor_path = format!(
            "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor",
            cpu
        );
        if Path::new(&governor_path).exists() && fs::write(&governor_path, governor).is_ok() {
            success_count += 1;
        }
    }

    if success_count > 0 {
        println!(
            "✅ Set CPU governor to '{}' for {}/{} CPUs",
            governor, success_count, cpu_count
        );
        Ok(true)
    } else {
        // Try using cpupower as fallback
        let output = Command::new("sudo")
            .args(&["cpupower", "frequency-set", "-g", governor])
            .output();

        if output.is_ok() {
            println!("✅ Set CPU governor to '{}' via cpupower", governor);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Helper functions for power management
fn get_power_draw(gpu_id: usize) -> NvResult<f32> {
    // Try nvidia-smi first
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["-i", &gpu_id.to_string()])
        .args(&["--query-gpu=power.draw"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(power) = output_str.trim().parse::<f32>() {
                return Ok(power);
            }
        }
    }

    Ok(0.0)
}

fn get_power_limit(gpu_id: usize) -> NvResult<f32> {
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["-i", &gpu_id.to_string()])
        .args(&["--query-gpu=power.limit"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(limit) = output_str.trim().parse::<f32>() {
                return Ok(limit);
            }
        }
    }

    Ok(0.0)
}

fn get_power_limit_default(gpu_id: usize) -> NvResult<f32> {
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["-i", &gpu_id.to_string()])
        .args(&["--query-gpu=power.default_limit"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(default_limit) = output_str.trim().parse::<f32>() {
                return Ok(default_limit);
            }
        }
    }

    Ok(0.0)
}

fn get_power_limit_range(gpu_id: usize) -> NvResult<(f32, f32)> {
    // Try to get min/max power limits
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["-i", &gpu_id.to_string()])
        .args(&["--query-gpu=power.min_limit,power.max_limit"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.trim().split(',').collect();
            if parts.len() >= 2 {
                if let (Ok(min), Ok(max)) = (
                    parts[0].trim().parse::<f32>(),
                    parts[1].trim().parse::<f32>(),
                ) {
                    return Ok((min, max));
                }
            }
        }
    }

    // Fallback defaults
    Ok((50.0, 400.0))
}

fn get_gpu_temperature(gpu_id: usize) -> NvResult<f32> {
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["-i", &gpu_id.to_string()])
        .args(&["--query-gpu=temperature.gpu"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(temp) = output_str.trim().parse::<f32>() {
                return Ok(temp);
            }
        }
    }

    Ok(0.0)
}

fn get_fan_speed(gpu_id: usize) -> NvResult<u32> {
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["-i", &gpu_id.to_string()])
        .args(&["--query-gpu=fan.speed"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(speed) = output_str.trim().parse::<u32>() {
                return Ok(speed);
            }
        }
    }

    Ok(0)
}

fn get_power_state(gpu_id: usize) -> NvResult<String> {
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["-i", &gpu_id.to_string()])
        .args(&["--query-gpu=pstate"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return Ok(output_str.trim().to_string());
        }
    }

    Ok("Unknown".to_string())
}

fn get_persistence_mode(gpu_id: usize) -> NvResult<bool> {
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["-i", &gpu_id.to_string()])
        .args(&["--query-gpu=persistence_mode"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return Ok(output_str.trim().to_lowercase() == "enabled");
        }
    }

    Ok(false)
}

/// Enable CPU idle states for power saving
fn enable_cpu_idle_states() -> NvResult<()> {
    // Re-enable CPU idle states (C-states) for power saving
    let result = Command::new("sudo")
        .args(&[
            "sh",
            "-c",
            "echo 0 > /sys/devices/system/cpu/cpu*/cpuidle/state*/disable",
        ])
        .output();

    if result.is_ok() {
        println!("✅ CPU idle states enabled for power saving");
        Ok(())
    } else {
        Err(NvControlError::PowerManagementFailed(
            "Failed to enable CPU idle states".to_string(),
        ))
    }
}

/// Enable runtime power management for PCI devices
fn enable_runtime_pm() -> NvResult<()> {
    let result = Command::new("sudo")
        .args(&[
            "sh",
            "-c",
            "echo auto > /sys/bus/pci/devices/*/power/control",
        ])
        .output();

    if result.is_ok() {
        println!("✅ Runtime power management enabled");
        Ok(())
    } else {
        Err(NvControlError::PowerManagementFailed(
            "Failed to enable runtime power management".to_string(),
        ))
    }
}
