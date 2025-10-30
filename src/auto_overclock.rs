// Automated Overclocking with Safety
// Auto-tune GPU for optimal performance with stability testing

use crate::benchmark::BenchmarkSuite;
use crate::overclocking::OverclockProfile;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoOCConfig {
    pub target: AutoOCTarget,
    pub safety_mode: SafetyMode,
    pub max_temp: f32,
    pub max_power: u32,
    pub stability_test_duration: u64,  // seconds
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AutoOCTarget {
    MaxPerformance,   // Push as hard as possible
    Balanced,         // Balance performance and power
    Efficiency,       // Best performance per watt
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SafetyMode {
    Conservative,  // Small steps, thorough testing
    Moderate,      // Balanced approach
    Aggressive,    // Larger steps, faster tuning
}

impl Default for AutoOCConfig {
    fn default() -> Self {
        Self {
            target: AutoOCTarget::Balanced,
            safety_mode: SafetyMode::Conservative,
            max_temp: 85.0,
            max_power: 100,
            stability_test_duration: 60,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AutoOCResult {
    pub successful: bool,
    pub final_profile: OverclockProfile,
    pub baseline_score: f64,
    pub final_score: f64,
    pub improvement: f64,  // Percentage
    pub iterations: usize,
    pub time_taken: Duration,
    pub errors: Vec<String>,
}

pub struct AutoOverclocker {
    config: AutoOCConfig,
    benchmark_suite: BenchmarkSuite,
}

impl AutoOverclocker {
    pub fn new(config: AutoOCConfig) -> NvResult<Self> {
        Ok(Self {
            config,
            benchmark_suite: BenchmarkSuite::new()?,
        })
    }

    /// Run automated overclocking wizard
    pub fn run_auto_tune(&self) -> NvResult<AutoOCResult> {
        println!("🚀 Starting automated overclocking...");
        println!("   Target: {:?}", self.config.target);
        println!("   Safety: {:?}", self.config.safety_mode);
        println!();

        let start_time = Instant::now();
        let mut errors = Vec::new();
        let mut iterations = 0;

        // Step 1: Get baseline performance
        println!("📊 Step 1/4: Measuring baseline performance...");
        let baseline_result = match self.benchmark_suite.run_full_benchmark(30) {
            Ok(result) => {
                println!("   ✅ Baseline score: {:.2}", result.total_score);
                result
            }
            Err(e) => {
                return Err(NvControlError::RuntimeError(format!("Baseline benchmark failed: {}", e)));
            }
        };

        // Step 2: Test stability at stock settings
        println!("\n🔥 Step 2/4: Testing stability at stock settings...");
        let stock_stable = self.test_stability(30)?;
        if !stock_stable {
            errors.push("System unstable at stock settings".to_string());
            return Ok(AutoOCResult {
                successful: false,
                final_profile: OverclockProfile::default(),
                baseline_score: baseline_result.total_score,
                final_score: baseline_result.total_score,
                improvement: 0.0,
                iterations,
                time_taken: start_time.elapsed(),
                errors,
            });
        }
        println!("   ✅ Stock settings stable");

        // Step 3: Iteratively increase clocks
        println!("\n⚡ Step 3/4: Finding optimal clock speeds...");
        let mut current_profile = OverclockProfile::default();
        let mut best_profile = current_profile.clone();
        let mut best_score = baseline_result.total_score;

        let (gpu_step, mem_step) = self.get_step_sizes();

        // GPU Core Clock tuning
        println!("   🔧 Tuning GPU core clock...");
        loop {
            iterations += 1;
            current_profile.gpu_clock_offset += gpu_step;

            println!("      Testing GPU +{} MHz...", current_profile.gpu_clock_offset);

            // Apply and test
            if let Err(e) = crate::overclocking::apply_overclock_profile(&current_profile) {
                errors.push(format!("Failed to apply OC: {}", e));
                break;
            }

            // Quick stability check
            if !self.test_stability(10)? {
                println!("      ❌ Unstable at +{} MHz", current_profile.gpu_clock_offset);
                current_profile.gpu_clock_offset -= gpu_step;  // Roll back
                break;
            }

            // Benchmark
            match self.benchmark_suite.run_full_benchmark(30) {
                Ok(result) => {
                    if result.total_score > best_score {
                        println!("      ✅ Improved score: {:.2} (+{:.1}%)",
                            result.total_score,
                            ((result.total_score - best_score) / best_score) * 100.0
                        );
                        best_score = result.total_score;
                        best_profile = current_profile.clone();
                    } else {
                        println!("      ⚠️  No improvement, stopping GPU tuning");
                        break;
                    }

                    // Safety checks
                    if result.max_temp > self.config.max_temp {
                        println!("      🌡️  Temperature limit reached ({:.1}°C)", result.max_temp);
                        break;
                    }
                }
                Err(e) => {
                    errors.push(format!("Benchmark failed: {}", e));
                    break;
                }
            }

            // Safety limit
            if current_profile.gpu_clock_offset >= 300 {
                println!("      ⚠️  Maximum safe offset reached");
                break;
            }
        }

        // Memory Clock tuning
        println!("   🔧 Tuning memory clock...");
        current_profile = best_profile.clone();

        loop {
            iterations += 1;
            current_profile.memory_clock_offset += mem_step;

            println!("      Testing Memory +{} MHz...", current_profile.memory_clock_offset);

            // Apply and test
            if let Err(e) = crate::overclocking::apply_overclock_profile(&current_profile) {
                errors.push(format!("Failed to apply OC: {}", e));
                break;
            }

            // Quick stability check
            if !self.test_stability(10)? {
                println!("      ❌ Unstable at +{} MHz", current_profile.memory_clock_offset);
                current_profile.memory_clock_offset -= mem_step;
                break;
            }

            // Benchmark
            match self.benchmark_suite.run_full_benchmark(30) {
                Ok(result) => {
                    if result.total_score > best_score {
                        println!("      ✅ Improved score: {:.2} (+{:.1}%)",
                            result.total_score,
                            ((result.total_score - best_score) / best_score) * 100.0
                        );
                        best_score = result.total_score;
                        best_profile = current_profile.clone();
                    } else {
                        println!("      ⚠️  No improvement, stopping memory tuning");
                        break;
                    }

                    // Safety checks
                    if result.max_temp > self.config.max_temp {
                        println!("      🌡️  Temperature limit reached");
                        break;
                    }
                }
                Err(e) => {
                    errors.push(format!("Benchmark failed: {}", e));
                    break;
                }
            }

            // Safety limit
            if current_profile.memory_clock_offset >= 1000 {
                println!("      ⚠️  Maximum safe offset reached");
                break;
            }
        }

        // Step 4: Final stability test
        println!("\n🔥 Step 4/4: Final stability test...");
        let _ = crate::overclocking::apply_overclock_profile(&best_profile);

        let final_stable = self.test_stability(self.config.stability_test_duration)?;

        if !final_stable {
            println!("   ❌ Final stability test failed, rolling back...");
            errors.push("Final stability test failed".to_string());

            // Try slightly lower settings
            best_profile.gpu_clock_offset -= gpu_step;
            best_profile.memory_clock_offset -= mem_step;

            let _ = crate::overclocking::apply_overclock_profile(&best_profile);

            let retry_stable = self.test_stability(self.config.stability_test_duration)?;
            if !retry_stable {
                return Ok(AutoOCResult {
                    successful: false,
                    final_profile: OverclockProfile::default(),
                    baseline_score: baseline_result.total_score,
                    final_score: baseline_result.total_score,
                    improvement: 0.0,
                    iterations,
                    time_taken: start_time.elapsed(),
                    errors,
                });
            }
        }

        println!("   ✅ Stability test passed!");

        let improvement = ((best_score - baseline_result.total_score) / baseline_result.total_score) * 100.0;

        Ok(AutoOCResult {
            successful: true,
            final_profile: best_profile,
            baseline_score: baseline_result.total_score,
            final_score: best_score,
            improvement,
            iterations,
            time_taken: start_time.elapsed(),
            errors,
        })
    }

    /// Test system stability
    fn test_stability(&self, duration_secs: u64) -> NvResult<bool> {
        match self.benchmark_suite.run_stability_test(duration_secs as u32) {
            Ok(stable) => Ok(stable),
            Err(_) => Ok(false),  // Treat errors as instability
        }
    }

    /// Get step sizes based on safety mode
    fn get_step_sizes(&self) -> (i32, i32) {
        match self.config.safety_mode {
            SafetyMode::Conservative => (10, 50),   // Small steps
            SafetyMode::Moderate => (25, 100),      // Medium steps
            SafetyMode::Aggressive => (50, 200),    // Large steps
        }
    }

    /// Print result summary
    pub fn print_result(&self, result: &AutoOCResult) {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🚀 Automated Overclocking Complete!");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        if result.successful {
            println!("✅ Status: SUCCESS");
            println!("\n📊 Results:");
            println!("  Baseline Score:     {:.2}", result.baseline_score);
            println!("  Final Score:        {:.2}", result.final_score);
            println!("  Improvement:        +{:.1}%", result.improvement);
            println!("\n⚡ Optimal Settings:");
            println!("  GPU Clock Offset:   {:+} MHz", result.final_profile.gpu_clock_offset);
            println!("  Memory Offset:      {:+} MHz", result.final_profile.memory_clock_offset);
            println!("  Power Limit:        {}%", result.final_profile.power_limit);
            println!("\n📈 Statistics:");
            println!("  Iterations:         {}", result.iterations);
            println!("  Time Taken:         {:.1} minutes", result.time_taken.as_secs_f64() / 60.0);
        } else {
            println!("❌ Status: FAILED");
            println!("\nAuto-tuning was unable to find stable overclocking settings.");

            if !result.errors.is_empty() {
                println!("\n⚠️  Errors:");
                for error in &result.errors {
                    println!("  • {}", error);
                }
            }
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AutoOCConfig::default();
        assert_eq!(config.target, AutoOCTarget::Balanced);
    }

    #[test]
    fn test_step_sizes() {
        let config = AutoOCConfig {
            safety_mode: SafetyMode::Conservative,
            ..Default::default()
        };

        if let Ok(overclocker) = AutoOverclocker::new(config) {
            let (gpu_step, mem_step) = overclocker.get_step_sizes();
            assert_eq!(gpu_step, 10);
            assert_eq!(mem_step, 50);
        }
    }
}
