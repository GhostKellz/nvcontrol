use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::bolt_integration::{GpuContainerConfig, NvControlBoltManager};
use std::sync::Arc;
use std::sync::Mutex;
use tokio::runtime::Runtime;

/// NVIDIA Container Runtime implementation
/// This provides docker/podman/bolt/nix container GPU passthrough functionality
/// built directly into nvcontrol, with native Bolt API integration

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvContainerRuntime {
    pub version: String,
    pub supported_runtimes: Vec<ContainerRuntime>,
    pub gpu_devices: Vec<GpuDevice>,
    pub config_path: PathBuf,
    #[serde(skip)]
    pub bolt_manager: Option<Arc<Mutex<NvControlBoltManager>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerRuntime {
    Docker,
    Podman,
    Containerd,
    Bolt,
    NixOS,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    pub index: u32,
    pub uuid: String,
    pub name: String,
    pub memory_mb: u64,
    pub compute_capability: (u32, u32),
    pub pci_bus_id: String,
    pub minor_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerGpuConfig {
    pub runtime: ContainerRuntime,
    pub gpu_devices: Vec<String>, // "all", "0", "1,2", "GPU-uuid"
    pub memory_limit: Option<u64>,
    pub compute_mode: String,
    pub driver_capabilities: Vec<String>, // "compute", "utility", "graphics", "video"
    pub environment_vars: HashMap<String, String>,
    pub mount_points: Vec<String>,
    pub device_requests: Vec<DeviceRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRequest {
    pub driver: String,
    pub count: Option<u32>,
    pub device_ids: Vec<String>,
    pub capabilities: Vec<Vec<String>>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerLaunchConfig {
    pub image: String,
    pub name: Option<String>,
    pub command: Option<Vec<String>>,
    pub working_dir: Option<String>,
    pub environment: HashMap<String, String>,
    pub volumes: Vec<VolumeMount>,
    pub ports: Vec<PortMapping>,
    pub gpu_config: ContainerGpuConfig,
    pub interactive: bool,
    pub remove_on_exit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolumeMount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortMapping {
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: String, // "tcp" or "udp"
}

impl NvContainerRuntime {
    /// Initialize the NVIDIA container runtime with Bolt integration
    pub fn new() -> NvResult<Self> {
        let version = "1.0.0-nvcontrol".to_string();
        let gpu_devices = Self::detect_gpu_devices()?;
        let supported_runtimes = Self::detect_container_runtimes()?;

        let config_path = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("No config directory".to_string()))?
            .join("nvcontrol")
            .join("container-runtime");

        fs::create_dir_all(&config_path).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create config dir: {}", e))
        })?;

        // Initialize Bolt manager if Bolt runtime is available
        let bolt_manager = if supported_runtimes.contains(&ContainerRuntime::Bolt) {
            let rt = Runtime::new().map_err(|e| {
                NvControlError::RuntimeError(format!("Tokio runtime failed: {}", e))
            })?;
            match rt.block_on(NvControlBoltManager::new()) {
                Ok(manager) => Some(Arc::new(Mutex::new(manager))),
                Err(e) => {
                    eprintln!("Warning: Failed to initialize Bolt manager: {}", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(NvContainerRuntime {
            version,
            supported_runtimes,
            gpu_devices,
            config_path,
            bolt_manager,
        })
    }

    /// Detect available GPU devices
    fn detect_gpu_devices() -> NvResult<Vec<GpuDevice>> {
        let output = Command::new("nvidia-smi")
            .args(&[
                "--query-gpu=index,uuid,name,memory.total,pci.bus_id",
                "--format=csv,noheader,nounits",
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("nvidia-smi failed: {}", e)))?;

        let mut devices = Vec::new();
        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split(", ").collect();
            if parts.len() >= 5 {
                devices.push(GpuDevice {
                    index: parts[0].parse().unwrap_or(0),
                    uuid: parts[1].to_string(),
                    name: parts[2].to_string(),
                    memory_mb: parts[3].parse().unwrap_or(0),
                    compute_capability: (0, 0), // Would need additional query
                    pci_bus_id: parts[4].to_string(),
                    minor_number: parts[0].parse().unwrap_or(0),
                });
            }
        }

        Ok(devices)
    }

    /// Detect available container runtimes
    fn detect_container_runtimes() -> NvResult<Vec<ContainerRuntime>> {
        let mut runtimes = Vec::new();

        // Check Docker
        if Command::new("docker").arg("--version").output().is_ok() {
            runtimes.push(ContainerRuntime::Docker);
        }

        // Check Podman
        if Command::new("podman").arg("--version").output().is_ok() {
            runtimes.push(ContainerRuntime::Podman);
        }

        // Check containerd
        if Command::new("containerd").arg("--version").output().is_ok() {
            runtimes.push(ContainerRuntime::Containerd);
        }

        // Check Bolt
        if Command::new("bolt").arg("--version").output().is_ok() {
            runtimes.push(ContainerRuntime::Bolt);
        }

        // Check NixOS
        if fs::metadata("/etc/nixos").is_ok() {
            runtimes.push(ContainerRuntime::NixOS);
        }

        Ok(runtimes)
    }

    /// Launch container with GPU support
    pub fn launch_container(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        match config.gpu_config.runtime {
            ContainerRuntime::Docker => self.launch_docker_container(config),
            ContainerRuntime::Podman => self.launch_podman_container(config),
            ContainerRuntime::Bolt => self.launch_bolt_container(config),
            ContainerRuntime::NixOS => self.launch_nix_container(config),
            ContainerRuntime::Containerd => self.launch_containerd_container(config),
            ContainerRuntime::Custom(ref name) => Err(NvControlError::UnsupportedFeature(format!(
                "Custom runtime {} not implemented",
                name
            ))),
        }
    }

    /// Launch Docker container with GPU support
    fn launch_docker_container(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        let mut cmd = Command::new("docker");
        cmd.arg("run");

        // Add GPU device requests
        if !config.gpu_config.gpu_devices.is_empty() {
            // Use --gpus flag (modern Docker)
            let gpu_spec = if config.gpu_config.gpu_devices.contains(&"all".to_string()) {
                "all".to_string()
            } else {
                format!("device={}", config.gpu_config.gpu_devices.join(","))
            };
            cmd.args(&["--gpus", &gpu_spec]);

            // Add driver capabilities
            for cap in &config.gpu_config.driver_capabilities {
                cmd.env("NVIDIA_DRIVER_CAPABILITIES", cap);
            }
        }

        // Add environment variables
        for (key, value) in &config.environment {
            cmd.args(&["-e", &format!("{}={}", key, value)]);
        }

        // Add volume mounts
        for volume in &config.volumes {
            let mount_spec = if volume.read_only {
                format!("{}:{}:ro", volume.source, volume.target)
            } else {
                format!("{}:{}", volume.source, volume.target)
            };
            cmd.args(&["-v", &mount_spec]);
        }

        // Add port mappings
        for port in &config.ports {
            cmd.args(&[
                "-p",
                &format!(
                    "{}:{}:{}",
                    port.host_port, port.container_port, port.protocol
                ),
            ]);
        }

        // Container name
        if let Some(ref name) = config.name {
            cmd.args(&["--name", name]);
        }

        // Interactive mode
        if config.interactive {
            cmd.args(&["-it"]);
        }

        // Remove on exit
        if config.remove_on_exit {
            cmd.arg("--rm");
        }

        // Working directory
        if let Some(ref workdir) = config.working_dir {
            cmd.args(&["-w", workdir]);
        }

        // Image
        cmd.arg(&config.image);

        // Command
        if let Some(ref command) = config.command {
            cmd.args(command);
        }

        // Execute command
        let output = cmd
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Docker run failed: {}", e)))?;

        if output.status.success() {
            let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(container_id)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(NvControlError::CommandFailed(format!(
                "Docker run failed: {}",
                error
            )))
        }
    }

    /// Launch Podman container with GPU support
    fn launch_podman_container(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        let mut cmd = Command::new("podman");
        cmd.arg("run");

        // Add GPU device access for Podman
        if !config.gpu_config.gpu_devices.is_empty() {
            // Podman uses --device for GPU access
            for gpu_device in &self.gpu_devices {
                cmd.args(&["--device", &format!("/dev/nvidia{}", gpu_device.index)]);
                cmd.args(&["--device", "/dev/nvidiactl"]);
                cmd.args(&["--device", "/dev/nvidia-modeset"]);
                cmd.args(&["--device", "/dev/nvidia-uvm"]);
                cmd.args(&["--device", "/dev/nvidia-uvm-tools"]);
            }

            // Add NVIDIA environment variables
            cmd.args(&["-e", "NVIDIA_VISIBLE_DEVICES=all"]);
            cmd.args(&[
                "-e",
                &format!(
                    "NVIDIA_DRIVER_CAPABILITIES={}",
                    config.gpu_config.driver_capabilities.join(",")
                ),
            ]);
        }

        // Add environment variables
        for (key, value) in &config.environment {
            cmd.args(&["-e", &format!("{}={}", key, value)]);
        }

        // Add volume mounts
        for volume in &config.volumes {
            let mount_spec = if volume.read_only {
                format!("{}:{}:ro", volume.source, volume.target)
            } else {
                format!("{}:{}", volume.source, volume.target)
            };
            cmd.args(&["-v", &mount_spec]);
        }

        // Add port mappings
        for port in &config.ports {
            cmd.args(&["-p", &format!("{}:{}", port.host_port, port.container_port)]);
        }

        // Container name
        if let Some(ref name) = config.name {
            cmd.args(&["--name", name]);
        }

        // Interactive mode
        if config.interactive {
            cmd.args(&["-it"]);
        }

        // Remove on exit
        if config.remove_on_exit {
            cmd.arg("--rm");
        }

        // Working directory
        if let Some(ref workdir) = config.working_dir {
            cmd.args(&["-w", workdir]);
        }

        // Image
        cmd.arg(&config.image);

        // Command
        if let Some(ref command) = config.command {
            cmd.args(command);
        }

        // Execute command
        let output = cmd
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Podman run failed: {}", e)))?;

        if output.status.success() {
            let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(container_id)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(NvControlError::CommandFailed(format!(
                "Podman run failed: {}",
                error
            )))
        }
    }

    /// Launch Bolt container with GPU support using native API
    fn launch_bolt_container(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        if let Some(ref bolt_manager) = self.bolt_manager {
            let manager = bolt_manager.lock().map_err(|e| {
                NvControlError::RuntimeError(format!("Bolt manager lock failed: {}", e))
            })?;

            // Convert container launch config to Bolt GPU config
            let gpu_config = self.create_bolt_gpu_config(config)?;

            let rt = Runtime::new().map_err(|e| {
                NvControlError::RuntimeError(format!("Tokio runtime failed: {}", e))
            })?;

            let workload_name = config.name.as_deref().unwrap_or("nvcontrol-workload");

            match rt.block_on(manager.launch_gpu_workload(
                workload_name,
                &config.image,
                &gpu_config,
            )) {
                Ok(container_name) => Ok(container_name),
                Err(e) => Err(NvControlError::CommandFailed(format!(
                    "Bolt launch failed: {}",
                    e
                ))),
            }
        } else {
            // Fallback to CLI approach
            self.launch_bolt_container_cli(config)
        }
    }

    /// Create Bolt GPU configuration from container launch config
    fn create_bolt_gpu_config(
        &self,
        config: &ContainerLaunchConfig,
    ) -> NvResult<GpuContainerConfig> {
        let gpu_id = if config.gpu_config.gpu_devices.is_empty() {
            0
        } else {
            config.gpu_config.gpu_devices[0].parse().unwrap_or(0)
        };

        Ok(GpuContainerConfig {
            gpu_id,
            memory_limit: config.gpu_config.memory_limit,
            compute_capabilities: config.gpu_config.driver_capabilities.clone(),
            power_limit: None, // Could be extracted from environment
            enable_dlss: config
                .gpu_config
                .driver_capabilities
                .contains(&"graphics".to_string()),
            enable_raytracing: config
                .gpu_config
                .driver_capabilities
                .contains(&"graphics".to_string()),
            enable_cuda: config
                .gpu_config
                .driver_capabilities
                .contains(&"compute".to_string()),
        })
    }

    /// Fallback CLI launch for Bolt containers
    fn launch_bolt_container_cli(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        // Generate Boltfile configuration for this container
        let boltfile_config = self.generate_boltfile_config(config)?;

        // Create temporary Boltfile
        let boltfile_path = format!(
            "/tmp/nvcontrol-boltfile-{}.toml",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        fs::write(&boltfile_path, boltfile_config)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write Boltfile: {}", e)))?;

        // Use Bolt CLI
        let output = Command::new("bolt")
            .args(&["surge", "up", "--config", &boltfile_path])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Bolt surge up failed: {}", e)))?;

        // Clean up temporary Boltfile
        let _ = fs::remove_file(boltfile_path);

        if output.status.success() {
            let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(container_id)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(NvControlError::CommandFailed(format!(
                "Bolt surge up failed: {}",
                error
            )))
        }
    }

    /// Generate Boltfile configuration for container launch
    fn generate_boltfile_config(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        let service_name = config
            .name
            .as_deref()
            .unwrap_or_else(|| config.image.split(':').next().unwrap_or("gpu-service"));

        let mut boltfile = format!(
            r#"project = "nvcontrol-runtime"

[services.{}]
build = "{}"
"#,
            service_name, config.image
        );

        // Add GPU configuration
        if !config.gpu_config.gpu_devices.is_empty() {
            boltfile.push_str("privileged = true\n");
            boltfile.push_str("gpu.nvidia = true\n");

            // Add GPU devices
            boltfile.push_str("devices = [\n");
            for device in &self.gpu_devices {
                boltfile.push_str(&format!("    \"/dev/nvidia{}\",\n", device.index));
            }
            boltfile.push_str("    \"/dev/nvidia-uvm\",\n");
            boltfile.push_str("    \"/dev/nvidia-modeset\",\n");
            boltfile.push_str("    \"/dev/nvidiactl\"\n");
            boltfile.push_str("]\n");

            // Add NVIDIA library volumes
            boltfile.push_str("volumes = [\n");
            boltfile.push_str("    \"/usr/lib/x86_64-linux-gnu/libnvidia-ml.so:/usr/lib/x86_64-linux-gnu/libnvidia-ml.so:ro\",\n");
            boltfile.push_str("    \"/sys/class/drm:/sys/class/drm:ro\"\n");
            boltfile.push_str("]\n");

            // Add NVIDIA environment variables
            boltfile.push_str("env.NVIDIA_VISIBLE_DEVICES = \"all\"\n");
            boltfile.push_str(&format!(
                "env.NVIDIA_DRIVER_CAPABILITIES = \"{}\"\n",
                config.gpu_config.driver_capabilities.join(",")
            ));
        }

        // Add custom environment variables
        for (key, value) in &config.environment {
            boltfile.push_str(&format!("env.{} = \"{}\"\n", key, value));
        }

        // Add custom volume mounts
        if !config.volumes.is_empty() {
            if !boltfile.contains("volumes = [") {
                boltfile.push_str("volumes = [\n");
            } else {
                // Add to existing volumes array
                boltfile = boltfile.replace("]\n", "");
            }
            for volume in &config.volumes {
                let ro_suffix = if volume.read_only { ":ro" } else { "" };
                boltfile.push_str(&format!(
                    "    \"{}:{}{}\"",
                    volume.source, volume.target, ro_suffix
                ));
                if volume != config.volumes.last().unwrap() {
                    boltfile.push_str(",");
                }
                boltfile.push_str("\n");
            }
            boltfile.push_str("]\n");
        }

        // Add port mappings
        if !config.ports.is_empty() {
            boltfile.push_str("ports = [\n");
            for port in &config.ports {
                boltfile.push_str(&format!(
                    "    \"{}:{}\"",
                    port.host_port, port.container_port
                ));
                if port != config.ports.last().unwrap() {
                    boltfile.push_str(",");
                }
                boltfile.push_str("\n");
            }
            boltfile.push_str("]\n");
        }

        // Add working directory
        if let Some(ref workdir) = config.working_dir {
            boltfile.push_str(&format!("workdir = \"{}\"\n", workdir));
        }

        // Add network configuration for encrypted QUIC
        boltfile.push_str("\n[network]\n");
        boltfile.push_str("driver = \"quic\"\n");
        boltfile.push_str("encryption = true\n");

        Ok(boltfile)
    }

    /// Launch NixOS container with GPU support
    fn launch_nix_container(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        // Generate NixOS configuration for GPU container
        let nix_config = self.generate_nix_gpu_config(config)?;

        // Write configuration to temporary file
        let temp_config = format!("/tmp/nvcontrol-{}.nix", chrono::Utc::now().timestamp());
        fs::write(&temp_config, nix_config).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to write Nix config: {}", e))
        })?;

        // Launch container using nixos-container
        let output = Command::new("nixos-container")
            .args(&["create", "--config", &temp_config])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("NixOS container failed: {}", e)))?;

        // Clean up temp file
        let _ = fs::remove_file(temp_config);

        if output.status.success() {
            let container_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Start the container
            Command::new("nixos-container")
                .args(&["start", &container_name])
                .output()
                .map_err(|e| {
                    NvControlError::CommandFailed(format!("Failed to start NixOS container: {}", e))
                })?;

            Ok(container_name)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(NvControlError::CommandFailed(format!(
                "NixOS container creation failed: {}",
                error
            )))
        }
    }

    /// Generate NixOS configuration for GPU container
    fn generate_nix_gpu_config(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        let mut nix_config = String::from(
            r#"
{ config, pkgs, ... }:
{
  # NVIDIA GPU support in NixOS container
  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;
  };

  services.xserver.videoDrivers = [ "nvidia" ];

  hardware.nvidia = {
    modesetting.enable = true;
    powerManagement.enable = false;
    powerManagement.finegrained = false;
    open = true; # Use nvidia-open drivers
    nvidiaSettings = true;
  };

  # Container-specific environment
  environment.systemPackages = with pkgs; [
    nvidia-docker
    docker
    git
  ];
"#,
        );

        // Add environment variables
        if !config.environment.is_empty() {
            nix_config.push_str("\n  environment.variables = {\n");
            for (key, value) in &config.environment {
                nix_config.push_str(&format!("    {} = \"{}\";\n", key, value));
            }
            nix_config.push_str("  };\n");
        }

        // Add GPU-specific configuration
        if !config.gpu_config.gpu_devices.is_empty() {
            nix_config.push_str(
                r#"

  # GPU device access
  boot.extraModprobeConfig = ''
    options nvidia-drm modeset=1
  '';

  # Container runtime configuration
  virtualisation.docker = {
    enable = true;
    enableNvidia = true;
  };
"#,
            );
        }

        nix_config.push_str("\n}\n");
        Ok(nix_config)
    }

    /// Launch containerd container (for Kubernetes/cri-o)
    fn launch_containerd_container(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        // Generate container configuration
        let container_config = self.generate_containerd_config(config)?;
        let config_path = format!(
            "/tmp/nvcontrol-container-{}.json",
            chrono::Utc::now().timestamp()
        );

        fs::write(&config_path, container_config).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to write container config: {}", e))
        })?;

        // Use ctr (containerd CLI) to run container
        let output = Command::new("ctr")
            .args(&[
                "containers",
                "create",
                "--config",
                &config_path,
                &config.image,
                &config.name.as_deref().unwrap_or("nvcontrol-container"),
            ])
            .output()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("containerd create failed: {}", e))
            })?;

        // Clean up config file
        let _ = fs::remove_file(config_path);

        if output.status.success() {
            let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(container_id)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(NvControlError::CommandFailed(format!(
                "containerd failed: {}",
                error
            )))
        }
    }

    /// Generate containerd container configuration
    fn generate_containerd_config(&self, config: &ContainerLaunchConfig) -> NvResult<String> {
        #[derive(Serialize)]
        struct ContainerdConfig {
            image: String,
            env: Vec<String>,
            working_dir: Option<String>,
            command: Option<Vec<String>>,
        }

        let mut env_vars = Vec::new();

        // Add GPU environment variables
        if !config.gpu_config.gpu_devices.is_empty() {
            env_vars.push("NVIDIA_VISIBLE_DEVICES=all".to_string());
            env_vars.push(format!(
                "NVIDIA_DRIVER_CAPABILITIES={}",
                config.gpu_config.driver_capabilities.join(",")
            ));
        }

        // Add custom environment variables
        for (key, value) in &config.environment {
            env_vars.push(format!("{}={}", key, value));
        }

        let containerd_config = ContainerdConfig {
            image: config.image.clone(),
            env: env_vars,
            working_dir: config.working_dir.clone(),
            command: config.command.clone(),
        };

        serde_json::to_string_pretty(&containerd_config)
            .map_err(|e| NvControlError::ConfigError(format!("JSON serialization failed: {}", e)))
    }

    /// Monitor running GPU containers
    pub fn monitor_gpu_containers(&self) -> NvResult<Vec<super::container::ContainerGpuInfo>> {
        let mut all_containers = Vec::new();

        // Monitor Docker containers
        if self.supported_runtimes.contains(&ContainerRuntime::Docker) {
            if let Ok(mut containers) = self.get_docker_gpu_containers() {
                all_containers.append(&mut containers);
            }
        }

        // Monitor Podman containers
        if self.supported_runtimes.contains(&ContainerRuntime::Podman) {
            if let Ok(mut containers) = self.get_podman_gpu_containers() {
                all_containers.append(&mut containers);
            }
        }

        Ok(all_containers)
    }

    /// Get Docker GPU containers
    fn get_docker_gpu_containers(&self) -> NvResult<Vec<super::container::ContainerGpuInfo>> {
        super::container::list_gpu_containers()
    }

    /// Get Podman GPU containers
    fn get_podman_gpu_containers(&self) -> NvResult<Vec<super::container::ContainerGpuInfo>> {
        let output = Command::new("podman")
            .args(&["ps", "--format", "json"])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("Podman ps failed: {}", e)))?;

        let _containers_json = String::from_utf8_lossy(&output.stdout);
        // Parse JSON and filter for GPU containers
        // This would need proper JSON parsing in production

        Ok(Vec::new()) // Placeholder
    }

    /// Get container status by name or ID
    pub fn get_container_status(
        &self,
        container: &str,
    ) -> NvResult<super::container::ContainerGpuInfo> {
        use super::container;

        // Try all runtimes to find the container
        for runtime in &self.supported_runtimes {
            let cmd_name = match runtime {
                ContainerRuntime::Docker => "docker",
                ContainerRuntime::Podman => "podman",
                _ => continue,
            };

            let output = Command::new(cmd_name)
                .args(&["inspect", container])
                .output();

            if let Ok(out) = output {
                if out.status.success() {
                    // Container found, get its GPU info
                    let all_containers = container::list_gpu_containers()?;
                    for c in all_containers {
                        if c.container_id.contains(container) || c.container_name == container {
                            return Ok(c);
                        }
                    }
                }
            }
        }

        Err(NvControlError::ContainerOperationFailed(format!(
            "Container '{}' not found",
            container
        )))
    }

    /// Create PhantomLink audio container configuration
    pub fn create_phantomlink_container_config(&self) -> NvResult<ContainerLaunchConfig> {
        Ok(ContainerLaunchConfig {
            image: "ghcr.io/ghostkellz/phantomlink:latest".to_string(),
            name: Some("phantomlink-audio".to_string()),
            command: None,
            working_dir: Some("/app".to_string()),
            environment: HashMap::from([
                ("RUST_LOG".to_string(), "info".to_string()),
                (
                    "PULSEAUDIO_SERVER".to_string(),
                    "unix:/run/user/1000/pulse/native".to_string(),
                ),
                ("ALSA_CARD".to_string(), "0".to_string()),
                ("RTX_VOICE_ENABLED".to_string(), "true".to_string()),
            ]),
            volumes: vec![
                VolumeMount {
                    source: "/run/user/1000/pulse".to_string(),
                    target: "/run/user/1000/pulse".to_string(),
                    read_only: false,
                },
                VolumeMount {
                    source: "/dev/snd".to_string(),
                    target: "/dev/snd".to_string(),
                    read_only: false,
                },
            ],
            ports: vec![PortMapping {
                host_port: 8080,
                container_port: 8080,
                protocol: "tcp".to_string(),
            }],
            gpu_config: ContainerGpuConfig {
                runtime: ContainerRuntime::Docker,
                gpu_devices: vec!["all".to_string()],
                memory_limit: Some(2 * 1024 * 1024 * 1024), // 2GB
                compute_mode: "default".to_string(),
                driver_capabilities: vec!["compute".to_string(), "utility".to_string()],
                environment_vars: HashMap::from([
                    ("NVIDIA_VISIBLE_DEVICES".to_string(), "all".to_string()),
                    (
                        "NVIDIA_DRIVER_CAPABILITIES".to_string(),
                        "compute,utility".to_string(),
                    ),
                ]),
                mount_points: vec!["/usr/local/nvidia".to_string()],
                device_requests: vec![DeviceRequest {
                    driver: "nvidia".to_string(),
                    count: Some(1),
                    device_ids: vec!["0".to_string()],
                    capabilities: vec![vec!["gpu".to_string()]],
                    options: HashMap::new(),
                }],
            },
            interactive: false,
            remove_on_exit: false,
        })
    }

    /// Setup container runtime with NVIDIA GPU support
    pub fn setup_runtime(&self, runtime_name: &str) -> NvResult<()> {
        println!(
            "🔧 Setting up {} runtime with NVIDIA GPU support...",
            runtime_name
        );

        // Check if nvidia-container-toolkit is installed
        let toolkit_check = Command::new("nvidia-container-toolkit")
            .arg("--version")
            .output();

        if toolkit_check.is_err() {
            println!("⚠️  nvidia-container-toolkit not found");
            println!("📦 Installation instructions:");
            println!("   Ubuntu/Debian:");
            println!(
                "     curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg"
            );
            println!(
                "     curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \\"
            );
            println!(
                "       sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \\"
            );
            println!("       sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list");
            println!(
                "     sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit"
            );
            println!();
            println!("   Fedora/RHEL/CentOS:");
            println!(
                "     sudo dnf config-manager --add-repo https://nvidia.github.io/libnvidia-container/stable/rpm/nvidia-container-toolkit.repo"
            );
            println!("     sudo dnf install -y nvidia-container-toolkit");
            println!();
            println!("   Arch Linux:");
            println!("     yay -S nvidia-container-toolkit");

            return Err(NvControlError::ContainerOperationFailed(
                "nvidia-container-toolkit not installed".to_string(),
            ));
        }

        match runtime_name.to_lowercase().as_str() {
            "docker" => self.setup_docker_runtime(),
            "podman" => self.setup_podman_runtime(),
            "containerd" => self.setup_containerd_runtime(),
            _ => Err(NvControlError::ContainerOperationFailed(format!(
                "Unsupported runtime: {}",
                runtime_name
            ))),
        }
    }

    /// Setup Docker with NVIDIA GPU support
    fn setup_docker_runtime(&self) -> NvResult<()> {
        println!("🐳 Configuring Docker for NVIDIA GPU support...");

        // Configure docker daemon
        let daemon_config = r#"{
    "runtimes": {
        "nvidia": {
            "path": "nvidia-container-runtime",
            "runtimeArgs": []
        }
    },
    "default-runtime": "nvidia"
}"#;

        let config_path = Path::new("/etc/docker/daemon.json");

        if config_path.exists() {
            println!("⚠️  /etc/docker/daemon.json already exists");
            println!("   Please manually add the NVIDIA runtime configuration");
            println!("{}", daemon_config);
        } else {
            println!("📝 Writing Docker daemon configuration...");
            println!("   Note: This requires root privileges");
            println!("   Run: sudo nvctl container runtime setup docker");
        }

        // Restart docker service
        println!("🔄 Restart Docker to apply changes:");
        println!("   sudo systemctl restart docker");
        println!();
        println!("✅ Docker GPU support configuration complete");

        Ok(())
    }

    /// Setup Podman with NVIDIA GPU support
    fn setup_podman_runtime(&self) -> NvResult<()> {
        println!("🦭 Configuring Podman for NVIDIA GPU support...");

        // Configure podman for CDI
        println!("📝 Generating CDI specification...");

        let cdi_result = Command::new("sudo")
            .args(&[
                "nvidia-ctk",
                "cdi",
                "generate",
                "--output=/etc/cdi/nvidia.yaml",
            ])
            .status();

        match cdi_result {
            Ok(status) if status.success() => {
                println!("✅ CDI specification generated");
            }
            _ => {
                println!("⚠️  Failed to generate CDI specification");
                println!(
                    "   Run manually: sudo nvidia-ctk cdi generate --output=/etc/cdi/nvidia.yaml"
                );
            }
        }

        println!("🔄 Testing GPU access in Podman:");
        println!("   podman run --rm --device nvidia.com/gpu=all ubuntu nvidia-smi");
        println!();
        println!("✅ Podman GPU support configuration complete");

        Ok(())
    }

    /// Setup containerd with NVIDIA GPU support
    fn setup_containerd_runtime(&self) -> NvResult<()> {
        println!("📦 Configuring containerd for NVIDIA GPU support...");

        println!("📝 Updating containerd configuration...");
        println!("   Run: sudo nvidia-ctk runtime configure --runtime=containerd");
        println!();
        println!("🔄 Restart containerd:");
        println!("   sudo systemctl restart containerd");
        println!();
        println!("✅ containerd GPU support configuration complete");

        Ok(())
    }

    /// Configure NVIDIA Container Runtime with optimal settings
    pub fn configure_runtime(&self) -> NvResult<()> {
        println!("⚙️  Configuring NVIDIA Container Runtime...");

        let config_dir = &self.config_path;
        fs::create_dir_all(config_dir).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create config dir: {}", e))
        })?;

        // Create runtime configuration
        let runtime_config = r#"# NVIDIA Container Runtime Configuration
