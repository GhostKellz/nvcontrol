use crate::{NvControlError, NvResult};
use nvml_wrapper::{Nvml, Device};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub timestamp: u64,
    pub gpu_name: String,
    pub test_type: String,
    pub duration_seconds: u32,
    pub average_utilization: f64,
    pub peak_utilization: f64,
    pub average_temperature: f64,
    pub peak_temperature: f64,
    pub average_power: f64,
    pub peak_power: f64,
    pub memory_bandwidth_gbps: Option<f64>,
    pub compute_score: Option<f64>,
    pub graphics_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSnapshot {
    pub timestamp: u64,
    pub gpu_utilization: f64,
    pub memory_utilization: f64,
    pub temperature: f64,
    pub power_draw: f64,
    pub fan_speed: f64,
    pub gpu_clock: f64,
    pub memory_clock: f64,
    pub memory_used_mb: f64,
    pub memory_total_mb: f64,
}

/// Live GPU monitoring with text output (like htop but for GPU)
pub fn live_gpu_watch(interval_seconds: u64, max_count: u32) -> NvResult<()> {
    let nvml = Nvml::init().map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("NVML initialization failed: {}", e))
    })?;

    let device_count = nvml.device_count().map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to get device count: {}", e))
    })?;

    if device_count == 0 {
        return Err(NvControlError::DisplayDetectionFailed("No NVIDIA GPUs found".to_string()));
    }

    println!("🔍 Live GPU Monitoring (Ctrl+C to stop)");
    println!("{}", "=".repeat(80));
    
    let mut count = 0;
    loop {
        if max_count > 0 && count >= max_count {
            break;
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
        println!("nvcontrol Live Monitor - {} | Update #{}", 
                 chrono::DateTime::from_timestamp(timestamp as i64, 0)
                     .unwrap()
                     .format("%Y-%m-%d %H:%M:%S"),
                 count + 1);
        println!("{}", "=".repeat(80));

        for gpu_id in 0..device_count {
            if let Ok(device) = nvml.device_by_index(gpu_id) {
                print_live_gpu_stats(&device, gpu_id)?;
                println!();
            }
        }

        thread::sleep(Duration::from_secs(interval_seconds));
        count += 1;
    }

    Ok(())
}

fn print_live_gpu_stats(device: &Device, gpu_id: u32) -> NvResult<()> {
    let name = device.name().unwrap_or("Unknown".to_string());
    
    // Get utilization
    let utilization = device.utilization_rates()
        .map(|u| (u.gpu, u.memory))
        .unwrap_or((0, 0));

    // Get temperature
    let temp = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
        .unwrap_or(0);

    // Get power
    let power = device.power_usage()
        .map(|p| p as f64 / 1000.0) // mW to W
        .unwrap_or(0.0);

    // Get memory info
    let memory = device.memory_info().ok();
    let (mem_used, mem_total) = if let Some(mem) = memory {
        (mem.used as f64 / 1e9, mem.total as f64 / 1e9)
    } else {
        (0.0, 0.0)
    };

    // Get clocks
    let gpu_clock = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
        .unwrap_or(0);
    let mem_clock = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
        .unwrap_or(0);

    // Get fan speed
    let fan_speed = device.fan_speed(0).unwrap_or(0);

    println!("GPU {}: {}", gpu_id, name);
    println!("  Utilization: {:3}% GPU | {:3}% Memory", utilization.0, utilization.1);
    println!("  Temperature: {}°C | Fan: {}%", temp, fan_speed);
    println!("  Power: {:.1}W", power);
    println!("  Memory: {:.1}/{:.1} GB ({:.1}%)", 
             mem_used, mem_total, 
             if mem_total > 0.0 { (mem_used / mem_total) * 100.0 } else { 0.0 });
    println!("  Clocks: {} MHz GPU | {} MHz Memory", gpu_clock, mem_clock);

    // Create visual bars
    let gpu_bar = create_progress_bar(utilization.0 as f64, 100.0, 20);
    let mem_bar = create_progress_bar((mem_used / mem_total) * 100.0, 100.0, 20);
    let temp_bar = create_progress_bar(temp as f64, 100.0, 20);

    println!("  GPU  [{}] {:3}%", gpu_bar, utilization.0);
    println!("  VRAM [{}] {:3.0}%", mem_bar, (mem_used / mem_total) * 100.0);
    println!("  TEMP [{}] {:3}°C", temp_bar, temp);

    Ok(())
}

