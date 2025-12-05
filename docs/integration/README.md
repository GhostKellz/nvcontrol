# nvcontrol Integrations

This directory documents nvcontrol's integrations with other tools in the CKTechX ecosystem.

## Available Integrations

| Integration | Status | Description |
|-------------|--------|-------------|
| [nvbind](./NVBIND.md) | ✅ Active | GPU container runtime with sub-microsecond latency |
| [Bolt](./BOLT.md) | ✅ Active | Gaming-optimized container runtime |
| [ghostwave](./GHOSTWAVE.md) | ✅ Active | GPU-accelerated audio denoising |
| [nvhud](./NVHUD.md) | 🔄 Planned | Zig-based GPU monitoring overlay |

## Quick Start

### nvbind + nvcontrol (Gaming Containers)

```bash
# Apply nvcontrol gaming profile
nvctl profile apply gaming

# Launch container with nvbind
nvbind run --runtime bolt --gpu all steam:latest

# Monitor combined performance
nvctl gpu stat --container-aware
```

### ghostwave + nvcontrol (Audio Production)

```bash
# Apply quiet profile for audio work
nvctl profile apply audio-production

# Launch ghostwave with optimized GPU
ghostwave start
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  ┌─────────┐  ┌─────────┐  ┌───────────┐  ┌─────────────┐  │
│  │ nvbind  │  │  Bolt   │  │ ghostwave │  │   nvhud     │  │
│  └────┬────┘  └────┬────┘  └─────┬─────┘  └──────┬──────┘  │
│       │            │             │               │          │
│       └────────────┴──────┬──────┴───────────────┘          │
│                           │                                  │
│                           ▼                                  │
│                    ┌─────────────┐                          │
│                    │  nvcontrol  │                          │
│                    │  (GPU API)  │                          │
│                    └──────┬──────┘                          │
│                           │                                  │
└───────────────────────────┼─────────────────────────────────┘
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
