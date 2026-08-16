# Driver Diagnostics API

The `drivers` module separates version-derived expectations from facts probed on
the running system.

## Status and release diagnostics

- `get_driver_status()` reports loaded version, module type, DKMS/GSP state, and
  kernel alignment.
- `collect_release_diagnostics()` gathers structured GPU/package/ownership facts.
- `summarize_release_diagnostics()` classifies the result without mutating the host.
- support-bundle helpers serialize the same evidence with optional redaction.

## NVIDIA 610 capabilities

`DriverCapabilities::from_version()` provides branch gates. Runtime helpers then
probe what the active device and kernel actually expose:

- `detect_vulkan_extensions()` runs full `vulkaninfo` with overlay variables
  disabled and recognizes the 610 extension set, `VK_EXT_descriptor_heap`, and
  `VK_KHR_video_decode_h265`;
- `detect_egl_fp16()` checks the current EGL implementation;
- DRM color status distinguishes driver/kernel readiness from active KMS;
- `is_geforce_now_flatpak_installed()` checks user and system Flatpak scopes.

```rust
let status = nvcontrol::drivers::get_driver_status()?;
let capabilities = nvcontrol::drivers::get_driver_capabilities()?;
let extensions = nvcontrol::drivers::detect_vulkan_extensions();
```

These probes report capabilities. They do not force VKD3D-Proton environment
variables, install GeForce NOW, change module parameters, or enable DRM KMS.

See [NVIDIA 610 open driver](../drivers/open-610.md) and
[GeForce NOW Linux](../integration/geforce-now.md).
