# Backend Abstraction API

*Added in v0.7.6*

The backend abstraction layer enables deterministic testing without NVIDIA hardware.

## Overview

nvcontrol separates hardware access into traits:
- `NvmlBackend` - GPU queries (temperature, clocks, power, fans)
- `DisplayCommandRunner` - Display commands (xrandr, hyprctl, gsettings)

Both have real and mock implementations.

## NvmlBackend Trait

```rust
use nvcontrol::nvml_backend::{NvmlBackend, SharedNvmlBackend, MockNvmlBackend};
use std::sync::Arc;

// Create mock backend
let backend: SharedNvmlBackend = Arc::new(MockNvmlBackend::single_gpu());

// Use in tests
assert!(backend.is_available());
let metrics = backend.get_metrics(0)?;
```

### Mock Constructors

```rust
// Single GPU (default mock data)
let mock = MockNvmlBackend::single_gpu();

// Multi-GPU setup
let mock = MockNvmlBackend::multi_gpu(4);

// No GPU (test error handling)
let mock = MockNvmlBackend::no_gpu();
```

### Key Methods

```rust
pub trait NvmlBackend: Send + Sync {
    fn is_available(&self) -> bool;
    fn device_count(&self) -> NvResult<u32>;
    fn get_metrics(&self, index: u32) -> NvResult<GpuMetrics>;
    fn get_device_info(&self, index: u32) -> NvResult<GpuDeviceInfo>;
    fn get_temperature(&self, index: u32) -> NvResult<u32>;
    fn get_fan_speed(&self, index: u32, fan: u32) -> NvResult<u32>;
    fn get_power_usage(&self, index: u32) -> NvResult<u32>;
    // ... more methods
}
```

## DisplayCommandRunner Trait

```rust
use nvcontrol::display_backend::{DisplayCommandRunner, MockDisplayRunner};

// Create compositor-specific mock
let runner = MockDisplayRunner::kde();
let runner = MockDisplayRunner::gnome();
let runner = MockDisplayRunner::hyprland();

// Custom mock
let runner = MockDisplayRunner::wayland()
    .with_compositor("cosmic")
    .with_command_output("cosmic-comp", "mock output");
```

### Mock Constructors

```rust
// X11 environment
let mock = MockDisplayRunner::x11();

// Wayland environment
let mock = MockDisplayRunner::wayland();

// Compositor-specific
let mock = MockDisplayRunner::kde();
let mock = MockDisplayRunner::gnome();
let mock = MockDisplayRunner::hyprland();

// Headless (test error handling)
let mock = MockDisplayRunner::headless();
```

## GuiBackendContext

Unified context for UI applications:

```rust
use nvcontrol::nvml_backend::GuiBackendContext;

// Production
let ctx = GuiBackendContext::new();

// Testing
let ctx = GuiBackendContext::mock();

// Query through context
let metrics = ctx.get_metrics(0)?;
let info = ctx.get_device_info(0)?;
```

## Testing Example

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nvcontrol::nvml_backend::{MockNvmlBackend, create_mock_backend};

    #[test]
    fn test_multi_gpu_detection() {
        let backend = MockNvmlBackend::multi_gpu(4);

        assert_eq!(backend.device_count().unwrap(), 4);

        for i in 0..4 {
            let info = backend.get_device_info(i).unwrap();
            assert_eq!(info.index, i);
        }
    }

    #[test]
    fn test_no_gpu_error_handling() {
        let backend = MockNvmlBackend::no_gpu();

        assert!(!backend.is_available());
        assert!(backend.get_metrics(0).is_err());
    }
}
```

## Integration with Modules

Modules accept backend parameters:

```rust
use nvcontrol::monitoring;
use nvcontrol::nvml_backend::create_mock_backend;

let backend = create_mock_backend();
let metrics = monitoring::collect_metrics_with_backend(&backend, 0)?;
```

## See Also

- [Backend Architecture](../config/BACKEND_ARCHITECTURE.md) - Full architecture details
- [test_mock_backends.rs](../../tests/test_mock_backends.rs) - Test examples
