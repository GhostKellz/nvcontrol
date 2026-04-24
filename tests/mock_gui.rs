//! Deterministic GUI-context tests using mock backends.

#[test]
fn test_gui_backend_context_creation() {
    use nvcontrol::nvml_backend::GuiBackendContext;

    let ctx = GuiBackendContext::mock();

    assert!(ctx.is_nvml_available());
    assert_eq!(ctx.device_count, 1);
    assert!(!ctx.driver_version.is_empty());

    let metrics = ctx.get_metrics(0).unwrap();
    assert!(metrics.temperature > 0);
    assert!(metrics.gpu_clock_mhz > 0);
}

#[test]
fn test_gui_backend_context_with_multi_gpu() {
    use nvcontrol::display_backend::create_mock_runner_x11;
    use nvcontrol::nvml_backend::{GuiBackendContext, MockNvmlBackend};
    use std::sync::Arc;

    let nvml = Arc::new(MockNvmlBackend::multi_gpu(3));
    let display = create_mock_runner_x11();
    let ctx = GuiBackendContext::with_backends(nvml, display);

    assert_eq!(ctx.device_count, 3);

    for i in 0..3 {
        let info = ctx.get_device_info(i).unwrap();
        assert_eq!(info.index, i);
    }
}

#[test]
fn test_tui_app_creation() {
    use nvcontrol::tui::{TuiApp, ViewMode};

    let _app = TuiApp::with_view(ViewMode::Nvtop);
}
