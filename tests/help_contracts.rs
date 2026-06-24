mod common;

fn help_output(args: &[&str]) -> String {
    common::run_success(args)
}

#[test]
fn top_level_help_mentions_core_release_surfaces() {
    let stdout = help_output(&["--help"]);
    assert!(stdout.contains("driver"));
    assert!(stdout.contains("doctor"));
    assert!(stdout.contains("setup"));
    assert!(stdout.contains("completion"));
    assert!(stdout.contains("vibrance"));
    assert!(stdout.contains("cuda"));
}

#[test]
fn driver_help_mentions_diagnostics_and_support_bundle() {
    let stdout = help_output(&["driver", "--help"]);
    assert!(stdout.contains("diagnose-release"));
    assert!(stdout.contains("support-bundle"));
    assert!(stdout.contains("dkms"));
    assert!(stdout.contains("source"));
}

#[test]
fn completion_help_mentions_supported_shells() {
    let stdout = help_output(&["completion", "--help"]);
    assert!(stdout.contains("bash"));
    assert!(stdout.contains("zsh"));
    assert!(stdout.contains("fish"));
}

#[test]
fn fan_help_mentions_auto_and_curve_workflows() {
    let stdout = help_output(&["fan", "--help"]);
    assert!(stdout.contains("info"));
    assert!(stdout.contains("set"));
    assert!(stdout.contains("auto"));
    assert!(stdout.contains("curve"));

    let curve = help_output(&["fan", "curve", "--help"]);
    assert!(curve.contains("show"));
    assert!(curve.contains("apply"));
    assert!(curve.contains("set"));
}

#[test]
fn setup_help_mentions_readiness_and_permissions() {
    let stdout = help_output(&["setup", "--help"]);
    assert!(stdout.contains("check"));
    assert!(stdout.contains("permissions"));
    assert!(stdout.contains("status"));
    assert!(stdout.contains("remove"));
}

#[test]
fn config_help_mentions_capture_preview_and_apply() {
    let stdout = help_output(&["config", "--help"]);
    assert!(stdout.contains("capture"));
    assert!(stdout.contains("preview"));
    assert!(stdout.contains("apply"));
    assert!(stdout.contains("diff"));
}

#[test]
fn gaming_auto_help_mentions_daemon_and_service_flow() {
    let stdout = help_output(&["gaming", "auto", "--help"]);
    assert!(stdout.contains("start"));
    assert!(stdout.contains("status"));
    assert!(stdout.contains("install-service"));
    assert!(stdout.contains("enable-service"));
}

#[test]
fn cuda_help_mentions_ollama_and_workloads() {
    let stdout = help_output(&["cuda", "--help"]);
    assert!(stdout.contains("ollama"));
    assert!(stdout.contains("workloads"));
    assert!(stdout.contains("tools"));
    assert!(stdout.contains("env"));
    assert!(stdout.contains("smoke"));
}

#[test]
fn ai_alias_exposes_cuda_workflows() {
    let stdout = help_output(&["ai", "--help"]);
    assert!(stdout.contains("doctor"));
    assert!(stdout.contains("ollama"));
    assert!(stdout.contains("workloads"));
    assert!(stdout.contains("env"));
    assert!(stdout.contains("smoke"));
}
