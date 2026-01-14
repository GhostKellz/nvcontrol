use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerGpuInfo {
    pub container_id: String,
    pub container_name: String,
    pub image: String,
    pub gpu_devices: Vec<String>,
    pub gpu_memory_limit: Option<u64>,
    pub gpu_utilization: f32,
    pub power_usage: f32,
    pub status: ContainerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerStatus {
    Running,
    Paused,
    Stopped,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerGpuProfile {
    pub name: String,
    pub description: String,
    pub power_limit: Option<u32>,
    pub memory_limit: Option<u64>,
    pub compute_mode: ComputeMode,
    pub persistence_mode: bool,
    pub auto_boost: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComputeMode {
    Default,
    Exclusive,
    Prohibited,
    ExclusiveProcess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesGpuResource {
    pub namespace: String,
    pub pod_name: String,
    pub container_name: String,
    pub gpu_request: Option<String>,
    pub gpu_limit: Option<String>,
    pub node_name: String,
    pub gpu_utilization: f32,
}

/// Check if NVIDIA Container Runtime is available
pub fn is_nvidia_runtime_available() -> bool {
    // Check for nvidia-container-runtime
    Command::new("nvidia-container-runtime")
        .arg("--version")
        .output()
        .is_ok()
}

/// Check if Docker with NVIDIA runtime is configured
pub fn is_docker_nvidia_configured() -> NvResult<bool> {
    let output = Command::new("docker")
        .args(&["info", "--format", "{{.Runtimes}}"])
        .output()
        .map_err(|e| NvControlError::CommandFailed(format!("Docker check failed: {}", e)))?;

    let runtime_info = String::from_utf8_lossy(&output.stdout);
    Ok(runtime_info.contains("nvidia"))
}

/// List all containers using GPU resources
pub fn list_gpu_containers() -> NvResult<Vec<ContainerGpuInfo>> {
    let mut containers = Vec::new();

    // Get all running containers
    let output = Command::new("docker")
        .args(&[
            "ps",
            "--format",
            "{{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}",
        ])
        .output()
        .map_err(|e| NvControlError::CommandFailed(format!("Docker ps failed: {}", e)))?;

    let containers_list = String::from_utf8_lossy(&output.stdout);

    for line in containers_list.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            let container_id = parts[0].to_string();
            let container_name = parts[1].to_string();
            let image = parts[2].to_string();

            // Check if container uses GPU
            if let Ok(gpu_info) = get_container_gpu_info(&container_id) {
                containers.push(ContainerGpuInfo {
                    container_id,
                    container_name,
                    image,
                    gpu_devices: gpu_info.0,
                    gpu_memory_limit: gpu_info.1,
                    gpu_utilization: gpu_info.2,
                    power_usage: gpu_info.3,
                    status: ContainerStatus::Running,
                });
            }
        }
    }

    Ok(containers)
}

/// Get GPU information for a specific container
fn get_container_gpu_info(container_id: &str) -> NvResult<(Vec<String>, Option<u64>, f32, f32)> {
    // Check container environment for GPU devices
    let output = Command::new("docker")
        .args(&["inspect", container_id, "--format", "{{.Config.Env}}"])
        .output()
        .map_err(|e| NvControlError::CommandFailed(format!("Container inspect failed: {}", e)))?;

    let env_vars = String::from_utf8_lossy(&output.stdout);
    let mut gpu_devices = Vec::new();
    let memory_limit = None;

    // Parse NVIDIA_VISIBLE_DEVICES
    if env_vars.contains("NVIDIA_VISIBLE_DEVICES") {
        // Extract GPU device IDs
        for env_var in env_vars.split_whitespace() {
            if env_var.starts_with("NVIDIA_VISIBLE_DEVICES=") {
                let devices = env_var.split('=').nth(1).unwrap_or("");
                gpu_devices = devices.split(',').map(|s| s.to_string()).collect();
                break;
            }
        }
    }

    // Get runtime GPU stats (simplified - would need nvidia-ml-py or similar in container)
    let gpu_utilization = get_container_gpu_utilization(container_id).unwrap_or(0.0);
    let power_usage = get_container_power_usage(container_id).unwrap_or(0.0);

    Ok((gpu_devices, memory_limit, gpu_utilization, power_usage))
}

/// Get GPU utilization for a container (simplified implementation)
fn get_container_gpu_utilization(_container_id: &str) -> NvResult<f32> {
    // In a real implementation, this would use nvidia-ml-py in the container
    // or parse nvidia-smi output with container process mapping
    let output = Command::new("nvidia-smi")
        .args(&[
            "--query-compute-apps=pid,used_memory",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .map_err(|e| NvControlError::CommandFailed(format!("nvidia-smi failed: {}", e)))?;

    let _processes = String::from_utf8_lossy(&output.stdout);

    // Map container processes to GPU usage (simplified)
    // This would need proper process mapping in production
    Ok(0.0) // Placeholder
}

/// Get power usage for a container
fn get_container_power_usage(_container_id: &str) -> NvResult<f32> {
    // Similar to utilization - would need proper container->GPU mapping
    Ok(0.0) // Placeholder
}

/// Apply GPU profile to container
pub fn apply_container_gpu_profile(
    container_id: &str,
    profile: &ContainerGpuProfile,
) -> NvResult<()> {
    println!(
        "Applying GPU profile '{}' to container {}",
        profile.name, container_id
    );

    // Set power limit if specified
    if let Some(power_limit) = profile.power_limit {
        set_container_power_limit(container_id, power_limit)?;
    }

    // Set compute mode
    set_container_compute_mode(container_id, &profile.compute_mode)?;

    // Set persistence mode
    set_container_persistence_mode(container_id, profile.persistence_mode)?;

    Ok(())
}

/// Set power limit for container's GPU usage
fn set_container_power_limit(container_id: &str, power_limit: u32) -> NvResult<()> {
    // This would typically involve:
    // 1. Identifying which GPU(s) the container uses
    // 2. Setting cgroup limits or using nvidia-ml-py
    // 3. Applying runtime constraints

    println!(
        "Setting power limit {} W for container {}",
        power_limit, container_id
    );
    Ok(())
}

/// Set compute mode for container
fn set_container_compute_mode(container_id: &str, mode: &ComputeMode) -> NvResult<()> {
    let mode_str = match mode {
        ComputeMode::Default => "0",
        ComputeMode::Exclusive => "1",
        ComputeMode::Prohibited => "2",
        ComputeMode::ExclusiveProcess => "3",
    };

    println!(
        "Setting compute mode {} for container {}",
        mode_str, container_id
    );
    Ok(())
}

/// Set persistence mode for container
fn set_container_persistence_mode(container_id: &str, enabled: bool) -> NvResult<()> {
    println!(
        "Setting persistence mode {} for container {}",
        enabled, container_id
    );
    Ok(())
}

/// List Kubernetes GPU resources
pub fn list_kubernetes_gpu_resources() -> NvResult<Vec<KubernetesGpuResource>> {
    let mut resources = Vec::new();

    // Check if kubectl is available
    let output = Command::new("kubectl")
        .args(["get", "pods", "--all-namespaces", "-o", "json"])
        .output()
        .map_err(|e| NvControlError::CommandFailed(format!("kubectl failed: {}", e)))?;

    if !output.status.success() {
        return Ok(resources); // kubectl not configured or no access
    }

    let pods_json = String::from_utf8_lossy(&output.stdout);

    // Parse JSON using serde_json
    if let Ok(pods_data) = serde_json::from_str::<serde_json::Value>(&pods_json) {
        if let Some(items) = pods_data.get("items").and_then(|i| i.as_array()) {
            for pod in items {
                let namespace = pod
                    .pointer("/metadata/namespace")
                    .and_then(|n| n.as_str())
                    .unwrap_or("default")
                    .to_string();

                let pod_name = pod
                    .pointer("/metadata/name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let node_name = pod
                    .pointer("/spec/nodeName")
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Check each container for GPU resources
                if let Some(containers) = pod.pointer("/spec/containers").and_then(|c| c.as_array()) {
                    for container in containers {
                        let container_name = container
                            .get("name")
                            .and_then(|n| n.as_str())
                            .unwrap_or("unknown")
                            .to_string();

                        // Check for nvidia.com/gpu in resources
                        let gpu_request = container
                            .pointer("/resources/requests/nvidia.com~1gpu")
                            .and_then(|g| g.as_str())
                            .map(|s| s.to_string());

                        let gpu_limit = container
                            .pointer("/resources/limits/nvidia.com~1gpu")
                            .and_then(|g| g.as_str())
                            .map(|s| s.to_string());

                        // Only add containers that request GPUs
                        if gpu_request.is_some() || gpu_limit.is_some() {
                            resources.push(KubernetesGpuResource {
                                namespace: namespace.clone(),
                                pod_name: pod_name.clone(),
                                container_name,
                                gpu_request,
                                gpu_limit,
                                node_name: node_name.clone(),
                                gpu_utilization: 0.0, // Would need nvidia-smi in pod to get actual utilization
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(resources)
}

/// Check if NVIDIA device plugin is running in the cluster
pub fn check_nvidia_device_plugin() -> NvResult<bool> {
    let output = Command::new("kubectl")
        .args(["get", "pods", "-n", "kube-system", "-l", "name=nvidia-device-plugin-ds", "-o", "name"])
        .output()
        .map_err(|e| NvControlError::CommandFailed(format!("kubectl failed: {}", e)))?;

    if !output.status.success() {
        return Ok(false);
    }

    let result = String::from_utf8_lossy(&output.stdout);
    Ok(!result.trim().is_empty())
}

/// Get GPU node capacity information
pub fn get_cluster_gpu_capacity() -> NvResult<HashMap<String, u32>> {
    let mut capacity = HashMap::new();

    let output = Command::new("kubectl")
        .args(["get", "nodes", "-o", "json"])
        .output()
        .map_err(|e| NvControlError::CommandFailed(format!("kubectl failed: {}", e)))?;

    if !output.status.success() {
        return Ok(capacity);
    }

    let nodes_json = String::from_utf8_lossy(&output.stdout);

    if let Ok(nodes_data) = serde_json::from_str::<serde_json::Value>(&nodes_json) {
        if let Some(items) = nodes_data.get("items").and_then(|i| i.as_array()) {
            for node in items {
                let node_name = node
                    .pointer("/metadata/name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Check allocatable GPU resources
                if let Some(gpu_count) = node
                    .pointer("/status/allocatable/nvidia.com~1gpu")
                    .and_then(|g| g.as_str())
                    .and_then(|s| s.parse::<u32>().ok())
                {
                    if gpu_count > 0 {
                        capacity.insert(node_name, gpu_count);
                    }
                }
            }
        }
    }

    Ok(capacity)
}

/// Monitor container GPU usage
pub fn monitor_container_gpu_usage() -> NvResult<HashMap<String, f32>> {
    let containers = list_gpu_containers()?;
    let mut usage_map = HashMap::new();

    for container in containers {
        usage_map.insert(container.container_id, container.gpu_utilization);
    }

    Ok(usage_map)
}

/// Create container-optimized GPU profile
pub fn create_container_profile(name: &str, workload_type: &str) -> ContainerGpuProfile {
    match workload_type {
        "ml-training" => ContainerGpuProfile {
            name: name.to_string(),
            description: "Optimized for ML training workloads".to_string(),
            power_limit: Some(300),
            memory_limit: None,
            compute_mode: ComputeMode::ExclusiveProcess,
            persistence_mode: true,
            auto_boost: true,
        },
        "inference" => ContainerGpuProfile {
            name: name.to_string(),
            description: "Optimized for ML inference".to_string(),
            power_limit: Some(200),
            memory_limit: Some(8 * 1024 * 1024 * 1024), // 8GB
            compute_mode: ComputeMode::Default,
            persistence_mode: true,
            auto_boost: false,
        },
        "gaming" => ContainerGpuProfile {
            name: name.to_string(),
            description: "Optimized for gaming containers".to_string(),
            power_limit: Some(250),
            memory_limit: None,
            compute_mode: ComputeMode::Default,
            persistence_mode: false,
            auto_boost: true,
        },
        _ => ContainerGpuProfile {
            name: name.to_string(),
            description: "Default container profile".to_string(),
            power_limit: Some(225),
            memory_limit: None,
            compute_mode: ComputeMode::Default,
            persistence_mode: false,
            auto_boost: false,
        },
    }
}

/// Save container profiles to disk
pub fn save_container_profiles(profiles: &[ContainerGpuProfile]) -> NvResult<()> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("Could not find config directory".to_string()))?
        .join("nvcontrol");

    fs::create_dir_all(&config_dir)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to create config dir: {}", e)))?;

    let profiles_file = config_dir.join("container_profiles.json");
    let json = serde_json::to_string_pretty(profiles)
        .map_err(|e| NvControlError::ConfigError(format!("JSON serialization failed: {}", e)))?;

    fs::write(profiles_file, json)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to write profiles: {}", e)))?;

    Ok(())
}

/// Load container profiles from disk
pub fn load_container_profiles() -> NvResult<Vec<ContainerGpuProfile>> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| NvControlError::ConfigError("Could not find config directory".to_string()))?
        .join("nvcontrol");

    let profiles_file = config_dir.join("container_profiles.json");

    if !profiles_file.exists() {
        // Return default profiles
        return Ok(vec![
            create_container_profile("ML Training", "ml-training"),
            create_container_profile("ML Inference", "inference"),
            create_container_profile("Gaming", "gaming"),
            create_container_profile("Default", "default"),
        ]);
    }

    let json = fs::read_to_string(profiles_file)
        .map_err(|e| NvControlError::ConfigError(format!("Failed to read profiles: {}", e)))?;

    let profiles: Vec<ContainerGpuProfile> = serde_json::from_str(&json)
        .map_err(|e| NvControlError::ConfigError(format!("JSON parsing failed: {}", e)))?;

    Ok(profiles)
}

/// Get container runtime information
pub fn get_container_runtime_info() -> NvResult<HashMap<String, String>> {
    let mut info = HashMap::new();

    // Check Docker version
    if let Ok(output) = Command::new("docker").args(&["--version"]).output() {
        info.insert(
            "docker_version".to_string(),
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        );
    }

    // Check NVIDIA Container Runtime
    if let Ok(output) = Command::new("nvidia-container-runtime")
        .args(&["--version"])
        .output()
    {
        info.insert(
            "nvidia_runtime_version".to_string(),
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        );
    }

    // Check if nvidia runtime is configured in Docker
    info.insert(
        "nvidia_runtime_configured".to_string(),
        is_docker_nvidia_configured()?.to_string(),
    );

    // Check containerd (if available)
    if let Ok(output) = Command::new("containerd").args(&["--version"]).output() {
        info.insert(
            "containerd_version".to_string(),
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        );
    }

    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_profile_creation() {
        let profile = create_container_profile("Test", "ml-training");
        assert_eq!(profile.name, "Test");
        assert_eq!(
            profile.compute_mode as u8,
            ComputeMode::ExclusiveProcess as u8
        );
        assert!(profile.persistence_mode);
    }

    #[test]
    fn test_runtime_detection() {
        // This will depend on the environment
        let available = is_nvidia_runtime_available();
        // Just ensure it doesn't panic - result depends on system
        let _ = available;
    }
}
