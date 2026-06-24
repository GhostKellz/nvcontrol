# Architecture

nvcontrol is a Rust desktop/CLI tool around NVIDIA driver state, display controls,
monitoring, gaming workflows, configuration profiles, and support diagnostics. The
project favors direct local inspection with explicit user action for anything that
changes hardware, services, containers, or persistent configuration.

## Command Architecture

```mermaid
flowchart TD
    cli["nvctl CLI"] --> gpu["gpu\ninfo, list, stat"]
    cli --> display["display\nlayout, vibrance, HDR, VRR"]
    cli --> driver["driver\ninfo, validate, diagnose, support"]
    cli --> setup["setup\nfirst-run readiness"]
    cli --> config["config\ncapture, preview, diff, apply"]
    cli --> gaming["gaming\nauto profiles, launch hooks, latency"]
    cli --> container["container\nruntime doctor, smoke tests"]
    cli --> cuda["cuda / ai\nCUDA, Ollama, workload diagnostics"]
    cli --> tui["tui / nvtop\ninteractive monitoring"]

    gpu --> nvml["NVML backend"]
    driver --> proc["/proc + kernel module facts"]
    driver --> gsp["GSP firmware checks"]
    display --> nvkms["NVKMS / display backend"]
    config --> profiles["profile bundles"]
    gaming --> services["user service + launcher hooks"]
    container --> runtimes["Docker / Podman / containerd"]
    cuda --> local_tools["nvidia-smi / nvcc / ollama / docker / nvidia-ctk"]
    tui --> nvml
    tui --> driver
    tui --> cuda
```

## Read/Write Boundary

```mermaid
flowchart LR
    subgraph ReadOnly["Read-only paths"]
        doctor["doctor / support diagnostics"]
        setup["setup check"]
        cuda["cuda doctor / ai workloads"]
        tui_status["TUI status tabs"]
        preview["config preview / diff"]
    end

    subgraph Mutating["Explicit mutating paths"]
        vibrance["vibrance set/reset"]
        oc["overclock apply/reset"]
        fan["fan set/curve apply"]
        power["power limit/profile"]
        service["gaming auto service install/start/stop"]
        profile_apply["config apply"]
    end

    ReadOnly --> guidance["issues, fixes, commands"]
    guidance --> user["user decides"]
    user --> Mutating
```

The release tests should default to the read-only side. Live hardware regressions
must remain explicit and opt-in.

## CUDA/AI Diagnostics Path

```mermaid
flowchart TD
    command["nvctl cuda doctor\nnvctl ai doctor"] --> collect["collect_cuda_doctor"]

    collect --> cuda_info["get_cuda_info"]
    collect --> tools["collect_cuda_tools"]
    collect --> ollama["collect_ollama_cuda_status"]
    collect --> workloads["recommend_ai_workloads"]

    cuda_info --> smi_driver["nvidia-smi\n--query-gpu=driver_version"]
    cuda_info --> smi_gpu["nvidia-smi\nindex,name,memory"]
    cuda_info --> nvcc["nvcc --version"]
    cuda_info --> toolkit_path["toolkit path search"]

    tools --> path_lookup["PATH lookup"]
    tools --> versions["tool --version"]

    ollama --> cli["ollama --version"]
    ollama --> tcp["TCP connect\n127.0.0.1:11434"]
    ollama --> docker["docker --version"]
    ollama --> ctk["nvidia-ctk --version"]
    ollama --> gpu_memory["sum detected VRAM"]

    workloads --> thresholds["VRAM thresholds\n8, 12, 14, 16 GiB"]

    smi_driver --> report["CudaDoctorReport"]
    smi_gpu --> report
    nvcc --> report
    toolkit_path --> report
    path_lookup --> report
    versions --> report
    cli --> report
    tcp --> report
    docker --> report
    ctk --> report
    gpu_memory --> report
    thresholds --> report

    report --> human["human output"]
    report --> json["JSON"]
    report --> yaml["YAML"]
    report --> tui["TUI CUDA/AI tab\n60s cache"]
```

## TUI Data Flow

```mermaid
flowchart TD
    event_loop["TUI event loop\n100ms ticks"] --> input["keyboard/mouse input"]
    event_loop --> refresh["refresh when not paused"]
    input --> state["TuiApp state"]
    refresh --> init["lazy backend init"]
    init --> nvml["NVML metrics"]
    refresh --> processes["GPU process list\nrate-limited"]
    refresh --> dlss["DLSS cache\n60s"]
    refresh --> cuda["CUDA/AI cache\n60s"]
    refresh --> asus["ASUS power cache\n2s"]

    nvml --> state
    processes --> state
    dlss --> state
    cuda --> state
    asus --> state
    state --> render["ratatui render"]
```

## Support Bundle Flow

```mermaid
flowchart LR
    user["user"] --> doctor["nvctl doctor --support"]
    user --> driver_bundle["nvctl driver support-bundle"]
    tui["TUI Drivers tab"] --> driver_bundle

    doctor --> diagnostics["release diagnostics"]
    driver_bundle --> diagnostics
    diagnostics --> redact["optional path/id redaction"]
    redact --> text["text report"]
    redact --> metadata["JSON metadata"]
    redact --> tarball["tar.gz bundle"]
```

Support artifacts should capture enough runtime context to debug driver, GSP, DKMS,
container, setup, and CUDA/AI reports without forcing repeated back-and-forth.

