# VRR / G-SYNC Control

nvcontrol reads and changes compositor variable-refresh policy where the active
desktop exposes a supported command. It deliberately separates observed state
from assumptions about the monitor.

## Commands

```bash
nvctl vrr status
nvctl vrr enable DP-1
nvctl vrr disable DP-1
nvctl vrr configure DP-1 --min-refresh 48 --max-refresh 240
```

`status` can report:

- the compositor's current enabled/disabled policy;
- VRR capability when the compositor explicitly exposes it;
- the highest advertised display mode;
- G-SYNC, FreeSync, and VRR range only when a backend actually reports them.

DisplayPort, a high refresh mode, or an enabled policy is not proof of G-SYNC
certification, FreeSync support, Low Framerate Compensation, or a 48 Hz minimum.
Unknown values are shown as `not reported`.

The `configure` minimum and maximum are nvcontrol settings; current compositor
helpers generally toggle adaptive-sync policy rather than rewriting a monitor's
EDID range. There are no `--adaptive-sync` or `--lfc` flags on this command.

## Compositor routes

| Desktop | Query/apply route | Notes |
|---|---|---|
| KDE Plasma | `kscreen-doctor` | Policy 0 never, 1 always, 2 automatic |
| GNOME | Mutter experimental feature | Session-wide toggle; per-output facts may remain unreported |
| Hyprland | `hyprctl` | Uses compositor monitor state |
| Sway | `swaymsg` | Uses output adaptive-sync control |
| COSMIC | `cosmic-randr` through `nvctl monitors set-vrr` | Preserves current mode and changes adaptive-sync only |
| X11 | `xrandr`/`nvidia-settings` fallback | Legacy session path |

### KDE

```bash
kscreen-doctor -j
nvctl vrr enable DP-2
```

KDE's automatic policy enables VRR when compositor conditions are met; it does
not mean VRR is active for every frame at the instant status is queried.

### COSMIC

Use the multi-monitor command for the tested COSMIC path:

```bash
cosmic-randr
nvctl monitors set-vrr DP-2 --enabled
```

The controller parses the current COSMIC resolution and refresh rate and repeats
them when applying `--adaptive-sync automatic`. Calling the same command without
`--enabled` selects the disabled state. v0.8.12 validated that a no-change apply
keeps the active mode intact on Pop!_OS 24.04.

## Troubleshooting

1. Run `nvctl vrr status` and the compositor's native query command.
2. Confirm the connector name from the active session; names vary by compositor
   and passthrough guests do not describe host monitor ownership.
3. Treat `not reported` as missing evidence, not as unsupported hardware.
4. If VRR causes flicker or TTY-switch crashes, disable it through nvcontrol or
   the compositor before changing driver/module settings.

## Related documentation

- [Display API](../api/display.md)
- [Pop!_OS COSMIC](../distros/popos-cosmic.md)
- [NVIDIA driver compatibility](../drivers/nvidia-driver.md)
