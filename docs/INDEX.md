# nvcontrol Documentation Hub

Complete documentation for nvcontrol v0.7.3 - Modern NVIDIA Settings Manager for Linux + Wayland.

## Quick Start

**New Users:**
1. [README](../README.md) - Project overview and installation
2. [BUILDING](../BUILDING.md) - Build from source
3. [CONTRIBUTING](../CONTRIBUTING.md) - Development guidelines
4. [TUI User Guide](TUI_USER_GUIDE.md) - Terminal interface quickstart

**RTX 5090 Users:**
- [RTX 5090 Setup Guide](RTX_5090_SETUP_GUIDE.md) - Blackwell-specific setup

---

## Core Documentation

### Installation & Setup

- [**Building from Source**](BUILDING.md) - Compilation and dependencies
  - Cargo features (gui, tray, cli-only)
  - System dependencies
  - Distribution-specific guides

### Command Reference

- [**Commands Overview**](COMMANDS.md) - Complete CLI reference
  - GPU monitoring and control
  - Overclocking and power management
  - Display and color management
  - Fan control
  - Gaming optimizations

- [**Command Details**](commands/)
  - [GPU Commands](commands/gpu.md)
  - [Power Management](commands/power.md)
  - [Overclocking](commands/overclock.md)
  - [Gaming Profiles](commands/gaming.md)
  - [Container Integration](commands/container.md)
  - [Configuration](commands/config.md)

### User Guides

- [**TUI User Guide**](TUI_USER_GUIDE.md) - Terminal UI walkthrough
  - Live GPU monitoring
  - Real-time graphs
  - Interactive controls

- [**Digital Vibrance Guide**](digital-vibrance.md) - Color saturation control
  - Wayland support (nVibrant)
  - Per-display configuration
  - Gaming profiles

### Display & Graphics Guides

- [**VRR/G-SYNC Control**](VRR_GSYNC.md) - Variable refresh rate
  - G-SYNC and FreeSync support
  - Per-compositor configuration
  - Gaming optimizations

- [**HDR Control**](HDR_CONTROL.md) - High Dynamic Range
  - KDE/GNOME/Hyprland HDR
  - Color depth settings
  - Display capabilities

- [**Image Sharpening**](IMAGE_SHARPENING.md) - GPU post-processing
  - NVKMS-based sharpening
  - Per-display control
  - Gaming presets

- [**Overclocking Guide**](OVERCLOCKING.md) - Performance tuning
  - Safe overclocking methods
  - Auto-tuning wizard
  - Architecture-specific guidance

---

## API Reference

### Core APIs

- [**GPU Management**](api/GPU.md) - GPU monitoring and info
  - Real-time statistics
  - Hardware detection
  - Temperature and power monitoring

- [**Power Management**](api/POWER.md) - Power limits and modes
  - Power limit control
  - Power mode presets
  - TDP management

- [**Overclocking**](api/OVERCLOCK.md) - Clock speed control
  - GPU/memory offset control
  - Stress testing
  - Safety features

- [**Fan Control**](api/FAN.md) - Fan speed management
  - Manual fan control
  - Custom fan curves
  - Preset curves (silent, balanced, aggressive)

- [**Display Management**](api/DISPLAY.md) - Display and color control
  - Digital vibrance
  - Multi-display support
  - HDR control

---

## Integration Guides

### Container Runtimes

- [**Bolt Integration**](integration/BOLT.md) - Gaming container runtime
  - Profile auto-apply
  - GPU optimization for containers
  - Performance monitoring

- [**nvbind Integration**](integration/NVBIND.md) - GPU container passthrough
  - Unified GPU management
  - Container-aware monitoring
  - Gaming optimization workflows

### Applications

- [**ghostwave Integration**](integration/GHOSTWAVE.md) - RTX Voice integration
  - Low-latency audio profiles
  - Silent operation
  - Automatic GPU optimization

---

## GPU Support

### Supported GPUs

