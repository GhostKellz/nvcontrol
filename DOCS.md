# nvcontrol Documentation

## Overview

nvcontrol is a modern, feature-rich NVIDIA GPU management tool designed specifically for Linux and Wayland environments. Unlike traditional tools that were designed for Windows and ported to Linux, nvcontrol is built from the ground up for the Linux ecosystem.

## Core Philosophy

### **Linux-Native Design**
- **Package Manager Integration**: Seamless installation through system package managers
- **Systemd Integration**: Proper service management and startup integration  
- **Desktop Integration**: Follows XDG standards with proper .desktop files
- **Shell Integration**: Comprehensive shell completions for power users

### **Wayland-First Architecture**
- **No X11 Dependencies**: Core functionality works entirely on Wayland
- **Compositor-Specific Optimizations**: Custom implementations for KDE, GNOME, Hyprland, Sway
- **Future-Proof Design**: Ready for the post-X11 Linux desktop

## Architecture

### **Modular Design**

```
nvcontrol/
├── src/lib.rs              # Core library with error types
├── src/gpu.rs              # GPU monitoring and NVML integration
├── src/overclocking.rs     # Advanced overclocking with safety
├── src/vrr.rs              # VRR management across compositors  
├── src/upscaling.rs        # DLSS/FSR/XeSS per-game profiles
├── src/fan.rs              # Fan control with NVML + sysfs
├── src/drivers.rs          # Package manager integration
├── src/display.rs          # Display detection and HDR
├── src/vibrance.rs         # Digital vibrance with nvibrant integration
├── src/power.rs            # GPU power management and profiles
├── src/theme.rs            # Modern UI themes
└── src/bin/
    ├── nvcontrol.rs        # GUI application (eframe/egui)
    └── nvctl.rs            # CLI tool with comprehensive commands
```

### **Technology Stack**

#### **Core Technologies**
- **Rust**: Memory-safe systems programming with excellent performance
- **NVML**: Direct NVIDIA driver integration for hardware monitoring
- **eframe/egui**: Modern immediate-mode GUI framework
- **clap**: Advanced CLI argument parsing with shell completions

#### **Linux Integration** 
- **DBUS**: Desktop environment integration
- **systemd**: Service management and startup integration
- **sysfs**: Direct hardware access for advanced features
- **Wayland Protocols**: Native compositor communication

## Features Deep Dive

### **1. GPU Monitoring & Management**

#### **Real-Time Stats**
```rust
// NVML integration for live GPU data
if let Ok(nvml) = Nvml::init() {
    if let Ok(device) = nvml.device_by_index(0) {
        let temp = device.temperature(TemperatureSensor::Gpu)?;
        let power = device.power_usage()?;
        let util = device.utilization_rates()?.gpu;
        let mem_info = device.memory_info()?;
    }
}
```

#### **TUI Dashboard**
- Live updating terminal interface with charts
- Color-coded temperature warnings
- Real-time VRAM usage tracking
- Fan speed monitoring with RPM display

### **2. Advanced Overclocking**

#### **Safety-First Approach**
```rust
pub struct OverclockProfile {
    pub gpu_clock_offset: i32,     // MHz offset from base
    pub memory_clock_offset: i32,  // MHz offset from base  
    pub power_limit: u8,           // Percentage (50-120%)
    pub temp_limit: u8,            // Temperature limit in Celsius
    pub fan_curve: Vec<(u8, u8)>,  // (temp, fan_speed) pairs
}
```

#### **Multi-Method Application**
- **X11**: nvidia-settings integration for traditional setups
- **Wayland**: Direct sysfs manipulation with proper permissions
- **Validation**: Hardware capability detection and limit enforcement

#### **Stress Testing**
- Integration with popular stress testing tools (glmark2, furmark, vkmark)
- Automatic temperature monitoring during tests
- Stability validation with automated rollback

### **3. VRR (Variable Refresh Rate) Management**

#### **Universal Compositor Support**
```rust
pub fn apply_vrr_settings(display_name: &str, settings: &VrrSettings) -> NvResult<()> {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    
    match desktop.as_str() {
        "KDE" => apply_vrr_kde(display_name, settings),
        "GNOME" => apply_vrr_gnome(settings), 
        "Hyprland" => apply_vrr_hyprland(display_name, settings),
        "sway" => apply_vrr_sway(display_name, settings),
        _ => apply_vrr_x11(display_name, settings),
    }
}
```

#### **Per-Application Profiles**
- Game-specific VRR settings that auto-apply
- Competitive gaming presets (high refresh, low latency)
- Single-player presets (quality over performance)
- Browser/desktop presets (power saving)

### **4. Upscaling Technology Management**

#### **Multi-Technology Support**
```rust
pub enum UpscalingTechnology {
    DLSS,    // NVIDIA RTX series
    FSR,     // AMD/Universal  
    XeSS,    // Intel Arc
    Native,  // No upscaling
}
```

#### **Game Integration Methods**
- **Configuration Files**: Direct modification of game config files
- **Environment Variables**: Runtime environment setup for supported games
- **Registry Manipulation**: Wine/Proton game compatibility
- **Auto-Detection**: Process monitoring for automatic profile application

### **5. Fan Control System**

