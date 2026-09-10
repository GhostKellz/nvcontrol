# NVIDIA Driver Compatibility

Canonical nvcontrol version guidance by NVIDIA driver branch.

## Recommended Matrix

| NVIDIA Driver Branch | Recommended nvcontrol Version | Notes |
|----------------------|-------------------------------|-------|
| 615 open driver | v0.8.13 or newer | Allocation and command ABI support; live vibrance validated |
| 610 open driver | current releases containing the multi-ABI fix | Retained allocation ABI and regression target |
| 600 | current releases containing the multi-ABI fix | Retained 595-family fallback; no local upstream 600 tag to measure |
| 595 | current `main` branch and releases containing the multi-ABI fix | Runtime-selected 595 allocation layout |
| 590 and earlier | older vibrance-compatible build, typically `v0.8.5` | Use when digital vibrance must work on older drivers |

## Current Baseline

If you are running the NVIDIA open 615 driver branch, use the current nvcontrol codebase:

```bash
git clone https://github.com/GhostKellz/nvcontrol
cd nvcontrol
git checkout main
cargo build --release
```

This is the development path. Installers follow published releases; an unpublished support update must be built from its checkout.

## Transitional 595 Branch

Driver 595 uses a larger `AllocDevice` parameter layout than 610. Current
nvcontrol selects that known layout at runtime while preserving the 610 layout
as the first choice on 610 through 614. The newer allocation layout is tried
first on 615 and newer; retries are restricted to verified sizes after `EPERM`.

```bash
git clone https://github.com/GhostKellz/nvcontrol
cd nvcontrol
cargo build --release --bin nvctl --no-default-features
```

## 590 And Earlier

For 590 and earlier, use the older vibrance-compatible path with the older NVKMS layout.

- `v0.8.5` is the documented fallback build
- This is primarily about keeping digital vibrance working on the older NVKMS layout

```bash
git clone https://github.com/GhostKellz/nvcontrol
cd nvcontrol
git checkout v0.8.5
cargo build --release --bin nvctl --no-default-features
```

## How To Check Your Driver Version

```bash
nvidia-smi --query-gpu=driver_version --format=csv,noheader
cat /sys/module/nvidia/version
nvctl driver info
```

## Related Docs

- [legacy.md](legacy.md)
- [open-615.md](open-615.md)
- [open-610.md](open-610.md)
- [nvkms-abi-changes.md](nvkms-abi-changes.md)
