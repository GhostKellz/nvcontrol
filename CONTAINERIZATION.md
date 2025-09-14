# nvcontrol Containerization & GPU Passthrough

Complete guide to nvcontrol's built-in container GPU passthrough capabilities, supporting Docker, Podman, NixOS, and PhantomLink integration.

## 🚀 Overview

nvcontrol includes a **pure Rust container runtime** that provides comprehensive GPU passthrough capabilities for containerized workloads. No external dependencies - everything is built directly into nvctl.

### Key Features

- **Multi-Runtime Support**: Docker, Podman, containerd, NixOS containers
- **PhantomLink Integration**: Audio processing container with RTX Voice
- **GPU Profiles**: Optimized configurations for ML, gaming, inference
- **Pure Rust Implementation**: No nvidia-docker dependency
- **Wayland Native**: Works seamlessly with modern Linux desktops

### Supported Container Runtimes

| Runtime | Support Level | GPU Passthrough Method | Status |
|---------|---------------|----------------------|---------|
| **Docker** | ✅ Full | `--gpus` flag + device requests | Production Ready |
| **Podman** | ✅ Full | `--device` + environment variables | Production Ready |
| **NixOS** | ✅ Full | Declarative configuration | Beta |
| **containerd** | ⚠️ Basic | CRI-O integration | Development |

## 📋 Quick Start

### Prerequisites

```bash
# Ensure NVIDIA drivers and container runtime
nvidia-smi                          # Verify NVIDIA driver
docker --version                    # Verify Docker installation

# Test GPU access
nvctl ct runtime test              # Test GPU passthrough
nvctl ct runtime info              # Runtime information
```

### Basic Container Launch

```bash
# Launch CUDA container with GPU access
nvctl ct launch --image nvidia/cuda:12.0-runtime-ubuntu20.04 --gpu all -i

# Launch ML training container
nvctl ct launch \
  --image tensorflow/tensorflow:latest-gpu \
  --name ml-training \
  --gpu all \
  --interactive

# Launch with specific GPU
nvctl ct launch --image pytorch/pytorch:latest --gpu "0,1" --name pytorch-dev
```

## 🐳 Container Runtime Commands

### Container Launch & Management

#### Basic Launch
```bash
# Simple GPU container launch
nvctl ct launch --image <image>                    # Basic launch
nvctl ct launch --image <image> --gpu all         # All GPUs
nvctl ct launch --image <image> --gpu "0,1"       # Specific GPUs
```

#### Advanced Launch Options
```bash
nvctl ct launch \
  --image nvidia/cuda:12.0-devel \
  --name cuda-dev \
  --gpu all \
  --interactive \
  --rm \
  --runtime podman
```

**Parameters:**
- `--image`: Container image to run
- `--name`: Container name (optional)
- `--gpu`: GPU devices (`all`, `0`, `1,2`, `GPU-uuid`)
- `--interactive`: Interactive mode (`-i`)
- `--rm`: Remove container on exit
- `--runtime`: Container runtime (`docker`, `podman`, `nix`)

#### Container Management
```bash
nvctl ct list                       # List GPU-enabled containers
nvctl ct status                     # All container GPU status
nvctl ct status --container ml-training  # Specific container
nvctl ct monitor --container ml-training --interval 5  # Monitor
```

### PhantomLink Audio Container

PhantomLink is a Rust-based Wavelink XLR alternative that leverages GPU acceleration for audio processing.

#### Basic PhantomLink Launch
```bash
# Launch PhantomLink with defaults
nvctl ct phantomlink

# Production mode with RTX Voice
nvctl ct phantomlink --rtx-voice --mode prod

# Development mode
nvctl ct phantomlink --mode dev --audio-device hw:0
```

#### PhantomLink Configuration

**Launch Modes:**
- `prod` (default): Production mode with optimized settings
- `dev`: Development mode with debug logging
- `minimal`: Minimal resource usage

**Options:**
- `--rtx-voice`: Enable NVIDIA RTX Voice noise suppression
- `--audio-device`: Specify audio device (e.g., `hw:0`)
- `--mode`: Launch mode configuration

**PhantomLink Features:**
- **RTX Voice Integration**: GPU-accelerated noise suppression
- **Low Latency Audio**: Optimized for real-time processing
- **Container Isolation**: Secure audio processing environment
- **Web Interface**: Management UI at `http://localhost:8080`

#### Example PhantomLink Configurations
```bash
# Streaming setup with noise suppression
nvctl ct phantomlink \
  --rtx-voice \
  --mode prod \
  --audio-device hw:1

# Development setup
nvctl ct phantomlink \
  --mode dev \
  --audio-device pulse
```

## 📊 GPU Profiles & Management

### Container GPU Profiles

nvcontrol includes optimized GPU profiles for different workload types.

#### List Available Profiles
```bash
nvctl ct profiles list
```

**Built-in Profiles:**
- **ML Training**: High power, exclusive GPU access, persistence enabled
- **ML Inference**: Balanced power, memory limits, fast startup
- **Gaming**: Optimized for gaming containers, balanced settings
- **Default**: Standard configuration for general use

