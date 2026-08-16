# ASUS Power Monitor API

`asus_power_detector` exposes read-only 12V-2x6 telemetry for known ASUS Astral
and Matrix subsystem IDs.

## Main types

- `AsusPowerDetector::new(pci_id)` binds discovery to one NVIDIA PCI device.
- `read_power_rails()` returns `PowerConnectorStatus` from hwmon or direct SMBus.
- `PowerRailReading` carries optional millivolts, milliamps, watts, and warning state.
- `PowerHistory` records bounded samples and calculates averages, peaks, trends,
  and warning counts.
- `detect_asus_gpus()` returns supported ASUS devices without probing unknown boards.

```rust
use nvcontrol::asus_power_detector::{AsusPowerDetector, PowerHistory};

let detector = AsusPowerDetector::new("0000:01:00.0")?;
let mut history = PowerHistory::new();
let status = detector.read_and_record(&mut history)?;

println!("source: {}", status.source);
println!("connector: {:?} W", status.total_power_w);
for pin in status.rails {
    println!("pin {}: {:?} mV {:?} mA", pin.rail_id + 1, pin.voltage_mv, pin.current_ma);
}
```

## Source and safety contract

The detector prefers an `astral12vhpwr` hwmon device. Otherwise it issues one
24-byte `I2C_SMBUS_I2C_BLOCK_DATA` read at command `0x80` and address `0x2b` on
the selected GPU's NVIDIA I2C adapter 1. It has no register-data write operation,
does not scan unrelated buses, and does not invoke shutdown or power-limit actions.

See [Power Detector+](../hardware/power-detection.md) for protocol, thresholds,
permissions, and supported board IDs.
