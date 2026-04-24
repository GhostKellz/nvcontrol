# Fan Control API

GPU fan speed and curve management.

## Functions

### `set_fan_speed(gpu_id: u32, speed_percent: u8) -> Result<()>`

Set manual fan speed.

**Parameters**:
- `gpu_id`: GPU index
- `speed_percent`: Fan speed 0-100%

**Example**:
```rust
use nvcontrol::fan;

// Set fan to 70%
fan::set_fan_speed(0, 70)?;
```

### `set_auto_fan(gpu_id: u32) -> Result<()>`

Enable automatic fan control.

**Example**:
```rust
fan::set_auto_fan(0)?;
```

### `set_fan_curve(gpu_id: u32, curve: &[(u8, u8)]) -> Result<()>`

Set custom fan curve.

**Parameters**:
- `curve`: Vector of (temperature, fan_speed) pairs

**Example**:
```rust
let curve = vec![
    (30, 20),   // 30°C → 20% fan
    (60, 50),   // 60°C → 50% fan
    (75, 80),   // 75°C → 80% fan
    (85, 100),  // 85°C → 100% fan
];
fan::set_fan_curve(0, &curve)?;
```

## Structs

### `FanInfo`

```rust
pub struct FanInfo {
    pub count: u32,          // Number of fans
    pub speeds: Vec<u32>,    // Current RPM per fan
    pub percentages: Vec<u8>, // Current % per fan
    pub mode: FanMode,
}
```

### `FanMode`

```rust
pub enum FanMode {
    Auto,           // Automatic (driver controlled)
    Manual(u8),     // Manual speed (%)
    Curve(Vec<(u8, u8)>),  // Custom curve
}
```

### `FanCurvePreset`

```rust
pub enum FanCurvePreset {
    Silent,      // Quiet operation (30-60%)
    Balanced,    // Balanced cooling (40-80%)
    Aggressive,  // Maximum cooling (50-100%)
    Custom(Vec<(u8, u8)>),
}
```

## Presets

### Silent
```rust
// Optimized for low noise
let curve = FanCurvePreset::Silent.to_curve();
// (30,20) (50,30) (70,50) (85,70)
```

### Balanced
```rust
// Balanced noise/cooling
let curve = FanCurvePreset::Balanced.to_curve();
// (30,30) (60,50) (75,70) (85,90)
```

### Aggressive
```rust
// Maximum cooling
let curve = FanCurvePreset::Aggressive.to_curve();
// (30,40) (60,60) (75,85) (85,100)
```

## CLI Commands

### `nvctl fan set <speed>`

Set manual fan speed.

```bash
nvctl fan set 70

# Output:
# Fan speed set to 70% (2100 RPM)
```

### `nvctl fan auto`

Enable automatic fan control.

```bash
nvctl fan auto

# Output:
# Automatic fan control enabled
```

### `nvctl fan curve <preset|custom>`

Set fan curve.

```bash
# Use preset
nvctl fan curve silent
nvctl fan curve balanced
nvctl fan curve aggressive

# Custom curve
nvctl fan curve "30:20,60:50,75:80,85:100"
```

### `nvctl fan info`

Display fan information.

```bash
nvctl fan info

# Output:
# Fans: 3
# Fan 0: 1850 RPM (62%)
# Fan 1: 1820 RPM (61%)
# Fan 2: 1840 RPM (62%)
# Mode: Curve (Balanced)
```

## Configuration

```toml
# ~/.config/nvcontrol/profiles/gaming.toml
[fan]
mode = "curve"
preset = "aggressive"

# Or custom curve
[[fan.curve]]
temp = 30
speed = 40

[[fan.curve]]
temp = 60
speed = 60

[[fan.curve]]
temp = 75
speed = 85

[[fan.curve]]
temp = 85
speed = 100
```

## Safety Features

- Minimum speed: 20% (prevents damage)
- Maximum speed: 100%
- Auto fallback on driver issues
- Temperature monitoring with automatic boost

## Advanced Usage

### Temperature-Based Auto-Adjust

```rust
use nvcontrol::{fan, gpu};

fn smart_fan_control(gpu_id: u32) -> Result<()> {
    let stats = gpu::get_real_time_stats(gpu_id)?;

    if stats.temperature > 85 {
        // Emergency cooling
        fan::set_fan_speed(gpu_id, 100)?;
    } else if stats.temperature > 75 {
        // Aggressive cooling
        fan::set_fan_curve(gpu_id, &FanCurvePreset::Aggressive.to_curve())?;
    } else {
        // Normal operation
        fan::set_fan_curve(gpu_id, &FanCurvePreset::Balanced.to_curve())?;
    }

    Ok(())
}
```
