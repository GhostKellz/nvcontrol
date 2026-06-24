# nvcontrol Integrations

This directory documents nvcontrol's integrations with other tools in the CKTechX ecosystem.

## Experimental Integrations

The following integration remains in `experimental/`:

| Integration | Status | Description |
|-------------|--------|-------------|
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

```mermaid
flowchart TD
    subgraph Integrations["Integration surfaces"]
        Docker["Docker"]
        Podman["Podman"]
        Containerd["containerd"]
        Nix["NixOS containers"]
    end

    subgraph Nvcontrol["nvcontrol"]
        CLI["nvctl container commands"]
        RuntimeDoctor["container runtime doctor"]
        GpuApi["GPU and driver APIs"]
        Support["support bundle metadata"]
    end

    subgraph System["Local NVIDIA stack"]
        Runtime["NVIDIA container runtime\nnvidia-ctk / CDI"]
        Driver["NVIDIA driver\nNVML / NVKMS"]
        Devices["/dev/nvidia* devices"]
    end

    Docker --> CLI
    Podman --> CLI
    Containerd --> CLI
    Nix --> CLI
    CLI --> RuntimeDoctor
    RuntimeDoctor --> Runtime
    RuntimeDoctor --> Support
    GpuApi --> Driver
    Runtime --> Devices
    Driver --> Devices
```

## See Also

- [Backend Architecture](../config/backend-architecture.md) - Internal backend design
- [API Reference](../api/reference.md) - nvcontrol Rust API
