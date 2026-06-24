# Release Validation

Release validation is the evidence path for deciding whether a tag is ready. It
separates non-mutating checks, live hardware checks, package/install checks, and
explicitly gated hardware mutation.

## Validation Pipeline

```mermaid
flowchart TD
    worktree["Reviewed worktree"] --> static["static checks\nfmt, check, clippy"]
    static --> tests["test suites\nunit + focused CLI/docs/packaging"]
    tests --> audit["dependency audit\ncargo audit + dependency review"]
    audit --> package["package creation\ncargo package"]
    package --> live["live 610+ smoke tests"]
    live --> install["install/update/uninstall smoke"]
    install --> docs["docs and release notes"]
    docs --> tag{"tag candidate?"}

    live --> hardware_gate["hardware-mutating tests\nexplicit opt-in only"]
    hardware_gate --> docs
```

## Gate Matrix

| Gate | Command Or Evidence | Mutates System | Release Meaning |
|------|---------------------|----------------|-----------------|
| Formatting | `cargo fmt --all --check` | No | Rust style is stable |
| Compile | `cargo check --all-targets` | No | All targets typecheck |
| Clippy | `cargo clippy --all-targets -- -D warnings` | No | No accepted warning debt |
| Tests | `cargo test` or focused release suites | No by default | Parser, docs, packaging, regressions pass |
| Audit | `cargo audit` | No | No known RustSec vulnerabilities |
| Package | `cargo package --allow-dirty --no-verify` | No | Crate package can be assembled |
| Driver live smoke | `nvctl driver info`, `diagnose-release`, `validate --driver 610` | Read-only | Actual GPU/driver path is visible |
| Support smoke | `nvctl doctor --support` and support-bundle creation | Writes support artifact | Support artifacts are usable |
| Install smoke | installer, desktop file, icons, completions, services | Yes | Release artifact installs and removes cleanly |
| Vibrance regression | `NVCONTROL_RUN_HARDWARE_TESTS=1 ... --ignored` | Yes | Explicit live display mutation path works |

## Live 610+ Smoke Flow

```mermaid
sequenceDiagram
    participant Operator
    participant Nvctl as nvctl
    participant Driver as NVIDIA driver
    participant FS as support output

    Operator->>Nvctl: nvctl driver info
    Nvctl->>Driver: query NVML, GSP, modules, runtime probes
    Driver-->>Nvctl: 610+ open-driver facts
    Nvctl-->>Operator: 610+ feature section

    Operator->>Nvctl: nvctl driver diagnose-release
    Nvctl->>Driver: kernel/userspace/GSP alignment checks
    Nvctl-->>Operator: findings + fix hints

    Operator->>Nvctl: nvctl doctor --support
    Nvctl->>FS: write support tarball + metadata
    FS-->>Operator: artifact path
```

## Release Decision Flow

```mermaid
flowchart TD
    checks["All required gates collected"] --> blockers{"Any blocking failures?"}
    blockers -->|yes| fix["fix or explicitly defer before tag"]
    fix --> checks
    blockers -->|no| version{"Version/tag state clean?"}
    version -->|no| decide["choose new version or explicit tag repair plan"]
    version -->|yes| artifacts{"Artifacts install and run?"}
    artifacts -->|no| fix
    artifacts -->|yes| publish["tag + publish release"]
```

## Known Non-Goals For Normal Checks

- Normal CI and local test runs must not change display vibrance.
- Normal diagnostics must not start or stop Ollama, Docker containers, or user
  services.
- Helper probes such as `vulkaninfo` must be isolated from overlay layers when
  possible and treated as optional runtime evidence.

