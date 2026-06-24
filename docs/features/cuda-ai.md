# CUDA, Ollama, And Local AI/ML

The CUDA/AI feature area is for Linux desktop systems where the same NVIDIA GPU is
used for display, gaming, CUDA development, local inference, image generation, and
containers. The initial `v0.8.9` scope is intentionally diagnostic: expose what is
installed, what the driver can see, and what is likely to work before the user starts
changing services or workloads.

## Scope

| Included | Not Included |
|----------|--------------|
| CUDA runtime/toolkit discovery | CUDA installation |
| Ollama CLI and local service detection | Starting/stopping Ollama |
| Docker and NVIDIA Container Toolkit detection | Running containers |
| VRAM-based workload recommendations | Benchmarking or model execution |
| JSON/YAML output for scripting | Automatic tuning of model parameters |
| TUI read-only CUDA/AI dashboard tab | Hardware-mutating tests |

## Feature Map

```mermaid
flowchart TD
    subgraph Host["Linux host"]
        driver["NVIDIA driver\n610+ open preferred"]
        devices["/dev/nvidia*\nNVML visibility"]
        toolkit["CUDA toolkit\nnvcc optional"]
        docker["Docker runtime"]
        ctk["nvidia-ctk"]
        ollama_cli["ollama CLI"]
        ollama_svc["Ollama service\n127.0.0.1:11434"]
    end

    subgraph Nvctl["nvcontrol"]
        cli["nvctl cuda / nvctl ai"]
        tui["TUI CUDA/AI tab"]
        report["CudaDoctorReport"]
        recommendations["AI workload fit"]
    end

    driver --> devices
    devices --> cli
    toolkit --> cli
    docker --> cli
    ctk --> cli
    ollama_cli --> cli
    ollama_svc --> cli

    cli --> report
    report --> recommendations
    report --> tui
    report --> json["JSON/YAML support output"]
    report --> human["human guidance"]
```

## Ollama Paths

There are two common Ollama CUDA paths:

```mermaid
flowchart LR
    subgraph Native["Native Ollama"]
        ncli["ollama CLI"] --> nsvc["ollama serve\nlocalhost:11434"]
        nsvc --> ngpu["CUDA runtime\nvisible NVIDIA GPU"]
    end

    subgraph Container["Container Ollama"]
        docker["docker"] --> gpu_runtime["--gpus=all"]
        ctk["nvidia-ctk"] --> gpu_runtime
        gpu_runtime --> image["ollama/ollama"]
        image --> cgpu["CUDA runtime inside container"]
    end

    nvctl["nvctl cuda ollama"] --> ncli
    nvctl --> nsvc
    nvctl --> docker
    nvctl --> ctk
    nvctl --> advice["print commands + issues\nwithout executing them"]
```

`nvctl cuda ollama` reports both paths because many systems have native Ollama
installed while still wanting a reproducible GPU-container smoke test.

## Diagnostic Sequence

```mermaid
sequenceDiagram
    participant User
    participant Nvctl as nvctl cuda doctor
    participant SMI as nvidia-smi
    participant NVCC as nvcc
    participant PATH as PATH tools
    participant Ollama as 127.0.0.1:11434

    User->>Nvctl: run read-only doctor
    Nvctl->>SMI: query driver, GPUs, VRAM
    SMI-->>Nvctl: driver/GPU facts or unavailable
    Nvctl->>NVCC: query CUDA toolkit version
    NVCC-->>Nvctl: toolkit version or unavailable
    Nvctl->>PATH: locate ollama, docker, nvidia-ctk, ncu, nsys
    PATH-->>Nvctl: tool paths and version lines
    Nvctl->>Ollama: TCP connect timeout check
    Ollama-->>Nvctl: reachable or not reachable
    Nvctl-->>User: report, issues, fixes, workload fit
```

## TUI Integration

The dashboard includes a `CUDA/AI` tab. It uses the same read-only doctor path as the
CLI and caches results for 60 seconds so the TUI does not shell out every frame.

The tab shows:

- CUDA driver and toolkit state
- GPU count and VRAM
- Ollama CLI and service state
- Docker plus NVIDIA Container Toolkit readiness
- workload-fit summary
- top issues from the doctor report

## Workload Guidance

`nvctl ai workloads` classifies common local workloads from detected VRAM:

| Workload | Good Fit Signal | Notes |
|----------|-----------------|-------|
| Ollama 7B/8B quantized LLMs | 8 GiB+ VRAM | Q4/Q5 models are the practical first smoke test. |
| Ollama 13B/14B quantized LLMs | 14-16 GiB+ VRAM | Context size can push memory above model size. |
| Stable Diffusion / image generation | 12 GiB+ VRAM | SDXL workflows benefit from more VRAM headroom. |
| PyTorch/TensorFlow training | 16 GiB+ VRAM | Containers are preferred for reproducible framework stacks. |

## Safety Boundary

CUDA/AI diagnostics stay on the read-only side of the nvcontrol safety boundary:

```mermaid
flowchart TD
    facts["collect facts"] --> classify["classify readiness"]
    classify --> report["print report"]
    report --> user_action["user-reviewed command"]

    user_action -. optional/manual .-> service["start service"]
    user_action -. optional/manual .-> container["run container"]
    user_action -. optional/manual .-> install["install packages"]

    facts -. never directly .-> service
    facts -. never directly .-> container
    facts -. never directly .-> install
```

This keeps normal CLI checks, TUI refreshes, and tests from mutating hardware or
service state.

