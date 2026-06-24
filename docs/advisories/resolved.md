# Resolved Advisories

This file tracks dependency and code-level security or stability issues resolved by release work.

## v0.8.10

| Item | Affected Area | Resolution | Verification |
|------|---------------|------------|--------------|
| Release metadata drift after hotfix bump | Packaging, docs, generated shell surfaces | Updated Cargo-aligned packaging to `0.8.10`, refreshed shell completions, and added the packaged `nvctl(1)` man page. | packaging sanity tests, clap help tests |
| Fan CLI documentation exposed commands not wired in clap | `nvctl fan` CLI and docs | Added `fan auto` and `fan curve show/apply/set` command handling, with current-temperature application for saved profiles when NVML temperature is available. | `cargo check --all-targets`, help contract tests |

## v0.8.9

| Item | Affected Area | Resolution | Verification |
|------|---------------|------------|--------------|
| Rust dependency audit warning for transitive `memmap2` | Rust dependency graph | Refreshed compatible dependencies so `memmap2` resolves to `0.9.11`. | `cargo audit` |
| Dependabot Rust dependency drift | Direct Rust dependencies | Updated direct crates requested by Dependabot: `thiserror` 2, `which` 8, `directories` 6, `nvml-wrapper` 0.12.1, `toml` 1.1, `sysinfo` 0.39, `notify` 8.2, `dirs` 6, `console` 0.16, and `nix` 0.31. | `cargo check --all-targets`, `cargo clippy --all-targets -- -D warnings`, focused release tests |
| Vulkan helper crash through implicit overlays | `nvctl driver info` runtime probing | Changed the Vulkan extension probe to launch `vulkaninfo --summary` with MangoHud/vkBasalt disabled and explicit layer/preload variables cleared. The crash source was an overlay layer, not the NVIDIA 610.43.02 driver path. | overlay-safe `vulkaninfo --summary`, `target/debug/nvctl driver info` |
| `sysinfo` API migration breakage | Game process detection | Migrated process refresh and process-name handling to the `sysinfo` 0.39 API. | focused CLI/regression tests and `cargo check --all-targets` |

## Recording Rule

When a dependency or code update clears an advisory or release blocker, record:

- advisory identifier when one exists
- affected dependency or module
- fixed version or commit
- release that shipped the fix
- verification command or live evidence that confirmed the fix

Security-relevant release notes should also be reflected in [../../CHANGELOG.md](../../CHANGELOG.md) and [../../SECURITY.md](../../SECURITY.md).
