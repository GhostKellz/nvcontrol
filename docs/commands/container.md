# Container Commands

## Overview
Docker and container GPU management with NVIDIA runtime support.

## Commands

### `nvctl nvbind list`
List all GPU containers.

**Usage:**
```bash
nvctl nvbind list [--gpu-only] [--metrics]
```

**Options:**
- `--gpu-only` - Show only containers using GPUs
- `--metrics` - Display performance metrics

**Output:**
- Container ID
- Container name
- Image
- GPU devices
- GPU utilization (with --metrics)
- Power usage (with --metrics)

**Example:**
```bash
nvctl nvbind list --gpu-only --metrics
```

**Output format:**
```
┌────────────────────────────────────────────────────────────────────┐
│                       GPU Containers                               │
├──────────────┬──────────────────────┬─────────────┬───────────────┤
│ Container ID │ Name                 │ Image       │ GPU Devices   │
├──────────────┼──────────────────────┼─────────────┼───────────────┤
│ abc123def456 │ ml-training          │ pytorch     │ 0,1           │
│ 789ghi012jkl │ game-server          │ nvidia/cuda │ 0             │
└──────────────┴──────────────────────┴─────────────┴───────────────┘
```

---

### `nvctl container list`
List Docker containers with GPU information.

**Usage:**
```bash
nvctl container list
```

**Similar to `nvctl nvbind list` but uses Docker runtime directly**

---

## Requirements

- Docker installed and running
- NVIDIA Container Toolkit
- User must be in `docker` group or use sudo

**Check installation:**
```bash
docker --version
nvidia-smi
docker run --rm --gpus all nvidia/cuda:11.0-base nvidia-smi
```

---

## Troubleshooting

**No containers found:**
- Make sure Docker is running: `sudo systemctl start docker`
- Launch GPU container: `docker run --gpus all nvidia/cuda:11.0-base`

**Permission denied:**
- Add user to docker group: `sudo usermod -aG docker $USER`
- Log out and back in

**NVIDIA runtime not found:**
- Install NVIDIA Container Toolkit
- Configure Docker to use NVIDIA runtime
