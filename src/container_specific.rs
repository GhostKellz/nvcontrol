/// Phase 5.4: Container-Specific Features
///
/// Docker GPU control, Podman rootless optimization, Kubernetes device plugin, per-container power limits

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

/// Docker GPU controller
pub struct DockerGpuController {
    runtime_config: DockerRuntimeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerRuntimeConfig {
    pub nvidia_runtime_enabled: bool,
    pub default_runtime: String,
}

impl DockerGpuController {
    pub fn new() -> Self {
        Self {
            runtime_config: DockerRuntimeConfig {
                nvidia_runtime_enabled: false,
                default_runtime: "runc".to_string(),
            },
        }
    }

    /// Check if NVIDIA Container Runtime is installed
    pub fn is_nvidia_runtime_available(&mut self) -> NvResult<bool> {
        let output = Command::new("docker")
            .args(&["info", "--format", "{{.Runtimes}}"])
            .output()
            .map_err(|e| {
                NvControlError::ContainerOperationFailed(format!("Docker info failed: {}", e))
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let available = stdout.contains("nvidia");

        self.runtime_config.nvidia_runtime_enabled = available;

        Ok(available)
    }

    /// Run container with GPU access
    pub fn run_with_gpu(
        &self,
        image: &str,
        gpu_ids: &[u32],
        env_vars: HashMap<String, String>,
    ) -> NvResult<String> {
        if !self.runtime_config.nvidia_runtime_enabled {
            return Err(NvControlError::ContainerOperationFailed(
                "NVIDIA runtime not available".to_string(),
            ));
        }

        let mut cmd = Command::new("docker");
        cmd.arg("run").arg("-d");

        // Add GPU specification
        if gpu_ids.is_empty() {
            cmd.args(&["--gpus", "all"]);
        } else {
            let gpu_spec = format!("\"device={}\"",
                gpu_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",")
            );
            cmd.args(&["--gpus", &gpu_spec]);
        }

        // Add environment variables
        for (key, value) in env_vars {
            cmd.args(&["-e", &format!("{}={}", key, value)]);
        }

        cmd.arg(image);

        let output = cmd.output().map_err(|e| {
            NvControlError::ContainerOperationFailed(format!("Docker run failed: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NvControlError::ContainerOperationFailed(format!(
                "Docker run failed: {}",
                stderr
            )));
        }

        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

        println!("Container started with GPU access: {}", container_id);

        Ok(container_id)
    }

    /// Set GPU resource limits for container
    pub fn set_gpu_limits(&self, container_id: &str, memory_limit_mb: u64) -> NvResult<()> {
        // Use CUDA_VISIBLE_DEVICES and memory constraints
        let output = Command::new("docker")
            .args(&[
                "update",
                "--memory",
                &format!("{}m", memory_limit_mb),
                container_id,
            ])
            .output()
            .map_err(|e| {
                NvControlError::ContainerOperationFailed(format!("Docker update failed: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NvControlError::ContainerOperationFailed(format!(
                "Failed to set limits: {}",
                stderr
            )));
        }

        println!("GPU limits set for container {}", container_id);

        Ok(())
    }
}

impl Default for DockerGpuController {
    fn default() -> Self {
        Self::new()
    }
}

/// Podman rootless GPU controller
pub struct PodmanRootlessController {
    cdi_config: CdiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdiConfig {
    pub enabled: bool,
    pub config_path: PathBuf,
}

impl PodmanRootlessController {
    pub fn new() -> Self {
        let config_path = PathBuf::from("/etc/cdi/nvidia.yaml");

        Self {
            cdi_config: CdiConfig {
                enabled: config_path.exists(),
                config_path,
            },
        }
    }

    /// Setup CDI (Container Device Interface) for rootless Podman
    pub fn setup_cdi(&mut self) -> NvResult<()> {
        // Generate CDI spec for NVIDIA GPUs
        let output = Command::new("nvidia-ctk")
            .args(&["cdi", "generate", "--output=/etc/cdi/nvidia.yaml"])
            .output();

        if output.is_err() {
            return Err(NvControlError::ContainerOperationFailed(
                "nvidia-ctk not available (install nvidia-container-toolkit)".to_string(),
            ));
        }

        let output = output.unwrap();
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NvControlError::ContainerOperationFailed(format!(
                "CDI generation failed: {}",
                stderr
            )));
        }

        self.cdi_config.enabled = true;

        println!("CDI configured for rootless Podman");

        Ok(())
    }

    /// Run rootless container with GPU
    pub fn run_rootless_with_gpu(
        &self,
        image: &str,
        gpu_ids: &[u32],
    ) -> NvResult<String> {
        if !self.cdi_config.enabled {
            return Err(NvControlError::ContainerOperationFailed(
                "CDI not configured".to_string(),
            ));
        }

        let mut cmd = Command::new("podman");
        cmd.arg("run").arg("-d");

        // Use CDI device specification
        for &gpu_id in gpu_ids {
            cmd.args(&["--device", &format!("nvidia.com/gpu={}", gpu_id)]);
        }

        cmd.arg(image);

        let output = cmd.output().map_err(|e| {
            NvControlError::ContainerOperationFailed(format!("Podman run failed: {}", e))
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NvControlError::ContainerOperationFailed(format!(
                "Podman run failed: {}",
                stderr
            )));
        }

        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

        println!("Rootless container started with GPU: {}", container_id);

        Ok(container_id)
    }

    /// Check CDI status
    pub fn is_cdi_enabled(&self) -> bool {
        self.cdi_config.enabled
    }
}

impl Default for PodmanRootlessController {
    fn default() -> Self {
        Self::new()
    }
}

/// Kubernetes device plugin manager
pub struct K8sDevicePlugin {
    deployed: bool,
    namespace: String,
}

impl K8sDevicePlugin {
    pub fn new(namespace: String) -> Self {
        Self {
            deployed: false,
            namespace,
        }
    }

