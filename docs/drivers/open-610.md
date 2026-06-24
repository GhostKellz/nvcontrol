# NVIDIA 610 Driver Analysis

> **Driver Version:** 610.43.02
> **Kernel Modules:** open-gpu-kernel-modules 610.43.02
> **Minimum Kernel:** 6.6+ (7.0+ recommended)

## Overview

The NVIDIA 610 driver series is the first stable release following the 595 branch. It introduces new Vulkan extensions, FP16 EGL framebuffer support on Wayland, DMABUF mmap for discrete GPUs, and DRM color pipeline support (kernel 6.19+). This release also includes an NVKMS ABI break that required nvcontrol updates.

For the full nvcontrol version matrix across 590, 595, and 610, see [nvidia-driver.md](nvidia-driver.md).

## Driver Requirements

| Requirement | 610 Series | Previous (595) |
|-------------|------------|----------------|
| Wayland | 1.20+ | 1.20+ |
| glibc | 2.27+ | 2.27+ |
| X.Org Server | 1.17+ (ABI 19) | 1.17+ |
| Kernel | 4.15+ (6.6+ recommended) | 4.15+ |
| Kernel Modules | Open only | Open only |

## Key Changes in 610.43.02

### NVKMS ABI Break

The `NvKmsAllocDeviceReply` struct size changed from **888 bytes** (595) to **816 bytes** (610) — a 72-byte reduction. This is a silent ABI break: the kernel validates `paramSize` on every ioctl and returns `EPERM` when the size doesn't match, which looks like a permissions error but is actually a struct mismatch.

**Impact on nvcontrol:** The padding in `src/nvkms_bindings.rs` was updated from 888 to 816 bytes. Without this fix, all NVKMS ioctls (vibrance, display detection) fail with `EPERM`.

**Verification method:**
```c
// Compile against 610 headers to verify struct sizes
#include "nvidia-modeset/nvkms-api-types.h"
#include "nvidia-modeset/nvkms-api.h"
printf("NvKmsAllocDeviceReply: %zu\n", sizeof(struct NvKmsAllocDeviceReply));
// Expected: 816
```

### New Vulkan Extensions

| Extension | Purpose |
|-----------|---------|
| `VK_KHR_device_group_creation` | Logical devices from multiple physical devices |
| `VK_EXT_shader_long_vector` | Extended vector types in shaders |
| `VK_KHR_internally_synchronized_queues` | Driver-managed queue synchronization |
| `VK_NV_push_constant_bank` | Extended push constant storage |

nvcontrol detects these at runtime via `vulkaninfo --summary` in the `detect_vulkan_extensions()` helper.

### FP16 EGL on Wayland

Driver 610+ adds `EGL_EXT_pixel_format_float` support, enabling 16-bit floating-point framebuffer configs on Wayland compositors. This improves HDR and wide-gamut color rendering.

nvcontrol detects this via `eglinfo` output and surfaces it in:
- `nvctl driver info` (610+ Features section)
- `nvctl wayland status` (Capabilities section)

### DMABUF mmap for Discrete GPUs

Support for `mmap()` on DMA-BUF file descriptors exported from discrete NVIDIA GPUs. Previously only available on integrated GPUs.

### DRM Color Pipeline (Kernel 6.19+)

Per-plane DRM color pipeline support in the `nvidia-drm` kernel module. Requires both driver 610+ **and** kernel 6.19+. nvcontrol checks both conditions and reports:
- "Active" — driver 610+ and kernel 6.19+
- "Driver ready (kernel 6.19+ required)" — driver 610+ but kernel too old
- "No" — driver < 610

### Multiplanar YCbCr DRM Format Modifiers

Support for DRM format modifiers on multiplanar YCbCr formats, improving video decode and display pipeline efficiency.

## nvcontrol Compatibility

### Changes Required for 610

| Component | Change | File |
|-----------|--------|------|
| NVKMS bindings | AllocDeviceReply padding 888→816 | `src/nvkms_bindings.rs` |
| Driver capabilities | Added 4 new 610+ flags | `src/drivers.rs` |
| Runtime detection | Vulkan, EGL, kernel version helpers | `src/drivers.rs` |
| Wayland integration | FP16 EGL capability field | `src/wayland_integration.rs` |
| CLI output | 610+ features section in `driver info` | `src/drivers.rs` |

### Capability Flags Added

```rust
// DriverCapabilities (610+ gating)
pub has_vulkan_device_group: bool,
pub has_fp16_egl_wayland: bool,
pub has_dmabuf_mmap: bool,
pub has_drm_color_pipeline: bool,
```

## Testing Checklist

- [x] `nvctl vibrance 200` — sets vibrance on all displays
- [x] `nvctl vibrance 100` — resets vibrance
- [x] `nvctl driver info` — shows 610+ features section
- [x] `nvctl wayland status` — shows FP16 EGL capability
- [x] `cargo test` — all tests pass (346 total)
- [x] NVKMS struct sizes verified against local `~/open-gpu-kernel-modules` headers at `NVIDIA_VERSION = 610.43.02`
- [ ] `vulkaninfo --summary` — verify extension detection on live system
- [ ] `eglinfo` — verify FP16 EGL detection on live system

## Hardware Validation Notes

| GPU Family | 610+ Open Driver Status | nvcontrol Status |
|------------|--------------------------|------------------|
| RTX 50 / Blackwell | Primary target, needs broader tester coverage | ABI and capability path implemented; live hardware validation still requested |
| RTX 40 / Ada | Expected supported path | Needs repeat smoke tests for vibrance, VRR, support bundle, and setup check |
| RTX 30 / Ampere | Expected supported path | Needs repeat smoke tests for vibrance, VRR, support bundle, and setup check |
| Older 595/590-era systems | Use compatibility matrix | Prefer older nvcontrol builds where NVKMS layout compatibility matters |

## References

- [open-gpu-kernel-modules 610.43.02](https://github.com/NVIDIA/open-gpu-kernel-modules/releases/tag/610.43.02)
- [NVKMS ABI Changes](nvkms-abi-changes.md)

---

## Changelog

| Date | Change |
|------|--------|
| 2026-06-23 | Added v0.8.9 setup/support-bundle validation notes and local 610.43.02 source verification reference |
| 2026-05-26 | Initial analysis of 610.43.02 — NVKMS ABI fix, capability flags, runtime detection |
