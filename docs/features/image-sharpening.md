# Image Sharpening

NVIDIA image sharpening support status and compatibility notes.

## Overview

Recent NVIDIA driver branches removed or changed the older NVKMS image-sharpening attribute path that earlier nvcontrol docs described. nvcontrol should treat image sharpening as a capability-dependent feature: report whether the loaded stack exposes it, avoid promising that it is available on every GPU, and prefer game/upscaler sharpening when the driver path is unavailable.

When available, sharpening can enhance edge definition and clarity. When unavailable, commands should report that state instead of implying a permissions problem.

## Quick Start

```bash
# Check sharpening status
nvctl display sharpening status

# Try a moderate intensity when reported available
nvctl display sharpening set 50

# Maximum requested sharpening
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
  Available: <yes|no>
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

Older nvcontrol builds attempted to use NVKMS (NVIDIA Kernel Modeset) ioctls for image sharpening:

- **Device:** `/dev/nvidia-modeset`
- **Attribute:** `ImageSharpening`
- **Method:** Direct kernel interface (no nvidia-settings required)

### Supported Hardware

| Architecture | Support |
|--------------|---------|
| Blackwell (RTX 50xx) | Capability-dependent; verify with `nvctl display sharpening status` |
| Ada Lovelace (RTX 40xx) | Capability-dependent; verify with `nvctl display sharpening status` |
| Ampere (RTX 30xx) | Capability-dependent; verify with `nvctl display sharpening status` |
| Turing (RTX 20xx) | Capability-dependent; verify with `nvctl display sharpening status` |
| Pascal (GTX 10xx) | Legacy/basic path; do not assume availability |

### Performance Impact

When the driver exposes sharpening, performance impact is expected to be small. Validate visually and with the target game/application because driver behavior and compositor paths vary.

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

Per-display sharpening is only available when the loaded driver/backend exposes the required control:

```bash
# Set for specific display
nvctl display sharpening set-display 0 50
nvctl display sharpening set-display 1 0
```

## Comparison with Game Sharpening

| Feature | nvcontrol (NVKMS) | In-Game |
|---------|-------------------|---------|
| Scope | Driver/backend-dependent | Per-game |
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

1. **Check driver/runtime status:**
   ```bash
   nvctl display sharpening status
   ```

2. **Verify NVKMS access:**
   ```bash
   ls -la /dev/nvidia-modeset
   ```

3. **Check driver branch notes:**
   - Current drivers may not expose the older NVKMS sharpening attribute
   - If status reports unavailable, use game/upscaler sharpening instead

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
nvctl config apply --input gaming
```

## API Reference

For programmatic access:

```rust
use nvcontrol::display_controls::{DisplayControls, ImageSharpeningInfo};

// Get sharpening info
let controls = DisplayControls::new(device_handle, disp_handle, dpy_id)?;
let info: ImageSharpeningInfo = controls.get_image_sharpening_info()?;
if !info.available {
    // Report unavailable to the user instead of applying.
}

// Set sharpening
controls.set_image_sharpening(50)?;

// Reset to default
controls.reset_image_sharpening()?;
```

## Related Documentation

- [Display Commands](../commands.md#display-commands) - Display management
- [HDR Control](hdr.md) - High Dynamic Range
- [Overclocking Guide](overclocking.md) - Performance tuning

---
