# Building nvcontrol

This guide covers building nvcontrol from source for all supported platforms.

## Prerequisites

### Required

- **Rust 1.98+** (edition 2024)
- **Cargo** (comes with Rust)
- **NVIDIA Driver** compatible with your target nvcontrol build
- **Linux kernel 6.0+** (6.6+ recommended)

Use [drivers/nvidia-driver.md](drivers/nvidia-driver.md) as the source of truth for mapping NVIDIA driver branches to the correct nvcontrol version.

The repository pins its compiler in `rust-toolchain.toml`. Distro Rust packages
may be older; check the compiler before building. If rustup supplies Rust, Debian
package dependency checks still require matching package metadata; the acceptance
run checked all other dependencies separately before bypassing that check.

### Build Dependencies

#### Arch Linux (Premier Platform)
```bash
sudo pacman -S rust clang pkg-config wayland libxkbcommon fontconfig freetype2
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

Native Linux host builds write artifacts to `target/release/` by default. Only explicit `--target ...`
builds use a target-triple subdirectory such as `target/x86_64-unknown-linux-gnu/release/`.

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
| `gui` | Enable GUI application with egui | Yes |

The TUI is included independently; there is no `tui` feature flag. Use
`--no-default-features --bin nvctl` for a CLI-only build.

### Build Profiles

```bash
# Development (fast compile, debug symbols)
cargo build

# Release (optimized, slower compile)
cargo build --release

# Release with debug info
CARGO_PROFILE_RELEASE_DEBUG=true cargo build --release
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
# Stage the Debian recipe in the source checkout
cp -a release/deb debian
dpkg-checkbuilddeps
dpkg-buildpackage -b -us -uc
sudo dpkg -i ../nvcontrol_*.deb
```

#### Fedora
```bash
# Requires rpmdevtools; downloads the source referenced by the spec
rpmdev-setuptree
spectool -g -R release/fedora/nvcontrol.spec
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
sudo pacman -S nvidia-open-dkms nvidia-utils

# Ubuntu
sudo apt install nvidia-driver-610-open

# Fedora
sudo dnf install akmod-nvidia
```

For nvcontrol's current primary path, prefer the NVIDIA 615 open kernel module packages where your distribution provides them.

The current release retains native 595/610 support; 590 uses the separate legacy path. See the driver matrix.

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
./target/release/nvctl display vibrance get
```

## Next Steps

- See [commands.md](commands.md) for CLI usage
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines
