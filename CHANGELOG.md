# Changelog

All notable changes to nvcontrol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.4] - 2026-01-26

### Added
- **GPU Selection Persistence**: `nvctl gpu select` now saves selection to config
- **Lutris Game Scanner**: Parses `~/.config/lutris/games/*.yml` for game detection
- **Heroic Game Scanner**: Parses `~/.config/heroic/sideload_apps/library.json`
- **EDID HDR Parsing**: Display HDR capability detection from EDID data
- **NVLink Detection**: Multi-GPU NVLink status via nvidia-smi
- **Primary GPU Detection**: DRM sysfs-based primary GPU identification
- **Overclock Offset Getter**: `get_current_offsets()` for reading current OC state
- **Voltage Curve Power Target**: Automatic power limit calculation for undervolting

### Changed
- Replaced 7 unsafe `libc::geteuid()` calls with safe `nix::unistd::geteuid().is_root()`
- Replaced 6 unsafe `std::mem::zeroed()` calls with `bytemuck::Zeroable::zeroed()`
- Improved NVKMS ioctl error handling with `nix::errno::Errno::result()`
- Refactored TUI GPU monitor with proper error handling and cleanup guard
- Mutex operations now use poison recovery (`unwrap_or_else(|e| e.into_inner())`)

### Fixed
- Potential panics from unwrap calls in production code paths
- Terminal state now properly restored on TUI monitor exit or panic
- Game profile auto-apply no longer panics on poisoned mutex
- **GUI Performance on Wayland/KDE**: Eliminated frame timing issues
  - Replaced unconditional `request_repaint()` with rate-limited repaints (500ms)
  - Moved System tab subprocess calls (nvidia-smi, uname, lspci) to 30s cached refresh
  - Removed duplicate repaint calls from GPU, Fan, and Overclock tabs
  - Added `has_active()` method to ToastManager for conditional repaints
  - Prevents KDE Plasma compositor frame timeout and desktop lockups

## [0.8.3] - 2026-01-13

### Added
- **Legacy GPU Detection** (`nvctl driver check`):
  - `detect_gpu_architecture()` for Maxwell/Pascal/Turing/Ampere/Ada/Blackwell detection
  - Deprecation warnings for Maxwell/Pascal GPUs on driver 590+
  - Recommends AUR legacy driver packages (nvidia-470xx, nvidia-535xx, nvidia-390xx)
  - Unit tests for architecture detection

- **Explicit Sync Commands** (`nvctl wayland explicit-sync`):
  - `status` - Comprehensive check of driver/kernel/compositor support
  - `enable` - Enables explicit sync in KDE/GNOME/Hyprland
  - Detects compositor versions (Plasma 6.1+, GNOME 46+, Hyprland 0.39+)
  - Shows active status when all components support explicit sync

- **Top-level HDR Command** (`nvctl hdr`):
  - `status`, `enable`, `disable` - HDR control across all displays
  - `config` - Show HDR configuration (peak brightness, tone mapping, color space)
  - `set-brightness <nits>` - Set peak brightness (400-10000 nits)
  - `tools` - References renodx, PumboAutoHDR, VK_hdr_layer for game HDR
  - `capabilities` - Display HDR capabilities from EDID

- **DLSS 4.5 Support**:
  - Added `Dlss4_5` version enum for enhanced Multi-Frame Generation
  - DLSS 4.5 detected when driver 590+ on RTX 50 series (Blackwell)
  - Status shows Multi-Frame Generation (up to 4x) and Enhanced Frame Pacing
  - Optical Flow Accelerator generation displayed (Gen 4 for RTX 50)

- **Kubernetes GPU Detection** (container.rs):
  - Proper JSON parsing for K8s pod GPU resources (`nvidia.com/gpu`)
  - `check_nvidia_device_plugin()` - Checks if NVIDIA device plugin is running
  - `get_cluster_gpu_capacity()` - GPU capacity per node in cluster

