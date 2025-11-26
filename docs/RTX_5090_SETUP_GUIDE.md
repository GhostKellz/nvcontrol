# ASUS ROG Astral RTX 5090 - Complete Setup Guide

**Your Upgrade:** RTX 4090 → ASUS ROG Astral GeForce RTX 5090 (32GB GDDR7)
**System:** Arch Linux + Wayland + NVIDIA Open Kernel Modules 580.105.08

---

## Table of Contents

1. [Pre-Installation Checklist](#pre-installation-checklist)
2. [System Requirements](#system-requirements)
3. [Installation Steps](#installation-steps)
4. [Post-Installation Validation](#post-installation-validation)
5. [nvcontrol Configuration](#nvcontrol-configuration)
6. [Kernel Module Optimization](#kernel-module-optimization)
7. [ASUS Astral Specific Features](#asus-astral-specific-features)
8. [Performance Tuning](#performance-tuning)
9. [Troubleshooting](#troubleshooting)

---

## Pre-Installation Checklist

### ✅ Your System is Already Ready!

Your Arch system is **ALREADY CONFIGURED** for RTX 5090:

```
✅ Resizable BAR: ENABLED (32 GB)
✅ Above 4G Decoding: ENABLED
✅ PCIe Gen 4: x16 @ 16 GT/s (fully compatible with Gen 5)
✅ Open GPU Kernel Modules: 580.105.08 (latest, with Blackwell support)
```

### Critical Pre-Checks

#### 1. **Power Supply (PSU)**
- **Required:** 1000W or higher
- **Connector:** 12V-2x6 (12VHPWR)
- **Your 4090:** 450W TDP, 850W PSU recommended
- **ASUS Astral 5090:** 600W TDP (630W max), **1000W PSU recommended**

**Action:** Verify your current PSU wattage and connector availability.

```bash
# Check current PSU info (if monitoring is available)
sensors | grep power
```

#### 2. **Case Clearance**
- **GPU Dimensions:** 357.6 x 149.3 x 76 mm
- **Length:** 357mm (14.1 inches)
- **Width:** 3.8 slots (76mm thick)
- **Weight:** ~2.5kg

**Action:** Measure your case to ensure:
- GPU bay supports 357mm+ length
- 3.8 PCIe slot clearance (likely blocks adjacent slots)
- No RAM/cable interference

#### 3. **Secure Boot Status**

⚠️ **IMPORTANT:** Some users reported boot failures with RTX 5090 when **both Secure Boot AND TPM** are enabled.

```bash
# Check Secure Boot status
mokutil --sb-state

# If enabled and you encounter boot issues after installation:
# Disable Secure Boot in BIOS
```

**Reference:** [NVIDIA Forums - Linux boot failures with RTX 5090](https://forums.developer.nvidia.com/t/linux-fails-to-boot-with-secure-boot-tpm-enabled-nvidia-rtx-5090-bar-allocation-errors/343254)

#### 4. **Driver Version**

```bash
# Check current driver version
nvidia-smi --query-gpu=driver_version --format=csv,noheader

# Should be: 580.105.08 or newer
```

If you need to update:

```bash
# Update NVIDIA packages
sudo pacman -Syu nvidia-open-dkms nvidia-utils

# Or from AUR for latest
yay -Syu nvidia-open-dkms nvidia-utils
```

#### 5. **Backup Your Current Configuration**

Before removing your RTX 4090:

```bash
# Backup nvcontrol config
cp ~/.config/nvcontrol/config.toml ~/.config/nvcontrol/config.toml.4090.backup

# Backup X11 config (if using X11)
sudo cp /etc/X11/xorg.conf /etc/X11/xorg.conf.4090.backup

# Export current nvidia-settings
nvidia-settings --save=/tmp/nvidia-settings-4090.rc
```

---

## System Requirements

### RTX 5090 ASUS ROG Astral Specifications

| Component | Specification |
|-----------|---------------|
| **Architecture** | Blackwell (GB202) |
| **CUDA Cores** | 21,760 |
| **Tensor Cores** | 680 (5th Gen) - 1360 for FP |
| **RT Cores** | 170 (4th Gen) |
| **Memory** | 32GB GDDR7 @ 28 Gbps |
| **Memory Bus** | 512-bit |
| **Bandwidth** | 1,792 GB/s |
| **Boost Clock** | 2610 MHz (OC Mode) |
| **Default Clock** | 2580 MHz |
| **TDP** | 600W (630W max) |
| **PCIe** | Gen 5.0 x16 (backward compatible) |
| **Power Connector** | 12V-2x6 (12VHPWR) |
| **Cooling** | Quad-fan vapor chamber |
| **RGB** | ASUS Aura ARGB |
| **Display Outputs** | 3x DP 2.1a, 1x HDMI 2.1a |

### ASUS Astral Enhancements vs Reference

- **Factory Overclock:** 2610 MHz boost (+30 MHz vs reference)
- **Higher Power Limit:** 630W max (+30W vs reference)
- **Quad-Fan Cooling:** 20% more air pressure vs tri-fan
- **Safe OC Headroom:** +175 MHz GPU, +1500 MHz memory
- **Premium Build:** 14-layer PCB, die-cast metal frame

---

## Installation Steps

### Step 1: Prepare the System

```bash
# 1. Stop any GPU-intensive processes
pkill -f steam
pkill -f gamescope
pkill -f nvcontrol

# 2. Stop display manager
sudo systemctl stop sddm  # KDE
# or
sudo systemctl stop gdm   # GNOME

# 3. Switch to TTY
# Press Ctrl+Alt+F3

# 4. Unload NVIDIA modules
sudo rmmod nvidia_drm
sudo rmmod nvidia_modeset
sudo rmmod nvidia_uvm
sudo rmmod nvidia

# 5. Power off
sudo poweroff
```

### Step 2: Physical Installation

1. **Power Off and Unplug** the system completely
2. **Ground yourself** (touch metal case)
3. **Remove RTX 4090:**
   - Disconnect power cables
   - Unscrew retention bracket
   - Release PCIe latch
   - Carefully remove card
4. **Install RTX 5090:**
   - Align with PCIe x16 slot
   - Firmly press down until latch clicks
   - Secure with screws
   - Connect **12V-2x6 power connector**
   - Ensure all cables are secure

### Step 3: BIOS Verification

Boot into BIOS and verify:

- ✅ **Resizable BAR:** Enabled (should already be)
- ✅ **Above 4G Decoding:** Enabled (should already be)
- ✅ **PCIe Slot:** Gen 4 x16 (auto-detect is fine)
- ⚠️ **Secure Boot:** Consider disabling if you encounter boot issues

### Step 4: First Boot

```bash
# 1. Boot the system
# 2. Watch for any BAR allocation errors in boot messages
# 3. Log in to TTY or desktop

# 4. Check detection
nvidia-smi

# Expected output:
# +-----------------------------------------------------------------------------------------+
# | NVIDIA-SMI 580.105.08     Driver Version: 580.105.08      CUDA Version: 12.8           |
# |-----------------------------------------+------------------------+----------------------+
# | GPU  Name                  Persistence-M | Bus-Id        Disp.A | Volatile Uncorr. ECC |
# | Fan  Temp   Perf          Pwr:Usage/Cap |          Memory-Usage | GPU-Util  Compute M. |
# |                                          |                       |               MIG M. |
# |=========================================+========================+======================|
# |   0  NVIDIA GeForce RTX 5090      Off   | 00000000:01:00.0 On  |                  N/A |
# |  0%   42C    P8              45W / 600W |    512MiB / 32768MiB |      2%      Default |
# |                                          |                       |                  N/A |
# +-----------------------------------------+------------------------+----------------------+
```

---

## Post-Installation Validation

### Use nvcontrol for Comprehensive Validation

```bash
# 1. Full system validation
nvcontrol validate-system

# Expected output:
# ╔═══════════════════════════════════════════════════════╗
# ║         RTX 50-Series System Validation               ║
# ╚═══════════════════════════════════════════════════════╝
#
# 󰢮 GPU Detection
#   ✅ RTX 5090 detected
#   ✅ ASUS ROG Astral variant
#   ✅ 32GB GDDR7 memory
#   ✅ Blackwell architecture (GB202)
#
#  Resizable BAR
#   ✅ Enabled
#   ✅ Size: 32 GB (optimal)
#   ✅ Region 0: 16 MB
#   ✅ Region 1: 32 GB (ReBAR)
#
#  PCIe Configuration
#   ✅ Generation: Gen 4 (compatible with Gen 5)
#   ✅ Link Speed: 16 GT/s
#   ✅ Link Width: x16
#   ✅ Performance impact: < 2%
#
#  System Features
#   ✅ Above 4G Decoding: Enabled
#   ✅ IOMMU: Enabled
#   ⚠️  Secure Boot: Enabled (may cause issues)
#
#  Driver Status
#   ✅ Version: 580.105.08
#   ✅ Type: OpenKernel
#   ✅ GSP Firmware: Active
#   ✅ Blackwell Support: Yes
#
# Overall Status: ✅ READY FOR RTX 5090

# 2. Check GPU info
nvcontrol gpu-info

# Expected output:
# ╔═══════════════════════════════════════════════════════╗
# ║                    GPU Information                    ║
# ╚═══════════════════════════════════════════════════════╝
#
# 󰢮 Device: ASUS ROG Astral GeForce RTX 5090
# 󰐾 Architecture: Blackwell (GB202)
# 󰘚 CUDA Cores: 21,760
# 󰍛 Tensor Cores: 680 (5th Gen) - 1360 for FP ops
#  RT Cores: 170 (4th Gen)
#
#  Memory
#   Type: GDDR7
#   Size: 32 GB
#   Bus: 512-bit
#   Bandwidth: 1,792 GB/s
#
#  Clock Speeds
#   Base: 2280 MHz
#   Boost: 2610 MHz (OC Mode)
#   Default: 2580 MHz
#
# ⚡ Power
#   TDP: 600W
#   Max: 630W
#   Connector: 12V-2x6
#
# 󰈐 Cooling: Quad-Fan Vapor Chamber
# 󰏘 RGB: ASUS Aura ARGB
#
#  Display Outputs
#   DisplayPort: 3x DP 2.1a (4K@480Hz, 8K@165Hz)
#   HDMI: 1x HDMI 2.1a
#
# 󰑙 VRR: G-Sync Compatible
#  DLSS: 4 (Multi-Frame Generation)
```

### Check Driver Info

```bash
nvcontrol driver-info

# Expected output:
# ╔═══════════════════════════════════════════════════════╗
# ║           NVIDIA Kernel Driver Information            ║
# ╚═══════════════════════════════════════════════════════╝
#
# Driver Version: 580.105.08
# Kernel Version: 6.17.7-273-tkg-linux-ghost
# Driver Type: OpenKernel
# GSP Firmware: 580.105.08
#
# Loaded Modules:
#   • nvidia_open
#   • nvidia_modeset
#   • nvidia_uvm
#   • nvidia_drm
#
# Supported Architectures:
#   • Blackwell (GB202)       ✅
#   • Ada Lovelace (AD10x)    ✅
#
# Features:
#   GSP Firmware: ✅
#   Resizable BAR: ✅
#   Confidential Compute: ✅
#   DisplayPort 2.1a: ✅
#   Multi-Instance GPU: ✅
#
# RTX 50-Series Support: ✅ Fully Supported
```

### Verify DLSS 4 Support

```bash
nvcontrol dlss-info

# Expected output:
# ╔═══════════════════════════════════════════════════════╗
# ║                DLSS 4 Capabilities                    ║
# ╚═══════════════════════════════════════════════════════╝
#
# DLSS Version: 4.0
# Multi-Frame Generation: ✅ Supported
#
# Frame Generation Modes:
#   2x Frame Gen: ✅ Available
#   3x Frame Gen: ✅ Available
#   4x Frame Gen: ✅ Available
#
# 󰘚 Tensor Cores: 1360 (for FP operations)
#  Optical Flow Accelerator: Gen 4
#
# Backward Compatibility:
#   DLSS 3.x: ✅
#   DLSS 2.x: ✅
#
# Performance Estimates:
#   4K Native: 100-165 FPS (AAA titles)
#   4K + DLSS 4 (4x): 400-660 FPS
```

---

## nvcontrol Configuration

### Generate Optimized Profiles for RTX 5090

```bash
# 1. Generate RTX 5090 profiles
nvcontrol profile generate RTX-5090

# This creates profiles in: ~/.config/nvcontrol/profiles/

# Created profiles:
# - stock.toml
# - performance.toml
# - quiet.toml
# - max_performance.toml
```

### Profile Breakdown

#### **Stock Profile** (`stock.toml`)
```toml
[profile]
name = "Stock"
description = "Factory default settings"

[overclocking]
gpu_offset = 0
memory_offset = 0
power_limit = 100  # 600W

[thermal]
temp_limit = 90
fan_curve = "auto"

[features]
dlss = true
vrr = true
```

#### **Performance Profile** (`performance.toml`)
```toml
[profile]
name = "Performance"
description = "Optimized for gaming"

[overclocking]
gpu_offset = 175      # +175 MHz
memory_offset = 1500  # +1500 MHz (GDDR7 safe)
power_limit = 105     # 630W

[thermal]
temp_limit = 92
fan_curve = "aggressive"

[features]
dlss = true
dlss_mode = "quality"
vrr = true
latency_mode = "ultra"
```

#### **Quiet Profile** (`quiet.toml`)
```toml
[profile]
name = "Quiet"
description = "Reduced power and noise"

[overclocking]
gpu_offset = -100
memory_offset = -200
power_limit = 85  # 510W

[thermal]
temp_limit = 85
fan_curve = "silent"

[features]
dlss = true
dlss_mode = "balanced"
vrr = true
```

#### **Max Performance Profile** (`max_performance.toml`)
```toml
[profile]
name = "Max Performance"
description = "Extreme OC for benchmarking"

[overclocking]
gpu_offset = 210      # +210 MHz (1.2x safe offset)
memory_offset = 1650  # +1650 MHz
power_limit = 105     # 630W

[thermal]
temp_limit = 92
fan_curve = "max"

[features]
dlss = false  # For raw performance testing
vrr = true
latency_mode = "ultra"
```

### Apply Profiles

```bash
# Apply performance profile
nvcontrol profile apply performance

# Apply quiet profile for desktop work
nvcontrol profile apply quiet

# Apply max performance for benchmarks
nvcontrol profile apply max_performance

# Return to stock
nvcontrol profile apply stock
```

### Custom Fan Curve for Quad-Fan Setup

The ASUS Astral has a **quad-fan design** with better cooling capacity.

```bash
# Set aggressive fan curve
nvcontrol fan --curve 30:25 40:35 50:45 60:60 70:75 80:90 90:100

# Explanation:
# 30°C: 25% fan speed (quieter at idle)
# 40°C: 35%
# 50°C: 45%
# 60°C: 60%
# 70°C: 75%
# 80°C: 90%
# 90°C: 100% (max cooling)

# Save as preset
nvcontrol fan --save aggressive
```

---

## Kernel Module Optimization

### Generate Optimized Configuration

nvcontrol can generate an optimized `/etc/modprobe.d/nvidia.conf`:

```bash
# Generate config
sudo nvcontrol driver-optimize --generate > /tmp/nvidia.conf

# Review the config
cat /tmp/nvidia.conf

# If satisfied, install it
sudo mv /tmp/nvidia.conf /etc/modprobe.d/nvidia.conf

# Rebuild initramfs
sudo mkinitcpio -P

# Reboot for changes to take effect
sudo reboot
```

### Generated Configuration

The `driver-optimize` command creates:

```bash
# NVIDIA Kernel Module Configuration for RTX 50-Series (Blackwell)

# Enable GSP firmware (required for open kernel modules)
options nvidia NVreg_EnableGpuFirmware=1

# Enable Resizable BAR
options nvidia NVreg_EnableResizableBar=1

# Dynamic Power Management (D3Cold)
options nvidia NVreg_DynamicPowerManagement=0x02

# Preserve video memory allocations across suspend
options nvidia NVreg_PreserveVideoMemoryAllocations=1

# Enable HDMI 2.1 fixed rate link
options nvidia NVreg_EnableHDMI20=1

# Temporal dithering for better image quality (OLED displays)
options nvidia NVreg_TemporalDithering=1

# Enable DisplayPort 2.1a support (Blackwell feature)
options nvidia NVreg_EnableDP21=1

# Improve performance with open kernel modules
options nvidia_drm modeset=1

# Enable NVIDIA runtime power management
options nvidia_drm fbdev=1
```

### Parameter Explanations

| Parameter | Value | Description |
|-----------|-------|-------------|
| `NVreg_EnableGpuFirmware` | 1 | **Required** for open kernel modules, enables GSP firmware |
| `NVreg_EnableResizableBar` | 1 | Enables ReBAR support (must also be enabled in BIOS) |
| `NVreg_DynamicPowerManagement` | 0x02 | Fine-grained power management, GPU powers down when idle |
| `NVreg_PreserveVideoMemoryAllocations` | 1 | Preserves VRAM across suspend, faster resume |
| `NVreg_EnableHDMI20` | 1 | Enables HDMI 2.1 features |
| `NVreg_TemporalDithering` | 1 | Improves image quality on OLED displays |
| `NVreg_EnableDP21` | 1 | Enables DP 2.1a support (4K@480Hz, 8K@165Hz) |
| `nvidia_drm modeset` | 1 | Kernel mode setting, better Wayland support |
| `nvidia_drm fbdev` | 1 | Runtime power management |

### Verify Parameters Loaded

```bash
# Check if parameters are active
cat /sys/module/nvidia/parameters/NVreg_EnableGpuFirmware
# Should output: 1

cat /sys/module/nvidia/parameters/NVreg_EnableResizableBar
# Should output: 1

cat /sys/module/nvidia/parameters/NVreg_EnableDP21
# Should output: 1
```

### Early KMS (Kernel Mode Setting)

For Wayland, enable early KMS:

Edit `/etc/mkinitcpio.conf`:

```bash
# Add NVIDIA modules to MODULES array
MODULES=(nvidia nvidia_modeset nvidia_uvm nvidia_drm)
```

Then rebuild:

```bash
sudo mkinitcpio -P
sudo reboot
```

---

## ASUS Astral Specific Features

### RGB Control via OpenRGB

The ASUS Astral has **ASUS Aura ARGB** lighting.

#### Install OpenRGB

```bash
# Install OpenRGB
yay -S openrgb

# Load i2c modules (required for GPU RGB control)
sudo modprobe i2c-dev
sudo modprobe i2c-nvidia_gpu

# Add to autoload
echo "i2c-dev" | sudo tee /etc/modules-load.d/i2c.conf
echo "i2c-nvidia_gpu" | sudo tee -a /etc/modules-load.d/i2c.conf

# Give user access to i2c
sudo usermod -aG i2c $USER

# Reboot for permissions to take effect
sudo reboot
```

#### Use nvcontrol RGB Commands

```bash
# Set static ROG red
nvcontrol rgb --mode static --color FF0000

# Temperature reactive mode (color changes with temp)
nvcontrol rgb --mode temp-reactive

# Breathing cyan
nvcontrol rgb --mode breathing --color 00FFFF

# Rainbow mode
nvcontrol rgb --mode rainbow

# Turn off LEDs (stealth mode)
nvcontrol rgb --mode off
```

#### RGB Modes Available

| Mode | Description | nvcontrol Command |
|------|-------------|-------------------|
| **Static** | Solid color | `--mode static --color RRGGBB` |
| **Breathing** | Pulsing color | `--mode breathing --color RRGGBB` |
| **Rainbow** | Full spectrum cycling | `--mode rainbow` |
| **Temp Reactive** | Color based on GPU temp | `--mode temp-reactive` |
| **Load Reactive** | Color based on GPU usage | `--mode load-reactive` |
| **Off** | LEDs disabled | `--mode off` |

#### Temperature Reactive Colors

When using `--mode temp-reactive`:

- **< 50°C:** Blue/Cyan (cool)
- **50-60°C:** Green (normal)
- **60-70°C:** Yellow (warm)
- **70-80°C:** Orange (hot)
- **> 80°C:** Red (very hot)

### Quad-Fan Cooling Management

The ASUS Astral has **4 fans** instead of the typical 3.

```bash
# Check fan count
nvcontrol fan --info

# Expected output:
# Fans detected: 4
# Current speed: 35%
# Current temp: 42°C

# Set custom curve optimized for quad-fan
nvcontrol fan --curve 30:20 40:30 50:40 60:55 70:70 80:85 90:100

# The lower idle speeds (20% at 30°C) are possible due to better cooling
```

### Factory Overclock Detection

The ASUS Astral comes factory overclocked to **2610 MHz boost**:

```bash
nvcontrol gpu-info | grep "Boost"

# Output:
# Boost: 2610 MHz (OC Mode)
# Default: 2580 MHz
```

This means you're already starting **+30 MHz** above reference.

### Higher Power Limits

The ASUS Astral supports up to **630W** (vs 600W reference):

```bash
# Check current power limit
nvidia-smi -q -d POWER

# Set to max (105% = 630W)
nvcontrol oc --power 105
```

---

## Performance Tuning

### Multi-Monitor Configuration

You mentioned **OLED 4K + IPS 1440p** setup (possibly adding a third monitor).

#### Apply nvcontrol Multi-Monitor Profile

```bash
# Set up dual monitor layout with optimized settings
nvcontrol monitors --layout dual-oled-ips

# This automatically configures:
# Monitor 1 (OLED 4K):
#   - Vibrance: 300 (lower, OLEDs are saturated)
#   - HDR: Enabled
#   - Refresh: 120Hz
#   - Gamma: 2.4 (for HDR content)
#
# Monitor 2 (IPS 1440p):
#   - Vibrance: 600 (boost for IPS)
#   - Refresh: 165Hz
#   - Saturation: 1.15x
#   - Contrast: 1.05x
```

#### Manual Per-Monitor Configuration

```bash
# List connected displays
nvcontrol monitors --list

# Output:
# Display 0: DP-0 (OLED 4K, 3840x2160 @ 120Hz)
# Display 1: DP-1 (IPS 1440p, 2560x1440 @ 165Hz)

# Set vibrance per display
nvcontrol vibrance --display 0 --value 300  # OLED
nvcontrol vibrance --display 1 --value 600  # IPS

# Enable HDR on OLED
nvcontrol hdr --display 0 --enable
```

### Gaming Optimizations

Apply Linux-specific gaming optimizations:

```bash
# Apply competitive preset (max FPS, low latency)
nvcontrol gaming-profile competitive

# This enables:
# - PowerMizer: Prefer Maximum Performance
# - Threaded OpenGL: On
# - ESYNC/FSYNC: Enabled
# - DXVK async: Enabled
# - CPU Governor: Performance
# - GameMode: Active
# - LatencyFleX: Enabled
```

### DLSS 4 Configuration

```bash
# Enable DLSS with quality mode
nvcontrol dlss --mode quality

# Available modes:
# - performance: Max FPS boost
# - balanced: Balanced quality/performance
# - quality: Best image quality
# - ultra-quality: Native-like quality

# Enable Multi-Frame Generation (2x/3x/4x)
nvcontrol dlss --frame-gen 4x

# This can give up to 4x FPS boost in supported games
```

### Benchmark Validation

After setup, run benchmarks to validate performance:

```bash
# Install benchmarking tools
yay -S unigine-heaven unigine-superposition glmark2

# Run Heaven benchmark
unigine-heaven

# Run Superposition
unigine-superposition

# Quick OpenGL test
glmark2
```

Expected performance (rough estimates):

- **Heaven Benchmark (4K Ultra):** 250-300 FPS
- **Superposition (4K Optimized):** 180-220 FPS
- **Cyberpunk 2077 (4K Ultra, RT):** 100-120 FPS native, 400+ with DLSS 4

---

## Troubleshooting

### Issue 1: System Won't Boot After Installing 5090

**Symptoms:**
- Black screen during boot
- PCI BAR allocation errors in dmesg
- System hangs at boot

**Potential Causes:**
- Secure Boot + TPM conflict
- ReBAR misconfiguration
- Insufficient PSU power

**Solutions:**

```bash
# 1. Boot into BIOS and disable Secure Boot
# 2. If still fails, add kernel parameter:

# Edit /etc/default/grub
GRUB_CMDLINE_LINUX="pci=realloc"

# Rebuild GRUB config
sudo grub-mkconfig -o /boot/grub/grub.cfg

# Reboot
```

**Reference:** [NVIDIA Forums - Linux boot failures](https://forums.developer.nvidia.com/t/linux-fails-to-boot-with-secure-boot-tpm-enabled-nvidia-rtx-5090-bar-allocation-errors/343254)

### Issue 2: GPU Not Detected / nvidia-smi Shows Nothing

**Symptoms:**
- `nvidia-smi` shows no GPU
- `lspci` shows GPU but not NVIDIA driver

**Solutions:**

```bash
# 1. Check if modules are loaded
lsmod | grep nvidia

# If not, load them manually
sudo modprobe nvidia
sudo modprobe nvidia_modeset
sudo modprobe nvidia_uvm
sudo modprobe nvidia_drm

# 2. Check dmesg for errors
dmesg | grep -i nvidia

# 3. Rebuild modules
sudo dkms remove nvidia-open/580.105.08 --all
sudo dkms install nvidia-open/580.105.08

# 4. Regenerate initramfs
sudo mkinitcpio -P

# 5. Reboot
sudo reboot
```

### Issue 3: ReBAR Not Working

**Symptoms:**
- nvcontrol reports ReBAR disabled
- Small BAR size (16MB instead of 32GB)

**Solutions:**

```bash
# 1. Verify BIOS settings
# - Resizable BAR: Enabled
# - Above 4G Decoding: Enabled

# 2. Check BAR size
lspci -vv -s 01:00.0 | grep "Region 1"

# Should show something like:
# Region 1: Memory at <address> (64-bit, prefetchable) [size=32G]

# 3. Verify driver parameter
cat /sys/module/nvidia/parameters/NVreg_EnableResizableBar

# Should output: 1

# 4. If still not working, force enable:
sudo modprobe -r nvidia_drm nvidia_modeset nvidia_uvm nvidia
sudo modprobe nvidia NVreg_EnableResizableBar=1

# 5. Make permanent in /etc/modprobe.d/nvidia.conf
echo "options nvidia NVreg_EnableResizableBar=1" | sudo tee -a /etc/modprobe.d/nvidia.conf
```

### Issue 4: Quad-Fan Not Detected

**Symptoms:**
- Only 3 fans shown
- Fan control not working on all fans

**Solutions:**

```bash
# 1. Update to latest driver
sudo pacman -Syu nvidia-open-dkms

# 2. Check nvidia-settings
nvidia-settings -q fans

# 3. Enable Coolbits (for fan control)
# Add to /etc/X11/xorg.conf.d/20-nvidia.conf:
Section "Device"
    Identifier "NVIDIA Card"
    Driver "nvidia"
    Option "Coolbits" "28"
EndSection

# 4. Restart X11/Wayland session
```

### Issue 5: OpenRGB Can't Detect GPU

**Symptoms:**
- OpenRGB doesn't show RTX 5090
- RGB controls not working

**Solutions:**

```bash
# 1. Load i2c modules
sudo modprobe i2c-dev
sudo modprobe i2c-nvidia_gpu

# 2. Add to autoload
echo "i2c-dev" | sudo tee /etc/modules-load.d/i2c.conf
echo "i2c-nvidia_gpu" | sudo tee -a /etc/modules-load.d/i2c.conf

# 3. Check i2c devices
ls -la /dev/i2c-*

# 4. Give user i2c access
sudo usermod -aG i2c $USER

# 5. Reboot
sudo reboot

# 6. Run OpenRGB as root first time to detect
sudo openrgb --detect

# 7. After detection, regular user should work
openrgb
```

### Issue 6: DisplayPort 2.1a Not Working

**Symptoms:**
- Can't achieve 4K @ 480Hz
- Monitor reports DP 1.4 instead of 2.1a

**Solutions:**

```bash
# 1. Verify driver parameter
cat /sys/module/nvidia/parameters/NVreg_EnableDP21

# Should output: 1

# 2. Enable if not set
echo "options nvidia NVreg_EnableDP21=1" | sudo tee -a /etc/modprobe.d/nvidia.conf

# 3. Rebuild initramfs and reboot
sudo mkinitcpio -P
sudo reboot

# 4. Test with xrandr
xrandr --verbose | grep -A 10 "DP-"

# 5. Force DP 2.1a mode
xrandr --output DP-0 --mode 3840x2160 --rate 480

# 6. Use nvcontrol
nvcontrol display --enable-dp21
nvcontrol display --test-dp21
```

### Issue 7: High Idle Power / Temps

**Symptoms:**
- Idle power > 100W
- Idle temp > 50°C

**Solutions:**

```bash
# 1. Enable dynamic power management
echo "options nvidia NVreg_DynamicPowerManagement=0x02" | sudo tee -a /etc/modprobe.d/nvidia.conf

# 2. Check current power state
nvidia-smi -q -d POWER

# 3. Apply quiet profile
nvcontrol profile apply quiet

# 4. Set conservative fan curve
nvcontrol fan --curve 30:20 50:35 70:60 90:90

# 5. Rebuild initramfs and reboot
sudo mkinitcpio -P
sudo reboot
```

### Issue 8: DLSS 4 Not Available in Games

**Symptoms:**
- DLSS option missing in games
- Only DLSS 2/3 available

**Solutions:**

```bash
# 1. Verify DLSS 4 support
nvcontrol dlss-info

# 2. Update Proton (for Steam games)
# In Steam: Settings > Compatibility > Enable Steam Play
# Select: Proton Experimental or latest GE-Proton

# 3. Set environment variables for Proton
# Add to game launch options:
PROTON_ENABLE_NVAPI=1 DXVK_NVAPI_DRIVER_VERSION=58010508 %command%

# 4. For native Linux games, update game to latest version

# 5. Check game compatibility:
# DLSS 4 is only available in games that explicitly support it
# Most games will need updates to support multi-frame generation
```

---

## Quick Reference Commands

### Essential nvcontrol Commands

```bash
# System validation
nvcontrol validate-system

# GPU info
nvcontrol gpu-info

# Driver info
nvcontrol driver-info

# Apply profile
nvcontrol profile apply performance

# Overclock
nvcontrol oc --gpu +175 --mem +1500 --power 105

# Fan control
nvcontrol fan --curve 30:25 50:45 70:75 90:100

# RGB control
nvcontrol rgb --mode temp-reactive

# Multi-monitor setup
nvcontrol monitors --layout dual-oled-ips

# DLSS configuration
nvcontrol dlss --mode quality --frame-gen 4x

# Gaming optimizations
nvcontrol gaming-profile competitive

# Generate kernel config
sudo nvcontrol driver-optimize --generate > /tmp/nvidia.conf
```

### Essential nvidia-smi Commands

```bash
# Basic GPU info
nvidia-smi

# Detailed query
nvidia-smi -q

# Monitor in real-time
watch -n 1 nvidia-smi

# Check power
nvidia-smi -q -d POWER

# Check clocks
nvidia-smi -q -d CLOCK

# Check temperature
nvidia-smi -q -d TEMPERATURE
```

### Essential System Commands

```bash
# Check ReBAR
lspci -vv -s 01:00.0 | grep "Region 1"

# Check driver version
nvidia-smi --query-gpu=driver_version --format=csv,noheader

# Check loaded modules
lsmod | grep nvidia

# Check kernel parameters
cat /sys/module/nvidia/parameters/NVreg_EnableResizableBar
cat /sys/module/nvidia/parameters/NVreg_EnableGpuFirmware
cat /sys/module/nvidia/parameters/NVreg_EnableDP21

# Rebuild initramfs
sudo mkinitcpio -P

# Check dmesg for errors
dmesg | grep -i nvidia
dmesg | grep -i "bar\|pci"
```

---

## Summary Checklist

### Pre-Installation
- [ ] PSU is 1000W or higher
- [ ] Case supports 357mm GPU length and 3.8 slots
- [ ] Backup current config files
- [ ] Check Secure Boot status
- [ ] Update to driver 580.105.08 or newer

### Installation
- [ ] Physically install RTX 5090
- [ ] Connect 12V-2x6 power connector
- [ ] Verify BIOS settings (ReBAR, Above 4G)
- [ ] Boot and check `nvidia-smi` detects GPU

### Post-Installation
- [ ] Run `nvcontrol validate-system`
- [ ] Run `nvcontrol gpu-info` - should show "ASUS ROG Astral"
- [ ] Run `nvcontrol driver-info` - should show driver 580.105.08+
- [ ] Generate and install optimized kernel config
- [ ] Enable early KMS in initramfs
- [ ] Reboot

### Configuration
- [ ] Apply performance profile: `nvcontrol profile apply performance`
- [ ] Set fan curve for quad-fan: `nvcontrol fan --curve 30:25 50:45 70:75 90:100`
- [ ] Configure multi-monitor: `nvcontrol monitors --layout dual-oled-ips`
- [ ] Install OpenRGB and configure RGB
- [ ] Test DLSS 4: `nvcontrol dlss-info`
- [ ] Apply gaming optimizations: `nvcontrol gaming-profile competitive`

### Testing
- [ ] Run benchmark (Heaven/Superposition)
- [ ] Test RGB modes
- [ ] Test fan control
- [ ] Test overclocking
- [ ] Test multi-monitor setup
- [ ] Test DLSS 4 in supported game

---

## Additional Resources

- **nvcontrol Documentation:** `docs/`
- **RTX 50-Series Readiness:** `docs/RTX_50_SERIES_READINESS.md`
- **ASUS Astral Features:** `docs/ASUS_ASTRAL_FEATURES.md`
- **Kernel Driver Optimizations:** `docs/KERNEL_DRIVER_580_OPTIMIZATIONS.md`
- **TUI User Guide:** `docs/TUI_USER_GUIDE.md`

### External Links

- [ASUS ROG Astral Product Page](https://rog.asus.com/graphics-cards/graphics-cards/rog-astral/rog-astral-rtx5090-o32g-gaming/)
- [NVIDIA RTX 5090 Specs](https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/rtx-5090/)
- [NVIDIA Open Kernel Modules](https://github.com/NVIDIA/open-gpu-kernel-modules)
- [OpenRGB](https://openrgb.org/)
- [Arch Linux NVIDIA Wiki](https://wiki.archlinux.org/title/NVIDIA)

---

**Your ASUS ROG Astral RTX 5090 is ready to unleash maximum performance on Arch Linux! 🚀**
