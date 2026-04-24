# Companion Flow

nvcontrol no longer ships a GTK3-backed tray icon. The lightweight desktop companion flow keeps the desktop integration surface small and lower maintenance.

## Commands

```bash
# Send a desktop notification test
nvctl companion notify-test

# Open project docs with the desktop handler
nvctl companion open-docs
```

## Purpose

The companion flow is intended for:

- confirming desktop notification support
- simple desktop launch/hand-off actions
- avoiding tray-specific GTK/libappindicator maintenance risk

## Future Direction

This path is a safer base for later desktop helper work such as:

- alert-only background helpers
- release diagnostic notifications
- launcher integration without reintroducing deprecated tray stacks
