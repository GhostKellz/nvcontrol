# nvcontrol Configuration

This directory documents nvcontrol's configuration system, session persistence, and backend architecture.

## Configuration Files

nvcontrol stores configuration in `~/.config/nvcontrol/`:

| File | Purpose |
|------|---------|
| `config.toml` | User preferences (theme, vibrance, HDR, OSD) |
| `tui_state.toml` | TUI session persistence (GPU, tab, fan curves, OC) |

## Documentation

- [Session Persistence](session-persistence.md) - TUI state saving/loading
- [Backend Architecture](backend-architecture.md) - NVML/Display abstraction layer
- [Migration Guide](migration.md) - Upgrading from previous versions

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

- [commands.md](../commands.md) - CLI command reference
- [tui-user-guide.md](../tui-user-guide.md) - TUI usage guide
