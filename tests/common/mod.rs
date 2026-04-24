#![allow(dead_code)]

use assert_cmd::cargo::CommandCargoExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use tempfile::Builder;

pub fn nvctl_command() -> Command {
    let mut cmd = Command::cargo_bin("nvctl").expect("Failed to locate nvctl test binary");
    cmd.env("NVCONTROL_SUPPRESS_SUPPORT_NOTIFICATIONS", "1");
    cmd
}

pub fn output_text(output: std::process::Output) -> String {
    String::from_utf8(output.stdout).expect("Command output was not valid UTF-8")
}

pub fn run_success(args: &[&str]) -> String {
    let output = nvctl_command()
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute {:?}", args));
    assert!(output.status.success(), "command failed for {:?}", args);
    output_text(output)
}

pub fn wait_for_path(path: &Path) -> bool {
    for _ in 0..20 {
        if path.exists() {
            return true;
        }
        thread::sleep(Duration::from_millis(50));
    }
    false
}

pub fn temp_output_path(prefix: &str, suffix: &str) -> PathBuf {
    let file = Builder::new()
        .prefix(prefix)
        .suffix(suffix)
        .tempfile()
        .expect("Failed to create temp output path");
    let path = file.path().to_path_buf();
    drop(file);
    let _ = std::fs::remove_file(&path);
    path
}
