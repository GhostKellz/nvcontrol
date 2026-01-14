# nvctl Command Reference

The complete CLI reference for nvctl - your NVIDIA GPU control center.

```
nvctl [OPTIONS] <COMMAND>
```

**Global Options:**
- `-v, --verbose` - Enable verbose output
- `--format <human|json|table>` - Output format
- `--no-color` - Disable colored output
- `-h, --help` - Print help
- `-V, --version` - Print version

---

## Quick Reference

| Command | Description |
|---------|-------------|
| `nvctl gpu info` | GPU information |
| `nvctl driver info` | Driver status (version, kernel, GSP, DKMS) |
| `nvctl driver check` | Driver health checks + legacy GPU warnings |
| `nvctl dlss status` | DLSS capabilities (2/3/3.5/4/4.5) |
| `nvctl hdr status` | HDR status across displays |
| `nvctl wayland explicit-sync status` | Explicit sync support |
| `nvctl tui` | Interactive TUI menu |
| `nvctl nvtop` | Real-time GPU monitor |
| `nvctl doctor` | System diagnostics |
| `nvctl fan info` | Fan status |
| `nvctl power status` | Power status |
| `nvctl config show` | Current configuration |

---

## Core Commands

### nvctl gpu
GPU information and control.

```bash
nvctl gpu info              # Comprehensive GPU info (name, driver, VRAM, arch)
nvctl gpu stat              # Live TUI dashboard with real-time graphs
nvctl gpu capabilities      # Overclocking limits and capabilities
```

### nvctl tui
Launch interactive TUI with full dashboard.

```bash
nvctl tui                   # Main TUI menu
```

**TUI Modes:**
- **nvtop** - Real-time GPU monitoring (htop-style)
- **dashboard** - Full control panel with tabs

**Keyboard:**
- `q` - Quit
- `?` - Help
- `t` - Cycle themes
- `1-9` - Jump to tab
- `Tab` - Next GPU

### nvctl nvtop
GPU monitor (htop-style).

```bash
nvctl nvtop                 # Launch real-time GPU monitor
```

### nvctl doctor
Run system diagnostics.

```bash
nvctl doctor                # Check GPU, drivers, and system health
```

### nvctl system
System information and platform detection.

```bash
nvctl system info           # Distro, compositor, driver info
nvctl system compositor     # Detected Wayland compositor
nvctl system distro         # Linux distribution
nvctl system optimize       # Platform optimization recommendations
```

### nvctl version
Show detailed version information.

```bash
nvctl version               # Version, build info, features
```

---

## Display & Visuals

### nvctl display
Display and monitor management.

```bash
nvctl display info          # Display info (resolution, refresh, HDR)
nvctl display ls            # List all displays
```

### nvctl vibrance
Digital Vibrance control (0-200%).

```bash
nvctl vibrance get                    # Current vibrance levels
nvctl vibrance set <percent>          # Set all displays (0-200%)
nvctl vibrance set-display <id> <pct> # Set specific display
nvctl vibrance reset                  # Reset to default (100%)
nvctl vibrance list                   # List available displays
nvctl vibrance info                   # Driver compatibility info
```

**Quick Examples:**
```bash
nvctl vibrance set 150      # Enhanced colors (+50%)
nvctl vibrance set 200      # Maximum saturation
nvctl vibrance set 80       # Reduced saturation
nvctl vibrance set 0        # Grayscale
```

### nvctl color
Color and vibrance control (alias).

```bash
nvctl color profile list    # List color profiles
nvctl color profile apply   # Apply color profile
```

### nvctl monitors
Multi-monitor management.

```bash
nvctl monitors list         # List all monitors
nvctl monitors layout       # Show current layout
nvctl monitors arrange      # Arrange monitors
```

---

## Performance & Overclocking

### nvctl overclock
GPU overclocking and performance.

```bash
nvctl overclock info        # Current clocks, power, temps
nvctl overclock apply \
  --gpu-offset <mhz> \
  --memory-offset <mhz> \
  --power-limit <percent>   # Apply overclock settings
nvctl overclock profile <name>  # Apply saved profile
nvctl overclock stress-test <minutes>  # Stability test
nvctl overclock reset       # Reset to defaults
```

**Examples:**
```bash
# Gaming overclock
nvctl overclock apply --gpu-offset 150 --memory-offset 800 --power-limit 115

# Apply preset
nvctl overclock profile gaming
nvctl overclock profile quiet
nvctl overclock profile extreme

# Stability test (10 minutes)
nvctl overclock stress-test 10
```

