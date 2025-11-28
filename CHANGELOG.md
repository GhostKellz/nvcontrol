# Changelog

All notable changes to nvcontrol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
| 0.7.2 | 2024-11-27 | ASUS Power Detector+, GUI polish |
| 0.7.1 | 2024-11-27 | Display controls, native vibrance |
| 0.7.0 | 2024-11-24 | Full GUI, TUI, Wayland-first rewrite |
| 0.6.0 | 2024-10-15 | Initial TUI |
| 0.5.0 | 2024-09-20 | Initial CLI |