#### Create Custom Profile
```bash
# Create ML training profile
nvctl ct profiles create \
  --name "Custom ML Training" \
  --workload ml-training

# Create gaming profile
nvctl ct profiles create \
  --name "Game Container" \
  --workload gaming
```

#### Apply Profile to Container
```bash
nvctl ct profiles apply \
  --profile "ML Training" \
  --container ml-training-container
```

### Profile Specifications

#### ML Training Profile
```toml
name = "ML Training"
description = "Optimized for ML training workloads"
power_limit = 300W
compute_mode = "ExclusiveProcess"
persistence_mode = true
auto_boost = true
memory_limit = null  # No limit
```

#### ML Inference Profile
```toml
name = "ML Inference"
description = "Optimized for ML inference"
power_limit = 200W
compute_mode = "Default"
persistence_mode = true
auto_boost = false
memory_limit = "8GB"
```

#### Gaming Profile
```toml
name = "Gaming"
description = "Optimized for gaming containers"
power_limit = 250W
compute_mode = "Default"
persistence_mode = false
auto_boost = true
memory_limit = null
```

## 🔧 Runtime Configuration

### Runtime Information
```bash
nvctl ct runtime info              # Show runtime information
nvctl ct runtime test              # Test GPU passthrough
nvctl ct runtime configure         # Configure NVIDIA runtime
```

### Docker Configuration

#### Automatic Setup
```bash
nvctl ct runtime setup --runtime docker
```

#### Manual Docker Configuration
```json
# /etc/docker/daemon.json
{
  "runtimes": {
    "nvidia": {
      "path": "nvidia-container-runtime",
      "runtimeArgs": []
    }
  },
  "default-runtime": "nvidia"
}
```

### Podman Configuration

#### Automatic Setup
```bash
nvctl ct runtime setup --runtime podman
```

#### Manual Podman Configuration
Podman uses device passthrough for GPU access:
```bash
# GPU device access
podman run --device /dev/nvidia0 \
           --device /dev/nvidiactl \
           --device /dev/nvidia-modeset \
           --device /dev/nvidia-uvm \
           nvidia/cuda:12.0-runtime
```

### NixOS Configuration

#### Declarative Container Configuration
```bash
nvctl ct runtime setup --runtime nix
```

Generated NixOS configuration:
```nix
{ config, pkgs, ... }:
{
  # NVIDIA GPU support
  hardware.opengl.enable = true;
  services.xserver.videoDrivers = [ "nvidia" ];

  hardware.nvidia = {
    modesetting.enable = true;
    open = true;  # Use nvidia-open drivers
    nvidiaSettings = true;
  };

  # Container support
  virtualisation.docker = {
    enable = true;
    enableNvidia = true;
  };
}
```

## 🎯 Use Cases & Examples

### Machine Learning Development

#### PyTorch Development Container
```bash
# Launch PyTorch development environment
nvctl ct launch \
  --image pytorch/pytorch:2.0.1-cuda11.7-cudnn8-devel \
  --name pytorch-dev \
  --gpu all \
  --interactive \
  --runtime docker

# Apply ML training profile
nvctl ct profiles apply --profile "ML Training" --container pytorch-dev

# Monitor training progress
nvctl ct monitor --container pytorch-dev --interval 2
```

#### TensorFlow Training
```bash
# TensorFlow with Jupyter
nvctl ct launch \
  --image tensorflow/tensorflow:latest-gpu-jupyter \
  --name tf-training \
  --gpu "0,1" \
  --ports 8888:8888

# Custom environment variables
NVIDIA_VISIBLE_DEVICES=all nvctl ct launch \
  --image custom/ml-training:latest \
  --gpu all
```

### Gaming & Entertainment

#### Gaming Container
```bash
# Steam in container with GPU
nvctl ct launch \
  --image games/steam-nvidia:latest \
  --name gaming \
  --gpu all \
  --interactive \
  --volumes /home/user/games:/games

# Apply gaming profile
nvctl ct profiles apply --profile "Gaming" --container gaming
```

#### PhantomLink Audio Setup
```bash
# Streaming setup with noise suppression
nvctl ct phantomlink \
  --rtx-voice \
  --audio-device hw:1 \
  --mode prod

# Monitor container
nvctl ct monitor --container phantomlink-audio
```

### Development & Testing

#### CUDA Development
```bash
# CUDA development environment
nvctl ct launch \
  --image nvidia/cuda:12.0-devel \
  --name cuda-dev \
  --gpu all \
  --interactive \
  --volumes $PWD:/workspace \
  --workdir /workspace
```

#### Multi-GPU Testing
```bash
# Test with specific GPUs
nvctl ct launch \
  --image nvidia/cuda:12.0-runtime \
  --gpu "0" \
  --name gpu0-test

nvctl ct launch \
  --image nvidia/cuda:12.0-runtime \
  --gpu "1" \
  --name gpu1-test

# Monitor both containers
nvctl ct list
nvctl gpu stat  # Overall GPU monitoring
```

