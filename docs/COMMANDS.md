# nvcontrol Command Reference

Complete CLI command reference for nvcontrol.

## 🌈 Digital Vibrance

### Simple Vibrance Commands
```bash
# Essential vibrance control (0-200% range)
nvctl vibe 100        # Default vibrance (100%)
nvctl vibe 150        # Enhanced colors (150%)
nvctl vibe 200        # Maximum saturation (200%)
nvctl vibe 80         # Reduced saturation (80%)
nvctl vibe 0          # Grayscale (0%)
```

### Advanced Vibrance Commands
```bash
# Current vibrance status
nvctl display vibrance get          # Show current vibrance levels
nvctl display vibrance info         # Driver capabilities & requirements

# Per-display control
nvctl display vibrance set 120      # Set all displays to 120%
nvctl display vibrance set-display 0 150  # Set specific display
nvctl display vibrance reset        # Reset to default (100%)
nvctl display vibrance list         # List available displays

# Raw vibrance values (-1024 to 1023)
nvctl display vibrance set-raw 512 -200 800  # Raw values per display
```

### Vibrance Features
- **Pure Rust Implementation**: No external dependencies (no nvibrant required)
- **NVIDIA Open Drivers**: Direct integration with 580+ drivers
- **Wayland & X11 Support**: Works on all display servers
- **Per-Display Control**: Individual settings for multi-monitor
- **Real-time Adjustment**: Instant color changes

## 🐳 Container GPU Control (nvctl ct / nvctl ctr)

### Container Launch & Management
```bash
# Basic container launch with GPU
nvctl ct launch --image nvidia/cuda:12.0-runtime-ubuntu20.04 --gpu all -i
nvctl ct launch --image tensorflow/tensorflow:latest-gpu --name ml-training

# Advanced launch options
nvctl ct launch \
  --image pytorch/pytorch:latest \
  --name pytorch-dev \
  --gpu "0,1" \
  --interactive \
  --rm \
  --runtime podman

# Container management
nvctl ct list                       # List GPU-enabled containers
nvctl ct status                     # Container GPU status
nvctl ct monitor --container ml-training --interval 5
```

### PhantomLink Audio Container
```bash
# PhantomLink Rust wavelink XLR alternative
nvctl ct phantomlink                # Launch with defaults
nvctl ct phantomlink --mode prod    # Production mode
nvctl ct phantomlink --mode dev     # Development mode
nvctl ct phantomlink --mode minimal # Minimal resources
nvctl ct phantomlink --rtx-voice    # Enable RTX Voice noise suppression
nvctl ct phantomlink --audio-device hw:0  # Specific audio device
```

### Container Profiles & Runtime
```bash
# Profile management
nvctl ct profiles list              # List available profiles
nvctl ct profiles create --name custom-ml --workload ml-training
nvctl ct profiles create --name gaming --workload gaming
nvctl ct profiles apply --profile ml-training --container ml-container

# Runtime management
nvctl ct runtime info              # Container runtime information
nvctl ct runtime test              # Test GPU passthrough capabilities
nvctl ct runtime setup --runtime docker  # Setup runtime
nvctl ct runtime configure         # Configure NVIDIA Container Runtime
```

## 🎮 GPU Commands

- `nvctl gpu info`  
  Show comprehensive GPU information (name, driver, VRAM, architecture, compute capability).

- `nvctl gpu stat`  
  Launch a beautiful live TUI dashboard for GPU monitoring with real-time graphs for:
  - Temperature (with color-coded warnings)
  - Fan speed (RPM and percentage)  
  - VRAM usage (used/total with percentage)
  - GPU utilization (with load indicator)
  - Power consumption (watts)
  - Clock speeds (base/boost/current)
  
  Press `q` to quit, `r` to refresh, `h` for help.

- `nvctl gpu capabilities`  
  Show detailed GPU overclocking capabilities and safe limits including:
  - Maximum safe GPU clock offset
  - Memory overclocking headroom
  - Power limit range
  - Temperature thresholds
  - Voltage modification support

## 🖥️ Display Commands

- `nvctl display info`  
  Show comprehensive display information:
  - Connected outputs with resolution and refresh rate
  - Display names and manufacturer info
  - Connection types (DisplayPort, HDMI, DVI)
  - Color depth and color space support
  - HDR capabilities and current status

- `nvctl display ls`  
  List all detected displays in a clean table format:
  - Display index for use in other commands
  - Display name (manufacturer + model)
  - Connection type and port
  - Current resolution and refresh rate
  - HDR status indicator

### 🌈 Vibrance Commands

- `nvctl display vibrance get`  
  Show current vibrance levels for all displays with percentage values.