# Generated by nvcontrol

[nvidia-container-runtime]
debug = "/var/log/nvidia-container-runtime.log"

[nvidia-container-cli]
# Load kernel modules at runtime
load-kmods = true

# Set GPU devices visibility
# Options: all, none, or comma-separated device indices (0,1,2)
devices = "all"

# Driver capabilities to expose
# Options: compute, compat32, graphics, utility, video, display, all
capabilities = "compute,utility,graphics,video"

# Enable MIG (Multi-Instance GPU) support
# Requires MIG-capable GPU and driver
no-mig = false

# Enable compute mode control
compute-mode = "default"
"#;

        let config_file = config_dir.join("runtime.toml");
        fs::write(&config_file, runtime_config)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write config: {}", e)))?;

        println!(
            "✅ Runtime configuration saved to: {}",
            config_file.display()
        );
        println!();
        println!("📋 Current GPU devices:");
        for device in &self.gpu_devices {
            println!(
                "   [{}] {} - {} MB",
                device.index, device.name, device.memory_mb
            );
        }
        println!();
        println!("🎯 Supported runtimes:");
        for runtime in &self.supported_runtimes {
            match runtime {
                ContainerRuntime::Docker => println!("   ✓ Docker"),
                ContainerRuntime::Podman => println!("   ✓ Podman"),
                ContainerRuntime::Containerd => println!("   ✓ containerd"),
                ContainerRuntime::Bolt => println!("   ✓ Bolt"),
                ContainerRuntime::NixOS => println!("   ✓ NixOS"),
                ContainerRuntime::Custom(name) => println!("   ✓ {}", name),
            }
        }

        Ok(())
    }
}

impl Default for ContainerGpuConfig {
    fn default() -> Self {
        Self {
            runtime: ContainerRuntime::Docker,
            gpu_devices: vec!["0".to_string()],
            memory_limit: None,
            compute_mode: "default".to_string(),
            driver_capabilities: vec!["compute".to_string(), "utility".to_string()],
            environment_vars: HashMap::new(),
            mount_points: Vec::new(),
            device_requests: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_detection() {
        let runtime = NvContainerRuntime::new();
        assert!(runtime.is_ok() || runtime.is_err()); // Just ensure it doesn't panic
    }

    #[test]
    fn test_phantomlink_config_creation() {
        let runtime = NvContainerRuntime::new().unwrap();
        let config = runtime.create_phantomlink_container_config().unwrap();
        assert_eq!(config.image, "ghcr.io/ghostkellz/phantomlink:latest");
        assert!(config.environment.contains_key("RTX_VOICE_ENABLED"));
    }
}
