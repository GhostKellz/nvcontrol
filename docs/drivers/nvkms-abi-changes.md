# NVKMS ABI Changes

Tracks breaking NVKMS ioctl struct size changes across NVIDIA open driver releases. When struct sizes don't match the driver's expectations, `ioctl()` returns `-EPERM` — the kernel validates `paramSize != dispatch[cmd].paramSize` and returns `NV_FALSE` immediately.

## Driver 610.43.02

**Breaking change:** `NvKmsAllocDeviceReply` reduced from 888 to **816 bytes** (72 bytes removed from internal capability fields).

Local verification source for the v0.8.10 release pass: `~/open-gpu-kernel-modules/version.mk` reports `NVIDIA_VERSION = 610.43.02`, with the authoritative `NvKmsAllocDeviceReply` definition in `src/nvidia-modeset/interface/nvkms-api.h`.

| Struct | 595 Size | 610 Size | Delta |
|--------|----------|----------|-------|
| `NvKmsAllocDeviceReply` | 888 | **816** | -72 |
| `NvKmsAllocDeviceParams` | 1512 | **1440** | -72 |

All other NVKMS structs remained unchanged (AllocDeviceRequest=620, QueryDpyDynamicDataReply=35096, QueryDispParams=172, etc).

**New driver capabilities (610+):**
- Vulkan: `VK_KHR_device_group_creation`, `VK_EXT_shader_long_vector`, `VK_KHR_internally_synchronized_queues`, `VK_NV_push_constant_bank`
- FP16 EGL framebuffer config on Wayland
- DRM format modifiers for multiplanar YCbCr formats
- `mmap` on DMABUF file descriptors exported from discrete GPUs
- Per-plane DRM color pipeline support (kernel 6.19+)

**Fix:** Updated `_padding` in `NvKmsAllocDeviceReply` from `888 - 52 = 836` to `816 - 52 = 764`.

---

## Driver 595.45.04

**Breaking changes:** Multiple struct size corrections and enum changes from pre-595 bindings.

| Struct | Old Size | 595 Size | Notes |
|--------|----------|----------|-------|
| `NvKmsAllocDeviceReply` | 1248 | **888** | Significantly smaller |
| `NvKmsAllocDeviceParams` | 1868 | **1512** | Request + Reply with alignment |
| `NvKmsQueryDpyDynamicDataReply` | 35088 | **35096** | 8 bytes larger |

### Struct Field Changes

**NvKmsAllocDeviceRequest:**
- Removed `sli_mosaic` field (was at offset 40)
- Removed `try_infer_sli_mosaic_from_existing_device` field (was at offset 41)
- Fields `no3d` and `enable_console_hotplug_handling` moved to offsets 40 and 41
- Added 2-byte padding before `registryKeys` to maintain alignment

**NvKmsDpyAttribute Enum:**
- Removed `ImageSharpening` (was 11), `ImageSharpeningAvailable` (was 12), `ImageSharpeningDefault` (was 13)
- `DigitalVibrance` remains at index **10** (unchanged across all versions)
- `RequestedColorSpace` and subsequent attributes shifted down by 3

**NvKmsAllocDeviceStatus Enum:**
- Renamed `BadDeviceId` to `BadRequest`
- Renamed `AlreadyAllocated` to `FatalError`
- Added `NoHardwareAvailable` and `CoreChannelAllocFailed`

### Alignment Requirements

`NvKmsAllocDeviceReply` requires `align(8)` due to `NvU64` fields (`vtFbBaseAddress`, `vtFbSize`).

---

## How to Verify Struct Sizes

Compile a test program against the NVIDIA driver headers:

```c
#include "nvtypes.h"
#include "nvmisc.h"
#include "nvlimits.h"
#include "nvkms-api-types.h"
#include "nvkms-api.h"

int main() {
    printf("NvKmsAllocDeviceRequest: %zu\n", sizeof(struct NvKmsAllocDeviceRequest));
    printf("NvKmsAllocDeviceReply: %zu\n", sizeof(struct NvKmsAllocDeviceReply));
    printf("NvKmsAllocDeviceParams: %zu\n", sizeof(struct NvKmsAllocDeviceParams));
    return 0;
}
```

```bash
DRIVER_SRC="/path/to/open-gpu-kernel-modules"
gcc -o check_sizes check_sizes.c \
    -I${DRIVER_SRC}/kernel-open/common/inc \
    -I${DRIVER_SRC}/src/common/sdk/nvidia/inc \
    -I${DRIVER_SRC}/src/nvidia-modeset/interface \
    -I${DRIVER_SRC}/src/common/unix/common/inc
```

## Driver Version Detection

The driver version is read from `/sys/module/nvidia/version` and passed in the `versionString` field of `NvKmsAllocDeviceRequest`. This must exactly match the loaded driver or the ioctl will fail with `NVKMS_ALLOC_DEVICE_STATUS_VERSION_MISMATCH`.

## References

- NVIDIA Open GPU Kernel Modules: https://github.com/NVIDIA/open-gpu-kernel-modules
- Key header files:
  - `src/nvidia-modeset/interface/nvkms-api.h`
  - `src/nvidia-modeset/interface/nvkms-api-types.h`
  - `kernel-open/nvidia-modeset/nvkms-ioctl.h`
