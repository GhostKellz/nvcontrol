# CI and local testing

## Current workflow

[GitHub CI](../../.github/workflows/ci.yml) remains unchanged. Pushes and pull
requests to main run on a self-hosted runner. The job builds the CLI without
default features, runs strict CLI/library Clippy and library tests, and checks
GPU/CLI commands. Formatting currently emits a warning rather than failing.
The job display name is not proof of the physical GPU currently attached.
The compiler pin is in [rust-toolchain.toml](../../rust-toolchain.toml).

## On-demand local gates

Run from a checkout. Bash and the pinned Rust toolchain are needed for builds;
installed-artifact diagnostics use `NVCTL=/usr/bin/nvctl` and do not need Cargo.
Distro diagnostics check their OS identity before running. They inventory packages,
the loaded driver, module metadata and DKMS when installed, then run read-only
CLI checks. They do not install packages or modify GPU settings. Output can contain
host/package information; review it before sharing.

```bash
dev/test-all.sh
NVCTL=/usr/bin/nvctl dev/diagnose-arch.sh
NVCTL=/usr/bin/nvctl dev/diagnose-fedora.sh
NVCTL=/usr/bin/nvctl dev/diagnose-popos.sh
```

`dev/test-features.sh` replaces the root `test_no_tray.sh`: it builds CLI and GUI
with explicit features and retains the build cache. `dev/test-all.sh` adds strict
formatting, all-target/all-feature Clippy and all-feature integration tests.
`dev/test-cli.sh` uses Cargo's current checkout unless `NVCTL` selects an exact
artifact. Required command failures produce a nonzero exit status. Optional tools
are labeled skipped; a successful command can still report an unavailable feature.

The only scripted hardware write is a separate checkout regression:

```bash
NVCONTROL_RUN_HARDWARE_TESTS=1 dev/test-hardware.sh --vibrance
```

It captures, applies, reads back and restores exact original vibrance levels.
It does not run every ignored test. Run it with exclusive access to display settings;
an external process changing vibrance concurrently invalidates the comparison.

GUI/TUI acceptance remains interactive: launch the actual installed package,
check the dock icon, scroll every tab at the guest's normal resolution, compare
Settings and GPU memory units, inspect driver/kernel diagnostics, and exercise
TUI navigation/help. A process surviving five seconds is not GUI acceptance.
See [release validation](release-validation.md) for artifact and release gates.

See [test beds](test-beds.md) for the hardware matrix and [methodology](methodology.md) for planned expansion.
