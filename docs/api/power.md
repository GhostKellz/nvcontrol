# Power API

nvcontrol has two separate power surfaces:

- `power` changes NVML-backed board power policy;
- `asus_power_detector` reads supported Astral/Matrix connector telemetry.

Connector telemetry never changes the power limit or shuts down the host.

## Board power

```rust
use nvcontrol::power;

let gpus = power::get_power_info()?;
power::set_power_limit_percentage(90)?;
power::set_power_profile("balanced")?;
```

The public module also provides persistence mode, clock-boost, power-saving,
adaptive-management, custom-profile, monitoring, and automation helpers. These
are mutating operations unless their documentation explicitly says otherwise;
inspect current state and capture a profile before applying them.

```bash
nvctl power status
nvctl power limit --percentage 90
nvctl power profile --profile balanced
```

## Astral 12V-2x6 telemetry

```rust
use nvcontrol::asus_power_detector::{AsusPowerDetector, PowerHistory};

let detector = AsusPowerDetector::new("0000:01:00.0")?;
let mut history = PowerHistory::new();
let connector = detector.read_and_record(&mut history)?;
println!("{:?} W via {}", connector.total_power_w, connector.source);
```

`PowerConnectorStatus` contains the detected model, I2C bus, backend source,
six `PowerRailReading` values, connector total, optional load-gated balance,
warning state, health, and timestamp.

```bash
nvctl asus detect
nvctl asus power
nvctl asus power --json
nvctl asus power --watch --interval 1
```

See [ASUS Power Monitor API](asus-power-monitor.md) and
[Power Detector+](../hardware/power-detection.md).
