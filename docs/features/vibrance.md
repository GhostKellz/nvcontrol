# Digital Vibrance - Pure Rust Implementation

## Overview

nvcontrol implements **pure Rust digital vibrance control** that works on both Wayland and X11, with zero external dependencies except for the NVIDIA driver itself.

## Quick Start

```bash
# Set vibrance to 150% (enhanced colors)
nvctl vibrance 150

# Shorthand alias
nvctl vibe 150

# Reset to default (100%)
nvctl vibrance 100

# Maximum saturation (200%)
nvctl vibrance 200

# Reduced saturation (50%)
nvctl vibrance 50
```

## How It Works

### Architecture

nvcontrol uses a **dual-path approach** for maximum compatibility:

1. **Primary: NVKMS ioctls** (requires permissions)
   - Direct `/dev/nvidia-modeset` device communication
   - Uses NVIDIA's official NVKMS API from open-gpu-kernel-modules
   - Zero overhead, instant response
   - Requires: user in `video` group OR sudo

2. **Fallback: nvidia-settings** (works everywhere)
   - Calls `nvidia-settings -a DigitalVibrance=X`
   - Works without special permissions
   - Compatible with existing setups

### Vibrance Range

| Percentage | Raw Value | Effect |
|------------|-----------|--------|
| 0%         | -1024     | Grayscale |
| 50%        | -512      | Desaturated |
| 100%       | 0         | Default (no effect) |
| 150%       | 512       | Enhanced colors (+50%) |
| 200%       | 1023      | Maximum saturation |

**Formula:**
- `0-100%` maps to `-1024` to `0`
- `100-200%` maps to `0` to `1023`

## Advanced Usage

### Per-Display Control

```bash
# Set specific display
nvctl display vibrance set-display 0 150

# Query current value
nvctl display vibrance get

# List all displays
nvctl display vibrance list
```

### Display Information

```bash
# Show comprehensive vibrance info
nvctl display vibrance info
```

**Output:**
```
🌈 Pure Rust Digital Vibrance Information:
══════════════════════════════════════════════════
  Driver Version: "580.95.05"
  NVIDIA Open Drivers: ✅ Yes

💡 Features:
  ✅ Direct driver integration (no external deps)
  ✅ Works on Wayland and X11
  ✅ Per-display control
  ✅ Real-time adjustment

🖥️ Supported Displays: 2

🔧 Requirements:
  • NVIDIA Open Drivers 580+
  • nvidia_drm.modeset=1 kernel parameter
  • /dev/nvidia-modeset access (or run as root)
```

### Systemd Auto-Start

Create `~/.config/systemd/user/nvctl-vibrance.service`:

```ini
[Unit]
Description=Apply digital vibrance on startup
After=graphical.target

[Service]
Type=oneshot
ExecStartPre=/bin/sleep 3
ExecStart=/usr/local/bin/nvctl vibe 150

[Install]
WantedBy=default.target
```

Enable it:
```bash
systemctl --user enable --now nvctl-vibrance.service
```

### Shell Integration

Add to `~/.zshrc` or `~/.bashrc`:

```bash
# Quick vibrance control
alias vibe='nvctl vibrance'
alias vibe-reset='nvctl vibrance 100'
alias vibe-max='nvctl vibrance 200'
alias vibe-gaming='nvctl vibrance 150'
```

## Compatibility

### Supported Drivers

- ✅ NVIDIA Proprietary (495+)
- ✅ NVIDIA Open (515+, **recommended 580+**)
- ✅ nvidia-dkms
- ❌ Nouveau (not supported - lacks vibrance API)

### Display Servers

- ✅ **Wayland** (KDE, GNOME, Hyprland, Sway, etc.)
- ✅ **X11** (traditional setups)
- ✅ **XWayland** (mixed environments)

### Display Connectors

- ✅ HDMI
- ✅ DisplayPort (including MST)
- ✅ DVI
- ✅ USB-C (with DP alt mode)

## Technical Details

### NVKMS API

nvcontrol uses the official NVIDIA Kernel Mode-Setting (NVKMS) API:

```rust
// From archive/open-gpu-kernel-modules/src/nvidia-modeset/interface/nvkms-api.h
enum NvKmsDpyAttribute {
    NV_KMS_DPY_ATTRIBUTE_DIGITAL_VIBRANCE = 10,
    // Range: -1024 to 1023
}
```

**ioctl Flow:**
1. Open `/dev/nvidia-modeset`
2. `NVKMS_IOCTL_ALLOC_DEVICE` - Allocate device handle
3. `NVKMS_IOCTL_QUERY_DISP` - Query connected displays
4. `NVKMS_IOCTL_SET_DPY_ATTRIBUTE` - Set digital vibrance
5. `NVKMS_IOCTL_FREE_DEVICE` - Clean up

### Implementation Files

| File | Purpose |
|------|---------|
| `src/nvkms_bindings.rs` | NVKMS API bindings (structs, enums, ioctls) |
| `src/vibrance_native.rs` | Pure Rust vibrance controller |
| `src/vibrance.rs` | Legacy fallback (nvidia-settings) |

### Reference Implementation

