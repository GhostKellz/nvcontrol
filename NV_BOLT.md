# NV_BOLT: NVIDIA GPU Container Management with BOLT

nvcontrol integrates with [BOLT](https://github.com/CK-Technology/bolt) to provide enterprise-grade GPU container management, gaming optimizations, and high-performance networking for NVIDIA systems.

## Overview

The `nvctl bolt` command suite provides comprehensive container runtime operations specifically optimized for NVIDIA GPU workloads, gaming, and machine learning applications.

## Features

### 🚀 GPU Container Management
- **GPU Passthrough**: Full NVIDIA GPU access in containers
- **DLSS Support**: Enable NVIDIA DLSS acceleration
- **Ray Tracing**: Real-time ray tracing capabilities
- **Memory Management**: GPU memory limits and optimization
- **Power Control**: GPU power limit configuration

### 🎮 Gaming Integration
- **Proton Support**: Windows game compatibility via Proton
- **Steam Integration**: Direct Steam game launching
- **Gaming Profiles**: Optimized configurations for gaming workloads
- **Low Latency**: Ultra-low latency networking and audio

### ⚡ High-Performance Infrastructure
- **QUIC Networking**: Encrypted high-performance networking
- **Surge Orchestration**: Multi-service container orchestration
- **Image Building**: GPU-optimized container image building
- **Storage Integration**: S3 and GhostBay storage backends

## Installation

BOLT integration is enabled by default in nvcontrol. Ensure you have:

```bash
# BOLT runtime installed
curl -sSL https://get.bolt.sh | sh

# NVIDIA Container Toolkit
distribution=$(. /etc/os-release;echo $ID$VERSION_ID)
curl -s -L https://nvidia.github.io/nvidia-docker/gpgkey | sudo apt-key add -
curl -s -L https://nvidia.github.io/nvidia-docker/$distribution/nvidia-docker.list | sudo tee /etc/apt/sources.list.d/nvidia-docker.list
sudo apt-get update && sudo apt-get install -y nvidia-container-toolkit
```

## Command Reference

### Container Lifecycle

#### Launch GPU Workloads
```bash
# Basic GPU container
nvctl bolt launch --name ml-training --image tensorflow/tensorflow:latest-gpu

# Gaming workload with DLSS and ray tracing
nvctl bolt launch --name gaming-rig --image nvidia/vulkan:latest \
  --gpu-id 0 --dlss --raytracing --memory-limit 8

# High-performance compute workload
nvctl bolt launch --name hpc-job --image nvidia/cuda:12.0-devel \
  --gpu-id 0,1 --memory-limit 16 --power-limit 300
```

#### List Containers
```bash
# List all GPU containers
nvctl bolt list

# With verbose output
nvctl bolt list --verbose
```

#### Container Control
```bash
# Stop container
nvctl bolt stop --container ml-training

# Remove container (with force)
nvctl bolt remove --container ml-training --force
```

### Gaming Operations

#### Setup Gaming Environment
```bash
# Configure gaming environment
nvctl bolt gaming --name "Cyberpunk 2077" --proton 8.0 --winver win10

# Custom Proton version
nvctl bolt gaming --name "Elden Ring" --proton 7.0 --winver win11
```

#### Launch Games
```bash
# Launch Steam game by App ID
nvctl bolt game --steam-id 1091500 --args "--fullscreen --high-quality"

# Launch game with custom arguments
nvctl bolt game --steam-id 292030 --args "--windowed --dev-mode"
```

### Service Orchestration

#### Surge Management
```bash
# Start all services from Boltfile.toml
nvctl bolt up

# Start specific services
nvctl bolt up gaming-proton gpu-monitor

# Force recreate containers
nvctl bolt up --force-recreate

# Stop all services
nvctl bolt down

# Stop with volume cleanup
nvctl bolt down --volumes
```

#### Status Monitoring
```bash
# Show Surge orchestration status
nvctl bolt status

# JSON output for automation
nvctl bolt status --format json
```

### Infrastructure Operations

#### Image Building
```bash
# Build GPU-optimized image
nvctl bolt build --dockerfile ./containers/ml-training/Dockerfile --tag nvcontrol/ml:latest

# Build with custom context
nvctl bolt build --dockerfile ./gaming/Dockerfile.bolt --tag nvcontrol/gaming:v1.0
```

#### Network Management
```bash
# Create high-performance GPU network
nvctl bolt network --name gpu-cluster --subnet 10.100.0.0/16

# Create with QUIC encryption
nvctl bolt network --name gaming-net --subnet 192.168.100.0/24
```

#### Monitoring Services
```bash
# Create GPU monitoring service
nvctl bolt monitor --web --port 8080

# Access monitoring dashboard at http://localhost:8080
```

## Configuration

### Boltfile.toml

nvcontrol includes a comprehensive Boltfile.toml with pre-configured services:

```toml
# GPU Management Service
[services.gpu-manager]
build = "./containers/gpu-manager"
privileged = true
gpu.nvidia = true
env.NVIDIA_VISIBLE_DEVICES = "all"
ports = ["3000:3000"]

# Gaming Container with Proton
[services.gaming-proton]
build = "./containers/gaming"
gpu.nvidia = true
env.PROTON_VERSION = "8.0"
env.NVIDIA_DRIVER_CAPABILITIES = "all"

# ML Training Service
[services.ml-training]
build = "./containers/ml-training"
gpu.nvidia = true
env.CUDA_VISIBLE_DEVICES = "0,1"
volumes = ["./data:/workspace/data"]
ports = ["8888:8888", "6006:6006"]
```

### GPU Configuration Profiles

nvcontrol provides pre-configured GPU profiles:

#### ML Training Profile
```rust
GpuContainerConfig {
    gpu_id: 0,
    memory_limit: Some(8_000_000_000), // 8GB
    compute_capabilities: ["compute", "utility"],
    power_limit: Some(100),
    enable_cuda: true,
    enable_dlss: false,
    enable_raytracing: false,
}
```

#### Gaming Profile
```rust
GpuContainerConfig {
    gpu_id: 0,
    memory_limit: None, // Unlimited
    compute_capabilities: ["compute", "utility", "graphics"],
    power_limit: Some(120),
    enable_cuda: true,
    enable_dlss: true,
    enable_raytracing: true,
}
```

#### Inference Profile
```rust
GpuContainerConfig {
    gpu_id: 0,
    memory_limit: Some(4_000_000_000), // 4GB
    compute_capabilities: ["compute"],
    power_limit: Some(80),
    enable_cuda: true,
    enable_dlss: false,
    enable_raytracing: false,
}
```

## Advanced Usage

### Custom GPU Workloads

#### Machine Learning Training
```bash
# TensorFlow with GPU acceleration
nvctl bolt launch --name tensorflow-gpu \
  --image tensorflow/tensorflow:latest-gpu \
  --gpu-id 0 --memory-limit 12 \
  --power-limit 250

# PyTorch distributed training
nvctl bolt launch --name pytorch-distributed \
  --image pytorch/pytorch:latest \
  --gpu-id 0,1,2,3 --memory-limit 32 \
  --power-limit 400
```

#### High-Performance Computing
```bash
# CUDA development environment
nvctl bolt launch --name cuda-dev \
  --image nvidia/cuda:12.0-devel \
  --gpu-id 0 --memory-limit 16

# Scientific computing with GPU acceleration
nvctl bolt launch --name scientific-compute \
  --image nvidia/numba:latest \
  --gpu-id 0,1 --memory-limit 24 \
  --power-limit 350
```

### Gaming Workloads

#### Steam Games with Proton
```bash
# Setup gaming environment
nvctl bolt gaming --name "Steam Gaming" --proton 8.0

# Launch specific games
nvctl bolt game --steam-id 1091500    # Cyberpunk 2077
nvctl bolt game --steam-id 292030     # The Witcher 3
nvctl bolt game --steam-id 1174180    # Red Dead Redemption 2
```

#### Custom Gaming Containers
```bash
# Gaming with DLSS and ray tracing
nvctl bolt launch --name custom-gaming \
  --image ghcr.io/games/custom:latest \
  --gpu-id 0 --dlss --raytracing \
  --memory-limit 16 --power-limit 300
```

### Development Workflows

#### Container Development
```bash
# Build development image
nvctl bolt build --dockerfile ./dev/Dockerfile.bolt --tag nvcontrol/dev:latest

# Launch development environment
nvctl bolt launch --name dev-env \
  --image nvcontrol/dev:latest \
  --gpu-id 0 --memory-limit 8
```

#### Testing and Benchmarking
```bash
# Launch GPU benchmark container
nvctl bolt launch --name gpu-benchmark \
  --image nvidia/cuda:12.0-base \
  --gpu-id 0 --memory-limit 4

# Monitor GPU usage during testing
nvctl bolt monitor --web --port 9090
```

## Integration with nvcontrol

### Native Vibrance Control

BOLT containers automatically inherit nvcontrol's native vibrance capabilities:

```bash
# Set vibrance for all displays in BOLT containers
nvctl display vibrance set 75

# Apply gaming vibrance profile
nvctl display vibrance gaming
```

### Profile Integration

nvcontrol profiles automatically configure BOLT containers:

```bash
# Apply gaming profile (includes BOLT gaming container setup)
nvctl profile apply gaming

# Custom profile with BOLT integration
nvctl profile create custom-ml --bolt-config ml-training
```

## Monitoring and Observability

### GPU Metrics
```bash
# Real-time GPU monitoring in containers
nvctl bolt monitor --web

# Export metrics to Prometheus
nvctl bolt monitor --prometheus --port 9090
```

### Container Logs
```bash
# Stream container logs
nvctl bolt logs --container ml-training --follow

# Export logs for analysis
nvctl bolt logs --container gpu-manager --export /tmp/logs.json
```

### Performance Monitoring
```bash
# Container performance stats
nvctl bolt stats --container gaming-proton

# Network performance metrics
nvctl bolt network stats --name gpu-cluster
```

## Troubleshooting

### Common Issues

#### GPU Not Detected
```bash
# Verify GPU availability
nvidia-smi

# Check NVIDIA container runtime
docker run --rm --gpus all nvidia/cuda:11.0-base nvidia-smi

# Restart BOLT runtime
sudo systemctl restart bolt
```

#### Container Start Failures
```bash
# Check container logs
nvctl bolt logs --container failed-container

# Verify image availability
docker images | grep nvidia

# Check resource constraints
nvctl bolt status --verbose
```

#### Gaming Performance Issues
```bash
# Verify Proton installation
nvctl bolt gaming --name test --proton 8.0

# Check graphics driver
nvidia-settings

# Monitor GPU usage
nvctl bolt monitor --web
```

### Debug Mode
```bash
# Enable verbose logging
nvctl bolt --verbose status

# Debug container launch
nvctl bolt launch --name debug-test --image nvidia/cuda:12.0-base --verbose

# Check BOLT runtime status
bolt status --debug
```

## API Integration

### Programmatic Access

nvcontrol's BOLT integration can be used programmatically:

```rust
use nvcontrol::bolt_integration::{NvControlBoltManager, GpuContainerConfig};

// Initialize BOLT manager
let manager = NvControlBoltManager::new().await?;

// Launch GPU workload
let config = GpuContainerConfig {
    gpu_id: 0,
    enable_dlss: true,
    enable_raytracing: true,
    memory_limit: Some(8_000_000_000),
    ..Default::default()
};

let container_name = manager.launch_gpu_workload(
    "ml-training",
    "tensorflow/tensorflow:latest-gpu",
    &config
).await?;
```

### Configuration Management

```rust
// Create custom GPU profiles
let gaming_profile = create_gaming_profile();
let ml_profile = create_ml_training_profile();
let inference_profile = create_inference_profile();

// Apply profiles programmatically
manager.launch_gpu_workload("gaming", "custom:latest", &gaming_profile).await?;
```

## Security Considerations

### Container Security
- BOLT containers run with minimal privileges by default
- GPU device access is strictly controlled
- Network isolation via QUIC encryption
- Volume mounts are read-only where possible

### Resource Isolation
- GPU memory limits prevent resource exhaustion
- Power limits protect hardware from overutilization
- Network quotas prevent bandwidth abuse
- CPU/memory cgroups enforce container limits

## Performance Optimization

### GPU Performance
- Enable GPU memory pools for reduced allocation overhead
- Use tensor memory optimization for ML workloads
- Configure GPU boost clocks for maximum performance
- Optimize memory transfer patterns

### Network Performance
- QUIC protocol reduces latency vs TCP
- Container-to-container networking bypasses host stack
- GPU Direct RDMA for high-bandwidth applications
- Network buffer tuning for sustained throughput

### Storage Performance
- Local NVMe storage for temporary data
- S3 integration for persistent datasets
- GhostBay CDN for image distribution
- Container layer caching for fast startup

## Examples Repository

Complete examples are available in the nvcontrol repository:

- `examples/bolt/ml-training/` - Machine learning training setup
- `examples/bolt/gaming/` - Gaming environment configuration
- `examples/bolt/hpc/` - High-performance computing examples
- `examples/bolt/monitoring/` - Monitoring and observability setup

## Contributing

To contribute to nvcontrol's BOLT integration:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

For BOLT-specific issues, please report to:
- nvcontrol issues: https://github.com/your-org/nvcontrol/issues
- BOLT issues: https://github.com/CK-Technology/bolt/issues

## License

nvcontrol's BOLT integration is released under the same license as nvcontrol. BOLT is licensed separately under its own terms.