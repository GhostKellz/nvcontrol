# Overclocking Guide

Comprehensive GPU overclocking guide for nvcontrol.

## Overview

nvcontrol provides safe, validated GPU overclocking with:

- **GPU clock offset** - Increase core clock speed
- **Memory clock offset** - Increase VRAM bandwidth
- **Power limit control** - Adjust TDP for headroom
- **Automated wizard** - Safe auto-tuning
- **Stress testing** - Stability validation

> **Note:** Overclocking is available via **CLI** (`nvctl overclock`) and **TUI** (`nvctl gpu stat`).
> The GUI application does not include an overclock tab because overclocking requires X11 display server access, which is not available on Wayland. For Wayland users, use the CLI commands documented below.

## Quick Start

```bash
# View current clocks and capabilities
nvctl overclock info

# Apply a modest overclock
nvctl overclock apply --gpu-offset 100 --memory-offset 500

# Use automated wizard
nvctl overclock auto

# Reset to stock
nvctl overclock reset
```

## Commands

### `nvctl overclock info`

Display comprehensive overclocking information.

**Output includes:**
- Current clock speeds (base/boost/effective)
- Memory timings and bandwidth
- Power consumption and limits
- Temperature readings
- Voltage information (if available)

**Example output:**
```
GPU Overclock Status:
══════════════════════════════════════════════════════════════
  Model: NVIDIA GeForce RTX 5090
  Architecture: Blackwell

  Clocks:
    Base Clock: 2230 MHz
    Boost Clock: 2610 MHz
    Current: 2580 MHz
    Memory: 28000 MHz (GDDR7)

  Offsets:
    GPU Offset: +150 MHz
    Memory Offset: +500 MHz

  Power:
    Current: 425W
    Limit: 450W (Min: 300W, Max: 630W)

  Thermals:
    Temperature: 62°C
    Throttle: None
══════════════════════════════════════════════════════════════
```

### `nvctl overclock apply [OPTIONS]`

Apply manual overclock settings with safety validation.

**Options:**
- `--gpu-offset <mhz>` - GPU clock offset (-200 to +300 MHz typical)
- `--memory-offset <mhz>` - Memory clock offset (-500 to +1500 MHz typical)
- `--power-limit <percent>` - Power limit percentage (50-120% typical)

**Examples:**
```bash
# Modest gaming overclock
nvctl overclock apply --gpu-offset 100 --memory-offset 500

# Aggressive performance
nvctl overclock apply --gpu-offset 200 --memory-offset 1000 --power-limit 115

# Power-limited (quieter operation)
nvctl overclock apply --gpu-offset 50 --power-limit 85
```

### `nvctl overclock auto [OPTIONS]`

Automated overclocking wizard with safety features.

**Options:**
- `--target <mode>` - Target: `max-performance`, `balanced`, `efficiency`
- `--safety <mode>` - Safety: `conservative`, `moderate`, `aggressive`
- `--max-temp <celsius>` - Temperature limit (default: 85°C)
- `--max-power <percent>` - Power limit (default: 100%)
- `--stability-duration <secs>` - Test duration (default: 60s)

**Examples:**
```bash
# Safe balanced overclock (recommended)
nvctl overclock auto

# Maximum performance, aggressive tuning
nvctl overclock auto --target max-performance --safety aggressive --max-temp 90

# Efficiency optimized (lower power, good clocks)
nvctl overclock auto --target efficiency --safety conservative
```

**Process:**
1. Baseline benchmark measurement
2. Stock stability verification
3. Iterative GPU clock tuning
4. Memory clock tuning
5. Final stability test
6. Auto-rollback on failure

**Duration:** 10-30 minutes depending on safety mode

### `nvctl overclock profile <name>`

Load and apply saved overclock profile.

```bash
nvctl overclock profile gaming-max
nvctl overclock profile quiet
nvctl overclock profile balanced
```

**Built-in Profiles:**
- `gaming` - +150 GPU, +800 memory, 110% power
- `quiet` - +50 GPU, +200 memory, 80% power
- `balanced` - +100 GPU, +500 memory, 100% power
- `extreme` - +200 GPU, +1200 memory, 120% power (use with caution)

### `nvctl overclock stress-test [duration]`

Run comprehensive GPU stress test with monitoring.

**Parameters:**
- `duration` - Test duration in minutes (default: 5, max: 60)

**Features:**
- Real-time temperature monitoring
- Automatic safety shutdown on overheat
- Stability validation with error detection
- Performance metrics reporting

```bash
# Quick stability check
nvctl overclock stress-test 5

# Extended validation
nvctl overclock stress-test 15

# Overnight stress test
nvctl overclock stress-test 60
```

### `nvctl overclock reset`

Safely reset all overclocking settings to hardware defaults.

```bash
nvctl overclock reset
```

**Actions:**
- Clears GPU offset
- Clears memory offset
- Resets power limit to 100%
- Restores default fan behavior

## GPU Capabilities

### `nvctl gpu capabilities`

View detailed overclocking capabilities and safe limits.

```bash
nvctl gpu capabilities
```

**Output includes:**
- Maximum safe GPU clock offset
- Memory overclocking headroom
- Power limit range
- Temperature thresholds
- Voltage modification support (if available)

## Safety Features

nvcontrol implements multiple safety layers:

### Temperature Protection

- **Soft limit:** Warning at 80°C
- **Hard limit:** Auto-throttle at 85°C
- **Emergency:** Auto-reset at 90°C+

### Power Protection

- Validates against hardware limits
- Prevents exceeding PSU capacity
- Monitors power connector status (ASUS ROG)