### Changed
- DLSS version detection now correctly identifies DLSS 4 for RTX 50 series
- `DlssVersion` enum now implements `Display` trait for user-friendly output
- Documentation updated with explicit sync guide and legacy GPU migration info

### Fixed
- DLSS status was showing "Dlss3_5" on RTX 50 series instead of "DLSS 4"

## [0.8.2] - 2026-01-12

### Added
- **DLSS Management**: Full DLSS DLL management for Proton gaming
  - Scan Steam/Lutris/Heroic games for DLSS DLLs
  - Read actual DLL versions from PE headers
  - Upgrade game DLLs with backup/restore
  - Generate Proton launch options (PROTON_DLSS_UPGRADE)
- GUI cleanup and improvements

## [0.8.1] - 2026-01-11

### Added
- **GSP Firmware Management**: GSP firmware status and troubleshooting
- **DKMS Automation**: Enhanced DKMS workflow for nvidia-open
  - Source type detection (Packaged/Git/Manual)
  - `nvctl driver source` commands (status/init/update/sync)
  - `nvctl driver dkms cleanup` for old kernel modules
  - Fixed PIPESTATUS handling in DKMS wrapper script

## [0.7.6] - 2025-12-04

### Added
- **Backend Abstraction Layer** for deterministic testing without NVIDIA hardware:
  - `NvmlBackend` trait with `RealNvmlBackend` and `MockNvmlBackend` implementations
  - `DisplayCommandRunner` trait with `ShellDisplayRunner` and `MockDisplayRunner`
  - `SharedNvmlBackend` (`Arc<dyn NvmlBackend>`) for thread-safe sharing across modules
  - Extended trait methods: `get_driver_version`, `get_power_limit`, `get_power_limit_constraints`, `get_power_limit_default`, `set_power_limit`, `get_cuda_cores`, `get_compute_capability`, `get_uuid`, `get_pci_bus_id`, `get_fan_count`, `is_fan_control_supported`
- **GuiBackendContext**: Unified context struct bundling `SharedNvmlBackend` + `SharedDisplayRunner` for GUI/TUI use
  - `GuiBackendContext::new()` for production, `GuiBackendContext::mock()` for testing
  - Convenience methods: `get_metrics()`, `get_device_info()`, `is_nvml_available()`
- **Process Listing API**: `ProcessInfo` struct and backend methods for GPU process queries
  - `get_running_graphics_processes(index)` and `get_running_compute_processes(index)`
  - Supports both real NVML and mock backends for deterministic testing
- **Max Clock API**: `get_max_gpu_clock(index)` and `get_max_memory_clock(index)` for OC tab display
- **Backend Status Enum**: `BackendStatus` for unified UI messaging across all interfaces
  - Variants: `Available`, `NvmlUnavailable`, `DisplayUnavailable`, `AllUnavailable`
  - Methods: `is_nvml_available()`, `is_display_available()`, `status_message()`
- **Cached Metrics**: `CachedMetrics` struct with staleness detection
  - `is_stale(max_age_secs)` and `age_secs()` for metrics freshness tracking
- **Display Runner Availability**: `is_available()` method added to `DisplayCommandRunner` trait
- **TUI Session Persistence**: Settings survive restarts
  - `TuiSessionState` struct in `config.rs` saves selected GPU, tab, fan curve, OC settings
  - Automatic load on startup and save on exit
- Deterministic backend tests (`tests/test_mock_backends.rs`) with 37 test cases:
  - GPU metrics collection and multi-GPU enumeration via mocks
  - Display detection flows (X11/Wayland/headless)
  - Monitoring loop simulation without hardware
  - GPU info retrieval with error handling
  - Multi-GPU detection, count, and info via backend
  - Extended backend method tests (power limits, CUDA, fans)
  - Fan module tests (list, multi-GPU, health assessment, graceful fallback)
  - Compositor-specific display runner tests (KDE/GNOME/Hyprland)
  - HDR/VRR detection flow simulation via mocks
  - GuiBackendContext and TuiApp mock backend injection tests
