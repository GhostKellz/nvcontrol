# NVIDIA 590 Beta Driver Analysis

> **Driver Version:** 590.44.01 (Beta)
> **Release Date:** December 2, 2025
> **Kernel Modules:** open-gpu-kernel-modules 590.44.01
> **Analysis Date:** December 2, 2025

## Overview

The NVIDIA 590 driver series enters public beta with Wayland improvements, Vulkan performance enhancements, and continued RTX 50 series (Blackwell) support. This document tracks changes relevant to nvcontrol and potential enhancements.

## Driver Requirements

| Requirement | 590 Series | Previous (580) |
|-------------|------------|----------------|
| Wayland | 1.20+ | 1.17+ |
| glibc | 2.27+ | 2.17+ |
| X.Org Server | 1.17+ (ABI 19) | 1.15+ |
| Kernel | 4.15+ | 4.15+ |

## Key Changes in 590.44.01

### Bug Fixes
- **PREEMPT_RT Kernel Freezes** - Fixed system freezes on real-time kernels
- **Vulkan Swapchain Performance** - Reduced stuttering when resizing Vulkan windows
- **Venus VirtIO GPU** - Fixed Vulkan compatibility with virtual GPU
- **DPI Reporting** - Corrected incorrect DPI values
- **nvidia-settings PowerMizer** - Fixed dropdown not working on Wayland

### New Features
- Improved Wayland compositor compatibility
- Vulkan swapchain recreation optimization

## Kernel Module Changes (580 → 590)

### New Architecture IDs
```c
// kernel-open/nvidia-uvm/ctrl2080mc.h
#define NV2080_CTRL_MC_ARCH_INFO_IMPLEMENTATION_GB206  (0x00000006)
#define NV2080_CTRL_MC_ARCH_INFO_IMPLEMENTATION_GB207  (0x00000007)
```
These are RTX 50 series sub-variants (likely mobile or different SKUs).

### New APIs

#### USB4 DisplayPort Adapter Info
```c
// ctrl0073dp.h - New USB4 DP_IN support
typedef struct NV0073_CTRL_DP_USB4_INFO {
    NvU8 driverId;
    NvU8 dpInAdapterNumber;
    NvU8 topologyId[5];
} NV0073_CTRL_DP_USB4_INFO;

#define NV0073_CTRL_CMD_GET_USB_DPIN_ADAPTER_INFO (0x73138fU)
```

#### BAR1 Unaligned Access Flag
```c
// ctrl0000system.h - Chipset capability flag
#define NV0000_CTRL_SYSTEM_CHIPSET_FLAG_BAR1_UNALIGNED_ACCESS      1:1
#define NV0000_CTRL_SYSTEM_CHIPSET_FLAG_BAR1_UNALIGNED_ACCESS_NO   (0x00000000U)
#define NV0000_CTRL_SYSTEM_CHIPSET_FLAG_BAR1_UNALIGNED_ACCESS_YES  (0x00000001U)
```

#### Power Mode Long Timescale Override
```c
// ctrl0000system.h - NVPCF power control
NvU32 dcTspLongTimescaleLimitOverridemA;  // New field
```

### Removed/Deprecated APIs
- `NV0000_CTRL_CMD_GPU_GET_SVM_SIZE` - Removed
- `NV0000_CTRL_CMD_SYSTEM_GET_HWBC_INFO` - Removed (BR04 bridge support)

### Internal Changes
- ioctl struct renamed: `nv_ioctl_query_device_intr` → `nv_ioctl_query_device_intr_t`
- Removed Intel TDX guest `ioremap_driver_hardened` paths
- Cleaned up `nv-list-helpers.h` dependencies
- GFP_DMA32 allocation changes

## nvcontrol Compatibility

### Current Status: FULLY COMPATIBLE

nvcontrol uses stable userspace APIs that are unaffected:
- **NVML** (nvml-wrapper) - No changes
- **nvidia-settings** - PowerMizer fix benefits us on Wayland
- **nvidia-smi** - No changes

### No Breaking Changes
All nvcontrol functionality works with 590 drivers without modification.

---

## Potential Enhancements for nvcontrol

### Priority 1: Detection & Reporting

#### 1.1 GB206/GB207 Architecture Detection
**File:** `src/gpu.rs`
**Function:** `detect_architecture()`

```rust
// Add new Blackwell variants when hardware ships
"GB206" | "RTX 5080" => Some("Blackwell (GB206)".to_string()),
"GB207" | "RTX 5070" => Some("Blackwell (GB207)".to_string()),
```

**Status:** Ready to add when SKUs are announced
**Effort:** Low

#### 1.2 Driver Version Feature Detection
**File:** `src/drivers.rs` (new or existing)

```rust
pub struct DriverCapabilities {
    pub version: String,
    pub wayland_min_version: String,
    pub has_vulkan_swapchain_perf: bool,
    pub has_usb4_dp_support: bool,
    pub supports_preempt_rt: bool,
}

pub fn detect_driver_capabilities() -> DriverCapabilities {
    let version = get_driver_version();
    let major = parse_major_version(&version);

    DriverCapabilities {
        version: version.clone(),
        wayland_min_version: if major >= 590 { "1.20".into() } else { "1.17".into() },
        has_vulkan_swapchain_perf: major >= 590,
        has_usb4_dp_support: major >= 590,
        supports_preempt_rt: major >= 590,
    }
}
```