### nvctl fan
Fan control and curves.

```bash
nvctl fan info              # Fan RPM, percentage, capabilities
nvctl fan set <id> <pct>    # Set fan speed (0-100%)
nvctl fan curve apply <profile>  # Apply fan curve
nvctl fan curve show        # Show current curve
nvctl fan auto              # Return to automatic control
```

**Examples:**
```bash
nvctl fan set 0 75          # First fan to 75%
nvctl fan curve apply gaming  # Aggressive cooling curve
nvctl fan auto              # Automatic fan control
```

### nvctl power
Power management.

```bash
nvctl power info            # Power draw, limits, state
nvctl power status          # Current power status
nvctl power profile <name>  # Apply power profile
nvctl power limit <percent> # Set power limit
nvctl power persistence <on|off>  # GPU persistence mode
```

**Profiles:**
```bash
nvctl power profile performance   # Maximum performance
nvctl power profile balanced      # Balanced
nvctl power profile power_saver   # Power saving
```

### nvctl power-profile
Power profile management (AC/Battery, Activities).

```bash
nvctl power-profile list    # List profiles
nvctl power-profile apply   # Apply profile
nvctl power-profile status  # Current status
```

---

## Gaming & Optimization

### nvctl gaming
Gaming optimization and latency.

```bash
nvctl gaming enable         # Enable gaming optimizations
nvctl gaming disable        # Disable optimizations
nvctl gaming status         # Current status
nvctl gaming latency status # Latency info (CPU scheduler, etc.)
nvctl gaming latency enable # Enable low-latency mode
nvctl gaming gamescope      # Gamescope controls
nvctl gaming launch <profile>  # Launch with profile
nvctl gaming auto           # Auto-apply profiles
```

**Latency Status Output:**
```json
{
  "nvidia_reflex_available": false,
  "current_cpu_scheduler": "EEVDF",
  "gpu_scheduling_enabled": true,
  "preemption_timeout": null
}
```

### nvctl vrr
Variable Refresh Rate (VRR/G-Sync).

```bash
nvctl vrr status            # VRR capability per display
nvctl vrr enable <display>  # Enable VRR
nvctl vrr disable <display> # Disable VRR
nvctl vrr configure <display> \
  --min-refresh <hz> \
  --max-refresh <hz>        # Custom refresh range
```

**Examples:**
```bash
nvctl vrr status
nvctl vrr enable DP-1
nvctl vrr configure DP-1 --min-refresh 48 --max-refresh 144
```

### nvctl upscaling
AI Upscaling (DLSS/FSR/XeSS).

```bash
nvctl upscaling status      # Technology support status
nvctl upscaling enable <game> \
  --tech <dlss|fsr|xess> \
  --quality <level>         # Enable for game
nvctl upscaling disable <game>  # Disable upscaling
nvctl upscaling profiles    # List configured games
nvctl upscaling auto-detect # Background game detection
```

**Quality Levels:** `performance`, `balanced`, `quality`, `ultra`

**Examples:**
```bash
# Single-player (quality)
nvctl upscaling enable cyberpunk2077 --tech dlss --quality quality

# Competitive (max FPS)
nvctl upscaling enable cs2 --tech dlss --quality performance
```

### nvctl dlss
DLSS 3 Frame Generation.

```bash
nvctl dlss status           # DLSS support status
nvctl dlss enable           # Enable DLSS
nvctl dlss disable          # Disable DLSS
```

### nvctl osd
On-Screen Display (MangoHud).

```bash
nvctl osd enable            # Enable OSD overlay
nvctl osd disable           # Disable OSD
nvctl osd status            # Current status
nvctl osd config            # Configure settings
nvctl osd add <metric>      # Add metric to display
nvctl osd remove <metric>   # Remove metric
nvctl osd metrics           # List available metrics
nvctl osd check             # Check MangoHud installation
```

### nvctl shaders
Shader cache management.

```bash
nvctl shaders stats         # Cache statistics
nvctl shaders clear         # Clear all caches
nvctl shaders optimize      # Optimize compilation
nvctl shaders precompile <game>  # Precompile for game
nvctl shaders open          # Open cache folder
```

---

## Recording & Streaming

### nvctl recording
NVENC recording and streaming.

```bash
nvctl recording start       # Start recording
nvctl recording stop        # Stop recording
nvctl recording status      # Recording status
nvctl recording instant-replay  # Start instant replay
nvctl recording save        # Save replay clip
nvctl recording presets     # List presets
```

---

## Containers & Virtualization

### nvctl container
Container GPU control.

