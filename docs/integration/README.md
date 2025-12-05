# nvcontrol Integrations

This directory documents nvcontrol's integrations with other tools in the CKTechX ecosystem.

## Available Integrations

| Integration | Status | Description |
|-------------|--------|-------------|
| [nvhud](./NVHUD.md) | 🔄 Planned | Zig-based GPU monitoring overlay |

## Experimental Integrations

The following integrations have been moved to `experimental/` for future re-integration:

| Integration | Status | Description |
|-------------|--------|-------------|
| nvbind | 🧪 Experimental | GPU container runtime with sub-microsecond latency |
| Bolt | 🧪 Experimental | Gaming-optimized container runtime |
| ghostwave | 🧪 Experimental | GPU-accelerated audio denoising |

See [experimental/README.md](../../experimental/README.md) for details on these features.

## Container Runtime

nvcontrol provides native container runtime support for:
- **Docker** with nvidia-container-toolkit
- **Podman** with GPU support
- **containerd** with NVIDIA runtime
- **NixOS** container integration

### Quick Start

```bash
# List GPU containers
nvctl container list

# Launch container with GPU support
nvctl container launch -i nvidia/cuda:latest --gpu all

# Monitor container GPU usage
nvctl container monitor -c my-container
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  ┌─────────────┐  ┌─────────────────┐  ┌─────────────┐     │
│  │   Docker    │  │     Podman      │  │   nvhud     │     │
│  └──────┬──────┘  └────────┬────────┘  └──────┬──────┘     │
│         │                  │                   │            │
│         └──────────────────┴───────────────────┘            │
│                            │                                │
│                            ▼                                │
│                     ┌─────────────┐                         │
│                     │  nvcontrol  │                         │
│                     │  (GPU API)  │                         │
│                     └──────┬──────┘                         │
│                            │                                │
└────────────────────────────┼────────────────────────────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │  NVIDIA Driver  │
                    │  (NVML / NVKMS) │
                    └─────────────────┘
```

## See Also

- [Backend Architecture](../config/BACKEND_ARCHITECTURE.md) - Internal backend design
- [API Reference](../api/README.md) - nvcontrol Rust API