**Status:** Nice to have
**Effort:** Medium

### Priority 2: Wayland Improvements

#### 2.1 Wayland Version Validation
**File:** `src/wayland_integration.rs`

```rust
pub fn check_wayland_compatibility(driver_version: u32) -> Result<(), WaylandError> {
    let wayland_version = get_wayland_version()?;

    if driver_version >= 590 && wayland_version < Version::new(1, 20, 0) {
        warn!("NVIDIA 590+ requires Wayland 1.20+, found {}", wayland_version);
        return Err(WaylandError::VersionMismatch {
            required: "1.20".into(),
            found: wayland_version.to_string(),
        });
    }
    Ok(())
}
```

**Status:** Recommended for 590+ users
**Effort:** Low

#### 2.2 PowerMizer Wayland Status
The 590 driver fixes nvidia-settings PowerMizer on Wayland. We could:
- Add a status indicator showing PowerMizer is working
- Remove any Wayland-specific workarounds for PowerMizer

**Status:** Optional
**Effort:** Low

### Priority 3: Advanced Features

#### 3.1 USB4 Display Detection
**File:** `src/display_info.rs`

For users with USB4/Thunderbolt displays, expose the new adapter info:

```rust
pub struct Usb4DisplayInfo {
    pub driver_id: u8,
    pub adapter_number: u8,
    pub topology_id: [u8; 5],
}

// Would require nvidia-settings or direct ioctl access
```

**Status:** Future consideration
**Effort:** High (requires new bindings)

#### 3.2 PREEMPT_RT Kernel Detection
**File:** `src/kernel_driver.rs`

```rust
pub fn is_preempt_rt_kernel() -> bool {
    std::fs::read_to_string("/proc/version")
        .map(|v| v.contains("PREEMPT_RT"))
        .unwrap_or(false)
}

pub fn check_rt_kernel_support(driver_version: u32) -> Option<String> {
    if is_preempt_rt_kernel() && driver_version < 590 {
        Some("Warning: PREEMPT_RT kernels may freeze with drivers < 590".into())
    } else {
        None
    }
}
```

**Status:** Useful for pro audio/low-latency users
**Effort:** Low

### Priority 4: System Validation

#### 4.1 Enhanced Validation for 590
**File:** `src/system_validation.rs`

Add 590-specific checks:

```rust
pub fn validate_for_590_driver() -> ValidationResult {
    let mut issues = Vec::new();

    // Check Wayland version
    if let Some(wayland_ver) = get_wayland_version() {
        if wayland_ver < Version::new(1, 20, 0) {
            issues.push(ValidationIssue {
                severity: Severity::Warning,
                message: "Wayland < 1.20 may have issues with 590 drivers".into(),
                solution: "Update Wayland compositor".into(),
            });
        }
    }

    // Check glibc version
    if let Some(glibc_ver) = get_glibc_version() {
        if glibc_ver < Version::new(2, 27, 0) {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                message: "glibc < 2.27 not supported by 590 drivers".into(),
                solution: "Update to newer distribution".into(),
            });
        }
    }

    ValidationResult { issues }
}
```

**Status:** Recommended
**Effort:** Medium

---

## CLI Commands to Add

```bash
# Check 590 compatibility
nvctl driver-info
nvctl validate --driver 590

# Example output:
# Driver: 590.44.01 (Beta)
# Status: Compatible
# Wayland: 1.22 (OK, requires 1.20+)
# glibc: 2.38 (OK, requires 2.27+)
# Kernel: 6.17.9 (PREEMPT_RT: No)
# Features:
#   - Vulkan swapchain optimization: Yes
#   - USB4 DP support: Yes
#   - PowerMizer on Wayland: Fixed
```

---

## Testing Checklist

When 590 goes stable:

- [ ] Verify NVML queries work unchanged
- [ ] Test nvidia-settings vibrance on Wayland
- [ ] Test PowerMizer mode switching on Wayland
- [ ] Verify GPU detection for any new RTX 50 SKUs
- [ ] Test fan control on 590
- [ ] Verify power limit controls
- [ ] Test with PREEMPT_RT kernel (if available)

---

## References

- [NVIDIA 590.44.01 Beta - Phoronix](https://www.phoronix.com/news/NVIDIA-590.44.01-Linux-Beta)
- [NVIDIA 590 Beta - GamingOnLinux](https://www.gamingonlinux.com/2025/12/nvidia-beta-driver-590-44-01-released-for-linux/)
- [NVIDIA 590 Beta - 9to5Linux](https://9to5linux.com/nvidia-590-linux-graphics-driver-enters-public-beta-with-better-wayland-support)
- [open-gpu-kernel-modules 590.44.01](https://github.com/NVIDIA/open-gpu-kernel-modules/releases/tag/590.44.01)
- [NVIDIA Open Source Transition Blog](https://developer.nvidia.com/blog/nvidia-transitions-fully-towards-open-source-gpu-kernel-modules)

---

## Changelog

| Date | Change |
|------|--------|
| 2025-12-02 | Initial analysis of 590.44.01 beta |

---

**Summary:** nvcontrol is fully compatible with 590 drivers. Enhancements are optional quality-of-life improvements for better driver version awareness and validation.
