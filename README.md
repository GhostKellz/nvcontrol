# nvcontrol

[![Wayland Support](https://img.shields.io/badge/Wayland-Ready-brightgreen?logo=wayland)](https://wayland.freedesktop.org/)
[![NVIDIA](https://img.shields.io/badge/NVIDIA-Supported-brightgreen?logo=nvidia)](https://nvidia.com)
[![CLI & GUI](https://img.shields.io/badge/CLI_%2B_GUI-Full_Featured-blueviolet)](#features)
[![nvibrance Integration](https://img.shields.io/badge/nvibrance-Integrated-ff69b4)](https://github.com/juv/nvidia-vibrance-ctl)
[![Crates.io](https://img.shields.io/crates/v/nvcontrol)](https://crates.io/crates/nvcontrol)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/youruser/nvcontrol/ci.yml?branch=main)](https://github.com/youruser/nvcontrol/actions)

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
  - Digital Vibrance, color profiles, gamma control (via integrated [nvibrance-ctl](https://github.com/juv/nvidia-vibrance-ctl))
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

```sh
cargo install nvcontrol
```

Or download prebuilt binaries from the [Releases](https://github.com/youruser/nvcontrol/releases) page.

### Requirements

- NVIDIA GPU with nvidia open drivers (570+) at least during testing
- Wayland compositor (KDE, GNOME, Hyprland, Sway, etc.)
- Rust (for building from source)

---

## Usage

### CLI

```sh
nvctl --help
```

### GUI

Launch via your application launcher or run:

```sh
nvcontrol
```

---

## Roadmap

- [ ] Full support for all NVIDIA GPUs (Turing, Ampere, Ada, etc.)
- [ ] Flatpak and AppImage packaging
- [ ] Localization (multi-language support)
- [ ] Plugin system for community extensions
- [ ] More advanced monitoring widgets

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

- [nvidia-vibrance-ctl](https://github.com/Tremeschin/nVibrant) for Digital Vibrance integration
- [wlroots](https://gitlab.freedesktop.org/wlroots/wlroots) and the Wayland community
- All contributors and testers

