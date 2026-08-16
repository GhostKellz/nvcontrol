# Fedora

v0.8.12 was validated on Fedora 44 with an RTX 3070 passed through to the guest
and the open 610.57.04 kernel modules. The test covered compilation, the Rust test
suite, NVML/Vulkan diagnostics, native vibrance readback and no-change apply,
and the official GeForce NOW Flatpak.

## Driver packaging

Use one coherent driver source and let DNF update kernel and userspace components
together. NVIDIA's [Fedora driver guide](https://docs.nvidia.com/datacenter/tesla/driver-installation-guide/fedora.html)
documents the open-module package path. RPM Fusion users should follow its NVIDIA
Howto and verify the final transaction before accepting it, especially when
multilib gaming libraries are involved.

After an update, wait for akmods to finish before rebooting and verify the loaded
and packaged versions agree:

```bash
nvctl driver info
modinfo -F version nvidia
cat /proc/driver/nvidia/version
```

Do not assume `akmod-nvidia` versus `akmod-nvidia-open` from the package name
alone; verify `nvctl driver info` reports `Open Kernel (Dual MIT/GPL)` when open
modules are intended.

## Package validation

The Fedora spec requires Rust 1.97+, builds both binaries, and runs library tests
without masking failures. Optional gaming recommendations are Gamescope,
MangoHud, and GameMode.
