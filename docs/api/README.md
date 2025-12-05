# nvcontrol API Reference

This directory contains API documentation for nvcontrol's Rust library.

## Modules

| Module | Description |
|--------|-------------|
| [GPU](./GPU.md) | GPU info, monitoring, real-time stats |
| [Display](./DISPLAY.md) | Display detection, vibrance, color management |
| [Fan](./FAN.md) | Fan speed control and curves |
| [Power](./POWER.md) | Power limits and profiles |
| [Overclock](./OVERCLOCK.md) | Clock offsets and stress testing |
| [Backend](./BACKEND.md) | Backend abstraction for testing (v0.7.6+) |

## Quick Start

```rust
use nvcontrol::{gpu, fan, power};

fn main() -> nvcontrol::NvResult<()> {
    // Get GPU info
    let info = gpu::get_gpu_info(0)?;
    println!("GPU: {}", info.name);

    // Get real-time stats
    let stats = gpu::get_real_time_stats(0)?;
    println!("Temperature: {}°C", stats.temperature);

    // Set fan curve
    let curve = vec![(30, 20), (60, 50), (80, 80)];
    fan::set_fan_curve(0, &curve)?;

    Ok(())
}
```

## Error Handling

All functions return `NvResult<T>`:

```rust
use nvcontrol::NvResult;

fn safe_operation() -> NvResult<()> {
    let info = gpu::get_gpu_info(0)?;
    // ...
    Ok(())
}
```

## Testing with Mocks

See [BACKEND.md](./BACKEND.md) for testing with mock backends (v0.7.6+).
