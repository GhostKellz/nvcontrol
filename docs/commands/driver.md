# Driver Commands

Driver management, release diagnostics, DKMS, GSP, and kernel log workflows.

## Key Commands

```bash
# General driver status
nvctl driver info
nvctl driver check
nvctl driver capabilities

# Branch validation
nvctl driver validate --driver 590

# Release alignment diagnostics
nvctl driver diagnose-release
nvctl driver diagnose-release --format json
nvctl driver diagnose-release --format yaml

# Shareable support bundle
nvctl driver support-bundle
nvctl driver support-bundle --output ~/.local/state/nvcontrol/support/support.txt
nvctl driver support-bundle --gzip --redact-paths --output ~/.local/state/nvcontrol/support/support.txt.gz
nvctl driver support-bundle --redact-ids --log-tail 80

# GSP firmware details
nvctl driver gsp status
nvctl driver gsp diagnostics

# DKMS
nvctl driver dkms status
nvctl driver dkms doctor
nvctl driver dkms fix

# Source-build reproducibility
nvctl driver source status
nvctl driver source doctor
```

## Release Diagnostics

`nvctl driver diagnose-release` reports:

- running kernel vs installed module target
- detected initramfs tool and current boot cmdline
- `/boot` kernel and initramfs entry inventory
- boot/initramfs findings for the running kernel as supportability warnings
- userspace driver version
- loaded kernel module version
- GSP firmware layout/path/file
- expected firmware search paths
- GPU support classification using PCI device IDs when available
- Arch package presence/version hints for common mixed-state problems
- Arch package inventory useful for support bundles on Arch/CachyOS systems

Structured output is available with:

```bash
nvctl driver diagnose-release --format json
nvctl driver diagnose-release --format yaml
```

## Mixed-State Hints

The release diagnostics and `driver check` paths now help surface cases like:

- `nvidia-open` installed without expected firmware path visibility
- kernel/userspace release mismatch
- firmware layout present but incomplete for the detected GPU architecture
- Arch package combinations that suggest partial upgrades

## Notes

- Open-driver-capable status is based on PCI-aware classification first, with model-name fallback.
- Legacy GPUs are still surfaced as proprietary-focused or unsupported where appropriate.

## Examples

### Good State

```text
Running Kernel:  6.18.2-1-cachyos
Module Kernel:   6.18.2-1-cachyos
Kernel Match:    yes
Userspace:       595.58.03
Kernel Module:   595.58.03
Release Match:   aligned at 595.58.03
FW Layout:       per-chip
FW Path:         /lib/firmware/nvidia/gb202/gsp
```

### Bad State

```text
Running Kernel:  6.18.2-1-cachyos
Module Kernel:   6.17.9-arch1-1
Kernel Match:    no
Userspace:       595.58.03
Kernel Module:   590.48.01
Release Match:   mismatch: kernel module 590.48.01 vs userspace 595.58.03
Expected Paths:
  - /lib/firmware/nvidia/gb202/gsp (missing)
```

### Support Bundle

Use `nvctl driver support-bundle` before filing an issue when you suspect:

- kernel/userspace/GSP mismatch
- missing firmware paths
- DKMS rebuild issues
- source tree/version drift
- container runtime / toolkit issues on GPU container setups
- Xid or GSP log failures

Optional flags:

- `--gzip` compresses the report
- `--tarball` packages the report and metadata into a single tar.gz archive
- `--redact-paths` removes literal firmware path strings from the report body
- `--redact-ids` removes PCI and device identifiers from the report body
- `--log-tail` controls how many recent lines are captured for each log section

- plain text output writes a machine-readable sidecar metadata file at `<bundle>.json`
- gzip output is self-contained and appends the metadata JSON inside the compressed report
- tarball output is self-contained and stores `support.txt` plus `support.json` inside the archive

Support bundles now also capture:

- boot cmdline and initramfs tool detection
- `/boot` kernel and initramfs entries
- installed kernel trees under `/lib/modules`
- boot/initramfs findings for the running kernel
- Arch/CachyOS package inventory for NVIDIA/kernel packages
- DKMS doctor, source-build doctor, and container runtime doctor output

Arch pacman hook generation now covers:

- stock `linux`, `linux-lts`, `linux-zen`, and `linux-hardened`
- `linux-cachyos` and `linux-cachyos-lto`
- `linux-tkg-*`
- custom kernels discovered from `/boot/vmlinuz-*` entries, such as `vmlinuz-linux-ghost`

See also:

- `docs/drivers/diagnose-release.md`
- `docs/integration/support-bundle-sample.md`
- `docs/integration/issue-reporting.md`

GUI/TUI support workflow:

- GUI: open the `Support` tab
- GUI: set bundle path, refresh diagnostics, create bundle, or copy a short summary
- TUI: open `nvctl gpu stat`, switch to `Drivers`, then use `b` to create a support bundle and `x` to show the workflow hint
- support artifacts are written to disk only; opening them is always an explicit user action
- test runs suppress support-bundle notifications, but normal interactive CLI/GUI/TUI usage still notifies when a bundle is created
