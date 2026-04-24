# Container Commands

## Overview
Docker, Podman, containerd, and GPU runtime diagnostics.

### `nvctl container list`
List GPU-enabled containers.

**Usage:**
```bash
nvctl container list
```

---

### `nvctl container status`
Show GPU status for a specific container.

**Usage:**
```bash
nvctl container status [--container <id-or-name>]
```

---

### `nvctl container launch`
Launch a container with GPU access.

**Usage:**
```bash
nvctl container launch --image <image> [--name <name>] [--gpu <all|0|0,1>] [--runtime <docker|podman|containerd>]
```

---

### `nvctl container runtime info`
Show detected runtime/toolkit information.

**Usage:**
```bash
nvctl container runtime info
```

---

### `nvctl container runtime doctor`
Run NVIDIA container runtime diagnostics.

**Usage:**
```bash
nvctl container runtime doctor [--runtime <docker|podman|containerd>]
```

---

### `nvctl container runtime test`
Run a runtime-specific GPU smoke test.

**Usage:**
```bash
nvctl container runtime test [--runtime <docker|podman|containerd>]
```

---

### `nvctl container runtime setup`
Show or perform runtime setup guidance.

**Usage:**
```bash
nvctl container runtime setup --runtime <docker|podman|containerd>
```

---

### `nvctl container runtime configure`
Write nvcontrol runtime configuration defaults.

**Usage:**
```bash
nvctl container runtime configure
```

---

## Requirements

- Docker, Podman, or containerd installed when needed
- NVIDIA Container Toolkit
- Appropriate runtime permissions (`docker` group or root where required)

**Check installation:**
```bash
nvidia-smi
nvctl container runtime doctor --runtime docker
nvctl container runtime test --runtime docker
```

---

## Troubleshooting

**No containers found:**
- Make sure the chosen runtime is running
- Launch a test container: `nvctl container runtime test --runtime docker`

**Permission denied:**
- Add user to docker group: `sudo usermod -aG docker $USER`
- Log out and back in

**NVIDIA runtime not found:**
- Install NVIDIA Container Toolkit
- Configure the runtime with `nvctl container runtime setup --runtime docker`

**Supportability note:**
- container runtime doctor output is included automatically in `nvctl driver support-bundle` and `nvctl doctor --support`
