# ASUS Power Detector+ for Linux

nvcontrol includes native support for ASUS Power Detector+, a feature that monitors the 12V-2x6 power connector on high-end ASUS ROG graphics cards.

## What is Power Detector+?

Power Detector+ is an ASUS GPU Tweak III feature that monitors the current flow through each pin of the 12V-2x6 power connector. This helps detect:

- Loose or improperly seated connectors
- Poor-quality power cables
- Overloaded power rails
- Potential melting connector issues (as seen with early 12VHPWR connectors)

## Supported Cards

| Model | Subsystem ID | Status |
|-------|-------------|--------|
| ROG Astral RTX 5090 | 1043:89e3 | Fully Supported |
| ROG Matrix RTX 5090 | TBD | Planned |

## Usage

### Command Line

```bash
# Detect ASUS ROG GPUs
nvctl asus detect

# Read power rail status
nvctl asus power

# JSON output for scripting
nvctl asus power --json

# Continuous monitoring
nvctl asus power --watch
```

### TUI Dashboard

The Power tab in the TUI (`nvctl tui`) includes an ASUS Power Detector+ panel at the bottom when a supported card is detected.

## Health Status

The connector health is reported as:

| Status | Color | Meaning |
|--------|-------|---------|
| GOOD | Green | All rails under 7A per pin |
| WARNING | Yellow | One or more rails between 7-9.2A |
| CRITICAL | Red | Rails exceeding 9.2A (danger zone) |
| UNKNOWN | Gray | Unable to read sensor data |

## Technical Details

### How It Works

1. **Detection**: nvcontrol scans for NVIDIA GPUs with ASUS subsystem vendor ID (0x1043)
2. **I2C Bus Discovery**: Identifies the GPU's I2C bus that hosts the power monitoring chip
3. **Sensor Probing**: Reads from I2C address 0x2b (power monitor)
4. **Data Reading**: Reads 6 power rail registers (0x60, 0x62, 0x64, 0x66, 0x68, 0x6A)

### Register Format

- Each rail returns a 16-bit little-endian word
- Values are converted to estimated current in milliamps
- Calibration is approximate; exact conversion requires ASUS documentation

### Safety

This implementation is **READ-ONLY**:
- Uses `i2cget` for safe I2C reads
- Never writes to any I2C registers
- Cannot modify any GPU settings
- Safe to use at any time

## Troubleshooting

### "Read failed" Error

1. Ensure i2c-tools is installed: `pacman -S i2c-tools`
2. Check I2C device permissions
3. Try running with sudo: `sudo nvctl asus power`

### "No ASUS ROG GPU detected"

- Card must have ASUS subsystem vendor ID (0x1043)
- Feature only works on ROG Astral/Matrix series with power monitoring

### "I2C bus not found"

- GPU I2C buses are created by the nvidia driver
- Ensure nvidia driver is loaded: `lsmod | grep nvidia`
- Check for i2c-N directories: `ls /sys/bus/pci/devices/0000:01:00.0/i2c-*`

## Estimated Current Accuracy

Current readings are **estimates** based on observed values:

- The scaling factor is approximated (2 mA per raw unit)
- Actual values may vary based on sensor calibration
- Use as relative indicator, not absolute measurement
- ASUS GPU Tweak III uses proprietary calibration data

## See Also

- [ASUS_ASTRAL_FEATURES.md](./ASUS_ASTRAL_FEATURES.md) - Full ASUS ROG feature list
- [RTX_5090_SETUP_GUIDE.md](./RTX_5090_SETUP_GUIDE.md) - RTX 50-series setup guide