| Architecture | Compute | Example GPUs | Features |
|--------------|---------|--------------|----------|
| **Blackwell** | 10.0 | RTX 5060-5090 | Full support, FP4 Tensor Cores |
| **Ada Lovelace** | 8.9 | RTX 4060-4090 | Full support, DLSS 3 |
| **Ampere** | 8.6 | RTX 3060-3090 Ti | Full support, MIG |
| **Turing** | 7.5 | RTX 2060-2080 Ti | Full support |
| **Pascal** | 6.x | GTX 1060-1080 Ti | Basic support |

### Feature Support Matrix

| Feature | Blackwell | Ada | Ampere | Turing | Pascal |
|---------|-----------|-----|--------|--------|--------|
| **Monitoring** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Overclocking** | ✅ | ✅ | ✅ | ✅ | ⚠️  |
| **Power Management** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Fan Control** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Digital Vibrance** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **MIG Support** | ✅ | ❌ | ✅ | ❌ | ❌ |

---

## Hardware-Specific Guides

### ASUS ROG Astral RTX 5090

**Flagship Setup Guide**: [RTX 5090 Setup Guide](RTX_5090_SETUP_GUIDE.md)

**Specifications:**
- Blackwell GB202 architecture
- 32GB GDDR7 memory
- 630W max power (factory OC: 2610MHz boost)
- Quad-fan cooling system

**Optimized Profiles:**
```bash
# Gaming (450W)
nvctl profile apply gaming

# Max Performance (630W)
nvctl profile apply max-performance

# Silent Operation (300W)
nvctl profile apply quiet
```

**Features:**
- [ASUS-specific features documentation](ASUS_ASTRAL_FEATURES.md)
- Advanced fan control (4-fan management)
- Factory overclock profiles
- Temperature-optimized curves

### Driver Optimization

- [**Kernel Driver 580+ Optimizations**](KERNEL_DRIVER_580_OPTIMIZATIONS.md)
  - NVIDIA Open Kernel Module features
  - Wayland compositor optimization
  - GSP firmware benefits

---

## Platform Support

### Linux Distributions

**Primary Support** (Tier 1):
- ✅ **Arch Linux** - Premier platform, PKGBUILD available
- ✅ **Fedora 39+** - Full RPM support
- ✅ **Nobara** - Gaming-optimized Fedora, first-class support
- ✅ **Bazzite** - Fedora Atomic gaming, rpm-ostree compatible
- ✅ **Debian 12+** / Ubuntu 22.04+ - .deb packages
- ✅ **Pop!_OS** - Includes COSMIC desktop support

**Additional Support** (Tier 2):
- ✅ Manjaro
- ✅ openSUSE Tumbleweed
- ✅ Gentoo
- ✅ EndeavourOS

### Display Servers

**Primary**:
- ✅ **Wayland** - KDE Plasma 6+, GNOME 45+, Hyprland, Sway
  - Full feature support
  - nVibrant integration for digital vibrance

**Legacy**:
- ✅ **X11** - Full compatibility
  - Direct nvidia-settings integration

### Desktop Environments

**Optimized For:**
- KDE Plasma 6+ (Wayland)
- GNOME 45+ (Wayland)
- Hyprland
- Sway
- Pop!_OS COSMIC (new in v0.7.1)

**Supported:**
- XFCE
- Cinnamon
- MATE
- i3/bspwm
- Weston (reference)

---

## Configuration

### Config File Locations

```
~/.config/nvcontrol/config.toml           # Main configuration
~/.config/nvcontrol/profiles/             # Saved profiles
~/.config/nvcontrol/profiles/gaming.toml  # Gaming profile
~/.config/nvcontrol/profiles/silent.toml  # Silent operation
```

### Profile Examples

#### Gaming Profile
```toml
[profile]
name = "Gaming"
description = "High performance gaming"

[gpu]
overclock_gpu = 150
overclock_memory = 500

[power]
limit = 450  # Watts
mode = "MaxPerformance"

[fan]
preset = "aggressive"

[display.DP-1]
digital_vibrance = 800
```

#### Content Creation
```toml
[profile]
name = "Content Creation"

[gpu]
overclock_gpu = 0
overclock_memory = 0

[power]
limit = 350
mode = "Balanced"

[fan]
preset = "silent"

[display.DP-1]
digital_vibrance = 0  # Natural colors
```

---

## Use Cases

### Gaming Optimization

