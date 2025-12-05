use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaInfo {
    pub version: String,
    pub driver_version: String,
    pub runtime_version: Option<String>,
    pub devices: Vec<CudaDevice>,
    pub toolkit_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaDevice {
    pub id: u32,
    pub name: String,
    pub compute_capability: String,
    pub memory_total: u64,
    pub memory_free: u64,
    pub cuda_cores: Option<u32>,
}

/// Check if CUDA is available on the system
pub fn is_cuda_available() -> bool {
    Command::new("nvcc")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get comprehensive CUDA information
pub fn get_cuda_info() -> NvResult<CudaInfo> {
    let mut info = CudaInfo {
        version: "Unknown".to_string(),
        driver_version: "Unknown".to_string(),
        runtime_version: None,
        devices: Vec::new(),
        toolkit_path: None,
    };

    // Get CUDA driver version via nvidia-smi
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=driver_version"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            info.driver_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    // Get CUDA toolkit version
    if let Ok(output) = Command::new("nvcc").arg("--version").output() {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if let Some(version_line) = output_str.lines().find(|line| line.contains("release")) {
                if let Some(version) = extract_cuda_version(version_line) {
                    info.version = version;
                }
            }
        }
    }

    // Get CUDA devices information
    info.devices = get_cuda_devices()?;

    // Find CUDA toolkit path
    info.toolkit_path = find_cuda_toolkit_path();

    Ok(info)
}

fn extract_cuda_version(version_line: &str) -> Option<String> {
    // Extract version from line like "Cuda compilation tools, release 12.0, V12.0.76"
    if let Some(start) = version_line.find("release ") {
        let start = start + 8;
        if let Some(end) = version_line[start..].find(',') {
            return Some(version_line[start..start + end].to_string());
        }
    }
    None
}

fn get_cuda_devices() -> NvResult<Vec<CudaDevice>> {
    let mut devices = Vec::new();

    // Use nvidia-smi to get device information
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=index,name,memory.total,memory.free"])
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if let Some(device) = parse_device_line(line) {
                    devices.push(device);
                }
            }
        }
    }

    Ok(devices)
}

// Fix CUDA parse_device_line function to avoid move error
fn parse_device_line(line: &str) -> Option<CudaDevice> {
    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
    if parts.len() >= 4 {
        let id = parts[0].parse().ok()?;
        let name = parts[1].to_string();
        let memory_total = parts[2].parse::<u64>().ok()? * 1024 * 1024; // MB to bytes
        let memory_free = parts[3].parse::<u64>().ok()? * 1024 * 1024; // MB to bytes

        // Estimate compute capability based on GPU name (simplified)
        let compute_capability = estimate_compute_capability(&name);

        Some(CudaDevice {
            id,
            name: name.clone(),
            compute_capability,
            memory_total,
            memory_free,
            cuda_cores: estimate_cuda_cores(&name),
        })
    } else {
        None
    }
}

fn estimate_compute_capability(gpu_name: &str) -> String {
    // Simple heuristic based on GPU generation
    if gpu_name.contains("RTX 50") {
        "9.0".to_string() // Blackwell architecture
    } else if gpu_name.contains("RTX 40") {
        "8.9".to_string() // Ada Lovelace
    } else if gpu_name.contains("RTX 30") {
        "8.6".to_string() // Ampere
    } else if gpu_name.contains("RTX 20") || gpu_name.contains("GTX 16") {
        "7.5".to_string() // Turing
    } else if gpu_name.contains("GTX 10") {
        "6.1".to_string() // Pascal
    } else {
        "Unknown".to_string()
    }
}

