# Driver Documentation

NVIDIA driver compatibility, installation, and optimization guides.

## Contents

| Document | Description |
|----------|-------------|
| [legacy.md](legacy.md) | Support for drivers 590 and earlier |
| [595-abi-changes.md](595-abi-changes.md) | Driver 595 NVKMS API changes |
| [gsp.md](gsp.md) | GPU System Processor (GSP) firmware |
| [diagnose-release.md](diagnose-release.md) | Interpreting release diagnostics |
| [dkms.md](dkms.md) | Dynamic Kernel Module Support setup |
| [open-590.md](open-590.md) | NVIDIA Open driver 590 features |
| [kernel-580.md](kernel-580.md) | Kernel driver 580+ optimizations |

## Quick Links

- **Using driver 590 or earlier?** See [legacy.md](legacy.md)
- **Vibrance issues on 595+?** See [595-abi-changes.md](595-abi-changes.md)
- **DKMS build failures?** See [dkms.md](dkms.md)

## Driver Version Support Matrix

| Driver Version | nvcontrol Version | Notes |
|----------------|-------------------|-------|
| 595+ | v0.8.6+ (latest) | Full support |
| 560-590 | v0.8.5 | Use legacy build |
| < 560 | v0.8.5 | Untested |

See [legacy.md](legacy.md) for building older versions.
