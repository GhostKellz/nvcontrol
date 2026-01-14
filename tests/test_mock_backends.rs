//! Deterministic tests using mock backends
//!
//! These tests don't require real NVIDIA hardware or display commands.

use nvcontrol::display_backend::{
    DisplayCommandRunner, DisplayError, DisplayServer, MockDisplayRunner,
};
use nvcontrol::nvml_backend::{MockNvmlBackend, NvmlBackend};

// ============================================================================
// NVML Backend Tests
// ============================================================================

#[test]
fn test_mock_gpu_metrics_collection() {
    let backend = MockNvmlBackend::single_gpu();

    // Simulate what monitoring.rs does
    let device_count = backend.device_count().unwrap();
    assert_eq!(device_count, 1);

    for gpu_id in 0..device_count {
        let metrics = backend.get_metrics(gpu_id).unwrap();

        // Verify all metrics are populated
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

        // Each GPU should have slightly different metrics
        let metrics = backend.get_metrics(i).unwrap();
        assert!(metrics.temperature > 0);
    }

    // Out of bounds should error
    assert!(backend.get_device_info(4).is_err());
}

#[test]
fn test_mock_no_gpu_graceful_handling() {
    let backend = MockNvmlBackend::no_gpu();

    assert_eq!(backend.device_count().unwrap(), 0);
    assert!(!backend.is_available());

    // All queries should fail gracefully
    assert!(backend.get_device_info(0).is_err());
    assert!(backend.get_metrics(0).is_err());
    assert!(backend.get_temperature(0).is_err());
}

#[test]
fn test_mock_individual_metric_queries() {
    let backend = MockNvmlBackend::single_gpu();

    // Temperature
    let temp = backend.get_temperature(0).unwrap();
    assert!(temp > 0 && temp < 150);

    // Utilization
    let (gpu, mem) = backend.get_utilization(0).unwrap();
    assert!(gpu <= 100);
    assert!(mem <= 100);

    // Power
    let power_mw = backend.get_power_usage(0).unwrap();
    assert!(power_mw > 0);

    // Fan
    let fan = backend.get_fan_speed(0, 0).unwrap();
    assert!(fan <= 100);

    // Memory
    let (used, total) = backend.get_memory_info(0).unwrap();
    assert!(used <= total);
    assert!(total > 0);

    // Clocks
    let gpu_clock = backend.get_gpu_clock(0).unwrap();
    let mem_clock = backend.get_memory_clock(0).unwrap();
    assert!(gpu_clock > 0);
    assert!(mem_clock > 0);

    // Name
    let name = backend.get_name(0).unwrap();
    assert!(!name.is_empty());
}

// ============================================================================
// Display Backend Tests
// ============================================================================

#[test]
fn test_mock_x11_display_detection() {
    let runner = MockDisplayRunner::x11();

    assert_eq!(runner.get_display_server().unwrap(), DisplayServer::X11);

    // X11 tools should be available
    assert!(runner.command_available("xrandr"));
    assert!(runner.command_available("nvidia-settings"));

    // Wayland tools should not be available
    assert!(!runner.command_available("wlr-randr"));
    assert!(!runner.command_available("wayland-info"));
}

#[test]
fn test_mock_wayland_display_detection() {
    let runner = MockDisplayRunner::wayland();

    assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Wayland);

    // Wayland tools should be available
    assert!(runner.command_available("wlr-randr"));
    assert!(runner.command_available("wayland-info"));

    // X11 tools should not be available
    assert!(!runner.command_available("xrandr"));
}

#[test]
fn test_mock_xrandr_output_parsing() {
    let runner = MockDisplayRunner::x11();

    let output = runner.run_xrandr(&[]).unwrap();

    // Verify mock output contains expected data
    assert!(output.contains("Screen 0"));
    assert!(output.contains("DP-0 connected"));
    assert!(output.contains("2560x1440"));
    assert!(output.contains("143.91"));
}

