# Changelog

All notable changes to nvcontrol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.7] - 2026-04-22

### Changed
- **Dependency Updates**: Updated selected Rust dependencies for the next incremental release
  - `ratatui` 0.30
  - `indicatif` 0.18
- **GUI Stack Refresh**: Updated the egui/eframe stack to current 0.34-era releases
  - `eframe` 0.34.1
  - `egui_plot` 0.35.0
  - `egui-phosphor` 0.12.0
- **Tray Support Removed**: Removed the GTK3/libappindicator-backed tray implementation
  - This eliminates the remaining RustSec warnings from the default codebase
- **GUI Dependency Surface**: Disabled `eframe` default features and kept the explicit native Linux backend set used by nvcontrol
- **Driver Diagnostics Workflow**: Expanded release diagnostics and support collection into a fuller supportability pipeline
  - `nvctl driver diagnose-release` now supports JSON and YAML output
  - `nvctl driver support-bundle` now writes a text report plus JSON metadata sidecar
  - support bundles support gzip, tarball packaging, identifier/path redaction, and configurable log capture
  - `nvctl doctor --support` now runs diagnostics and produces a support tarball in one step
- **Display Preset Workflow**: Extended `nvctl monitors` to expose built-in monitor presets from the existing monitor profile system
  - added preset listing, suggestion, preview, and direct apply flows
  - reused the existing monitor profile definitions instead of adding another layout format
- **Profile Bundle Workflow**: `nvctl config` now has working bundle import/export/list behavior plus preview and diff commands
  - bundle previews summarize included OC, vibrance, and game profile content
  - bundle diffs support saved profile names or direct JSON bundle paths
  - bundle preview/diff now also accept `live` to snapshot current state
  - added bundle apply flow for replaying saved OC, power, fan, and vibrance settings
  - added `config capture` for saving current live state directly as a reusable bundle
  - live bundle capture now includes the current display layout
- **Game Launch Hooks**: Game launcher profiles now support pre-launch and post-exit hooks
  - launch flow now applies configured OC and power-limit settings through the existing launcher path
  - profile inspection now shows configured launch hooks in CLI output
  - added game profile create/delete flows, hook removal, and gamescope preset assignment
- **Game Auto Service**: `nvctl gaming auto start|stop|status` now uses a real PID-backed background lifecycle
  - added systemd user service install/enable/disable/uninstall flows for persistent startup
- **Container Runtime Doctor**: Added runtime doctor and smoke-test flows for Docker, Podman, and containerd
  - support bundles now include container runtime diagnostic output alongside DKMS/source-build data
- **Arch Hook Coverage**: Pacman hook generation now covers stock Arch kernels, `linux-zen`, `linux-cachyos-lto`, and custom `/boot/vmlinuz-*` kernel names such as `linux-ghost`
- **CLI Completion Generation**: Replaced the old hard-coded completion backend with clap-driven completion generation
  - refreshed the shipped Bash, Zsh, and Fish completion files from the real parser
  - completion generation now acts as a parser sanity check for the current CLI surface
- **CLI Surface Cleanup**: Tightened the public parser surface around current command paths
  - added the top-level `nvctl completion <bash|zsh|fish>` command
  - kept `nvctl vibrance <percentage>` as the primary workflow while preserving the `vibe` alias
  - removed deprecated top-level `drivers` and duplicate top-level `gsp` exposure in favor of `nvctl driver gsp`
- **Release Packaging Refresh**: Updated the release/install surfaces for the `0.8.7` desktop-first distribution flow
  - added `release/install-system.sh` to install the latest GitHub release tarball into `/usr/local`
  - installer now prefers the full `nvcontrol` GUI+CLI archive and falls back to the CLI-only `nvctl` archive when needed
  - Arch, Fedora, AppImage, Flatpak, Pop!_OS COSMIC, and root packaging metadata were refreshed for current binaries, desktop entry, icon paths, and release versioning
- **Release Workflow Refresh**: Release automation is now aligned with the single self-hosted Ubuntu 24.04 runner flow
  - tag builds validate and archive CLI and GUI artifacts before the publish job creates the GitHub release
  - stale runner-home assumptions were removed from CI/nightly/release workflow files
- **Developer Install Flow**: Refreshed local install and helper scripts for the current binary and completion workflow
  - `dev/install.sh` now builds the CLI and GUI explicitly, fails fast if artifacts are missing, and generates Bash/Zsh/Fish completions from the installed binary
- **Issue Reporting Templates**: Added GitHub issue templates for bug reports and feature requests
  - bug reports now direct users toward `nvctl driver diagnose-release` and support-bundle output before filing
