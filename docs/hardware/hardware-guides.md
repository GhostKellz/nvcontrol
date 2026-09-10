# Hardware-Specific Documentation

Setup guides and optimizations for specific GPU models.

## Contents

| Document | Description |
|----------|-------------|
| [rtx-5090-setup.md](rtx-5090-setup.md) | RTX 5090 (Blackwell) setup guide |
| [asus-astral.md](asus-astral.md) | ASUS ROG Astral/Matrix features |
| [astral-owners.md](astral-owners.md) | ASUS Astral-specific tips |
| [power-detection.md](power-detection.md) | ASUS power connector detection |

## Supported Architectures

| Architecture | GPUs | Status |
|--------------|------|--------|
| Blackwell | RTX 5060-5090 | RTX 5090 validated on open 615; retained 610 coverage |
| Ada Lovelace | RTX 4060-4090 | Expected 610+ path; repeat live smoke coverage still wanted |
| Ampere | RTX 3060-3090 Ti | RTX 3070 validated on Fedora/Pop!_OS with open 615; retained 595/610 coverage |
| Turing | RTX 2060-2080 Ti | Supported where the loaded driver exposes required NVML/display paths |
| Pascal | GTX 1060-1080 Ti | Basic/legacy support; check the driver compatibility matrix |

## Quick Links

- **RTX 50-series user?** Start with [rtx-5090-setup.md](rtx-5090-setup.md)
- **ASUS ROG Astral/Matrix?** See [asus-astral.md](asus-astral.md)

For nvcontrol-to-driver version guidance, see [`../drivers/nvidia-driver.md`](../drivers/nvidia-driver.md).