- `nvctl display vibrance set <percentage>`  
  Set vibrance for all displays using percentage (0-200%, where 100% is default):
  - Example: `nvctl display vibrance set 150` (50% more saturation)

- `nvctl display vibrance set-display <display> <percentage>`  
  Set vibrance for a specific display:
  - Example: `nvctl display vibrance set-display 0 120` (20% more vibrant on display 0)

- `nvctl display vibrance set-raw <values...>`  
  Set raw nvibrant values (-1024 to 1023) for multiple displays:
  - Example: `nvctl display vibrance set-raw 512 1023` (normal first, max second)

- `nvctl display vibrance list`  
  List all available displays with their indices for use in commands.

- `nvctl display vibrance reset`  
  Reset all displays to default vibrance (100% / 0 raw value).

- `nvctl display vibrance info`  
  Show nvibrant availability and driver compatibility information.
  - Supports multiple displays simultaneously

- `nvctl display hdr status`  
  Show HDR status for all displays with detailed information:
  - HDR capability detection
  - Current HDR mode (SDR/HDR10/HDR10+/Dolby Vision)
  - Color gamut information
  - Peak brightness levels

- `nvctl display hdr enable <display_id>`  
  Enable HDR for a specific display:
  - Example: `nvctl display hdr enable 0`
  - Automatically configures color space
  - Validates HDR compatibility

- `nvctl display hdr disable <display_id>`  
  Disable HDR and return to SDR mode:
  - Example: `nvctl display hdr disable 0`
  - Preserves color calibration settings

- `nvctl display hdr toggle <display_id>`  
  Smart toggle HDR on/off for a specific display:
  - Example: `nvctl display hdr toggle 0`
  - Remembers previous state

## ⚡ Overclocking Commands

- `nvctl overclock info`  
  Show comprehensive overclocking information:
  - Current clock speeds (base/boost/effective)
  - Memory timings and bandwidth
  - Power consumption and limits
  - Temperature readings
  - Voltage information (if available)

- `nvctl overclock apply --gpu-offset <mhz> --memory-offset <mhz> --power-limit <percent>`  
  Apply comprehensive overclocking settings with safety validation:
  ```bash
  nvctl overclock apply --gpu-offset 150 --memory-offset 800 --power-limit 115
  ```
  - GPU offset: -200 to +300 MHz (varies by card)
  - Memory offset: -500 to +1500 MHz (varies by card)  
  - Power limit: 50% to 120% (hardware dependent)
  - Automatic safety validation before application

- `nvctl overclock profile <name>`  
  Apply a saved overclocking profile:
  ```bash
  nvctl overclock profile gaming     # High performance
  nvctl overclock profile quiet      # Low noise
  nvctl overclock profile extreme    # Maximum overclock
  ```

- `nvctl overclock stress-test [duration]`  
  Run comprehensive GPU stress test with monitoring:
  - Duration in minutes (default: 5, max: 60)
  - Real-time temperature monitoring
  - Automatic safety shutdown on overheat
  - Stability validation with error detection
  - Example: `nvctl overclock stress-test 15`

- `nvctl overclock reset`  
  Safely reset all overclocking settings to hardware defaults:
  - Clears GPU and memory offsets
  - Resets power limit to 100%
  - Restores default fan curves
  - Validates reset was successful

## 🔄 VRR (Variable Refresh Rate) Commands

- `nvctl vrr status`  
  Show comprehensive VRR status:
  - VRR capability for each display
  - Current VRR mode (enabled/disabled/adaptive)
  - Refresh rate range (min/max Hz)
  - Compositor-specific VRR status
  - G-SYNC/FreeSync compatibility

- `nvctl vrr enable <display>`  
  Enable VRR for a specific display with automatic configuration:
  ```bash
  nvctl vrr enable DP-1        # DisplayPort 1
  nvctl vrr enable HDMI-A-1    # HDMI port 1
  ```
  - Automatic refresh rate range detection
  - Compositor-specific implementation
  - Validation of VRR compatibility

- `nvctl vrr disable <display>`  
  Disable VRR and use fixed refresh rate:
  ```bash
  nvctl vrr disable DP-1
  ```
  - Returns to highest available refresh rate
  - Preserves display configuration

- `nvctl vrr configure <display> --min-refresh <hz> --max-refresh <hz>`  
  Advanced VRR configuration with custom refresh rate ranges:
  ```bash
  nvctl vrr configure DP-1 --min-refresh 48 --max-refresh 144
  nvctl vrr configure HDMI-A-1 --min-refresh 60 --max-refresh 120
  ```
  - Custom refresh rate ranges for optimal experience
  - Game-specific optimizations
  - Automatic validation of supported ranges

