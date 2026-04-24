/// Safety and Recovery Tests
///
/// Tests error handling, hardware safety validation, and graceful degradation
/// when GPU or NVML is unavailable.
use nvcontrol::{
    NvControlError,
    error_recovery::{ErrorContext, NvmlFallback, RetryHandler},
    gpu_safe::SafeGpuController,
    hardware_safety::{OverclockValidation, PowerValidation, SafetyMonitor, SafetyThresholds},
};

#[test]
fn test_nvml_fallback_detection() {
    let fallback = NvmlFallback::new();
    let methods = fallback.available_methods();

    // Should detect at least one method
    assert!(!methods.is_empty());

    // Should have primary method
    assert!(fallback.primary_method().is_some());
    assert!(fallback.has_any_method());
    assert_eq!(fallback.primary_method(), methods.first().copied());
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
        Ok(OverclockValidation::Safe) => {}
        Ok(OverclockValidation::Warning { reason }) => {
            assert!(reason.contains("temperature"));
        }
        Ok(OverclockValidation::Unsafe { reason }) => {
            panic!("Should be safe but got unsafe: {}", reason);
        }
        Err(e) => assert!(matches!(e, NvControlError::GpuQueryFailed(_))),
    }
}

#[test]
fn test_overclock_validation_unsafe() {
    let monitor = SafetyMonitor::new(0, SafetyThresholds::default());

    // Unsafe overclock exceeds limits
    let result = monitor.validate_overclock_safe(600, 500);

    match result {
        Ok(OverclockValidation::Unsafe { reason }) => {
            assert!(reason.contains("exceeds safe limit"));
        }
        Ok(OverclockValidation::Safe) => {
            panic!("Should reject unsafe overclock");
        }
        Ok(OverclockValidation::Warning { .. }) => {
            panic!("Should reject, not warn");
        }
        Err(e) => panic!("Unsafe overclock should fail before hardware checks: {}", e),
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
            assert!(reason.contains("below 50%"));
        }
        _ => panic!("Should warn about low power limit"),
    }
}

#[test]
fn test_safe_gpu_controller_creation() {
    let _controller = SafeGpuController::new(0);

    // Should not panic on creation
}

#[test]
fn test_safe_gpu_info_retrieval() {
    let controller = SafeGpuController::new(0);

    match controller.get_info() {
        Ok(info) => {
            assert!(!info.name.is_empty());
            assert!(info.memory_total_mb > 0);
        }
        Err(e) => assert!(matches!(e, NvControlError::GpuQueryFailed(_))),
    }
}

#[test]
fn test_architecture_detection_blackwell() {
    let _controller = SafeGpuController::new(0);

    // Test detection from compute capability
    let arch =
        nvcontrol::gpu_safe::SafeGpuController::detect_architecture("RTX 5090", Some((10, 0)));

    assert_eq!(arch, Some("Blackwell".to_string()));
}

#[test]
fn test_architecture_detection_ada() {
    let arch =
        nvcontrol::gpu_safe::SafeGpuController::detect_architecture("RTX 4090", Some((8, 9)));

    assert_eq!(arch, Some("Ada Lovelace".to_string()));
}

#[test]
fn test_architecture_detection_ampere() {
    let arch =
        nvcontrol::gpu_safe::SafeGpuController::detect_architecture("RTX 3090", Some((8, 6)));

    assert_eq!(arch, Some("Ampere".to_string()));
}

#[test]
fn test_architecture_detection_from_name_fallback() {
    // Test name-based detection when compute capability unknown
    let arch = nvcontrol::gpu_safe::SafeGpuController::detect_architecture(
        "NVIDIA GeForce RTX 5090",
        None,
    );

    assert_eq!(arch, Some("Blackwell".to_string()));
}

#[test]
fn test_safety_check() {
    let controller = SafeGpuController::new(0);

    match controller.check_safety() {
        Ok(()) => {}
        Err(e) => assert!(matches!(e, NvControlError::GpuQueryFailed(_))),
    }
}

/// Stress test: Multiple rapid GPU queries
#[test]
fn test_rapid_gpu_queries() {
    let controller = SafeGpuController::new(0);
    let mut attempts = 0;

    for _ in 0..3 {
        attempts += 1;
        match controller.get_info() {
            Ok(info) => {
                assert!(!info.name.is_empty());
            }
            Err(e) => {
                assert!(matches!(e, NvControlError::GpuQueryFailed(_)));
                break;
            }
        }
    }

    assert!(attempts >= 1);
}

/// Test graceful degradation when NVML unavailable
#[test]
fn test_graceful_degradation() {
    let fallback = NvmlFallback::new();

    assert_eq!(
        fallback.has_any_method(),
        !fallback.available_methods().is_empty()
    );
}
