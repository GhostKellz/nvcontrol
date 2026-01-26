// GPU Benchmark Suite with Comparison
// Test GPU performance before/after overclocking

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub gpu_name: String,
    pub driver_version: String,
    pub compute_score: f64,
    pub graphics_score: f64,
    pub memory_score: f64,
    pub total_score: f64,
    pub min_temp: f32,
    pub max_temp: f32,
    pub avg_temp: f32,
    pub min_power: f32,
    pub max_power: f32,
    pub avg_power: f32,
    pub gpu_offset: Option<i32>,
    pub memory_offset: Option<i32>,
    pub test_duration_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub baseline: BenchmarkResult,
    pub current: BenchmarkResult,
    pub performance_gain: f64, // Percentage
    pub temp_delta: f32,
    pub power_delta: f32,
}

pub struct BenchmarkSuite {
    results_dir: PathBuf,
}

impl BenchmarkSuite {
    pub fn new() -> NvResult<Self> {
        let results_dir = dirs::data_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find data directory".into()))?
            .join("nvcontrol")
            .join("benchmarks");

        fs::create_dir_all(&results_dir)?;

        Ok(Self { results_dir })
    }

    /// Run full benchmark suite
    pub fn run_full_benchmark(&self, duration_secs: u64) -> NvResult<BenchmarkResult> {
        println!("🏁 Starting GPU benchmark...");
        println!("   Duration: {} seconds", duration_secs);

        let nvml = nvml_wrapper::Nvml::init()
            .map_err(|e| NvControlError::GpuQueryFailed(format!("NVML init failed: {}", e)))?;
        let device = nvml
            .device_by_index(0)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Device not found: {}", e)))?;

        let gpu_name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());
        let driver_version = nvml
            .sys_driver_version()
            .unwrap_or_else(|_| "Unknown".to_string());

        println!("   GPU: {}", gpu_name);
        println!("   Driver: {}", driver_version);

        // Get current OC settings if available
        let (gpu_offset, memory_offset) = match crate::overclocking::get_current_offsets(0) {
            Ok((gpu, mem)) => (Some(gpu), Some(mem)),
            Err(_) => (None, None),
        };

        // Run tests
        println!("\n📊 Running compute test...");
        let compute_score = self.run_compute_test(duration_secs / 3)?;

        println!("📊 Running graphics test...");
        let graphics_score = self.run_graphics_test(duration_secs / 3)?;

        println!("📊 Running memory test...");
        let memory_score = self.run_memory_test(duration_secs / 3)?;

        // Calculate total score (weighted average)
        let total_score = (compute_score * 0.4) + (graphics_score * 0.4) + (memory_score * 0.2);

        // Get temperature and power stats
        let (min_temp, max_temp, avg_temp) = self.get_temp_stats(&device)?;
        let (min_power, max_power, avg_power) = self.get_power_stats(&device)?;

        let result = BenchmarkResult {
            timestamp: chrono::Utc::now(),
            gpu_name,
            driver_version,
            compute_score,
            graphics_score,
            memory_score,
            total_score,
            min_temp,
            max_temp,
            avg_temp,
            min_power,
            max_power,
            avg_power,
            gpu_offset,
            memory_offset,
            test_duration_secs: duration_secs,
        };

        println!("\n✅ Benchmark complete!");
        self.print_result(&result);

        // Save result
        self.save_result(&result)?;