Our implementation is based on **[nvibrant](https://github.com/Tremeschin/nvibrant)** by @Tremeschin:
- nvibrant pioneered the NVKMS ioctl approach for Wayland
- nvcontrol extends this with full Rust implementation
- Added multi-display support, range validation, GUI integration

## Permissions Setup

### Option 1: Udev Rules (Recommended)

```bash
# Auto-setup (coming soon)
nvctl setup permissions

# Manual setup
sudo tee /etc/udev/rules.d/99-nvidia.rules <<EOF
# Allow access to nvidia-modeset for vibrance control
KERNEL=="nvidia-modeset", MODE="0666"
EOF

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### Option 2: User Group

```bash
# Add user to video group
sudo usermod -aG video $USER

# Re-login for changes to take effect
```

### Option 3: Sudo (Quick Test)

```bash
sudo nvctl vibe 150
```

## GUI Integration

### Slider Control

The nvcontrol GUI includes a vibrance slider:

```
┌─────────────────────────────────────┐
│ Digital Vibrance                    │
├─────────────────────────────────────┤
│ [━━━━━━━━━━━━━━━━━━━━━━━━━━━] 150% │
│  0%    50%   100%  150%   200%      │
└─────────────────────────────────────┘
```

- Real-time preview
- Per-display control
- Save as profile
- Auto-apply per-game

## Comparison with Alternatives

| Feature | nvctl | nvibrant | nvidia-settings | vibrantLinux |
|---------|-------|----------|-----------------|--------------|
| **Wayland Support** | ✅ | ✅ | ❌ (X11 only) | ⚠️ (hacky) |
| **Pure Rust** | ✅ | ❌ (C++) | ❌ (C) | ❌ (C++) |
| **GUI** | ✅ | ❌ | ✅ | ❌ |
| **Per-Display** | ✅ | ✅ | ✅ | ❌ |
| **CLI Simplicity** | `vibe 150` | `nvibrant 512` | Complex | Complex |
| **Auto-Apply** | ✅ | ⚠️ Manual | ⚠️ Manual | ⚠️ Manual |
| **GPU Controls** | ✅ Full suite | ❌ | ✅ Limited | ❌ |

## Troubleshooting

### "Failed to allocate NVKMS device"

**Cause:** Permission denied or driver version mismatch

**Solutions:**
1. Check driver version:
   ```bash
   cat /sys/module/nvidia/version
   # Should be 580+ for best compatibility
   ```

2. Verify modeset is enabled:
   ```bash
   cat /proc/cmdline | grep nvidia_drm.modeset
   # Should show: nvidia_drm.modeset=1
   ```

3. Add to kernel parameters if missing:
   ```bash
   # GRUB
   sudo nano /etc/default/grub
   # Add: nvidia_drm.modeset=1
   sudo update-grub

   # systemd-boot
   sudo nano /boot/loader/entries/arch.conf
   # Add to options: nvidia_drm.modeset=1
   ```

4. Use fallback (works without modeset):
   ```bash
   # Fallback automatically activates
   nvctl vibe 150
   ```

### "nvidia-settings failed"

**Cause:** nvidia-settings not installed

**Solution:**
```bash
# Arch
sudo pacman -S nvidia-settings

# Ubuntu/Debian
sudo apt install nvidia-settings

# Fedora
sudo dnf install nvidia-settings
```

### Vibrance resets on reboot

**Solution:** Use systemd service (see Auto-Start section above)

### Different vibrance on each monitor

**Expected behavior!** Use per-display control:
```bash
# Monitor 1: 150%, Monitor 2: 100%
nvctl display vibrance set-display 0 150
nvctl display vibrance set-display 1 100
```

## API Reference

### Rust API

```rust
use nvcontrol::vibrance_native;

// Set all displays to 150%
vibrance_native::set_vibrance_all_native(150)?;

// Set specific display
vibrance_native::set_display_vibrance_native(0, 0, 150)?;

// Get status
let status = vibrance_native::get_vibrance_status_native()?;

// List displays
let displays = vibrance_native::list_displays_native()?;

// Reset to default
vibrance_native::reset_vibrance_native()?;
```

### CLI API

```bash
# Set all displays
nvctl vibrance <0-200>

# Shorthand alias
nvctl vibe <0-200>

# Advanced control
nvctl display vibrance set-display <id> <0-200>
nvctl display vibrance get
nvctl display vibrance reset
nvctl display vibrance info
nvctl display vibrance list
```

## Performance

| Operation | NVKMS ioctl | nvidia-settings |
|-----------|-------------|-----------------|
| Set vibrance | < 1ms | ~50ms |
| Get vibrance | < 1ms | ~50ms |
| List displays | < 1ms | ~100ms |
| **Startup overhead** | None | ~200ms |

**Recommendation:** NVKMS ioctls for real-time control (gaming, color grading), fallback is fine for one-time setup.

## Known Issues

1. **NVKMS requires permissions** - Working as designed. Use udev rules or fallback.
2. **Vibrance resets after suspend** - Add resume hook:
   ```bash
   sudo tee /lib/systemd/system-sleep/nvctl-vibrance <<'EOF'
   #!/bin/bash
   if [ "$1" = "post" ]; then
       su - $USER -c "nvctl vibe 150"
   fi
   EOF
   sudo chmod +x /lib/systemd/system-sleep/nvctl-vibrance
   ```

## Future Enhancements

- [ ] Per-application auto-apply (game detection)
- [ ] Smooth vibrance transitions
- [ ] HDR + vibrance interaction
- [ ] Save/load vibrance profiles
- [ ] Wayland protocol extension (compositor-level)
- [ ] AMD/Intel GPU support (via different APIs)

## Credits

- **nvibrant** by [@Tremeschin](https://github.com/Tremeschin) - Pioneer of NVKMS vibrance control
- **NVIDIA open-gpu-kernel-modules** - Official NVKMS API headers
- **vibrantLinux** - Early Wayland vibrance work

## License

MIT License - See LICENSE file

---

**For more GPU controls, see:**
- [VRR/G-SYNC](./vrr-gsync.md)
- [HDR Control](./hdr.md)
- [Image Sharpening](./image-sharpening.md)
- [Overclocking](./overclocking.md)
