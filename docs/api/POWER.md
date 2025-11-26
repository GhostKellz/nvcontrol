# Power Management API

GPU power limit and profile management.

## Functions

### `set_power_limit(gpu_id: u32, watts: u32) -> Result<()>`

Set GPU power limit.

**Parameters**:
- `gpu_id`: GPU index
- `watts`: Power limit in watts

**Example**:
```rust
use nvcontrol::power;

// Set 350W power limit
power::set_power_limit(0, 350)?;
```

### `get_power_limit(gpu_id: u32) -> Result<PowerLimits>`

Get current and maximum power limits.

**Returns**: `PowerLimits` struct

**Example**:
```rust
let limits = power::get_power_limit(0)?;
println!("Current: {}W", limits.current);
println!("Default: {}W", limits.default);
println!("Max: {}W", limits.max);
```

### `set_power_mode(gpu_id: u32, mode: PowerMode) -> Result<()>`

Set power management mode.

**Modes**:
- `PowerMode::MaxPerformance` - 115% power limit
- `PowerMode::Balanced` - 100% power limit
- `PowerMode::Quiet` - 85% power limit
- `PowerMode::PowerSaver` - 70% power limit

**Example**:
```rust
use nvcontrol::power::PowerMode;

power::set_power_mode(0, PowerMode::Balanced)?;
```

## Structs

### `PowerLimits`

```rust
pub struct PowerLimits {
    pub current: u32,    // Current power limit in watts
    pub default: u32,    // Default power limit
    pub min: u32,        // Minimum power limit
    pub max: u32,        // Maximum power limit
}
```

### `PowerMode`

```rust
pub enum PowerMode {
    MaxPerformance,   // 115% TDP
    Balanced,         // 100% TDP
    Quiet,            // 85% TDP
    PowerSaver,       // 70% TDP
    Custom(u32),      // Custom wattage
}
```

## CLI Commands

### `nvctl power limit <watts>`

Set power limit.

```bash
nvctl power limit 350

# Output:
# Power limit set to 350W
```

### `nvctl power mode <mode>`

Set power mode.

```bash
nvctl power mode balanced
nvctl power mode quiet
nvctl power mode max
```

### `nvctl power info`

Display power information.

```bash
nvctl power info

# Output:
# Current: 350W
# Default: 450W
# Max: 600W
# Mode: Balanced
```

## Profiles

```toml
# ~/.config/nvcontrol/profiles/gaming.toml
[power]
mode = "MaxPerformance"
limit = 450  # Watts

[thermal]
target_temp = 75
```

Apply profile:
```bash
nvctl profile apply gaming
```
