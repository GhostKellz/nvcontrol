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

# Clean up old kernel modules
nvctl driver dkms cleanup
```

## Building from Source

For power users building nvidia-open from the git repository:

```bash
# Clone the repo
git clone https://github.com/NVIDIA/open-gpu-kernel-modules.git ~/open-gpu-kernel-modules
cd ~/open-gpu-kernel-modules
git checkout <driver-tag>

# Initialize (creates dkms.conf, symlink, registers with DKMS)
sudo nvctl driver source init ~/open-gpu-kernel-modules

# Build modules for all kernels
nvctl driver source sync

# Update to latest version
nvctl driver source update
```

See [Source Build Commands](#source-build-commands) for details.

## Commands

### `nvctl driver dkms status`

Shows detailed DKMS status for all installed kernels:

```
NVIDIA DKMS Status
══════════════════════════════════════════════════

DKMS:           installed
Driver:         <driver-version>
Registered:     yes
Source:         /usr/src/nvidia-<driver-version>
Source Type:    git (https://github.com/NVIDIA/open-gpu-kernel-modules.git)

Installed Kernels (4):
  ✓ <kernel-a> [nvidia: dkms, headers: ✓]
  ✓ <running-kernel> [nvidia: dkms, headers: ✓] (running)
  ✓ <kernel-b> [nvidia: dkms, headers: ✓]
  ✗ <kernel-c> [nvidia: MISSING, headers: ✗]

Pacman Hook:    installed (auto-rebuild enabled)
```

**Legend:**
- `nvidia: dkms` - Module built by DKMS
- `nvidia: manual` - Module installed manually (not DKMS-managed)
- `nvidia: MISSING` - No nvidia module for this kernel
- `headers: ✓/✗` - Whether kernel headers are installed (required for DKMS builds)

**Source Types:**
- `packaged (nvidia-open-dkms)` - Installed via pacman
- `git (url)` - From git clone (shows remote URL)
- `manual` - Manually copied to /usr/src

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
git checkout <driver-tag>
sudo cp -r . /usr/src/nvidia-<driver-version>
```

Then create `/usr/src/nvidia-<driver-version>/dkms.conf`:
```conf
PACKAGE_NAME="nvidia"
PACKAGE_VERSION="<driver-version>"
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
nvctl driver dkms build --kernel <kernel-version>

# Force rebuild even if already installed
nvctl driver dkms build --force
nvctl driver dkms build -f --kernel <kernel-version>
```

**Note:** Without `--force`, DKMS will skip kernels where nvidia is already installed. This is normal - use `--force` when you need to rebuild (e.g., after source changes).

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
nvctl driver dkms logs -k <kernel-version>
```

**Example Output:**
```
NVIDIA DKMS Build Logs
══════════════════════════════════════════════════

Pacman Hook Logs (/var/log/nvidia-dkms)
────────────────────────────────────────
✓ build-<timestamp>.log [OK] - 2m ago (156 lines)
✓ build-<older-timestamp>.log [OK] - 1d ago (142 lines)

DKMS Build Logs (/var/lib/dkms/nvidia/<driver-version>)
────────────────────────────────────────
✓ <running-kernel> [OK] - 3h ago (1842 lines)
✓ <kernel-a> [OK] - 1d ago (1836 lines)
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

### `nvctl driver dkms cleanup`

Remove nvidia modules from old kernels to free disk space:

```bash
# Dry run - shows what would be removed
nvctl driver dkms cleanup

# Keep 3 most recent kernels (plus running)
nvctl driver dkms cleanup --keep 3

# Actually remove (requires --execute)
nvctl driver dkms cleanup --execute
nvctl driver dkms cleanup --keep 2 --execute
```

**Example Output:**
```
NVIDIA DKMS Kernel Cleanup (dry run)
══════════════════════════════════════════════════

Running kernel: <running-kernel>
Keeping: 2 most recent kernels (plus running)

Keeping (3):
  ✓ <running-kernel> (running)
  ✓ <kernel-a>
  ✓ <kernel-b>

To remove (3):
  ✗ <old-kernel-a>
  ✗ <old-kernel-b>
  ✗ <old-kernel-c>

Dry run - no changes made.
Run with --execute to actually remove.
```

**Note:** This only removes nvidia modules from DKMS, not the kernel packages themselves. Use your package manager to remove unused kernels.

## Source Build Commands

For building nvidia-open directly from the git repository instead of using packaged nvidia-open-dkms.

### `nvctl driver source status`

Show source build status:

```bash
nvctl driver source status
```

**Example Output:**
```
NVIDIA Source Build Status
══════════════════════════════════════════════════

Source Path:    /usr/src/nvidia-<driver-version>
Source Type:    git (https://github.com/NVIDIA/open-gpu-kernel-modules.git)
Remote URL:     https://github.com/NVIDIA/open-gpu-kernel-modules.git
Current Tag:    <driver-tag>
Latest Tag:     <driver-tag>

Driver Version: <driver-version>
DKMS Registered: yes
```

### `nvctl driver source init`

Initialize DKMS from a git clone:

```bash
nvctl driver source init ~/open-gpu-kernel-modules
```

This will:
1. Verify it's a valid nvidia open-gpu-kernel-modules clone
2. Detect version from `version.mk`
3. Create `dkms.conf` if missing (with LLVM/Clang detection for CachyOS/TKG)
4. Create symlink in `/usr/src/nvidia-<version>`
5. Register with DKMS

### `nvctl driver source update`

Fetch and checkout the latest tag from git, then rebuild:

```bash
# Update and rebuild
nvctl driver source update

# Update without rebuilding
nvctl driver source update --no-build
```

**Note:** If the version changes, you may need to re-register with DKMS manually.

### `nvctl driver source sync`

Rebuild modules from current source without updating:

```bash
# Rebuild for all kernels
nvctl driver source sync

# Rebuild specific kernel
nvctl driver source sync --kernel <kernel-version>

# Force rebuild
nvctl driver source sync --force
```

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
nvctl driver dkms logs -k <kernel-version> -t 50

# Or view raw log files
cat /var/log/nvidia-dkms/latest.log
cat /var/lib/dkms/nvidia/<driver-version>/<kernel-version>/x86_64/log/make.log
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
