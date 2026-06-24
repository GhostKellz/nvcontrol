# Runtime Flows

This page documents the runtime flows that matter when reading or debugging
nvcontrol. It is intentionally operational: each diagram maps a user-facing
command or screen to the local system APIs it touches.

## Driver And Display Control Plane

```mermaid
flowchart TD
    user["User command or GUI action"] --> parser["clap command parser"]
    parser --> route{"Which control plane?"}

    route -->|"GPU facts"| nvml["NVML backend"]
    route -->|"Driver/GSP facts"| driver["driver diagnostics"]
    route -->|"Display/vibrance"| display["display backend"]
    route -->|"Profiles"| profiles["profile manager"]
    route -->|"CUDA/AI"| cuda["CUDA doctor"]

    nvml --> smi["nvidia-smi / NVML"]
    driver --> proc["/proc/driver/nvidia/version"]
    driver --> modinfo["modinfo nvidia"]
    driver --> firmware["/lib/firmware/nvidia/*"]
    display --> nvkms["/dev/nvidiactl + NVKMS ioctl"]
    display --> compositor["kscreen-doctor / hyprctl / gsettings"]
    profiles --> xdg["~/.config/nvcontrol"]
    cuda --> tools["nvidia-smi, nvcc, ollama, docker, nvidia-ctk"]

    smi --> report["human / JSON / YAML output"]
    proc --> report
    modinfo --> report
    firmware --> report
    nvkms --> report
    compositor --> report
    xdg --> report
    tools --> report
```

## 610+ Runtime Capability Probe

The 610+ path combines driver-version gates with runtime probes. Runtime probes
must be treated as optional evidence because helper tools can be missing or broken
on otherwise healthy systems.

```mermaid
flowchart TD
    info["nvctl driver info"] --> version["DriverCapabilities::from_version"]
    version --> gate{"driver >= 610?"}
    gate -->|no| legacy["skip 610+ runtime section"]
    gate -->|yes| probes["runtime probes"]

    probes --> vulkan["vulkaninfo --summary\nwith overlay-safe environment"]
    probes --> egl["eglinfo / eglinfo -B"]
    probes --> kernel["uname -r\nkernel >= 6.19"]

    vulkan --> vkext["notable Vulkan extensions"]
    egl --> fp16["FP16 EGL Wayland signal"]
    kernel --> drm["DRM color pipeline kernel readiness"]

    vkext --> output["610+ Features output"]
    fp16 --> output
    drm --> output
    version --> output
```

## Overlay-Safe Vulkan Probe

`vulkaninfo` is a helper process, not trusted core logic. It can load implicit
Vulkan layers such as MangoHud and vkBasalt. nvcontrol runs the probe with those
overlays disabled so diagnostics do not crash inside an overlay layer.

```mermaid
sequenceDiagram
    participant Nvctl as nvctl driver info
    participant Cmd as overlay_safe_vulkaninfo_command()
    participant Loader as Vulkan loader
    participant ICD as NVIDIA ICD

    Nvctl->>Cmd: build child command
    Cmd->>Cmd: set DISABLE_MANGOHUD=1
    Cmd->>Cmd: set DISABLE_VKBASALT=1
    Cmd->>Cmd: clear VK_INSTANCE_LAYERS, VK_LAYER_PATH, LD_PRELOAD
    Cmd->>Loader: run vulkaninfo --summary
    Loader->>ICD: enumerate instance/device capabilities
    ICD-->>Loader: extensions and device facts
    Loader-->>Nvctl: stdout or non-zero exit
    Nvctl-->>Nvctl: parse only known extension names
```

## Profile Bundle Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Captured: config capture --name
    [*] --> Imported: config import --input
    Captured --> Previewed: config preview
    Imported --> Previewed: config preview
    Previewed --> Compared: config diff
    Compared --> Applied: config apply
    Previewed --> Applied: config apply
    Applied --> Verified: driver/gpu/display inspection
    Applied --> RolledBack: apply previous capture
    RolledBack --> Verified
```

Profile bundles should keep a before/after path. For risky settings, capture the
current system first, preview the target, then apply only after the diff is clear.

## Gaming Auto Profile Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Disabled
    Disabled --> Configured: gaming auto config
    Configured --> ServiceInstalled: install-service
    Configured --> Running: start
    ServiceInstalled --> Running: enable-service
    Running --> Detected: matching process found
    Detected --> Delay: apply-delay window
    Delay --> Applied: profile applied
    Applied --> Watching: process still alive
    Watching --> Restored: game exits + restore-on-exit
    Restored --> Running
    Running --> Stopped: stop / disable-service
    Stopped --> Configured
```

This lifecycle is intentionally conservative. The apply delay exists to avoid
changing clocks, fans, or power at the same moment a game is still initializing.

## Support Artifact Pipeline

```mermaid
flowchart LR
    entry{"Entry point"} -->|"nvctl doctor --support"| doctor["doctor support flow"]
    entry -->|"nvctl driver support-bundle"| bundle["driver bundle flow"]
    entry -->|"GUI/TUI support action"| ui["support UI wrapper"]

    doctor --> release["release diagnostics"]
    bundle --> release
    ui --> release

    release --> gsp["GSP facts"]
    release --> dkms["DKMS doctor"]
    release --> source["source doctor"]
    release --> container["container runtime doctor"]
    release --> cuda["CUDA/AI diagnostics"]
    release --> logs["journal/dmesg tails"]

    gsp --> redact["redaction policy"]
    dkms --> redact
    source --> redact
    container --> redact
    cuda --> redact
    logs --> redact

    redact --> text["support.txt"]
    redact --> json["support.json"]
    text --> archive["optional tar.gz or gzip"]
    json --> archive
```

## TUI Refresh Model

```mermaid
flowchart TD
    tick["event loop tick"] --> paused{"paused?"}
    paused -->|yes| render["render cached state"]
    paused -->|no| refresh{"refresh interval due?"}
    refresh -->|no| render
    refresh -->|yes| collectors["collectors"]

    collectors --> nvml["GPU metrics"]
    collectors --> proc["graphics/compute process lists"]
    collectors --> drivers["driver/support summaries"]
    collectors --> cuda["CUDA/AI cache\nmanual or 60s"]
    collectors --> asus["ASUS power cache"]

    nvml --> state["TuiApp state"]
    proc --> state
    drivers --> state
    cuda --> state
    asus --> state
    state --> render
```

