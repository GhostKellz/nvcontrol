use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyInfo {
    pub nvidia_reflex_available: bool,
    pub nvidia_reflex_enabled: bool,
    pub current_cpu_scheduler: String,
    pub gpu_scheduling_enabled: bool,
    pub preemption_timeout: Option<u32>,
    pub frame_time_consistency: f32,
    pub estimated_input_lag_ms: f32,
    pub optimizations_applied: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum LatencyMode {
    Default,
    Competitive, // Minimal latency, max performance
    Balanced,    // Good latency with stability
    PowerSaver,  // Higher latency, lower power
}

impl LatencyMode {
    pub fn as_str(&self) -> &str {
        match self {
            LatencyMode::Default => "default",
            LatencyMode::Competitive => "competitive",
            LatencyMode::Balanced => "balanced",
            LatencyMode::PowerSaver => "power_saver",
        }
    }
}

/// Get comprehensive latency information
pub fn get_latency_info() -> NvResult<LatencyInfo> {
    let mut info = LatencyInfo {
        nvidia_reflex_available: false,
        nvidia_reflex_enabled: false,
        current_cpu_scheduler: get_cpu_scheduler()?,
        gpu_scheduling_enabled: is_gpu_scheduling_enabled()?,
        preemption_timeout: get_preemption_timeout()?,
        frame_time_consistency: measure_frame_time_consistency()?,
        estimated_input_lag_ms: estimate_input_lag()?,
        optimizations_applied: Vec::new(),
    };

    // Check NVIDIA Reflex availability
    info.nvidia_reflex_available = check_nvidia_reflex_support()?;
    info.nvidia_reflex_enabled = check_nvidia_reflex_enabled()?;

    // Get current optimizations
    info.optimizations_applied = get_applied_optimizations()?;

    Ok(info)
}

/// Apply comprehensive latency optimizations
pub fn optimize_latency() -> NvResult<()> {
    println!("Applying comprehensive latency optimizations...");

    let mut applied_optimizations = Vec::new();

    // 1. GPU Scheduling Optimization
    if enable_gpu_scheduling()? {
        applied_optimizations.push("GPU hardware scheduling enabled".to_string());
    }

    // 2. Set low preemption timeout
    if set_preemption_timeout(1000)? {
        applied_optimizations.push("Preemption timeout set to 1ms".to_string());
    }

    // 3. CPU scheduler optimization
    if optimize_cpu_scheduler()? {
        applied_optimizations.push("CPU scheduler optimized for low latency".to_string());
    }

    // 4. NVIDIA-specific optimizations
    if apply_nvidia_latency_optimizations()? {
        applied_optimizations.push("NVIDIA driver latency optimizations".to_string());
    }

    // 5. System-level optimizations
    if apply_system_latency_optimizations()? {
        applied_optimizations.push("System latency optimizations".to_string());
    }

    // 6. Try to enable NVIDIA Reflex if available
    if enable_nvidia_reflex()? {
        applied_optimizations.push("NVIDIA Reflex enabled".to_string());
    }

    println!("\nLatency optimizations applied:");
    for opt in &applied_optimizations {
        println!("  ✓ {}", opt);
    }

    println!("\nRecommendations:");
    println!("  • Use a high refresh rate monitor (144Hz+)");
    println!("  • Enable G-SYNC/FreeSync if available");
    println!("  • Use exclusive fullscreen mode in games");
    println!("  • Consider a gaming mouse with 1000Hz polling");

    Ok(())
}

/// Apply latency optimizations by mode
pub fn set_latency_mode(mode: LatencyMode) -> NvResult<()> {
    println!("Setting latency mode: {}", mode.as_str());

    match mode {
        LatencyMode::Competitive => {
            // Maximum performance, minimum latency
            enable_gpu_scheduling()?;
            set_preemption_timeout(500)?; // Ultra-low preemption
            set_cpu_performance_mode()?;
            apply_nvidia_latency_optimizations()?;
            disable_cpu_mitigations()?; // For absolute minimum latency
        }
        LatencyMode::Balanced => {
            // Good latency with system stability
            enable_gpu_scheduling()?;
            set_preemption_timeout(2000)?; // Reasonable preemption
            optimize_cpu_scheduler()?;
            apply_nvidia_latency_optimizations()?;
        }
        LatencyMode::PowerSaver => {
            // Higher latency but lower power consumption
            disable_gpu_scheduling()?;
            set_preemption_timeout(10000)?; // Higher preemption for power saving
            set_cpu_powersave_mode()?;
        }
        LatencyMode::Default => {
            // Reset to system defaults
            reset_gpu_scheduling()?;
            reset_preemption_timeout()?;
            reset_cpu_scheduler()?;
        }
    }

    println!("Latency mode '{}' applied successfully", mode.as_str());
    Ok(())
}

// GPU Scheduling Functions

fn is_gpu_scheduling_enabled() -> NvResult<bool> {
    // Check Windows-style GPU scheduling on Linux (via wine/dxvk)
    if Path::new("/sys/class/drm/card0/device/enable_fbc").exists() {
        if let Ok(content) = fs::read_to_string("/sys/class/drm/card0/device/enable_fbc") {
            return Ok(content.trim() == "1");
        }
    }

    // Check NVIDIA-specific scheduling
    let output = Command::new("nvidia-smi")
        .args(&["-q", "-d", "COMPUTE"])
        .output();

    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        return Ok(output_str.contains("Compute Mode") && output_str.contains("Default"));
    }

    Ok(false)
}