#[test]
fn test_mock_wayland_info_output() {
    let runner = MockDisplayRunner::wayland();

    let output = runner.run_wayland_info().unwrap();

    assert!(output.contains("wl_output"));
    assert!(output.contains("2560"));
    assert!(output.contains("1440"));
    assert!(output.contains("143.912 Hz"));
}

#[test]
fn test_mock_nvidia_settings_output() {
    let runner = MockDisplayRunner::x11();

    let output = runner
        .run_nvidia_settings(&["-q", "DigitalVibrance"])
        .unwrap();

    assert!(output.contains("DigitalVibrance"));
    assert!(output.contains("-1024"));
    assert!(output.contains("1023"));
}

#[test]
fn test_mock_missing_binary_error() {
    let runner = MockDisplayRunner::x11();

    // wlr-randr not available on X11 mock
    let result = runner.run_wlr_randr(&[]);
    assert!(matches!(result, Err(DisplayError::BinaryMissing(_))));

    let runner = MockDisplayRunner::wayland();

    // xrandr not available on Wayland mock
    let result = runner.run_xrandr(&[]);
    assert!(matches!(result, Err(DisplayError::BinaryMissing(_))));
}

#[test]
fn test_mock_headless_environment() {
    let runner = MockDisplayRunner::headless();

    assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Unknown);

    // Nothing should be available
    assert!(!runner.command_available("xrandr"));
    assert!(!runner.command_available("wlr-randr"));
    assert!(!runner.command_available("nvidia-settings"));

    // All commands should fail with BinaryMissing
    assert!(runner.run_xrandr(&[]).is_err());
    assert!(runner.run_wayland_info().is_err());
    assert!(runner.run_nvidia_settings(&[]).is_err());
}

#[test]
fn test_mock_custom_output_injection() {
    let custom_xrandr = "Custom display output for testing";
    let runner = MockDisplayRunner::x11().with_xrandr_output(custom_xrandr);

    let output = runner.run_xrandr(&[]).unwrap();
    assert_eq!(output, custom_xrandr);
}

// ============================================================================
// GPU module tests using backend
// ============================================================================

#[test]
fn test_gpu_info_retrieval() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::single_gpu());

    // Test is_nvidia_available
    assert!(nvcontrol::gpu::is_nvidia_available(&backend));

    // Test get_gpu_info
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

    // Should not be available
    assert!(!nvcontrol::gpu::is_nvidia_available(&backend));

    // get_gpu_info should return error
    assert!(nvcontrol::gpu::get_gpu_info(&backend).is_err());
}

// ============================================================================
// Multi-GPU module tests using backend
// ============================================================================

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

    // Driver version
    let version = backend.get_driver_version().unwrap();
    assert!(!version.is_empty());

    // Power limits
    let limit = backend.get_power_limit(0).unwrap();
    assert!(limit > 0);

    let (min, max) = backend.get_power_limit_constraints(0).unwrap();
    assert!(min < max);

    // CUDA
    let cores = backend.get_cuda_cores(0).unwrap();
    assert!(cores > 0);

    let (major, _minor) = backend.get_compute_capability(0).unwrap();
    assert!(major > 0);

    // Fan
    let fan_count = backend.get_fan_count(0).unwrap();
    assert!(fan_count > 0);

    assert!(backend.is_fan_control_supported(0));
}

// ============================================================================
// Integration-style tests (simulating real workflows)
// ============================================================================

