# HDR Control

High Dynamic Range (HDR) display control for Linux Wayland compositors.

## Overview

nvcontrol provides HDR management across KDE Plasma, GNOME, and Hyprland. HDR enables:

- **Wider color gamut** - More vivid, accurate colors
- **Higher contrast** - Deeper blacks, brighter whites
- **10-bit+ color depth** - Smoother gradients, no banding
- **Peak brightness** - Up to 1000+ nits for highlights

## Quick Start

```bash
# Check HDR status and capabilities
nvctl display hdr status

# Enable HDR
nvctl display hdr enable

# Disable HDR (return to SDR)
nvctl display hdr disable

# Toggle HDR on/off
nvctl display hdr toggle 0
```

## Commands

### `nvctl display hdr status`

Show comprehensive HDR status for all displays.

**Output includes:**
- GPU HDR support
- Compositor and version
- Per-display HDR state
- Color depth and gamut info

**Example output:**
```
HDR Status:
══════════════════════════════════════
  Compositor: hyprland
  GPU Support: ✅ Yes

  Monitors:
    DP-1 (2560x1440@165Hz): ✅ HDR Active
    HDMI-A-1 (3840x2160@60Hz): ❌ SDR

💡 To enable HDR:
  nvctl display hdr enable
  OR add to hyprland.conf: monitor=DP-1,2560x1440@165,auto,1,hdr
```

### `nvctl display hdr enable [display_id]`

Enable HDR for a display.

```bash
nvctl display hdr enable      # Enable on all capable displays
nvctl display hdr enable 0    # Enable on display 0
```

### `nvctl display hdr disable [display_id]`

Disable HDR and return to SDR mode.

```bash
nvctl display hdr disable     # Disable on all displays
nvctl display hdr disable 0   # Disable on display 0
```

### `nvctl display hdr toggle <display_id>`

Toggle HDR state for a specific display.

```bash
nvctl display hdr toggle 0
```

## Compositor Support

### KDE Plasma 6+

Full HDR support with System Settings integration.

**Enable via nvcontrol:**
```bash
nvctl display hdr enable
```

**Or via KDE System Settings:**
1. System Settings → Display and Monitor
2. Select your display
3. Enable "HDR" toggle
4. Adjust SDR brightness slider

**KDE HDR Features:**
- Per-display HDR control
- SDR content brightness scaling
- Wide color gamut support
- 10-bit color output

### GNOME 45+

HDR support via experimental features.

```bash
# Enable HDR
nvctl display hdr enable

# Equivalent gsettings
gsettings set org.gnome.mutter experimental-features "['hdr']"
```

**Note:** GNOME HDR is still maturing. Some applications may not render correctly.

### Hyprland

Native HDR support with monitor configuration.

```bash
# Enable via nvcontrol
nvctl display hdr enable

# Or configure in hyprland.conf
monitor=DP-1,2560x1440@165,auto,1,bitdepth,10
```

**Hyprland HDR environment variables:**
```bash
# Add to ~/.config/hypr/hyprland.conf
env = WLR_DRM_FORCE_HDR,1
```

## Hardware Requirements

### GPU Support

| Architecture | HDR Support |
|--------------|-------------|
| Blackwell (RTX 50xx) | ✅ Full |
| Ada Lovelace (RTX 40xx) | ✅ Full |
| Ampere (RTX 30xx) | ✅ Full |
| Turing (RTX 20xx) | ✅ Full |
| Pascal (GTX 10xx) | ⚠️ Limited |

### Display Requirements

- **Panel:** HDR10, HDR10+, or Dolby Vision capable
- **Connection:**
  - DisplayPort 1.4+ (recommended)
  - HDMI 2.0+ (HDR10)
  - HDMI 2.1 (HDR10+, Dolby Vision)
- **Color depth:** 10-bit panel (8-bit+FRC works but not ideal)

### Driver Requirements

- NVIDIA driver 545+ recommended
- NVIDIA Open Kernel Modules 580+ for best Wayland HDR

## HDR Standards

| Standard | Peak Brightness | Color Gamut | Metadata |
|----------|-----------------|-------------|----------|
| HDR10 | 1000 nits | Rec. 2020 | Static |
| HDR10+ | 4000 nits | Rec. 2020 | Dynamic |
| Dolby Vision | 10000 nits | Rec. 2020 | Dynamic |

nvcontrol primarily works with HDR10, which has the widest support.

## Color Settings

### Color Range

```bash
# Check current color range
nvctl display color range

# Set full RGB range (recommended for PC monitors)
nvctl display color range full

# Set limited range (for TVs)
nvctl display color range limited
```

### Color Space

```bash
# Check color space
nvctl display color space

# Set RGB (default for monitors)
nvctl display color space rgb

# Set YCbCr444 (HDR content)
nvctl display color space ycbcr444
```

## Gaming with HDR

### Optimal HDR Gaming Setup

```bash
# 1. Enable HDR
nvctl display hdr enable

# 2. Enable VRR for smooth gameplay
nvctl vrr enable DP-1

# 3. Verify settings
nvctl display hdr status
nvctl vrr status
```

### Per-Game HDR

Some games handle HDR internally. For games with HDR support:

1. Enable system HDR via nvcontrol
2. Enable HDR in game settings
3. Calibrate in-game HDR brightness

### SDR Games in HDR Mode

When HDR is enabled, SDR content is tone-mapped. KDE and GNOME allow adjusting SDR brightness:

- **KDE:** SDR brightness slider in Display Settings
- **GNOME:** Automatic tone mapping

## Troubleshooting

### HDR Not Available

1. **Check GPU support:**
   ```bash
   nvctl display hdr status
   ```

2. **Verify display connection:**
   - Use DisplayPort 1.4+ or HDMI 2.0+
   - Check cable is HDR-capable

3. **Check driver version:**
   ```bash
   nvidia-smi --query-gpu=driver_version --format=csv
   ```

### Washed Out Colors

- SDR content may look washed out in HDR mode
- Adjust SDR brightness in compositor settings
- Some apps need HDR-aware rendering

### Screen Goes Black

```bash
# Disable HDR to recover
nvctl display hdr disable

# Or via compositor settings
# KDE: kscreen-doctor output.DP-1.hdr.0
```

### Games Not Using HDR

- Ensure game has HDR support
- Enable HDR in game settings after system HDR is on
- Some games require fullscreen mode for HDR

## Configuration

HDR settings persist in compositor configuration:

### KDE
```
~/.local/share/kscreen/
```

### Hyprland
```bash
# hyprland.conf
monitor=DP-1,2560x1440@165,auto,1,bitdepth,10
```

## Related Documentation

- [VRR/G-SYNC Control](VRR_GSYNC.md) - Variable refresh rate
- [Display Commands](COMMANDS.md#display-commands) - Display management
- [Image Sharpening](IMAGE_SHARPENING.md) - GPU image processing

---

**Last Updated**: December 2024 (v0.7.3)
