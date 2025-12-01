# VRR / G-SYNC Control

Variable Refresh Rate (VRR) and G-SYNC control for smooth, tear-free gaming on Linux.

## Overview

nvcontrol provides unified VRR/G-SYNC management across all major Wayland compositors and X11. This includes:

- **G-SYNC**: NVIDIA's proprietary VRR technology (DisplayPort)
- **G-SYNC Compatible**: FreeSync monitors certified by NVIDIA
- **FreeSync/Adaptive Sync**: AMD's open VRR standard (works with NVIDIA 10xx+)

## Quick Start

```bash
# Check VRR status for all displays
nvctl vrr status

# Enable VRR on your primary display
nvctl vrr enable DP-1

# Disable VRR
nvctl vrr disable DP-1

# Configure custom refresh range
nvctl vrr configure DP-1 --min-refresh 48 --max-refresh 165
```

## Commands

### `nvctl vrr status`

Show comprehensive VRR status for all connected displays.

**Output includes:**
- Display name and connection type
- VRR/G-SYNC/FreeSync support
- Current enabled state
- Refresh rate range (min/max Hz)
- Compositor-specific VRR policy

**Example output:**
```
VRR Status:
══════════════════════════════════════════════════════════════
  Display      │ VRR     │ G-SYNC  │ Range     │ Status
───────────────┼─────────┼─────────┼───────────┼────────────
  DP-1         │ ✅      │ ✅      │ 48-165Hz  │ Enabled (Auto)
  HDMI-A-1     │ ✅      │ ❌      │ 48-120Hz  │ Disabled
══════════════════════════════════════════════════════════════
```

### `nvctl vrr enable <display>`

Enable VRR for a specific display.

```bash
nvctl vrr enable DP-1        # DisplayPort 1
nvctl vrr enable HDMI-A-1    # HDMI port 1
nvctl vrr enable DP-2        # DisplayPort 2
```

### `nvctl vrr disable <display>`

Disable VRR and use fixed refresh rate.

```bash
nvctl vrr disable DP-1
```

### `nvctl vrr configure <display> [OPTIONS]`

Advanced VRR configuration with custom settings.

**Options:**
- `--min-refresh <hz>` - Minimum refresh rate (default: 48)
- `--max-refresh <hz>` - Maximum refresh rate (display max)
- `--adaptive-sync` - Enable adaptive sync mode
- `--lfc` - Enable Low Framerate Compensation

**Examples:**
```bash
# Competitive gaming (high refresh, disable LFC for lowest latency)
nvctl vrr configure DP-1 --min-refresh 120 --max-refresh 240

# Cinematic gaming (wide range with LFC)
nvctl vrr configure DP-1 --min-refresh 24 --max-refresh 165 --lfc

# Balance (typical gaming setup)
nvctl vrr configure DP-1 --min-refresh 48 --max-refresh 144
```

## Compositor Support

nvcontrol automatically detects and configures VRR for your compositor:

### KDE Plasma 6+

Uses `kscreen-doctor` for VRR policy management.

**VRR Policies:**
- `0` = Never (VRR disabled)
- `1` = Always (VRR always active)
- `2` = Automatic (VRR in fullscreen apps)

```bash
# Check KDE VRR settings
kscreen-doctor -j | grep vrrPolicy

# nvcontrol handles this automatically
nvctl vrr enable DP-1  # Sets policy to Automatic (2)
```

### GNOME 45+

Uses mutter experimental features.

```bash
# Enable VRR in GNOME
nvctl vrr enable DP-1

# Equivalent gsettings command
gsettings set org.gnome.mutter experimental-features "['variable-refresh-rate']"
```

### Hyprland

Direct `hyprctl` integration with monitor configuration.

```bash
# Enable VRR
nvctl vrr enable DP-1

# Or configure in hyprland.conf:
# monitor=DP-1,2560x1440@165,auto,1,vrr,1
```

**Hyprland VRR values:**
- `0` = Off
- `1` = On
- `2` = Fullscreen only

### Sway

Uses `swaymsg` for adaptive sync control.

```bash
# Enable adaptive sync
nvctl vrr enable DP-1

# Equivalent swaymsg command
swaymsg output DP-1 adaptive_sync enable
```

### X11 (Legacy)

Falls back to `xrandr` and `nvidia-settings` for G-SYNC.

```bash
# Enable G-SYNC via nvidia-settings
nvidia-settings -a "[gpu:0]/GPUGSyncAllowed=1"
```

## Hardware Requirements

### G-SYNC Displays

- Native G-SYNC module (premium monitors)
- DisplayPort connection required
- NVIDIA GPU GTX 650 Ti or newer

### G-SYNC Compatible (FreeSync)

- FreeSync/Adaptive Sync certified monitor
- DisplayPort 1.2a+ or HDMI 2.1
- NVIDIA GPU GTX 10xx series or newer
- Driver 417.71 or newer

### Refresh Rate Ranges

| Monitor Type | Typical Range |
|--------------|---------------|
| Budget 1080p | 48-144Hz |
| Gaming 1440p | 48-165Hz |
| High-end 4K | 48-144Hz |
| Esports | 48-240Hz+ |

## Per-Application Settings

nvcontrol supports per-application VRR profiles:

```bash
# Steam games - full VRR support
# CS2 - high refresh, no LFC for competitive
# Firefox - VRR disabled (power saving)
```

Configure in `~/.config/nvcontrol/vrr_profiles.toml`:

```toml
[steam]
enabled = true
min_refresh_rate = 48
max_refresh_rate = 165
adaptive_sync = true
lfc = true

[cs2]
enabled = true
min_refresh_rate = 60
max_refresh_rate = 240
adaptive_sync = true
lfc = false  # Competitive preference

[firefox]
enabled = false  # Save power during browsing
```

## Troubleshooting

### VRR Not Working

1. **Check monitor support:**
   ```bash
   nvctl vrr status
   ```

2. **Verify DisplayPort connection:**
   - G-SYNC requires DisplayPort
   - HDMI 2.1 supports VRR on newer cards

3. **Check NVIDIA driver settings:**
   ```bash
   nvidia-settings -q GPUGSyncAllowed
   ```

4. **Enable G-SYNC Compatible in nvidia-settings:**
   - Open nvidia-settings
   - X Server Display Configuration
   - Enable "Allow G-SYNC Compatible" for your monitor

### Flickering Issues

- Try adjusting min refresh rate higher
- Disable Low Framerate Compensation
- Check for driver updates

### Black Screen After Enable

```bash
# Reset VRR settings
nvctl vrr disable DP-1

# Or reset via compositor settings
kscreen-doctor output.DP-1.vrrpolicy.0
```

## Related Documentation

- [HDR Control](HDR_CONTROL.md) - High Dynamic Range settings
- [Display Commands](commands/gpu.md) - Display management
- [Gaming Profiles](commands/gaming.md) - Game-specific optimizations

---

**Last Updated**: December 2024 (v0.7.3)
