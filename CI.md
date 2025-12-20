# nvcontrol CI Infrastructure

Self-hosted CI with real NVIDIA GPU hardware for comprehensive testing across GPU generations.

## Current CI Workflow

```
Push/PR to main → Self-hosted Runner (RTX 2060) → Build + Test + GPU Smoke Tests
```

### Pipeline Stages

| Stage | Description |
|-------|-------------|
| **Build CLI** | `cargo build --bin nvctl --release --no-default-features` |
| **Clippy** | Zero warnings policy (`-D warnings`) |
| **Format Check** | `cargo fmt --check` |
| **Unit Tests** | `cargo test --lib --no-default-features` |
| **GPU Smoke Test** | Real GPU commands (`gpu info`, `doctor`) |
| **CLI Test Suite** | Comprehensive command validation |

### Self-Hosted Runner

The CI runs on dedicated hardware with a real NVIDIA GPU, not containers or emulation. This ensures:

- Real NVML/nvidia-smi integration testing
- Actual driver interaction validation
- GPU-specific feature detection works correctly

## Current Hardware

| Location | GPU | Role | Status |
|----------|-----|------|--------|
| Dev Workstation | **RTX 5090** (Blackwell) | Daily driver, development | Active |
| Proxmox Cluster | **RTX 4090** (Ada) | VM passthrough, testing | Active |
| Proxmox Cluster | **RTX 3070** (Ampere) | VM passthrough, testing | Active |
| CI Server | **RTX 2060** (Turing) | CI runner (nv-osmium) | Active |

### Architecture Coverage

| Architecture | Generation | Current Coverage |
|--------------|------------|------------------|
| Blackwell | RTX 50 series | RTX 5090 (dev) |
| Ada Lovelace | RTX 40 series | RTX 4090 (proxmox) |
| Ampere | RTX 30 series | RTX 3070 (proxmox) |
| Turing | RTX 20 series | RTX 2060 (CI) |

## Planned Infrastructure

### Multi-GPU CI Matrix

Goal: Test every commit against multiple GPU generations to ensure compatibility.

```
┌─────────────────────────────────────────────────────────────┐
│                     GitHub Actions                          │
├─────────────────────────────────────────────────────────────┤
│  Push/PR to main                                            │
│       │                                                     │
│       ▼                                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│  │ Turing  │  │ Ampere  │  │  Ada    │  │Blackwell│        │
│  │ RTX 20  │  │ RTX 30  │  │ RTX 40  │  │ RTX 50  │        │
│  │  2060   │  │  3070   │  │  4090   │  │  5090   │        │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘        │
│       │            │            │            │              │
│       ▼            ▼            ▼            ▼              │
│  ┌─────────────────────────────────────────────────┐       │
│  │              All Tests Pass?                     │       │
│  │     Turing ✓  Ampere ✓  Ada ✓  Blackwell ✓      │       │
│  └─────────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

### Planned Runners

| Runner Name | GPU | Architecture | Location | Status |
|-------------|-----|--------------|----------|--------|
| nv-osmium | RTX 2060 | Turing | CI Server | **Active** |
| nv-ampere | RTX 3070/3080 | Ampere | TBD | Planned |
| nv-ada | RTX 4070/4090 | Ada | TBD | Planned |
| nv-blackwell | RTX 5090 | Blackwell | Dev Workstation | Planned |

### Future Workflow

```yaml
# Planned: .github/workflows/ci-matrix.yml
jobs:
  build:
    runs-on: ubuntu-latest
    # Build once, test on multiple GPUs

  test-turing:
    needs: build
    runs-on: [self-hosted, gpu-turing]

  test-ampere:
    needs: build
    runs-on: [self-hosted, gpu-ampere]

  test-ada:
    needs: build
    runs-on: [self-hosted, gpu-ada]

  test-blackwell:
    needs: build
    runs-on: [self-hosted, gpu-blackwell]
```

## Test Categories

### Unit Tests (No GPU Required)
- Config parsing
- Theme handling
- Error recovery logic
- Data structures

### GPU Smoke Tests (Real GPU Required)
- `nvctl gpu info` - Basic GPU detection
- `nvctl fan info` - Fan control detection
- `nvctl power status` - Power management
- `nvctl doctor` - System diagnostics
- `nvctl driver info` - Driver detection

### Architecture-Specific Tests (Planned)
| Feature | Turing | Ampere | Ada | Blackwell |
|---------|--------|--------|-----|-----------|
| GSP Firmware | Optional | Optional | Required | Required |
| DLSS 2 | ✓ | ✓ | ✓ | ✓ |
| DLSS 3 (Frame Gen) | ✗ | ✗ | ✓ | ✓ |
| DLSS 4 | ✗ | ✗ | ✗ | ✓ |
| AV1 Encode | ✗ | ✗ | ✓ | ✓ |
| DisplayPort 2.1 | ✗ | ✗ | ✗ | ✓ |

## Running CI Locally

### Prerequisites
- NVIDIA GPU with driver 535+
- Rust stable toolchain
- `nvidia-smi` accessible

### Commands

```bash
# Full CI pipeline
cargo build --bin nvctl --release --no-default-features
cargo clippy --bin nvctl --lib --no-default-features -- -D warnings
cargo fmt --check
cargo test --lib --no-default-features

# GPU smoke test
./target/release/nvctl gpu info
./target/release/nvctl doctor
```

### With All Features (GUI/TUI)

```bash
# Full build with GUI
cargo build --release

# All tests
cargo test
```

## CI Configuration

### Environment Variables

| Variable | Description |
|----------|-------------|
| `PATH` | Must include rustup toolchain bin |
| `RUST_LOG` | Set to `debug` for verbose output |

### Runner Requirements

- Linux x86_64
- NVIDIA driver 535+ (550+ recommended)
- nvidia-open preferred for GSP testing
- Rust stable (1.75+)
- 8GB+ RAM
- SSD recommended for fast builds

### Runner Labels

| Label | Meaning |
|-------|---------|
| `self-hosted` | Not GitHub-hosted |
| `gpu-turing` | RTX 20 series GPU |
| `gpu-ampere` | RTX 30 series GPU |
| `gpu-ada` | RTX 40 series GPU |
| `gpu-blackwell` | RTX 50 series GPU |

## Troubleshooting CI

### Common Issues

**Build fails with NVML errors:**
```
GPU not accessible in CI environment
```
Fix: Ensure runner has GPU access and nvidia-smi works.

**Clippy warnings fail build:**
```
error: ... warning: ...
```
Fix: Run `cargo clippy -- -D warnings` locally and fix all warnings.

**Format check fails:**
```
Diff in src/...
```
Fix: Run `cargo fmt` before committing.

### Checking Runner Status

```bash
# On the runner machine
nvidia-smi
rustc --version
cargo --version
```

## Contributing

When adding new features:

1. Ensure unit tests don't require GPU
2. Add GPU smoke tests for new commands
3. Document architecture-specific behavior
4. Test locally with `cargo clippy -- -D warnings`
5. Run `cargo fmt` before committing

## References

- [GitHub Actions Self-Hosted Runners](https://docs.github.com/en/actions/hosting-your-own-runners)
- [NVIDIA Driver Documentation](https://docs.nvidia.com/datacenter/tesla/index.html)
- [nvcontrol DKMS Guide](docs/DKMS.md)
- [nvcontrol GSP Guide](docs/GSP.md)