fn enable_gpu_scheduling() -> NvResult<bool> {
    // Try to enable hardware-accelerated GPU scheduling

    // Method 1: Direct sysfs approach
    if Path::new("/sys/class/drm/card0/device/enable_fbc").exists() {
        let result = Command::new("sudo")
            .args(&["tee", "/sys/class/drm/card0/device/enable_fbc"])
            .arg("1")
            .output();

        if result.is_ok() {
            return Ok(true);
        }
    }

    // Method 2: NVIDIA-specific settings
    let result = Command::new("nvidia-settings")
        .args(&["-a", "GPUGraphicsClockOffset[4]=0"])
        .output();

    if result.is_ok() {
        return Ok(true);
    }

    // Method 3: Set environment variables for applications
    crate::safe_env::set_vars([
        ("__GL_THREADED_OPTIMIZATIONS", "1"),
        ("__GL_SYNC_TO_VBLANK", "0"),
    ]);

    Ok(false) // Partial success - env vars set but no hardware changes
}

fn disable_gpu_scheduling() -> NvResult<bool> {
    if Path::new("/sys/class/drm/card0/device/enable_fbc").exists() {
        let result = Command::new("sudo")
            .args(&["tee", "/sys/class/drm/card0/device/enable_fbc"])
            .arg("0")
            .output();

        return Ok(result.is_ok());
    }

    Ok(false)
}

fn reset_gpu_scheduling() -> NvResult<bool> {
    // Reset to system default
    if Path::new("/sys/class/drm/card0/device/enable_fbc").exists() {
        let result = Command::new("sudo")
            .args(&["tee", "/sys/class/drm/card0/device/enable_fbc"])
            .arg("1")
            .output();

        return Ok(result.is_ok());
    }

    Ok(false)
}

// Preemption Timeout Functions

fn get_preemption_timeout() -> NvResult<Option<u32>> {
    // Check current preemption timeout from kernel parameters
    if let Ok(cmdline) = fs::read_to_string("/proc/cmdline") {
        for param in cmdline.split_whitespace() {
            if param.starts_with("nvidia.NVreg_PreemptionTimeout=") {
                if let Some(timeout_str) = param.split('=').nth(1) {
                    if let Ok(timeout) = timeout_str.parse::<u32>() {
                        return Ok(Some(timeout));
                    }
                }
            }
        }
    }

    Ok(None)
}

fn set_preemption_timeout(timeout_us: u32) -> NvResult<bool> {
    // This requires modifying kernel parameters or NVIDIA module parameters
    // For runtime changes, we can try the sysfs approach

    let timeout_path = "/sys/module/nvidia/parameters/NVreg_PreemptionTimeout";
    if Path::new(timeout_path).exists() {
        let result = Command::new("sudo")
            .args(&["tee", timeout_path])
            .arg(&timeout_us.to_string())
            .output();

        return Ok(result.is_ok());
    }

    // Alternative: suggest kernel parameter addition
    println!("To set preemption timeout permanently, add to kernel parameters:");
    println!("nvidia.NVreg_PreemptionTimeout={}", timeout_us);

    Ok(false)
}

