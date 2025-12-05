# nvcontrol Configuration

This directory documents nvcontrol's configuration system, session persistence, and backend architecture.

## Configuration Files

nvcontrol stores configuration in `~/.config/nvcontrol/`:

| File | Purpose |
|------|---------|
| `config.toml` | User preferences (theme, vibrance, HDR, OSD) |
| `tui_state.toml` | TUI session persistence (GPU, tab, fan curves, OC) |

## Documentation

- [Session Persistence](./SESSION_PERSISTENCE.md) - TUI state saving/loading
- [Backend Architecture](./BACKEND_ARCHITECTURE.md) - NVML/Display abstraction layer
- [Migration Guide](./MIGRATION.md) - Upgrading from previous versions

## Quick Reference

### Config Locations

```bash
# View config directory
ls ~/.config/nvcontrol/

# Reset to defaults
rm ~/.config/nvcontrol/config.toml
rm ~/.config/nvcontrol/tui_state.toml
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `XDG_CONFIG_HOME` | Override config directory base |
| `NVCONTROL_LOG` | Set log level (error, warn, info, debug, trace) |

## See Also

- [COMMANDS.md](../COMMANDS.md) - CLI command reference
- [TUI_USER_GUIDE.md](../TUI_USER_GUIDE.md) - TUI usage guide
