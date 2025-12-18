# ASUS ROG Astral RTX 5090 - Linux Quick Start

Welcome, Astral owner! This guide covers nvcontrol features specific to your card.

## Power Detector+

Your Astral has ASUS Power Detector+ for monitoring the 12V-2x6 power connector health.

### Quick Check

```bash
nvctl asus power
```

Output shows:
- **Connector Health**: GOOD / WARNING / CRITICAL
- **Per-rail current**: Each of the 6 power rails
- **Total connector power**: Estimated watts through connector

### What the Health Status Means

| Status | Action |
|--------|--------|
| **GOOD** (Green) | All normal, no action needed |
| **WARNING** (Yellow) | Monitor closely under load, check connector seating |
| **CRITICAL** (Red) | Stop heavy workloads, reseat connector, check cable quality |

### Best Practices

1. **First Boot**: Run `nvctl asus power` to establish baseline readings
2. **After Moving PC**: Check connector health after any physical changes
3. **Under Load**: Monitor during sustained GPU workloads
4. **Cable Quality**: Use only high-quality 12V-2x6 cables rated for 600W+

## TUI Dashboard

```bash
nvctl tui
```

Navigate to the **Power** tab to see:
- Real-time power draw
- Power history graph
- ASUS Power Detector+ panel (bottom)

## All ASUS Commands

```bash
# Detect your card
nvctl asus detect

# Power connector health
nvctl asus power

# GPU status overview
nvctl asus status

# JSON output for scripts
nvctl asus power --json
```

## Overclocking

nvcontrol supports overclocking on the Astral:

```bash
# View current clocks
nvctl overclock info

# Apply GPU offset (in MHz)
nvctl overclock gpu-offset 100

# Apply memory offset
nvctl overclock mem-offset 200
```

## Fan Control

```bash
# View fan status
nvctl fan status

# Set fan curve
nvctl fan curve "30:40,50:50,70:70,85:100"
```

## Troubleshooting

### Power Detector Shows "Read failed"

1. Install i2c-tools: `sudo pacman -S i2c-tools`
2. Run with sudo: `sudo nvctl asus power`
3. Check nvidia driver is loaded: `nvidia-smi`

### Card Not Detected

Your Astral should be detected automatically. Verify with:
```bash
lspci -nn | grep -i nvidia
# Should show: 10de:XXXX subsystem 1043:89e3
```

## More Information

- [docs/POWER_DETECTION.md](./docs/POWER_DETECTION.md) - Technical details
- [docs/ASUS_ASTRAL_FEATURES.md](./docs/ASUS_ASTRAL_FEATURES.md) - Full feature list
- [docs/RTX_5090_SETUP_GUIDE.md](./docs/RTX_5090_SETUP_GUIDE.md) - RTX 50 setup