#### **Multi-Layer Approach**
```rust
pub fn set_fan_speed(fan_id: usize, speed_percent: u8) -> NvResult<()> {
    // Try NVML first (enterprise drivers)
    if let Ok(nvml) = Nvml::init() { /* ... */ }
    
    // Try nvidia-settings (X11)
    if std::env::var("DISPLAY").is_ok() {
        return set_fan_speed_nvidia_settings(fan_id, speed_percent);
    }
    
    // Try direct sysfs manipulation (requires root)
    set_fan_speed_sysfs(fan_id, speed_percent)
}
```

#### **Hardware Abstraction**
- Automatic detection of controllable fans
- Safety limits based on hardware capabilities
- Temperature-based automatic curves
- Manual override with safety warnings

### **6. Driver Management**

#### **Package Manager Integration**
```bash
# Arch Linux
nvctl drivers install proprietary  # Uses pacman
nvctl drivers install open         # nvidia-open package

# Ubuntu/Debian  
nvctl drivers install proprietary  # Uses apt with proper PPA
nvctl drivers install open         # nvidia-kernel-open-dkms

# Fedora
nvctl drivers install proprietary  # Uses dnf with RPM Fusion
```

#### **DKMS Management**
- Automatic DKMS module building and installation
- Conflict resolution between driver types
- Kernel update compatibility checking
- Rollback support for failed installations

## GUI Application

### **Modern UI Design**

#### **Theme System**
```rust
pub struct ModernTheme {
    pub primary: String,      // Bright cyan for NVIDIA branding
    pub secondary: String,    // Orange accent for highlights
    pub background: String,   // Deep black background  
    pub surface: String,      // Dark gray for cards/panels
    pub glass_alpha: f32,     // Glass morphism transparency
}
```

#### **Component Architecture**
- **Tab-based Navigation**: GPU, Display, Overclock, Fan, Settings
- **Real-time Updates**: Live data refresh with configurable intervals
- **Safety Warnings**: Color-coded alerts for dangerous operations
- **Quick Actions**: One-click presets for common operations

### **Responsive Design**
- **Minimum Window Size**: 800x500 for usability
- **Scalable UI Elements**: Works on HiDPI displays
- **Keyboard Navigation**: Full accessibility support
- **Touch Support**: Ready for touchscreen Linux devices

## CLI Tools

### **Command Structure**
```bash
nvctl <module> <command> [options]

# Examples
nvctl gpu stat                    # Live monitoring
nvctl overclock apply --gpu-offset 150
nvctl vrr enable DP-1
nvctl upscaling enable cyberpunk2077 --tech dlss
nvctl drivers install open
```

### **Shell Integration**
- **Completions**: Bash, Zsh, Fish support with context-aware suggestions
- **Man Pages**: Comprehensive documentation integration
- **Exit Codes**: Proper error handling for scripting
- **JSON Output**: Machine-readable output for integration

## Configuration

### **Config File Locations**
```
~/.config/nvcontrol/config.toml    # User settings
~/.config/nvcontrol/profiles/      # Saved profiles
~/.config/nvcontrol/themes/        # Custom themes
```

### **Profile System**
```toml
[default_profile]
gpu_offset = 0
memory_offset = 0
power_limit = 100
fan_curve = [[30, 20], [70, 70], [85, 100]]

[gaming_profile]  
gpu_offset = 150
memory_offset = 800
power_limit = 115
```

## Safety & Security

### **Permission Model**
- **Minimum Privileges**: Only request necessary permissions
- **Capability Detection**: Graceful degradation when features unavailable
- **User Confirmation**: Dangerous operations require explicit confirmation
- **Logging**: Comprehensive operation logging for debugging

### **Hardware Protection**
- **Temperature Monitoring**: Automatic shutdown on overheat
- **Power Limiting**: Respect hardware power delivery limits
- **Voltage Protection**: No unsafe voltage modifications
- **Rollback Capability**: Automatic revert on system instability

## Development

### **Building**
```bash
# Debug build
cargo build

# Release build  
cargo build --release

# GUI enabled
cargo build --features gui

# All features
cargo build --features "gui,tray"
```

### **Testing**
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# CLI testing
./test_no_tray.sh
```

### **Code Standards**
- **Rust 2021 Edition**: Latest language features
- **clippy**: Strict linting for code quality
- **rustfmt**: Consistent code formatting
- **Documentation**: Comprehensive inline documentation

## Troubleshooting

### **Common Issues**

#### **NVML Not Available**
```bash
# Check driver installation
nvidia-smi
lsmod | grep nvidia

# Install proper drivers
nvctl drivers install proprietary
```

#### **Fan Control Not Working**
```bash
# Check permissions
ls -la /sys/class/hwmon/

# Try with elevated privileges
sudo nvctl fan set 0 50
```

#### **VRR Not Available**
```bash
# Check compositor support
echo $XDG_CURRENT_DESKTOP

# Enable experimental features (GNOME)
gsettings set org.gnome.mutter experimental-features "['variable-refresh-rate']"
```

### **Debug Mode**
```bash
# Enable verbose logging
RUST_LOG=debug nvctl gpu stat

# GUI debug mode
RUST_LOG=debug nvcontrol --debug
```

## Contributing

### **Development Environment**
- **Rust Stable**: Required toolchain
- **NVIDIA GPU**: For hardware testing
- **Linux Desktop**: Wayland preferred for testing
- **Multiple Distros**: Test across package managers

### **Code Organization**
- **Feature Branches**: Separate branches for major features
- **Modular Design**: Keep modules independent and testable
- **Documentation**: Update docs with code changes
- **Testing**: Include tests for new functionality

For more information, see [BUILDING.md](BUILDING.md) and [COMMANDS.md](COMMANDS.md).

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
