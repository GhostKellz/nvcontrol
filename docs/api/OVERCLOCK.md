# Overclocking API

GPU and memory clock control with safety features.

## Functions

### `apply_overclock(gpu_id: u32, gpu_offset: i32, memory_offset: i32) -> Result<()>`

Apply GPU and memory clock offsets.

**Parameters**:
- `gpu_id`: GPU index
- `gpu_offset`: GPU clock offset in MHz (±500)
- `memory_offset`: Memory clock offset in MHz (±1000)

**Example**:
```rust
use nvcontrol::overclock;

// +150MHz GPU, +500MHz memory
overclock::apply_overclock(0, 150, 500)?;
```

### `get_current_clocks(gpu_id: u32) -> Result<ClockInfo>`

Get current clock speeds.

**Returns**: `ClockInfo` with current and base clocks

**Example**:
```rust
let clocks = overclock::get_current_clocks(0)?;
println!("GPU: {} MHz (base: {})", clocks.gpu_current, clocks.gpu_base);
println!("Memory: {} MHz (base: {})", clocks.memory_current, clocks.memory_base);
```

### `stress_test(gpu_id: u32, duration_secs: u64) -> Result<StressTestResult>`

Run stability stress test.

**Parameters**:
- `gpu_id`: GPU to test
- `duration_secs`: Test duration

**Returns**: `StressTestResult` with stability metrics

**Example**:
```rust
let result = overclock::stress_test(0, 300)?;  // 5 minute test
if result.stable {
    println!("Overclock is stable!");
} else {
    println!("Instability detected at {}s", result.failure_time);
}
```

## Structs

### `ClockInfo`

```rust
pub struct ClockInfo {
    pub gpu_base: u32,        // Base GPU clock (MHz)
    pub gpu_current: u32,     // Current GPU clock (MHz)
    pub gpu_boost: u32,       // Boost clock (MHz)
    pub memory_base: u32,     // Base memory clock (MHz)
    pub memory_current: u32,  // Current memory clock (MHz)
}
```

### `OverclockProfile`

```rust
pub struct OverclockProfile {
    pub name: String,
    pub gpu_offset: i32,      // MHz offset
    pub memory_offset: i32,   // MHz offset
    pub power_limit: u32,     // Watts
    pub fan_curve: Vec<(u8, u8)>,  // (temp, fan_speed) pairs
}
```

### `StressTestResult`

```rust
pub struct StressTestResult {
    pub stable: bool,
    pub duration: u64,        // Seconds completed
    pub max_temp: i32,        // Peak temperature
    pub avg_power: u32,       // Average power draw
    pub failure_time: Option<u64>,  // When instability occurred
}
```

## Safety Features

### Automatic Limits

- GPU offset: ±500 MHz
- Memory offset: ±1000 MHz
- Temperature cutoff: 90°C
- Power limit enforcement

### Validation

```rust
use nvcontrol::overclock::OverclockLimits;

let limits = OverclockLimits::detect(0)?;
println!("Safe GPU offset range: {} to {} MHz", limits.gpu_min, limits.gpu_max);
println!("Safe memory offset range: {} to {} MHz", limits.memory_min, limits.memory_max);
```

## CLI Commands

### `nvctl overclock apply <gpu_offset> <memory_offset>`

Apply overclock.

```bash
nvctl overclock apply +150 +500

# Output:
# Applied overclock: +150 MHz GPU, +500 MHz memory
# Current clocks: GPU 2640 MHz, Memory 10500 MHz
```

### `nvctl overclock reset`

Reset to stock clocks.

```bash
nvctl overclock reset

# Output:
# Overclocks reset to default
```

### `nvctl overclock stress [duration]`

Run stress test.

```bash
nvctl overclock stress 300  # 5 minutes

# Live output:
# Testing overclock stability...
# Temperature: 75°C | Power: 350W | Time: 45s / 300s
# ...
# Test complete: STABLE ✓
```

### `nvctl overclock info`

Display current overclock status.

```bash
nvctl overclock info

# Output:
# GPU Clock: 2640 MHz (+150 MHz)
# Memory Clock: 10500 MHz (+500 MHz)
# Status: Stable
```

## Profiles

```toml
# ~/.config/nvcontrol/profiles/ultra.toml
[overclock]
gpu_offset = 150
memory_offset = 500
validated = true

[power]
limit = 450

[fan]
curve = [[60, 40], [75, 70], [85, 100]]
```

Apply:
```bash
nvctl profile apply ultra
```
