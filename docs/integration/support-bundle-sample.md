# Redacted Support Bundle Sample

Example of a redacted support bundle shape for issue reporting:

```text
nvcontrol support bundle
=======================

[release diagnostics]
running_kernel=<kernel-version>
module_kernel=<kernel-version>
kernel_match=true
userspace_driver_version=<driver-version>
kernel_module_version=<driver-version>
release_alignment=structurally aligned at <driver-version>, firmware file present but filename does not encode release

[gpus]
NVIDIA GeForce RTX 5090 | pci=<redacted-pci> | device=<redacted-device> | chip=gb2xx | arch=Blackwell | open_capable=true

[firmware paths]
<redacted-path>

[ownership]
<redacted-path> | owner=/usr/lib/firmware/nvidia/gb202/gsp/ is owned by linux-firmware-nvidia <firmware-package-version> | verify=n/a

[arch packages]
nvidia | missing | n/a
nvidia-utils | installed | <driver-version>-1
linux-firmware | installed | <firmware-package-version>
```

Each generated support bundle also writes a JSON sidecar metadata file containing the machine-readable release diagnostics summary.
