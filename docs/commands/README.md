# nvctl Command Reference

Complete command-line reference for nvcontrol.

## Quick Links

- [GPU Commands](gpu.md) - GPU info, monitoring, multi-GPU
- [Overclocking](overclock.md) - Manual and automated overclocking
- [Power Management](power.md) - Power curves, limits, schedules
- [Gaming](gaming.md) - Game profiles, auto-application, latency
- [Configuration](config.md) - Settings, profiles, import/export
- [Containers](container.md) - Docker GPU management

## Command Categories

### GPU Management
```bash
nvctl gpu info              # GPU information
nvctl gpu list              # List all GPUs
nvctl gpu select <idx>      # Select active GPU
nvctl gpu stat              # Live monitoring TUI
nvctl gpu benchmark         # Performance benchmark
```

### Overclocking
```bash
nvctl overclock auto        # Automated wizard
nvctl overclock apply       # Manual overclock
nvctl overclock profile     # Load saved profile
nvctl overclock reset       # Reset to stock
```

### Power Management
```bash
nvctl power status          # Current power stats
nvctl power limit <pct>     # Set power limit
nvctl power curve show      # Display curve
nvctl power curve add       # Add curve point
nvctl power curve enable    # Enable curve mode
```

### Gaming
```bash
nvctl gaming auto status    # Auto-profile status
nvctl gaming auto enable    # Enable auto-apply
nvctl gaming auto config    # Configure settings
nvctl gaming launch run     # Launch game with profile
nvctl gaming latency        # Latency optimizations
nvctl gaming gamescope      # Gamescope compositor
```

### Configuration
```bash
nvctl config show           # Show configuration
nvctl config backup         # Backup settings
nvctl config restore        # Restore from backup
nvctl config export         # Export profile
nvctl config import         # Import profile
nvctl config profiles       # List profiles
```

### Containers
```bash
nvctl nvbind list           # List GPU containers
nvctl container list        # Docker containers
```

### Display & Vibrance
```bash
nvctl vibrance <pct>        # Set vibrance
nvctl display info          # Display information
nvctl display vrr           # VRR/G-Sync control
```

### Fan Control
```bash
nvctl fan info              # Fan information
nvctl fan set <id> <pct>    # Set fan speed
```

### Diagnostics
```bash
nvctl doctor                # System diagnostics & health check
nvctl monitor               # System monitoring
nvctl benchmark             # GPU benchmarks
```

### Drivers
```bash
nvctl drivers status        # Driver information
nvctl driver info           # Driver capabilities & requirements
nvctl driver validate --driver 590  # Validate target branch
```

### Other
```bash
nvctl osd enable            # Enable OSD overlay
```

## Global Options

All commands support:
- `--help` - Show command help
- `--format` - Output format where applicable (json, table, human)
- `--no-color` - Disable colored output

## Examples

### Complete Workflow

**1. Check GPU status:**
```bash
nvctl gpu info
```

**2. Run auto-overclock:**
```bash
nvctl overclock auto --target balanced --safety conservative
```

**3. Set up power curve:**
```bash
nvctl power curve add 70 90
nvctl power curve add 80 80
nvctl power curve enable
```

**4. Enable game profile auto-application:**
```bash
nvctl gaming auto config --poll-interval 2 --apply-delay 3
nvctl gaming auto enable
```

**5. Export your tuned profile:**
```bash
nvctl config export --profile my-gaming-profile --output ~/gaming.toml
```

### Multi-GPU Setup

```bash
# List all GPUs
nvctl gpu list

# Select GPU 1
nvctl gpu select 1

# Apply overclock to selected GPU
nvctl overclock apply --gpu-offset 100 --memory-offset 400
```

### Container Gaming

```bash
# List GPU containers
nvctl nvbind list --gpu-only --metrics

# Launch game with GPU optimization
nvctl gaming gamescope launch --preset competitive steam
```

## Configuration Files

Location: `~/.config/nvcontrol/`

- `config.toml` - Main configuration
- `tui_state.toml` - TUI session persistence (v0.7.6+)
- `power_management.toml` - Power curves
- `game_profile_auto.toml` - Game auto-profiles
- `profiles/` - Saved profiles directory

See [Configuration Guide](../config/README.md) for details.

## Getting Help

```bash
nvctl --help                    # General help
nvctl <command> --help          # Command-specific help
nvctl <command> <sub> --help    # Subcommand help
```

## Version

```bash
nvctl version                   # Show version info
```
