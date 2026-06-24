# Feature Documentation

Guides for nvcontrol display and GPU features.

## Contents

| Document | Description |
|----------|-------------|
| [vibrance.md](vibrance.md) | Digital vibrance control (color saturation) |
| [hdr.md](hdr.md) | High Dynamic Range display setup |
| [vrr-gsync.md](vrr-gsync.md) | Variable Refresh Rate / G-SYNC / FreeSync |
| [image-sharpening.md](image-sharpening.md) | GPU post-processing sharpening |
| [overclocking.md](overclocking.md) | GPU/memory clock tuning |
| [cuda-ai.md](cuda-ai.md) | CUDA, Ollama, and local AI/ML diagnostics |

## Quick Start

```bash
# Digital vibrance (color boost)
nvctl vibrance 150

# Check HDR status
nvctl display hdr status

# View overclock limits
nvctl overclock info

# Check CUDA/Ollama AI readiness
nvctl cuda doctor
nvctl ai workloads
```

## Feature Compatibility

| Feature | Wayland | X11 | Driver Req |
|---------|---------|-----|------------|
| Digital Vibrance | Yes | Yes | 610+ recommended for current NVKMS path |
| HDR | Compositor-dependent | Limited | 610+ recommended |
| VRR/G-SYNC | Compositor-dependent | Yes | Current NVIDIA driver recommended |
| Overclocking | Backend/permission-dependent | Backend/permission-dependent | Current NVIDIA driver recommended |
| CUDA/AI diagnostics | Yes | Yes | 535+ runtime, 610+ preferred |
