# nvcontrol Documentation

## Overview

nvcontrol is a modern, full-featured NVIDIA settings manager for Linux, designed for Wayland compositors (KDE, GNOME, Hyprland, Sway, etc.) and NVIDIA open drivers (>= 570). It provides both a CLI (nvctl) and a GUI for controlling GPU, display, color, and fan settings.

---

## Supported Platforms
- **Wayland** (KDE Plasma 6+, GNOME, Hyprland, Sway, etc.)
- **NVIDIA Open Drivers** (>= 570, required for most features)
- **X11** (legacy support, some features may be limited)

---

## Key Features
- Per-display digital vibrance (via [nVibrant](https://github.com/Tremeschin/nVibrant))
- Real-time GPU monitoring (TUI and GUI)
- Fan speed monitoring and (planned) control
- ICC profile management and HDR toggle (stub)
- Profiles and automation (planned)

---

## Wayland + KDE Notes
- **Wayland is the primary target.**
- KDE Plasma 6+ is recommended for best HDR and color management support.
- Some features (e.g., vibrance, gamma, HDR) may require recent NVIDIA drivers and kernel parameters (e.g., `nvidia_drm.modeset=1`).
- nVibrant is required for digital vibrance on Wayland (see [nVibrant](https://github.com/Tremeschin/nVibrant)).

---

## Usage Examples

- Launch the GUI:
  ```sh
  nvcontrol
  ```
- Show GPU info:
  ```sh
  nvctl gpu info
  ```
- Live GPU stats (TUI):
  ```sh
  nvctl gpu stat
  ```
- List displays:
  ```sh
  nvctl display ls
  ```
- Set vibrance:
  ```sh
  nvctl display vibrance 512 1023
  ```

---

## Troubleshooting
- If vibrance does not work, ensure nVibrant is installed and in your PATH.
- For HDR, ensure you are running KDE Plasma 6+ and have a compatible monitor and driver.
- Some features may require running as root or with specific permissions.

---

## Roadmap
- Full fan control (curves, manual override)
- Advanced display management (resolution, refresh, orientation)
- Profile save/load and automation
- System tray widget
- More robust error handling and notifications

---

For CLI command details, see COMMANDS.md.
