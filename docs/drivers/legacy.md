# Legacy Driver Support (590 And Earlier)

Current nvcontrol supports the known 595 and 610 `AllocDevice` layouts at
runtime. This legacy guidance applies to driver 590 and earlier.

Use [nvidia-driver.md](nvidia-driver.md) as the source of truth for branch-to-version mapping. This document only expands on the older-build path.

- **590 and earlier**: use an older vibrance-compatible commit path. The documented fallback here is `v0.8.5`.
- **595**: use current nvcontrol with runtime NVKMS ABI selection.
- **610+ open driver**: use the current `main` branch.

## Why?

Driver 595 introduced breaking changes to the NVKMS ioctl API:
- Struct sizes changed (NvKmsAllocDeviceReply, NvKmsAllocDeviceParams)
- SLI/Mosaic fields removed from NvKmsAllocDeviceRequest
- ImageSharpening attributes removed

These changes are not ABI-compatible, so nvcontrol sends the branch-appropriate
parameter size. Pre-595 support still requires an older layout and attribute table.

## Quick Reference

| Driver Version | nvcontrol Version | Git Reference |
|----------------|-------------------|---------------|
| 610+ open driver | current `main` branch | latest |
| 595 | current multi-ABI nvcontrol | runtime-selected 595 layout |
| 560-590 | older vibrance-compatible build, commonly `v0.8.5` | `v0.8.5` tag or commit `2235bb3` |
| < 560 | v0.8.5 | Same as above (untested) |

## Building v0.8.5 for Legacy Drivers

```bash
# Clone the repo
git clone https://github.com/ghostkellz/nvcontrol.git
cd nvcontrol

# Checkout v0.8.5
git checkout v0.8.5
# Or by commit: git checkout 2235bb3

# Build
cargo build --release --bin nvctl --no-default-features

# Install
sudo cp target/release/nvctl /usr/local/bin/
```

## Download Pre-built Binary

Check the [v0.8.5 Release](https://github.com/ghostkellz/nvcontrol/releases/tag/v0.8.5) for pre-built binaries.

## Checking Your Driver Version

```bash
# Method 1: nvidia-smi
nvidia-smi --query-gpu=driver_version --format=csv,noheader

# Method 2: sysfs
cat /sys/module/nvidia/version

# Method 3: nvctl
nvctl gpu info
```

## Feature Differences

For older stacks, `v0.8.5` is the documented fallback build:
- Digital vibrance works on 590 and earlier with the older compatible build path
- 595 native vibrance is supported by the current multi-ABI path
- Image sharpening was available on 590-era drivers and removed in 595
- All other features identical

## Upgrade Path

When you upgrade to driver 610+ open:
1. Update nvcontrol to latest: `git checkout main && cargo build --release`
2. Reinstall: `sudo cp target/release/nvctl /usr/local/bin/`
3. Note: Image sharpening is no longer available (NVIDIA removed it)

## Troubleshooting

**"No connected displays found" on driver 590:**
- Make sure you're using an older vibrance-compatible build, not the current 610-targeted build
- `v0.8.5` is the first fallback to try

**595 compatibility issues:**
- Confirm `nvctl display vibrance get` reports the live value and driver range
- Capture the exact driver version and `EPERM` stage in a support bundle

**EPERM errors:**
- Often an NVKMS parameter-size mismatch rather than a Unix permission error
- Use current nvcontrol for 595/610; use the legacy build only for 590 and earlier