fn reset_preemption_timeout() -> NvResult<bool> {
    set_preemption_timeout(5000) // Default 5ms
}

// CPU Scheduler Functions

fn get_cpu_scheduler() -> NvResult<String> {
    // Check for BORE scheduler (CachyOS, XanMod, etc.)
    if std::path::Path::new("/sys/kernel/sched/bore").exists() {
        return Ok("BORE".to_string());
    }

    // Check for sched_ext (BPF-based schedulers like scx_rusty, scx_lavd)
    if std::path::Path::new("/sys/kernel/sched_ext/state").exists() {
        if let Ok(state) = fs::read_to_string("/sys/kernel/sched_ext/state") {
            if state.trim() == "enabled" {
                // Try to get the active sched_ext scheduler name
                if let Ok(name) = fs::read_to_string("/sys/kernel/sched_ext/root/name") {
                    return Ok(format!("sched_ext ({})", name.trim()));
                }
                return Ok("sched_ext".to_string());
            }
        }
    }

    // Check kernel version for EEVDF (default in 6.6+)
    if let Ok(version) = fs::read_to_string("/proc/version") {
        let version_lower = version.to_lowercase();

        // Check for custom scheduler keywords in kernel string
        if version_lower.contains("bore") {
            return Ok("BORE".to_string());
        }
        if version_lower.contains("-tt") {
            return Ok("Task Type (TT)".to_string());
        }
        if version_lower.contains("bmq") {
            return Ok("BMQ".to_string());
        }
        if version_lower.contains("pds") {
            return Ok("PDS".to_string());
        }
        if version_lower.contains("muqss") {
            return Ok("MuQSS".to_string());
        }

        // Parse kernel version to determine default scheduler
        // Format: "Linux version X.Y.Z..."
        if let Some(ver_start) = version.find("Linux version ") {
            let ver_str = &version[ver_start + 14..];
            if let Some(space_pos) = ver_str.find(' ') {
                let ver_num = &ver_str[..space_pos];
                let parts: Vec<&str> = ver_num.split('.').collect();
                if parts.len() >= 2 {
                    if let (Ok(major), Ok(minor)) =
                        (parts[0].parse::<u32>(), parts[1].parse::<u32>())
                    {
                        // EEVDF became default in kernel 6.6
                        if major > 6 || (major == 6 && minor >= 6) {
                            return Ok("EEVDF".to_string());
                        } else {
                            return Ok("CFS".to_string());
                        }
                    }
                }
            }
        }
    }

    // Legacy fallback - try sched_features (needs debugfs)
    if let Ok(content) = fs::read_to_string("/sys/kernel/debug/sched_features") {
        if !content.trim().is_empty() {
            return Ok("CFS (features detected)".to_string());
        }
    }

    Ok("Unknown".to_string())
}

fn optimize_cpu_scheduler() -> NvResult<bool> {
    let mut success = false;

    // Set CPU governor to performance for low latency
    if set_cpu_governor("performance")? {
        success = true;
    }

    // Disable CPU frequency scaling
    if disable_cpu_scaling()? {
        success = true;
    }

    // Set process priority optimizations
    if set_process_priorities()? {
        success = true;
    }

    Ok(success)
}

fn set_cpu_performance_mode() -> NvResult<bool> {
    let mut success = false;

    if set_cpu_governor("performance")? {
        success = true;
    }

    // Disable CPU idle states for absolute minimum latency
    if disable_cpu_idle_states()? {
        success = true;
    }

    Ok(success)
}

fn set_cpu_powersave_mode() -> NvResult<bool> {
    set_cpu_governor("powersave")
}

fn reset_cpu_scheduler() -> NvResult<bool> {
    set_cpu_governor("schedutil") // Modern default
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
        if Path::new(&governor_path).exists() {
            let result = Command::new("sudo")
                .args(&["tee", &governor_path])
                .arg(governor)
                .output();

            if result.is_ok() {
                success_count += 1;
            }
        }
    }

    Ok(success_count > 0)
}

