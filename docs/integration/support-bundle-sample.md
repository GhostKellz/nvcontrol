# Redacted Support Bundle Sample

Example of a redacted support bundle shape for issue reporting:

```text
nvcontrol support bundle
=======================

[release diagnostics]
running_kernel=6.19.12-1-cachyos-lto
module_kernel=6.19.12-1-cachyos-lto
kernel_match=true
userspace_driver_version=595.58.03
kernel_module_version=595.58.03
release_alignment=structurally aligned at 595.58.03, firmware file present but filename does not encode release

[gpus]
NVIDIA GeForce RTX 5090 | pci=<redacted-pci> | device=<redacted-device> | chip=gb2xx | arch=Blackwell | open_capable=true

[firmware paths]
<redacted-path>

[ownership]
<redacted-path> | owner=/usr/lib/firmware/nvidia/gb202/gsp/ is owned by linux-firmware-nvidia 20260410-1 | verify=n/a

[arch packages]
nvidia | missing | n/a
nvidia-utils | installed | 595.58.03-1
linux-firmware | installed | 20260410-1
```

Each generated support bundle also writes a JSON sidecar metadata file containing the machine-readable release diagnostics summary.
