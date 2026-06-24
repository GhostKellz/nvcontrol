# Accepted Advisories

No RustSec security advisories are knowingly accepted for `nvcontrol v0.8.10`.

## Accepted Non-Advisory Dependency Risk

| Item | Status | Reason | Removal Path |
|------|--------|--------|--------------|
| `serde_yaml` 0.9.x | Accepted compatibility risk | Upstream crate is deprecated, but `cargo audit` does not currently report a RustSec vulnerability for it. nvcontrol still uses YAML-compatible config/profile surfaces. | Replace with a maintained YAML parser or migrate YAML support behind an explicit compatibility path after profile/config compatibility tests are in place. |

## Recording Rule

If a vulnerability or dependency risk is knowingly accepted for a release, record:

- advisory identifier or dependency name
- affected dependency or module
- reason for acceptance
- compensating controls
- target release or condition for removal
- verification command used during release review

Keep this file in sync with [../../SECURITY.md](../../SECURITY.md), [../../CHANGELOG.md](../../CHANGELOG.md), and any future audit ignore configuration.
