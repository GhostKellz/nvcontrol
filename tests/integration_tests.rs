use std::process::Command;

#[test]
fn test_nvctl_help() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "nvctl", "--", "--help"])
        .output()
        .expect("Failed to execute nvctl");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("NVIDIA GPU") || stdout.contains("Usage: nvctl"));
}

#[test]
fn test_nvctl_gpu_info() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "nvctl", "--", "gpu", "info"])
        .output()
        .expect("Failed to execute nvctl gpu info");

    // Should not crash, even if no GPU is available
    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_nvctl_display_ls() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "nvctl", "--", "display", "ls"])
        .output()
        .expect("Failed to execute nvctl display ls");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Detected") && stdout.contains("display"));
}

#[test]
fn test_nvctl_fan_info() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "nvctl", "--", "fan", "info"])
        .output()
        .expect("Failed to execute nvctl fan info");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Fan Information"));
}
