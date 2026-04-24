# Configuration Commands

## Overview
Configuration management, profile bundle import/export, backups, and live-state bundle workflows.

## Commands

### `nvctl config show`
Display current nvcontrol configuration.

**Usage:**
```bash
nvctl config show
```

---

### `nvctl config edit`
Open configuration file in editor.

**Usage:**
```bash
nvctl config edit
```

---

### `nvctl config reset`
Reset all configuration to defaults (with confirmation).

**Usage:**
```bash
nvctl config reset
```

---

### `nvctl config backup`
Backup current configuration.

**Usage:**
```bash
nvctl config backup [--output <path>]
```

**Options:**
- `--output` - Output file path (default: timestamped backup)

**Example:**
```bash
nvctl config backup --output my-settings.tar.gz
```

---

### `nvctl config restore <path>`
Restore configuration from backup.

**Usage:**
```bash
nvctl config restore <path>
```

**Example:**
```bash
nvctl config restore my-settings.tar.gz
```

---

## Profile Management

### `nvctl config export`
Export GPU profile to file.

**Usage:**
```bash
nvctl config export --profile <name> --output <file>
```

**Options:**
- `--profile` - Profile name to export
- `--output` - Output file path (JSON or TOML)

**Example:**
```bash
nvctl config export --profile gaming-max --output gaming.toml
```

**Profile includes:**
- Overclock settings
- Power limits
- Fan curves
- Vibrance settings

---

### `nvctl config import`
Import a profile bundle from file.

**Usage:**
```bash
nvctl config import --input <file> [--name <name>] [--skip-validation]
```

**Options:**
- `--input` - Input file path (JSON or TOML)
- `--name` - Profile name (default: uses filename)
- `--skip-validation` - Skip safety validation checks

**Example:**
```bash
nvctl config import --input gaming.toml --name my-gaming-profile
```

**Safety:**
- Validates temperature limits
- Checks power limit bounds
- Verifies overclock offsets are reasonable
- Use `--skip-validation` only for trusted profiles

---

### `nvctl config capture`
Capture the current live state into a saved profile bundle.

**Usage:**
```bash
nvctl config capture --name <profile>
```

**Captures when available:**
- current overclock offsets
- power limit
- fan curve
- vibrance state
- current display layout

This is useful for creating a supportable before/after baseline before driver, kernel, or display changes.

**Example:**
```bash
nvctl config capture --name current-desktop
```

---

### `nvctl config preview`
Preview a saved profile bundle or a live snapshot.

**Usage:**
```bash
nvctl config preview --input <file|profile|live>
```

**Examples:**
```bash
nvctl config preview --input my-gaming-profile
nvctl config preview --input ./profiles/gaming.json
nvctl config preview --input live
```

`live` captures the current bundle-compatible state and summarizes it without saving a file.
That live snapshot now includes current display layout information when available.

---

### `nvctl config diff`
Diff two saved bundles, file paths, or a live snapshot.

**Usage:**
```bash
nvctl config diff --current <file|profile|live> --target <file|profile|live>
```

**Examples:**
```bash
nvctl config diff --current live --target my-gaming-profile
nvctl config diff --current baseline --target tuned
```

---

### `nvctl config apply`
Apply a saved profile bundle or a live snapshot-compatible bundle to the current system.

**Usage:**
```bash
nvctl config apply --input <file|profile|live>
```

**Applies when present in the bundle:**
- display layout
- overclock settings
- power limit
- fan curve
- vibrance settings

**Examples:**
```bash
nvctl config apply --input my-gaming-profile
nvctl config apply --input ./profiles/quiet.json
```

---

### `nvctl config profiles`
List all saved profile bundles.

**Usage:**
```bash
nvctl config profiles
```

**Output:**
- Profile names
- Descriptions
- Bundle metadata available in the saved JSON

---

## Configuration Files

**Location:** `~/.config/nvcontrol/`

**Files:**
- `config.toml` - Main configuration
- `power_management.toml` - Power curves and schedules
- `game_profile_auto.toml` - Game profile auto-application settings
- `profiles/` - Saved profile bundles directory
