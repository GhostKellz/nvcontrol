# Display Management API

Display detection, color management, and digital vibrance control.

## Functions

### `list_displays() -> Result<Vec<DisplayInfo>>`

List all connected displays.

**Returns**: Vector of `DisplayInfo`

**Example**:
```rust
use nvcontrol::display;

let displays = display::list_displays()?;
for d in displays {
    println!("{}: {} ({})", d.id, d.model, d.resolution);
}
```

### `set_digital_vibrance(display_id: &str, value: i32) -> Result<()>`

Set digital vibrance (color saturation).

**Parameters**:
- `display_id`: Display identifier (e.g., "DP-1")
- `value`: Vibrance level (-1024 to 1023, default 0)

**Example**:
```rust
// Boost colors for gaming
display::set_digital_vibrance("DP-1", 512)?;

// Natural colors for photo editing
display::set_digital_vibrance("DP-1", 0)?;
```

### `get_digital_vibrance(display_id: &str) -> Result<i32>`

Get current digital vibrance value.

**Example**:
```rust
let vibrance = display::get_digital_vibrance("DP-1")?;
println!("Current vibrance: {}", vibrance);
```

## Structs

### `DisplayInfo`

```rust
pub struct DisplayInfo {
    pub id: String,              // e.g., "DP-1", "HDMI-1"
    pub model: String,           // Monitor model name
    pub manufacturer: String,    // Manufacturer
    pub resolution: String,      // e.g., "3840x2160"
    pub refresh_rate: u32,       // Hz
    pub connected: bool,         // Connection status
    pub primary: bool,           // Primary display
    pub hdr_capable: bool,       // HDR support
    pub vrr_capable: bool,       // VRR/G-SYNC/FreeSync
}
```

### `ColorProfile`

```rust
pub struct ColorProfile {
    pub name: String,
    pub digital_vibrance: i32,
    pub gamma: f32,
    pub brightness: f32,
    pub contrast: f32,
}
```

## CLI Commands

### `nvctl display ls`

List displays.

```bash
nvctl display ls

# Output:
# DP-1: ASUS ROG PG27AQN (2560x1440@360Hz) ✓ Primary
# HDMI-1: LG 27UK650 (3840x2160@60Hz)
```

### `nvctl display vibrance <display> <value>`

Set digital vibrance.

```bash
# Boost vibrance for gaming
nvctl display vibrance DP-1 750

# Reset to default
nvctl display vibrance DP-1 0
```

### `nvctl display info <display>`

Show display details.

```bash
nvctl display info DP-1

# Output:
# Display: DP-1
# Model: ASUS ROG PG27AQN
# Resolution: 2560x1440
# Refresh: 360 Hz
# Digital Vibrance: 750
# HDR: Supported
# VRR: Enabled (G-SYNC)
```

## Profiles

### Gaming Profile
```toml
# ~/.config/nvcontrol/profiles/gaming.toml
[display.DP-1]
digital_vibrance = 750
vrr_enabled = true

[display.HDMI-1]
digital_vibrance = 500
```

### Content Creation Profile
```toml
# ~/.config/nvcontrol/profiles/photo-editing.toml
[display.DP-1]
digital_vibrance = 0      # Natural colors
gamma = 2.2
color_profile = "sRGB"
```

## Advanced Features

### Per-Application Vibrance

```rust
use nvcontrol::display;

fn auto_apply_vibrance(app_name: &str, display: &str) -> Result<()> {
    let vibrance = match app_name {
        "cs2" | "valorant" => 900,        // Competitive gaming
        "cyberpunk2077" => 750,           // Single-player
        "gimp" | "krita" => 0,            // Photo editing
        "blender" => 300,                 // 3D work
        _ => 500,                         // Default
    };

    display::set_digital_vibrance(display, vibrance)?;
    Ok(())
}
```

### Multi-Display Management

```rust
fn optimize_multi_display() -> Result<()> {
    let displays = display::list_displays()?;

    for d in displays {
        if d.primary {
            // Primary for gaming
            display::set_digital_vibrance(&d.id, 800)?;
        } else {
            // Secondary for monitoring
            display::set_digital_vibrance(&d.id, 400)?;
        }
    }

    Ok(())
}
```

## Wayland Support

nvcontrol uses [nVibrant](https://github.com/Tremeschin/nVibrant) for Wayland digital vibrance:

```bash
# Ensure nVibrant is installed
paru -S nvibrant-cli

# nvcontrol will automatically use nVibrant on Wayland
nvctl display vibrance DP-1 750
```

## HDR Control

```rust
// Enable HDR (requires KDE Plasma 6+ or supported compositor)
display::set_hdr_enabled("DP-1", true)?;

// Check HDR status
let hdr_enabled = display::is_hdr_enabled("DP-1")?;
```

CLI:
```bash
nvctl display hdr on DP-1
nvctl display hdr off DP-1
```
