# Building nvcontrol

This guide covers building nvcontrol from source for all supported platforms.

## Prerequisites

### Required

- **Rust 1.75+** (edition 2024)
- **Cargo** (comes with Rust)
- **NVIDIA Driver 535+** (565+ recommended, 580+ optimal for RTX 50-series)
- **Linux kernel 6.0+** (6.6+ recommended)

### Build Dependencies

#### Arch Linux (Premier Platform)
```bash
sudo pacman -S rust cargo clang pkg-config wayland libxkbcommon fontconfig freetype2
```

#### Debian/Ubuntu
```bash
sudo apt install cargo rustc libclang-dev pkg-config libwayland-dev \
    libxkbcommon-dev libfontconfig1-dev libfreetype6-dev
```

#### Fedora/Nobara
```bash
sudo dnf install rust cargo clang-devel pkgconfig wayland-devel \
    libxkbcommon-devel fontconfig-devel freetype-devel
```

#### Pop!_OS (with COSMIC)
```bash
sudo apt install cargo rustc libclang-dev pkg-config libwayland-dev \
    libxkbcommon-dev libfontconfig1-dev libfreetype6-dev
# Optional: cosmic-randr for display control
```

## Quick Build

```bash
# Clone the repository
git clone https://github.com/GhostKellz/nvcontrol
cd nvcontrol

# Build CLI only (fastest)
cargo build --release --bin nvctl

# Build GUI (requires gui feature)
cargo build --release --bin nvcontrol --features gui

# Build everything
cargo build --release --all-features
```

## Build Targets

### CLI Tool (`nvctl`)
The command-line interface for all nvcontrol features.

```bash
cargo build --release --bin nvctl
```

Output: `target/release/nvctl`

### GUI Application (`nvcontrol`)
The graphical interface with TUI dashboard.

```bash
cargo build --release --bin nvcontrol --features gui
```

Output: `target/release/nvcontrol`

## Build Options

### Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `gui` | Enable GUI application with egui | No |
| `tui` | Enable TUI dashboard with ratatui | Yes |

### Build Profiles

```bash
# Development (fast compile, debug symbols)
cargo build

# Release (optimized, slower compile)
cargo build --release

# Release with debug info
cargo build --release --profile release-with-debug
```

## Cross-Compilation

### For x86_64 (default)
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

## Installation

### Manual Installation
```bash
# Install binaries
sudo install -Dm755 target/release/nvctl /usr/bin/nvctl
sudo install -Dm755 target/release/nvcontrol /usr/bin/nvcontrol

# Install desktop file (optional)
sudo install -Dm644 assets/nvcontrol.desktop /usr/share/applications/nvcontrol.desktop
```

### Using the Install Script
```bash
./dev/install.sh
```

### Package Installation

#### Arch Linux
```bash
cd release/arch
makepkg -si
```

#### Debian/Ubuntu
```bash
# Build .deb package
dpkg-buildpackage -us -uc
sudo dpkg -i ../nvcontrol_*.deb
```

#### Fedora
```bash
rpmbuild -ba release/fedora/nvcontrol.spec
sudo rpm -i ~/rpmbuild/RPMS/x86_64/nvcontrol-*.rpm
```

## Development Build

### With Warnings
```bash
RUSTFLAGS="-W dead_code -W unused_imports -W unused_variables" cargo build
```

### Check Without Building
```bash
cargo check
cargo check --all-features
```

### Run Tests
```bash
# All tests
cargo test

# Library tests only (skip hardware tests)
cargo test --lib -- --skip hardware --skip nvml

# Specific test
cargo test test_vibrance
```

### Documentation
```bash
# Generate documentation
cargo doc --open

# With private items
cargo doc --document-private-items --open
```

## Troubleshooting

### Missing NVIDIA Drivers
```
error: NVIDIA drivers not detected
```
Install NVIDIA drivers:
```bash
# Arch
sudo pacman -S nvidia nvidia-utils

# Ubuntu
sudo apt install nvidia-driver-535

# Fedora
sudo dnf install akmod-nvidia
```

### Permission Denied on /dev/nvidia-modeset
```bash
# Add user to video group
sudo usermod -aG video $USER
# Log out and back in
```

### Clang/LLVM Not Found
```bash
# Arch
sudo pacman -S clang

# Ubuntu
sudo apt install libclang-dev

# Fedora
sudo dnf install clang-devel
```

### Wayland Libraries Missing
```bash
# Arch
sudo pacman -S wayland libxkbcommon

# Ubuntu
sudo apt install libwayland-dev libxkbcommon-dev

# Fedora
sudo dnf install wayland-devel libxkbcommon-devel
```

## Verification

After building, verify the installation:

```bash
# Check CLI
./target/release/nvctl --version
./target/release/nvctl gpu info

# Check GUI launches
./target/release/nvcontrol &

# Test vibrance (requires NVIDIA GPU)
./target/release/nvctl display vibrance list
```

## Performance Notes

- Release builds are ~10x faster than debug builds
- GUI requires ~50MB additional dependencies
- First build may take 2-5 minutes (caching speeds up subsequent builds)
- Incremental builds are typically <30 seconds

## Next Steps

- See [COMMANDS.md](COMMANDS.md) for CLI usage
- See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines
- See [docs/](docs/) for detailed documentation
