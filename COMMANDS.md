# nvcontrol COMMANDS

This document lists all CLI commands and options for nvcontrol (nvctl).

## GPU Commands

- `nvctl gpu info`  
  Show basic GPU information (name, driver, VRAM, etc).

- `nvctl gpu stat`  
  Launch a live TUI dashboard for GPU stats (temperature, fan, VRAM, utilization, power, etc). Press `q` to quit.

- `nvctl gpu capabilities`  
  Show GPU overclocking capabilities and limits.

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

## Overclocking Commands

- `nvctl overclock info`  
  Show memory timings and GPU information.

- `nvctl overclock apply --gpu-offset <mhz> --memory-offset <mhz> --power-limit <percent>`  
  Apply overclocking settings. Example: `nvctl overclock apply --gpu-offset 100 --memory-offset 500 --power-limit 110`

- `nvctl overclock profile <name>`  
  Apply a saved overclocking profile.

- `nvctl overclock stress-test [duration]`  
  Run GPU stress test for specified minutes (default: 5). Example: `nvctl overclock stress-test 10`

- `nvctl overclock reset`  
  Reset all overclocking settings to defaults.

## VRR (Variable Refresh Rate) Commands

- `nvctl vrr status`  
  Show VRR/Adaptive Sync status for all displays.

- `nvctl vrr enable <display>`  
  Enable VRR for a specific display. Example: `nvctl vrr enable DP-1`

- `nvctl vrr disable <display>`  
  Disable VRR for a specific display. Example: `nvctl vrr disable DP-1`

- `nvctl vrr configure <display> --min-refresh <hz> --max-refresh <hz>`  
  Configure VRR refresh rate range. Example: `nvctl vrr configure DP-1 --min-refresh 48 --max-refresh 144`

## Upscaling (DLSS/FSR) Commands

- `nvctl upscaling status`  
  Show upscaling technology capabilities (DLSS, FSR, XeSS support).

- `nvctl upscaling enable <game> --tech <technology> --quality <level>`  
  Enable upscaling for a game. Example: `nvctl upscaling enable cyberpunk2077 --tech dlss --quality balanced`
  
  Technologies: `dlss`, `fsr`, `xess`, `native`  
  Quality levels: `performance`, `balanced`, `quality`, `ultra`

- `nvctl upscaling disable <game>`  
  Disable upscaling for a game. Example: `nvctl upscaling disable cyberpunk2077`

- `nvctl upscaling profiles`  
  List all game upscaling profiles.

- `nvctl upscaling auto-detect`  
  Auto-detect running games with upscaling profiles.

## Driver Management Commands

- `nvctl drivers status`  
  Show current driver status, version, and available updates.

- `nvctl drivers install <type>`  
  Install NVIDIA drivers. Types: `proprietary`, `open`, `open-beta`  
  Example: `nvctl drivers install open`

- `nvctl drivers update`  
  Update current driver to latest version.

- `nvctl drivers rollback`  
  Rollback to previous driver version (Arch Linux only).

## Shell Completion

- Install shell completions:
  ```bash
  # Automatic detection
  ./scripts/install-completions.sh
  
  # Manual shell specification
  ./scripts/install-completions.sh zsh
  ./scripts/install-completions.sh bash
  ./scripts/install-completions.sh fish
  ```

## Examples & Use Cases

### Gaming Setup
```bash
# Enable VRR for competitive gaming
nvctl vrr enable DP-1

# Apply performance overclock
nvctl overclock apply --gpu-offset 150 --memory-offset 800 --power-limit 115

# Enable DLSS for single-player games
nvctl upscaling enable cyberpunk2077 --tech dlss --quality quality

# Enable DLSS Performance for competitive games  
nvctl upscaling enable cs2 --tech dlss --quality performance
```

### Display Configuration
```bash
# Set high vibrance for gaming
nvctl display vibrance 800 600

# Enable HDR for media consumption
nvctl display hdr enable 0

# Check VRR status
nvctl vrr status
```

### Driver Management
```bash
# Check driver status
nvctl drivers status

# Install open source drivers
nvctl drivers install open

# Update to latest drivers
nvctl drivers update
```

### Performance Monitoring
```bash
# Live GPU monitoring
nvctl gpu stat

# Check overclocking capabilities
nvctl gpu capabilities

# Run stress test
nvctl overclock stress-test 15
```

---

For more details, see DOCS.md or run `nvctl <command> --help`.
