# Distribution Support

nvcontrol targets a common NVIDIA/Wayland runtime rather than hiding distro
differences. Arch is the primary development platform; Fedora and Pop!_OS are
hardware-backed compatibility targets. CachyOS follows the Arch userspace model
but has its own kernel and NVIDIA module tooling.

| Distribution | v0.8.12 validation | Driver coverage | Notes |
|---|---|---|---|
| Arch Linux | RTX 5090, KDE Wayland | open 610.57.04 | Primary development and full Astral validation |
| Fedora 44 | RTX 3070 passthrough | open 610.57.04 | Build, tests, NVML, Vulkan, GFN, and vibrance |
| Pop!_OS 24.04 COSMIC | RTX 3070 passthrough | open 595.84 | 595 NVKMS ABI and COSMIC VRR coverage |
| CachyOS | Not yet a dedicated test host | 610+ expected | Arch-compatible, but kernel/module packaging differs |

The two passthrough guests share one GPU and are not simultaneous test targets.
Guest-visible connectors and EDIDs do not describe the physical Arch workstation
monitors.

Start with [Arch](arch.md), [Fedora](fedora.md), [Pop!_OS COSMIC](popos-cosmic.md),
or [CachyOS](cachyos.md). The driver branch matrix remains in
[NVIDIA driver compatibility](../drivers/nvidia-driver.md).
