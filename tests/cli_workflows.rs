mod common;

#[test]
fn test_nvctl_help() {
    let stdout = common::run_success(&["--help"]);
    assert!(stdout.contains("Advanced command-line interface for NVIDIA GPU control"));
    assert!(stdout.contains("Usage: nvctl [OPTIONS] <COMMAND>"));
}

#[test]
fn test_nvctl_gpu_info() {
    let output = common::nvctl_command()
        .args(["gpu", "info"])
        .output()
        .expect("Failed to execute nvctl gpu info");

    assert!(output.status.success() || output.status.code() == Some(1));
}

#[test]
fn test_nvctl_display_ls() {
    let stdout = common::run_success(&["display", "ls"]);
    assert!(stdout.contains("Detected"));
    assert!(stdout.contains("display(s)"));
}

#[test]
fn test_nvctl_fan_info() {
    let stdout = common::run_success(&["fan", "info"]);
    assert!(stdout.contains("Fan Information"));
}

#[test]
fn test_nvctl_driver_info() {
    let stdout = common::run_success(&["driver", "info"]);
    assert!(stdout.contains("Driver"));
    assert!(stdout.contains("Module Type:"));
}

#[test]
fn test_nvctl_driver_validate() {
    let stdout = common::run_success(&["driver", "validate", "--driver", "590"]);
    assert!(stdout.contains("System Validation for Driver"));
    assert!(stdout.contains("590"));
}

#[test]
fn test_nvctl_driver_diagnose_release() {
    let stdout = common::run_success(&["driver", "diagnose-release"]);
    assert!(stdout.contains("Release Diagnostics"));
}

#[test]
fn test_nvctl_driver_diagnose_release_json() {
    let stdout = common::run_success(&["driver", "diagnose-release", "--format", "json"]);
    assert!(stdout.contains("running_kernel"));
    assert!(stdout.contains("firmware_layout"));
}

#[test]
fn test_nvctl_driver_diagnose_release_yaml() {
    let stdout = common::run_success(&["driver", "diagnose-release", "--format", "yaml"]);
    assert!(stdout.contains("running_kernel:"));
}

#[test]
fn test_nvctl_companion_help() {
    let stdout = common::run_success(&["companion", "--help"]);
    assert!(stdout.contains("Lightweight desktop companion actions"));
    assert!(stdout.contains("notify-test"));
}

#[test]
fn test_nvctl_driver_support_bundle() {
    let output_path = common::temp_output_path("nvcontrol-test-", ".txt");
    let metadata_path = output_path.with_extension("txt.json");
    let output = common::nvctl_command()
        .args([
            "driver",
            "support-bundle",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute nvctl driver support-bundle");

    assert!(output.status.success());
    assert!(common::wait_for_path(&output_path));
    let bundle = std::fs::read_to_string(&output_path).unwrap();
    assert!(bundle.contains("nvcontrol support bundle"));
    assert!(common::wait_for_path(&metadata_path));
    let metadata = std::fs::read_to_string(&metadata_path).unwrap();
    assert!(metadata.contains("release_diagnostics"));
    let _ = std::fs::remove_file(&output_path);
    let _ = std::fs::remove_file(&metadata_path);
}

#[test]
fn test_nvctl_driver_support_bundle_gzip() {
    let output_path = common::temp_output_path("nvcontrol-test-", ".txt.gz");
    let metadata_path = output_path.with_extension("txt.json");
    let output = common::nvctl_command()
        .args([
            "driver",
            "support-bundle",
            "--output",
            output_path.to_str().unwrap(),
            "--gzip",
            "--redact-paths",
        ])
        .output()
        .expect("Failed to execute nvctl driver support-bundle --gzip");

    assert!(output.status.success());
    assert!(common::wait_for_path(&output_path));
    assert!(!common::wait_for_path(&metadata_path));
    let _ = std::fs::remove_file(&output_path);
}

#[test]
fn test_nvctl_driver_support_bundle_tarball() {
    let output_path = common::temp_output_path("nvcontrol-test-", ".tar.gz");
    let output = common::nvctl_command()
        .args([
            "driver",
            "support-bundle",
            "--output",
            output_path.to_str().unwrap(),
            "--tarball",
            "--redact-paths",
            "--redact-ids",
        ])
        .output()
        .expect("Failed to execute nvctl driver support-bundle --tarball");

    assert!(output.status.success());
    assert!(common::wait_for_path(&output_path));
    let _ = std::fs::remove_file(&output_path);
}

#[test]
fn test_nvctl_driver_dkms_doctor() {
    let stdout = common::run_success(&["driver", "dkms", "doctor"]);
    assert!(stdout.contains("NVIDIA DKMS Doctor"));
}

#[test]
fn test_nvctl_driver_source_doctor() {
    let stdout = common::run_success(&["driver", "source", "doctor"]);
    assert!(stdout.contains("NVIDIA Source Build Doctor"));
}

#[test]
fn test_nvctl_container_runtime_doctor() {
    let stdout = common::run_success(&["container", "runtime", "doctor", "--runtime", "docker"]);
    assert!(stdout.contains("NVIDIA Container Runtime Doctor"));
    assert!(stdout.contains("Severity:"));
}

#[test]
fn test_nvctl_config_capture_and_preview_live() {
    let profile_name = format!(
        "test-live-capture-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );

    let capture = common::nvctl_command()
        .args(["config", "capture", "--name", &profile_name])
        .output()
        .expect("Failed to execute nvctl config capture");
    assert!(capture.status.success());

    let preview = common::nvctl_command()
        .args(["config", "preview", "--input", "live"])
        .output()
        .expect("Failed to execute nvctl config preview --input live");
    assert!(preview.status.success());
    let stdout = String::from_utf8(preview.stdout).unwrap();
    assert!(stdout.contains("Profile Bundle") || stdout.contains("Display Layout"));
}

#[test]
fn test_nvctl_doctor_support() {
    let output_path = common::temp_output_path("nvcontrol-test-", "-doctor.tar.gz");
    let _output = common::nvctl_command()
        .args([
            "doctor",
            "--support",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute nvctl doctor --support");

    assert!(common::wait_for_path(&output_path));
    let _ = std::fs::remove_file(&output_path);
}

#[test]
fn test_nvctl_doctor_support_json() {
    let output_path = common::temp_output_path("nvcontrol-test-", "-doctor-json.tar.gz");
    let output = common::nvctl_command()
        .args([
            "doctor",
            "--support",
            "--format",
            "json",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute nvctl doctor --support --format json");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("\"severity\""));
    assert!(stdout.contains("Release Diagnostics"));
    let _ = std::fs::remove_file(&output_path);
}
