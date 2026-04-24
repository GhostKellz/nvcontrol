# nvcontrol Documentation

Complete documentation for nvcontrol - Modern NVIDIA Settings Manager for Linux + Wayland.

## Quick Start

**New Users:**
1. [README](../README.md) - Project overview and installation
2. [Building](building.md) - Build from source
3. [Commands](commands.md) - Current CLI reference
4. [TUI Guide](tui-user-guide.md) - Terminal interface quickstart

**RTX 50-series Users:**
- [RTX 5090 Setup](hardware/rtx-5090-setup.md) - Blackwell-specific setup

---

## Documentation Index

### Getting Started

| Document | Description |
|----------|-------------|
| [building.md](building.md) | Build from source, dependencies, feature flags |
| [commands.md](commands.md) | Complete CLI command reference |
| [tui-user-guide.md](tui-user-guide.md) | Terminal UI walkthrough |

### Features

GPU display and performance features.

| Document | Description |
|----------|-------------|
| [features/vibrance.md](features/vibrance.md) | Digital vibrance (color saturation) |
| [features/hdr.md](features/hdr.md) | High Dynamic Range display |
| [features/vrr-gsync.md](features/vrr-gsync.md) | Variable Refresh Rate / G-SYNC |
| [features/image-sharpening.md](features/image-sharpening.md) | GPU post-processing |
| [features/overclocking.md](features/overclocking.md) | GPU/memory clock tuning |

### Drivers

NVIDIA driver compatibility and optimization.

| Document | Description |
|----------|-------------|
| [drivers/legacy.md](drivers/legacy.md) | Support for drivers 590 and earlier |
| [drivers/595-abi-changes.md](drivers/595-abi-changes.md) | Driver 595 NVKMS API changes |
| [drivers/gsp.md](drivers/gsp.md) | GPU System Processor firmware |
| [drivers/diagnose-release.md](drivers/diagnose-release.md) | How to read release diagnostics |
| [drivers/dkms.md](drivers/dkms.md) | Dynamic Kernel Module Support |
| [drivers/open-590.md](drivers/open-590.md) | NVIDIA Open driver 590 |
| [drivers/kernel-580.md](drivers/kernel-580.md) | Kernel driver 580+ optimizations |

### Hardware

GPU-specific setup guides.

| Document | Description |
|----------|-------------|
| [hardware/rtx-5090-setup.md](hardware/rtx-5090-setup.md) | RTX 5090 (Blackwell) setup |
| [hardware/asus-astral.md](hardware/asus-astral.md) | ASUS ROG Astral/Matrix features |
| [hardware/astral-owners.md](hardware/astral-owners.md) | ASUS Astral tips |
| [hardware/power-detection.md](hardware/power-detection.md) | Power connector detection |

### Commands

Detailed command documentation.

| Document | Description |
|----------|-------------|
| [commands/gpu.md](commands/gpu.md) | GPU info and monitoring |
| [commands/driver.md](commands/driver.md) | Driver info, GSP, DKMS, release diagnostics |
| [commands/power.md](commands/power.md) | Power management |
| [commands/overclock.md](commands/overclock.md) | Overclocking controls |
| [commands/gaming.md](commands/gaming.md) | Gaming profiles |
| [commands/config.md](commands/config.md) | Configuration management |
| [commands/container.md](commands/container.md) | Container GPU passthrough |

### API Reference

Rust library API documentation.

| Document | Description |
|----------|-------------|
| [api/gpu.md](api/gpu.md) | GPU monitoring API |
| [api/power.md](api/power.md) | Power management API |
| [api/overclock.md](api/overclock.md) | Overclocking API |
| [api/fan.md](api/fan.md) | Fan control API |
| [api/display.md](api/display.md) | Display management API |
| [api/backend.md](api/backend.md) | Backend abstraction |

### Configuration

| Document | Description |
|----------|-------------|
| [config/backend-architecture.md](config/backend-architecture.md) | Backend design |
| [config/migration.md](config/migration.md) | Version upgrade guide |
| [config/session-persistence.md](config/session-persistence.md) | TUI state saving |

### Integration

| Document | Description |
|----------|-------------|
| [integration/nvhud.md](integration/nvhud.md) | NVHUD overlay integration |
| [integration/companion.md](integration/companion.md) | Lightweight desktop companion flow |
| [integration/issue-reporting.md](integration/issue-reporting.md) | Driver/GSP issue reporting workflow |
| [integration/support-bundle-sample.md](integration/support-bundle-sample.md) | Redacted support bundle example |
| [release-checklist.md](release-checklist.md) | Final release verification checklist |

### Experimental

Prototype integrations and parked code live outside `docs/` in [`../experimental/README.md`](../experimental/README.md).

---

## GPU Support Matrix

| Architecture | Example GPUs | Status |
|--------------|--------------|--------|
| **Blackwell** | RTX 5060-5090 | Full support |
| **Ada Lovelace** | RTX 4060-4090 | Full support |
| **Ampere** | RTX 3060-3090 Ti | Full support |
| **Turing** | RTX 2060-2080 Ti | Full support |
| **Pascal** | GTX 1060-1080 Ti | Basic support |

## Platform Support

**Linux Distributions** (Tier 1):
- Arch Linux (premier platform)
- Fedora 39+ / Nobara / Bazzite
- Debian 12+ / Ubuntu 22.04+ / Pop!_OS

**Display Servers:**
- Wayland (KDE, GNOME, Hyprland, Sway)
- X11 (full compatibility)

---

## Quick Examples

```bash
# GPU monitoring
nvctl gpu info
nvctl gpu stat
nvctl nvtop

# Digital vibrance
nvctl vibe 150

# Overclocking
nvctl overclock apply --gpu-offset 150 --memory-offset 500

# Power management
nvctl power limit --percentage 90

# Create support bundle
nvctl doctor --support
```

---

## Resources

- [Contributing](../CONTRIBUTING.md) - Development guidelines
- [Changelog](../CHANGELOG.md) - Version history
- [GitHub Issues](https://github.com/ghostkellz/nvcontrol/issues)