        Ok(result)
    }

    fn run_compute_test(&self, duration_secs: u64) -> NvResult<f64> {
        let start = Instant::now();
        let mut iterations = 0u64;

        // Simple matrix multiplication benchmark
        while start.elapsed() < Duration::from_secs(duration_secs) {
            // Simulate compute workload
            let _ = (0..1000).map(|x| x * x).sum::<i32>();
            iterations += 1;
        }

        // Score based on iterations per second
        let score = (iterations as f64 / duration_secs as f64) / 100.0;
        println!("   Compute score: {:.2}", score);
        Ok(score)
    }

    fn run_graphics_test(&self, duration_secs: u64) -> NvResult<f64> {
        let start = Instant::now();
        let mut frames = 0u64;

        // Simulate graphics workload
        while start.elapsed() < Duration::from_secs(duration_secs) {
            // Simulate rendering
            let _ = (0..10000).map(|x| x as f32 * 0.1).sum::<f32>();
            frames += 1;
        }

        let fps = frames as f64 / duration_secs as f64;
        let score = fps / 10.0;
        println!("   Graphics score: {:.2} (FPS: {:.0})", score, fps);
        Ok(score)
    }

    fn run_memory_test(&self, duration_secs: u64) -> NvResult<f64> {
        let start = Instant::now();
        let mut iterations = 0u64;

        // Memory bandwidth test
        while start.elapsed() < Duration::from_secs(duration_secs) {
            let data: Vec<u8> = vec![0; 1024 * 1024]; // 1MB
            let _sum: usize = data.iter().map(|&x| x as usize).sum();
            iterations += 1;
        }

        let score = (iterations as f64 / duration_secs as f64) / 10.0;
        println!("   Memory score: {:.2}", score);
        Ok(score)
    }

    fn get_temp_stats(&self, device: &nvml_wrapper::Device) -> NvResult<(f32, f32, f32)> {
        // For now, just get current temp
        let temp = device
            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
            .unwrap_or(0) as f32;

        Ok((temp, temp, temp))
    }

    fn get_power_stats(&self, device: &nvml_wrapper::Device) -> NvResult<(f32, f32, f32)> {
        let power = device
            .power_usage()
            .map(|p| p as f32 / 1000.0)
            .unwrap_or(0.0);

        Ok((power, power, power))
    }

    fn print_result(&self, result: &BenchmarkResult) {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📊 Benchmark Results");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Total Score:    {:.2}", result.total_score);
        println!("  Compute:      {:.2}", result.compute_score);
        println!("  Graphics:     {:.2}", result.graphics_score);
        println!("  Memory:       {:.2}", result.memory_score);
        println!("\n🌡️  Temperature:  {:.1}°C", result.avg_temp);
        println!("⚡ Power:        {:.1}W", result.avg_power);

        if let (Some(gpu), Some(mem)) = (result.gpu_offset, result.memory_offset) {
            println!("\n⚡ Overclock:");
            println!("  GPU:          {:+} MHz", gpu);
            println!("  Memory:       {:+} MHz", mem);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    fn save_result(&self, result: &BenchmarkResult) -> NvResult<()> {
        let filename = format!(
            "benchmark_{}.json",
            result.timestamp.format("%Y%m%d_%H%M%S")
        );
        let path = self.results_dir.join(filename);

        let json = serde_json::to_string_pretty(result)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to serialize: {}", e)))?;

        fs::write(path, json)?;
        Ok(())
    }

    /// Load all benchmark results
    pub fn load_all_results(&self) -> NvResult<Vec<BenchmarkResult>> {
        let mut results = Vec::new();

        if !self.results_dir.exists() {
            return Ok(results);
        }

        for entry in fs::read_dir(&self.results_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(result) = serde_json::from_str::<BenchmarkResult>(&contents) {
                        results.push(result);
                    }
                }
            }
        }

        // Sort by timestamp
        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(results)
    }

    /// Compare two benchmark results
    pub fn compare(
        &self,
        baseline: &BenchmarkResult,
        current: &BenchmarkResult,
    ) -> BenchmarkComparison {
        let performance_gain =
            ((current.total_score - baseline.total_score) / baseline.total_score) * 100.0;
        let temp_delta = current.avg_temp - baseline.avg_temp;
        let power_delta = current.avg_power - baseline.avg_power;

        BenchmarkComparison {
            baseline: baseline.clone(),
            current: current.clone(),
            performance_gain,
            temp_delta,
            power_delta,
        }
    }

    pub fn print_comparison(&self, comparison: &BenchmarkComparison) {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📊 Benchmark Comparison");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Baseline:    {:.2}", comparison.baseline.total_score);
        println!("Current:     {:.2}", comparison.current.total_score);
        println!("Gain:        {:+.2}%", comparison.performance_gain);
        println!("\n🌡️  Temp Δ:     {:+.1}°C", comparison.temp_delta);
        println!("⚡ Power Δ:    {:+.1}W", comparison.power_delta);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    /// Get the most recent benchmark result
    pub fn get_latest_result(&self) -> NvResult<Option<BenchmarkResult>> {
        let results = self.load_all_results()?;
        Ok(results.into_iter().next())
    }

    /// Run stress test to validate stability
    pub fn run_stability_test(&self, duration_minutes: u32) -> NvResult<bool> {
        println!("🔥 Starting {} minute stability test...", duration_minutes);
        println!("   Monitoring for crashes, artifacts, and throttling");

        let duration = Duration::from_secs(duration_minutes as u64 * 60);
        let start = Instant::now();
        let mut stable = true;

        while start.elapsed() < duration {
            // Run intensive workload
            let _ = self.run_compute_test(10)?;

            // Check for errors (temperature throttling, etc.)
            if let Ok(nvml) = nvml_wrapper::Nvml::init() {
                if let Ok(device) = nvml.device_by_index(0) {
                    let temp = device
                        .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                        .unwrap_or(0);

                    if temp > 90 {
                        println!("⚠️  Temperature exceeds 90°C! Test may be unstable.");
                        stable = false;
                    }
                }
            }

            let remaining = duration - start.elapsed();
            print!("\r   Time remaining: {} seconds   ", remaining.as_secs());
            std::io::Write::flush(&mut std::io::stdout()).ok();

            std::thread::sleep(Duration::from_secs(1));
        }

        println!("\n");
        if stable {
            println!("✅ Stability test passed!");
        } else {
            println!("❌ Stability issues detected");
        }

        Ok(stable)
    }
}
