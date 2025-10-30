# GPU Commands

## Overview
GPU management commands for monitoring, benchmarking, and multi-GPU control.

## Commands

### `nvctl gpu info`
Display comprehensive GPU information.

**Usage:**
```bash
nvctl gpu info [--format <format>]
```

**Options:**
- `--format` - Output format: `table`, `json`, `human` (default: `table`)

**Example:**
```bash
nvctl gpu info --format json
```

---

### `nvctl gpu list`
List all detected GPUs with details.

**Usage:**
```bash
nvctl gpu list [--format <format>]
```

**Options:**
- `--format` - Output format: `table`, `json`, `human`

**Output includes:**
- GPU index
- GPU name
- Temperature
- Utilization %
- VRAM (GB)
- CUDA cores
- Compute capability

**Example:**
```bash
nvctl gpu list --format table
```

---

### `nvctl gpu select <index>`
Select active GPU for subsequent commands.

**Usage:**
```bash
nvctl gpu select <index>
```

**Arguments:**
- `<index>` - GPU index (0, 1, 2, etc.)

**Example:**
```bash
nvctl gpu select 1
```

**Note:** Currently not persistent across commands.

---

### `nvctl gpu stat`
Launch live TUI dashboard for GPU monitoring.

**Usage:**
```bash
nvctl gpu stat
```

**Features:**
- Real-time temperature, utilization, power monitoring
- Clock speeds and memory usage
- Interactive terminal UI

---

### `nvctl gpu benchmark`
Run GPU performance benchmark.

**Usage:**
```bash
nvctl gpu benchmark [--duration <secs>] [--test-type <type>]
```

**Options:**
- `--duration` - Benchmark duration in seconds (default: 30)
- `--test-type` - Test type: `compute`, `graphics`, `memory`, `all`

**Example:**
```bash
nvctl gpu benchmark --duration 60 --test-type all
```

---

### `nvctl gpu capabilities`
Show detailed GPU overclocking capabilities.

**Usage:**
```bash
nvctl gpu capabilities
```
