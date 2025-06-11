# nvcontrol COMMANDS

This document lists all CLI commands and options for nvcontrol (nvctl).

## GPU Commands

- `nvctl gpu info`  
  Show basic GPU information (name, driver, VRAM, etc).

- `nvctl gpu stat`  
  Launch a live TUI dashboard for GPU stats (temperature, fan, VRAM, utilization, power, etc). Press `q` to quit.

## Display Commands

- `nvctl display info`  
  Show display information (connected outputs, names, types).

- `nvctl display ls`  
  List all detected displays (index, name, type).

- `nvctl display vibrance <levels>`  
  Set digital vibrance per display (requires nVibrant). Example: `nvctl display vibrance 512 1023`

## Fan Commands

- (Planned) `nvctl fan info`  
  Show fan speeds and control status.

- (Planned) `nvctl fan set <fan_id> <percent>`  
  Set fan speed (if supported).

## Profiles & Automation

- (Planned) `nvctl profile save <name>`  
  Save current settings as a profile.

- (Planned) `nvctl profile load <name>`  
  Load a saved profile.

## Other

- `nvctl --help`  
  Show all available commands and options.

---

For more details, see DOCS.md or run `nvctl <command> --help`.
