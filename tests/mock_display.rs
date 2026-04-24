//! Deterministic display-facing tests using mock runners.

use nvcontrol::display_backend::{
    DisplayCommandRunner, DisplayError, DisplayServer, MockDisplayRunner,
};

#[test]
fn test_mock_x11_display_detection() {
    let runner = MockDisplayRunner::x11();

    assert_eq!(runner.get_display_server().unwrap(), DisplayServer::X11);
    assert!(runner.command_available("xrandr"));
    assert!(runner.command_available("nvidia-settings"));
    assert!(!runner.command_available("wlr-randr"));
    assert!(!runner.command_available("wayland-info"));
}

#[test]
fn test_mock_wayland_display_detection() {
    let runner = MockDisplayRunner::wayland();

    assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Wayland);
    assert!(runner.command_available("wlr-randr"));
    assert!(runner.command_available("wayland-info"));
    assert!(!runner.command_available("xrandr"));
}

#[test]
fn test_mock_xrandr_output_parsing() {
    let runner = MockDisplayRunner::x11();
    let output = runner.run_xrandr(&[]).unwrap();

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
    let result = runner.run_wlr_randr(&[]);
    assert!(matches!(result, Err(DisplayError::BinaryMissing(_))));

    let runner = MockDisplayRunner::wayland();
    let result = runner.run_xrandr(&[]);
    assert!(matches!(result, Err(DisplayError::BinaryMissing(_))));
}

#[test]
fn test_mock_headless_environment() {
    let runner = MockDisplayRunner::headless();

    assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Unknown);
    assert!(!runner.command_available("xrandr"));
    assert!(!runner.command_available("wlr-randr"));
    assert!(!runner.command_available("nvidia-settings"));
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

#[test]
fn test_display_detection_flow() {
    let runner = MockDisplayRunner::x11();

    match runner.get_display_server().unwrap() {
        DisplayServer::X11 => {
            let output = runner.run_xrandr(&[]).unwrap();
            assert!(output.contains("connected"));
        }
        DisplayServer::Wayland => {
            let output = runner.run_wlr_randr(&[]).unwrap();
            assert!(output.contains("Enabled"));
        }
        DisplayServer::Unknown => {}
    }
}

#[test]
fn test_vibrance_query_output() {
    let runner = MockDisplayRunner::x11();

    if runner.command_available("nvidia-settings") {
        let output = runner
            .run_nvidia_settings(&["-q", "DigitalVibrance"])
            .unwrap();

        assert!(output.contains("DigitalVibrance"));
        assert!(output.contains("0"));
    }
}

#[test]
fn test_kde_display_runner() {
    let runner = MockDisplayRunner::kde();

    assert!(runner.command_available("kscreen-doctor"));
    assert!(runner.command_available("qdbus"));
    assert!(runner.command_available("wayland-info"));
    assert_eq!(runner.get_compositor(), "kde");

    let output = runner.run_command("kscreen-doctor", &["-j"]).unwrap();
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

    assert!(runner.command_available("gsettings"));
    assert_eq!(runner.get_compositor(), "gnome");

    let output = runner
        .run_command(
            "gsettings",
            &["get", "org.gnome.mutter", "experimental-features"],
        )
        .unwrap();

    assert!(output.contains("variable-refresh-rate"));
}

#[test]
fn test_hyprland_display_runner() {
    let runner = MockDisplayRunner::hyprland();

    assert!(runner.command_available("hyprctl"));
    assert_eq!(runner.get_compositor(), "hyprland");

    let output = runner.run_command("hyprctl", &["monitors", "-j"]).unwrap();
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
fn test_vrr_detection_flow_kde() {
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
                assert_eq!(vrr_policy, 0);
            }
        }
    }
}

#[test]
fn test_hdr_detection_flow_hyprland() {
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

            let hdr_enabled =
                current_format.contains("101010") || current_format.contains("16161616");

            assert_eq!(name, "DP-1");
            assert!(!hdr_enabled);
        }
    }
}

#[test]
fn test_custom_compositor_mock() {
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
    let runner = MockDisplayRunner::x11();
    assert!(runner.command_available("xrandr"));

    let runner_limited = MockDisplayRunner::headless();
    assert!(runner_limited.run_nvidia_settings(&[]).is_err());
}

#[test]
fn test_mock_display_vrr_mode_values() {
    let kde_runner = MockDisplayRunner::kde();
    let output = kde_runner.run_command("kscreen-doctor", &["-j"]).unwrap();
    let json: serde_json::Value = serde_json::from_str(&output).unwrap();

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

    if let Some(modes) = monitors[0].get("availableModes").and_then(|m| m.as_array()) {
        let first_mode = modes[0].as_str().unwrap();
        assert!(first_mode.contains("@"));
    }
}
