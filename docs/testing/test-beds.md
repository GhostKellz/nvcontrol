# Test beds

Arch is the primary build and development environment. Fedora and Pop!_OS are
native-package and desktop acceptance targets. Debian is planned, not currently
validated. Connection details and guest lifecycle notes live in the local task
runbook; this page defines coverage rather than access.

## Hardware coverage

| Hardware | Role | Evidence requirement |
| --- | --- | --- |
| RTX 5090, Arch | Primary development host | Checkout checks and live readback; preserve user's settings |
| RTX 3070, Proxmox | Fedora and Pop!_OS/COSMIC acceptance | Native RPM/Deb installs; only one shared-GPU guest running |
| RTX 2060, nvghostrunner | User-reported separate server | Record actual driver/runner before claiming coverage |
| RTX 4090, local | Additional available GPU | Schedule an explicit hardware run; not implied by existing CI |


More GPUs can be added after recording their OS, driver and ownership. A listed GPU
is not a passed test. See the [methodology](methodology.md) and dated release
evidence for completed runs.
