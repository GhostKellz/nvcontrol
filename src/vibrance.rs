use std::process::Command;

/// Set vibrance for each display using nVibrant.
/// `levels` should be a vector of values (-1024 to 1023) for each display in physical port order.
pub fn set_vibrance(levels: &[i16]) {
    let args: Vec<String> = levels.iter().map(|l| l.to_string()).collect();
    // Try nvibrant first, then fallback to uvx nvibrant
    let status = Command::new("nvibrant")
        .args(&args)
        .status()
        .or_else(|_| Command::new("uvx").arg("nvibrant").args(&args).status());

    match status {
        Ok(s) if s.success() => println!("Vibrance set: {:?}", levels),
        Ok(s) => eprintln!("nvibrant exited with status: {}", s),
        Err(e) => eprintln!(
            "nVibrant not found. Please install from https://github.com/Tremeschin/nVibrant. Error: {}",
            e
        ),
    }
}