## 🐳 Container Runtime Deep Dive

### Docker Integration

#### GPU Device Requests
nvcontrol automatically configures Docker GPU access:

```rust
// Device request configuration
DeviceRequest {
    driver: "nvidia".to_string(),
    count: Some(1),
    device_ids: vec!["0".to_string()],
    capabilities: vec![vec!["gpu".to_string()]],
    options: HashMap::new(),
}
```

#### Environment Variables
```bash
NVIDIA_VISIBLE_DEVICES=all
NVIDIA_DRIVER_CAPABILITIES=compute,utility
```

### Podman Integration

#### Device Passthrough
```bash
# Automatic device detection and passthrough
/dev/nvidia0           # GPU 0
/dev/nvidia1           # GPU 1 (if available)
/dev/nvidiactl         # Control device
/dev/nvidia-modeset    # Display subsystem
/dev/nvidia-uvm        # Unified memory
/dev/nvidia-uvm-tools  # UVM tools
```

### NixOS Integration

#### Container Configuration Generation
```bash
# Generate NixOS container config
nvctl ct runtime configure --runtime nix --gpu-support

# Apply configuration
nixos-container create --config generated-config.nix gpu-container
nixos-container start gpu-container
```

## 🔍 Troubleshooting

### Common Issues

#### GPU Not Detected in Container
```bash
# Test GPU passthrough
nvctl ct runtime test

# Check runtime configuration
nvctl ct runtime info

# Verify NVIDIA devices
ls -la /dev/nvidia*

# Test with simple container
nvctl ct launch --image nvidia/cuda:12.0-runtime --gpu all -i --rm
```

#### Container Launch Failures
```bash
# Check runtime logs
docker logs <container_name>
podman logs <container_name>

# Verify GPU access
nvidia-smi
nvctl gpu info

# Test with minimal container
nvctl ct launch --image hello-world --rm
```

#### PhantomLink Audio Issues
```bash
# Check audio devices
aplay -l
pactl list sources

# Test audio container
nvctl ct phantomlink --mode minimal

# Check container logs
docker logs phantomlink-audio
```

### Debug Commands

#### Verbose Logging
```bash
# Enable debug logging
export RUST_LOG=debug
nvctl ct launch --image nvidia/cuda:12.0-runtime --gpu all

# Container-specific debugging
nvctl ct status --container <name> --verbose
```

#### GPU Access Verification
```bash
# In container - verify GPU access
nvidia-smi
nvcc --version
python -c "import torch; print(torch.cuda.is_available())"
```

## 🚀 Advanced Configuration

### Custom Container Profiles

#### Create Advanced Profile
```bash
# Create profile with specific settings
nvctl ct profiles create \
  --name "High Performance ML" \
  --workload custom \
  --power-limit 350 \
  --memory-limit 16GB \
  --compute-mode exclusive
```

#### Profile Configuration File
```toml
[profile]
name = "Custom Profile"
description = "Custom workload profile"

[gpu]
power_limit = 300
memory_limit = "8GB"
compute_mode = "ExclusiveProcess"
persistence_mode = true
auto_boost = true

[container]
driver_capabilities = ["compute", "utility", "graphics"]
environment_vars = { "NVIDIA_VISIBLE_DEVICES" = "all" }

[monitoring]
enable_metrics = true
metric_interval = 5
```

### Container Network Configuration

#### PhantomLink Networking
```bash
# Custom port mapping
nvctl ct phantomlink \
  --port 8080:8080 \
  --port 8443:8443 \
  --network bridge
```

#### Multi-Container Setup
```bash
# ML training with monitoring
nvctl ct launch --image training:latest --name ml-train --gpu "0,1"
nvctl ct launch --image monitoring:latest --name ml-monitor --link ml-train
```

## 📈 Performance Optimization

### GPU Memory Management
```bash
# Limit container GPU memory
nvctl ct launch \
  --image tensorflow/tensorflow:latest-gpu \
  --gpu-memory 8GB \
  --gpu all
```

### Container Resource Limits
```bash
# CPU and memory limits
nvctl ct launch \
  --image nvidia/cuda:12.0-runtime \
  --cpus 4.0 \
  --memory 16GB \
  --gpu all
```

### Monitoring & Metrics
```bash
# Real-time container monitoring
nvctl ct monitor --container ml-training --interval 1

# Export container metrics
nvctl ct metrics --container ml-training --format json
```

---

## 🤝 Contributing

Contributions to nvcontrol's containerization features are welcome! Areas of interest:

- Additional container runtime support
- Enhanced PhantomLink features
- Performance optimizations
- Documentation improvements

## 📄 License

nvcontrol containerization features are licensed under the MIT License.

---

**nvcontrol - Container GPU Passthrough Made Simple**

*Built for NVIDIA Open Drivers • Docker & Podman Ready • PhantomLink Integrated*