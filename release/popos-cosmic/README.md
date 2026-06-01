# nvcontrol for Pop!_OS Cosmic

## Overview

nvcontrol provides native NVIDIA GPU control for Pop!_OS with the COSMIC desktop environment.

## Installation

### From .deb package (recommended)

```bash
# One-line install (downloads the latest full GUI + CLI tarball)
curl -fsSL https://nv.cktech.sh | sudo bash
```

`https://nv.cktech.sh` redirects to `https://raw.githubusercontent.com/GhostKellz/nvcontrol/main/release/install-system.sh`.

### From source

```bash
# Install build dependencies
sudo apt install cargo rustc libwayland-dev libxkbcommon-dev pkg-config

# Clone and build
git clone https://github.com/GhostKellz/nvcontrol
cd nvcontrol
cargo build --release --bin nvctl
cargo build --release --bin nvcontrol --features gui

# Install
sudo install -Dm755 target/release/nvctl /usr/bin/nvctl
sudo install -Dm755 target/release/nvcontrol /usr/bin/nvcontrol
```

## COSMIC Desktop Integration

nvcontrol automatically detects COSMIC desktop and uses:

- **cosmic-randr** for display configuration when available
- **Native NVKMS ioctls** for digital vibrance (no nvidia-settings needed)
- **COSMIC compositor** for VRR/HDR when supported

### Compositor Detection

nvcontrol detects COSMIC via:
```
XDG_CURRENT_DESKTOP=COSMIC
```

### Display Control

On COSMIC, nvcontrol can use `cosmic-randr` for display management:

```bash
# List displays
nvctl display ls

# Set vibrance (uses native NVKMS on COSMIC)
nvctl display vibrance set-display 0 150

# VRR configuration
nvctl vrr status
```

## System76 Driver Compatibility

nvcontrol works with both:
- `system76-driver-nvidia` (Pop!_OS default)
- Standard `nvidia-driver` packages

For nvcontrol-to-driver compatibility guidance, see [`../../docs/drivers/nvidia-driver.md`](../../docs/drivers/nvidia-driver.md).

## Quick Start

```bash
# GPU information
nvctl gpu info

# Launch TUI dashboard
nvctl nvtop

# Launch GUI
nvcontrol

# Digital vibrance (boost colors)
nvctl display vibrance set-display 0 150

# Enable VRR
nvctl vrr enable DP-1
```

## Troubleshooting

### COSMIC not detected

Ensure `XDG_CURRENT_DESKTOP=COSMIC` is set. Check with:
```bash
echo $XDG_CURRENT_DESKTOP
```

### Vibrance not working

Digital vibrance requires `/dev/nvidia-modeset` access:
```bash
# Check device exists
ls -la /dev/nvidia-modeset

# If permission denied, ensure you're in the 'video' group
sudo usermod -aG video $USER
```

### cosmic-randr not found

Install from Pop!_OS cosmic packages:
```bash
sudo apt install cosmic-randr
```

## Features

| Feature | COSMIC Support |
|---------|---------------|
| Digital Vibrance | Native NVKMS |
| VRR | cosmic-randr / DRM |
| HDR | compositor-dependent |
| Overclocking | nvidia-smi / NVML |
| Fan Control | NVML |
| Multi-Monitor | cosmic-randr |

## Support

- GitHub Issues: https://github.com/GhostKellz/nvcontrol/issues
- Pop!_OS Forums: https://pop-planet.info/
