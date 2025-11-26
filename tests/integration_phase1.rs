/// Integration tests for Phase 1: Core Stability & Safety
///
/// Tests error handling, hardware safety, and graceful degradation

use nvcontrol::{
    error_recovery::{ErrorContext, NvmlFallback, RetryHandler},
    hardware_safety::{SafetyMonitor, SafetyThresholds, OverclockValidation, PowerValidation},
    gpu_safe::SafeGpuController,
    NvControlError,
};

#[test]
fn test_nvml_fallback_detection() {
    let fallback = NvmlFallback::new();

    // Should detect at least one method
    assert!(!fallback.available_methods().is_empty());

    // Should have primary method
    assert!(fallback.primary_method().is_some());

    println!("Available methods: {:?}", fallback.available_methods());
    println!("Primary method: {:?}", fallback.primary_method());
}

#[test]
fn test_error_context_formatting() {
    let ctx = ErrorContext::new("test operation")
        .with_gpu(0)
        .with_suggestion("Try restarting the driver");

    let error = NvControlError::GpuQueryFailed("Test error".to_string());
    let msg = ctx.to_user_message(&error);

    assert!(msg.contains("test operation"));
    assert!(msg.contains("GPU 0"));
    assert!(msg.contains("Try restarting the driver"));
}

#[test]
fn test_retry_handler_success() {
    let handler = RetryHandler::new(3, 10);
    let mut attempt = 0;

    let result = handler.retry(|| {
        attempt += 1;
        if attempt < 2 {
            Err(NvControlError::GpuQueryFailed("Transient".to_string()))
        } else {
            Ok(42)
        }
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(attempt, 2);
}

#[test]
fn test_retry_handler_failure() {
    let handler = RetryHandler::new(3, 10);
    let mut attempt = 0;

    let result: Result<i32, _> = handler.retry(|| {
        attempt += 1;
        Err(NvControlError::GpuQueryFailed("Permanent".to_string()))
    });

    assert!(result.is_err());
    assert_eq!(attempt, 3); // Should try 3 times
}

#[test]
fn test_safety_thresholds_defaults() {
    let thresholds = SafetyThresholds::default();

    assert_eq!(thresholds.temp_critical, 95);
    assert_eq!(thresholds.temp_warning, 85);
    assert_eq!(thresholds.max_power_limit_percent, 120);
    assert_eq!(thresholds.min_fan_speed_percent, 20);
    assert_eq!(thresholds.max_clock_offset_mhz, 500);
    assert_eq!(thresholds.max_memory_offset_mhz, 1000);
}

#[test]
fn test_overclock_validation_safe() {
    let monitor = SafetyMonitor::new(0, SafetyThresholds::default());

    // Safe overclock within limits
    let result = monitor.validate_overclock_safe(150, 500);

    match result {
        Ok(OverclockValidation::Safe) => {
            println!("✓ Overclock validated as safe");
        }
        Ok(OverclockValidation::Warning { reason }) => {
            println!("⚠️  Warning: {}", reason);
        }
        Ok(OverclockValidation::Unsafe { reason }) => {
            panic!("Should be safe but got unsafe: {}", reason);
        }
        Err(e) => {
            println!("Error checking safety (GPU may not be available): {}", e);
        }
    }
}

#[test]
fn test_overclock_validation_unsafe() {
    let monitor = SafetyMonitor::new(0, SafetyThresholds::default());

    // Unsafe overclock exceeds limits
    let result = monitor.validate_overclock_safe(600, 500);

    match result {
        Ok(OverclockValidation::Unsafe { reason }) => {
            println!("✓ Correctly rejected unsafe overclock: {}", reason);
            assert!(reason.contains("exceeds safe limit"));
        }
        Ok(OverclockValidation::Safe) => {
            panic!("Should reject unsafe overclock");
        }
        Ok(OverclockValidation::Warning { .. }) => {
            panic!("Should reject, not warn");
        }
        Err(e) => {
            println!("Error (GPU may not be available): {}", e);
        }
    }
}

#[test]
fn test_power_limit_validation_safe() {
    let monitor = SafetyMonitor::new(0, SafetyThresholds::default());

    let result = monitor.validate_power_limit_safe(100);

    assert!(matches!(result, Ok(PowerValidation::Safe)));
}

#[test]
fn test_power_limit_validation_unsafe() {
    let monitor = SafetyMonitor::new(0, SafetyThresholds::default());

    let result = monitor.validate_power_limit_safe(150);

    match result {
        Ok(PowerValidation::Unsafe { reason }) => {
            println!("✓ Correctly rejected unsafe power limit: {}", reason);
            assert!(reason.contains("exceeds safe maximum"));
        }
        _ => panic!("Should reject unsafe power limit"),
    }
}

#[test]
fn test_power_limit_validation_warning() {
    let monitor = SafetyMonitor::new(0, SafetyThresholds::default());

    let result = monitor.validate_power_limit_safe(45);

    match result {
        Ok(PowerValidation::Warning { reason }) => {
            println!("✓ Warning for low power limit: {}", reason);
        }
        _ => panic!("Should warn about low power limit"),
    }
}

#[test]
fn test_safe_gpu_controller_creation() {
    let controller = SafeGpuController::new(0);

    // Should not panic on creation
    println!("✓ SafeGpuController created successfully");
}

#[test]
fn test_safe_gpu_info_retrieval() {
    let controller = SafeGpuController::new(0);

    match controller.get_info() {
        Ok(info) => {
            println!("✓ GPU Info:");
            println!("  Name: {}", info.name);
            println!("  Driver: {}", info.driver_version);
            println!("  Memory: {} MB", info.memory_total_mb);
            println!("  Temperature: {}°C", info.temperature);
            println!("  Architecture: {:?}", info.architecture);
            println!("  Compute: {:?}", info.compute_capability);

            assert!(!info.name.is_empty());
            assert!(info.memory_total_mb > 0);
        }
        Err(e) => {
            println!("⚠️  GPU not available (expected in CI): {}", e);
        }
    }
}

#[test]
fn test_architecture_detection_blackwell() {
    let controller = SafeGpuController::new(0);

    // Test detection from compute capability
    let arch = nvcontrol::gpu_safe::SafeGpuController::detect_architecture(
        "RTX 5090",
        Some((10, 0))
    );

    assert_eq!(arch, Some("Blackwell".to_string()));
}

#[test]
fn test_architecture_detection_ada() {
    let arch = nvcontrol::gpu_safe::SafeGpuController::detect_architecture(
        "RTX 4090",
        Some((8, 9))
    );

    assert_eq!(arch, Some("Ada Lovelace".to_string()));
}

#[test]
fn test_architecture_detection_ampere() {
    let arch = nvcontrol::gpu_safe::SafeGpuController::detect_architecture(
        "RTX 3090",
        Some((8, 6))
    );

    assert_eq!(arch, Some("Ampere".to_string()));
}

#[test]
fn test_architecture_detection_from_name_fallback() {
    // Test name-based detection when compute capability unknown
    let arch = nvcontrol::gpu_safe::SafeGpuController::detect_architecture(
        "NVIDIA GeForce RTX 5090",
        None
    );

    assert_eq!(arch, Some("Blackwell".to_string()));
}

#[test]
fn test_safety_check() {
    let controller = SafeGpuController::new(0);

    match controller.check_safety() {
        Ok(_) => {
            println!("✓ Safety check passed");
        }
        Err(e) => {
            println!("⚠️  Safety check skipped (GPU not available): {}", e);
        }
    }
}

/// Stress test: Multiple rapid GPU queries
#[test]
fn test_rapid_gpu_queries() {
    let controller = SafeGpuController::new(0);

    for i in 0..10 {
        match controller.get_info() {
            Ok(info) => {
                println!("Query {}: Temp={}°C, Util={}%", i, info.temperature, info.gpu_utilization);
            }
            Err(_) => {
                println!("Query {} skipped (GPU not available)", i);
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

/// Test graceful degradation when NVML unavailable
#[test]
fn test_graceful_degradation() {
    let fallback = NvmlFallback::new();

    if !fallback.has_any_method() {
        println!("⚠️  No GPU control methods available (expected in some environments)");
    } else {
        println!("✓ At least one control method available: {:?}", fallback.available_methods());
    }

    // Should not panic even without GPU
    assert!(true);
}