**Competitive Gaming** (CS2, Valorant):
```bash
nvctl profile apply competitive
# Settings: +150/+500 OC, 900 vibrance, aggressive fans
```

**Single-Player Gaming** (Cyberpunk 2077):
```bash
nvctl profile apply ultra-gaming
# Settings: +175/+600 OC, 750 vibrance, balanced fans
```

### Professional Workflows

**Photo Editing**:
```bash
nvctl profile apply photo-editing
# Settings: No OC, 0 vibrance, silent fans
```

**3D Rendering**:
```bash
nvctl profile apply rendering
# Settings: +100/+400 OC, max power, aggressive cooling
```

**Live Streaming**:
```bash
nvctl profile apply streaming
# Settings: Moderate OC, silent fans for clean audio
```

### Container Gaming

**With nvbind**:
```bash
# Apply nvcontrol profile, launch container
nvctl profile apply gaming-container
nvbind run --runtime bolt --gpu all --nvcontrol-profile gaming steam:latest
```

**With Bolt**:
```bash
# Bolt auto-applies nvcontrol profiles
bolt surge up --gpu --nvcontrol-profile competitive
```

---

## Troubleshooting

### Common Issues

**NVML not available**:
```bash
# Check driver installation
nvidia-smi

# Install drivers
nvctl drivers install open      # NVIDIA Open Kernel Modules
nvctl drivers install proprietary
```

**Fan control not working**:
```bash
# Check permissions
ls -la /sys/class/hwmon/

# May require root for sysfs access
sudo nvctl fan set 50
```

**Digital vibrance not working (Wayland)**:
```bash
# Install nVibrant
paru -S nvibrant-cli

# Verify installation
which nvibrant
```

**Overclocking not persistent**:
```bash
# Apply profile at login
nvctl profile apply gaming --autostart
```

### Debug Mode

```bash
# Enable verbose logging
RUST_LOG=debug nvctl gpu stat

# GUI debug mode
RUST_LOG=debug nvcontrol
```

---

## Development

### Building

```bash
# Full build (GUI + CLI)
cargo build --release --all-features

# CLI only (minimal dependencies)
cargo build --release --no-default-features

# GUI without tray
cargo build --release --features gui
```

### Contributing

See [Contributing Guidelines](../CONTRIBUTING.md) for:
- Code style and standards
- Testing requirements
- Pull request process

### Architecture

```
nvcontrol/
├── src/lib.rs              # Core library
├── src/gpu.rs              # GPU monitoring (NVML)
├── src/overclocking.rs     # Clock control
├── src/power.rs            # Power management
├── src/fan.rs              # Fan control
├── src/display.rs          # Display/color management
├── src/drivers.rs          # Driver installation
└── src/bin/
    ├── nvcontrol.rs        # GUI (egui)
    └── nvctl.rs            # CLI tool
```

---

## Performance Targets

### CLI Performance

| Operation | Target | Achieved |
|-----------|--------|----------|
| GPU info query | <50ms | ✅ ~30ms |
| Stats update | <100ms | ✅ ~80ms |
| Profile apply | <500ms | ✅ ~350ms |

### GUI Performance

| Metric | Target | Status |
|--------|--------|--------|
| Startup time | <2s | ✅ |
| Frame rate | 60 FPS | ✅ |
| Memory usage | <100MB | ✅ ~70MB |
| Stats refresh | 1s | ✅ Configurable |

---

## Resources

### Documentation

- [Main README](../README.md)
- [Changelog](../CHANGELOG.md)
- [License](../LICENSE)

### Community

- [GitHub Discussions](https://github.com/ghostkellz/nvcontrol/discussions)
- [Issue Tracker](https://github.com/ghostkellz/nvcontrol/issues)
- [Discord](https://discord.gg/nvcontrol) (planned)

### Related Projects

- [nvbind](https://github.com/ghostkellz/nvbind) - GPU container runtime
- [ghostwave](https://github.com/ghostkellz/ghostwave) - RTX Voice alternative
- [nVibrant](https://github.com/Tremeschin/nVibrant) - Wayland vibrance

---

**Last Updated**: December 2024 (v0.7.3 - ASUS Astral 5090, Power Detector+, enhanced GPU info)