```bash
nvctl container list        # List GPU containers
nvctl container status      # Container GPU status
nvctl container launch \
  --image <image> \
  --gpu <all|0,1> \
  --name <name>             # Launch container
nvctl container monitor     # Monitor container
nvctl container profiles    # Profile management
nvctl container runtime     # Runtime configuration
```

**Examples:**
```bash
# Launch CUDA container
nvctl container launch --image nvidia/cuda:12.0-runtime --gpu all -i

# PyTorch with specific GPUs
nvctl container launch --image pytorch/pytorch:latest --gpu 0,1 --name ml-train
```

### nvctl passthrough
GPU Passthrough (VFIO/VMs).

```bash
nvctl passthrough status    # Passthrough status
nvctl passthrough list      # GPUs and PCI addresses
nvctl passthrough iommu     # IOMMU groups
nvctl passthrough bind-vfio <gpu>    # Bind to VFIO
nvctl passthrough unbind-vfio <gpu>  # Unbind from VFIO
nvctl passthrough persistent # Persistent VFIO binding
nvctl passthrough test-container     # Test Docker passthrough
nvctl passthrough qemu-command       # Generate QEMU command
nvctl passthrough hugepages # Setup hugepages
```

---

## Drivers & System

### nvctl driver
Unified driver management, status, and kernel modules.

```bash
# Status & Information
nvctl driver info           # Comprehensive status (GPU, driver, kernel, GSP, DKMS)
nvctl driver info --paste   # Compact output for Discord/forums
nvctl driver check          # Health checks with warnings
nvctl driver capabilities   # Driver feature requirements

# Validation
nvctl driver validate --driver <major>  # Validate system for driver version

# Installation & Updates
nvctl driver install <type> # Install driver (proprietary, open, open-beta)
nvctl driver update         # Update to latest version
nvctl driver rollback       # Rollback (Arch Linux only)

# Kernel Logs
nvctl driver logs --filter nvidia   # All nvidia kernel logs
nvctl driver logs --filter gsp      # GSP-specific logs
nvctl driver logs --filter xid      # Xid errors (GPU faults)
nvctl driver logs --tail 50         # Last 50 lines
```

**Install Types:** `proprietary`, `open`, `open-beta`

#### nvctl driver dkms
DKMS kernel module management for nvidia-open.

```bash
nvctl driver dkms status    # Detailed status for all kernels
nvctl driver dkms setup     # Set up DKMS for nvidia-open
nvctl driver dkms build     # Build for all kernels
nvctl driver dkms build --kernel <ver>  # Build for specific kernel
nvctl driver dkms build --force  # Force rebuild even if installed
nvctl driver dkms logs      # Show build logs (summary)
nvctl driver dkms logs -t 50  # Show last 50 lines of each log
nvctl driver dkms logs -k <ver>  # Logs for specific kernel
nvctl driver dkms unregister  # Remove nvidia from DKMS
nvctl driver dkms hook      # Install pacman hook (with logging + notifications)
nvctl driver dkms fix       # Attempt to fix DKMS issues
nvctl driver dkms cleanup   # Remove old kernel modules (dry run)
nvctl driver dkms cleanup --keep 3  # Keep 3 most recent kernels
nvctl driver dkms cleanup --execute  # Actually remove old modules
```

**Example Output - `nvctl driver dkms status`:**
```
NVIDIA DKMS Status
══════════════════════════════════════════════════

DKMS:           installed
Driver:         590.48.01
Registered:     yes
Source:         /usr/src/nvidia-590.48.01
Source Type:    git (https://github.com/NVIDIA/open-gpu-kernel-modules.git)

Installed Kernels (4):
  ✓ 6.18.1-zen1-2-zen [nvidia: dkms, headers: ✓]
  ✓ 6.18.2-1-cachyos-lto [nvidia: dkms, headers: ✓] (running)
  ✓ 6.18.2-273-tkg-linux-ghost [nvidia: dkms, headers: ✓]
  ✗ 6.18.1-1-cachyos-lto [nvidia: MISSING, headers: ✗]

Pacman Hook:    installed (auto-rebuild enabled)
```

**Source Types:**
- `packaged (nvidia-open-dkms)` - Installed via package manager
- `git (url)` - From git clone of open-gpu-kernel-modules
- `manual` - Manually copied to /usr/src

#### nvctl driver source
Build nvidia-open from source (git clone workflow).

