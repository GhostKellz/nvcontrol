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
| `/tmp/nvcontrol/` | Runtime state | User-only |

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

### v0.8.6 Advisory Status

Audit performed: 2026-04-03
Tool version: cargo-audit 0.21.x
Result: **0 vulnerabilities on Linux**, 13 warnings

#### Accepted Warnings

The following advisory warnings are accepted for v0.8.6. They are transitive dependencies with low practical risk:

| Advisory | Crate | Severity | Source | Disposition |
|----------|-------|----------|--------|-------------|
| RUSTSEC-2026-0009 | `time` | Medium (DoS) | `mac-notification-sys` | macOS-only, not compiled on Linux |
| RUSTSEC-2024-0412-0420 | GTK3 bindings | Warning (unmaintained) | `tray-icon` | Functional, awaiting GTK4 migration |
| RUSTSEC-2024-0429 | `glib` | Warning (unsound) | `tray-icon` -> GTK3 | VariantStrIter not in code path |
| RUSTSEC-2026-0002 | `lru` | Warning (unsound) | `ratatui` | IterMut not in code path |
| RUSTSEC-2025-0119 | `number_prefix` | Warning (unmaintained) | `indicatif` | Progress bars only |
| RUSTSEC-2024-0436 | `paste` | Warning (unmaintained) | `ratatui`, `metal` | Macro crate, no runtime risk |
| RUSTSEC-2024-0370 | `proc-macro-error` | Warning (unmaintained) | `glib-macros` | Compile-time only |

#### Rationale

- **time DoS**: Only affects macOS via `mac-notification-sys`. nvcontrol is Linux-only.
- **GTK3 unmaintained**: `tray-icon` crate uses GTK3 for system tray on Linux. Still functional, upstream will migrate to GTK4.
- **glib/lru unsoundness**: Affected iterator APIs are not used in our code paths.
- **Compile-time crates**: `paste`, `proc-macro-error` only run at compile time.

#### Remediation Plan

These warnings will be addressed when:
1. `tray-icon` migrates to GTK4 bindings
2. `ratatui` updates `lru` dependency
3. `indicatif` replaces `number_prefix`

## Security Hardening Checklist

For users running nvcontrol:

- [ ] Run GUI/TUI as unprivileged user when possible (read-only monitoring)
- [ ] Use `sudo nvctl` only for operations requiring root (fan control, power limits)
- [ ] Review profiles before importing from untrusted sources
- [ ] Keep NVIDIA drivers updated (535+ required)
- [ ] Verify binary signatures if installing from releases

## Changelog

Security-related changes are documented in [CHANGELOG.md](CHANGELOG.md) under the "Security" section for each release.
