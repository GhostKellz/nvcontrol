//! Deterministic NVML-facing tests using mock backends.

use nvcontrol::nvml_backend::{MockNvmlBackend, NvmlBackend};

#[test]
fn test_mock_gpu_metrics_collection() {
    let backend = MockNvmlBackend::single_gpu();

    let device_count = backend.device_count().unwrap();
    assert_eq!(device_count, 1);

    for gpu_id in 0..device_count {
        let metrics = backend.get_metrics(gpu_id).unwrap();

        assert!(metrics.gpu_utilization <= 100);
        assert!(metrics.memory_utilization <= 100);
        assert!(metrics.temperature > 0);
        assert!(metrics.power_draw_mw > 0);
        assert!(metrics.fan_speed <= 100);
        assert!(metrics.gpu_clock_mhz > 0);
        assert!(metrics.memory_clock_mhz > 0);
        assert!(metrics.memory_used_bytes <= metrics.memory_total_bytes);
    }
}

#[test]
fn test_mock_multi_gpu_enumeration() {
    let backend = MockNvmlBackend::multi_gpu(4);

    assert_eq!(backend.device_count().unwrap(), 4);

    for i in 0..4 {
        let info = backend.get_device_info(i).unwrap();
        assert_eq!(info.index, i);
        assert!(info.name.contains("Mock GPU"));

        let metrics = backend.get_metrics(i).unwrap();
        assert!(metrics.temperature > 0);
    }

    assert!(backend.get_device_info(4).is_err());
}

#[test]
fn test_mock_no_gpu_graceful_handling() {
    let backend = MockNvmlBackend::no_gpu();

    assert_eq!(backend.device_count().unwrap(), 0);
    assert!(!backend.is_available());
    assert!(backend.get_device_info(0).is_err());
    assert!(backend.get_metrics(0).is_err());
    assert!(backend.get_temperature(0).is_err());
}

#[test]
fn test_mock_individual_metric_queries() {
    let backend = MockNvmlBackend::single_gpu();

    let temp = backend.get_temperature(0).unwrap();
    assert!(temp > 0 && temp < 150);

    let (gpu, mem) = backend.get_utilization(0).unwrap();
    assert!(gpu <= 100);
    assert!(mem <= 100);

    let power_mw = backend.get_power_usage(0).unwrap();
    assert!(power_mw > 0);

    let fan = backend.get_fan_speed(0, 0).unwrap();
    assert!(fan <= 100);

    let (used, total) = backend.get_memory_info(0).unwrap();
    assert!(used <= total);
    assert!(total > 0);

    let gpu_clock = backend.get_gpu_clock(0).unwrap();
    let mem_clock = backend.get_memory_clock(0).unwrap();
    assert!(gpu_clock > 0);
    assert!(mem_clock > 0);

    let name = backend.get_name(0).unwrap();
    assert!(!name.is_empty());
}

#[test]
fn test_gpu_info_retrieval() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::single_gpu());

    assert!(nvcontrol::gpu::is_nvidia_available(&backend));

    let info = nvcontrol::gpu::get_gpu_info(&backend).unwrap();
    assert!(!info.name.is_empty());
    assert!(info.temperature > 0);
    assert!(info.memory_total > 0);
    assert!(info.gpu_utilization <= 100);
    assert!(info.memory_utilization <= 100);
}

#[test]
fn test_gpu_info_no_gpu() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::no_gpu());

    assert!(!nvcontrol::gpu::is_nvidia_available(&backend));
    assert!(nvcontrol::gpu::get_gpu_info(&backend).is_err());
}

#[test]
fn test_multi_gpu_detection_with_backend() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::multi_gpu(4));

    let gpus = nvcontrol::multi_gpu::detect_gpus_with_backend(&backend).unwrap();
    assert_eq!(gpus.len(), 4);

    for (i, gpu) in gpus.iter().enumerate() {
        assert_eq!(gpu.index, i as u32);
        assert!(!gpu.uuid.is_empty());
        assert!(!gpu.pci_bus_id.is_empty());
        assert!(!gpu.driver_version.is_empty());
        assert!(gpu.vram_total > 0);
    }
}

#[test]
fn test_multi_gpu_single_info() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::single_gpu());

    let gpu = nvcontrol::multi_gpu::get_gpu_info_with_backend(0, &backend).unwrap();
    assert_eq!(gpu.index, 0);
    assert!(gpu.is_primary);
    assert!(gpu.cuda_cores.is_some());
    assert!(gpu.compute_capability.is_some());
}

#[test]
fn test_multi_gpu_count() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::multi_gpu(3));

    let count = nvcontrol::multi_gpu::get_gpu_count_with_backend(&backend).unwrap();
    assert_eq!(count, 3);
}

#[test]
fn test_extended_backend_methods() {
    let backend = MockNvmlBackend::single_gpu();

    let version = backend.get_driver_version().unwrap();
    assert!(!version.is_empty());

    let limit = backend.get_power_limit(0).unwrap();
    assert!(limit > 0);

    let (min, max) = backend.get_power_limit_constraints(0).unwrap();
    assert!(min < max);

    let cores = backend.get_cuda_cores(0).unwrap();
    assert!(cores > 0);

    let (major, _minor) = backend.get_compute_capability(0).unwrap();
    assert!(major > 0);

    let fan_count = backend.get_fan_count(0).unwrap();
    assert!(fan_count > 0);

    assert!(backend.is_fan_control_supported(0));
}

#[test]
fn test_monitoring_loop_derived_values() {
    let backend = MockNvmlBackend::single_gpu();
    let device_count = backend.device_count().unwrap();

    for _ in 0..10 {
        for gpu_id in 0..device_count {
            let metrics = backend.get_metrics(gpu_id).unwrap();

            let gpu_pct = metrics.gpu_utilization;
            let mem_pct = metrics.memory_utilization;
            let power_w = metrics.power_draw_mw as f64 / 1000.0;
            let mem_used_gb = metrics.memory_used_bytes as f64 / 1e9;
            let mem_total_gb = metrics.memory_total_bytes as f64 / 1e9;

            assert!(gpu_pct <= 100);
            assert!(mem_pct <= 100);
            assert!(power_w >= 0.0);
            assert!(mem_used_gb <= mem_total_gb);
        }
    }
}

#[test]
fn test_fan_list_with_backend() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::single_gpu());

    let fans = nvcontrol::fan::list_fans_with_backend(&backend);

    assert!(!fans.is_empty());

    let fan = &fans[0];
    assert_eq!(fan.id, 0);
    assert!(fan.rpm.is_some());
    assert!(fan.percent.is_some());
    assert!(fan.max_rpm.is_some());
}

#[test]
fn test_fan_list_multi_gpu() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::multi_gpu(2));

    let fans = nvcontrol::fan::list_fans_with_backend(&backend);

    assert!(fans.len() >= 2);

    let ids: Vec<usize> = fans.iter().map(|f| f.id).collect();
    assert!(ids.contains(&0));
    assert!(ids.contains(&10));
}

#[test]
fn test_fan_health_assessment() {
    use nvcontrol::fan::FanHealthStatus;
    use std::sync::Arc;

    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::single_gpu());
    let fans = nvcontrol::fan::list_fans_with_backend(&backend);

    for fan in &fans {
        assert_eq!(fan.health_status, FanHealthStatus::Healthy);
    }
}

#[test]
fn test_fan_no_gpu_graceful() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::no_gpu());

    let fans = nvcontrol::fan::list_fans_with_backend(&backend);

    assert!(!fans.is_empty());
}
