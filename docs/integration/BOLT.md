# nvcontrol + Bolt Container Runtime Integration

Integration guide for using nvcontrol with Bolt container runtime for optimized gaming containers.

## Overview

Bolt is a next-generation container runtime optimized for gaming workloads. Combined with nvcontrol, you get full GPU control for containerized games.

## Quick Start

```bash
# Install both tools
paru -S nvcontrol bolt

# Launch gaming container with GPU control
bolt surge up --gpu --nvcontrol-profile gaming

# Monitor GPU performance
nvctl gpu stat
```

## Gaming Profiles

### Competitive Gaming
```toml
# ~/.config/nvcontrol/profiles/competitive.toml
[profile]
name = "Competitive Gaming"
digital_vibrance = 90
power_limit = 115
fan_curve = "aggressive"

[bolt]
isolation = "exclusive"
priority = "realtime"
```

### Content Creation
```toml
[profile]
name = "Streaming"
digital_vibrance = 75
power_limit = 100
fan_curve = "balanced"

[bolt]
isolation = "shared"
encoder = "nvenc"
```

## Auto-Apply on Container Launch

```bash
# Bolt configuration with nvcontrol integration
bolt config set gpu.control nvcontrol
bolt config set gpu.profile gaming
```

## API Integration

```rust
use nvcontrol::gpu::GpuController;
use bolt::container::Container;

// Launch container with GPU optimization
let gpu = GpuController::new()?;
gpu.apply_profile("gaming")?;

let container = Container::create()
    .with_gpu()
    .with_nvcontrol_profile("gaming")
    .launch()?;
```

## Monitoring

```bash
# Real-time container GPU stats
nvctl gpu stat --container-mode

# Bolt-specific metrics
bolt metrics --include-gpu
```