fn estimate_cuda_cores(gpu_name: &str) -> Option<u32> {
    // Rough estimates for common GPUs
    // RTX 50 Series (Blackwell)
    if gpu_name.contains("RTX 5090") {
        Some(21760) // 170 SMs × 128 cores per SM
    } else if gpu_name.contains("RTX 5080") {
        Some(10752) // 84 SMs × 128 cores per SM
    } else if gpu_name.contains("RTX 5070 Ti") {
        Some(8960) // 70 SMs × 128 cores per SM
    } else if gpu_name.contains("RTX 5070") {
        Some(6144) // 48 SMs × 128 cores per SM
    } else if gpu_name.contains("RTX 5060 Ti") {
        Some(4608) // 36 SMs × 128 cores per SM
    } else if gpu_name.contains("RTX 5060") {
        Some(3584) // 28 SMs × 128 cores per SM
    // RTX 40 Series (Ada Lovelace)
    } else if gpu_name.contains("RTX 4090") {
        Some(16384)
    } else if gpu_name.contains("RTX 4080") {
        Some(9728)
    } else if gpu_name.contains("RTX 4070 Ti") {
        Some(7680)
    } else if gpu_name.contains("RTX 4070") {
        Some(5888)
    } else if gpu_name.contains("RTX 4060 Ti") {
        Some(4352)
    } else if gpu_name.contains("RTX 4060") {
        Some(3072)
    // RTX 30 Series (Ampere)
    } else if gpu_name.contains("RTX 3090") {
        Some(10496)
    } else if gpu_name.contains("RTX 3080") {
        Some(8704)
    } else if gpu_name.contains("RTX 3070") {
        Some(5888)
    } else if gpu_name.contains("RTX 3060") {
        Some(3584)
    } else {
        None
    }
}

fn find_cuda_toolkit_path() -> Option<PathBuf> {
    let possible_paths = vec!["/usr/local/cuda", "/opt/cuda", "/usr/lib/cuda"];

    for path in possible_paths {
        let path = PathBuf::from(path);
        if path.exists() && path.join("bin/nvcc").exists() {
            return Some(path);
        }
    }

    None
}

/// Get CUDA development environment information
pub fn get_cuda_dev_info() -> NvResult<()> {
    println!("CUDA Development Environment:");
    println!("============================");

    let info = get_cuda_info()?;

    println!("CUDA Toolkit Version: {}", info.version);
    println!("Driver Version: {}", info.driver_version);

    if let Some(toolkit_path) = info.toolkit_path {
        println!("Toolkit Path: {}", toolkit_path.display());
    } else {
        println!("Toolkit Path: Not found");
    }

    println!("\nCUDA Devices:");
    for device in info.devices {
        println!("  Device {}: {}", device.id, device.name);
        println!("    Compute Capability: {}", device.compute_capability);
        println!(
            "    Memory: {:.1} GB total, {:.1} GB free",
            device.memory_total as f64 / (1024.0 * 1024.0 * 1024.0),
            device.memory_free as f64 / (1024.0 * 1024.0 * 1024.0)
        );
        if let Some(cores) = device.cuda_cores {
            println!("    CUDA Cores: {}", cores);
        }
        println!();
    }

    // Check for development tools
    println!("Development Tools:");
    check_cuda_tool("nvcc", "CUDA Compiler");
    check_cuda_tool("nsight", "Nsight Debugging");
    check_cuda_tool("nvprof", "CUDA Profiler");
    check_cuda_tool("ncu", "Nsight Compute");
    check_cuda_tool("nsys", "Nsight Systems");

    Ok(())
}

fn check_cuda_tool(tool: &str, description: &str) {
    let available = Command::new(tool)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    let status = if available {
        "✓ Available"
    } else {
        "✗ Not found"
    };
    println!("  {}: {}", description, status);
}

/// Install CUDA development tools
pub fn install_cuda_toolkit() -> NvResult<()> {
    println!("Installing CUDA Toolkit...");

    // This would need to be implemented per-distro
    println!("Please install CUDA Toolkit manually:");
    println!("  • Download from https://developer.nvidia.com/cuda-downloads");
    println!("  • Or use your package manager:");
    println!("    - Ubuntu: sudo apt install nvidia-cuda-toolkit");
    println!("    - Arch: sudo pacman -S cuda");
    println!("    - Fedora: sudo dnf install cuda");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuda_availability() {
        // Should not crash even if CUDA is not available
        let available = is_cuda_available();
        // Just verify function runs without panic - result depends on system
        let _ = available;
    }

    #[test]
    fn test_version_extraction() {
        let line = "Cuda compilation tools, release 12.0, V12.0.76";
        assert_eq!(extract_cuda_version(line), Some("12.0".to_string()));
    }

    #[test]
    fn test_compute_capability_estimation() {
        assert_eq!(estimate_compute_capability("RTX 4090"), "8.9");
        assert_eq!(estimate_compute_capability("RTX 3080"), "8.6");
        assert_eq!(estimate_compute_capability("Unknown GPU"), "Unknown");
    }
}
