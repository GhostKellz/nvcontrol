# Fedora

The current support update was tested on Fedora 44 with an RTX 3070, first on
the retained open 610 driver and then on open 615 after a full system update.
Both runs passed the full test suite and native vibrance apply/readback/exact
restoration. The native RPM built and installed successfully; its CLI and TUI
passed live acceptance. GUI visual acceptance is tracked separately in the
[release evidence](../advisories/v0.8.13-release-notes.md).

## Driver packaging

Use one coherent driver source and let DNF update kernel and userspace components
together. NVIDIA's [Fedora driver guide](https://docs.nvidia.com/datacenter/tesla/driver-installation-guide/fedora.html)
documents the open-module package path. RPM Fusion users should follow its NVIDIA
Howto and verify the final transaction before accepting it, especially when
multilib gaming libraries are involved.

After an update, wait for the module build to finish (akmods with RPM Fusion,
DKMS with NVIDIA's packages) before rebooting. Verify the loaded and packaged
versions agree:

```bash
nvctl driver info
modinfo -F version nvidia
cat /proc/driver/nvidia/version
```

Do not assume `akmod-nvidia` versus `akmod-nvidia-open` from the package name
alone; verify `nvctl driver info` reports `Open Kernel (Dual MIT/GPL)` when open
modules are intended.

## Package validation

The Fedora spec requires Rust 1.98+, builds both binaries, and runs library tests
without masking failures. Its runtime dependencies accept either NVIDIA or RPM
Fusion library packages and require NVML explicitly. The package uses the shared
NVIDIA icon and matching desktop/application identity. Optional gaming
recommendations are Gamescope,
MangoHud, and GameMode.
