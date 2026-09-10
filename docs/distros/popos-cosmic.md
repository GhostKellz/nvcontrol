# Pop!_OS COSMIC

The current release was tested on Pop!_OS 24.04 with COSMIC and an RTX 3070
passthrough GPU, first on open 595 and then on matching open 615 modules and
multilib userspace. The native Debian package, GUI/TUI, vibrance, and distro
DKMS diagnostics passed. The newer guest kernel also has its NVIDIA module
installed automatically. See the [release evidence](../advisories/v0.8.13-release-notes.md)
for exact versions and results.

System76 provides an NVIDIA install image and the `system76-driver-nvidia`
package; see the official [installation guide](https://support.system76.com/support/articles/install-pop/).
On hybrid laptops, COSMIC normally uses hybrid graphics and per-application
offload as described by [System76 graphics switching](https://support.system76.com/support/articles/graphics-switch-pop/).

## COSMIC display behavior

`nvctl monitors set-vrr` uses `cosmic-randr`, preserves the active resolution and
refresh rate, strips terminal formatting from command output, and changes only
adaptive-sync policy. COSMIC output names are guest/session facts; they do not
prove which physical monitor is connected to a passthrough host.

```bash
cosmic-randr
nvctl monitors status
nvctl monitors set-vrr DP-2 --enabled
```

The enabled Pop repositories used by the test guest did not provide Gamescope,
so it was not installed from a foreign repository. MangoHud, GameMode, Vulkan
tools, Wayland tools, and the official GeForce NOW Flatpak were validated.