#[test]
fn test_simulated_monitoring_loop() {
    let backend = MockNvmlBackend::single_gpu();

    // Simulate what the TUI monitoring loop does
    let device_count = backend.device_count().unwrap();

    for _ in 0..10 {
        // 10 iterations
        for gpu_id in 0..device_count {
            let metrics = backend.get_metrics(gpu_id).unwrap();

            // Calculate utilization percentage
            let gpu_pct = metrics.gpu_utilization;
            let mem_pct = metrics.memory_utilization;

            // Calculate power in watts
            let power_w = metrics.power_draw_mw as f64 / 1000.0;

            // Calculate memory usage
            let mem_used_gb = metrics.memory_used_bytes as f64 / 1e9;
            let mem_total_gb = metrics.memory_total_bytes as f64 / 1e9;

            // Verify calculations don't overflow/underflow
            assert!(gpu_pct <= 100);
            assert!(mem_pct <= 100);
            assert!(power_w >= 0.0);
            assert!(mem_used_gb <= mem_total_gb);
        }
    }
}

#[test]
fn test_simulated_display_detection_flow() {
    // Test X11 flow
    let runner = MockDisplayRunner::x11();

    let server = runner.get_display_server().unwrap();
    match server {
        DisplayServer::X11 => {
            // On X11, use xrandr
            let output = runner.run_xrandr(&[]).unwrap();
            assert!(output.contains("connected"));
        }
        DisplayServer::Wayland => {
            // On Wayland, use wlr-randr
            let output = runner.run_wlr_randr(&[]).unwrap();
            assert!(output.contains("Enabled"));
        }
        DisplayServer::Unknown => {
            // Headless - no display commands
        }
    }
}

#[test]
fn test_simulated_vibrance_query() {
    let runner = MockDisplayRunner::x11();

    if runner.command_available("nvidia-settings") {
        let output = runner
            .run_nvidia_settings(&["-q", "DigitalVibrance"])
            .unwrap();

        // Parse vibrance value (mocked as 0)
        if output.contains("DigitalVibrance") {
            // Real parsing would extract the value
            assert!(output.contains("0"));
        }
    }
}

// ============================================================================
// Fan module tests using backend
// ============================================================================

#[test]
fn test_fan_list_with_backend() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::single_gpu());

    let fans = nvcontrol::fan::list_fans_with_backend(&backend);

    // Should have at least one fan
    assert!(!fans.is_empty());

    // Check first fan properties
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

    // With 2 GPUs, should have at least 2 fans (one per GPU)
    assert!(fans.len() >= 2);

    // Fans should have different IDs based on GPU index
    let ids: Vec<usize> = fans.iter().map(|f| f.id).collect();
    assert!(ids.contains(&0)); // GPU 0, fan 0
    assert!(ids.contains(&10)); // GPU 1, fan 0
}

#[test]
fn test_fan_health_assessment() {
    use nvcontrol::fan::FanHealthStatus;
    use std::sync::Arc;

    // Test healthy scenario
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::single_gpu());
    let fans = nvcontrol::fan::list_fans_with_backend(&backend);

    // Mock backend returns healthy metrics
    for fan in &fans {
        assert_eq!(fan.health_status, FanHealthStatus::Healthy);
    }
}

#[test]
fn test_fan_no_gpu_graceful() {
    use std::sync::Arc;
    let backend: Arc<dyn NvmlBackend> = Arc::new(MockNvmlBackend::no_gpu());

    let fans = nvcontrol::fan::list_fans_with_backend(&backend);

    // Should return fallback fan info (nvidia-smi or stub)
    // Even with no GPU, the function should not panic
    assert!(!fans.is_empty());
}

// ============================================================================
// Compositor-Specific Display Runner Tests (HDR/VRR support)
// ============================================================================

#[test]
fn test_kde_display_runner() {
    let runner = MockDisplayRunner::kde();

    // KDE tools should be available
    assert!(runner.command_available("kscreen-doctor"));
    assert!(runner.command_available("qdbus"));
    assert!(runner.command_available("wayland-info"));
    assert_eq!(runner.get_compositor(), "kde");

    // Get kscreen-doctor output
    let output = runner.run_command("kscreen-doctor", &["-j"]).unwrap();

    // Parse JSON to verify structure
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();
    let outputs = json.get("outputs").and_then(|o| o.as_array()).unwrap();
    assert_eq!(outputs.len(), 1);

    let display = &outputs[0];
    assert_eq!(display.get("name").unwrap().as_str().unwrap(), "DP-1");
    assert!(display.get("connected").unwrap().as_bool().unwrap());
    assert_eq!(display.get("vrrPolicy").unwrap().as_i64().unwrap(), 0);
}

