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
fn dev_cli_script_propagates_failures() {
    for (binary, expected) in [("/bin/true", 0), ("/bin/false", 1)] {
        let output = std::process::Command::new("bash")
            .arg("dev/test-cli.sh")
            .env("NVCTL", binary)
            .env_remove("DISPLAY")
            .env_remove("WAYLAND_DISPLAY")
            .output()
            .expect("Run headless CLI gate with controlled command status");
        assert_eq!(output.status.code(), Some(expected));
    }
}

#[test]
fn dev_hardware_mutation_requires_explicit_opt_in() {
    let output = std::process::Command::new("bash")
        .args(["dev/test-hardware.sh", "--vibrance"])
        .env_remove("NVCONTROL_RUN_HARDWARE_TESTS")
        .output()
        .expect("Run mutation gate without permission");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("opt into"));
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
    use nvcontrol::vibrance_native::{
        NativeVibranceController, percentage_to_vibrance, vibrance_to_percentage,
    };
    if std::env::var("NVCONTROL_RUN_HARDWARE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live vibrance regression; set NVCONTROL_RUN_HARDWARE_TESTS=1");
        return;
    }

    struct RestoreVibrance {
        controller: NativeVibranceController,
        values: Vec<(usize, i64)>,
    }
    impl Drop for RestoreVibrance {
        fn drop(&mut self) {
            for &(index, raw) in &self.values {
                if let Err(error) = self.controller.set_vibrance(index, raw) {
                    eprintln!("Failed to restore display {index} to raw {raw}: {error}");
                }
            }
        }
    }

    let controller = NativeVibranceController::new().expect("Open live NVKMS controller");
    let values: Vec<_> = controller
        .connectors
        .iter()
        .filter(|c| c.connected)
        .map(|c| {
            (
                c.connector_index as usize,
                c.current_vibrance
                    .expect("Read original vibrance before changing it"),
            )
        })
        .collect();
    assert!(!values.is_empty(), "No connected displays to test");
    let original = values.clone();
    let mut restore = RestoreVibrance { controller, values };
    for &(index, raw) in &original {
        let percentage = vibrance_to_percentage(raw);
        let target = if percentage < 200 {
            percentage + 1
        } else {
            percentage - 1
        };
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_nvctl"))
            .args([
                "display",
                "vibrance",
                "set-display",
                &index.to_string(),
                &target.to_string(),
            ])
            .output()
            .expect("Execute built CLI vibrance command");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let readback =
            NativeVibranceController::new().expect("Reopen controller for hardware readback");
        assert_eq!(
            readback.connectors[index].current_vibrance,
            Some(percentage_to_vibrance(target))
        );
        restore
            .controller
            .set_vibrance(index, raw)
            .expect("Restore original raw value");
        eprintln!("Display {index}: raw {raw} -> {target}% -> raw {raw}");
    }
    drop(restore);
    let restored = NativeVibranceController::new().expect("Verify final restored values");
    for (index, raw) in original {
        assert_eq!(restored.connectors[index].current_vibrance, Some(raw));
    }
}
