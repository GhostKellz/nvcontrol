# Backend Architecture

nvcontrol uses a backend abstraction layer so the CLI, TUI, GUI, diagnostics, and tests can share the same GPU and display access model without opening duplicate driver sessions or shelling out through unreviewed command paths.

## Architecture Map

```mermaid
flowchart TD
    subgraph Surfaces["User-facing surfaces"]
        CLI["nvctl CLI"]
        TUI["TUI dashboard"]
        GUI["nvcontrol GUI"]
        Notify["Notifications"]
        Tests["Mock-driven tests"]
    end

    subgraph Context["GuiBackendContext"]
        NvmlShared["SharedNvmlBackend\nArc<dyn NvmlBackend>"]
        DisplayShared["SharedDisplayRunner\nArc<dyn DisplayCommandRunner>"]
        Status["BackendStatus"]
        Tracker["StatusTracker\nhotplug debounce"]
        Cache["CachedMetrics\nper GPU"]
    end

    subgraph NvmlLayer["NVML backend layer"]
        NvmlTrait["NvmlBackend trait"]
        RealNvml["RealNvmlBackend\nnvml_wrapper"]
        MockNvml["MockNvmlBackend\nunit/integration tests"]
    end

    subgraph DisplayLayer["Display command layer"]
        DisplayTrait["DisplayCommandRunner trait"]
        ShellRunner["ShellDisplayRunner\nallow-listed commands"]
        MockDisplay["MockDisplayRunner\nKDE/GNOME/Hyprland/Sway mocks"]
    end

    subgraph System["Local system interfaces"]
        Driver["NVIDIA driver\nlibnvidia-ml.so"]
        Device["/dev/nvidiactl\nNVKMS ioctls"]
        Helpers["xrandr, nvidia-settings,\nhyprctl, gsettings,\nkscreen-doctor, swaymsg"]
    end

    CLI --> Context
    TUI --> Context
    GUI --> Context
    Notify --> NvmlShared
    Tests --> MockNvml
    Tests --> MockDisplay

    Context --> NvmlTrait
    Context --> DisplayTrait
    Context --> Status
    Context --> Tracker
    Context --> Cache

    NvmlTrait --> RealNvml
    NvmlTrait --> MockNvml
    DisplayTrait --> ShellRunner
    DisplayTrait --> MockDisplay

    RealNvml --> Driver
    ShellRunner --> Helpers
    ShellRunner --> Device
```

## Component Responsibilities

| Component | Primary type | Responsibility |
|-----------|--------------|----------------|
| Shared NVML backend | `SharedNvmlBackend` | Shared `Arc<dyn NvmlBackend>` used by GPU, monitoring, fan, power, notifications, multi-GPU, and UI paths |
| Real NVML backend | `RealNvmlBackend` | Production NVML access through `nvml-wrapper` |
| Mock NVML backend | `MockNvmlBackend` | Deterministic GPU metrics, device counts, and error states for tests |
| Shared display runner | `SharedDisplayRunner` | Shared `Arc<dyn DisplayCommandRunner>` for display helper commands |
| Shell display runner | `ShellDisplayRunner` | Production runner that only executes allow-listed display helper binaries |
| Mock display runner | `MockDisplayRunner` | Deterministic compositor/display command responses for tests |
| Backend context | `GuiBackendContext` | Combines NVML, display runner, device count, driver version, status, cache, and status debounce |
| Status tracker | `StatusTracker` | Debounces backend availability changes so UI layers do not flicker during hotplug events |
| Metrics cache | `CachedMetrics` | Keeps the latest successful GPU metrics and lets UI code detect stale values |

## Backend Creation Flow

```mermaid
sequenceDiagram
    participant Surface as CLI/TUI/GUI/Test
    participant Context as GuiBackendContext
    participant Nvml as NvmlBackend
    participant Display as DisplayCommandRunner
    participant Status as StatusTracker

    Surface->>Context: new(), mock(), or with_backends()
    Context->>Nvml: is_available()
    Context->>Display: is_available()
    Context->>Context: derive BackendStatus
    Context->>Status: initialize reported status
    Context-->>Surface: shared backend context
```

Production callers use `GuiBackendContext::new()` or the shared backend constructors. Tests use `GuiBackendContext::mock()` or `GuiBackendContext::with_backends(...)` so they can exercise UI and command behavior without live NVIDIA hardware.

## Runtime Status Model

