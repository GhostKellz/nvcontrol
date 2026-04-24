# Legacy Driver Support (590 and Earlier)

If you're running NVIDIA driver **590 or earlier**, you need nvcontrol **v0.8.5** for digital vibrance support.

## Why?

Driver 595 introduced breaking changes to the NVKMS ioctl API:
- Struct sizes changed (NvKmsAllocDeviceReply, NvKmsAllocDeviceParams)
- SLI/Mosaic fields removed from NvKmsAllocDeviceRequest
- ImageSharpening attributes removed

These changes are **not backwards compatible**. Using v0.8.6+ on driver 590 or earlier will fail with `EPERM` errors.

## Quick Reference

| Driver Version | nvcontrol Version | Git Reference |
|----------------|-------------------|---------------|
| 595+ | v0.8.6+ (latest) | `main` branch |
| 560-590 | v0.8.5 | `v0.8.5` tag or commit `2235bb3` |
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

v0.8.5 includes all features **except** driver 595 compatibility fixes:
- Digital vibrance (works on 590 and earlier)
- Image sharpening (available on 590 and earlier, removed in 595)
- All other features identical

## Upgrade Path

When you upgrade to driver 595+:
1. Update nvcontrol to latest: `git checkout main && cargo build --release`
2. Reinstall: `sudo cp target/release/nvctl /usr/local/bin/`
3. Note: Image sharpening is no longer available (NVIDIA removed it)

## Troubleshooting

**"No connected displays found" on driver 590:**
- Make sure you're using v0.8.5, not v0.8.6+
- Check: `nvctl --version` should show `0.8.5`

**EPERM errors:**
- Version mismatch between nvctl and driver
- Use the correct version per the table above
