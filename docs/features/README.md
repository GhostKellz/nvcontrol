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

## Quick Start

```bash
# Digital vibrance (color boost)
nvctl vibe 150

# Check HDR status
nvctl display hdr status

# View overclock limits
nvctl overclock info
```

## Feature Compatibility

| Feature | Wayland | X11 | Driver Req |
|---------|---------|-----|------------|
| Digital Vibrance | Yes | Yes | 495+ |
| HDR | Yes | Limited | 535+ |
| VRR/G-SYNC | Yes | Yes | 470+ |
| Overclocking | Yes | Yes | 535+ |
