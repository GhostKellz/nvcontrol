# Image Sharpening

NVIDIA GPU-based image sharpening control via NVKMS.

## Overview

nvcontrol provides native image sharpening control through NVIDIA's modeset kernel interface. This is a GPU-level post-processing effect that:

- Enhances edge definition and clarity
- Works globally across all applications
- Has minimal performance impact
- Operates independently of game settings

## Quick Start

```bash
# Check sharpening status
nvctl display sharpening status

# Enable with default intensity (50%)
nvctl display sharpening set 50

# Maximum sharpening
nvctl display sharpening set 100

# Disable (set to 0)
nvctl display sharpening set 0

# Reset to driver default
nvctl display sharpening reset
```

## Commands

### `nvctl display sharpening status`

Show current image sharpening status and capabilities.

**Output includes:**
- Availability (hardware support)
- Current value
- Default value
- Valid range

**Example output:**
```
Image Sharpening Status:
══════════════════════════════════════
  Available: ✅ Yes
  Current Value: 50
  Default Value: 0
  Range: 0 - 100
══════════════════════════════════════
```

### `nvctl display sharpening set <value>`

Set image sharpening intensity.

**Parameters:**
- `value` - Sharpening intensity (0-100)
  - `0` = Disabled (no sharpening)
  - `50` = Moderate (recommended)
  - `100` = Maximum sharpening

```bash
nvctl display sharpening set 50    # Moderate
nvctl display sharpening set 75    # High
nvctl display sharpening set 100   # Maximum
```

### `nvctl display sharpening reset`

Reset to driver default value.

```bash
nvctl display sharpening reset
```

## Technical Details

### Implementation

nvcontrol uses NVKMS (NVIDIA Kernel Modeset) ioctls to control image sharpening:

- **Device:** `/dev/nvidia-modeset`
- **Attribute:** `ImageSharpening`
- **Method:** Direct kernel interface (no nvidia-settings required)

### Supported Hardware

| Architecture | Support |
|--------------|---------|
| Blackwell (RTX 50xx) | ✅ Full |
| Ada Lovelace (RTX 40xx) | ✅ Full |
| Ampere (RTX 30xx) | ✅ Full |
| Turing (RTX 20xx) | ✅ Full |
| Pascal (GTX 10xx) | ⚠️ Limited |

### Performance Impact

Image sharpening has minimal GPU overhead:
- < 1% performance impact at any setting
- No additional VRAM usage
- Works on desktop and fullscreen content

## Use Cases

### Gaming

Image sharpening can enhance visual clarity, especially useful when:
- Using DLSS/FSR upscaling (sharpens upscaled output)
- Playing at lower internal resolution
- Preferring sharper visuals

```bash
# Gaming preset - moderate sharpening
nvctl display sharpening set 50

# Competitive gaming - max clarity
nvctl display sharpening set 75
```

### Content Creation

For color-accurate work, disable sharpening:

```bash
# Photo/video editing - disable
nvctl display sharpening set 0
```

### Desktop Use

Light sharpening can improve text clarity:

```bash
# Desktop - subtle enhancement
nvctl display sharpening set 25
```

## Per-Display Control

Image sharpening can be configured per display:

```bash
# Set for specific display
nvctl display sharpening set-display 0 50
nvctl display sharpening set-display 1 0
```

## Comparison with Game Sharpening

| Feature | nvcontrol (NVKMS) | In-Game |
|---------|-------------------|---------|
| Scope | Global (all apps) | Per-game |
| Control | CLI/GUI | Game settings |
| Persistence | System-wide | Per-game saves |
| Performance | ~0% | Varies |
| Quality | Consistent | Game-dependent |

## Integration with DLSS/FSR

When using upscaling technologies:

1. **DLSS Sharpening:** Use DLSS's built-in sharpening
2. **FSR Sharpening:** FSR includes RCAS sharpening pass
3. **GPU Sharpening:** Can stack but may over-sharpen

**Recommended approach:**
- Use game/upscaler sharpening first
- Add GPU sharpening only if more clarity needed
- Keep values moderate (25-50) to avoid artifacts

## Troubleshooting

### Sharpening Not Available

1. **Check driver version:**
   ```bash
   nvidia-smi --query-gpu=driver_version --format=csv
   ```

2. **Verify NVKMS access:**
   ```bash
   ls -la /dev/nvidia-modeset
   ```

3. **Check GPU support:**
   - Pascal and newer GPUs support image sharpening
   - Older GPUs may not have this feature

### No Visible Effect

- Ensure value is non-zero: `nvctl display sharpening set 50`
- Effect is subtle - compare with 0 vs 100 to see difference
- Most visible on upscaled or lower-resolution content

### Over-Sharpening Artifacts

If edges look harsh or haloed:
- Reduce sharpening value
- Typical comfortable range: 25-50
- Values above 75 may introduce visible artifacts

## Configuration

Sharpening settings can be persisted in nvcontrol profiles:

```toml
# ~/.config/nvcontrol/profiles/gaming.toml
[display]
image_sharpening = 50
```

Apply profile:
```bash
nvctl profile apply gaming
```

## API Reference

For programmatic access:

```rust
use nvcontrol::display_controls::{DisplayControls, ImageSharpeningInfo};

// Get sharpening info
let controls = DisplayControls::new(device_handle, disp_handle, dpy_id)?;
let info: ImageSharpeningInfo = controls.get_image_sharpening_info()?;

// Set sharpening
controls.set_image_sharpening(50)?;

// Reset to default
controls.reset_image_sharpening()?;
```

## Related Documentation

- [Display Commands](COMMANDS.md#display-commands) - Display management
- [HDR Control](HDR_CONTROL.md) - High Dynamic Range
- [Overclocking Guide](OVERCLOCKING.md) - Performance tuning

---

**Last Updated**: December 2024 (v0.7.3)
