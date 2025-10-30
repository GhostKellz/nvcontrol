# Gaming Commands

## Overview
Game profile management and automatic GPU optimization for gaming.

## Auto-Profile Application

### `nvctl gaming auto status`
Show game profile auto-application status and configuration.

**Usage:**
```bash
nvctl gaming auto status
```

**Output:**
- Enabled/disabled status
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

**Behavior:**
- Monitors running processes every poll interval
- Applies GPU profiles when games are detected
- Restores defaults when games exit

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
