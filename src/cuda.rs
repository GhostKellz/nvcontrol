use crate::NvResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatus {
    pub name: String,
    pub available: bool,
    pub path: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaCudaStatus {
    pub cli: ToolStatus,
    pub service_reachable: bool,
    pub docker: ToolStatus,
    pub nvidia_ctk: ToolStatus,
    pub nvidia_container_runtime_ready: bool,
    pub gpu_devices: usize,
    pub gpu_memory_total_gb: f64,
    pub native_env: Vec<(String, String)>,
    pub docker_run_command: String,
    pub smoke_test_command: String,
    pub issues: Vec<String>,
    pub fixes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiWorkloadRecommendation {
    pub workload: String,
    pub fit: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaDoctorReport {
    pub cuda: CudaInfo,
    pub tools: Vec<ToolStatus>,
    pub ollama: OllamaCudaStatus,
    pub ai_recommendations: Vec<AiWorkloadRecommendation>,
    pub issues: Vec<String>,
    pub fixes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaEnvReport {
    pub exports: Vec<(String, String)>,
    pub shell_lines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaSmokePlan {
    pub dry_run: bool,
    pub native_checks: Vec<String>,
    pub container_checks: Vec<String>,
    pub notes: Vec<String>,
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

pub fn collect_cuda_doctor() -> NvResult<CudaDoctorReport> {
    let cuda = get_cuda_info()?;
    let tools = collect_cuda_tools();
    let ollama = collect_ollama_cuda_status(&cuda);
    let ai_recommendations = recommend_ai_workloads(&cuda);
    let mut issues = Vec::new();
    let mut fixes = Vec::new();

    if cuda.driver_version == "Unknown" {
        issues.push("NVIDIA driver was not detected via nvidia-smi".to_string());
        fixes.push("Install/repair the NVIDIA driver and verify `nvidia-smi` works".to_string());
    }

    if cuda.devices.is_empty() {
        issues.push("No CUDA-capable NVIDIA devices were detected".to_string());
        fixes.push("Check driver state with `nvctl driver diagnose-release`".to_string());
    }

    if cuda.version == "Unknown" {
        issues.push("CUDA toolkit compiler `nvcc` was not detected".to_string());
        fixes.push("Install CUDA toolkit if you need to build CUDA code; runtime-only AI tools may not need nvcc".to_string());
    }

    if !ollama.issues.is_empty() {
        issues.extend(ollama.issues.iter().cloned());
        fixes.extend(ollama.fixes.iter().cloned());
    }

    Ok(CudaDoctorReport {
        cuda,
        tools,
        ollama,
        ai_recommendations,
        issues,
        fixes,
    })
}

pub fn print_cuda_doctor() -> NvResult<()> {
    let report = collect_cuda_doctor()?;

    println!("CUDA / AI Doctor");
    println!("================");
    println!();
    print_cuda_info_summary(&report.cuda);

    println!();
    println!("Tools:");
    for tool in &report.tools {
        println!(
            "  {}: {}{}",
            tool.name,
            if tool.available { "found" } else { "missing" },
            tool.version
                .as_ref()
                .map(|version| format!(" ({version})"))
                .unwrap_or_default()
        );
    }

    println!();
    print_ollama_status(&report.ollama);

    println!();
    print_ai_recommendations(&report.ai_recommendations);

    if !report.issues.is_empty() {
        println!();
        println!("Issues:");
        for issue in &report.issues {
            println!("  - {issue}");
        }
    }

    if !report.fixes.is_empty() {
        println!();
        println!("Suggested fixes:");
        for fix in &report.fixes {
            println!("  - {fix}");
        }
    }

    Ok(())
}

pub fn print_cuda_info_summary(info: &CudaInfo) {
    println!("CUDA:");
    println!("  Driver version: {}", info.driver_version);
    println!("  Toolkit version: {}", info.version);
    println!(
        "  Toolkit path: {}",
        info.toolkit_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "not found".to_string())
    );
    println!("  Devices: {}", info.devices.len());
    for device in &info.devices {
        println!(
            "  - GPU {}: {} | compute {} | {:.1} GiB total / {:.1} GiB free",
            device.id,
            device.name,
            device.compute_capability,
            bytes_to_gib(device.memory_total),
            bytes_to_gib(device.memory_free)
        );
    }
}

pub fn collect_ollama_cuda_status(cuda: &CudaInfo) -> OllamaCudaStatus {
    let cli = tool_status("ollama", &["--version"]);
    let docker = tool_status("docker", &["--version"]);
    let nvidia_ctk = tool_status("nvidia-ctk", &["--version"]);
    let service_reachable = tcp_port_open("127.0.0.1:11434", Duration::from_millis(150));
    let nvidia_container_runtime_ready = docker.available && nvidia_ctk.available;
    let gpu_memory_total_gb = if cuda.devices.is_empty() {
        0.0
    } else {
        cuda.devices
            .iter()
            .map(|device| bytes_to_gib(device.memory_total))
            .sum()
    };

    let mut issues = Vec::new();
    let mut fixes = Vec::new();

    if !cli.available {
        issues.push("Ollama CLI is not installed or not on PATH".to_string());
        fixes.push(
            "Install Ollama for native local-model serving, or use the official Docker image"
                .to_string(),
        );
    }

    if cli.available && !service_reachable {
        issues.push("Ollama service is not reachable on 127.0.0.1:11434".to_string());
        fixes
            .push("Start Ollama with `ollama serve` or enable its user/system service".to_string());
    }

    if cuda.devices.is_empty() {
        issues.push("No NVIDIA GPU is visible for Ollama CUDA inference".to_string());
        fixes.push(
            "Verify `nvidia-smi`, driver version, and `/dev/nvidia*` device access".to_string(),
        );
    }

    if docker.available && !nvidia_ctk.available {
        issues.push("Docker is available but nvidia-ctk was not found".to_string());
        fixes.push(
            "Install NVIDIA Container Toolkit before running Ollama GPU containers".to_string(),
        );
    }

    OllamaCudaStatus {
        cli,
        service_reachable,
        docker,
        nvidia_ctk,
        nvidia_container_runtime_ready,
        gpu_devices: cuda.devices.len(),
        gpu_memory_total_gb,
        native_env: vec![
            ("CUDA_VISIBLE_DEVICES".to_string(), "0".to_string()),
            ("OLLAMA_HOST".to_string(), "127.0.0.1:11434".to_string()),
            ("OLLAMA_MODELS".to_string(), "~/.ollama/models".to_string()),
        ],
        docker_run_command: "docker run -d --gpus=all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama".to_string(),
        smoke_test_command: "docker exec -it ollama ollama run llama3".to_string(),
        issues,
        fixes,
    }
}

pub fn print_ollama_status(status: &OllamaCudaStatus) {
    println!("Ollama CUDA:");
    println!(
        "  CLI: {}",
        if status.cli.available {
            "found"
        } else {
            "missing"
        }
    );
    if let Some(version) = &status.cli.version {
        println!("  CLI version: {version}");
    }
    println!(
        "  Service 127.0.0.1:11434: {}",
        if status.service_reachable {
            "reachable"
        } else {
            "not reachable"
        }
    );
    println!("  NVIDIA GPUs visible: {}", status.gpu_devices);
    println!(
        "  Total detected VRAM: {:.1} GiB",
        status.gpu_memory_total_gb
    );
    println!(
        "  Docker GPU runtime: {}",
        if status.nvidia_container_runtime_ready {
            "ready to configure/test"
        } else {
            "missing docker or nvidia-ctk"
        }
    );
    println!();
    println!("  Native environment:");
    for (key, value) in &status.native_env {
        println!("    {key}={value}");
    }
    println!();
    println!("  Docker run:");
    println!("    {}", status.docker_run_command);
    println!("  Smoke test:");
    println!("    {}", status.smoke_test_command);
}

pub fn collect_cuda_env() -> NvResult<CudaEnvReport> {
    let info = get_cuda_info()?;
    let ollama = collect_ollama_cuda_status(&info);
    let mut exports = ollama.native_env;

    if let Some(path) = &info.toolkit_path {
        exports.push(("CUDA_HOME".to_string(), path.display().to_string()));
        exports.push(("PATH".to_string(), format!("{}/bin:$PATH", path.display())));
        exports.push((
            "LD_LIBRARY_PATH".to_string(),
            format!("{}/lib64:$LD_LIBRARY_PATH", path.display()),
        ));
    }

    let shell_lines = exports
        .iter()
        .map(|(key, value)| format!("export {key}={}", shell_quote(value)))
        .collect();

    Ok(CudaEnvReport {
        exports,
        shell_lines,
    })
}

pub fn print_cuda_env(report: &CudaEnvReport) {
    println!("CUDA / Ollama environment:");
    for line in &report.shell_lines {
        println!("{line}");
    }
}

pub fn collect_cuda_smoke_plan() -> NvResult<CudaSmokePlan> {
    let info = get_cuda_info()?;
    let ollama = collect_ollama_cuda_status(&info);

    Ok(CudaSmokePlan {
        dry_run: true,
        native_checks: vec![
            "nvidia-smi".to_string(),
            "nvcc --version".to_string(),
            "ollama --version".to_string(),
            "ollama list".to_string(),
            "ollama run llama3".to_string(),
        ],
        container_checks: vec![
            "docker --version".to_string(),
            "nvidia-ctk --version".to_string(),
            "docker run --rm --gpus=all nvidia/cuda:12.9.1-base-ubuntu24.04 nvidia-smi".to_string(),
            ollama.docker_run_command,
            ollama.smoke_test_command,
        ],
        notes: vec![
            "Dry-run only: nvctl does not execute these commands".to_string(),
            format!("Detected CUDA toolkit: {}", info.version),
            format!("Detected NVIDIA GPUs: {}", info.devices.len()),
        ],
    })
}

pub fn print_cuda_smoke_plan(plan: &CudaSmokePlan) {
    println!("CUDA / AI smoke plan (dry-run)");
    println!("==============================");
    println!();
    println!("Native checks:");
    for command in &plan.native_checks {
        println!("  {command}");
    }
    println!();
    println!("Container checks:");
    for command in &plan.container_checks {
        println!("  {command}");
    }
    println!();
    println!("Notes:");
    for note in &plan.notes {
        println!("  - {note}");
    }
}

pub fn recommend_ai_workloads(cuda: &CudaInfo) -> Vec<AiWorkloadRecommendation> {
    let max_vram_gb = cuda
        .devices
        .iter()
        .map(|device| bytes_to_gib(device.memory_total))
        .fold(0.0, f64::max);

    let fit = |required: f64| {
        if max_vram_gb >= required {
            "good fit"
        } else if max_vram_gb > 0.0 {
            "possible with smaller models/quantization"
        } else {
            "not enough GPU data"
        }
        .to_string()
    };

    vec![
        AiWorkloadRecommendation {
            workload: "Ollama 7B/8B quantized LLMs".to_string(),
            fit: fit(8.0),
            notes: vec![
                "Good first CUDA inference smoke test".to_string(),
                "Prefer Q4/Q5 quantized models on 8-12 GiB cards".to_string(),
            ],
        },
        AiWorkloadRecommendation {
            workload: "Ollama 13B/14B quantized LLMs".to_string(),
            fit: fit(14.0),
            notes: vec![
                "Benefits from 16 GiB+ VRAM".to_string(),
                "Context size can push memory use beyond model size".to_string(),
            ],
        },
        AiWorkloadRecommendation {
            workload: "Stable Diffusion / image generation".to_string(),
            fit: fit(12.0),
            notes: vec![
                "12 GiB+ VRAM is comfortable for many SDXL paths".to_string(),
                "Monitor VRAM and power with `nvctl gpu stat`".to_string(),
            ],
        },
        AiWorkloadRecommendation {
            workload: "PyTorch/TensorFlow training".to_string(),
            fit: fit(16.0),
            notes: vec![
                "Install framework wheels matching your CUDA runtime".to_string(),
                "Use containers for reproducible training stacks".to_string(),
            ],
        },
    ]
}

fn shell_quote(value: &str) -> String {
    if value.chars().all(|ch| {
        ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.' | '/' | ':' | '~' | '$')
    }) {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

pub fn print_ai_recommendations(recommendations: &[AiWorkloadRecommendation]) {
    println!("AI/ML workload fit:");
    for recommendation in recommendations {
        println!("  {}: {}", recommendation.workload, recommendation.fit);
        for note in &recommendation.notes {
            println!("    - {note}");
        }
    }
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

fn collect_cuda_tools() -> Vec<ToolStatus> {
    vec![
        tool_status("nvidia-smi", &["--version"]),
        tool_status("nvcc", &["--version"]),
        tool_status("ncu", &["--version"]),
        tool_status("nsys", &["--version"]),
        tool_status("python3", &["--version"]),
        tool_status("docker", &["--version"]),
        tool_status("nvidia-ctk", &["--version"]),
        tool_status("ollama", &["--version"]),
    ]
}

fn tool_status(name: &str, version_args: &[&str]) -> ToolStatus {
    let path = find_executable(name).map(|path| path.display().to_string());
    let version = Command::new(name)
        .args(version_args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            stdout
                .lines()
                .chain(stderr.lines())
                .find(|line| !line.trim().is_empty())
                .unwrap_or("")
                .trim()
                .to_string()
        })
        .filter(|version| !version.is_empty());

    ToolStatus {
        name: name.to_string(),
        available: path.is_some(),
        path,
        version,
    }
}

fn find_executable(name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|path| path.join(name))
            .find(|candidate| candidate.is_file())
    })
}

fn tcp_port_open(addr: &str, timeout: Duration) -> bool {
    addr.parse()
        .ok()
        .and_then(|socket_addr| std::net::TcpStream::connect_timeout(&socket_addr, timeout).ok())
        .is_some()
}

fn bytes_to_gib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
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

    #[test]
    fn test_ai_recommendations_do_not_require_gpu() {
        let info = CudaInfo {
            version: "Unknown".to_string(),
            driver_version: "Unknown".to_string(),
            runtime_version: None,
            devices: Vec::new(),
            toolkit_path: None,
        };

        let recommendations = recommend_ai_workloads(&info);
        assert!(
            recommendations
                .iter()
                .any(|item| item.workload.contains("Ollama"))
        );
    }

    #[test]
    fn test_ollama_status_contains_docker_gpu_command() {
        let info = CudaInfo {
            version: "Unknown".to_string(),
            driver_version: "Unknown".to_string(),
            runtime_version: None,
            devices: Vec::new(),
            toolkit_path: None,
        };

        let status = collect_ollama_cuda_status(&info);
        assert!(status.docker_run_command.contains("--gpus=all"));
        assert!(status.docker_run_command.contains("ollama/ollama"));
    }
}
