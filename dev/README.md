# nvcontrol Docker Testing Environment

This directory contains Docker-based testing infrastructure for nvcontrol on Arch Linux with NVIDIA GPU support.

## Prerequisites

1. **NVIDIA Container Toolkit** (required for GPU access in containers)
   ```bash
   # Install nvidia-container-toolkit
   sudo pacman -S nvidia-container-toolkit

   # Configure Docker to use NVIDIA runtime
   sudo nvidia-ctk runtime configure --runtime=docker
   sudo systemctl restart docker
   ```

2. **Docker Compose v2**
   ```bash
   sudo pacman -S docker-compose
   ```

3. **User Permissions**
   ```bash
   sudo usermod -aG docker $USER
   # Log out and back in for group changes to take effect
   ```

## Quick Start

From the project root:

```bash
# Build all images
cd dev
docker-compose build

# Run all tests
docker-compose up nvcontrol-test

# Run specific test suite
docker-compose up nvcontrol-unit-tests        # Unit tests (no GPU)
docker-compose up nvcontrol-integration-tests  # Integration tests (GPU required)

# Interactive development shell with GPU access
docker-compose run --rm nvcontrol-dev
```

## Available Services

### 1. **nvcontrol-test** - Main Testing Environment
Full test suite with GPU access, Wayland/X11 support, and comprehensive logging.

```bash
docker-compose up nvcontrol-test
```

Features:
- Full GPU access via NVIDIA runtime
- X11 and Wayland display server support
- All NVIDIA devices mounted
- Trace-level logging (`RUST_LOG=debug,nvcontrol=trace`)

### 2. **nvcontrol-unit-tests** - Unit Tests
Runs library unit tests without GPU requirements.

```bash
docker-compose up nvcontrol-unit-tests
```

Fast unit tests for business logic and non-GPU functionality.

### 3. **nvcontrol-integration-tests** - Integration Tests
GPU-dependent integration tests with full backtrace.

```bash
docker-compose up nvcontrol-integration-tests
```

Tests GPU control, NVML operations, fan control, overclocking, etc.

### 4. **nvcontrol-bench** - Benchmarks
Run performance benchmarks.

```bash
docker-compose up nvcontrol-bench
```

Results saved to `../bench-results/` on the host.

### 5. **nvcontrol-coverage** - Code Coverage
Generate code coverage reports with `cargo-tarpaulin`.

```bash
docker-compose up nvcontrol-coverage
```

HTML coverage report saved to `../coverage/` on the host.

### 6. **nvcontrol-lint** - Clippy Linter
Run Clippy with all warnings as errors.

```bash
docker-compose up nvcontrol-lint
```

Ensures code quality and catches common mistakes.

### 7. **nvcontrol-dev** - Interactive Development Shell
Interactive bash shell with full GPU access for debugging and exploration.

```bash
docker-compose run --rm nvcontrol-dev

# Inside container:
cargo build --all-features
cargo test --test gpu_tests
nvidia-smi
nvctl doctor
```

Perfect for:
- Debugging failing tests
- Exploring NVML API
- Testing on different GPU configurations
- Manual testing with `nvctl` commands

## Architecture

### Base Image: Arch Linux
The Dockerfile uses `archlinux:latest` to match the target deployment environment.

### Installed Packages
- **Rust toolchain**: stable with clippy and rustfmt
- **NVIDIA tools**: drivers, CUDA, OpenCL, nvidia-settings
- **Gaming tools**: gamemode, mangohud, gamescope
- **Container tools**: Docker, Podman
- **Debug tools**: valgrind, gdb, strace, htop
- **Wayland/X11**: wayland, weston
- **Development**: neovim, tmux
- **Rust tools**: cargo-tarpaulin, cargo-watch, cargo-audit

### Volumes
- `..:/workspace/nvcontrol` - Source code (read/write)
- `../test-results:/workspace/test-results` - Test output
- `../bench-results:/workspace/bench-results` - Benchmark results
- `../coverage:/workspace/coverage` - Coverage reports
- `cargo-cache:/home/tester/.cargo` - Persistent Cargo cache

### GPU Access
All GPU-enabled services use:
- `runtime: nvidia` - NVIDIA Container Runtime
- `/dev/nvidia*` devices mounted
- `NVIDIA_VISIBLE_DEVICES=all`
- `NVIDIA_DRIVER_CAPABILITIES=all`