- **Compositor Mock Support**: `MockDisplayRunner` extended with:
  - `kde()`, `gnome()`, `hyprland()` constructors with realistic mock outputs
  - `with_compositor()`, `with_command_output()` builders for custom scenarios
  - Mock outputs for kscreen-doctor, gsettings, hyprctl

### Changed
- `src/gpu.rs` now uses `SharedNvmlBackend` instead of direct `nvml_wrapper` calls
- `src/monitoring.rs` refactored: `live_gpu_watch`, `export_gpu_metrics`, `run_gpu_benchmark` accept backend parameter
- `src/multi_gpu.rs` refactored: `detect_gpus_with_backend`, `get_gpu_info_with_backend`, `get_gpu_count_with_backend` accept backend; legacy functions create backend internally
- `src/fan.rs` refactored: `list_fans_with_backend`, `enable_zero_rpm_mode_with_backend`, `monitor_fan_health_with_backend` accept backend; legacy functions create backend internally
- `src/advanced_power.rs` refactored: `DynamicPowerManager`, `BatteryBoost`, `PowerProfileManager` now accept `SharedNvmlBackend`; legacy constructors create backend internally
- CLI (`nvctl`) and Interactive CLI create backend once at startup, pass to GPU commands
- CI gate enforces zero clippy warnings and deterministic test coverage
- `src/tui.rs`: `TuiApp` fully migrated to use `GuiBackendContext`
  - All NVML queries (metrics, processes, clocks, power, fans) now go through backend
  - Removed `Device`, `UsedGpuMemory` imports - only `Nvml` retained for legacy struct field
  - `draw_memory`, `draw_processes`, `draw_overclocking`, `draw_fan` all use backend
  - `BackendStatus` integrated for displaying error banners when NVML/display unavailable
- `src/tray.rs`: `SystemTray::with_backend()` accepts shared backend, eliminates duplicate NVML sessions
- `src/notifications.rs`: `AlertMonitorThread::with_backend()` uses shared backend for metrics polling
- nvbind and bolt moved to expirmental and testing functionality. Still early concept and not ready for daily use. 

### Security
- **ShellDisplayRunner Hardening**: Command allow-list with absolute paths
  - Only approved system utilities can be executed (xrandr, nvidia-settings, hyprctl, gsettings, etc.)
  - Commands not in allow-list are rejected with `DisplayError::CommandNotAllowed`
  - Multiple distro paths supported per command (e.g., `/usr/bin/qdbus`, `/usr/lib/qt6/bin/qdbus`)
  - Prevents PATH injection attacks and ensures predictable behavior

### Fixed
- **State Migration Safeguards**: Upgrade from v0.7.5 preserves user settings
  - Version field added to `TuiSessionState` for future migrations
  - Values validated and clamped to safe ranges on load
  - Corrupt state files backed up before resetting to defaults
  - Tests cover migration from v0 (no version) to v1
- **BackendStatus Hotplug Debouncing**: Prevents UI flicker during rapid attach/detach
  - `StatusTracker` debounces status transitions (2-second threshold)
  - `refresh_status()` method for periodic backend availability checks
  - `is_status_transitioning()` indicates pending status changes
  - Prevents indicator flicker during eGPU/USB-C dock hotplug events

## [0.7.5] - 2025-12-03

### Changed
- Clippy cleanup: properly fixed lint warnings instead of suppressing them
- Reduced `#![allow(clippy::...)]` directives in lib.rs
- Version removed from README badges (now uses crates.io badge as source of truth)
- Download URLs in README now use `/latest/` instead of hardcoded versions

### Fixed
- Code quality improvements across multiple modules

## [0.7.4] - 2025-12-02