### Stability Validation

- Pre-apply validation of settings
- Automatic rollback on crash/hang
- Stress test verification

## Architecture-Specific Guidance

### Blackwell (RTX 50xx)

```bash
# RTX 5090 typical headroom
nvctl overclock apply --gpu-offset 150 --memory-offset 600 --power-limit 110
```

- Excellent power efficiency
- GDDR7 overclocks well
- Monitor 8-pin power connectors

### Ada Lovelace (RTX 40xx)

```bash
# RTX 4090 typical headroom
nvctl overclock apply --gpu-offset 200 --memory-offset 1000 --power-limit 115
```

- Great overclocking potential
- GDDR6X memory sensitive to heat
- Good thermal headroom

### Ampere (RTX 30xx)

```bash
# RTX 3080 typical headroom
nvctl overclock apply --gpu-offset 150 --memory-offset 800 --power-limit 110
```

- Memory benefits from good airflow
- Power limit is often the limitation
- GDDR6X runs hot

### Turing (RTX 20xx)

```bash
# RTX 2080 typical headroom
nvctl overclock apply --gpu-offset 100 --memory-offset 500 --power-limit 105
```

- Mature architecture, well understood
- Conservative is usually better
- Good stability at modest clocks

## Profile Configuration

Save custom profiles in `~/.config/nvcontrol/profiles/`:

```toml
# ~/.config/nvcontrol/profiles/my-gaming.toml
[profile]
name = "My Gaming"
description = "Personal gaming overclock"

[gpu]
overclock_gpu = 175
overclock_memory = 750

[power]
limit_percent = 112
mode = "MaxPerformance"

[fan]
preset = "aggressive"
min_speed = 40

[safety]
max_temp = 83
```

Apply with:
```bash
nvctl profile apply my-gaming
```

## Power Limit Tuning

### Understanding Power Limits

| Limit Type | Description |
|------------|-------------|
| Default | Factory power limit |
| Min | Minimum allowed (eco mode) |
| Max | Maximum allowed (overclock) |

### Setting Power Limits

```bash
# Set to 90% (power saving)
nvctl power limit 90

# Set to 115% (more headroom)
nvctl power limit 115

# Check current limits
nvctl power info
```

### Power vs Performance

Higher power = more sustained boost clocks:

```
Power Limit    Expected Boost
    80%        Base clock + ~100MHz
   100%        Rated boost clock
   115%        Boost + 100-200MHz
   120%        Maximum sustained
```

## Memory Overclocking

### GDDR Memory Types

| Type | Typical Headroom | Notes |
|------|------------------|-------|
| GDDR7 | +500-1000 MHz | New, high bandwidth |
| GDDR6X | +200-800 MHz | Runs hot, needs cooling |
| GDDR6 | +500-1200 MHz | Stable, good headroom |

### Finding Memory Limit

```bash
# Start conservative
nvctl overclock apply --memory-offset 200

# Test stability
nvctl overclock stress-test 5

# Increase gradually (+100 at a time)
nvctl overclock apply --memory-offset 300
nvctl overclock stress-test 5

# Repeat until artifacts appear, then back off 100MHz
```

### Memory Error Detection

- Artifacts/flickering = memory unstable
- Black screen = too aggressive
- Crashes = reduce offset by 100-200MHz

## Monitoring During Overclock

### Live Stats

```bash
# TUI monitoring dashboard
nvctl gpu stat
```

Press `q` to quit, shows:
- Temperature (color-coded)
- Fan speed
- Clock speeds
- Power consumption
- GPU utilization

### Logging

```bash
# Log performance data
nvctl gpu stat > performance.log
```

## Troubleshooting

### Overclock Not Applying

1. **Check permissions:**
   ```bash
   # May need root for sysfs access
   sudo nvctl overclock apply --gpu-offset 100
   ```

2. **Verify support:**
   ```bash
   nvctl gpu capabilities
   ```

3. **Check coolbit settings:**
   - Ensure nvidia-settings/xorg allows OC
   - Wayland: NVKMS provides direct access

### System Instability

```bash
# Immediately reset to stock
nvctl overclock reset

# Or if GUI frozen, via TTY:
sudo nvidia-smi -rgc  # Reset GPU clocks
sudo nvidia-smi -rmc  # Reset memory clocks
```

### Not Reaching Boost Clocks

Common causes:
1. **Thermal throttling** - Improve cooling
2. **Power throttling** - Increase power limit
3. **VRM throttling** - Card-specific limit

Check throttle reason:
```bash
nvctl gpu info | grep throttle
```

### Overclock Lost After Reboot

```bash
# Apply profile at startup
nvctl profile apply gaming --autostart

# Or add to startup script
echo "nvctl overclock apply --gpu-offset 100 --memory-offset 500" >> ~/.config/autostart-scripts/nvcontrol.sh
```

## Best Practices

1. **Start conservative** - Begin with low offsets
2. **Test stability** - Always stress test new settings
3. **Monitor temps** - Keep under 80°C for longevity
4. **Incremental changes** - +50MHz steps for GPU, +100MHz for memory
5. **Backup profiles** - Save working configs
6. **Know your limits** - Each card's silicon is different

## Related Documentation

- [Power Management](commands/power.md) - Power limit control
- [Fan Control](api/FAN.md) - Cooling configuration
- [GPU Commands](commands/gpu.md) - Monitoring
- [VRR/G-SYNC](VRR_GSYNC.md) - Display optimization

---

**Last Updated**: December 2024 (v0.7.3)
