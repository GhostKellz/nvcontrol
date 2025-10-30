# Overclocking Commands

## Overview
GPU overclocking with manual control and automated wizards.

## Commands

### `nvctl overclock auto`
Automated overclocking wizard with safety features.

**Usage:**
```bash
nvctl overclock auto [OPTIONS]
```

**Options:**
- `--target <mode>` - Target mode: `max-performance`, `balanced`, `efficiency` (default: `balanced`)
- `--safety <mode>` - Safety mode: `conservative`, `moderate`, `aggressive` (default: `conservative`)
- `--max-temp <celsius>` - Maximum temperature limit (default: 85)
- `--max-power <percent>` - Maximum power limit % (default: 100)
- `--stability-duration <secs>` - Stability test duration (default: 60)

**Examples:**
```bash
# Safe balanced overclock
nvctl overclock auto

# Maximum performance, faster tuning
nvctl overclock auto --target max-performance --safety aggressive --max-temp 90

# Efficiency optimized
nvctl overclock auto --target efficiency --safety conservative
```

**Process:**
1. Baseline benchmark measurement
2. Stock stability verification
3. Iterative clock tuning (GPU then memory)
4. Final stability test with auto-rollback

**Duration:** 10-30 minutes depending on safety mode

---

### `nvctl overclock apply`
Apply manual overclock settings.

**Usage:**
```bash
nvctl overclock apply [OPTIONS]
```

**Options:**
- `--gpu-offset <mhz>` - GPU clock offset in MHz
- `--memory-offset <mhz>` - Memory clock offset in MHz
- `--power-limit <percent>` - Power limit percentage (50-120)

**Example:**
```bash
nvctl overclock apply --gpu-offset 150 --memory-offset 500 --power-limit 110
```

---

### `nvctl overclock profile <name>`
Load and apply saved overclock profile.

**Usage:**
```bash
nvctl overclock profile <name>
```

**Example:**
```bash
nvctl overclock profile gaming-max
```

---

### `nvctl overclock reset`
Reset GPU to stock settings.

**Usage:**
```bash
nvctl overclock reset
```

---

### `nvctl overclock info`
Display current overclock settings and memory timings.

**Usage:**
```bash
nvctl overclock info
```
