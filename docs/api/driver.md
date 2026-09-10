# Driver Diagnostics API

The `drivers` module separates version-derived expectations from facts probed on
the running system.

## Status and release diagnostics

- `get_driver_status()` reports loaded version, module type, DKMS/GSP state, and
  kernel alignment.
- `collect_release_diagnostics()` gathers structured GPU/package/ownership facts.
- `summarize_release_diagnostics()` classifies the result without mutating the host.
- support-bundle helpers serialize the same evidence with optional redaction.

## Driver and runtime capabilities

`DriverCapabilities::from_version()` provides branch gates. Runtime helpers then
probe what the active device and kernel actually expose:

- `detect_vulkan_extensions()` runs full `vulkaninfo` with overlay variables
  disabled and recognizes the existing extension set plus `VK_NV_low_latency`,
  `VK_NV_low_latency2`, and `VK_EXT_cluster_acceleration_structure`;
- `detect_egl_fp16()` checks the current EGL implementation;
- DRM color status distinguishes driver/kernel readiness from active KMS;
- `is_geforce_now_flatpak_installed()` checks user and system Flatpak scopes.

```rust
let status = nvcontrol::drivers::get_driver_status()?;
let capabilities = nvcontrol::drivers::get_driver_capabilities()?;
let extensions = nvcontrol::drivers::detect_vulkan_extensions();
```

`DriverCapabilities::detect()` verifies low-latency revision 2 and cluster
acceleration from runtime output. `from_version()` leaves those runtime fields
false until probed. The cgroups, 12 bpc, and display-clock override fields report
driver support; they do not assert that the hardware or kernel is configured.

These probes report capabilities. They do not force VKD3D-Proton environment
variables, install GeForce NOW, change module parameters, or enable DRM KMS.

See [NVIDIA 615 support](../drivers/open-615.md), [NVIDIA 610 open driver](../drivers/open-610.md) and
[GeForce NOW Linux](../integration/geforce-now.md).
