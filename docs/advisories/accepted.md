# Accepted Advisories

No RustSec security advisories are knowingly accepted for the current locked nvcontrol dependency graph.

## Accepted Non-Advisory Dependency Risk

None. The previously accepted `serde_yaml` 0.9.x deprecation risk was retired in
v0.8.11 by migrating to the maintained `serde_norway` fork, which is API-compatible
and keeps the existing YAML config/profile surfaces working unchanged.

## Recording Rule

If a vulnerability or dependency risk is knowingly accepted for a release, record:

- advisory identifier or dependency name
- affected dependency or module
- reason for acceptance
- compensating controls
- target release or condition for removal
- verification command used during release review

Keep this file in sync with [../../SECURITY.md](../../SECURITY.md), [../../CHANGELOG.md](../../CHANGELOG.md), and any future audit ignore configuration.