- **Support UI Workflow**: Added a dedicated GUI support tab and companion support actions for diagnostics sharing
  - GUI support actions can create bundles, reopen the last bundle, and copy a redacted support workflow command
- **Game Auto Service Packaging**: Packaging moved toward the current `nvctl gaming auto daemon` user-service flow
  - Arch and Fedora package definitions were updated to ship the newer game auto-profile service behavior instead of the older game-detect wording

### Fixed
- **GPU Driver Reporting**: Corrected GPU info paths that could report the GPU name instead of the loaded driver version
- **Open Driver Detection**: Improved runtime detection of NVIDIA open kernel modules using `/proc/driver/nvidia/version` and module license data
- **GSP Firmware Checks**: Reworked firmware validation to use nvcontrol's current GSP status logic instead of relying on legacy version-only firmware paths
- **GSP Status Messaging**: Improved CLI/TUI output so newer proprietary-driver systems do not incorrectly report GSP as unavailable
- **Open Driver Diagnostics**: Improved release-alignment, firmware-layout, package-state, and ownership diagnostics for NVIDIA 595-era open-driver troubleshooting
- **Driver/TUI Workflow Hints**: Drivers TUI tab now surfaces DKMS doctor, source state, runtime doctor, and updated support workflow hints
- **Support Artifact Defaults**: Moved support bundle defaults away from `/tmp` to XDG state storage under `~/.local/state/nvcontrol/support/`
- **Support Bundle UX**: Removed automatic archive opening after bundle creation to avoid `xdg-open`/Ark spam and kept opening as an explicit user action
- **Support Bundle Payload**: Added boot entries, installed kernels, initramfs tool/images/findings, and Arch package inventory to support artifacts
- **Support Notification Dedupe**: Support-bundle notifications are now deduped by saved path, and integration tests suppress support notifications entirely to avoid desktop spam during `cargo test`
- **Boot/Initramfs Diagnostics**: Release diagnostics now report boot cmdline and initramfs state conservatively as supportability facts and warnings instead of treating healthy `nvidia-open` runtime systems as broken from initramfs contents alone
- **ASUS Power Monitor UI**: Reworked GUI presentation so the power monitor remains visible in the header while also having a dedicated Power tab and clearer support messaging
- **Clap Parser Cleanup**: Fixed several command definitions that were invalid or stale once real completion generation exercised the parser
  - resolved duplicate short-flag collisions in container and gamescope-related commands
  - changed value-taking boolean toggles to explicit `--enabled <true|false>` forms where required by clap
  - kept the top-level `nvctl vibrance <percentage>` workflow working while tightening related display/color command argument forms
- **Deprecated Driver Noise**: Removed the noisy `nvctl drivers is deprecated` warning from the legacy hidden path
- **Fedora Packaging Service Install**: Corrected the Fedora spec to install the desktop entry and current `nvcontrol-game-profile-auto.service` user unit explicitly
- **Installer Runtime Compatibility**: The release installer now accepts either `python3` or `python` when resolving release metadata
- **AppImage Base Refresh**: Updated the AppImage builder to Ubuntu 24.04 (`noble`) and `nvidia-utils-590`, and removed the stale GTK3 dependency from that surface

### Maintenance
- **Release Audit Review**: Reviewed the archived NVIDIA open GPU kernel modules snapshot (`595.58.03`) against current nvcontrol assumptions
- **Open Driver Polish**: Identified follow-up work for release alignment checks, broader PCI ID support diagnostics, and more precise GSP firmware layout reporting
- **Cargo Audit Cleanup**: Removed the `paste` advisory by updating the egui stack and removed GTK3/libappindicator advisories by deleting tray support
- **Documentation Refresh**: Added command, diagnostics, companion, and issue-reporting docs for the new support workflow
- **Release Surface Audit**: Reviewed shipped assets, package metadata, and release archives to remove stale paths and update package/install docs for the current release layout
- **Test Suite Hardening**: Modernized release-surface CLI tests around `assert_cmd` and `tempfile`, and added dedicated `help_contracts`, `regressions`, and `packaging_sanity` suites to catch parser, packaging, and workflow drift earlier

## [0.8.6] - 2026-03-06

### Added
- **NVIDIA Driver 595 Compatibility**: Full support for driver 595.45.04 (open-source kernel modules)
  - Updated NVKMS struct sizes to match driver ABI changes
  - Digital vibrance working on driver 595+
  - Documentation: `595_DRIVER_VIBRANCE.md` details all struct/enum changes

### Changed
- **Dependencies Updated**: 79 crates updated to latest compatible versions
  - clap 4.5.60, tokio 1.50.0, chrono 0.4.44, libc 0.2.182, bitflags 2.11.0
  - All futures-* crates to 0.3.32, wayland-* crates updated
  - zbus 5.14.0, zerocopy 0.8.40, and many others
