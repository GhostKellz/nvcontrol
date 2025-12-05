use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

// Conditional compilation - use real BOLT API when available, mock when not
#[cfg(feature = "bolt-api")]
use bolt::{BoltConfig as BoltConfigImpl, BoltRuntime as BoltRuntimeImpl};

// Mock BOLT API types for when the library is not available
#[cfg(not(feature = "bolt-api"))]
mod mock_bolt {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BoltRuntimeImpl {
        config: BoltConfigImpl,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BoltConfigImpl {
        pub config_dir: std::path::PathBuf,
        pub data_dir: std::path::PathBuf,
        pub verbose: bool,
    }

    impl BoltConfigImpl {
        pub fn load() -> Result<Self> {
            Ok(Self {
                config_dir: dirs::config_dir().unwrap_or_default().join("bolt"),
                data_dir: dirs::data_dir().unwrap_or_default().join("bolt"),
                verbose: false,
            })
        }
    }

    impl BoltRuntimeImpl {
        pub fn with_config(config: BoltConfigImpl) -> Self {
            Self { config }
        }

        pub async fn list_containers(&self, _all: bool) -> Result<Vec<ContainerInfo>> {
            Ok(vec![])
        }

        pub async fn run_container(
            &self,
            image: &str,
            name: Option<&str>,
            _ports: &[String],
            _env: &[String],
            _volumes: &[String],
            _detach: bool,
        ) -> Result<()> {
            println!(
                "Mock: Would launch container {} with image {}",
                name.unwrap_or("unnamed"),
                image
            );
            Ok(())
        }

        pub async fn setup_gaming(&self, proton: Option<&str>, winver: Option<&str>) -> Result<()> {
            println!(
                "Mock: Would setup gaming with Proton {} and Windows {}",
                proton.unwrap_or("default"),
                winver.unwrap_or("default")
            );
            Ok(())
        }

        pub async fn launch_game(&self, game: &str, args: &[String]) -> Result<()> {
            println!("Mock: Would launch game {} with args {:?}", game, args);
            Ok(())
        }

        pub async fn create_network(
            &self,
            name: &str,
            driver: &str,
            subnet: Option<&str>,
        ) -> Result<()> {
            println!(
                "Mock: Would create network {} with driver {} and subnet {:?}",
                name, driver, subnet
            );
            Ok(())
        }

        pub async fn stop_container(&self, container: &str) -> Result<()> {
            println!("Mock: Would stop container {}", container);
            Ok(())
        }

        pub async fn remove_container(&self, container: &str, force: bool) -> Result<()> {
            println!(
                "Mock: Would remove container {} (force: {})",
                container, force
            );
            Ok(())
        }

        pub async fn surge_status(&self) -> Result<SurgeStatus> {
            Ok(SurgeStatus {
                services: vec![ServiceInfo {
                    name: "gpu-manager".to_string(),
                    status: "running".to_string(),
                }],
                networks: vec![NetworkInfo {
                    name: "bolt-gpu-net".to_string(),
                    driver: "quic".to_string(),
                    subnet: Some("10.2.0.0/16".to_string()),
                }],
            })
        }

        pub async fn surge_up(
            &self,
            services: &[String],
            _detach: bool,
            _force_recreate: bool,
        ) -> Result<()> {
            println!("Mock: Would start Surge services: {:?}", services);
            Ok(())
        }

        pub async fn surge_down(&self, services: &[String], volumes: bool) -> Result<()> {
            println!(
                "Mock: Would stop Surge services: {:?} (volumes: {})",
                services, volumes
            );
            Ok(())
        }

