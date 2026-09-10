# CachyOS

CachyOS is Arch-based and should use nvcontrol's Arch userspace path, but its
kernel and NVIDIA module packaging are distinct. It is not yet a dedicated
nvcontrol hardware test target. The primary development workstation runs Arch Linux
with a CachyOS LTO kernel; that validates the custom-kernel/DKMS combination, not
the complete CachyOS distribution.

CachyOS provides `chwd` for hardware profiles, including NVIDIA open-module
profiles. Follow the [chwd guide](https://wiki.cachyos.org/features/chwd/chwd/)
instead of mixing manually selected module families. CachyOS also ships
precompiled NVIDIA modules for its kernels; its
[kernel guide](https://wiki.cachyos.org/features/kernel/) warns that these can
replace equivalent DKMS modules.

```bash
chwd --list-all
nvctl driver info
nvctl driver dkms doctor
```

Keep exactly one module strategy for each installed kernel:

- CachyOS precompiled open module matching that kernel, or
- `nvidia-open-dkms` plus matching kernel headers.

Do not install both merely to make nvcontrol happy. nvcontrol needs a functioning,
version-aligned NVIDIA stack; it does not require a particular CachyOS kernel
scheduler or optimization profile.