## 🚀 Upscaling (DLSS/FSR/XeSS) Commands

- `nvctl upscaling status`  
  Show comprehensive upscaling technology status:
  - DLSS support and version (RTX cards only)
  - FSR support and version (universal)
  - XeSS support and version (Intel Arc + others)
  - Currently configured games and settings
  - Hardware compatibility information

- `nvctl upscaling enable <game> --tech <technology> --quality <level>`  
  Enable upscaling for specific games with precise control:
  ```bash
  # High-end single player games
  nvctl upscaling enable cyberpunk2077 --tech dlss --quality quality
  nvctl upscaling enable metro_exodus --tech dlss --quality ultra
  
  # Competitive gaming (maximum FPS)
  nvctl upscaling enable cs2 --tech dlss --quality performance
  nvctl upscaling enable valorant --tech fsr --quality performance
  
  # Universal compatibility
  nvctl upscaling enable witcher3 --tech fsr --quality balanced
  ```
  
  **Technologies**: `dlss`, `fsr`, `xess`, `native`  
  **Quality levels**: `performance`, `balanced`, `quality`, `ultra`

- `nvctl upscaling disable <game>`  
  Disable upscaling and return to native rendering:
  ```bash
  nvctl upscaling disable cyberpunk2077
  ```
  - Removes game-specific configurations
  - Cleans up environment variables
  - Preserves other game settings

- `nvctl upscaling profiles`  
  List all configured game upscaling profiles:
  - Game names and detection methods
  - Current upscaling technology and quality
  - Profile creation date and last used
  - Performance impact estimates

- `nvctl upscaling auto-detect`  
  Intelligent game detection and profile application:
  - Monitors running processes for known games
  - Automatically applies appropriate upscaling profiles
  - Learning system for new game detection
  - Background service mode for seamless experience

## ⚡ Power Management Commands

- `nvctl power info`  
  Show comprehensive power information for all GPUs:
  - Current power draw (watts)
  - Power limits (current/default/min/max)
  - Temperature and fan speed
  - Power state (P0, P1, P2, etc.)
  - Persistence mode status

- `nvctl power profile <profile>`  
  Apply predefined power management profiles:
  ```bash
  nvctl power profile performance    # Maximum performance mode
  nvctl power profile balanced      # Balanced power/performance
  nvctl power profile power_saver   # Power saving mode
  ```
  - **Performance**: 100% power limit, persistence mode enabled
  - **Balanced**: 85% power limit, automatic performance
  - **Power Saver**: 70% power limit, minimum performance

- `nvctl power limit <percentage>`  
  Set power limit as percentage of maximum:
  ```bash
  nvctl power limit 90     # Set to 90% of maximum power
  ```
  - Range: Varies by GPU (typically 50-120%)
  - Uses nvidia-smi or sysfs fallback for Wayland
  - Requires appropriate permissions

- `nvctl power persistence <on|off>`  
  Control GPU persistence mode:
  - `on`: Keep GPU initialized for faster startup
  - `off`: Allow GPU to power down when idle
  - Useful for gaming and compute workloads

## 🌀 Fan Commands

- `nvctl fan info`  
  Show comprehensive fan information:
  - Number of controllable fans
  - Current RPM and percentage for each fan
  - Fan control capabilities (read-only vs controllable)
  - Temperature sensor locations
  - Fan curve information (if available)

- `nvctl fan set <fan_id> <percent>`  
  Set specific fan speed with safety validation:
  ```bash
  nvctl fan set 0 75      # Set first fan to 75%
  nvctl fan set 1 60      # Set second fan to 60%
  ```
  - Range: 0-100% with safety limits
  - Automatic validation of controllable fans
  - Temperature monitoring during manual control

## 🔧 Driver Management Commands

- `nvctl drivers status`  
  Comprehensive driver status information:
  - Current driver version and type (proprietary/open)
  - Available updates with version comparison
  - DKMS module status for all kernel versions
  - Loaded kernel modules and dependencies
  - Installation method and package information

- `nvctl drivers install <type>`  
  Install NVIDIA drivers with package manager integration:
  ```bash
  nvctl drivers install proprietary  # Standard NVIDIA drivers
  nvctl drivers install open         # Open-source NVIDIA drivers  
  nvctl drivers install open-beta    # Beta open-source drivers
  ```
  - Automatic package manager detection (pacman/apt/dnf)
  - Repository setup for distributions requiring it
  - Conflict resolution with existing drivers
  - DKMS integration for kernel compatibility

