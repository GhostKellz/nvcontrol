# Migration Guide

## Upgrading to v0.7.6

v0.7.6 introduces session persistence and backend abstraction. Most users won't need to do anything - the upgrade is automatic.

### Automatic Migration

When you first run v0.7.6:

1. **Config files**: Existing `~/.config/nvcontrol/config.toml` is preserved
2. **New state file**: `tui_state.toml` is created on first TUI exit
3. **Value validation**: Any out-of-range values are clamped to safe defaults

### Breaking Changes

**None** - v0.7.6 is fully backwards compatible.

### New Files

| File | Purpose |
|------|---------|
| `~/.config/nvcontrol/tui_state.toml` | TUI session state (new) |

### If You Have Issues

1. **TUI won't start**: Delete state file
   ```bash
   rm ~/.config/nvcontrol/tui_state.toml
   ```

2. **Corrupt backup exists**: Check for `.bak` file
   ```bash
   ls ~/.config/nvcontrol/*.bak
   ```

3. **Full reset**:
   ```bash
   rm -rf ~/.config/nvcontrol/
   ```

## Version History

| Version | Config Changes |
|---------|----------------|
| 0.7.6 | Added `tui_state.toml` with version tracking |
| 0.7.5 | No config changes |
| 0.7.0 | Initial `config.toml` format |

## Future Migrations

The `version` field in `tui_state.toml` enables future migrations:

```toml
version = 1  # Current schema version
```

When future versions change the schema, nvcontrol will automatically migrate your settings.