```bash
nvctl driver source status  # Show source build status and git info
nvctl driver source init <path>  # Initialize from git clone
nvctl driver source update  # Git fetch + checkout latest tag + rebuild
nvctl driver source update --no-build  # Update without rebuilding
nvctl driver source sync    # Rebuild from current source
nvctl driver source sync --force  # Force rebuild
nvctl driver source sync --kernel <ver>  # Build specific kernel only
```

#### Legacy GPU Support
Driver 590+ deprecates Maxwell and Pascal GPUs. `nvctl driver check` will warn you.

**Affected GPUs:**
- **Maxwell (GTX 9xx)**: Deprecated, use `nvidia-470xx-dkms` from AUR
- **Pascal (GTX 10xx)**: Deprecated, use `nvidia-535xx-dkms` from AUR
- **Kepler (GTX 6xx/7xx)**: Unsupported, use `nvidia-390xx-dkms` from AUR

**RTX 50 Series (Blackwell):** Requires nvidia-open driver (no proprietary option).

**Example - Setting up from source:**
```bash
# Clone the repo
git clone https://github.com/NVIDIA/open-gpu-kernel-modules.git
cd open-gpu-kernel-modules
git checkout 590.48.01

# Initialize (creates dkms.conf, symlink, registers with DKMS)
nvctl driver source init ~/open-gpu-kernel-modules

# Build modules
nvctl driver source sync

# Later, update to new version
nvctl driver source update
```

**Example Output - `nvctl driver source status`:**
```
NVIDIA Source Build Status
══════════════════════════════════════════════════

Source Path:    /usr/src/nvidia-590.48.01
Source Type:    git (https://github.com/NVIDIA/open-gpu-kernel-modules.git)
Remote URL:     https://github.com/NVIDIA/open-gpu-kernel-modules.git
Current Tag:    590.48.01
Latest Tag:     590.48.01

Driver Version: 590.48.01
DKMS Registered: yes
```

#### nvctl driver gsp
GSP firmware management (nvidia-open only).

```bash
nvctl driver gsp status       # GSP firmware status
nvctl driver gsp enable       # Enable GSP firmware
nvctl driver gsp disable      # Disable (fallback mode)
nvctl driver gsp diagnostics  # Run GSP diagnostics
nvctl driver gsp explain      # Learn about GSP (what/why/issues)
nvctl driver gsp check-update # Check for updates
nvctl driver gsp update       # Update GSP firmware
```

**Example Output - `nvctl driver gsp status`:**
```
NVIDIA GSP Firmware Status

GSP Enabled:    Yes
GSP State:      loaded
Version:        590.48.01
Firmware:       /lib/firmware/nvidia/gb202/gsp
GPU Arch:       gb202
```

**Example Output - `nvctl driver info`:**
```
Driver Information
══════════════════════════════════════════════════

GPU:            NVIDIA GeForce RTX 5090 [00000000:01:00.0]
Driver:         590.48.01 (nvidia-open)
Module Type:    Open Kernel (Dual MIT/GPL)
Built By:       chris@arch (Thu Dec 18 08:13:20 PM EST 2025)

Kernel:         6.18.2-1-cachyos-lto (running)
Module For:     6.18.2-1-cachyos-lto ✓

GSP:            enabled (loaded)
GSP Firmware:   590.48.01
                /lib/firmware/nvidia/gb202/gsp
GPU Arch:       gb202

DKMS:           Not managed (manually installed)

Modules Loaded: nvidia_drm nvidia_uvm nvidia_modeset nvidia
```

**Example Output - `nvctl driver check`:**
```
Driver Health Check
══════════════════════════════════════════════════

Passed:
  ✓ Kernel version matches module
  ✓ Kernel headers installed
  ✓ All 4 installed kernels have nvidia modules
  ✓ GSP firmware present for nvidia-open

Warnings:
  ⚠️  DKMS installed but nvidia not registered

GSP Firmware:
  ✓ nvidia-open driver: Using nvidia-open kernel module
  ✓ GSP enabled: GSP firmware is enabled
  ✓ GSP initialization: GSP firmware loaded
  ✓ GSP firmware files: Found at /lib/firmware/nvidia/gb202/gsp

──────────────────────────────────────────────────
⚠️  1 warning(s), no errors
```

### nvctl arch
Arch Linux integration.

```bash
nvctl arch                  # Pacman hooks, DKMS management
```

### nvctl wayland
Wayland NVIDIA optimization.

```bash
nvctl wayland status        # Wayland NVIDIA configuration status
nvctl wayland optimize      # Apply optimal Wayland configuration
nvctl wayland export-env    # Generate environment variables script
nvctl wayland switch-driver # Switch between nvidia-open and nvidia-dkms
```

