use crate::notifications::NotificationManager;
use crate::{NvControlError, NvResult};
use std::process::Command;

pub fn send_test_notification() -> NvResult<()> {
    NotificationManager::new()?.send_test_notification()
}

pub fn open_docs() -> NvResult<()> {
    let status = Command::new("xdg-open")
        .arg("docs/README.md")
        .status()
        .map_err(|e| NvControlError::CommandFailed(format!("Failed to launch xdg-open: {}", e)))?;

    if status.success() {
        Ok(())
    } else {
        Err(NvControlError::CommandFailed(format!(
            "xdg-open exited with status {}",
            status
        )))
    }
}

pub fn open_path(path: &str) -> NvResult<()> {
    let status = Command::new("xdg-open")
        .arg(path)
        .status()
        .map_err(|e| NvControlError::CommandFailed(format!("Failed to launch xdg-open: {}", e)))?;

    if status.success() {
        Ok(())
    } else {
        Err(NvControlError::CommandFailed(format!(
            "xdg-open exited with status {}",
            status
        )))
    }
}
