# nvcontrol

<div align="center">
  <img src="assets/icons/nvctl_logo.png" alt="nvcontrol Logo" width="128" height="128">

  **Modern NVIDIA Settings Manager for Linux + Wayland**

  [![Rust](https://img.shields.io/badge/Rust-1.95+-orange.svg?style=for-the-badge)](https://www.rust-lang.org)
  [![NVIDIA](https://img.shields.io/badge/NVIDIA-Driver%20595+-green.svg?style=for-the-badge)](https://github.com/NVIDIA/open-gpu-kernel-modules)
  [![Wayland](https://img.shields.io/badge/Wayland-Native-brightgreen.svg?style=for-the-badge)](https://wayland.freedesktop.org/)
  [![TUI](https://img.shields.io/badge/TUI-ratatui-orange.svg?style=for-the-badge)](https://github.com/ratatui/ratatui)
  [![GUI](https://img.shields.io/badge/GUI-egui-blue.svg?style=for-the-badge)](https://github.com/emilk/egui)
  [![ASUS](https://img.shields.io/badge/ASUS-Power%20Detector+-red.svg?style=for-the-badge)](docs/hardware/power-detection.md)
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE)

  *The missing NVIDIA Control Panel for Linux*
</div>

---

> **Warning**: Overclocking and voltage modification features are experimental. Use at your own risk. Improper settings can cause system instability or hardware damage. Always monitor temperatures and power draw when adjusting GPU settings.

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

Install the latest desktop-first release:

```bash
curl -fsSL https://nvctl.cktech.sh | sudo bash
```

```bash
# GPU information (like nvidia-smi)
nvctl gpu info

# Driver capabilities and requirements
nvctl driver info

# Release diagnostics and support bundle
nvctl driver diagnose-release
nvctl doctor --support

# Validate readiness for driver 590 branch
nvctl driver validate --driver 590

# Full TUI dashboard
nvctl gpu stat

# htop-style GPU monitor
nvctl nvtop

# Digital vibrance (0-200, default 100)
nvctl vibrance 150

# Launch GUI
nvcontrol
```

## Support Workflow

```bash
# Human-readable diagnostics
nvctl driver diagnose-release
nvctl driver check

# Machine-readable diagnostics
nvctl driver diagnose-release --format json

# Shareable support artifact
nvctl driver support-bundle --tarball --redact-paths --redact-ids --log-tail 80 \
  --output ~/.local/state/nvcontrol/support/support.tar.gz

# One-shot support workflow
nvctl doctor --support --output ~/.local/state/nvcontrol/support/doctor-support.tar.gz

# TUI support actions
nvctl gpu stat   # Drivers tab: b=create bundle, x=workflow hint

# Shell completions
nvctl completion bash > nvctl.bash
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
# One-line install (downloads the latest full GUI + CLI tarball)
curl -fsSL https://nvctl.cktech.sh | sudo bash
```

### Fedora/Nobara
```bash
# One-line install (downloads the latest full GUI + CLI tarball)
curl -fsSL https://nvctl.cktech.sh | sudo bash
```

### From Source
```bash
git clone https://github.com/GhostKellz/nvcontrol
cd nvcontrol
cargo build --release --bin nvctl
cargo build --release --bin nvcontrol --features gui
sudo install -Dm755 target/x86_64-unknown-linux-gnu/release/nvctl /usr/local/bin/nvctl
sudo install -Dm755 target/x86_64-unknown-linux-gnu/release/nvcontrol /usr/local/bin/nvcontrol
```

See [docs/building.md](docs/building.md) for detailed build instructions.

## Features

### Digital Vibrance
Native implementation using NVKMS ioctls - works on Wayland without nvidia-settings.

```bash
nvctl display vibrance list          # Show all displays
nvctl vibrance 150                   # Boost colors (100=default)
nvctl display vibrance set-display 1 150
nvctl color vibrance set --value 512 -d 1
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
nvctl nvtop                          # htop-style monitor
nvctl gpu watch                      # Live monitoring
nvctl gpu watch --interval 1         # Custom refresh rate
```

### Driver Support Workflow
```bash
nvctl driver diagnose-release                       # Kernel/userspace/GSP alignment
nvctl driver diagnose-release --format json         # Structured diagnostics
nvctl driver support-bundle --gzip --redact-paths \
  --redact-ids --output ~/.local/state/nvcontrol/support/support.txt.gz
nvctl doctor --support                              # Run diagnostics + create support tarball
```

### Overclocking

> **Caution**: Overclocking is experimental. Improper settings can cause instability or hardware damage. Use at your own risk.

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

> **Caution**: Modifying power limits is experimental. Ensure adequate cooling before increasing limits.

```bash
nvctl power status                   # Power info
nvctl power limit --percentage 90    # Set power limit %
nvctl power persistence --enabled true
```

### Shell Completions
```bash
nvctl completion bash > nvctl.bash
nvctl completion zsh > _nvctl
nvctl completion fish > nvctl.fish
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
| Hyprland | Native NVKMS | hyprctl | Yes |
| Sway | Native NVKMS | swaymsg | No |
| Pop!_OS COSMIC | Native NVKMS | cosmic-randr | Yes |

## ASUS ROG Features

Native support for ASUS ROG graphics cards with vendor-specific features.

### Power Detector+ (ROG Astral/Matrix)

Monitor 12V-2x6 power connector health in real-time:

```bash
# Check power connector status
nvctl asus power

# Output:
# ASUS Power Detector+ - ROG Astral RTX 5090
# ═══════════════════════════════════════
# Connector Health: [GOOD]
# ...
```

Health status: **GOOD** (green) | **WARNING** (yellow) | **CRITICAL** (red)

### ASUS Commands

```bash
nvctl asus detect    # Detect ASUS ROG GPUs
nvctl asus power     # Power Detector+ status
nvctl asus status    # GPU overview
nvctl asus power --json  # JSON output for scripts
```

**Supported Cards:**
- ROG Astral RTX 5090
- ROG Matrix RTX 5090 (planned)

See [docs/hardware/astral-owners.md](docs/hardware/astral-owners.md) for Astral-specific guide.

## TUI Dashboard

Launch with `nvctl gpu stat`:

```
┌─ nvcontrol │ GPU 0 │ 55°C │ 85% │ LIVE ───────────────────┐
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

- [docs/](docs/) - Complete documentation hub
  - [docs/commands.md](docs/commands.md) - CLI reference
  - [docs/building.md](docs/building.md) - Build instructions
  - [docs/tui-user-guide.md](docs/tui-user-guide.md) - TUI dashboard guide
  - [docs/features/](docs/features/) - Feature guides (vibrance, HDR, VRR, etc.)
  - [docs/hardware/](docs/hardware/) - Hardware-specific guides
  - [docs/drivers/](docs/drivers/) - Driver compatibility
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines

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
- **Rust**: 1.95+ (for building from source)

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
- [Tokyo Night](https://github.com/tokyo-night/template) - Tokyo Night color scheme
- [tokyonight.nvim](https://github.com/folke/tokyonight.nvim) - Tokyo Night theme inspiration
- The Linux gaming community

---

<div align="center">

**Made for the Linux gaming community**

[![GitHub Stars](https://img.shields.io/github/stars/GhostKellz/nvcontrol?style=social)](https://github.com/GhostKellz/nvcontrol)

</div>