- **NvKmsAllocDeviceRequest**: Removed deprecated `sli_mosaic` and `try_infer_sli_mosaic_from_existing_device` fields (driver 595+)
- **NvKmsDpyAttribute Enum**: Removed `ImageSharpening`, `ImageSharpeningAvailable`, `ImageSharpeningDefault` (no longer supported)
- **NvKmsAllocDeviceStatus Enum**: `BadDeviceId` → `BadRequest`, `AlreadyAllocated` → `FatalError`, added `NoHardwareAvailable`, `CoreChannelAllocFailed`

### Fixed
- **EPERM on vibrance ioctl**: Corrected struct sizes causing driver param validation failures
  - `NvKmsAllocDeviceReply`: 888 bytes (was 1248)
  - `NvKmsAllocDeviceParams`: 1512 bytes (was 1868)
  - `NvKmsQueryDpyDynamicDataReply`: 35096 bytes (was 35088)
  - Added `align(8)` to `NvKmsAllocDeviceReply` for proper NvU64 field alignment

### Deprecated
- **Image Sharpening**: `get_image_sharpening_info()` now returns `available: false`, `set_image_sharpening()` returns error (removed from NVKMS in driver 595)

## [0.8.5] - 2026-02-12

### Added
- **Safe Environment Module** (`safe_env.rs`): Centralized unsafe env var operations with safety docs
- **ASUS Power Monitor+ Expansion**: Enhanced power monitoring for ROG Astral/Matrix cards
  - Power history tracking with trend analysis (Rising/Stable/Falling)
  - Statistics: Average, Peak, Min power with sample counts
  - Warning count tracking for power rail issues
  - GUI: Two-column layout with rails and statistics in GPU tab
  - TUI: Full Power Monitor+ section in Power tab with live updates
- **GPU Tab Sparklines**: Mini graphs for temperature, power, and utilization in Live Statistics

### Changed
- **GUI Overclock Tab Removed**: Overclocking requires X11; GUI is Wayland-native
  - CLI overclock commands (`nvctl overclock`) remain fully functional
  - TUI overclock tab retained with X11 requirement notice
- Keyboard shortcuts renumbered: 1=GPU, 2=Fan, 3=Display, 4=Vibrance, 5=HDR, 6=Profiles, 7=OSD, 8=Settings
- Sidebar now shows 14 tabs (down from 15)

### Removed
- `OcPreset` enum and overclock state fields from GUI
- `apply_oc_preset()`, `apply_overclock()`, `reset_overclock()` methods from GuiState
- `VoltageCurve` widget import (unused after OC removal)
- `src/gui/tabs/overclock.rs` file deleted

### Fixed
- Dead code cleanup: Removed unused overclock-related imports and state

### Security
- Reduced unsafe blocks from 31 to 15 (env vars now use `safe_env` module)
- Remaining unsafe is legitimate kernel interface code (NVKMS ioctl)
- Added `docs/SAFETY_IMPROVEMENTS_ROADMAP.md` for future safety improvements

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
- Deterministic backend tests (`tests/mock_nvml.rs`, `tests/mock_display.rs`, `tests/mock_gui.rs`) covering NVML, display, and GUI mock flows:
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
| 0.8.6 | 2026-03-06 | NVIDIA driver 595 compatibility, dependency updates |
| 0.8.5 | 2026-02-12 | Safe env module, GUI overclock removal, ASUS power expansion |
| 0.8.4 | 2026-01-26 | GPU selection persistence, game scanners, EDID HDR, NVLink |
| 0.8.3 | 2026-01-13 | Legacy GPU detection, explicit sync, HDR control, DLSS 4.5 |
| 0.8.2 | 2026-01-12 | DLSS DLL management for Proton gaming |
| 0.8.1 | 2026-01-11 | GSP firmware management, DKMS automation |
| 0.7.6 | 2025-12-04 | Backend abstraction, TUI session persistence, testability |
| 0.7.5 | 2025-12-03 | Clippy cleanup, code quality polish |
| 0.7.4 | 2025-12-02 | Multi-runner CI/CD, CLI + GUI builds |
| 0.7.3 | 2025-12-01 | ASUS Astral enhancements, Hyprland VRR/HDR |
| 0.7.2 | 2024-11-27 | ASUS Power Detector+, GUI polish |
| 0.7.1 | 2024-11-27 | Display controls, native vibrance |
| 0.7.0 | 2024-11-24 | Full GUI, TUI, Wayland-first rewrite |
| 0.6.0 | 2024-10-15 | Initial TUI |
| 0.5.0 | 2024-09-20 | Initial CLI |
