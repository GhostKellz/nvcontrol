# Safety Improvements Roadmap

This document tracks remaining `unsafe` blocks in nvcontrol and plans for reducing them.

## Current Status (v0.8.5)

| Category | Count | Status |
|----------|-------|--------|
| Environment variables | 0 | ✅ Moved to `safe_env.rs` |
| NVKMS ioctl calls | 13 | Legitimate - kernel interface |
| Union field access | 1 | Legitimate - FFI type |
| libc::ioctl | 1 | Legitimate - system call |
| **Total** | **15** | Down from 31 |

## Legitimate Unsafe (Cannot Eliminate)

These unsafe blocks interact with the Linux kernel or C code and cannot be made fully safe:

### 1. `nvkms_bindings.rs` - Core ioctl
```rust
// Line 489 - Raw system call to NVIDIA kernel module
unsafe { libc::ioctl(fd, NVKMS_IOCTL_IOWR, &params) }
```
**Why unsafe?** `libc::ioctl` is a raw syscall with no Rust safety guarantees.

### 2. `vibrance_native.rs` - NVKMS operations (7 blocks)
- `AllocDevice` - Allocate NVKMS device handle
- `QueryDisp` - Query display information
- `QueryConnectorStaticData` - Get connector info
- `QueryDpyDynamicData` - Get display dynamic state
- `SetDpyAttribute` - Set vibrance/attributes
- `FreeDevice` - Release device handle

**Why unsafe?** Calling `unsafe fn nvkms_ioctl()`.

### 3. `display_controls.rs` - Display attribute access (4 blocks)
- `GetDpyAttribute` - Read display attribute
- `SetDpyAttribute` - Write display attribute
- `GetDpyAttributeValidValues` - Get valid range
- Union field access for range values

**Why unsafe?** Calling `unsafe fn nvkms_ioctl()` + accessing C union fields.

## Improvement Plan

### Phase 1: Safe Wrapper Functions (Medium Priority)

Create type-safe wrappers that encapsulate unsafe internally:

```rust
// Before (caller needs unsafe block)
unsafe {
    nvkms_ioctl(fd, NvKmsIoctlCommand::SetDpyAttribute, &mut params)?;
}

// After (safe API, unsafe hidden inside)
nvkms::set_display_attribute(fd, display_id, attr, value)?;
```

**Files to create:**
- `src/nvkms/mod.rs` - Safe NVKMS API
- `src/nvkms/device.rs` - Device allocation/free
- `src/nvkms/display.rs` - Display queries
- `src/nvkms/attributes.rs` - Attribute get/set

**Benefits:**
- Reduces unsafe blocks from 15 to ~3 (one per module)
- Type-safe API prevents misuse
- Better error messages

### Phase 2: RAII Device Handle (Low Priority)

Create a `NvkmsDevice` struct that manages lifetime:

```rust
pub struct NvkmsDevice {
    fd: OwnedFd,
    device_handle: u32,
}

impl NvkmsDevice {
    pub fn open() -> Result<Self, NvControlError> { ... }
    pub fn set_vibrance(&self, display: u32, level: i32) -> Result<()> { ... }
}

impl Drop for NvkmsDevice {
    fn drop(&mut self) {
        // Automatically calls FreeDevice
    }
}
```

**Benefits:**
- Automatic resource cleanup
- Impossible to use after close
- Cleaner API

### Phase 3: Consider `nix` Crate Integration (Low Priority)

The `nix` crate provides safe wrappers for many syscalls. Could potentially:
- Use `nix::ioctl_readwrite!` macro for type-safe ioctl
- Replace raw `libc::ioctl` with nix wrapper

**Blockers:**
- NVKMS is NVIDIA-proprietary, no upstream nix support
- Would need custom ioctl definitions

## Not Planned

These will remain unsafe as they're fundamental to the architecture:

1. **`libc::ioctl`** - Cannot wrap a syscall safely without knowing the specific ioctl
2. **Union field access** - C unions have no Rust equivalent for safe access
3. **Raw pointer passing to kernel** - Required by ioctl interface

## References

- [Rust Unsafe Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/)
- [NVIDIA NVKMS Documentation](https://github.com/NVIDIA/open-gpu-kernel-modules)
- [nix crate ioctl](https://docs.rs/nix/latest/nix/sys/ioctl/index.html)

---

*Last updated: v0.8.5 (2026-02-12)*