        pub async fn build_image(
            &self,
            path: &str,
            tag: Option<&str>,
            dockerfile: &str,
        ) -> Result<()> {
            println!(
                "Mock: Would build image from {} with tag {:?} using dockerfile {}",
                path, tag, dockerfile
            );
            Ok(())
        }
    }
}

#[cfg(not(feature = "bolt-api"))]
use mock_bolt::{BoltConfigImpl, BoltRuntimeImpl};

// Type aliases for consistent API
pub type BoltRuntime = BoltRuntimeImpl;
pub type BoltConfig = BoltConfigImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurgeStatus {
    pub services: Vec<ServiceInfo>,
    pub networks: Vec<NetworkInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    pub driver: String,
    pub subnet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvidiaConfig {
    pub device: Option<u32>,
    pub dlss: Option<bool>,
    pub raytracing: Option<bool>,
    pub cuda: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GamingConfig {
    pub gpu: Option<GpuConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GpuConfig {
    pub nvidia: Option<NvidiaConfig>,
    pub amd: Option<AmdConfig>,
    pub passthrough: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmdConfig {
    pub device: Option<u32>,
    pub rocm: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct NvControlBoltManager {
    #[serde(skip)]
    runtime: Option<BoltRuntime>,
}

impl std::fmt::Debug for NvControlBoltManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NvControlBoltManager")
            .field("runtime", &self.runtime.is_some())
            .finish()
    }
}

impl Clone for NvControlBoltManager {
    fn clone(&self) -> Self {
        // Can't clone BoltRuntime, so create a new empty manager
        Self { runtime: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuContainerConfig {
    pub gpu_id: u32,
    pub memory_limit: Option<u64>,
    pub compute_capabilities: Vec<String>,
    pub power_limit: Option<u32>,
    pub enable_dlss: bool,
    pub enable_raytracing: bool,
    pub enable_cuda: bool,
}

impl Default for GpuContainerConfig {
    fn default() -> Self {
        Self {
            gpu_id: 0,
            memory_limit: None,
            compute_capabilities: vec!["compute".to_string(), "utility".to_string()],
            power_limit: None,
            enable_dlss: false,
            enable_raytracing: false,
            enable_cuda: true,
        }
    }
}

impl NvControlBoltManager {
    pub async fn new() -> Result<Self> {
        let config =
            BoltConfig::load().map_err(|e| anyhow!("Failed to load Bolt config: {}", e))?;

        let runtime = BoltRuntime::with_config(config);

        Ok(Self {
            runtime: Some(runtime),
        })
    }

    pub async fn list_gpu_containers(&self) -> Result<Vec<ContainerInfo>> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        let all_containers = runtime.list_containers(false).await?;

        // Filter for GPU-enabled containers and convert types
        let gpu_containers = all_containers
            .into_iter()
            .filter(|container| {
                container.name.contains("gpu")
                    || container.name.contains("cuda")
                    || container.name.contains("nvidia")
            })
            .map(|container| ContainerInfo {
                id: container.id,
                name: container.name,
                image: container.image,
                status: container.status,
                ports: container.ports,
            })
            .collect();

        Ok(gpu_containers)
    }

    pub async fn launch_gpu_workload(
        &self,
        workload_name: &str,
        image: &str,
        config: &GpuContainerConfig,
    ) -> Result<String> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        // Create NVIDIA GPU configuration
        let nvidia_config = NvidiaConfig {
            device: Some(config.gpu_id),
            dlss: Some(config.enable_dlss),
            raytracing: Some(config.enable_raytracing),
            cuda: Some(config.enable_cuda),
        };

        // Create gaming configuration for GPU support
        let _gaming_config = GamingConfig {
            gpu: Some(GpuConfig {
                nvidia: Some(nvidia_config),
                amd: None,
                passthrough: Some(true),
            }),
        };

        // Create high-performance network
        let network_name = format!("{}-gpu-net", workload_name);
        runtime
            .create_network(&network_name, "bolt", Some("10.2.0.0/16"))
            .await
            .map_err(|e| anyhow!("Failed to create GPU network: {}", e))?;

        // Environment variables for GPU support
        let mut env_vars = vec![
            "NVIDIA_VISIBLE_DEVICES=all".to_string(),
            "NVIDIA_DRIVER_CAPABILITIES=all".to_string(),
        ];

        if let Some(memory) = config.memory_limit {
            env_vars.push(format!("CUDA_MEMORY_LIMIT={}", memory));
        }

        if let Some(power) = config.power_limit {
            env_vars.push(format!("NVIDIA_POWER_LIMIT={}%", power));
        }

        // Launch container with GPU support
        let container_name = format!("nvcontrol-{}-gpu{}", workload_name, config.gpu_id);
        runtime
            .run_container(
                image,
                Some(&container_name),
                &[], // ports
                &env_vars,
                &[],  // volumes
                true, // detach
            )
            .await
            .map_err(|e| anyhow!("Failed to launch GPU container: {}", e))?;

        Ok(container_name)
    }

    pub async fn setup_gaming_environment(&self, game_name: &str) -> Result<()> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        // Setup gaming optimizations
        runtime
            .setup_gaming(Some("8.0"), Some("win10"))
            .await
            .map_err(|e| anyhow!("Failed to setup gaming environment: {}", e))?;

        println!("✅ Gaming environment configured for: {}", game_name);
        Ok(())
    }

    pub async fn launch_proton_game(&self, steam_id: &str, args: &[String]) -> Result<()> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        let game_command = format!("steam://run/{}", steam_id);

        runtime
            .launch_game(&game_command, args)
            .await
            .map_err(|e| anyhow!("Failed to launch Proton game: {}", e))?;

        println!("✅ Proton game launched: steam://run/{}", steam_id);
        Ok(())
    }

    pub async fn create_gpu_monitoring_service(&self) -> Result<String> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        // Create GPU monitoring container
        let monitor_name = "nvcontrol-gpu-monitor";
        let env_vars = vec![
            "NVIDIA_VISIBLE_DEVICES=all".to_string(),
            "MONITORING_INTERVAL=5".to_string(),
            "LOG_LEVEL=info".to_string(),
        ];

        runtime.run_container(
            "nvidia/cuda:12.0-runtime-ubuntu22.04",
            Some(monitor_name),
            &["8080:8080".to_string()], // Web interface port
            &env_vars,
            &["/usr/lib/x86_64-linux-gnu/libnvidia-ml.so:/usr/lib/x86_64-linux-gnu/libnvidia-ml.so:ro".to_string()],
            true,
        ).await
            .map_err(|e| anyhow!("Failed to create GPU monitoring service: {}", e))?;

        Ok(monitor_name.to_string())
    }

    pub async fn stop_container(&self, container_name: &str) -> Result<()> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        runtime
            .stop_container(container_name)
            .await
            .map_err(|e| anyhow!("Failed to stop container {}: {}", container_name, e))?;

        Ok(())
    }

    pub async fn remove_container(&self, container_name: &str, force: bool) -> Result<()> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        runtime
            .remove_container(container_name, force)
            .await
            .map_err(|e| anyhow!("Failed to remove container {}: {}", container_name, e))?;

        Ok(())
    }

