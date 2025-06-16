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

- `nvctl display hdr status`  
  Show HDR status for all displays.

- `nvctl display hdr enable <display_id>`  
  Enable HDR for a specific display. Example: `nvctl display hdr enable 0`

- `nvctl display hdr disable <display_id>`  
  Disable HDR for a specific display. Example: `nvctl display hdr disable 0`

- `nvctl display hdr toggle <display_id>`  
  Toggle HDR on/off for a specific display. Example: `nvctl display hdr toggle 0`

## Fan Commands

- `nvctl fan info`  
  Show fan speeds and control status.

- `nvctl fan set <fan_id> <percent>`  
  Set fan speed (if supported). Example: `nvctl fan set 0 75`

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