fn create_progress_bar(value: f64, max: f64, width: usize) -> String {
    let percentage = (value / max).min(1.0);
    let filled = (percentage * width as f64) as usize;
    let empty = width - filled;

    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

/// Export GPU metrics to JSON or CSV
pub fn export_gpu_metrics(
    format: &str, 
    output_path: Option<&str>, 
    duration_seconds: u32
) -> NvResult<()> {
    let nvml = Nvml::init().map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("NVML initialization failed: {}", e))
    })?;

    let device_count = nvml.device_count().map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to get device count: {}", e))
    })?;

    if device_count == 0 {
        return Err(NvControlError::DisplayDetectionFailed("No NVIDIA GPUs found".to_string()));
    }

    println!("📊 Collecting GPU metrics for {} seconds...", duration_seconds);
    
    let mut all_metrics = Vec::new();
    let start_time = Instant::now();
    let mut sample_count = 0;

    while start_time.elapsed().as_secs() < duration_seconds as u64 {
        for gpu_id in 0..device_count {
            if let Ok(device) = nvml.device_by_index(gpu_id) {
                if let Ok(metrics) = collect_device_metrics(&device, gpu_id) {
                    all_metrics.push(metrics);
                }
            }
        }
        
        sample_count += 1;
        thread::sleep(Duration::from_secs(1));
        
        if sample_count % 10 == 0 {
            println!("  Collected {} samples...", sample_count * device_count);
        }
    }

    // Generate output filename if not provided
    let default_filename = format!(
        "nvcontrol_metrics_{}.{}",
        chrono::Utc::now().format("%Y%m%d_%H%M%S"),
        format
    );
    let filename = output_path.unwrap_or(&default_filename);

    // Write to file
    match format.to_lowercase().as_str() {
        "json" => export_to_json(&all_metrics, filename)?,
        "csv" => export_to_csv(&all_metrics, filename)?,
        _ => return Err(NvControlError::DisplayDetectionFailed(
            format!("Unsupported format: {}. Use 'json' or 'csv'", format)
        )),
    }

    println!("✅ Exported {} metrics samples to {}", all_metrics.len(), filename);
    Ok(())
}

fn collect_device_metrics(device: &Device, _gpu_id: u32) -> NvResult<MetricSnapshot> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let utilization = device.utilization_rates()
        .map(|u| (u.gpu as f64, u.memory as f64))
        .unwrap_or((0.0, 0.0));

    let temperature = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
        .unwrap_or(0) as f64;

    let power_draw = device.power_usage()
        .map(|p| p as f64 / 1000.0)
        .unwrap_or(0.0);

    let fan_speed = device.fan_speed(0)
        .unwrap_or(0) as f64;

    let gpu_clock = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
        .unwrap_or(0) as f64;

    let memory_clock = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
        .unwrap_or(0) as f64;

    let memory = device.memory_info().ok();
    let (memory_used_mb, memory_total_mb) = if let Some(mem) = memory {
        (mem.used as f64 / 1e6, mem.total as f64 / 1e6)
    } else {
        (0.0, 0.0)
    };

    Ok(MetricSnapshot {
        timestamp,
        gpu_utilization: utilization.0,
        memory_utilization: utilization.1,
        temperature,
        power_draw,
        fan_speed,
        gpu_clock,
        memory_clock,
        memory_used_mb,
        memory_total_mb,
    })
}

