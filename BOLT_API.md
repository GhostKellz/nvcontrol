# Bolt Rust API Documentation

**Bolt** provides a comprehensive Rust API for container runtime operations, gaming optimizations, and orchestration. This document covers the complete API surface for integration into your Rust projects.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core API](#core-api)
- [Gaming Integration](#gaming-integration)
- [Network Management](#network-management)
- [Configuration Management](#configuration-management)
- [Error Handling](#error-handling)
- [Feature Flags](#feature-flags)
- [Integration Examples](#integration-examples)

## Installation

Add Bolt to your `Cargo.toml`:

```toml
[dependencies]
bolt = { git = "https://github.com/CK-Technology/bolt" }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

For specific features only:

```toml
[dependencies]
bolt = { git = "https://github.com/CK-Technology/bolt", features = ["gaming", "quic-networking"] }
```

## Quick Start

```rust
use bolt::api::*;

#[tokio::main]
async fn main() -> bolt::Result<()> {
    // Create a new Bolt runtime
    let runtime = BoltRuntime::new()?;

    // Run a basic container
    runtime.run_container(
        "nginx:latest",
        Some("web-server"),
        &["8080:80"],
        &[],
        &[],
        false
    ).await?;

    // List running containers
    let containers = runtime.list_containers(false).await?;
    println!("Running containers: {}", containers.len());

    Ok(())
}
```

## Core API

### BoltRuntime

The main entry point for all Bolt operations.

```rust
use bolt::{BoltRuntime, BoltConfig};

impl BoltRuntime {
    /// Create a new runtime with default configuration
    pub fn new() -> bolt::Result<Self>;

    /// Create runtime with custom configuration
    pub fn with_config(config: BoltConfig) -> Self;

    /// Container Management
    pub async fn run_container(
        &self,
        image: &str,
        name: Option<&str>,
        ports: &[String],
        env: &[String],
        volumes: &[String],
        detach: bool,
    ) -> bolt::Result<()>;

    pub async fn list_containers(&self, all: bool) -> bolt::Result<Vec<ContainerInfo>>;
    pub async fn stop_container(&self, container: &str) -> bolt::Result<()>;
    pub async fn remove_container(&self, container: &str, force: bool) -> bolt::Result<()>;

    /// Image Management
    pub async fn build_image(&self, path: &str, tag: Option<&str>, dockerfile: &str) -> bolt::Result<()>;
    pub async fn pull_image(&self, image: &str) -> bolt::Result<()>;
    pub async fn push_image(&self, image: &str) -> bolt::Result<()>;

    /// Surge Orchestration
    pub async fn surge_up(&self, services: &[String], detach: bool, force_recreate: bool) -> bolt::Result<()>;
    pub async fn surge_down(&self, services: &[String], volumes: bool) -> bolt::Result<()>;
    pub async fn surge_status(&self) -> bolt::Result<SurgeStatus>;
    pub async fn surge_scale(&self, services: &[String]) -> bolt::Result<()>;

    /// Gaming Features
    pub async fn setup_gaming(&self, proton: Option<&str>, winver: Option<&str>) -> bolt::Result<()>;
    pub async fn launch_game(&self, game: &str, args: &[String]) -> bolt::Result<()>;

    /// Network Management
    pub async fn create_network(&self, name: &str, driver: &str, subnet: Option<&str>) -> bolt::Result<()>;
    pub async fn list_networks(&self) -> bolt::Result<Vec<NetworkInfo>>;
    pub async fn remove_network(&self, name: &str) -> bolt::Result<()>;
}
```

### Data Types

```rust
/// Container information
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<String>,
}

/// Service orchestration status
#[derive(Debug, Clone)]
pub struct SurgeStatus {
    pub services: Vec<ServiceInfo>,
    pub networks: Vec<NetworkInfo>,
}

/// Network information
#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub name: String,
    pub driver: String,
    pub subnet: Option<String>,
}
```

## Gaming Integration

Bolt provides specialized gaming container support with GPU passthrough, Wine/Proton integration, and performance optimizations.

### Gaming Configuration

```rust
use bolt::config::*;

// NVIDIA GPU Configuration
let nvidia_config = NvidiaConfig {
    device: Some(0),           // GPU device ID
    dlss: Some(true),          // Enable DLSS
    raytracing: Some(true),    // Enable ray tracing
    cuda: Some(false),         // CUDA support
};

// AMD GPU Configuration
let amd_config = AmdConfig {
    device: Some(0),           // GPU device ID
    rocm: Some(true),          // ROCm support
};

// Audio Configuration
let audio_config = AudioConfig {
    system: "pipewire".to_string(),  // or "pulseaudio"
    latency: Some("low".to_string()),
};

// Wine/Proton Configuration
let wine_config = WineConfig {
    version: None,
    proton: Some("8.0".to_string()),
    winver: Some("win10".to_string()),
    prefix: Some("/games/wine-prefix".to_string()),
};

// Performance Configuration
let performance_config = PerformanceConfig {
    cpu_governor: Some("performance".to_string()),
    nice_level: Some(-10),
    rt_priority: Some(50),
};

// Complete Gaming Configuration
let gaming_config = GamingConfig {
    gpu: Some(GpuConfig {
        nvidia: Some(nvidia_config),
        amd: None,
        passthrough: Some(true),
    }),
    audio: Some(audio_config),
    wine: Some(wine_config),
    performance: Some(performance_config),
};
```

### Gaming Container Example

```rust
use bolt::{BoltRuntime, BoltFileBuilder};

#[tokio::main]
async fn main() -> bolt::Result<()> {
    let runtime = BoltRuntime::new()?;

    // Create gaming-optimized Boltfile
    let boltfile = BoltFileBuilder::new("gaming-setup")
        .add_gaming_service("steam", "bolt://steam:latest", gaming_config)
        .build();

    // Save and deploy
    let config = runtime.config();
    config.save_boltfile(&boltfile)?;

    runtime.surge_up(&[], false, false).await?;

    // Launch a game
    runtime.launch_game("steam://run/123456", &["--fullscreen".to_string()]).await?;

    Ok(())
}
```

## Network Management

Bolt supports multiple networking modes including high-performance QUIC networking for gaming.

```rust
use bolt::BoltRuntime;

let runtime = BoltRuntime::new()?;

// Create a gaming-optimized network with QUIC
runtime.create_network("gaming-net", "bolt", Some("10.1.0.0/16")).await?;

// Create a bridge network
runtime.create_network("app-net", "bridge", Some("172.20.0.0/16")).await?;

// List all networks
let networks = runtime.list_networks().await?;
for network in networks {
    println!("Network: {} (driver: {}, subnet: {:?})",
             network.name, network.driver, network.subnet);
}
```

## Configuration Management

### BoltConfig

```rust
use bolt::config::BoltConfig;
use std::path::PathBuf;

impl BoltConfig {
    /// Load configuration from default locations
    pub fn load() -> bolt::Result<Self>;

    /// Load Boltfile from configured path
    pub fn load_boltfile(&self) -> bolt::Result<BoltFile>;

    /// Save Boltfile to configured path
    pub fn save_boltfile(&self, boltfile: &BoltFile) -> bolt::Result<()>;
}

pub struct BoltConfig {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub boltfile_path: PathBuf,
    pub verbose: bool,
}
```

### Programmatic Boltfile Creation

```rust
use bolt::{BoltFileBuilder, Service};

// Method 1: Using the builder
let boltfile = BoltFileBuilder::new("my-project")
    .add_service("web", Service {
        image: Some("nginx:latest".to_string()),
        ports: Some(vec!["80:80".to_string()]),
        ..Default::default()
    })
    .add_gaming_service("game", "bolt://steam:latest", gaming_config)
    .build();

// Method 2: Creating example and modifying
let mut boltfile = bolt::config::create_example_boltfile();
boltfile.project = "my-custom-project".to_string();
```

## Error Handling

Bolt uses structured error types for better error handling:

```rust
use bolt::{BoltError, Result};

match runtime.run_container("invalid:image", None, &[], &[], &[], false).await {
    Ok(_) => println!("Container started successfully"),
    Err(BoltError::Runtime(err)) => {
        eprintln!("Runtime error: {}", err);
    },
    Err(BoltError::Config(err)) => {
        eprintln!("Configuration error: {}", err);
    },
    Err(BoltError::Gaming(err)) => {
        eprintln!("Gaming setup error: {}", err);
    },
    Err(err) => {
        eprintln!("Other error: {}", err);
    }
}
```

### Error Types

- `BoltError::Config` - Configuration and Boltfile errors
- `BoltError::Runtime` - Container runtime errors
- `BoltError::Network` - Networking errors
- `BoltError::Gaming` - Gaming setup errors
- `BoltError::Io` - File system I/O errors
- `BoltError::Serialization` - TOML parsing errors

## Feature Flags

Control which components are included:

```toml
[dependencies]
bolt = {
    git = "https://github.com/CK-Technology/bolt",
    features = ["gaming", "quic-networking", "nvidia-support"]
}
```

Available features:

- `gaming` - Gaming optimizations, GPU support, Wine/Proton
- `quic-networking` - Ultra-low latency QUIC networking
- `oci-runtime` - Full OCI container support
- `nvidia-support` - NVIDIA GPU passthrough (requires drivers)
- `amd-support` - AMD GPU support (requires Mesa/ROCm)

## Integration Examples

### Ghostforge Integration

```rust
// ghostforge/src/bolt_integration.rs
use bolt::{BoltRuntime, api::*};

pub struct GhostforgeBoltManager {
    runtime: BoltRuntime,
}

impl GhostforgeBoltManager {
    pub fn new() -> bolt::Result<Self> {
        Ok(Self {
            runtime: BoltRuntime::new()?,
        })
    }

    pub async fn create_gaming_container(
        &self,
        game_name: &str,
        gpu_config: GamingConfig
    ) -> bolt::Result<()> {
        // Create isolated gaming environment
        let boltfile = BoltFileBuilder::new(&format!("ghostforge-{}", game_name))
            .add_gaming_service("game-container", "bolt://gaming-base", gpu_config)
            .build();

        self.runtime.config().save_boltfile(&boltfile)?;
        self.runtime.surge_up(&[], false, false).await?;

        Ok(())
    }

    pub async fn launch_proton_game(&self, steam_id: &str) -> bolt::Result<()> {
        self.runtime.setup_gaming(Some("8.0"), Some("win10")).await?;
        self.runtime.launch_game(&format!("steam://run/{}", steam_id), &[]).await?;
        Ok(())
    }
}
```

### nvcontrol Integration

```rust
// nvcontrol/src/bolt_gpu.rs
use bolt::{BoltRuntime, api::*};

pub struct NvControlBoltIntegration {
    bolt_runtime: BoltRuntime,
}

impl NvControlBoltIntegration {
    pub async fn allocate_gpu_container(
        &self,
        workload: &str,
        gpu_id: u32
    ) -> bolt::Result<String> {
        let nvidia_config = NvidiaConfig {
            device: Some(gpu_id),
            cuda: Some(true),
            dlss: Some(false),
            raytracing: Some(false),
        };

        let gaming_config = GamingConfig {
            gpu: Some(GpuConfig {
                nvidia: Some(nvidia_config),
                passthrough: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Create GPU-enabled network
        self.bolt_runtime
            .create_network("gpu-net", "bolt", Some("10.2.0.0/16"))
            .await?;

        // Run GPU workload container
        let container_name = format!("gpu-workload-{}", gpu_id);
        self.bolt_runtime
            .run_container(workload, Some(&container_name), &[], &[], &[], true)
            .await?;

        Ok(container_name)
    }

    pub async fn monitor_gpu_containers(&self) -> bolt::Result<Vec<ContainerInfo>> {
        let containers = self.bolt_runtime.list_containers(false).await?;
        Ok(containers.into_iter()
            .filter(|c| c.name.contains("gpu-workload"))
            .collect())
    }
}
```

### Custom Project Integration

```rust
// my_project/src/main.rs
use bolt::{BoltRuntime, BoltFileBuilder, api::*};

#[tokio::main]
async fn main() -> bolt::Result<()> {
    let runtime = BoltRuntime::new()?;

    // Create multi-service application
    let boltfile = BoltFileBuilder::new("my-app")
        .add_service("database", Service {
            capsule: Some("postgres".to_string()),
            auth: Some(Auth {
                user: "app".to_string(),
                password: "secure_password".to_string(),
            }),
            storage: Some(Storage {
                size: "10Gi".to_string(),
                driver: None,
            }),
            ..Default::default()
        })
        .add_service("api", Service {
            build: Some("./api".to_string()),
            ports: Some(vec!["3000:3000".to_string()]),
            env: Some({
                let mut env = std::collections::HashMap::new();
                env.insert("DATABASE_URL".to_string(), "bolt://database".to_string());
                env
            }),
            depends_on: Some(vec!["database".to_string()]),
            ..Default::default()
        })
        .build();

    // Deploy the stack
    runtime.config().save_boltfile(&boltfile)?;
    runtime.surge_up(&[], false, false).await?;

    // Monitor status
    let status = runtime.surge_status().await?;
    println!("Deployed {} services", status.services.len());

    Ok(())
}
```

## Advanced Usage

### Custom Error Handling

```rust
use bolt::{BoltError, error::*};

pub fn handle_bolt_error(err: BoltError) {
    match err {
        BoltError::Runtime(RuntimeError::ContainerNotFound { name }) => {
            println!("Container '{}' not found, creating it...", name);
        },
        BoltError::Gaming(GamingError::GpuNotFound) => {
            println!("No GPU detected, falling back to software rendering");
        },
        BoltError::Network(NetworkError::QuicSetupFailed { reason }) => {
            println!("QUIC networking failed: {}, using standard networking", reason);
        },
        _ => {
            eprintln!("Unexpected error: {}", err);
        }
    }
}
```

### Performance Tuning

```rust
// Enable maximum performance for gaming
let performance_config = PerformanceConfig {
    cpu_governor: Some("performance".to_string()),
    nice_level: Some(-20),           // Highest priority
    rt_priority: Some(99),           // Real-time scheduling
};

// Low-latency audio setup
let audio_config = AudioConfig {
    system: "pipewire".to_string(),
    latency: Some("ultra-low".to_string()),
};
```

This completes the comprehensive Bolt Rust API documentation. The API is production-ready and optimized for gaming workloads, container orchestration, and high-performance networking.