#[test]
fn test_gnome_display_runner() {
    let runner = MockDisplayRunner::gnome();

    // GNOME tools should be available
    assert!(runner.command_available("gsettings"));
    assert_eq!(runner.get_compositor(), "gnome");

    // Get gsettings output
    let output = runner
        .run_command(
            "gsettings",
            &["get", "org.gnome.mutter", "experimental-features"],
        )
        .unwrap();

    // VRR should be enabled in mock
    assert!(output.contains("variable-refresh-rate"));
}

#[test]
fn test_hyprland_display_runner() {
    let runner = MockDisplayRunner::hyprland();

    // Hyprland tools should be available
    assert!(runner.command_available("hyprctl"));
    assert_eq!(runner.get_compositor(), "hyprland");

    // Get hyprctl output
    let output = runner.run_command("hyprctl", &["monitors", "-j"]).unwrap();

    // Parse JSON to verify structure
    let monitors: Vec<serde_json::Value> = serde_json::from_str(&output).unwrap();
    assert_eq!(monitors.len(), 1);

    let monitor = &monitors[0];
    assert_eq!(monitor.get("name").unwrap().as_str().unwrap(), "DP-1");
    assert_eq!(monitor.get("width").unwrap().as_i64().unwrap(), 2560);
    assert_eq!(monitor.get("height").unwrap().as_i64().unwrap(), 1440);
    assert_eq!(monitor.get("refreshRate").unwrap().as_f64().unwrap(), 165.0);
    assert!(!monitor.get("vrr").unwrap().as_bool().unwrap());
}

#[test]
fn test_simulated_vrr_detection_flow_kde() {
    // Simulate what vrr.rs does for KDE detection
    let runner = MockDisplayRunner::kde();

    if runner.command_available("kscreen-doctor") {
        let output = runner.run_command("kscreen-doctor", &["-j"]).unwrap();
        let json: serde_json::Value = serde_json::from_str(&output).unwrap();

        if let Some(outputs) = json.get("outputs").and_then(|o| o.as_array()) {
            for output in outputs {
                let connected = output
                    .get("connected")
                    .and_then(|c| c.as_bool())
                    .unwrap_or(false);
                let vrr_policy = output
                    .get("vrrPolicy")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let name = output
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("Unknown");

                assert!(connected);
                assert_eq!(name, "DP-1");
                assert_eq!(vrr_policy, 0); // VRR disabled in mock
            }
        }
    }
}

#[test]
fn test_simulated_hdr_detection_flow_hyprland() {
    // Simulate what hdr.rs does for Hyprland detection
    let runner = MockDisplayRunner::hyprland();

    if runner.command_available("hyprctl") {
        let output = runner.run_command("hyprctl", &["monitors", "-j"]).unwrap();
        let monitors: Vec<serde_json::Value> = serde_json::from_str(&output).unwrap();

        for monitor in &monitors {
            let name = monitor
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown");
            let current_format = monitor
                .get("currentFormat")
                .and_then(|f| f.as_str())
                .unwrap_or("");

            // Check for HDR-capable format
            let hdr_enabled =
                current_format.contains("101010") || current_format.contains("16161616");

            assert_eq!(name, "DP-1");
            assert!(!hdr_enabled); // Mock uses XRGB8888 (SDR)
        }
    }
}

