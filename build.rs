use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    // Only build nvibrant on Linux with NVIDIA drivers
    if cfg!(target_os = "linux") {
        let out_dir = env::var("OUT_DIR").unwrap();
        let nvibrant_path = Path::new("vendor/nvibrant");
        
        println!("cargo:rerun-if-changed=vendor/nvibrant");
        println!("cargo:rerun-if-changed=.gitmodules");
        
        // Check if nvibrant directory exists
        if nvibrant_path.exists() && nvibrant_path.join("pyproject.toml").exists() {
            println!("cargo:warning=Building nvibrant integration...");
            
            // Try to install nvibrant automatically
            let install_success = install_nvibrant();
            
            if install_success {
                println!("cargo:warning=✅ nvibrant integrated successfully");
                println!("cargo:rustc-env=NVIBRANT_INTEGRATED=1");
                
                // Try to find nvibrant binary location
                if let Ok(output) = Command::new("which").arg("nvibrant").output() {
                    if output.status.success() {
                        let nvibrant_bin = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        println!("cargo:rustc-env=NVIBRANT_BINARY_PATH={}", nvibrant_bin);
                        
                        // Create a copy in our target directory for bundling
                        let target_nvibrant = format!("{}/nvibrant", out_dir);
                        let _ = std::fs::copy(&nvibrant_bin, &target_nvibrant);
                        
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            if let Ok(metadata) = std::fs::metadata(&target_nvibrant) {
                                let mut perms = metadata.permissions();
                                perms.set_mode(0o755);
                                let _ = std::fs::set_permissions(&target_nvibrant, perms);
                            }
                        }
                    }
                }
            } else {
                println!("cargo:warning=⚠️ nvibrant installation failed - manual setup may be required");
                println!("cargo:warning=Run: pip install -e vendor/nvibrant");
            }
        } else {
            println!("cargo:warning=⚠️ nvibrant submodule not found");
            println!("cargo:warning=Run: git submodule update --init --recursive");
            println!("cargo:warning=Or: ./scripts/setup-nvibrant.sh");
        }
    }
}

fn install_nvibrant() -> bool {
    let nvibrant_path = Path::new("vendor/nvibrant");
    
    // Method 1: Try uv (fastest)
    if Command::new("uv").arg("--version").output().is_ok() {
        println!("cargo:warning=Installing nvibrant with uv...");
        let status = Command::new("uv")
            .args(["pip", "install", "-e", "."])
            .current_dir(nvibrant_path)
            .status();
        
        if status.map(|s| s.success()).unwrap_or(false) {
            return true;
        }
    }
    
    // Method 2: Try pip3
    if Command::new("pip3").arg("--version").output().is_ok() {
        println!("cargo:warning=Installing nvibrant with pip3...");
        let status = Command::new("pip3")
            .args(["install", "-e", ".", "--user"])
            .current_dir(nvibrant_path)
            .status();
        
        if status.map(|s| s.success()).unwrap_or(false) {
            return true;
        }
    }
    
    // Method 3: Try pip
    if Command::new("pip").arg("--version").output().is_ok() {
        println!("cargo:warning=Installing nvibrant with pip...");
        let status = Command::new("pip")
            .args(["install", "-e", ".", "--user"])
            .current_dir(nvibrant_path)
            .status();
        
        if status.map(|s| s.success()).unwrap_or(false) {
            return true;
        }
    }
    
    false
}