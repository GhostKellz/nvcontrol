# Issue Reporting

For driver, GSP, DKMS, or firmware problems, include a support bundle whenever possible.

## Recommended Workflow

```bash
# Human-readable release diagnostics
nvctl driver diagnose-release

# Machine-readable diagnostics
nvctl driver diagnose-release --format json

# Shareable bundle with path redaction
nvctl driver support-bundle --gzip --redact-paths --output ~/.local/state/nvcontrol/support/support.txt.gz

# Broader redaction with extra log context
nvctl driver support-bundle --gzip --redact-paths --redact-ids --log-tail 80 \
  --output ~/.local/state/nvcontrol/support/support.txt.gz

# One-shot diagnostics + support tarball
nvctl doctor --support --output ~/.local/state/nvcontrol/support/doctor-support.tar.gz
```

## What to Attach

- the output of `nvctl driver diagnose-release`
- the generated support bundle
- if you used plain text output, the generated `<bundle>.json` metadata sidecar for machine-readable diagnostics
- a short description of what changed:
  - kernel update
  - driver update
  - switch between `nvidia` and `nvidia-open`
  - suspend/resume failure
  - Xid or GSP log errors

## Good Report Example

- kernel version included
- driver branch/version included
- whether the issue is open-driver or proprietary-driver specific
- support bundle attached

## Real Validation Cases

Good cases to validate before reporting or after changing packages:

- proprietary 595 branch installed cleanly
- `nvidia-open` 595 branch installed cleanly
- kernel updated but DKMS not rebuilt yet
- expected firmware path missing after package switch
- `nvidia` and `nvidia-open` accidentally installed together
- `nvidia-utils` version does not match detected kernel/userspace release
- firmware ownership resolves to beta packages while the loaded stack is not on the same branch
- boot entry or initramfs image missing for the running kernel
- Arch/CachyOS custom kernel names such as `linux-cachyos-lto` or `linux-ghost` after updates

For custom kernels, nvcontrol derives pacman hook targets from discovered `/boot/vmlinuz-*` entries instead of relying only on a fixed package-name list.

## Exact Fix Commands

When nvcontrol detects common Arch package problems, the CLI, GUI Support tab, and TUI Drivers tab now surface exact fix-oriented suggestions such as:

- removing the wrong branch package
- syncing `nvidia-utils` to the loaded branch
- reinstalling firmware packages
- running the DKMS repair path
- reviewing boot/initramfs regeneration after kernel or branch changes

## When to Use Redaction

Use `--redact-paths` if you want to avoid sharing exact local path details while still preserving most diagnostic value.

Use `--redact-ids` as well if you want to hide PCI bus and raw device identifiers in the text bundle.

See also:

- `docs/integration/support-bundle-sample.md`
- `docs/release-checklist.md`