#[test]
fn test_custom_compositor_mock() {
    // Test that custom command outputs work
    let runner = MockDisplayRunner::wayland()
        .with_compositor("sway")
        .with_command_output(
            "swaymsg",
            r#"[{"name":"DP-1","active":true,"adaptive_sync_status":"disabled"}]"#,
        );

    assert_eq!(runner.get_compositor(), "sway");
    assert!(runner.command_available("swaymsg"));

    let output = runner
        .run_command("swaymsg", &["-t", "get_outputs"])
        .unwrap();
    assert!(output.contains("adaptive_sync_status"));
}

#[test]
fn test_display_runner_fallback_chain() {
    // Test the fallback behavior when commands are missing

    // X11 environment without nvidia-settings
    let runner = MockDisplayRunner::x11();
    assert!(runner.command_available("xrandr"));

    // Missing nvidia-settings would return error
    let runner_limited = MockDisplayRunner::headless();
    assert!(runner_limited.run_nvidia_settings(&[]).is_err());
}

#[test]
fn test_mock_display_vrr_mode_values() {
    // Verify VRR mode value parsing from mocks
    let kde_runner = MockDisplayRunner::kde();
    let output = kde_runner.run_command("kscreen-doctor", &["-j"]).unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();

    // KDE vrrPolicy: 0=Never, 1=Always, 2=Automatic
    if let Some(outputs) = json.get("outputs").and_then(|o| o.as_array()) {
        let vrr_policy = outputs[0]
            .get("vrrPolicy")
            .and_then(|v| v.as_i64())
            .unwrap();
        assert!((0..=2).contains(&vrr_policy));
    }
}

#[test]
fn test_mock_display_refresh_rate_parsing() {
    // Test refresh rate parsing from different compositor mocks
    let hyprland_runner = MockDisplayRunner::hyprland();
    let output = hyprland_runner
        .run_command("hyprctl", &["monitors", "-j"])
        .unwrap();
    let monitors: Vec<serde_json::Value> = serde_json::from_str(&output).unwrap();

    let refresh_rate = monitors[0]
        .get("refreshRate")
        .and_then(|r| r.as_f64())
        .unwrap();
    assert!((refresh_rate - 165.0).abs() < 0.1);

    // Test mode string parsing
    if let Some(modes) = monitors[0].get("availableModes").and_then(|m| m.as_array()) {
        let first_mode = modes[0].as_str().unwrap();
        assert!(first_mode.contains("@")); // Format: "2560x1440@165.00Hz"
    }
}

// ============================================================================
// GuiBackendContext Tests
// ============================================================================

#[test]
fn test_gui_backend_context_creation() {
    use nvcontrol::nvml_backend::GuiBackendContext;

    // Test mock context creation
    let ctx = GuiBackendContext::mock();

    assert!(ctx.is_nvml_available());
    assert_eq!(ctx.device_count, 1);
    assert!(!ctx.driver_version.is_empty());

    // Verify we can get metrics through the context
    let metrics = ctx.get_metrics(0).unwrap();
    assert!(metrics.temperature > 0);
    assert!(metrics.gpu_clock_mhz > 0);
}

#[test]
fn test_gui_backend_context_with_multi_gpu() {
    use nvcontrol::display_backend::create_mock_runner_x11;
    use nvcontrol::nvml_backend::{GuiBackendContext, MockNvmlBackend};
    use std::sync::Arc;

    // Create context with custom multi-GPU mock
    let nvml = Arc::new(MockNvmlBackend::multi_gpu(3));
    let display = create_mock_runner_x11();
    let ctx = GuiBackendContext::with_backends(nvml, display);

    assert_eq!(ctx.device_count, 3);

    // Verify all GPUs are accessible
    for i in 0..3 {
        let info = ctx.get_device_info(i).unwrap();
        assert_eq!(info.index, i);
    }
}

#[test]
fn test_tui_app_creation() {
    use nvcontrol::tui::{TuiApp, ViewMode};

    // Create TuiApp with view mode
    let _app = TuiApp::with_view(ViewMode::Nvtop);

    // Test passes if TuiApp is created successfully without panicking
    // Note: Backend injection via with_context() is planned for future versions
}
