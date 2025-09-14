# nvcontrol 🎮

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![NVIDIA](https://img.shields.io/badge/NVIDIA-Open%20Drivers%20580+-76B900.svg)](https://github.com/NVIDIA/open-gpu-kernel-modules)
[![Wayland](https://img.shields.io/badge/Wayland-Native-brightgreen.svg)](https://wayland.freedesktop.org/)

**The Ultimate NVIDIA GPU Control Tool for Linux**

nvcontrol is the most advanced NVIDIA GPU management solution for Linux, featuring pure Rust digital vibrance, comprehensive container GPU passthrough, and cutting-edge Wayland support.

## 🚀 Revolutionary Features

### 🌈 **Pure Rust Digital Vibrance**
- **Zero Dependencies**: Built-in vibrance control - no external tools needed
- **NVIDIA Open Drivers**: Direct integration with NVIDIA Open Drivers 580+
- **Simple Commands**: `nvctl vibe 150` - instant vibrance control
- **Per-Display Control**: Individual settings for multi-monitor setups
- **Wayland & X11**: Works on all display servers

### 🐳 **Container GPU Passthrough**
- **Built-in Container Runtime**: Pure Rust Docker/Podman/NixOS support
- **PhantomLink Integration**: Audio container with RTX Voice in one command
- **GPU Profiles**: Optimized profiles for ML, gaming, inference workloads
- **Multi-Runtime**: Docker, Podman, containerd, NixOS containers

### 🎮 **Advanced GPU Control**
- **Real-time Monitoring**: Live TUI dashboard with performance graphs
- **Smart Overclocking**: Safe limits with automatic stress testing
- **Fan Control**: Custom curves with thermal protection
- **Power Management**: Advanced profiles and thermal monitoring

### ⚡ **Upscaling Technologies**
- **DLSS/FSR/XeSS toggle** with per-game profiles
- **Automatic game detection** and profile application
- **Quality presets** (Performance, Balanced, Quality, Ultra)
- **Configuration file modification** for deep integration

### 🌀 **Fan Control**
- **Real-time fan monitoring** with RPM and percentage display
- **Manual fan speed control** (where supported)
- **Quick presets** (Quiet, Auto, Max)
- **Custom fan curves** (coming soon)

### 🚀 **Driver Management**
- **Package manager integration** (Arch, Ubuntu, Fedora)
- **Driver installation**: `nvctl drivers install proprietary/open/open-beta`
- **DKMS issue resolution** with automatic repair
- **Shell completions** for Bash, Zsh, Fish

## 🏗️ **Modern Architecture**

### **Wayland-Native**
- Full support for KDE, GNOME, Hyprland, Sway
- Direct compositor integration for VRR control
- No X11 dependencies for core functionality

### **Beautiful UI**
- **Glass morphism design** with NVIDIA-inspired themes
- **Dark/Light/Gaming** theme variants
- **Live GPU stats** with real-time graphs
- **Intuitive controls** with safety warnings

### **Essential Commands**

#### Pure Rust Digital Vibrance
```bash
# Simple vibrance control (0-200%)
nvctl vibe 100        # Default vibrance
nvctl vibe 150        # Enhanced colors
nvctl vibe 200        # Maximum saturation
nvctl vibe 80         # Reduced saturation

# Advanced vibrance control
nvctl display vibrance get          # Current status
nvctl display vibrance reset        # Reset to default
nvctl display vibrance info         # Show capabilities
```

#### Container GPU Control (nvctl ct / nvctl ctr)
```bash
# Launch GPU containers
nvctl ct launch --image nvidia/cuda:12.0-runtime-ubuntu20.04 --gpu all -i
nvctl ctr launch --image tensorflow/tensorflow:latest-gpu --name ml-training

# PhantomLink audio container with RTX Voice
nvctl ct phantomlink --rtx-voice --mode prod

# Container management
nvctl ct list                       # List GPU containers
nvctl ct runtime info              # Runtime information
nvctl ct runtime test              # Test GPU passthrough
```

#### GPU Monitoring & Control
```bash
# Live monitoring
nvctl gpu stat                      # Interactive TUI dashboard
nvctl gpu info                      # GPU information

# Overclocking
nvctl overclock apply --gpu-offset 150 --memory-offset 500
nvctl overclock stress-test 10      # Stability test

# Power & thermal
nvctl power limit 90                # Power limit percentage
nvctl fan set 0 75                  # Fan speed control
```

## 🎯 **What Makes nvcontrol Special**

### **Linux-First Design**
Unlike Windows tools ported to Linux, nvcontrol is built ground-up for Linux:
- **Package manager integration** instead of manual downloads
- **Systemd integration** for automatic startup
- **Proper desktop file** with correct categories and icons
- **Shell completion** for power users

### **Wayland Excellence** 
- **No X11 fallbacks** - true Wayland-native operation
- **Per-compositor optimization** (KDE vs GNOME vs Hyprland)
- **Future-proof architecture** ready for post-X11 world

### **Advanced Features**
- **Per-game profiles** that auto-apply when games launch
- **Thermal management** with predictive algorithms
- **Smart defaults** that learn from user behavior
- **Enterprise features** for multi-GPU setups

## 🚀 **Quick Start**

### **Prerequisites**
For full functionality, especially digital vibrance control:
```bash
# Python 3.9+ and pip/uv for nvibrant integration
sudo apt install python3 python3-pip  # Ubuntu/Debian
# OR
sudo pacman -S python python-pip      # Arch
# OR  
sudo dnf install python3 python3-pip  # Fedora

# nvibrant will be automatically installed during build
```

### **GUI Application**
```bash
# Install and run GUI
cargo build --release --features gui
./target/release/nvcontrol
```

### **CLI Tools** 
```bash
# Install shell completions
./scripts/install-completions.sh

# Basic usage
nvctl gpu info                   # GPU information
nvctl display vibrance 800 600  # Set vibrance
nvctl fan info                   # Fan status
```

### **Installation**
```bash
# Arch Linux (AUR)
yay -S nvcontrol-git

# Ubuntu/Debian
sudo dpkg -i nvcontrol_0.1.0_amd64.deb

# Fedora
sudo rpm -i nvcontrol-0.1.0.x86_64.rpm

# From source
git clone https://github.com/ghostkellz/nvcontrol
cd nvcontrol
cargo build --release
```

## 📚 **Documentation**

- [**COMMANDS.md**](COMMANDS.md) - Complete CLI reference
- [**DOCS.md**](DOCS.md) - Technical documentation  
- [**BUILDING.md**](BUILDING.md) - Build instructions

## 🎨 **Screenshots**

### Modern GUI
The application features a beautiful, modern interface with:
- **Real-time GPU monitoring** with live charts
- **Advanced overclocking controls** with safety warnings
- **VRR management** across all displays
- **Theme selection** (NVIDIA Dark, Light, Gaming)

### TUI Dashboard
```
┌─ GPU Stats ─────────────────────────────────────┐
│ 🎯 RTX 4090          🌡️ 72°C    ⚡ 380W        │
│ 📈 GPU: ████████░░ 85%   💾 VRAM: 18.2/24.0 GB │
│ 🌀 Fan: ██████░░░░ 65%   🔥 Temp: ████████░░    │
└─────────────────────────────────────────────────┘
```

## 🛣️ **Roadmap**

### **v0.2.0 - Enhanced Gaming**
- [ ] Gamescope integration
- [ ] Shader cache management  
- [ ] Latency optimization tools
- [ ] RGB lighting control

### **v0.3.0 - Enterprise Features**
- [ ] Multi-GPU management
- [ ] Remote monitoring
- [ ] Team management tools
- [ ] Performance analytics

### **v1.0.0 - Production Ready**
- [ ] Stable API
- [ ] Plugin system
- [ ] Marketplace integration
- [ ] Enterprise support

## 🤝 **Contributing**

nvcontrol is open source and welcomes contributions! See our contributing guidelines for details on:
- Code style and standards
- Testing requirements  
- Documentation updates
- Feature proposals

## 📄 **License**

MIT License - see [LICENSE](LICENSE) for details.

Copyright (c) 2025 CK Technology LLC

---

**Made with ❤️ for the Linux gaming community**

[![Wayland Support](https://img.shields.io/badge/Wayland-Ready-brightgreen?logo=wayland)](https://wayland.freedesktop.org/)
[![NVIDIA](https://img.shields.io/badge/NVIDIA-Supported-brightgreen?logo=nvidia)](https://nvidia.com)
[![CLI & GUI](https://img.shields.io/badge/CLI_%2B_GUI-Full_Featured-blueviolet)](#features)
[![nvibrance Integration](https://img.shields.io/badge/nvibrance-Integrated-ff69b4)](https://github.com/Tremeschin/nVibrant)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ghostkellz/nvcontrol/ci.yml?branch=main)](https://github.com/ghostkellz/nvcontrol/actions)

---

## The Missing NVIDIA Control Panel for Linux + Wayland

**nvcontrol** is a modern, fully featured NVIDIA settings manager for Linux.  
Think: _NVIDIA Control Panel & GeForce Experience, but for Linux + Wayland_.  
No more missing options. No more hacky workarounds. No more nvidia-settings being years behind.

- **Wayland native:** Full support for Wayland compositors (KDE, GNOME, Hyprland, Sway, etc.)
- **Legacy X11 compatible:** Works wherever NVIDIA is available.

---

## Features

- 🖥️ **Full GPU Control**  
  - Clock speeds, power limits, overclocking, undervolting
  - Memory timings, fan curves, temp/power monitoring
  - 🎛️ **Display & Color Management**
  - Per-display resolution, refresh rate, orientation, rotation
  - Digital Vibrance, color profiles, gamma control (via integrated [nVibrant](https://github.com/Tremeschin/nVibrant))
  - HDR toggling and fine-tuning (where supported)
  - Hotplug/multi-monitor configuration with persistent profiles

- 🔊 **Performance & Monitoring**
  - Real-time stats: GPU/VRAM usage, temps, wattage, per-process utilization
  - System tray widget for live monitoring and quick toggles
  - Advanced logging & export (JSON/CSV)

- 🌡️ **Fan & Thermal Control**
  - Custom fan curves, manual overrides, and auto-fan settings
  - Overheat protection, alerts, and fail-safe triggers

- 🖱️ **Input & Latency Tweaks**
  - Low-latency and frame pacing controls for gaming
  - Adjustable frame limiter, VRR/G-SYNC toggle, V-Sync, and more
  - VR/AR optimizations (if available)

- 🧩 **Profiles & Automation**
  - Game/app-specific profiles (auto-load settings per-app)

---

## Installation

### Pre-built Binaries
Download prebuilt binaries from the [Releases](https://github.com/ghostkellz/nvcontrol/releases) page.

### From Source
```sh
# Full installation with GUI and system tray
cargo install nvcontrol --all-features

# CLI only (no GUI dependencies)
cargo install nvcontrol --no-default-features

# GUI without system tray
cargo install nvcontrol --features gui
```

### Build Options
- `--all-features` - Full GUI with system tray support (default for releases)
- `--features gui` - GUI without system tray
- `--no-default-features` - CLI only, minimal dependencies

### Requirements

- NVIDIA GPU with nvidia open drivers (570+) at least during testing
- Wayland compositor (KDE, GNOME, Hyprland, Sway, etc.)
- Rust (for building from source)

### Optional Dependencies
- **GUI features**: GTK3, GLib (automatically handled by package managers)
- **System tray**: Desktop environment with system tray support
- **Digital vibrance**: [nVibrant](https://github.com/Tremeschin/nVibrant) for Wayland
- **AUR**: TBD

---

## Usage

### CLI

```sh
nvctl --help
```

### GUI

Launch via your application launcher or run:

```sh
# Full GUI (requires --features gui or --all-features during build)
nvcontrol
```

If built without GUI features, nvcontrol will display an error message.

---

## Roadmap

- [ ] Full support for all NVIDIA GPUs (Turing, Ampere, Ada, etc.)
- [ ] Flatpak & AUR packaging
- [ ] Profile system with save/load functionality
- [ ] System tray widget for quick access
- [ ] Advanced display management (resolution, refresh rate, orientation)
- [ ] Custom fan curves and thermal management
- [ ] Real-time notifications and alerts

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

For detailed build instructions, see [BUILDING.md](BUILDING.md).

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

- [nVibrant](https://github.com/Tremeschin/nVibrant) for Digital Vibrance integration
- [wlroots](https://gitlab.freedesktop.org/wlroots/wlroots) and the Wayland community
- All contributors and testers