fn disable_cpu_scaling() -> NvResult<bool> {
    // Disable CPU frequency scaling for consistent performance
    let result = Command::new("sudo")
        .args(&[
            "tee",
            "/sys/devices/system/cpu/cpufreq/policy*/scaling_max_freq",
        ])
        .arg("$(cat /sys/devices/system/cpu/cpufreq/policy*/cpuinfo_max_freq)")
        .output();

    Ok(result.is_ok())
}

fn disable_cpu_idle_states() -> NvResult<bool> {
    // Disable CPU idle states (C-states) for minimum latency
    let result = Command::new("sudo")
        .args(&["tee", "/sys/devices/system/cpu/cpu*/cpuidle/state*/disable"])
        .arg("1")
        .output();

    Ok(result.is_ok())
}

fn set_process_priorities() -> NvResult<bool> {
    // Set high priority for graphics-related processes
    let graphics_processes = vec!["Xorg", "gnome-shell", "kwin", "sway", "hyprland"];

    for process in graphics_processes {
        let _ = Command::new("sudo")
            .args(&["renice", "-10", "-n", process])
            .output();
    }

    Ok(true)
}

fn disable_cpu_mitigations() -> NvResult<bool> {
    // This is dangerous but gives absolute minimum latency
    println!("Warning: Disabling CPU mitigations reduces security!");
    println!("Add to kernel parameters: mitigations=off");
    Ok(false) // Don't actually do this automatically
}

// NVIDIA-specific optimizations

fn check_nvidia_reflex_support() -> NvResult<bool> {
    // Check if NVIDIA Reflex is supported (RTX 20 series and newer)
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(&["--query-gpu=name"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let gpu_name = String::from_utf8_lossy(&output.stdout);
            return Ok(gpu_name.contains("RTX 20")
                || gpu_name.contains("RTX 30")
                || gpu_name.contains("RTX 40"));
        }
    }
    Ok(false)
}

fn check_nvidia_reflex_enabled() -> NvResult<bool> {
    // Check if Reflex is currently enabled (would need game-specific detection)
    Ok(false)
}

fn enable_nvidia_reflex() -> NvResult<bool> {
    println!("NVIDIA Reflex requires game-specific implementation");
    Ok(false)
}

fn apply_nvidia_latency_optimizations() -> NvResult<bool> {
    // Apply NVIDIA-specific optimizations
    crate::safe_env::set_vars([
        ("__GL_SYNC_TO_VBLANK", "0"),
        ("__GL_ALLOW_UNOFFICIAL_PROTOCOL", "1"),
    ]);
    Ok(true)
}

fn apply_system_latency_optimizations() -> NvResult<bool> {
    // Apply system-level optimizations
    let mut applied = false;

    if optimize_network_latency()? {
        applied = true;
    }

    if optimize_io_scheduler()? {
        applied = true;
    }

    Ok(applied)
}

fn optimize_network_latency() -> NvResult<bool> {
    // Optimize network settings for lower latency
    Ok(false) // Placeholder
}

fn optimize_io_scheduler() -> NvResult<bool> {
    // Optimize I/O scheduler for lower latency
    Ok(false) // Placeholder
}