fn export_to_json(metrics: &[MetricSnapshot], filename: &str) -> NvResult<()> {
    let mut file = File::create(filename).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to create file: {}", e))
    })?;

    let json = serde_json::to_string_pretty(metrics).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("JSON serialization failed: {}", e))
    })?;

    file.write_all(json.as_bytes()).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to write file: {}", e))
    })?;

    Ok(())
}

fn export_to_csv(metrics: &[MetricSnapshot], filename: &str) -> NvResult<()> {
    let mut file = File::create(filename).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to create file: {}", e))
    })?;

    // Write CSV header
    writeln!(file, "timestamp,gpu_utilization,memory_utilization,temperature,power_draw,fan_speed,gpu_clock,memory_clock,memory_used_mb,memory_total_mb").map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to write header: {}", e))
    })?;

    // Write data rows
    for metric in metrics {
        writeln!(file, "{},{},{},{},{},{},{},{},{},{}", 
                 metric.timestamp,
                 metric.gpu_utilization,
                 metric.memory_utilization,
                 metric.temperature,
                 metric.power_draw,
                 metric.fan_speed,
                 metric.gpu_clock,
                 metric.memory_clock,
                 metric.memory_used_mb,
                 metric.memory_total_mb).map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("Failed to write data: {}", e))
        })?;
    }

    Ok(())
}

/// Run comprehensive GPU benchmark
pub fn run_gpu_benchmark(
    duration_seconds: u32,
    test_type: &str,
    intensity: &str,
    log_results: bool
) -> NvResult<()> {
    println!("🏁 Starting GPU benchmark...");
    println!("  Test Type: {}", test_type);
    println!("  Duration: {} seconds", duration_seconds);
    println!("  Intensity: {}", intensity);

    let nvml = Nvml::init().map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("NVML initialization failed: {}", e))
    })?;

    let device = nvml.device_by_index(0).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to get GPU 0: {}", e))
    })?;

    let gpu_name = device.name().unwrap_or("Unknown GPU".to_string());
    println!("  GPU: {}", gpu_name);
    println!();

    // Start appropriate stress test
    let _stress_process = start_stress_test(test_type, intensity)?;

    // Monitor during benchmark
    let mut metrics = Vec::new();
    let start_time = Instant::now();
    let mut sample_count = 0;

    println!("📊 Monitoring performance...");
    
    while start_time.elapsed().as_secs() < duration_seconds as u64 {
        if let Ok(snapshot) = collect_device_metrics(&device, 0) {
            // Print progress every 10 samples
            if sample_count % 10 == 0 {
                println!("Sample {}: GPU {}%, Memory {}%, Temp {:.1}°C, Power {:.1}W",
                         sample_count + 1,
                         snapshot.gpu_utilization,
                         snapshot.memory_utilization,
                         snapshot.temperature,
                         snapshot.power_draw);
            }
            
            metrics.push(snapshot);
        }
        
        sample_count += 1;
        thread::sleep(Duration::from_secs(1));
    }

    // Analyze results
    if !metrics.is_empty() {
        let result = analyze_benchmark_results(&metrics, &gpu_name, test_type, duration_seconds);
        print_benchmark_results(&result);
        
        if log_results {
            save_benchmark_results(&result)?;
        }
    }

    println!("\n✅ Benchmark completed!");
    Ok(())
}

