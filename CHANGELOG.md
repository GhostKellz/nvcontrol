# Changelog

All notable changes to nvcontrol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2025-07-06

### Added

#### **Fan Control System Overhaul**
- **Advanced Fan Control Features**
  - Custom fan curve support with temperature-based control points
  - Multiple fan profiles (Silent, Performance, Custom) with profile switching
  - Fan health monitoring and status reporting
  - Zero RPM mode support for silent operation at low temperatures
  - Hysteresis control to prevent fan speed oscillation
  - Manual fan testing and diagnostics
  - Fan curve validation and safety limits
  - Profile-based fan management with save/load functionality

#### **TUI (Terminal User Interface) Enhancements**
- **VRR (Variable Refresh Rate) Control**
  - VRR toggle functionality accessible via 'v' key
  - Real-time VRR status display in status bar
  - VRR state persistence and feedback
- **Gaming Mode Integration**
  - Gaming mode toggle via 'g' key with latency optimizations
  - Automatic performance fan profile application in gaming mode
  - Gaming mode status indication in TUI status bar
- **Enhanced Status Bar**
  - Multi-section status bar with feature status indicators
  - Real-time VRR and gaming mode status display
  - Improved GPU selection indicators for multi-GPU systems
- **Improved Help System**
  - Updated help popup with new keybinding information
  - Comprehensive feature documentation within TUI

#### **GUI (Graphical User Interface) Major Expansion**
- **New Dedicated Tabs**
  - **VRR Tab**: Complete VRR management interface with display detection and settings
  - **Latency Tab**: Comprehensive latency optimization controls and presets
  - **Recording Tab**: NVENC/AV1 recording with Shadowplay-like instant replay features
  - **Gamescope Tab**: Full Gamescope integration for Steam Deck and Linux gaming
  - **Shader Cache Tab**: Shader cache management and optimization tools
  - **Driver Management Tab**: Driver information and management utilities

#### **Recording System (NVENC/AV1)**
- **Professional Recording Features**
  - NVENC H.264/H.265 and AV1 encoding support
  - Lossless recording mode for maximum quality
  - Instant replay functionality (Shadowplay-like)
  - Configurable recording presets (Gaming, Streaming, Archive)
  - Manual and automatic recording controls
  - Real-time encoding status monitoring
- **Recording Presets**
  - Gaming preset: High quality, moderate file size
  - Streaming preset: Optimized for live streaming
  - Archive preset: Maximum quality preservation
  - Custom preset: User-defined settings

#### **System Tray Enhancements**
- **Live GPU Statistics**
  - Real-time GPU stats display in tray tooltip
  - Temperature, utilization, memory usage, and power draw
  - VRR and gaming mode status indicators
  - Multi-GPU support with individual GPU statistics

#### **Latency Optimization System**
- **Advanced Latency Controls**
  - GPU scheduling priority optimization
  - Low latency mode configuration
  - Frame pacing and presentation optimization
  - Input lag reduction techniques
  - Gaming-focused latency presets

#### **Gamescope Integration**
- **Complete Gamescope Support**
  - Gamescope configuration management
  - Resolution and refresh rate control
  - Upscaling and filtering options
  - HDR and color management
  - Steam Deck optimization features

### Enhanced

#### **Fan Control Module**
- **Data Structures**
  - New `FanCurve` struct with temperature points and hysteresis
  - `FanProfile` system for multiple curve management
  - `FanHealthStatus` for comprehensive health monitoring
  - Enhanced `FanInfo` with profile and health data
- **Advanced Algorithms**
  - Linear interpolation for smooth fan curve transitions
  - Hysteresis implementation to prevent speed oscillation
  - Temperature-based zero RPM threshold management
  - Health assessment based on RPM and temperature correlation

#### **TUI User Experience**
- **Navigation Improvements**
  - Enhanced keybinding system with feature toggles
  - Improved status feedback for user actions
  - Real-time feature state tracking and display
- **Status Management**
  - Temporary status message system with auto-clear
  - Multi-section status bar for better information density
  - Feature status indicators for VRR and gaming mode

#### **GUI Architecture**
- **Tab System Expansion**
  - Modular tab implementation for easy feature addition
  - Consistent UI patterns across all tabs
  - Responsive design with proper widget sizing
- **Feature Integration**
  - Seamless integration of recording controls
  - Real-time status updates across all tabs
  - Consistent theming and styling

### Fixed

#### **Build System**
- **Compilation Issues**
  - Fixed `break` statement with value in `for` loop
  - Resolved type mismatches in temperature comparisons
  - Fixed `MemoryInfo` struct initialization with all required fields
  - Updated egui `DragValue` API calls to use `clamp_range`
  - Removed unused imports and variables

#### **Code Quality**
- **Error Handling**
  - Improved error handling in fan control operations
  - Better fallback mechanisms for NVML operations
  - Enhanced error messages and user feedback

### Technical Improvements

#### **Dependencies**
- **New Dependencies**
  - Added `dirs` crate for proper directory management
  - Enhanced NVML wrapper usage for new features

#### **Module Structure**
- **New Modules**
  - `src/recording.rs`: Complete NVENC/AV1 recording implementation
  - Enhanced `src/fan.rs`: Advanced fan control with health monitoring
  - Updated `src/tray.rs`: Live GPU statistics in tooltip
  - Enhanced `src/tui.rs`: VRR and gaming mode integration

#### **API Design**
- **Function Signatures**
  - Improved error handling with consistent `NvResult` usage
  - Enhanced parameter validation and safety checks
  - Better abstraction for hardware interaction

### Performance

#### **Resource Management**
- **Memory Optimization**
  - Efficient data structures for fan curve management
  - Optimized GPU statistics collection for tray display
  - Reduced memory allocations in real-time operations

#### **Responsiveness**
- **UI Performance**
  - Non-blocking operations for hardware queries
  - Efficient widget updates in GUI tabs
  - Smooth real-time data updates in TUI

### Documentation

#### **Code Documentation**
- **Comprehensive Comments**
  - Detailed function documentation for new features
  - Clear explanation of complex algorithms
  - Usage examples for new API functions

#### **User Documentation**
- **Feature Documentation**
  - Updated help system with new keybindings
  - Clear feature descriptions in GUI tooltips
  - Comprehensive error messages and guidance

---

## Previous Versions

### [0.5.0] - Previous Release
- Initial TUI implementation
- Basic fan control
- GPU monitoring
- Overclocking support
- Profile system foundation

---

## Development Notes

### Build Status
- ✅ All compilation errors resolved
- ✅ Clean build with only minor warnings
- ✅ Both GUI and TUI binaries building successfully
- ✅ All new modules properly integrated

### Testing Status
- ✅ Basic functionality verified
- ⏳ Comprehensive testing in progress
- ⏳ Multi-GPU testing pending
- ⏳ Cross-platform validation pending

### Next Steps
- [ ] Comprehensive testing of new features
- [ ] User experience refinement
- [ ] Performance optimization
- [ ] Documentation completion
- [ ] Release preparation

---

**Note**: This changelog represents significant enhancements to nvcontrol, with major improvements to fan control, user interfaces, and new features like recording and VRR management. The changes maintain backward compatibility while substantially expanding functionality.