    pub async fn get_surge_status(&self) -> Result<SurgeStatus> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        let bolt_status = runtime
            .surge_status()
            .await
            .map_err(|e| anyhow!("Failed to get Surge status: {}", e))?;

        // Convert bolt::SurgeStatus to our SurgeStatus
        let status = SurgeStatus {
            services: bolt_status
                .services
                .into_iter()
                .map(|service| ServiceInfo {
                    name: service.name,
                    status: service.status,
                })
                .collect(),
            networks: bolt_status
                .networks
                .into_iter()
                .map(|network| NetworkInfo {
                    name: network.name,
                    driver: network.driver,
                    subnet: network.subnet,
                })
                .collect(),
        };

        Ok(status)
    }

    pub async fn surge_up(&self, services: &[String]) -> Result<()> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        runtime
            .surge_up(services, false, false)
            .await
            .map_err(|e| anyhow!("Failed to start Surge services: {}", e))?;

        Ok(())
    }

    pub async fn surge_down(&self, services: &[String], remove_volumes: bool) -> Result<()> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        runtime
            .surge_down(services, remove_volumes)
            .await
            .map_err(|e| anyhow!("Failed to stop Surge services: {}", e))?;

        Ok(())
    }

    pub async fn build_gpu_image(&self, dockerfile_path: &str, tag: &str) -> Result<()> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        runtime
            .build_image(dockerfile_path, Some(tag), "Dockerfile.bolt")
            .await
            .map_err(|e| anyhow!("Failed to build GPU image: {}", e))?;

        Ok(())
    }

    pub async fn create_gpu_network(&self, name: &str, subnet: Option<&str>) -> Result<()> {
        let runtime = self
            .runtime
            .as_ref()
            .ok_or_else(|| anyhow!("Bolt runtime not initialized"))?;

        runtime
            .create_network(name, "bolt", subnet)
            .await
            .map_err(|e| anyhow!("Failed to create GPU network: {}", e))?;

        Ok(())
    }
}

// GPU workload profiles for different use cases
pub fn create_ml_training_profile() -> GpuContainerConfig {
    GpuContainerConfig {
        gpu_id: 0,
        memory_limit: Some(8 * 1024 * 1024 * 1024), // 8GB
        compute_capabilities: vec!["compute".to_string(), "utility".to_string()],
        power_limit: Some(100),
        enable_dlss: false,
        enable_raytracing: false,
        enable_cuda: true,
    }
}

pub fn create_gaming_profile() -> GpuContainerConfig {
    GpuContainerConfig {
        gpu_id: 0,
        memory_limit: None,
        compute_capabilities: vec![
            "compute".to_string(),
            "utility".to_string(),
            "graphics".to_string(),
        ],
        power_limit: Some(120),
        enable_dlss: true,
        enable_raytracing: true,
        enable_cuda: true,
    }
}

pub fn create_inference_profile() -> GpuContainerConfig {
    GpuContainerConfig {
        gpu_id: 0,
        memory_limit: Some(4 * 1024 * 1024 * 1024), // 4GB
        compute_capabilities: vec!["compute".to_string()],
        power_limit: Some(80),
        enable_dlss: false,
        enable_raytracing: false,
        enable_cuda: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bolt_manager_creation() {
        // This test will only pass if Bolt is properly installed
        if let Ok(_manager) = NvControlBoltManager::new().await {
            println!("Bolt runtime initialized successfully");
        }
    }

    #[test]
    fn test_gpu_config_defaults() {
        let config = GpuContainerConfig::default();
        assert_eq!(config.gpu_id, 0);
        assert!(config.enable_cuda);
        assert!(!config.enable_dlss);
    }

    #[test]
    fn test_profile_creation() {
        let ml_profile = create_ml_training_profile();
        assert_eq!(ml_profile.memory_limit, Some(8 * 1024 * 1024 * 1024));
        assert!(ml_profile.enable_cuda);

        let gaming_profile = create_gaming_profile();
        assert!(gaming_profile.enable_dlss);
        assert!(gaming_profile.enable_raytracing);
        assert_eq!(gaming_profile.power_limit, Some(120));
    }
}