#### Explicit Sync
Explicit sync eliminates tearing on NVIDIA Wayland.

```bash
nvctl wayland explicit-sync status   # Check explicit sync support
nvctl wayland explicit-sync enable   # Enable in compositor
```

**Requirements:**
- Driver 555+ (560+ recommended)
- Kernel 6.1+ with DRM syncobj support
- Compositor with explicit sync: KDE Plasma 6.1+, GNOME 46+, Hyprland 0.39+

### nvctl hdr
HDR control and configuration.

```bash
nvctl hdr status            # HDR status across displays
nvctl hdr enable            # Enable HDR on all displays
nvctl hdr disable           # Disable HDR on all displays
nvctl hdr config            # Show HDR configuration
nvctl hdr set-brightness    # Set peak brightness (nits)
nvctl hdr tools             # HDR tools and game recommendations
nvctl hdr capabilities      # Display HDR capabilities (EDID)
```

**Per-display control:** Use `nvctl display hdr enable <id>` for specific displays.

### nvctl kde
KDE Plasma compositor optimization.

```bash
nvctl kde                   # KDE Plasma optimizations
```

---

## Configuration

### nvctl config
Configuration and profiles.

```bash
nvctl config show           # Current configuration
nvctl config edit           # Edit config file
nvctl config reset          # Reset to defaults
nvctl config backup         # Backup configuration
nvctl config restore        # Restore from backup
nvctl config export         # Export GPU profile
nvctl config import         # Import GPU profile
nvctl config profiles       # List profiles
```

### nvctl interactive
Interactive menu mode.

```bash
nvctl interactive           # Launch interactive menu
```

---

## ASUS ROG Features

### nvctl asus
ASUS ROG GPU features.

```bash
nvctl asus detect           # Detect ASUS ROG GPUs
nvctl asus power            # Power Detector+ status (12V monitoring)
nvctl asus status           # GPU Tweak-style status
nvctl asus aura             # Aura RGB control
```

---

## Real-World Examples

### Gaming Setup
```bash
# Ultimate gaming config
nvctl vrr enable DP-1
nvctl overclock apply --gpu-offset 150 --memory-offset 800 --power-limit 115
nvctl upscaling enable cyberpunk2077 --tech dlss --quality quality
nvctl fan set 0 70
nvctl gaming enable

# Competitive gaming (max FPS)
nvctl upscaling enable cs2 --tech dlss --quality performance
nvctl overclock apply --gpu-offset 200 --memory-offset 1000
```

### Content Creation
```bash
# Stable, quiet operation
nvctl power profile balanced
nvctl fan set 0 40
nvctl overclock reset
```

### Daily Desktop
```bash
# Power efficient
nvctl overclock reset
nvctl fan auto
nvctl vrr enable DP-1
nvctl vibrance set 120
```

### System Maintenance
```bash
# Health check
nvctl doctor
nvctl gpu info
nvctl driver info
nvctl driver check
nvctl fan info

# Stress test
nvctl overclock stress-test 10
```

### Scripting
```bash
# Conditional overclock
if nvctl overclock stress-test 1; then
    nvctl overclock apply --gpu-offset 200
else
    nvctl overclock apply --gpu-offset 100
fi

# JSON output for parsing
nvctl gpu info --format json | jq '.name'
nvctl gaming latency status --format json
```

---

## Shell Completions

```bash
# Generate completions
nvctl drivers generate-completions bash > nvctl.bash
nvctl drivers generate-completions zsh > _nvctl
nvctl drivers generate-completions fish > nvctl.fish

# Install (bash)
sudo mv nvctl.bash /etc/bash_completion.d/nvctl

# Install (zsh)
mv _nvctl ~/.zsh/completions/

# Install (fish)
mv nvctl.fish ~/.config/fish/completions/
```

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG=debug` | Enable debug logging |
| `NO_COLOR=1` | Disable colors |
| `NVCTL_CONFIG` | Custom config path |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Permission denied |
| 4 | GPU not found |
| 5 | Driver error |

---

## See Also

- [DKMS Integration](DKMS.md)
- [GSP Firmware](GSP.md)
- [VRR/G-SYNC Control](VRR_GSYNC.md)
- [HDR Control](HDR_CONTROL.md)
- [Overclocking Guide](OVERCLOCKING.md)
- [NVIDIA 590 Driver](NVIDIA_OPEN_590.md)
- [Building from Source](BUILDING.md)

---

**License:** MIT - See [LICENSE](../LICENSE)
