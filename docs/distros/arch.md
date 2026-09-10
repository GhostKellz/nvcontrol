# Arch Linux

Arch Linux is nvcontrol's primary development and release-validation platform.
v0.8.12 was validated on KDE Wayland with an RTX 5090, NVIDIA open 610.57.04,
and a CachyOS LTO kernel. That is an Arch installation using a third-party kernel,
not a CachyOS distro installation.

The current support update also passed native vibrance apply/readback on the
same RTX 5090 with NVIDIA open 615.71.09 using Rust 1.98.1. The regression restored both displays to their original raw values; the user
subsequently requested 200% on both. The final native pacman package was installed
and passed file-integrity and live diagnostic checks. See the
[release evidence](../advisories/v0.8.13-release-notes.md) for the complete status.

## Driver and kernel

For Blackwell, follow the [Arch NVIDIA guide](https://wiki.archlinux.org/title/NVIDIA).
Arch recommends the open kernel modules for Blackwell. Use the package matching
the kernel, or `nvidia-open-dkms` with the headers for every installed custom
kernel. Keep `nvidia-utils` and the loaded kernel module on the same version.

```bash
nvctl driver info
nvctl setup check
cat /sys/module/nvidia_drm/parameters/modeset
```

DRM KMS is enabled by default by current Arch NVIDIA packages, but nvcontrol
reports the live sysfs state rather than assuming it. A custom kernel does not
make the operating system CachyOS; record the distro, kernel package, module
version, and NVIDIA userspace version separately in bug reports.

## Install from source

```bash
cargo build --release --bin nvctl
cargo build --release --bin nvcontrol --features gui
dev/install.sh
```

The development installer targets `~/.local/bin`, installs desktop integration
and Bash/Zsh/Fish completions, and preserves user configuration during the
default uninstall path.

## Gaming helpers

Gamescope, MangoHud, GameMode, Vulkan tools, and the official GeForce NOW Flatpak
are optional. Install only the tools used by your workflow; nvcontrol diagnoses
their absence without treating every optional component as an error.
