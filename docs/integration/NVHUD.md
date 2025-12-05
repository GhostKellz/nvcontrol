# nvcontrol + nvhud Integration

*Status: Planned*

Integration guide for using nvcontrol with [nvhud](https://github.com/ghostkellz/nvhud), a Zig-based GPU monitoring overlay.

## Overview

nvhud provides a lightweight, low-latency GPU monitoring overlay for gaming. When integrated with nvcontrol, it can display real-time GPU metrics, profile status, and backend health.

## Planned Features

### Metrics Display
- GPU temperature, utilization, clocks
- Fan speed and power draw
- Memory usage
- nvcontrol profile status

### Backend Integration
- Shared NVML backend to avoid duplicate driver sessions
- Real-time metrics from nvcontrol's `CachedMetrics`
- Backend status indicator (Available/Unavailable)

### Profile Switching
- On-screen profile selector
- Quick-toggle for fan curves, OC presets
- Game-specific auto-apply indicators

## Architecture (Planned)

```
┌─────────────────────────────────────────┐
│              nvhud (Zig)                │
│  ┌─────────────────────────────────┐    │
│  │       Overlay Renderer          │    │
│  │  (Vulkan/OpenGL composition)    │    │
│  └──────────────┬──────────────────┘    │
│                 │                        │
│                 ▼                        │
│  ┌─────────────────────────────────┐    │
│  │      IPC / Shared Memory        │    │
│  └──────────────┬──────────────────┘    │
└─────────────────┼───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│           nvcontrol (Rust)              │
│  ┌─────────────────────────────────┐    │
│  │      GuiBackendContext          │    │
│  │  - CachedMetrics                │    │
│  │  - BackendStatus                │    │
│  │  - SharedNvmlBackend            │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

## IPC Protocol (Planned)

nvhud will communicate with nvcontrol via shared memory or Unix socket:

```json
{
  "type": "metrics_update",
  "gpu_index": 0,
  "data": {
    "temperature": 65,
    "utilization_gpu": 87,
    "utilization_memory": 45,
    "power_watts": 285,
    "fan_speed_percent": 62,
    "clock_gpu_mhz": 2580,
    "clock_memory_mhz": 10501,
    "memory_used_mb": 8192,
    "memory_total_mb": 24576
  },
  "backend_status": "Available",
  "active_profile": "Gaming"
}
```

## CLI Integration (Planned)

```bash
# Start nvhud with nvcontrol backend
nvhud --backend nvcontrol

# nvcontrol exports metrics for nvhud
nvctl metrics export --format nvhud --socket /tmp/nvcontrol-metrics.sock
```

## Configuration (Planned)

```toml
# ~/.config/nvhud/config.toml
[backend]
type = "nvcontrol"
socket = "/tmp/nvcontrol-metrics.sock"
refresh_rate_ms = 100

[overlay]
position = "top-left"
opacity = 0.8
show_profile = true
show_backend_status = true

[metrics]
temperature = true
utilization = true
power = true
fan_speed = true
clocks = true
memory = true
```

## Roadmap

1. **Phase 1**: Basic metrics IPC from nvcontrol to nvhud
2. **Phase 2**: Profile status and switching via overlay
3. **Phase 3**: Backend status indicator
4. **Phase 4**: Game-specific overlay presets

## Repository

- nvhud: [github.com/ghostkellz/nvhud](https://github.com/ghostkellz/nvhud)
- nvcontrol: [github.com/ghostkellz/nvcontrol](https://github.com/ghostkellz/nvcontrol)

## See Also

- [nvbind Integration](./NVBIND.md) - Container GPU runtime
- [Backend Architecture](../config/BACKEND_ARCHITECTURE.md) - nvcontrol backend design
