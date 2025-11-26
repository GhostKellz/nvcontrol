# nvcontrol TUI - Complete User Guide

**The Premier NVIDIA GPU Control Tool for Arch Linux + Wayland**

---

## Table of Contents
1. [Getting Started](#getting-started)
2. [Navigation](#navigation)
3. [Tabs Overview](#tabs-overview)
4. [Themes](#themes)
5. [Interactive Features](#interactive-features)
6. [Keybindings Reference](#keybindings-reference)
7. [Advanced Usage](#advanced-usage)
8. [RTX 5090 Specific](#rtx-5090-specific)

---

## Getting Started

### Launch the TUI
```bash
# Start live GPU monitoring dashboard
nvctl gpu stat

# Or use the full monitor command
nvctl monitor
```

### System Requirements
- **NVIDIA Open Kernel Modules 580+** (580.105.08 recommended)
- **RTX 50/40/30 Series GPU** (Blackwell/Ada/Ampere)
- **Arch Linux** with Wayland compositor (KDE/GNOME/Hyprland)
- **Nerd Font** for proper icon display (JetBrainsMono Nerd Font recommended)

### First Time Setup
The TUI starts with default safe settings:
- Theme: Tokyo Night (Night variant)
- GPU Offset: 0 MHz
- Memory Offset: 0 MHz
- Power Limit: 80%
- Fan Curve: Performance preset
- Update Interval: 1 second

---

## Navigation

### Tab Navigation
- **Tab/Shift+Tab** - Cycle through tabs
- **1-9** - Jump directly to tab number
- **←/→** - Previous/Next GPU (multi-GPU systems)
- **↑/↓** - Previous/Next GPU (alternative)

### General Controls
- **h** - Toggle help overlay
- **s** - Settings panel
- **Space/p** - Pause/Resume live updates
- **r** - Reset metrics history
- **t** - Cycle themes
- **q/Ctrl+C** - Quit

---

## Tabs Overview

### 1. 󰢮 Overview
**Live GPU Dashboard**

Displays at-a-glance GPU status with color-coded gauges:
- **GPU Utilization** - Green (low) → Yellow (med) → Red (high)
- **VRAM Usage** - Blue gauge with percentage
- **Temperature** - Color changes: Cyan (<50°C) → Green (50-70°C) → Yellow (70-80°C) → Red (>80°C)
- **Power Draw** - Optimized for RTX 5090's 600W TDP
- **GPU Information** - Model, memory, power limit
- **Mini Graphs** - Historical sparklines for quick trends

**Perfect for:** Quick status check, gaming sessions

---

### 2.  Performance
**Real-Time Performance Metrics**

**Left Panel - Current Stats:**
- GPU Utilization %
- Memory Utilization %
- Graphics Clock (MHz)
- Memory Clock (MHz)
- Power Usage (W)
- Temperature (°C)
- Fan Speed (%)

**Right Panel:**
- GPU Usage History (sparkline graph)
- 120 samples (2 minutes at 1Hz)

**Perfect for:** Benchmarking, stability testing

---

### 3.  Memory
**VRAM Analysis**

**Memory Statistics:**
- Total VRAM (GB)
- Used VRAM with percentage
- Free VRAM
- Memory Clock speed
- Active GPU processes (top 5)
- Per-process VRAM usage

**Visual Gauge:**
- Green: <70% usage
- Yellow: 70-90% usage
- Red: >90% usage (potential bottleneck)

**Perfect for:** ML workloads, game optimization, memory leak detection

---

### 4.  Temperature
**Thermal Monitoring**

**Current Temperatures:**
- All GPUs listed with color coding
- Selected GPU highlighted

**Temperature History:**
- Sparkline graph with max temperature tracking
- 2-minute rolling window

**Temperature Analysis:**
- Current temp with status (Excellent/Good/Warm/Hot/Critical)
- Thermal throttling indicator
- Critical threshold warning (95°C)
- Target threshold (83°C)
- Temperature guide with recommendations

**Thermal Zones:**
- < 60°C: Excellent (Cyan)
- 60-70°C: Good (Green)
- 70-80°C: Warm - normal under load (Yellow)
- 80-90°C: Hot - check cooling (Orange)
- \> 90°C: Critical - throttling may occur (Red)

**Perfect for:** Overclocking validation, cooling optimization

---

### 5. ⚡ Power
**Power Management**

**Current Power Usage:**
- Real-time wattage per GPU
- Percentage of power limit
- Color-coded efficiency indicators

**Power History:**
- Sparkline showing power draw trends
- Peak power tracking

**Power Analysis:**
- Current draw vs average (1 minute)
- Efficiency metric (utilization per watt)
- Power state classification:
  - Idle: < 50W
  - Light Load: 50-150W
  - Gaming: 150-250W
  - Heavy Compute: 250W+ (RTX 5090: up to 600W)

**Tips Section:**
- Power limit tuning advice
- Throttling detection
- Cooling correlation

**Perfect for:** Efficiency tuning, PSU planning

---

### 6.  Processes
**GPU Process Monitor**

**Active Processes:**
- Graphics processes (GFX)
- Compute processes (COMP)
- Per-process VRAM usage
- Process ID (PID)

**Process Summary:**
- Total GPU process count
- Note about permissions for process names

**Tip:** Use `nvidia-smi` for detailed process info including names

**Perfect for:** Debugging hangs, finding memory hogs

---

### 7. 󰥏 Overclocking
**Interactive Performance Tuning**

#### Current OC Status
Displays real-time clock speeds and limits:
- GPU Clock (current/max MHz)
- Memory Clock (current/max MHz)
- Power Limit (current/max W)
- Current OC settings (offsets and power %)

#### Interactive Controls
**Press 'o' to enter OC Mode**

**Live Visual Sliders:**
```
GPU Offset:    [-200] ──────●─────────── [+200] +150 MHz
Memory Offset: [-1000] ────────●──────── [+1000] +1000 MHz
Power Limit:   [50%] ──────────────●──── [100%] 95%
```

**Controls in OC Mode:**
- **←/→** - Adjust GPU offset (±10 MHz increments)
- **↑/↓** - Adjust Memory offset (±50 MHz increments)
- **+/-** - Adjust Power limit (±5% increments)
- **Enter** - Apply settings
- **o/Esc** - Exit OC mode

**Quick Presets (Press 1-4):**
1. **Stock** - 0/0/80% (safe defaults)
2. **Mild OC** - +75/+500/90% (conservative)
3. **Performance** - +150/+1000/95% (balanced)
4. **Extreme** - +200/+1500/100% (RTX 5090 max!)

#### RTX 5090 Specific Guidance
- **GDDR7 Memory:** Safe OC up to +1500 MHz
- **GPU Boost:** +150-200 MHz typical stable
- **Power:** 600W TDP (630W max for ASUS ROG Astral)
- **Cooling Required:** Ensure excellent airflow

⚠️ **CAUTION:** Overclocking may void warranty and cause instability!

**Perfect for:** Enthusiasts, competitive gaming, benchmarking

---

### 8. 󰈐 Fan Control
**Custom Fan Curves**

#### Current Fan Status
**Multi-Fan Display:**
- Fan 0-3: RPM estimates and percentages
- Temperature-aware icons (different colors based on GPU temp)
- Current GPU temperature

**ASUS ROG Astral Specific:**
- Displays all 4 fans independently
- Each fan can have custom curves (future)

#### Interactive Fan Curve Editor
**Press 'f' to enter Fan Mode**

**Curve Points (editable):**
```
►  30°C -> 20%
   50°C -> 40%
   70°C -> 60%
   80°C -> 80%
   90°C -> 100%
```

**Controls in Fan Mode:**
- **←/→** - Select curve point (► marker)
- **↑/↓** - Adjust fan % for selected point (±5%)
- **Enter** - Apply curve
- **f/Esc** - Exit fan mode

**Quick Presets:**
- **Silent** - Quiet operation, higher temps
- **Auto** - Default balanced curve
- **Performance** - Aggressive cooling (shown above)
- **Aggressive** - Max cooling, louder
- **0 RPM Mode** - Fans stop when cool (<30°C)

#### Features
- **Per-fan curves** (ASUS Astral quad-fan)
- **Visual curve editor** with ASCII graph
- **Real-time adjustment** with live feedback
- **Temperature-based automation**

**Perfect for:** Noise optimization, extreme overclocking, silent operation

---

### 9.  Profiles
**Performance Profile Management**

#### Built-in Profiles
**System Profiles:**
- **Silent** - 50% power, stock clocks, quiet fans
- **Balanced** - 80% power, default (active by default)
- **Performance** - 95% power, +150 GPU, +1000 MEM
- **Extreme** - 100% power, +200 GPU, +1500 MEM (RTX 5090)

**Game-Specific Profiles:**
- **Cyberpunk 2077** - DLSS 4 enabled, 4K Ultra preset
- **Counter-Strike 2** - Competitive mode, low latency focus
- **Stable Diffusion** - 90% power limit, memory OC for AI

#### Profile Actions
- **Enter** - Apply selected profile
- **n** - Create new profile from current settings
- **d** - Delete custom profile

#### Active Profile Details
Shows current configuration:
- GPU Offset (MHz)
- Memory Offset (MHz)
- Power Limit (%)
- Fan Curve preset
- Digital Vibrance setting

#### Features
- **Auto-apply per game** (detects running games)
- **Export/Import** profiles (JSON format)
- **Save current settings** as new profile
- **Quick switching** with hotkeys

**Perfect for:** Game optimization, workflow switching, preset management

---

## Themes

### Available Themes (Press 't' to cycle)

1. **Tokyo Night (Night)** - Darkest, deep blue bg (#1a1b26)
2. **Tokyo Night (Storm)** - Balanced blue (#24283b)
3. **Tokyo Night (Moon)** - Blue-tinted (#222436)
4. **Dracula** - Purple/pink dark (#282a36)
5. **ROG Red** - ASUS gaming theme with red accents
6. **Matrix Green** - Classic green on black terminal
7. **Cyberpunk** - Pink/cyan neon

### Theme Features
- **Semantic Colors** - Temp/usage/power aware
- **Consistent Across Tabs** - Unified look
- **Nerd Font Icons** - Beautiful icons throughout
- **Performance Optimized** - Instant theme switching

### Color-Coded Metrics
All themes adapt to metrics:
- **Temperature:** Cyan → Green → Yellow → Red
- **Usage:** Green (low) → Yellow (med) → Red (high)
- **Power:** Teal (efficient) → Blue (normal) → Orange (high)

---

## Interactive Features

### Overclocking Mode
**Activation:** Press 'o' on the Overclocking tab

**What it does:**
- Enables arrow key controls for OC adjustments
- Shows live slider positions
- Status bar displays current values
- Allows preset quick-switching (1-4)

**Safety:**
- Stays within safe ranges (-200 to +200 GPU, -1000 to +1000 MEM)
- Visual feedback on all changes
- Settings saved but not applied until Enter

**Exit:** Press 'o' or Esc

---

### Fan Curve Mode
**Activation:** Press 'f' on the Fan Control tab

**What it does:**
- Enables curve point editing
- Shows ► marker on selected point
- Live percentage adjustments
- Immediate visual feedback

**Usage:**
1. Select point with ←/→
2. Adjust fan % with ↑/↓
3. See changes immediately
4. Press Enter to apply

**Exit:** Press 'f' or Esc

---

### VRR Toggle
**Activation:** Press 'v' anywhere

**What it does:**
- Toggles Variable Refresh Rate (G-Sync/FreeSync)
- Works per-display
- Shows status in status bar
- Instant application

**Status:** Green when enabled, gray when disabled

---

### Gaming Mode
**Activation:** Press 'g' anywhere

**What it does:**
- Applies latency optimizations
- Sets performance fan profile
- Optimizes frame pacing
- Shows status in status bar

**Status:** Shows gaming icon when active

---

## Keybindings Reference

### Essential
| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit TUI |
| `Ctrl+C` | Force quit |
| `h` / `F1` | Toggle help |
| `s` | Settings panel |
| `Space` / `p` | Pause/Resume updates |
| `r` | Reset metrics history |
| `t` | Cycle themes |

### Navigation
| Key | Action |
|-----|--------|
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |
| `1-9` | Jump to tab (1=Overview, 9=Profiles) |
| `←` / `→` | Previous/Next GPU |
| `↑` / `↓` | Previous/Next GPU (alt) |

### Features
| Key | Action |
|-----|--------|
| `v` | Toggle VRR/G-Sync |
| `g` | Toggle Gaming Mode |
| `o` | Toggle Overclocking Mode (tab 7) |
| `f` | Toggle Fan Control Mode (tab 8) |
| `e` | Export metrics (planned) |

### OC Mode (Tab 7, press 'o')
| Key | Action |
|-----|--------|
| `←` / `→` | GPU offset ±10 MHz |
| `↑` / `↓` | Memory offset ±50 MHz |
| `+` / `-` | Power limit ±5% |
| `1` | Stock preset |
| `2` | Mild OC preset |
| `3` | Performance preset |
| `4` | Extreme preset |
| `Enter` | Apply settings |
| `o` / `Esc` | Exit OC mode |

### Fan Mode (Tab 8, press 'f')
| Key | Action |
|-----|--------|
| `←` / `→` | Select curve point |
| `↑` / `↓` | Adjust fan % ±5% |
| `Enter` | Apply curve |
| `f` / `Esc` | Exit fan mode |

---

## Advanced Usage

### Multi-GPU Setup
When multiple GPUs are detected:
- Use `←`/`→` to switch between GPUs
- Each GPU has independent metrics
- OC settings per GPU
- Fan curves per GPU
- Status bar shows: "GPU 2/3"

### Metrics Export
Press 'e' to export current metrics to JSON:
```json
{
  "gpu_0": [
    {
      "temperature": 72.5,
      "gpu_utilization": 95.2,
      "memory_utilization": 85.0,
      "power_draw": 380.5,
      ...
    }
  ]
}
```

**Filename:** `nvcontrol_metrics_YYYYMMDD_HHMMSS.json`

### Stability Testing
1. Navigate to **Overclocking** tab
2. Enter OC mode ('o')
3. Apply conservative settings (+50 GPU, +250 MEM)
4. Switch to **Temperature** tab
5. Launch stress test externally
6. Monitor temps - should stay <85°C
7. If stable for 30 min, increase gradually

### Optimal Fan Curve Creation
1. Navigate to **Fan Control** tab
2. Enter fan mode ('f')
3. Start with aggressive curve (high fan %)
4. Run normal workload
5. Gradually reduce fan % at each temp point
6. Find lowest noise with acceptable temps
7. Apply with Enter

---

## RTX 5090 Specific

### Recommended Settings

#### For Gaming (Silent)
```
Profile: Balanced
GPU Offset: +75 MHz
Memory Offset: +500 MHz
Power Limit: 85%
Fan Curve: Silent (0 RPM mode enabled)
Expected: 65-75°C, near-silent
```

#### For Competitive Gaming (Performance)
```
Profile: Performance
GPU Offset: +150 MHz
Memory Offset: +1000 MHz
Power Limit: 95%
Fan Curve: Performance
Expected: 70-80°C, moderate noise
```

#### For Benchmarking (Extreme)
```
Profile: Extreme
GPU Offset: +200 MHz
Memory Offset: +1500 MHz (GDDR7 safe)
Power Limit: 100% (630W on ASUS Astral)
Fan Curve: Aggressive
Expected: 80-85°C, loud but stable
```

### ASUS ROG Astral Features
- **Quad-Fan Cooling** - All 4 fans display in Fan Control tab
- **Factory OC** - Base boost 2610 MHz (vs 2580 reference)
- **630W Max Power** - Higher than reference 600W
- **0 RPM Mode** - Fans stop completely below 30°C

### GDDR7 Memory Notes
- Much higher OC headroom than GDDR6X
- +1500 MHz is conservative and safe
- +2000 MHz possible with excellent cooling
- Memory temps auto-managed

### Blackwell Architecture Benefits
- Improved power efficiency at stock
- Better thermal characteristics
- DLSS 4 multi-frame generation ready
- PCIe Gen 5.0 (backward compatible)

---

## Troubleshooting

### Icons Not Displaying
**Problem:** Boxes instead of icons
**Solution:** Install a Nerd Font:
```bash
# Arch Linux
yay -S ttf-jetbrains-mono-nerd

# Then set your terminal to use JetBrainsMono Nerd Font
```

### OC Settings Not Applying
**Problem:** Overclock doesn't take effect
**Solution:**
- Ensure nvidia-settings is not conflicting
- Check kernel module parameters
- Reboot may be required
- Verify open drivers 580+

### Fan Control Not Working
**Problem:** Fan speeds don't change
**Solution:**
- Some GPUs require manual control enable
- Check ASUS Armoury Crate isn't loaded
- Linux kernel 6.17+ recommended
- May need `nvidia-settings -a GPUFanControlState=1`

### Temperature Reading Incorrect
**Problem:** Shows 0°C or very high temps
**Solution:**
- Wait a few seconds for NVML initialization
- Check `nvidia-smi` shows temps
- Ensure GPU is not in sleep mode
- Update to latest drivers

### Paused on Startup
**Problem:** TUI shows PAUSED immediately
**Solution:** Press Space or 'p' to resume

---

## Tips & Best Practices

### Daily Use
- Start TUI in background: `nvctl gpu stat &`
- Use theme matching your terminal
- Set conservative OC for daily driver
- Enable 0 RPM mode for silence

### Benchmarking
1. Reset metrics ('r') before test
2. Apply performance profile
3. Pause ('p') after benchmark
4. Export metrics ('e')
5. Compare JSON files

### Development (CUDA/ML)
- Monitor Memory tab for VRAM usage
- Use Processes tab to find memory leaks
- Balanced profile for sustained workloads
- Watch power tab for throttling

### Gaming Sessions
- Enable Gaming Mode ('g')
- Enable VRR ('v')
- Monitor Overview tab during gameplay
- Use Performance profile

---

## Integration with nvctl CLI

The TUI complements the CLI:

```bash
# Apply settings from CLI, monitor in TUI
nvctl overclock apply --gpu-offset 150 --memory-offset 1000

# Launch TUI to see results
nvctl gpu stat

# Toggle VRR from CLI or TUI
nvctl vrr enable

# Fan curves work together
nvctl fan curve apply gaming
```

---

## Future Features (Planned)

- [ ] DLSS/FSR toggle tab
- [ ] CUDA toolkit info tab
- [ ] Container management tab
- [ ] Display/HDR management tab
- [ ] Per-game auto-profiles
- [ ] RGB lighting control (ASUS Aura)
- [ ] Voltage curve editing
- [ ] Custom dashboard layouts
- [ ] Remote monitoring (network)
- [ ] Alert/notification system

---

## Support & Feedback

**GitHub:** https://github.com/ghostkellz/nvcontrol
**Issues:** Report bugs and request features
**Docs:** Check README.md and COMMANDS.md

---

**Made with ❤️ for the Arch Linux + NVIDIA + Wayland community**

*Last Updated: 2025-11-23 - RTX 5090 Ready!*
