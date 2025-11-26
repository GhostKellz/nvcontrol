# GPU Management API

Core GPU monitoring and control functionality.

## Functions

### `get_gpu_info(gpu_id: u32) -> Result<GpuInfo>`

Get comprehensive GPU information.

**Returns**: `GpuInfo` struct with GPU details

**Example**:
```rust
use nvcontrol::gpu;

let info = gpu::get_gpu_info(0)?;
println!("GPU: {}", info.name);
println!("Driver: {}", info.driver_version);
println!("Temperature: {}°C", info.temperature);
```

### `get_real_time_stats(gpu_id: u32) -> Result<GpuStats>`

Get live GPU statistics.

**Returns**: `GpuStats` with real-time metrics

**Fields**:
- `temperature: i32` - GPU temperature in Celsius
- `power_usage: u32` - Power draw in watts
- `utilization: f32` - GPU utilization percentage
- `memory_used: u64` - Used VRAM in bytes
- `memory_total: u64` - Total VRAM in bytes
- `fan_speed: u32` - Fan speed in RPM
- `clock_gpu: u32` - Current GPU clock in MHz
- `clock_memory: u32` - Current memory clock in MHz

**Example**:
```rust
let stats = gpu::get_real_time_stats(0)?;
println!("GPU Load: {:.1}%", stats.utilization);
println!("VRAM: {} / {} MB",
    stats.memory_used / 1024 / 1024,
    stats.memory_total / 1024 / 1024);
```

## Structs

### `GpuInfo`

Comprehensive GPU information.

```rust
pub struct GpuInfo {
    pub id: u32,
    pub name: String,
    pub pci_bus: String,
    pub driver_version: String,
    pub cuda_version: Option<String>,
    pub architecture: String,
    pub compute_capability: Option<(u32, u32)>,
    pub memory_total: u64,
    pub temperature: i32,
    pub power_limit_current: u32,
    pub power_limit_max: u32,
}
```

### `GpuStats`

Real-time GPU statistics.

```rust
pub struct GpuStats {
    pub temperature: i32,
    pub power_usage: u32,
    pub utilization: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub fan_speed: u32,
    pub clock_gpu: u32,
    pub clock_memory: u32,
}
```

## CLI Commands

### `nvctl gpu info`

Display GPU information.

```bash
nvctl gpu info

# Output:
# GPU 0: NVIDIA GeForce RTX 4090
#   Driver: 545.29.06
#   CUDA: 12.3
#   Architecture: Ada Lovelace
#   Memory: 24 GB
```

### `nvctl gpu stat`

Live GPU monitoring (TUI).

```bash
nvctl gpu stat

# Real-time dashboard with:
# - Temperature graph
# - Power usage
# - GPU/Memory utilization
# - Fan speed
# - Clock speeds
```

## Error Handling

```rust
use nvcontrol::NvResult;

fn monitor_gpu() -> NvResult<()> {
    let info = gpu::get_gpu_info(0)?;

    if info.temperature > 85 {
        eprintln!("Warning: GPU temperature high: {}°C", info.temperature);
    }

    Ok(())
}
```