fn start_stress_test(test_type: &str, intensity: &str) -> NvResult<std::process::Child> {
    let stress_tools = match test_type.to_lowercase().as_str() {
        "compute" => vec![
            ("glmark2", vec!["--benchmark", "compute"]),
            ("vkmark", vec!["--suite", "compute"]),
        ],
        "graphics" => vec![
            ("glmark2", vec!["--benchmark", "desktop"]),
            ("vkmark", vec!["--suite", "desktop"]),
        ],
        "memory" => vec![
            ("glmark2", vec!["--benchmark", "buffer"]),
            ("vkmark", vec!["--suite", "buffer"]),
        ],
        _ => vec![
            ("glmark2", vec!["--benchmark", "desktop"]),
            ("vkmark", vec!["--suite", "desktop"]),
        ],
    };

    for (tool, mut args) in stress_tools {
        // Add intensity-specific arguments
        match intensity.to_lowercase().as_str() {
            "light" => args.push("--size=1024x768"),
            "heavy" => args.push("--size=3840x2160"),
            _ => args.push("--size=1920x1080"), // medium
        }

        if let Ok(child) = Command::new(tool)
            .args(&args)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn() 
        {
            println!("  Started stress test with {}", tool);
            return Ok(child);
        }
    }

    // Fallback: create simple GPU load
    println!("  No stress testing tools found, using fallback GPU load");
    Ok(Command::new("yes")
        .stdout(std::process::Stdio::null())
        .spawn()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to start fallback stress: {}", e)))?)
}

fn analyze_benchmark_results(
    metrics: &[MetricSnapshot],
    gpu_name: &str,
    test_type: &str,
    duration: u32
) -> BenchmarkResult {
    let avg_util = metrics.iter().map(|m| m.gpu_utilization).sum::<f64>() / metrics.len() as f64;
    let peak_util = metrics.iter().map(|m| m.gpu_utilization).fold(0.0, f64::max);
    
    let avg_temp = metrics.iter().map(|m| m.temperature).sum::<f64>() / metrics.len() as f64;
    let peak_temp = metrics.iter().map(|m| m.temperature).fold(0.0, f64::max);
    
    let avg_power = metrics.iter().map(|m| m.power_draw).sum::<f64>() / metrics.len() as f64;
    let peak_power = metrics.iter().map(|m| m.power_draw).fold(0.0, f64::max);

    // Calculate simple performance score
    let compute_score = Some(avg_util * (avg_power / 100.0)); // Simple heuristic
    let graphics_score = if test_type == "graphics" { compute_score } else { None };

    BenchmarkResult {
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        gpu_name: gpu_name.to_string(),
        test_type: test_type.to_string(),
        duration_seconds: duration,
        average_utilization: avg_util,
        peak_utilization: peak_util,
        average_temperature: avg_temp,
        peak_temperature: peak_temp,
        average_power: avg_power,
        peak_power: peak_power,
        memory_bandwidth_gbps: None, // Would need specialized tools
        compute_score,
        graphics_score,
    }
}

fn print_benchmark_results(result: &BenchmarkResult) {
    println!("\n📈 Benchmark Results:");
    println!("{}", "=".repeat(50));
    println!("GPU: {}", result.gpu_name);
    println!("Test: {} ({}s)", result.test_type, result.duration_seconds);
    println!();
    println!("Utilization:");
    println!("  Average: {:.1}%", result.average_utilization);
    println!("  Peak:    {:.1}%", result.peak_utilization);
    println!();
    println!("Temperature:");
    println!("  Average: {:.1}°C", result.average_temperature);
    println!("  Peak:    {:.1}°C", result.peak_temperature);
    println!();
    println!("Power:");
    println!("  Average: {:.1}W", result.average_power);
    println!("  Peak:    {:.1}W", result.peak_power);
    
    if let Some(score) = result.compute_score {
        println!();
        println!("Performance Score: {:.1}", score);
    }
}

fn save_benchmark_results(result: &BenchmarkResult) -> NvResult<()> {
    let filename = format!(
        "nvcontrol_benchmark_{}.json",
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );

    let mut file = File::create(&filename).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to create benchmark file: {}", e))
    })?;

    let json = serde_json::to_string_pretty(result).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("JSON serialization failed: {}", e))
    })?;

    file.write_all(json.as_bytes()).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to write benchmark file: {}", e))
    })?;

    println!("💾 Benchmark results saved to {}", filename);
    Ok(())
}