## Testing Workflow

### 1. Quick Validation
```bash
# Lint + unit tests (no GPU needed)
docker-compose up nvcontrol-lint
docker-compose up nvcontrol-unit-tests
```

### 2. Full Test Suite
```bash
# All tests including GPU integration
docker-compose up nvcontrol-test
```

### 3. Specific Feature Testing
```bash
# Interactive shell for manual testing
docker-compose run --rm nvcontrol-dev

# Inside container:
cargo test --test gpu_tests -- --nocapture
cargo test --test fan_control -- --nocapture
nvctl gpu info
nvctl fan auto
```

### 4. Performance Analysis
```bash
# Benchmarks
docker-compose up nvcontrol-bench

# Coverage
docker-compose up nvcontrol-coverage
```

## Troubleshooting

### GPU Not Detected
```bash
# Verify NVIDIA runtime is configured
docker run --rm --gpus all nvidia/cuda:12.0-base nvidia-smi

# Check nvidia-container-toolkit
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker
```

### Permission Denied
```bash
# Add user to docker group
sudo usermod -aG docker $USER
# Log out and back in

# Or run with sudo (not recommended)
sudo docker-compose up nvcontrol-test
```

### Build Failures
```bash
# Clean rebuild
docker-compose build --no-cache

# Remove old containers
docker-compose down -v
```

### X11 Display Issues
```bash
# Allow Docker to access X11
xhost +local:docker

# Or set DISPLAY manually
export DISPLAY=:0
docker-compose up nvcontrol-test
```

## CI/CD Integration

This Docker setup is designed to integrate with GitHub Actions or GitLab CI:

```yaml
# Example GitHub Actions workflow
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: |
          cd dev
          docker-compose up --exit-code-from nvcontrol-unit-tests nvcontrol-unit-tests
          docker-compose up --exit-code-from nvcontrol-lint nvcontrol-lint
```

For GPU-enabled CI, use self-hosted runners with NVIDIA GPUs.

## Performance Tips

1. **Use the Cargo cache volume** - Speeds up rebuilds significantly
2. **Run unit tests first** - Faster feedback without GPU overhead
3. **Parallel testing** - Run lint + unit tests concurrently
4. **Incremental builds** - Mount source as volume, rebuild inside container

## Multi-GPU Testing

To test specific GPUs:

```bash
# Use only GPU 0
NVIDIA_VISIBLE_DEVICES=0 docker-compose up nvcontrol-test

# Use GPUs 0 and 1
NVIDIA_VISIBLE_DEVICES=0,1 docker-compose up nvcontrol-test
```

## Debugging

### Attach to Running Container
```bash
docker exec -it nvcontrol-dev bash
```

### View Logs
```bash
docker-compose logs -f nvcontrol-test
```

### Inspect State
```bash
docker-compose run --rm nvcontrol-dev bash
# Inside:
nvidia-smi
lspci | grep -i nvidia
cat /proc/driver/nvidia/version
```

## Clean Up

```bash
# Stop all containers
docker-compose down

# Remove volumes (including Cargo cache)
docker-compose down -v

# Remove images
docker rmi nvcontrol:test
```

## ROG Astral 5090 Specific Testing

For testing on your ROG Astral RTX 5090:

```bash
# Verify GPU is detected
docker-compose run --rm nvcontrol-dev nvidia-smi

# Run full test suite with your GPU
docker-compose up nvcontrol-test

# Interactive testing
docker-compose run --rm nvcontrol-dev
# Inside:
nvctl gpu info          # Should show ROG Astral RTX 5090
nvctl overclock info    # Check OC capabilities
nvctl fan status        # Verify fan control
nvctl profile list      # Test profile system
```

The test environment automatically detects ASUS ROG cards and applies the Nord theme.

## Next Steps

1. **Write Integration Tests** - See `../tests/` directory
2. **Add Benchmarks** - See `../benches/` directory
3. **CI/CD Pipeline** - GitHub Actions with self-hosted runner
4. **Multi-GPU Tests** - Test SLI/NVLink configurations
5. **Hardware Validation** - Test on real ROG Astral 5090
