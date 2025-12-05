# Building nvcontrol

This document provides detailed instructions for building nvcontrol from source.

## Quick Start

```sh
# Clone the repository
git clone https://github.com/ghostkellz/nvcontrol.git
cd nvcontrol

# Build CLI only (minimal dependencies)
cargo build --no-default-features

# Build GUI without system tray
cargo build --features gui

# Build everything (default)
cargo build --all-features
```

## Feature Flags

nvcontrol uses Cargo features to provide flexible builds:

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `gui` | Graphical interface | GTK3, GLib, eframe/egui |
| `tray` | System tray integration | GUI + tray-icon crate |
| `default` | Both GUI and tray | All dependencies |

## Build Configurations

### CLI Only (Minimal)
Perfect for servers, CI, or minimal installations:
```sh
cargo build --no-default-features --release
```
**Result**: Only `nvctl` binary, no GUI dependencies

### GUI without System Tray
For desktop use without system tray:
```sh
cargo build --features gui --release
```
**Result**: Both `nvctl` and `nvcontrol` binaries, no tray support

### Full Build (Recommended)
Complete desktop experience:
```sh
cargo build --all-features --release
```
**Result**: Both binaries with full functionality

## System Dependencies

### Ubuntu/Debian
```sh
# For GUI builds
sudo apt-get update
sudo apt-get install \
    libgtk-3-dev \
    libglib2.0-dev \
    libgdk-pixbuf2.0-dev \
    libpango1.0-dev \
    libatk1.0-dev \
    libcairo2-dev \
    pkg-config \
    build-essential

# For CLI-only builds
sudo apt-get install pkg-config build-essential
```

### Arch Linux
```sh
# For GUI builds
sudo pacman -S gtk3 glib2 gdk-pixbuf2 pango atk cairo pkgconf base-devel

# For CLI-only builds
sudo pacman -S pkgconf base-devel
```

### Fedora/RHEL
```sh
# For GUI builds
sudo dnf install gtk3-devel glib2-devel gdk-pixbuf2-devel \
    pango-devel atk-devel cairo-devel pkgconfig gcc

# For CLI-only builds
sudo dnf install pkgconfig gcc
```

## Runtime Dependencies

### For Digital Vibrance Support
Install nVibrant for Wayland digital vibrance control:
```sh
# Using uvx (recommended)
uvx nvibrant

# Or install directly
cargo install nvibrant
```

### For HDR Support
Requires one of:
- KDE Plasma 6+ with `kscreen-doctor`
- GNOME 46+ with experimental features
- Hyprland with `hyprctl`
- Sway with `swaymsg`

## Development Setup

### Setting up the development environment
```sh
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install additional tools
rustup component add clippy rustfmt

# Clone and setup
git clone https://github.com/ghostkellz/nvcontrol.git
cd nvcontrol

# Test build
cargo check --all-features
```

### Running Tests
```
# Run library tests
cargo test --lib --no-default-features

# Run with all features
cargo test --all-features

# Deterministic backend tests (no NVIDIA hardware required)
cargo test --test test_mock_backends

# Integration tests
cargo test --test integration_tests
```

### Code Quality
```
# Format code
cargo fmt

# Run linter
cargo clippy --all-features -- -D warnings

# Check CLI-only build
cargo clippy --no-default-features -- -D warnings

# Ensure deterministic mocks stay in sync
cargo test --test test_mock_backends -- --ignored
```

## CI/CD Notes

### GitHub Actions
The project uses two workflows:

1. **CI** (`ci.yml`): Builds with `--no-default-features` to avoid GUI dependencies
2. **Release** (`release.yml`): Builds with `--all-features` on self-hosted runner

### Self-hosted Runner Requirements
For release builds on `nv-palladium`:
- NVIDIA GPU with drivers
- Full desktop environment
- All GUI dependencies installed
- System tray support

## Troubleshooting

### Common Build Issues

**Error**: `glib-sys` build failed
```
Solution: Install GUI development libraries or use --no-default-features
```

**Error**: `nvcontrol` binary missing GUI
```
Solution: Build with --features gui or --all-features
```

**Error**: System tray not working
```
Solution: Ensure desktop environment supports system tray or build with --features gui
```

### Performance Notes
- Release builds (`--release`) are significantly faster
- CLI-only builds have minimal overhead
- GUI builds require OpenGL support

## Cross-compilation

Currently not officially supported, but may work with:
```sh
# Example for different targets (untested)
cargo build --target x86_64-unknown-linux-musl --no-default-features
```

For cross-compilation, GUI features will likely require target-specific system libraries.