```mermaid
stateDiagram-v2
    [*] --> Available
    Available --> PendingNvmlDown: NVML unavailable observed
    Available --> PendingDisplayDown: display runner unavailable observed
    PendingNvmlDown --> Available: backend recovers before debounce threshold
    PendingDisplayDown --> Available: backend recovers before debounce threshold
    PendingNvmlDown --> NvmlUnavailable: debounce threshold elapsed
    PendingDisplayDown --> DisplayUnavailable: debounce threshold elapsed
    NvmlUnavailable --> Available: NVML available again
    DisplayUnavailable --> Available: display runner available again
    NvmlUnavailable --> AllUnavailable: display runner also unavailable
    DisplayUnavailable --> AllUnavailable: NVML also unavailable
    AllUnavailable --> Available: both backends available
```

`BackendStatus` is intentionally coarse:

| Status | Meaning |
|--------|---------|
| `Available` | NVML and display command runner are available |
| `NvmlUnavailable(String)` | Display path is available, but NVML is unavailable |
| `DisplayUnavailable(String)` | NVML is available, but the display command runner is unavailable |
| `AllUnavailable { ... }` | Neither backend path is currently available |

## Metrics Cache Flow

```mermaid
flowchart TD
    request["UI or command requests metrics"] --> cache{"cache fresh enough?"}
    cache -->|yes| cached["return cached metrics"]
    cache -->|no| nvml["query NvmlBackend"]
    nvml --> ok{"query succeeded?"}
    ok -->|yes| update["update CachedMetrics"]
    update --> output["return live metrics"]
    ok -->|no| fallback{"previous cache exists?"}
    fallback -->|yes| stale["return stale cache with age available"]
    fallback -->|no| error["return backend error"]
```

The cache keeps dashboards usable when a single NVML read fails, while still allowing stale-data checks through the cache age helpers.

## Display Command Security Model

```mermaid
flowchart TD
    caller["Display feature request"] --> runner["ShellDisplayRunner"]
    runner --> allow{"command allowed?"}
    allow -->|no| reject["DisplayError::CommandNotAllowed"]
    allow -->|yes| path{"absolute path known?"}
    path -->|no| reject
    path -->|yes| exec["run helper with explicit args"]
    exec --> parse["parse helper output"]
    parse --> result["DisplayResult"]
```

`ShellDisplayRunner` is not a generic shell wrapper. It is constrained to reviewed helper binaries such as `xrandr`, `nvidia-settings`, `hyprctl`, `gsettings`, `kscreen-doctor`, and `swaymsg`. Commands outside the allow-list are rejected before execution.

## Test Strategy

```mermaid
flowchart LR
    unit["Unit tests"] --> mock_nvml["MockNvmlBackend"]
    unit --> mock_display["MockDisplayRunner"]
    cli["CLI workflow tests"] --> context["GuiBackendContext::with_backends"]
    tui["UI behavior tests"] --> context
    context --> deterministic["deterministic metrics,\nGPU counts, compositor output"]
    deterministic --> assertions["stable assertions without live hardware"]
```

Mock backends cover:

- single-GPU, multi-GPU, and no-GPU NVML states
- deterministic temperatures, fan speeds, power usage, utilization, clocks, and memory values
- compositor-specific display command responses
- unavailable backend states for status and fallback testing

## Current Module Usage

| Module | Backend path |
|--------|--------------|
| `gpu.rs` | `SharedNvmlBackend` |
| `monitoring.rs` | `SharedNvmlBackend` |
| `multi_gpu.rs` | `SharedNvmlBackend` |
| `fan.rs` | `SharedNvmlBackend` |
| `advanced_power.rs` | `SharedNvmlBackend` |
| `interactive_cli.rs` | `SharedNvmlBackend` |
| `notifications.rs` | `SharedNvmlBackend` |
| `tui/mod.rs` | `GuiBackendContext` |
| `display_backend.rs` | `SharedDisplayRunner` and `ShellDisplayRunner` |
| `vrr.rs`, `hdr.rs`, display feature paths | `SharedDisplayRunner` where display helpers are required |

## Operational Boundaries

- Backend docs should describe the current architecture rather than pinning historical release numbers.
- Production backend paths should remain explicit about which local APIs they touch.
- Test examples should use mock backends instead of requiring live NVIDIA hardware.
- Display command execution should stay allow-listed and argument-based.
- UI refresh code should treat temporary backend loss as a debounced state change, not an immediate hard failure.
