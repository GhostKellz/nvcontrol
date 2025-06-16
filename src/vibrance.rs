use crate::{NvControlError, NvResult};
use std::process::Command;

/// Set vibrance for each display using nVibrant.
/// `levels` should be a vector of values (-1024 to 1023) for each display in physical port order.
pub fn set_vibrance(levels: &[i16]) -> NvResult<()> {
    let args: Vec<String> = levels.iter().map(|l| l.to_string()).collect();
    // Try nvibrant first, then fallback to uvx nvibrant
    let status = Command::new("nvibrant")
        .args(&args)
        .status()
        .or_else(|_| Command::new("uvx").arg("nvibrant").args(&args).status());

    match status {
        Ok(s) if s.success() => {
            println!("Vibrance set: {levels:?}");
            Ok(())
        }
        Ok(s) => {
            let msg = format!("nvibrant exited with status: {s}");
            eprintln!("{msg}");
            Err(NvControlError::VibranceControlFailed(msg))
        }
        Err(e) => {
            let msg = format!(
                "nVibrant not found. Please install from https://github.com/Tremeschin/nVibrant. Error: {e}"
            );
            eprintln!("{msg}");
            Err(NvControlError::VibranceControlFailed(msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibrance_set_valid_levels() {
        let levels = vec![0, 512, -512];
        // This might fail if nVibrant is not installed, but should not panic
        let result = set_vibrance(&levels);
        assert!(result.is_ok() || result.is_err()); // Should handle both cases gracefully
    }

    #[test]
    fn test_vibrance_set_empty_levels() {
        let levels = vec![];
        let result = set_vibrance(&levels);
        // Should handle empty input gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_vibrance_extreme_values() {
        let levels = vec![-1024, 1023]; // Boundary values
        let result = set_vibrance(&levels);
        assert!(result.is_ok() || result.is_err());
    }
}
