# NVControl Integration with Bolt

Integration guide for [NVControl](https://github.com/ghostkellz/nvcontrol) - A modern NVIDIA control panel with containerized CLI management using Bolt runtime.

## Overview

NVControl is a next-generation NVIDIA control panel that leverages Bolt's container runtime for isolated GPU management operations. The `nvctl ct` (container) command uses Bolt instead of Docker/Podman for superior performance and GPU resource management.

## Architecture

```
nvcontrol
├── GUI Application (Rust/Tauri)
├── nvctl CLI (with ct subcommand)
└── Bolt Integration
    ├── GPU Capsules
    ├── Driver Containers
    └── Monitoring Services
```

## Integration Setup

### 1. NVControl Boltfile Configuration

Create `Boltfile.toml` in your nvcontrol project:

```toml
project = "nvcontrol"

# Main GPU management service
[services.gpu-manager]
build = "./containers/gpu-manager"
privileged = true
gpu.nvidia = true
devices = [
    "/dev/nvidia0",
    "/dev/nvidia-uvm",
    "/dev/nvidia-modeset",
    "/dev/nvidiactl"
]
volumes = [
    "/usr/lib/x86_64-linux-gnu/libnvidia-ml.so:/usr/lib/x86_64-linux-gnu/libnvidia-ml.so:ro",
    "/sys/class/drm:/sys/class/drm:ro"
]
env.NVIDIA_VISIBLE_DEVICES = "all"
env.NVIDIA_DRIVER_CAPABILITIES = "all"

# GPU monitoring service
[services.gpu-monitor]
build = "./containers/monitor"
gpu.nvidia = true
volumes = ["/proc:/host/proc:ro"]
env.HOST_PROC = "/host/proc"

# Driver update service
[services.driver-updater]
build = "./containers/driver-updater"
privileged = true
volumes = [
    "/lib/modules:/lib/modules:ro",
    "/usr/src:/usr/src:ro"
]

# Web interface for remote management
[services.web-interface]
build = "./web"
ports = ["8080:8080"]
env.API_ENDPOINT = "bolt://gpu-manager:3000"

[network]
driver = "quic"
encryption = true
```

### 2. Container Definitions

#### GPU Manager Container (`containers/gpu-manager/Dockerfile.bolt`)

```dockerfile
FROM bolt://ubuntu:22.04

RUN apt-get update && apt-get install -y \
    nvidia-utils-535 \
    nvidia-settings \
    python3 \
    python3-pip

COPY gpu_manager.py /app/
WORKDIR /app

CMD ["python3", "gpu_manager.py"]
```

#### Monitor Container (`containers/monitor/Dockerfile.bolt`)

```dockerfile
FROM bolt://alpine:latest

RUN apk add --no-cache nvidia-ml-py3 python3

COPY monitor.py /app/
WORKDIR /app

CMD ["python3", "monitor.py"]
```

### 3. nvctl CLI Integration

Modify your `nvctl` CLI to use Bolt:

```rust
// src/commands/container.rs
use bolt::runtime::oci::Container;
use bolt::surge::Orchestrator;

pub async fn container_command(args: ContainerArgs) -> anyhow::Result<()> {
    match args.subcommand {
        ContainerSubcommand::Start => start_gpu_services().await,
        ContainerSubcommand::Stop => stop_gpu_services().await,
        ContainerSubcommand::Status => check_gpu_status().await,
        ContainerSubcommand::Update => update_drivers().await,
    }
}

async fn start_gpu_services() -> anyhow::Result<()> {
    let boltfile = bolt::config::Boltfile::from_path("Boltfile.toml")?;
    let orchestrator = Orchestrator::new(boltfile);

    // Start GPU management stack
    orchestrator.surge_up().await?;

    println!("✅ GPU management services started");
    Ok(())
}

async fn update_drivers() -> anyhow::Result<()> {
    // Use Bolt's capsule system for driver updates
    let container = Container::new("driver-updater")?;
    container.start().await?;

    // Wait for completion and check results
    let exit_code = container.wait().await?;
    if exit_code == 0 {
        println!("✅ Driver update completed successfully");
    } else {
        anyhow::bail!("❌ Driver update failed");
    }

    Ok(())
}
```

### 4. GPU Resource Management

Use Bolt's GPU abstraction layer:

```rust
// src/gpu/management.rs
use bolt::runtime::gpu::nvidia::NvidiaGpu;
use bolt::capsules::snapshots::Snapshot;

pub struct GpuManager {
    bolt_orchestrator: Orchestrator,
}

impl GpuManager {
    pub async fn new() -> anyhow::Result<Self> {
        let boltfile = Boltfile::from_path("Boltfile.toml")?;
        let orchestrator = Orchestrator::new(boltfile);

        Ok(Self {
            bolt_orchestrator: orchestrator,
        })
    }

    pub async fn overclock_gpu(&self, gpu_id: u32, memory: u32, core: u32) -> anyhow::Result<()> {
        // Create snapshot before overclocking
        let snapshot = Snapshot::create("pre-overclock").await?;

        // Execute overclocking in isolated container
        let result = self.bolt_orchestrator
            .execute_in_service("gpu-manager", &format!(
                "nvidia-settings -a '[gpu:{}]/GPUMemoryTransferRateOffset[3]={}'",
                gpu_id, memory
            ))
            .await;

        match result {
            Ok(_) => {
                println!("✅ Overclocking applied successfully");
                Ok(())
            }
            Err(e) => {
                // Restore snapshot on failure
                snapshot.restore().await?;
                Err(e)
            }
        }
    }
}
```

## nvctl ct Command Usage

```bash
# Start GPU management services
nvctl ct start

# Check GPU service status
nvctl ct status

# Update NVIDIA drivers in isolated environment
nvctl ct update

# Stop all GPU services
nvctl ct stop

# Monitor GPU performance
nvctl ct monitor

# Create GPU configuration snapshot
nvctl ct snapshot create stable-config

# Restore GPU configuration
nvctl ct snapshot restore stable-config
```

## Advanced Features

### 1. Multi-GPU Support

```toml
[services.gpu-manager-0]
build = "./containers/gpu-manager"
gpu.nvidia = true
env.CUDA_VISIBLE_DEVICES = "0"

[services.gpu-manager-1]
build = "./containers/gpu-manager"
gpu.nvidia = true
env.CUDA_VISIBLE_DEVICES = "1"
```

### 2. Driver Testing Environment

```toml
[services.driver-test]
build = "./containers/driver-test"
gpu.nvidia = true
volumes = ["./test-drivers:/drivers"]
env.NVIDIA_DRIVER_VERSION = "535.154.05"
```

### 3. Remote GPU Management

```rust
use bolt::network::quic::QuicClient;

pub async fn remote_gpu_control(endpoint: &str) -> anyhow::Result<()> {
    let client = QuicClient::connect(endpoint).await?;

    // Send GPU commands over QUIC
    client.send_command("overclock", &gpu_params).await?;

    Ok(())
}
```

## Security Considerations

1. **Privileged Containers**: GPU management requires privileged access - Bolt provides safer isolation
2. **Driver Isolation**: Use Bolt capsules to isolate driver updates from host system
3. **Snapshot Recovery**: Always create snapshots before risky operations
4. **QUIC Encryption**: Enable encrypted communication for remote management

## Troubleshooting

- **GPU Access Denied**: Ensure proper device permissions in Boltfile
- **Driver Conflicts**: Use Bolt's isolation to prevent host contamination
- **Performance Issues**: Check GPU resource allocation in container specs
- **Network Issues**: Verify QUIC transport configuration

## Migration from Docker

Replace existing Docker commands:

```bash
# Old Docker approach
docker run --gpus all --privileged nvidia/cuda:latest nvidia-smi

# New Bolt approach
bolt surge run gpu-manager -- nvidia-smi
```

## Benefits of Bolt Integration

1. **Better GPU Resource Management**: Native GPU abstraction
2. **Snapshot/Restore**: Safe driver testing and configuration changes
3. **QUIC Networking**: Secure remote GPU management
4. **Lightweight Containers**: Better performance than Docker
5. **Declarative Configuration**: Version-controlled GPU setups