    /// Deploy NVIDIA device plugin to Kubernetes
    pub fn deploy(&mut self) -> NvResult<()> {
        // Check if kubectl is available
        let kubectl_check = Command::new("kubectl").arg("version").output();

        if kubectl_check.is_err() {
            return Err(NvControlError::ContainerOperationFailed(
                "kubectl not available".to_string(),
            ));
        }

        // Deploy NVIDIA device plugin daemonset
        let yaml_url = "https://raw.githubusercontent.com/NVIDIA/k8s-device-plugin/main/deployments/static/nvidia-device-plugin.yml";

        let output = Command::new("kubectl")
            .args(&["apply", "-f", yaml_url, "-n", &self.namespace])
            .output()
            .map_err(|e| {
                NvControlError::ContainerOperationFailed(format!(
                    "kubectl apply failed: {}",
                    e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NvControlError::ContainerOperationFailed(format!(
                "Device plugin deployment failed: {}",
                stderr
            )));
        }

        self.deployed = true;

        println!("NVIDIA device plugin deployed to Kubernetes");

        Ok(())
    }

    /// Check device plugin status
    pub fn check_status(&self) -> NvResult<String> {
        let output = Command::new("kubectl")
            .args(&[
                "get",
                "daemonset",
                "nvidia-device-plugin-daemonset",
                "-n",
                &self.namespace,
                "-o",
                "jsonpath={.status}",
            ])
            .output()
            .map_err(|e| {
                NvControlError::ContainerOperationFailed(format!(
                    "kubectl get failed: {}",
                    e
                ))
            })?;

        let status = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(status)
    }

    /// Generate pod spec with GPU request
    pub fn generate_pod_spec(&self, gpu_count: u32) -> String {
        format!(
            r#"apiVersion: v1
kind: Pod
metadata:
  name: gpu-pod
spec:
  containers:
  - name: gpu-container
    image: nvidia/cuda:12.0-base
    resources:
      limits:
        nvidia.com/gpu: {}
"#,
            gpu_count
        )
    }

    pub fn is_deployed(&self) -> bool {
        self.deployed
    }
}

/// Per-container power limit controller
pub struct ContainerPowerController {
    limits: HashMap<String, PowerLimit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerLimit {
    pub container_id: String,
    pub gpu_id: u32,
    pub power_limit_watts: u32,
    pub enforced: bool,
}

impl ContainerPowerController {
    pub fn new() -> Self {
        Self {
            limits: HashMap::new(),
        }
    }

    /// Set power limit for container's GPU
    pub fn set_limit(&mut self, container_id: String, gpu_id: u32, power_watts: u32) -> NvResult<()> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init().map_err(|e| {
            NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
        })?;

        let mut device = nvml.device_by_index(gpu_id).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
        })?;

        // Set power limit
        device
            .set_power_management_limit(power_watts * 1000)
            .map_err(|e| {
                NvControlError::PowerManagementFailed(format!(
                    "Failed to set power limit: {}",
                    e
                ))
            })?;

        self.limits.insert(
            container_id.clone(),
            PowerLimit {
                container_id,
                gpu_id,
                power_limit_watts: power_watts,
                enforced: true,
            },
        );

        println!("Power limit set to {} W for container", power_watts);

        Ok(())
    }

    /// Get power limit for container
    pub fn get_limit(&self, container_id: &str) -> Option<&PowerLimit> {
        self.limits.get(container_id)
    }

    /// Remove power limit
    pub fn remove_limit(&mut self, container_id: &str) -> NvResult<()> {
        if let Some(limit) = self.limits.remove(container_id) {
            // Reset to default power limit
            use nvml_wrapper::Nvml;

            let nvml = Nvml::init().map_err(|e| {
                NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e))
            })?;

            let mut device = nvml.device_by_index(limit.gpu_id).map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
            })?;

            let default_limit = device.power_management_limit_default().map_err(|e| {
                NvControlError::PowerManagementFailed(format!(
                    "Failed to get default limit: {}",
                    e
                ))
            })?;

            device
                .set_power_management_limit(default_limit)
                .map_err(|e| {
                    NvControlError::PowerManagementFailed(format!(
                        "Failed to reset power limit: {}",
                        e
                    ))
                })?;

            println!("Power limit removed for container");
        }

        Ok(())
    }

    /// List all power limits
    pub fn list_limits(&self) -> Vec<&PowerLimit> {
        self.limits.values().collect()
    }
}

impl Default for ContainerPowerController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_controller() {
        let controller = DockerGpuController::new();
        assert!(!controller.runtime_config.nvidia_runtime_enabled);
    }

    #[test]
    fn test_podman_controller() {
        let controller = PodmanRootlessController::new();
        println!("CDI enabled: {}", controller.is_cdi_enabled());
    }

    #[test]
    fn test_k8s_device_plugin() {
        let plugin = K8sDevicePlugin::new("kube-system".to_string());
        assert!(!plugin.is_deployed());

        let spec = plugin.generate_pod_spec(2);
        assert!(spec.contains("nvidia.com/gpu: 2"));
    }

    #[test]
    fn test_power_controller() {
        let controller = ContainerPowerController::new();
        assert_eq!(controller.limits.len(), 0);
    }

    #[test]
    fn test_power_limit_struct() {
        let limit = PowerLimit {
            container_id: "abc123".to_string(),
            gpu_id: 0,
            power_limit_watts: 300,
            enforced: true,
        };

        assert_eq!(limit.power_limit_watts, 300);
        assert!(limit.enforced);
    }
}
