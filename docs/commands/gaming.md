# Gaming Commands

## Overview
Game profile management and automatic GPU optimization for gaming.

## Auto-Profile Application

### `nvctl gaming auto start`
Start the background auto-profile service.

**Usage:**
```bash
nvctl gaming auto start
```

**Notes:**
- syncs launcher profiles into the detector store first
- spawns a background `nvctl` service process
- `status` reports PID and last applied profile when available

---

### `nvctl gaming auto stop`
Stop the background auto-profile service.

**Usage:**
```bash
nvctl gaming auto stop
```

---

### `nvctl gaming auto status`
Show game profile auto-application status and configuration.

**Usage:**
```bash
nvctl gaming auto status
```

**Output:**
- Enabled/disabled status
- Running state
- Service PID (when active)
- systemd user service status (when available)
- Poll interval (seconds)
- Apply delay (seconds)
- Restore on exit setting

---

### `nvctl gaming auto enable`
Enable automatic profile application.

**Usage:**
```bash
nvctl gaming auto enable
```

This updates the saved configuration. Use `nvctl gaming auto start` to launch the background service.

---

### `nvctl gaming auto disable`
Disable automatic profile application.

**Usage:**
```bash
nvctl gaming auto disable
```

---

### `nvctl gaming auto config`
Configure auto-application settings.

**Usage:**
```bash
nvctl gaming auto config [OPTIONS]
```

**Options:**
- `--poll-interval <secs>` - Game detection poll interval (1-10 seconds)
- `--apply-delay <secs>` - Delay before applying profile (0-10 seconds, anti-crash protection)
- `--restore-on-exit <bool>` - Restore default profile when game exits

**Examples:**
```bash
# Fast detection with no delay
nvctl gaming auto config --poll-interval 1 --apply-delay 0

# Safe mode with 5 second delay
nvctl gaming auto config --poll-interval 2 --apply-delay 5 --restore-on-exit true
```

---

### `nvctl gaming auto install-service`
Install a systemd user service unit for persistent auto-profile startup.

**Usage:**
```bash
nvctl gaming auto install-service
```

---

### `nvctl gaming auto uninstall-service`
Remove the systemd user service unit.

**Usage:**
```bash
nvctl gaming auto uninstall-service
```

---

### `nvctl gaming auto enable-service`
Enable and start the systemd user service.

**Usage:**
```bash
nvctl gaming auto enable-service
```

This is the recommended persistent startup path if you want auto-profile handling across login sessions.

---

### `nvctl gaming auto disable-service`
Disable and stop the systemd user service.

**Usage:**
```bash
nvctl gaming auto disable-service
```

---

## Game Launching

### `nvctl gaming launch run <profile>`
Launch a game with a profile.

**Usage:**
```bash
nvctl gaming launch run <profile> [-- <args>]
```

**Example:**
```bash
nvctl gaming launch run cyberpunk2077
nvctl gaming launch run cs2 -- -console -novid
```

---

### `nvctl gaming launch list`
List available game profiles.

**Usage:**
```bash
nvctl gaming launch list
```

---

### `nvctl gaming launch show <profile>`
Show details of a game profile.

**Usage:**
```bash
nvctl gaming launch show <profile>
```

---

### `nvctl gaming launch create`
Create a new game launch profile.

**Usage:**
```bash
nvctl gaming launch create <profile> <executable>
```

---

### `nvctl gaming launch delete`
Delete a game launch profile.

**Usage:**
```bash
nvctl gaming launch delete <profile>
```

---

### `nvctl gaming launch hook-add`
Add a pre-launch or post-exit hook.

**Usage:**
```bash
nvctl gaming launch hook-add <profile> <pre|post> <command> [args...]
```

---

### `nvctl gaming launch hook-list`
List hooks for a game profile.

**Usage:**
```bash
nvctl gaming launch hook-list <profile>
```

---

### `nvctl gaming launch hook-remove`
Remove a hook by phase and index from `hook-list`.

**Usage:**
```bash
nvctl gaming launch hook-remove <profile> <pre|post> <index>
```

---

### `nvctl gaming launch set-gamescope-preset`
Assign a named Gamescope preset to a launch profile.

**Usage:**
```bash
nvctl gaming launch set-gamescope-preset <profile> <preset>
```

---

### `nvctl gaming launch examples`
Create example launch profiles.

**Usage:**
```bash
nvctl gaming launch examples
```

## Latency Optimization

### `nvctl gaming latency optimize`
Apply latency optimizations.

**Usage:**
```bash
nvctl gaming latency optimize [--preset <preset>]
```

**Presets:**
- `ultra-low` - Maximum responsiveness
- `balanced` - Balance latency and power
- `quality` - Prioritize visual quality

---

## Gamescope

### `nvctl gaming gamescope launch <command>`
Launch game with Gamescope compositor.

**Usage:**
```bash
nvctl gaming gamescope launch [OPTIONS] <command>
```

**Options:**
- `--preset <preset>` - Gamescope preset: `performance`, `quality`, `balanced`, `competitive`, `cinematic`, `steamdeck`
- `--width <width>` - Override width
- `--height <height>` - Override height

**Example:**
```bash
nvctl gaming gamescope launch --preset competitive --width 1920 --height 1080 steam
```
