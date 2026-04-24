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

### v0.8.7 Advisory Status

Audit performed: 2026-04-22
Tool version: cargo-audit 0.21.x
Result: **0 known vulnerabilities on Linux**, with the GTK3 tray surface removed and the advisory set reduced accordingly

#### Accepted Warnings

The following advisory warnings are accepted for v0.8.7. They are transitive dependencies with low practical risk:

| Advisory | Crate | Severity | Source | Disposition |
|----------|-------|----------|--------|-------------|
| RUSTSEC-2026-0009 | `time` | Medium (DoS) | `mac-notification-sys` | macOS-only, not compiled on Linux |
| RUSTSEC-2026-0002 | `lru` | Warning (unsound) | `ratatui` | IterMut not in code path |
| RUSTSEC-2025-0119 | `number_prefix` | Warning (unmaintained) | `indicatif` | Progress bars only |

#### Rationale

- **time DoS**: Only affects macOS via `mac-notification-sys`. nvcontrol is Linux-only.
- **lru unsoundness**: The affected iterator API is not used in nvcontrol paths.
- **number_prefix**: Used only for progress display.

#### Remediation Plan

These warnings will be addressed when:
1. `ratatui` updates `lru` dependency
2. `indicatif` replaces `number_prefix`

## Security Hardening Checklist

For users running nvcontrol:

- [ ] Run GUI/TUI as unprivileged user when possible (read-only monitoring)
- [ ] Use `sudo nvctl` only for operations requiring root (fan control, power limits)
- [ ] Review profiles before importing from untrusted sources
- [ ] Keep NVIDIA drivers updated (535+ required)
- [ ] Review support bundles before sharing if using `nvctl doctor --support` or `nvctl driver support-bundle`

## Changelog

Security-related changes are documented in [CHANGELOG.md](CHANGELOG.md) under the "Security" section for each release.
