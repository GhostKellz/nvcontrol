# Legacy And Transitional Driver Support (595 and Earlier)

If you're running NVIDIA driver **595 or earlier**, do not assume the current `main` branch is the right build.

Use [nvidia-driver.md](nvidia-driver.md) as the source of truth for branch-to-version mapping. This document only expands on the older-build path.

- **590 and earlier**: use an older vibrance-compatible commit path. The documented fallback here is `v0.8.5`.
- **595**: use the transitional compatibility builds, typically `v0.8.4` or `v0.8.5`.
- **610+ open driver**: use the current `main` branch.

## Why?

Driver 595 introduced breaking changes to the NVKMS ioctl API:
- Struct sizes changed (NvKmsAllocDeviceReply, NvKmsAllocDeviceParams)
- SLI/Mosaic fields removed from NvKmsAllocDeviceRequest
- ImageSharpening attributes removed

These changes are **not backwards compatible**. The modern 610-targeted build should not be treated as the default choice for 595-and-earlier stacks.

## Quick Reference

| Driver Version | nvcontrol Version | Git Reference |
|----------------|-------------------|---------------|
| 610+ open driver | current `main` branch | latest |
| 595 | `v0.8.4` or `v0.8.5` | transitional compatibility path |
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
- 595 is a transitional branch; if one tag does not behave correctly on your system, test `v0.8.4` and `v0.8.5`
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
- Test `v0.8.4` and `v0.8.5`
- Treat 595 as a transitional branch rather than the current baseline

**EPERM errors:**
- Version mismatch between nvctl and driver
- Use the correct version per the table above
