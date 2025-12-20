# NVIDIA GSP Firmware

GSP (GPU System Processor) is a dedicated RISC-V processor embedded in NVIDIA GPUs (Turing and newer) that handles GPU management tasks.

## Quick Commands

```bash
# Check GSP status
nvctl driver gsp status

# Enable/disable GSP
nvctl driver gsp enable
nvctl driver gsp disable

# Learn about GSP
nvctl driver gsp explain

# View GSP logs
nvctl driver logs --filter gsp

# Full driver info (includes GSP)
nvctl driver info
```

## What is GSP?

GSP offloads GPU management from the CPU to a dedicated processor on the GPU:

- **Power management** - GPU power states, thermals
- **Initialization** - GPU boot and configuration
- **Security** - Firmware validation, secure boot
- **Error handling** - GPU fault recovery

## Requirements

| Component | Requirement |
|-----------|-------------|
| GPU | Turing or newer (RTX 20/30/40/50 series) |
| Driver | nvidia-open (open kernel modules) |
| Firmware | Included with nvidia-open package |

## Commands

### `nvctl driver gsp status`

Shows GSP firmware status:

```
NVIDIA GSP Firmware Status

GSP Enabled:    Yes
GSP State:      loaded
Version:        590.48.01
Firmware:       /lib/firmware/nvidia/gb202/gsp
GPU Arch:       gb202
```

States:
- `active` - GSP initialized successfully
- `loaded` - GSP firmware loaded (no explicit init message)
- `failed` - GSP initialization failed
- `not_loaded` - nvidia module not loaded

### `nvctl driver gsp explain`

Prints comprehensive explanation of:
- What GSP is and why it matters
- Common issues and solutions
- Relevant log commands

### `nvctl driver gsp enable`

Enables GSP firmware. Adds to `/etc/modprobe.d/nvidia.conf`:
```
options nvidia NVreg_EnableGpuFirmware=1
```

Requires reboot to take effect.

### `nvctl driver gsp disable`

Disables GSP (fallback to legacy mode). Not recommended for RTX 40/50 series.

### `nvctl driver gsp diagnostics`

Runs GSP diagnostics including:
- Init status from kernel logs
- Error/warning counts
- Current telemetry (power, temp, clocks)

## Integration with driver info

`nvctl driver info` includes a GSP block:

```
GSP:            enabled (loaded)
GSP Firmware:   590.48.01
                /lib/firmware/nvidia/gb202/gsp
GPU Arch:       gb202
```

## Integration with driver check

`nvctl driver check` includes GSP health checks:

```
GSP Firmware:
  ✓ nvidia-open driver: Using nvidia-open kernel module
  ✓ GSP enabled: GSP firmware is enabled
  ✓ GSP initialization: GSP firmware loaded
  ✓ GSP firmware files: Found at /lib/firmware/nvidia/gb202/gsp
```

## Viewing GSP Logs

```bash
# GSP-specific logs
nvctl driver logs --filter gsp

# All nvidia logs
nvctl driver logs --filter nvidia

# Xid errors (GPU faults)
nvctl driver logs --filter xid
```

## Firmware Locations

### New layout (per-chip directories)
```
/lib/firmware/nvidia/gb202/gsp/      # RTX 50 series (Blackwell)
/lib/firmware/nvidia/ad102/gsp/      # RTX 40 series (Ada)
/lib/firmware/nvidia/ga102/gsp/      # RTX 30 series (Ampere)
```

### Legacy layout
```
/lib/firmware/nvidia/590.48.01/
├── gsp_ga10x.bin
└── gsp_tu10x.bin
```

## Common Issues

### GSP init fails after kernel update

**Symptom:** GPU not working after kernel update, GSP errors in logs.

**Cause:** nvidia modules not rebuilt for new kernel.

**Solution:**
```bash
nvctl driver dkms fix
# or
nvctl driver dkms build
```

### GSP errors on resume from suspend

**Symptom:** GPU issues after suspend/resume, GSP timeout errors.

**Solution:** Enable video memory preservation:
```bash
# Add to /etc/modprobe.d/nvidia.conf
options nvidia NVreg_PreserveVideoMemoryAllocations=1
```

### GSP firmware load timeout

**Symptom:** Long boot time, timeout errors in dmesg.

**Cause:** Kernel/driver version mismatch.

**Solution:**
1. Rebuild DKMS: `nvctl driver dkms fix`
2. Verify kernel matches module: `nvctl driver info`

### Performance issues with GSP enabled

For troubleshooting, you can temporarily disable GSP:
```bash
nvctl driver gsp disable
# Reboot and test
```

Not recommended for RTX 40/50 series - GSP is required for full functionality.

## Modprobe Options

Common GSP-related modprobe options:

```conf
# /etc/modprobe.d/nvidia.conf

# Enable GSP (default for nvidia-open on Turing+)
options nvidia NVreg_EnableGpuFirmware=1

# Preserve VRAM across suspend (helps with GSP resume)
options nvidia NVreg_PreserveVideoMemoryAllocations=1

# Enable DRM modeset (required for Wayland)
options nvidia_drm modeset=1
```

After changing modprobe options:
```bash
sudo mkinitcpio -P  # Arch
# Reboot
```

## Paste-friendly Output

For sharing in Discord/forums:
```bash
nvctl driver info --paste
```

Produces:
```
GPU: NVIDIA GeForce RTX 5090, 590.48.01
Kernel: 6.18.2-1-cachyos-lto
Module: nvidia-open
GSP: on (loaded)
Arch: gb202
```

## Architecture Detection

nvctl detects GPU architecture for firmware path resolution:

| GPU Series | Architecture | Code |
|------------|--------------|------|
| RTX 5090/5080 | Blackwell | gb202 |
| RTX 5070/5060 | Blackwell | gb205 |
| RTX 4090/4080 | Ada Lovelace | ad102 |
| RTX 4070/4060 | Ada Lovelace | ad104 |
| RTX 3090/3080/3070 | Ampere | ga102 |
| RTX 3060/3050 | Ampere | ga106 |
| RTX 2080/2070 | Turing | tu102 |
| RTX 2060/GTX 16xx | Turing | tu106 |

## References

- [Arch Wiki - NVIDIA GSP Firmware](https://wiki.archlinux.org/title/NVIDIA#GSP_Firmware)
- [NVIDIA open-gpu-kernel-modules](https://github.com/NVIDIA/open-gpu-kernel-modules)
- [NVIDIA Open Source Transition](https://developer.nvidia.com/blog/nvidia-transitions-fully-towards-open-source-gpu-kernel-modules)
