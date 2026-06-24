mod common;

#[test]
fn completion_output_does_not_expose_removed_top_level_drivers_command() {
    let output = common::nvctl_command()
        .args(["completion", "bash"])
        .output()
        .expect("Failed to generate bash completions");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.contains(" generate-completions "));
    assert!(!stdout.contains(" drivers "));
}

#[test]
fn completions_include_current_setup_command_for_all_shells() {
    for shell in ["bash", "zsh", "fish"] {
        let output = common::nvctl_command()
            .args(["completion", shell])
            .output()
            .unwrap_or_else(|_| panic!("Failed to generate {shell} completions"));

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("setup"));
        assert!(stdout.contains("driver"));
        assert!(stdout.contains("vibrance"));
    }
}

#[test]
fn current_help_does_not_regress_to_old_driver_baselines() {
    for args in [
        vec!["vibrance", "--help"],
        vec!["display", "vibrance", "info"],
        vec!["driver", "validate", "--driver", "610"],
    ] {
        let output = common::nvctl_command()
            .args(args)
            .output()
            .expect("Failed to execute nvctl command");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{stdout}\n{stderr}");
        assert!(!combined.contains("580+"));
        assert!(!combined.contains("590+ required"));
    }
}

#[test]
fn top_level_help_does_not_expose_removed_gsp_alias() {
    let output = common::nvctl_command()
        .arg("--help")
        .output()
        .expect("Failed to execute nvctl --help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        !stdout
            .lines()
            .any(|line| line.trim_start().starts_with("gsp"))
    );
    assert!(stdout.contains("driver"));
}

#[test]
fn invalid_removed_gsp_command_fails_cleanly() {
    let output = common::nvctl_command()
        .args(["gsp", "status"])
        .output()
        .expect("Failed to execute nvctl gsp status");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unrecognized subcommand") || stderr.contains("Usage:"));
}

#[test]
fn support_bundle_plain_text_still_writes_metadata_sidecar() {
    let output_path = common::temp_output_path("nvcontrol-regression-", ".txt");
    let metadata_path = output_path.with_extension("txt.json");
    let output = common::nvctl_command()
        .args([
            "driver",
            "support-bundle",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute support bundle command");

    assert!(output.status.success());
    assert!(output_path.exists());
    assert!(metadata_path.exists());
    let metadata = std::fs::read_to_string(metadata_path).unwrap();
    assert!(metadata.contains("release_diagnostics"));
    assert!(metadata.contains("cuda_ai_diagnostics"));
}

#[test]
fn dev_scripts_gate_hardware_mutating_vibrance_tests() {
    for script in ["dev/test-cli.sh", "dev/test-all.sh"] {
        let content = std::fs::read_to_string(script).unwrap();
        assert!(
            !content.contains("vibrance 100") || content.contains("NVCONTROL_RUN_HARDWARE_TESTS"),
            "{script} must gate live vibrance mutation behind NVCONTROL_RUN_HARDWARE_TESTS"
        );
    }
}

#[test]
fn support_bundle_gzip_does_not_write_metadata_sidecar() {
    let output_path = common::temp_output_path("nvcontrol-regression-", ".txt.gz");
    let metadata_path = output_path.with_extension("txt.json");
    let output = common::nvctl_command()
        .args([
            "driver",
            "support-bundle",
            "--output",
            output_path.to_str().unwrap(),
            "--gzip",
        ])
        .output()
        .expect("Failed to execute support bundle gzip command");

    assert!(output.status.success());
    assert!(output_path.exists());
    assert!(!metadata_path.exists());
}

#[test]
fn config_preview_live_still_reports_bundle_content() {
    let output = common::nvctl_command()
        .args(["config", "preview", "--input", "live"])
        .output()
        .expect("Failed to execute config preview");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Profile Bundle") || stdout.contains("Display Layout"));
}

#[test]
fn power_persistence_uses_explicit_enabled_flag() {
    let output = common::nvctl_command()
        .args(["power", "persistence", "--help"])
        .output()
        .expect("Failed to execute power persistence help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--enabled"));
    assert!(!stdout.contains("<on|off>"));
}

#[test]
fn monitors_set_vrr_uses_explicit_enabled_flag() {
    let output = common::nvctl_command()
        .args(["monitors", "set-vrr", "--help"])
        .output()
        .expect("Failed to execute monitors set-vrr help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--enabled"));
}

#[test]
fn vibrance_alias_still_works() {
    let output = common::nvctl_command()
        .args(["vibe", "--help"])
        .output()
        .expect("Failed to execute vibrance alias help");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Vibrance percentage"));
}

#[test]
#[ignore = "mutates live display vibrance; run explicitly with NVCONTROL_RUN_HARDWARE_TESTS=1"]
fn live_vibrance_levels_apply_once() {
    if std::env::var("NVCONTROL_RUN_HARDWARE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live vibrance regression; set NVCONTROL_RUN_HARDWARE_TESTS=1");
        return;
    }

    for level in ["100", "150", "200"] {
        let output = common::nvctl_command()
            .args(["vibrance", level])
            .output()
            .unwrap_or_else(|_| panic!("Failed to set vibrance level {level}"));

        assert!(
            output.status.success(),
            "vibrance level {level} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
