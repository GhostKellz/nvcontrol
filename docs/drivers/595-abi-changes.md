# NVIDIA Driver 595 Vibrance Compatibility

This document details the changes required to support digital vibrance on NVIDIA driver 595.45.04 (open-source kernel modules).

## Summary

The NVIDIA 595 driver introduced breaking changes to the NVKMS ioctl API that affected our native digital vibrance implementation. The key issue was **struct size mismatches** between our Rust bindings and the actual driver ABI.

## Changes Made

### 1. Struct Size Corrections

The following structs had incorrect sizes in our bindings:

| Struct | Old Size | Correct Size | Notes |
|--------|----------|--------------|-------|
| `NvKmsAllocDeviceReply` | 1248 bytes | **888 bytes** | Significantly smaller |
| `NvKmsAllocDeviceParams` | 1868 bytes | **1512 bytes** | Request + Reply with alignment |
| `NvKmsQueryDpyDynamicDataReply` | 35088 bytes | **35096 bytes** | 8 bytes larger |

### 2. Struct Field Changes

**NvKmsAllocDeviceRequest:**
- Removed `sli_mosaic` field (was at offset 40)
- Removed `try_infer_sli_mosaic_from_existing_device` field (was at offset 41)
- Fields `no3d` and `enable_console_hotplug_handling` moved to offsets 40 and 41
- Added 2-byte padding before `registryKeys` to maintain alignment

**NvKmsDpyAttribute Enum:**
- Removed `ImageSharpening` (was 11)
- Removed `ImageSharpeningAvailable` (was 12)
- Removed `ImageSharpeningDefault` (was 13)
- `DigitalVibrance` remains at index **10** (unchanged)
- `RequestedColorSpace` and subsequent attributes shifted down by 3

**NvKmsAllocDeviceStatus Enum:**
- Renamed `BadDeviceId` to `BadRequest`
- Renamed `AlreadyAllocated` to `FatalError`
- Added `NoHardwareAvailable`
- Added `CoreChannelAllocFailed`

### 3. Alignment Requirements

**NvKmsAllocDeviceReply** requires `align(8)` due to `NvU64` fields (`vtFbBaseAddress`, `vtFbSize`). Without this alignment, the combined `NvKmsAllocDeviceParams` struct was 4 bytes too small.

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

Compile with:
```bash
DRIVER_SRC="/path/to/open-gpu-kernel-modules"
gcc -o check_sizes check_sizes.c \
    -I${DRIVER_SRC}/kernel-open/common/inc \
    -I${DRIVER_SRC}/src/common/sdk/nvidia/inc \
    -I${DRIVER_SRC}/src/nvidia-modeset/interface \
    -I${DRIVER_SRC}/src/common/unix/common/inc
```

## Files Modified

- `src/nvkms_bindings.rs` - Updated struct definitions and sizes
- `src/vibrance_native.rs` - Updated NvKmsAllocDeviceRequest initialization
- `src/display_controls.rs` - Removed ImageSharpening (no longer supported)

## Symptoms of Size Mismatch

When struct sizes don't match the driver's expectations:
- `ioctl()` returns `-EPERM` (Operation not permitted)
- The reply status remains unchanged (never populated by kernel)
- The error occurs at the NVKMS dispatch level before any actual processing

The driver validates `paramSize != dispatch[cmd].paramSize` and returns `NV_FALSE` immediately if sizes don't match.

## Driver Version Detection

The driver version is read from `/sys/module/nvidia/version` and passed in the `versionString` field of `NvKmsAllocDeviceRequest`. This must exactly match the loaded driver or the ioctl will fail with `NVKMS_ALLOC_DEVICE_STATUS_VERSION_MISMATCH`.

## Testing

```bash
# Set vibrance to 200% (max saturation boost)
nvctl vibrance 200

# Reset to default (100%)
nvctl vibrance 100
```

`nvctl vibe <percent>` remains available as a shorthand alias, but `nvctl vibrance <percent>` is the primary form documented throughout the project.

## References

- NVIDIA Open GPU Kernel Modules: https://github.com/NVIDIA/open-gpu-kernel-modules
- Key header files:
  - `src/nvidia-modeset/interface/nvkms-api.h`
  - `src/nvidia-modeset/interface/nvkms-api-types.h`
  - `kernel-open/nvidia-modeset/nvkms-ioctl.h`
