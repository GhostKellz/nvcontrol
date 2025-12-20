# NVIDIA DKMS Integration

Automatic kernel module management for nvidia-open drivers on Arch-based systems.

## Quick Start

```bash
# Check current status
nvctl driver dkms status

# Set up DKMS (if nvidia not registered)
nvctl driver dkms setup

# Build for all kernels
nvctl driver dkms build

# Install auto-rebuild hook for Arch
nvctl driver dkms hook
```

## Commands

### `nvctl driver dkms status`

Shows detailed DKMS status for all installed kernels:

```
NVIDIA DKMS Status
══════════════════════════════════════════════════

DKMS:           installed
Driver:         590.48.01
Registered:     yes
Source:         /usr/src/nvidia-590.48.01

Installed Kernels (4):
  ✓ 6.18.1-zen1-2-zen [nvidia: dkms, headers: ✓]
  ✓ 6.18.2-1-cachyos-lto [nvidia: dkms, headers: ✓] (running)
  ✓ 6.18.2-273-tkg-linux-ghost [nvidia: dkms, headers: ✓]
  ✗ 6.18.1-1-cachyos-lto [nvidia: MISSING, headers: ✗]

Pacman Hook:    installed (auto-rebuild enabled)
```

Legend:
- `nvidia: dkms` - Module built by DKMS
- `nvidia: manual` - Module installed manually (not DKMS-managed)
- `nvidia: MISSING` - No nvidia module for this kernel
- `headers: ✓/✗` - Whether kernel headers are installed (required for DKMS builds)

### `nvctl driver dkms setup`

Registers nvidia-open with DKMS. Requirements:
- DKMS installed (`sudo pacman -S dkms`)
- nvidia source in `/usr/src/nvidia-<version>/`
- `dkms.conf` in the source directory

For Arch Linux, the easiest path is:
```bash
sudo pacman -S nvidia-open-dkms
```

For manual builds from source:
```bash
git clone https://github.com/NVIDIA/open-gpu-kernel-modules.git
cd open-gpu-kernel-modules
git checkout 590.48.01
sudo cp -r . /usr/src/nvidia-590.48.01
```

Then create `/usr/src/nvidia-590.48.01/dkms.conf`:
```conf
PACKAGE_NAME="nvidia"
PACKAGE_VERSION="590.48.01"
BUILT_MODULE_NAME[0]="nvidia"
BUILT_MODULE_NAME[1]="nvidia-modeset"
BUILT_MODULE_NAME[2]="nvidia-drm"
BUILT_MODULE_NAME[3]="nvidia-uvm"
DEST_MODULE_LOCATION[0]="/kernel/drivers/video"
DEST_MODULE_LOCATION[1]="/kernel/drivers/video"
DEST_MODULE_LOCATION[2]="/kernel/drivers/video"
DEST_MODULE_LOCATION[3]="/kernel/drivers/video"
AUTOINSTALL="yes"
MAKE[0]="make -j$(nproc) NV_KERNEL_MODULES=1 NV_KERNEL_SOURCES=/lib/modules/$kernelver/build modules"
CLEAN="make clean"
```

### `nvctl driver dkms build`

Builds nvidia modules for all kernels with headers:

```bash
# Build for all kernels
nvctl driver dkms build

# Build for specific kernel
nvctl driver dkms build --kernel 6.18.2-1-cachyos-lto
```

### `nvctl driver dkms unregister`

Removes nvidia from DKMS. Does not remove existing modules from `/lib/modules`.

### `nvctl driver dkms logs`

View DKMS build logs to diagnose failures:

```bash
# Summary of all builds (shows OK/FAILED status)
nvctl driver dkms logs

# Show last 50 lines of each log
nvctl driver dkms logs -t 50

# Logs for specific kernel only
nvctl driver dkms logs -k 6.18.2-1-cachyos-lto
```

**Example Output:**
```
NVIDIA DKMS Build Logs
══════════════════════════════════════════════════

Pacman Hook Logs (/var/log/nvidia-dkms)
────────────────────────────────────────
✓ build-20251219-235312.log [OK] - 2m ago (156 lines)
✓ build-20251218-143022.log [OK] - 1d ago (142 lines)

DKMS Build Logs (/var/lib/dkms/nvidia/590.48.01)
────────────────────────────────────────
✓ 6.18.2-1-cachyos-lto [OK] - 3h ago (1842 lines)
✓ 6.18.1-zen1-2-zen [OK] - 1d ago (1836 lines)
```

Log locations:
- **Pacman hook logs:** `/var/log/nvidia-dkms/` (timestamped, symlink at `latest.log`)
- **DKMS internal logs:** `/var/lib/dkms/nvidia/<version>/<kernel>/x86_64/log/make.log`