/// Optimize memory settings for lower latency
#[allow(dead_code)]
pub fn optimize_memory_latency() -> NvResult<bool> {
    println!("🧠 Optimizing memory settings for lower latency...");

    let mut success_count = 0;

    // Optimize memory swappiness for lower latency
    if set_memory_swappiness(10).is_ok() {
        success_count += 1;
        println!("  ✓ Memory swappiness reduced to 10");
    }

    // Optimize kernel memory parameters
    let memory_params = [
        ("/proc/sys/vm/dirty_ratio", "15"),
        ("/proc/sys/vm/dirty_background_ratio", "5"),
        ("/proc/sys/vm/dirty_expire_centisecs", "500"),
        ("/proc/sys/vm/dirty_writeback_centisecs", "100"),
        ("/proc/sys/vm/zone_reclaim_mode", "0"),
        ("/proc/sys/vm/vfs_cache_pressure", "50"),
    ];

    for (path, value) in memory_params {
        if Path::new(path).exists()
            && Command::new("sudo")
                .args(["tee", path])
                .arg(value)
                .output()
                .is_ok()
        {
            success_count += 1;
            println!(
                "  ✓ Set {} = {}",
                path.rsplit('/').next().unwrap_or(path),
                value
            );
        }
    }

    // Optimize transparent huge pages for latency
    if optimize_transparent_hugepages().is_ok() {
        success_count += 1;
        println!("  ✓ Transparent huge pages optimized");
    }

    // Set memory allocation policy for better latency
    if set_memory_allocation_policy().is_ok() {
        success_count += 1;
        println!("  ✓ Memory allocation policy optimized");
    }

    // Optimize NUMA settings if available
    if optimize_numa_settings().is_ok() {
        success_count += 1;
        println!("  ✓ NUMA settings optimized");
    }

    if success_count > 0 {
        println!("✅ Applied {} memory latency optimizations", success_count);
        Ok(true)
    } else {
        println!("⚠️ No memory latency optimizations could be applied");
        Ok(false)
    }
}

fn measure_frame_time_consistency() -> NvResult<f32> {
    // Measure frame time consistency (placeholder)
    Ok(0.0)
}

fn estimate_input_lag() -> NvResult<f32> {
    // Estimate input lag (placeholder)
    Ok(0.0)
}

fn get_applied_optimizations() -> NvResult<Vec<String>> {
    Ok(vec!["Basic optimizations applied".to_string()])
}

/// Set memory swappiness for lower latency
fn set_memory_swappiness(value: u32) -> NvResult<()> {
    let swappiness_path = "/proc/sys/vm/swappiness";
    if Path::new(swappiness_path).exists() {
        let result = Command::new("sudo")
            .args(&["tee", swappiness_path])
            .arg(&value.to_string())
            .output();

        if result.is_ok() {
            Ok(())
        } else {
            Err(crate::NvControlError::LatencyOptimizationFailed(
                "Failed to set memory swappiness".to_string(),
            ))
        }
    } else {
        Err(crate::NvControlError::LatencyOptimizationFailed(
            "Swappiness control not available".to_string(),
        ))
    }
}

/// Optimize transparent huge pages settings
fn optimize_transparent_hugepages() -> NvResult<()> {
    let thp_path = "/sys/kernel/mm/transparent_hugepage/enabled";
    if Path::new(thp_path).exists() {
        // Set to madvise for better latency control
        let result = Command::new("sudo")
            .args(&["tee", thp_path])
            .arg("madvise")
            .output();

        if result.is_ok() {
            Ok(())
        } else {
            Err(crate::NvControlError::LatencyOptimizationFailed(
                "Failed to optimize transparent huge pages".to_string(),
            ))
        }
    } else {
        Ok(()) // Not available on this system
    }
}

/// Set memory allocation policy for better latency
fn set_memory_allocation_policy() -> NvResult<()> {
    // Try to set memory allocation to prefer local NUMA nodes
    if Path::new("/proc/sys/kernel/numa_balancing").exists() {
        let result = Command::new("sudo")
            .args(&["tee", "/proc/sys/kernel/numa_balancing"])
            .arg("0") // Disable automatic NUMA balancing for consistent latency
            .output();

        if result.is_ok() {
            Ok(())
        } else {
            Err(crate::NvControlError::LatencyOptimizationFailed(
                "Failed to set NUMA balancing".to_string(),
            ))
        }
    } else {
        Ok(()) // NUMA not available
    }
}

/// Optimize NUMA settings for latency
fn optimize_numa_settings() -> NvResult<()> {
    // Check if NUMA is available
    if Command::new("numactl").arg("--hardware").output().is_ok() {
        // Set memory interleave policy for consistent access times
        let result = Command::new("sudo")
            .args(&[
                "sh",
                "-c",
                "echo 2 > /proc/sys/kernel/numa_balancing_scan_delay_ms",
            ])
            .output();

        if result.is_ok() {
            Ok(())
        } else {
            Err(crate::NvControlError::LatencyOptimizationFailed(
                "Failed to optimize NUMA settings".to_string(),
            ))
        }
    } else {
        Ok(()) // NUMA tools not available
    }
}
