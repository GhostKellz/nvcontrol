# Backend Architecture

*Added in v0.7.6*

nvcontrol uses a backend abstraction layer for deterministic testing and unified resource management.

## Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
│  (CLI, TUI, GUI, Tray, Notifications)                       │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   GuiBackendContext                          │
│  ┌─────────────────────┐  ┌─────────────────────────────┐   │
│  │  SharedNvmlBackend  │  │  SharedDisplayRunner        │   │
│  │  Arc<dyn NvmlBackend>│  │  Arc<dyn DisplayCommandRunner>│ │
│  └─────────────────────┘  └─────────────────────────────┘   │
│                                                              │
│  + BackendStatus (debounced)                                │
│  + CachedMetrics per GPU                                    │
│  + StatusTracker (hotplug debouncing)                       │
└──────────────────────────┬──────────────────────────────────┘
                           │
          ┌────────────────┴────────────────┐
          ▼                                 ▼
┌──────────────────────┐        ┌──────────────────────┐
│   RealNvmlBackend    │        │  ShellDisplayRunner  │
│   (nvml_wrapper)     │        │  (allow-listed cmds) │
└──────────────────────┘        └──────────────────────┘
          │                                 │
          ▼                                 ▼
    NVIDIA Driver                 System Binaries
    (libnvidia-ml.so)       (/usr/bin/hyprctl, gsettings, etc.)
```

## Components

### NvmlBackend Trait

Abstracts all NVML queries:

```rust
pub trait NvmlBackend: Send + Sync {
    fn is_available(&self) -> bool;
    fn device_count(&self) -> NvResult<u32>;
    fn get_metrics(&self, index: u32) -> NvResult<GpuMetrics>;
    fn get_device_info(&self, index: u32) -> NvResult<GpuDeviceInfo>;
    fn get_temperature(&self, index: u32) -> NvResult<u32>;
    fn get_fan_speed(&self, index: u32, fan: u32) -> NvResult<u32>;
    fn get_power_usage(&self, index: u32) -> NvResult<u32>;
    // ... 20+ methods
}
```

**Implementations:**
- `RealNvmlBackend` - Production (wraps nvml_wrapper)
- `MockNvmlBackend` - Testing (configurable responses)

### DisplayCommandRunner Trait

Abstracts display-related shell commands:

```rust
pub trait DisplayCommandRunner: Send + Sync {
    fn run_xrandr(&self, args: &[&str]) -> DisplayResult<String>;
    fn run_nvidia_settings(&self, args: &[&str]) -> DisplayResult<String>;
    fn run_wayland_info(&self) -> DisplayResult<String>;
    fn run_command(&self, cmd: &str, args: &[&str]) -> DisplayResult<String>;
    fn command_available(&self, cmd: &str) -> bool;
    fn is_available(&self) -> bool;
}
```

**Implementations:**
- `ShellDisplayRunner` - Production (allow-listed absolute paths)
- `MockDisplayRunner` - Testing (compositor-specific mocks)

### GuiBackendContext

Unified context for all UI layers:

```rust
pub struct GuiBackendContext {
    pub nvml: SharedNvmlBackend,
    pub display: SharedDisplayRunner,
    pub device_count: u32,
    pub driver_version: String,
    pub status: BackendStatus,
    // Internal: metrics_cache, status_tracker
}
```

**Usage:**
```rust
// Production
let ctx = GuiBackendContext::new();

// Testing
let ctx = GuiBackendContext::mock();

// Custom backends
let ctx = GuiBackendContext::with_backends(nvml, display);
```

## BackendStatus

Unified status reporting with debouncing:

```rust
pub enum BackendStatus {
    Available,
    NvmlUnavailable(String),
    DisplayUnavailable(String),
    AllUnavailable { nvml_reason: String, display_reason: String },
}
```

### Hotplug Debouncing

Status changes are debounced (2-second threshold) to prevent UI flicker during rapid attach/detach events (eGPU, USB-C docks):

```rust
// Periodically refresh status (e.g., each frame)
let status = ctx.refresh_status();

// Check if transition is pending
if ctx.is_status_transitioning() {
    // Status may change soon
}
```

## Security: Command Allow-List

`ShellDisplayRunner` only executes binaries from an allow-list of absolute paths:

| Command | Allowed Paths |
|---------|---------------|
| `xrandr` | `/usr/bin/xrandr` |
| `nvidia-settings` | `/usr/bin/nvidia-settings`, `/usr/local/bin/nvidia-settings` |
| `hyprctl` | `/usr/bin/hyprctl` |
| `gsettings` | `/usr/bin/gsettings` |
| `kscreen-doctor` | `/usr/bin/kscreen-doctor`, `/usr/lib/kf6/bin/kscreen-doctor` |
| `swaymsg` | `/usr/bin/swaymsg` |
| ... | See `get_allowed_commands()` in `display_backend.rs` |

Commands not in the allow-list are rejected with `DisplayError::CommandNotAllowed`.

## Cached Metrics

Metrics are cached with staleness detection:

```rust
// Get metrics (updates cache on success)
let metrics = ctx.get_metrics(0)?;

// Check cache age
if let Some(age) = ctx.get_cached_metrics_age(0) {
    println!("Metrics are {}s old", age);
}

// Check staleness
if ctx.are_metrics_stale(0, 5) {
    println!("Metrics older than 5 seconds");
}
```

When NVML queries fail, cached values are returned if available.

## Testing with Mocks

```rust
#[test]
fn test_with_mock_backend() {
    let ctx = GuiBackendContext::mock();

    // Mock returns predictable data
    let metrics = ctx.get_metrics(0).unwrap();
    assert!(metrics.temperature > 0);

    // Test with custom mock
    let mock = MockNvmlBackend::multi_gpu(4);
    let display = MockDisplayRunner::kde();
    let ctx = GuiBackendContext::with_backends(
        Arc::new(mock),
        Arc::new(display),
    );
}
```

## Module Migration Status

All modules now use the backend abstraction:

| Module | Status |
|--------|--------|
| `gpu.rs` | ✅ SharedNvmlBackend |
| `monitoring.rs` | ✅ SharedNvmlBackend |
| `multi_gpu.rs` | ✅ SharedNvmlBackend |
| `fan.rs` | ✅ SharedNvmlBackend |
| `advanced_power.rs` | ✅ SharedNvmlBackend |
| `tui.rs` | ✅ GuiBackendContext |
| `tray.rs` | ✅ SharedNvmlBackend |
| `notifications.rs` | ✅ SharedNvmlBackend |
| `vrr.rs` | ✅ SharedDisplayRunner |
| `hdr.rs` | ✅ SharedDisplayRunner |