### Added
- Multi-runner release workflow with separate CLI and GUI builds
- Protobuf compiler added to GUI runner Dockerfile

### Changed
- Release workflow now builds CLI (minimal) and GUI (full features) separately
- Improved CI/CD pipeline reliability

## [0.7.3] - 2025-12-01

### Added
- ASUS ROG Astral RTX 5090 enhancements
- Power Detector+ improvements for 12V-2x6 connector monitoring
- ASUS Aura RGB integration
- Full Hyprland VRR/HDR support

### Changed
- Polished KDE and GNOME compositor integrations
- Improved headless CI detection for nightly builds

### Fixed
- CI workflow fixes for self-hosted runner PATH configuration
- Cross-compile target path corrections

## [0.7.2] - 2024-11-27

### Added
- **ASUS Power Detector+** support for ROG Astral RTX 5090
  - Real-time 12V-2x6 connector monitoring via I2C
  - Health status indicator (GOOD/WARNING/CRITICAL)
  - Per-rail current readings
  - CLI commands: `nvctl asus detect`, `nvctl asus power`
  - TUI integration in Power tab
- `ASTRAL_OWNERS.md` quick start guide for ROG Astral users
- `docs/POWER_DETECTION.md` technical documentation
- Experimental warning banner in README for overclocking/voltage features

### Changed
- Polished sidebar UI in GUI
  - Removed broken Nerd Font icons causing box characters
  - Teal/mint colored menu items (cyan when selected)
  - Cleaner text-only labels
- Updated documentation links in README

### Fixed
- Navbar icons displaying as boxes on systems without Nerd Fonts

## [0.7.1] - 2024-11-27

### Added
- New display controls: color range, color space, dithering
- RTX 5090/Blackwell architecture detection and support
- Native digital vibrance implementation (no nvidia-settings required)

### Changed
- Tokyo Night Moon as default theme
- Improved Wayland compositor detection
- Enhanced TUI dashboard layout

## [0.7.0] - 2024-11-24

### Added
- Complete GUI application (`nvcontrol`)
- TUI dashboard with multiple tabs
- Digital vibrance via native NVKMS ioctls
- VRR/G-SYNC control
- HDR configuration
- Fan curve management
- Overclocking interface (GPU/Memory offsets)
- Auto-OC stress testing
- Power curve management
- Game profile auto-application
- Shader cache management
- Container GPU passthrough
- RGB control (ASUS Aura integration)
- Benchmark suite
- OSD (MangoHud integration)
- Gamescope presets
- Recording (NVENC)
- Multiple theme support (Tokyo Night, Dracula, ROG Red, etc.)

### Changed
- Rewrote core from Python to Rust
- Wayland-first architecture
- Modular codebase structure

## [0.6.0] - 2024-10-15

### Added
- Initial TUI monitor
- Basic GPU stats display
- Fan speed reading

## [0.5.0] - 2024-09-20

### Added
- CLI tool (`nvctl`)
- GPU info command
- Basic vibrance control via nvidia-settings

---

## Version History Summary

| Version | Date | Highlights |
|---------|------|------------|
| 0.7.6 | 2025-12-04 | Backend abstraction, TUI session persistence, testability |
| 0.7.5 | 2025-12-03 | Clippy cleanup, code quality polish |
| 0.7.4 | 2025-12-02 | Multi-runner CI/CD, CLI + GUI builds |
| 0.7.3 | 2025-12-01 | ASUS Astral enhancements, Hyprland VRR/HDR |
| 0.7.2 | 2024-11-27 | ASUS Power Detector+, GUI polish |
| 0.7.1 | 2024-11-27 | Display controls, native vibrance |
| 0.7.0 | 2024-11-24 | Full GUI, TUI, Wayland-first rewrite |
| 0.6.0 | 2024-10-15 | Initial TUI |
| 0.5.0 | 2024-09-20 | Initial CLI |