### `nvctl driver dkms hook`

Installs a pacman hook with enhanced logging and desktop notifications:

**Files installed:**
```
/etc/pacman.d/hooks/nvidia-dkms.hook    # Pacman hook
/usr/local/bin/nvidia-dkms-build        # Wrapper script with logging
```

**Features:**
- Logs all builds to `/var/log/nvidia-dkms/build-YYYYMMDD-HHMMSS.log`
- Symlinks latest log to `/var/log/nvidia-dkms/latest.log`
- Desktop notification on success (low priority)
- Desktop notification on failure (critical priority)
- Visible error box in terminal if build fails

**Triggers on:**
- nvidia-open / nvidia-open-dkms package updates
- linux / linux-lts / linux-zen / linux-cachyos / linux-tkg-* kernel updates

**On failure, you'll see:**
```
╔════════════════════════════════════════════════════════════╗
║  NVIDIA DKMS build failed! Check logs:                     ║
║    nvctl driver dkms logs                                  ║
║    cat /var/log/nvidia-dkms/latest.log                     ║
╚════════════════════════════════════════════════════════════╝
```

### `nvctl driver dkms fix`

Attempts to fix common DKMS issues:
- Runs `dkms autoinstall`
- Removes and re-adds nvidia modules
- Rebuilds for current kernel

## Problem: Kernel Updates Breaking nvidia

**Symptom:** After updating your kernel (especially tkg/CachyOS kernels), nvidia driver fails to load.

**Cause:** Manually-installed nvidia modules are compiled for a specific kernel version. They don't auto-rebuild for new kernels.

**Solution:** Set up DKMS:

1. Check current status:
   ```bash
   nvctl driver dkms status
   # Shows: Registered: no, nvidia: manual
   ```

2. Install nvidia-open-dkms (Arch):
   ```bash
   sudo pacman -S nvidia-open-dkms
   ```

3. Build for all kernels:
   ```bash
   nvctl driver dkms build
   ```

4. Install auto-rebuild hook:
   ```bash
   sudo nvctl driver dkms hook
   ```

## Problem: Missing Kernel Headers

**Symptom:** DKMS build fails with "unable to find kernel source tree".

**Solution:** Install headers for your kernel:

```bash
# CachyOS
sudo pacman -S linux-cachyos-headers linux-cachyos-lto-headers

# Zen
sudo pacman -S linux-zen-headers

# TKG (from AUR)
# Headers usually built alongside kernel
```

## Common Issues

### "nvidia module not found after reboot"

1. Check if DKMS built for running kernel:
   ```bash
   nvctl driver dkms status
   ```

2. If kernel shows `MISSING`:
   ```bash
   nvctl driver dkms build --kernel $(uname -r)
   ```

### "dkms: module nvidia is not found"

nvidia not registered with DKMS:
```bash
nvctl driver dkms setup
```

### "modprobe: FATAL: Module nvidia not found"

Multiple possible causes:
1. DKMS didn't build for this kernel
2. initramfs needs rebuild

Fix:
```bash
nvctl driver dkms build
sudo mkinitcpio -P  # Arch
```

## Debugging Build Failures

When DKMS builds fail, use the logs command to diagnose:

```bash
# Quick check - shows OK/FAILED for each build
nvctl driver dkms logs

# See full error output
nvctl driver dkms logs -t 100

# Check specific kernel
nvctl driver dkms logs -k 6.18.2-1-cachyos-lto -t 50

# Or view raw log files
cat /var/log/nvidia-dkms/latest.log
cat /var/lib/dkms/nvidia/590.48.01/6.18.2-1-cachyos-lto/x86_64/log/make.log
```

**Common errors in logs:**

| Error | Cause | Fix |
|-------|-------|-----|
| `kernel source tree not found` | Missing headers | `sudo pacman -S linux-cachyos-headers` |
| `No rule to make target` | Source incomplete | Re-clone to `/usr/src/nvidia-<ver>` |
| `GPL-incompatible module` | Wrong driver type | Use nvidia-open, not proprietary |
| `modpost: module nvidia uses symbol` | Kernel mismatch | Rebuild with correct kernel headers |

## Integration with GSP

When using nvidia-open with GSP enabled:
- DKMS builds include GSP-enabled modules
- GSP firmware is separate from kernel modules
- See `nvctl driver gsp status` for GSP state

```bash
nvctl driver info
# Shows both DKMS and GSP status
```

## References

- [DKMS Documentation](https://github.com/dell/dkms)
- [Arch Wiki - DKMS](https://wiki.archlinux.org/title/Dynamic_Kernel_Module_Support)
- [NVIDIA open-gpu-kernel-modules](https://github.com/NVIDIA/open-gpu-kernel-modules)
