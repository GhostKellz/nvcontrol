# NVIDIA DKMS Integration

DKMS diagnostics and explicit rebuilds for NVIDIA modules on Arch, Fedora, and
Debian-family systems. Package-managed driver updates and manually maintained
Git sources have different ownership; nvcontrol must preserve that boundary.

## Distribution integration

| System | Kernel-update integration | Driver source |
| --- | --- | --- |
| Arch and derivatives | Standard DKMS pacman hooks, matching kernel headers | Packaged DKMS source or explicitly registered manual source |
| Fedora with NVIDIA DKMS packages | DKMS kernel-install hook, matching kernel-devel | RPM-owned source |
| Fedora with RPM Fusion akmods | akmods packaging, not an NVIDIA DKMS registration | Use the package's akmods workflow |
| Pop!_OS, Ubuntu, Debian with DKMS packages | Kernel/header package hooks | Debian-package-owned source |

Status queries the local package database and resolves `nvidia` or `nvidia-open`
registrations. Ambiguous or mismatched registrations block builds rather than
selecting an arbitrary source tree. A present hook establishes integration, not
proof that a future build will succeed: headers, source, compiler and signing
requirements must also be satisfied.

A kernel update can rebuild the same registered driver when its source enables
`AUTOINSTALL`. A driver update must also align userspace libraries. DKMS does not
fetch a Git tag or upgrade userland. Keep manually registered source immutable;
stage a new version separately, prove builds, then coordinate matching packages.
Do not remove the old working registration before a new build succeeds.

`dkms fix` builds without unregistering modules. Failures return a failing exit
status. `dkms hook` recognizes the standard Arch DKMS hook and does not add a
second autoinstall hook. Source updates refuse to change registered Git source
in place when a new tag is found. Package-owned source should be updated by its
own package manager. Explicit `dkms unregister` removes installed DKMS modules;
it is not a repair step.

The source tree's build configuration remains authoritative. In particular,
Clang/LTO kernels need compatible compiler/linker settings. nvcontrol must not
overwrite an existing `dkms.conf` to impose generic defaults.


## Commands

| Command | Behavior |
| --- | --- |
| `nvctl driver dkms status` | Registration identity, source ownership, installed kernel/header/module coverage, native hook paths |
| `nvctl driver dkms doctor` | Read-only source/header/registration findings |
| `nvctl driver dkms setup` | Register an existing source using its own dkms.conf; does not install missing prerequisites |
| `nvctl driver dkms build` | Install the resolved registration for kernels with headers; `--kernel` targets one kernel |
| `nvctl driver dkms fix` | Same registration-preserving build path; no blanket remove/re-add |
| `nvctl driver dkms logs` | Existing DKMS build logs and legacy custom-hook logs when present |
| `nvctl driver dkms hook` | Verify standard Arch integration; reports missing package hook instead of writing a duplicate |
| `nvctl driver dkms unregister` | Explicit removal of the selected registration and its installed modules |
| `nvctl driver dkms cleanup` | Preview old kernel module cleanup; mutation requires `--execute` |

Use each command's `--help` for current options. Source changes and explicit build,
setup, unregister and cleanup actions can require elevated privileges. Normal
status/doctor commands do not change driver state.

## Source build commands

`driver source status` and `driver source doctor` inspect the source tree.
`driver source init` is an explicit manual-source setup operation; prefer the
distribution's driver source package when available. `driver source sync` uses
the same resolved DKMS build path. `driver source update` fetches tags, but refuses
to switch an active registered source to a new tag in place. A staged driver
upgrade is a separate operation requiring matching userland and verified modules.

Do not confuse the module registration name with kernel output names:
`nvidia-open/<version>` and `nvidia/<version>` both produce `nvidia.ko` and its
companion modules. Detection must use the registration identity, while module
presence uses kmod's resolver for distro paths and compression formats.

## Acceptance

Run the relevant [distro diagnostic script](../testing/ci-workflow.md) before and
after package/kernel changes. Record the loaded driver and the module available
for the next boot. Older retained kernels can legitimately lack headers/modules;
report them separately from the running kernel rather than calling the working
GPU broken. A successful rebuild alone does not prove boot or suspend behavior.

The [test-bed matrix](../testing/test-beds.md) distinguishes current hardware
coverage from future Debian and additional GPU runs. RPM Fusion akmods is not
validated by a successful NVIDIA-DKMS test; keep those results separate.
