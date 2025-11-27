# nvcontrol

<div align="center">
  <img src="assets/icons/nvctl_logo.png" alt="nvcontrol Logo" width="128" height="128">

  **Modern NVIDIA Settings Manager for Linux + Wayland**

  [![Version](https://img.shields.io/badge/version-0.7.1-blue.svg)](https://github.com/GhostKellz/nvcontrol/releases)
  [![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![NVIDIA](https://img.shields.io/badge/NVIDIA-Open%20Drivers%20580+-76B900.svg)](https://github.com/NVIDIA/open-gpu-kernel-modules)
  [![Wayland](https://img.shields.io/badge/Wayland-Native-brightgreen.svg)](https://wayland.freedesktop.org/)

  *The missing NVIDIA Control Panel for Linux*
</div>

---

## Overview

nvcontrol is a comprehensive NVIDIA GPU control tool for Linux, designed from the ground up for Wayland. It provides everything you need to manage your NVIDIA GPU: digital vibrance, VRR/G-SYNC, HDR, overclocking, fan control, and more.

**Key Features:**
- Native digital vibrance via NVKMS ioctls (no nvidia-settings required)
- Full Wayland support: KDE Plasma, GNOME, Hyprland, Sway, Pop!_OS COSMIC
- RTX 50-series (Blackwell) and RTX 40/30-series support
- TUI dashboard + GUI application
- Container GPU passthrough

## Quick Start

```bash
# GPU information (like nvidia-smi)
nvctl gpu info

# Live monitoring dashboard
nvctl gpu stat

# Digital vibrance (0-200, default 100)
nvctl display vibrance set 150

# Launch GUI
nvcontrol
```

## Installation

### Arch Linux (Premier Platform)
```bash
# From AUR
yay -S nvcontrol-git

# Or build from source
git clone https://github.com/GhostKellz/nvcontrol
cd nvcontrol
makepkg -si -p release/arch/PKGBUILD
```

### Debian/Ubuntu
```bash
# Download from releases
wget https://github.com/GhostKellz/nvcontrol/releases/latest/download/nvcontrol_0.7.1-1_amd64.deb
sudo apt install ./nvcontrol_0.7.1-1_amd64.deb
```

### Fedora/Nobara
```bash
# Download from releases
sudo dnf install https://github.com/GhostKellz/nvcontrol/releases/latest/download/nvcontrol-0.7.1-1.fc39.x86_64.rpm
```

### From Source
```bash
git clone https://github.com/GhostKellz/nvcontrol
cd nvcontrol
cargo build --release
sudo install -Dm755 target/release/nvctl /usr/bin/nvctl
sudo install -Dm755 target/release/nvcontrol /usr/bin/nvcontrol
```

See [BUILDING.md](BUILDING.md) for detailed build instructions.

## Features

### Digital Vibrance
Native implementation using NVKMS ioctls - works on Wayland without nvidia-settings.

```bash
nvctl display vibrance list          # Show all displays
nvctl display vibrance set 150       # Boost colors (100=default)
nvctl display vibrance set 150 -d 1  # Specific display
```

### Display Controls
```bash
nvctl display ls                     # List displays
nvctl display color-range get        # Full vs Limited RGB
nvctl display color-space get        # RGB, YCbCr422, YCbCr444
nvctl display dithering get          # Dithering status
nvctl display sharpening set 5       # Image sharpening (0-10)
```

### VRR / G-SYNC
```bash
nvctl vrr status                     # Check VRR status
nvctl vrr enable DP-1                # Enable on display
nvctl vrr disable DP-1               # Disable
```

### GPU Monitoring
```bash
nvctl gpu info                       # GPU information
nvctl gpu stat                       # TUI dashboard
nvctl gpu watch                      # Live monitoring
nvctl gpu watch --interval 500       # Custom refresh rate
```

### Overclocking
```bash
nvctl overclock status               # Current clocks
nvctl overclock apply --gpu-offset 100 --memory-offset 500
nvctl overclock reset                # Reset to stock
```

### Fan Control
```bash
nvctl fan info                       # Fan status
nvctl fan set 0 75                   # Set fan 0 to 75%
nvctl fan auto                       # Return to auto
```

### Power Management
```bash
nvctl power status                   # Power info
nvctl power limit 90                 # Set power limit %
```

## Supported Hardware

| Generation | GPUs | Status |
|------------|------|--------|
| RTX 50 (Blackwell) | 5090, 5080, 5070 Ti/5070, 5060 Ti/5060 | Full Support |
| RTX 40 (Ada) | 4090, 4080, 4070 Ti/4070, 4060 Ti/4060 | Full Support |
| RTX 30 (Ampere) | 3090/3080/3070/3060 series | Full Support |
| RTX 20 (Turing) | 2080/2070/2060 series | Supported |
| GTX 16/10 | 1660/1650/1080/1070/1060 | Basic Support |

## Supported Compositors

| Compositor | Vibrance | VRR | HDR |
|------------|----------|-----|-----|
| KDE Plasma 6 | Native NVKMS | kscreen-doctor | Yes |
| GNOME 45+ | Native NVKMS | gsettings | Yes |
| Hyprland | Native NVKMS | hyprctl | Planned |
| Sway | Native NVKMS | swaymsg | No |
| Pop!_OS COSMIC | Native NVKMS | cosmic-randr | Yes |

## TUI Dashboard

Launch with `nvctl gpu stat`:

```
┌─ nvcontrol v0.7.1 │ GPU 0 │ 55°C │ 85% │ LIVE ─────────────┐
├─ Overview ─ Performance ─ Memory ─ Temp ─ Power ─ OC ─────┤
│                                                            │
│  GPU Stats                                                 │
│  ┌────────────────────────────────────────────────────┐   │
│  │ NVIDIA GeForce RTX 5090    72°C  ⚡ 380W           │   │
│  │ GPU:  [████████████████░░░░] 85%                   │   │
│  │ VRAM: [██████████████░░░░░░] 18.2/32.0 GB          │   │
│  │ Fan:  [████████████░░░░░░░░] 65%                   │   │
│  └────────────────────────────────────────────────────┘   │
│                                                            │
│  [q]uit [h]elp [t]heme [←→] tabs [↑↓] scroll              │
└────────────────────────────────────────────────────────────┘
```

Tabs: Overview, Performance, Memory, Temperature, Power, Processes, Overclock, Fan Control, Profiles, Tuner, Profiler, OSD, Settings

## Themes

nvcontrol includes several themes:
- **Tokyo Night Moon** (default)
- Tokyo Night / Tokyo Night Storm
- Dracula
- ROG Red (auto-selected for ASUS GPUs)
- Matrix Green
- Cyberpunk

Change theme with `t` in TUI or via Settings in GUI.

## Documentation

- [COMMANDS.md](COMMANDS.md) - Complete CLI reference
- [BUILDING.md](BUILDING.md) - Build instructions
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [docs/](docs/) - Additional documentation
  - [TUI_USER_GUIDE.md](docs/TUI_USER_GUIDE.md) - TUI dashboard guide
  - [RTX_5090_SETUP_GUIDE.md](docs/RTX_5090_SETUP_GUIDE.md) - RTX 50-series setup
  - [digital-vibrance.md](docs/digital-vibrance.md) - Vibrance implementation details

## Platform Support

| Platform | Package | Status |
|----------|---------|--------|
| **Arch Linux** | PKGBUILD | Premier |
| **Fedora** | .rpm | Tier 1 |
| **Nobara** | .rpm | Tier 1 (Gaming) |
| **Bazzite** | rpm-ostree | Tier 1 (Gaming) |
| **Pop!_OS** | .deb + COSMIC | Tier 1 |
| Debian/Ubuntu | .deb | Full |

## Requirements

- **NVIDIA Driver**: 535+ (565+ recommended, 580+ for RTX 50-series)
- **Linux Kernel**: 6.0+ (6.6+ recommended)
- **Rust**: 1.75+ (for building from source)

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Areas where help is needed:
- Compositor integrations (Hyprland HDR, COSMIC features)
- Testing on various hardware
- Documentation and translations
- Packaging for additional distros

## License

MIT License - see [LICENSE](LICENSE) for details.

Copyright (c) 2025 CK Technology LLC

## Acknowledgments

- [nvibrant](https://github.com/Tremeschin/nvibrant) - Inspiration for NVKMS vibrance implementation
- [nvidia-open-gpu-kernel-modules](https://github.com/NVIDIA/open-gpu-kernel-modules) - NVIDIA open source drivers
- The Linux gaming community

---

<div align="center">

**Made for the Linux gaming community**

[![GitHub Stars](https://img.shields.io/github/stars/GhostKellz/nvcontrol?style=social)](https://github.com/GhostKellz/nvcontrol)

</div>
