# nvcontrol Documentation

## Overview

nvcontrol is a modern, full-featured NVIDIA settings manager for Linux, designed for Wayland compositors (KDE, GNOME, Hyprland, Sway, etc.) and NVIDIA open drivers (>= 570). It provides both a CLI (nvctl) and an optional GUI for controlling GPU, display, color, and fan settings.

## Build Features

nvcontrol uses Cargo features to provide flexible builds:

- **`gui`** - Enables the graphical interface (eframe/egui)
- **`tray`** - Enables system tray integration
- **`default`** - Includes both `gui` and `tray` features

### Build Examples
```sh
# Full build (GUI + tray)
cargo build --all-features

# GUI only (no tray)
cargo build --features gui

# CLI only (minimal dependencies)
cargo build --no-default-features

# Release build for distribution
cargo build --release --all-features
```

---

## Deployment & CI

### Continuous Integration
The project uses GitHub Actions with two workflows:

- **CI** (`ci.yml`) - Runs on every push/PR, builds with `--no-default-features` to avoid GUI dependencies in headless environments
- **Release** (`release.yml`) - Runs on tags, builds with `--all-features` on self-hosted runner with full GUI support

### Self-Hosted Runner Requirements
The release workflow runs on `nv-palladium` with:
- NVIDIA GPU and drivers
- Full desktop environment 
- GTK3/GLib development libraries
- System tray support
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

## System Dependencies

### Runtime Dependencies
- `nvctl` (CLI): No additional dependencies beyond standard system libraries
- `nvcontrol` (GUI): GTK3, GLib, system tray support
- Digital vibrance: [nVibrant](https://github.com/Tremeschin/nVibrant) for Wayland

### Build Dependencies
```sh
# Ubuntu/Debian (for full GUI builds)
sudo apt-get install libgtk-3-dev libglib2.0-dev libgdk-pixbuf2.0-dev \
  libpango1.0-dev libatk1.0-dev libcairo2-dev pkg-config build-essential

# Minimal (for CLI-only builds)
sudo apt-get install pkg-config build-essential
```

### Distribution Support
- **Arch Linux**: AUR package (planned)
- **Ubuntu/Debian**: Manual build or download releases
- **Flatpak**: Planned
- **Self-hosted runners**: Full GUI support on nv-palladium

---

## Troubleshooting
- If vibrance does not work, ensure nVibrant is installed and in your PATH.
- For HDR, ensure you are running KDE Plasma 6+ and have a compatible monitor and driver.
- Some features may require running as root or with specific permissions.

### Build Issues
- **GUI dependencies missing**: Use `--no-default-features` for CLI-only builds
- **System tray errors**: Build with `--features gui` (excludes tray)
- **Headless CI environments**: Use `--no-default-features` in automation

### Runtime Issues
- **Permissions**: Some features may require elevated permissions
- **Missing dependencies**: Ensure all runtime dependencies are installed

---

## Roadmap
- Full fan control (curves, manual override)
- Advanced display management (resolution, refresh, orientation)
- Profile save/load and automation
- System tray widget
- More robust error handling and notifications

---

For CLI command details, see COMMANDS.md.
