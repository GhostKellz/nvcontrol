# Configuration Commands

## Overview
Configuration management, profile import/export, and backups.

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
Import GPU profile from file.

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

### `nvctl config profiles`
List all saved GPU profiles.

**Usage:**
```bash
nvctl config profiles
```

**Output:**
- Profile names
- Creation dates
- Profile types (overclock, gaming, etc.)

---

## Configuration Files

**Location:** `~/.config/nvcontrol/`

**Files:**
- `config.toml` - Main configuration
- `power_management.toml` - Power curves and schedules
- `game_profile_auto.toml` - Game profile auto-application settings
- `profiles/` - Saved GPU profiles directory