- `nvctl drivers update`  
  Update current driver to latest available version:
  - Automatic detection of update method
  - Package manager integration
  - DKMS rebuild for all kernel versions
  - Validation of successful update

- `nvctl drivers rollback`  
  Rollback to previous driver version (where supported):
  - Arch Linux: Full rollback using package cache
  - Other distributions: Planned support
  - Automatic DKMS cleanup
  - Configuration preservation

## 🐚 Shell Completion

- **Install shell completions** for enhanced CLI experience:
  ```bash
  # Automatic detection
  ./scripts/install-completions.sh
  
  # Manual shell specification
  ./scripts/install-completions.sh zsh
  ./scripts/install-completions.sh bash
  ./scripts/install-completions.sh fish
  ```

- **Generate completion scripts**:
  ```bash
  nvctl drivers generate-completions bash > nvctl.bash
  nvctl drivers generate-completions zsh > _nvctl
  nvctl drivers generate-completions fish > nvctl.fish
  ```

## 🎯 Real-World Examples & Use Cases

### **🎮 Gaming Setup Optimization**
```bash
# Ultimate gaming configuration
nvctl vrr enable DP-1                    # Enable VRR for main monitor
nvctl overclock apply --gpu-offset 150 --memory-offset 800 --power-limit 115
nvctl upscaling enable cyberpunk2077 --tech dlss --quality quality
nvctl fan set 0 70                       # Higher fan speed for stability

# Competitive gaming (maximum FPS)
nvctl upscaling enable cs2 --tech dlss --quality performance
nvctl overclock apply --gpu-offset 200 --memory-offset 1000
nvctl vrr configure DP-1 --min-refresh 120 --max-refresh 240
```

### **🎬 Content Creation Setup**
```bash
# Stable, reliable performance
nvctl display hdr enable 0               # Enable HDR for color accuracy
nvctl overclock apply --power-limit 90   # Lower power for quieter operation
nvctl fan set 0 40                       # Quiet fan profile
nvctl upscaling disable premiere_pro     # Native rendering for accuracy
```

### **💻 Daily Desktop Use**
```bash
# Power-efficient, quiet operation
nvctl overclock reset                    # Stock settings
nvctl fan set 0 30                       # Minimum fan speed
nvctl display vibrance 600 400           # Slight vibrance boost
nvctl vrr enable DP-1                    # Smooth desktop experience
```

### **🔧 System Maintenance**
```bash
# Check system health
nvctl gpu info                           # Verify GPU detection
nvctl drivers status                     # Check for driver updates
nvctl fan info                          # Monitor thermal status
nvctl gpu stat                          # Live monitoring dashboard

# Update and optimize
nvctl drivers update                     # Latest driver version
nvctl overclock stress-test 10          # Stability validation
```

### **🚀 Automated Workflows**
```bash
# Game launcher integration
nvctl upscaling auto-detect &            # Background game detection

# Startup optimization
nvctl vrr enable DP-1                    # Enable VRR at boot
nvctl overclock profile daily            # Apply daily use profile

# Performance monitoring
nvctl gpu stat > performance.log         # Log performance data
```

## 🔍 Advanced Usage

### **Scripting and Automation**
```bash
# Check if overclock is stable before applying
if nvctl overclock stress-test 1; then
    nvctl overclock apply --gpu-offset 200
else
    echo "Overclock unstable, using conservative settings"
    nvctl overclock apply --gpu-offset 100
fi

# Conditional VRR based on display capabilities
if nvctl vrr status | grep -q "supported"; then
    nvctl vrr enable DP-1
fi
```

### **Configuration Management**
```bash
# Export current settings
nvctl overclock info > my_overclock.profile
nvctl upscaling profiles > my_games.config

# System backup before major changes
nvctl drivers status > driver_backup.txt
```

### **Troubleshooting Commands**
```bash
# Verbose debugging
RUST_LOG=debug nvctl gpu stat

# Reset to safe defaults
nvctl overclock reset
nvctl fan set 0 auto
nvctl drivers status
```

---

## Related Documentation

For more GPU controls, see:

- [VRR/G-SYNC Control](VRR_GSYNC.md) - Variable refresh rate and G-SYNC
- [HDR Control](HDR_CONTROL.md) - High Dynamic Range display management
- [Image Sharpening](IMAGE_SHARPENING.md) - GPU image post-processing
- [Overclocking Guide](OVERCLOCKING.md) - Comprehensive overclocking reference

For detailed technical documentation, see [INDEX.md](INDEX.md). For building from source, see [BUILDING.md](BUILDING.md).

---

## License

MIT License - See [LICENSE](../LICENSE) file.
