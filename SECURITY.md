# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in nvcontrol, please report it responsibly:

1. **Do not** open a public GitHub issue for security vulnerabilities
2. Email security concerns to the maintainers (see CONTRIBUTING.md for contact)
3. Include detailed steps to reproduce the issue
4. Allow reasonable time for a fix before public disclosure

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.8.x   | Yes       |
| < 0.8   | No        |

## Security Considerations

### Privilege Requirements

nvcontrol interacts with NVIDIA kernel drivers and requires elevated privileges for certain operations:

| Operation | Privilege | Reason |
|-----------|-----------|--------|
| Read GPU info | User | NVML read-only access |
| Fan control | Root | Direct NVKMS ioctl |
| Power limits | Root | NVML privileged API |
| Overclock/Undervolt | Root | Direct NVKMS ioctl |
| Profile switching | User | Config file only |

### Driver Interaction

- **NVML API**: Read-only queries via `nvml-wrapper` crate
- **NVKMS ioctl**: Direct `/dev/nvidia*` device access for fan/clock control
- **No kernel modules**: Does not load or modify kernel modules

### Data Storage

| Location | Contents | Permissions |
|----------|----------|-------------|
| `~/.config/nvcontrol/` | User profiles, settings | User-only (0600) |
| `~/.local/state/nvcontrol/` | Support bundles, runtime state, diagnostics output | User-only |

### Network

nvcontrol makes **no network connections**. All operations are local.

## Dependency Auditing

We use `cargo audit` to check for known vulnerabilities in dependencies.

### Running an Audit

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit
cargo audit
```

### v0.8.11 Advisory Status

Audit performed: 2026-08-04
Tool version: cargo-audit 0.22.x
Result: **0 known RustSec vulnerabilities and 0 warnings reported by `cargo audit`.**

This release cleared four advisories and two unsoundness warnings that were
reachable on v0.8.10:

| Advisory | Crate | Resolution |
|----------|-------|------------|
| RUSTSEC-2026-0194 / RUSTSEC-2026-0195 | `quick-xml` 0.37.5, 0.39.4 | Both trees collapsed onto `quick-xml` 0.41.0 |
| RUSTSEC-2026-0190 (unsound) | `anyhow` 1.0.102 | Updated to 1.0.104 |
| RUSTSEC-2026-0221 (unsound) | `event-listener` 5.4.1 | Updated to 5.4.2 |

These fixes were gated behind the toolchain: the MSRV-aware resolver held every
patched release back while `rust-toolchain.toml` pinned 1.95.0. Raising the pin to
1.97.1 (MSRV 1.97) made all of them reachable.

#### Accepted Warnings

None. The deprecated `serde_yaml` dependency was replaced with the maintained
`serde_norway` fork.

#### Remediation Plan

Continue running `cargo audit` before each release. When advisories land in
transitive GUI/TUI stacks, check whether the MSRV pin is masking available fixes
before assuming no patch exists.

## Security Hardening Checklist

For users running nvcontrol:

- [ ] Run GUI/TUI as unprivileged user when possible (read-only monitoring)
- [ ] Use `sudo nvctl` only for operations requiring root (fan control, power limits)
- [ ] Review profiles before importing from untrusted sources
- [ ] Keep NVIDIA drivers updated (535+ required)
- [ ] Review support bundles before sharing if using `nvctl doctor --support` or `nvctl driver support-bundle`

## Changelog

Security-related changes are documented in [CHANGELOG.md](CHANGELOG.md) under the "Security" section for each release.
