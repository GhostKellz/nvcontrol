# Display API

Display support combines the native NVIDIA NVKMS vibrance controller with
compositor command backends for layout, HDR, and VRR policy.

## Native vibrance

```rust
use nvcontrol::vibrance_native::NativeVibranceController;

let mut controller = NativeVibranceController::new()?;
for connector in controller.list_displays() {
    println!("{connector:?}");
}
controller.set_vibrance_all(150)?;
```

Percentages are `0..=200`, with 100 as the neutral raw NVKMS value. v0.8.12
queries the live attribute and driver-advertised range, filters disconnected
NVKMS slots, and selects the known NVIDIA 595 or 610 allocation ABI at runtime.

Public helpers include:

- `get_vibrance_connectors_native()` for typed connector/readback data;
- `get_vibrance_status_native()` for structured status;
- `set_display_vibrance_native()` and `set_vibrance_all_native()`;
- `reset_vibrance_native()`;
- percentage/raw conversion helpers with rounded readback.

```bash
nvctl display vibrance get
nvctl display vibrance list
nvctl display vibrance set 150
nvctl display vibrance set-display 1 150
nvctl display vibrance reset
```

## VRR

`vrr::DisplayVrrCapability` uses `Option` for VRR, G-SYNC, FreeSync, and range
facts. `None` means the compositor did not report the fact; callers must not infer
support from DisplayPort or maximum refresh rate.

```rust
let displays = nvcontrol::vrr::detect_vrr_displays()?;
for display in displays {
    println!("{}: {:?}", display.display_name, display.supports_vrr);
}
```

See [VRR/G-SYNC](../features/vrr-gsync.md) for compositor-specific apply routes,
including the separate tested COSMIC `nvctl monitors set-vrr` path.

## HDR and color controls

HDR, gamma, color range, color space, and dithering are routed through the active
display/compositor backend. Use `nvctl display --help` for the exact current CLI;
support varies by compositor and missing capability must fail closed rather than
being reported from connector-type guesses.
