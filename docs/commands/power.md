# Power Management Commands

## Overview
Advanced power management with curves, schedules, and automation.

## Commands

### `nvctl power status`
Show current power settings and statistics.

**Usage:**
```bash
nvctl power status
```

**Output:**
- Current power draw
- Power limit
- Temperature
- Fan speed
- Persistence mode

---

### `nvctl power limit <percentage>`
Set GPU power limit.

**Usage:**
```bash
nvctl power limit <percentage>
```

**Arguments:**
- `<percentage>` - Power limit percentage (50-120)

**Example:**
```bash
nvctl power limit 90
```

---

### `nvctl power curve show`
Display current temperature-based power curve.

**Usage:**
```bash
nvctl power curve show
```

**Output:**
- Curve enabled status
- All curve points (Temperature → Power Limit)

---

### `nvctl power curve add <temp> <power>`
Add a point to the power curve.

**Usage:**
```bash
nvctl power curve add <temp> <power>
```

**Arguments:**
- `<temp>` - Temperature in Celsius
- `<power>` - Power limit percentage

**Example:**
```bash
nvctl power curve add 75 85
```

**Result:** Automatically sorts points by temperature

---

### `nvctl power curve remove <index>`
Remove a curve point by index.

**Usage:**
```bash
nvctl power curve remove <index>
```

**Example:**
```bash
nvctl power curve remove 2
```

---

### `nvctl power curve enable`
Enable temperature-based power curve.

**Usage:**
```bash
nvctl power curve enable
```

---

### `nvctl power curve disable`
Disable temperature-based power curve.

**Usage:**
```bash
nvctl power curve disable
```

---

### `nvctl power curve reset`
Reset power curve to defaults.

**Usage:**
```bash
nvctl power curve reset
```

**Default curve:**
- 40°C → 100% power
- 60°C → 90% power
- 75°C → 80% power
- 85°C → 70% power

---

### `nvctl power schedule`
Time-based power profile scheduling (coming soon).

**Usage:**
```bash
nvctl power schedule <action>
```

---

### `nvctl power monitor <duration>`
Monitor power consumption over time.

**Usage:**
```bash
nvctl power monitor --duration <seconds>
```

**Example:**
```bash
nvctl power monitor --duration 300
```
