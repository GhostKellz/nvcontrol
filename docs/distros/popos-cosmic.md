# Pop!_OS COSMIC

v0.8.12 was validated on Pop!_OS 24.04 with COSMIC, an RTX 3070 passthrough GPU,
and the open 595.84 driver. This is the regression target proving one nvcontrol
binary can select the NVIDIA 595 or 610 NVKMS allocation ABI at runtime.

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
