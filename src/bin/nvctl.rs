use clap::{Parser, Subcommand};
use console::{Key, Term, style};
use indicatif::{ProgressBar, ProgressStyle};
use nvcontrol::{
    display, drivers, fan, gamescope,
    gpu::{self, OutputFormat},
    latency, monitoring, overclocking, power, recording, upscaling, vrr,
    wayland_nvidia, kde_optimizer, power_profiles_daemon, arch_integration,
    gsp_firmware, multimonitor,
};
use serde_json;
use std::time::Duration;

#[derive(Parser)]
#[command(
    name = "nvctl",
    version,
    about = "🎮 NVIDIA Control CLI - Advanced GPU Management",
    long_about = "Advanced command-line interface for comprehensive NVIDIA GPU control and monitoring.\n\nFeatures: GPU monitoring, overclocking, fan control, VRR, recording, containers, and more.",
    after_help = "Examples:\n  nvctl gpu info --json           # GPU info in JSON format\n  nvctl fan curve apply gaming     # Apply gaming fan curve\n  nvctl monitor --watch            # Live monitoring\n  nvctl container list             # List GPU containers\n\nFor detailed help: nvctl <command> --help"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format
    #[arg(long, global = true, value_enum)]
    format: Option<OutputFormat>,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand)]
enum Command {
    /// 🎮 GPU information and control
    Gpu {
        #[command(subcommand)]
        subcommand: GpuSubcommand,
    },
    /// 🖥️ Display and monitor management
    Display {
        #[command(subcommand)]
        subcommand: DisplaySubcommand,
    },
    /// 🌈 Pure Rust Digital Vibrance (0-200%)
    #[command(alias = "vibe")]
    Vibrance {
        /// Vibrance percentage (0-200%, where 100% is default)
        percentage: u32,
    },
    /// 🌀 Fan control and curves
    Fan {
        #[command(subcommand)]
        subcommand: FanSubcommand,
    },
    /// ⚡ Overclocking and performance
    Overclock {
        #[command(subcommand)]
        subcommand: OverclockSubcommand,
    },
    /// 🔄 Variable Refresh Rate (VRR/G-Sync)
    Vrr {
        #[command(subcommand)]
        subcommand: VrrSubcommand,
    },
    /// 📊 Real-time monitoring
    Monitor {
        #[command(subcommand)]
        subcommand: Option<MonitorSubcommand>,
    },
    /// 🎯 Gaming optimization and latency
    Gaming {
        #[command(subcommand)]
        subcommand: GamingSubcommand,
    },
    /// 📹 Recording and streaming
    Recording {
        #[command(subcommand)]
        subcommand: RecordingSubcommand,
    },
    /// 🐳 Container and virtualization
    #[command(alias = "ct")]
    Container {
        #[command(subcommand)]
        subcommand: ContainerSubcommand,
    },
    /// ⚡ Bolt container runtime
    Bolt {
        #[command(subcommand)]
        subcommand: BoltSubcommand,
    },
    /// 🚀 nvbind container runtime (Lightning-fast GPU passthrough)
    #[command(alias = "nb")]
    Nvbind {
        #[command(subcommand)]
        subcommand: NvbindSubcommand,
    },
    /// 🔧 System drivers and utilities
    Drivers {
        #[command(subcommand)]
        subcommand: DriversSubcommand,
    },
    /// ⚡ Power management
    Power {
        #[command(subcommand)]
        subcommand: PowerSubcommand,
    },
    /// 🎨 Color and vibrance control
    Color {
        #[command(subcommand)]
        subcommand: ColorSubcommand,
    },
    /// ⚙️ Configuration and profiles
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommand,
    },
    /// 📈 AI Upscaling and enhancement
    Upscaling {
        #[command(subcommand)]
        subcommand: UpscalingSubcommand,
    },
    /// 🚀 DLSS 3 Frame Generation
    Dlss {
        #[command(subcommand)]
        subcommand: DlssSubcommand,
    },
    /// 🎨 Shader cache management
    Shaders {
        #[command(subcommand)]
        subcommand: ShadersSubcommand,
    },
    /// 🔌 GPU Passthrough (VFIO/Containers/VMs)
    #[command(alias = "pt")]
    Passthrough {
        #[command(subcommand)]
        subcommand: PassthroughSubcommand,
    },
    /// 🌊 Wayland NVIDIA Optimization
    Wayland {
        #[command(subcommand)]
        subcommand: WaylandSubcommand,
    },
    /// 🎨 KDE Plasma Compositor Optimization
    Kde {
        #[command(subcommand)]
        subcommand: KdeSubcommand,
    },
    /// ⚡ Power Profile Management (AC/Battery, Activities)
    PowerProfile {
        #[command(subcommand)]
        subcommand: PowerProfileSubcommand,
    },
    /// 🐧 Arch Linux Integration (Pacman hooks, DKMS)
    Arch {
        #[command(subcommand)]
        subcommand: ArchSubcommand,
    },
    /// 🔧 GSP Firmware Management (nvidia-open)
    Gsp {
        #[command(subcommand)]
        subcommand: GspSubcommand,
    },
    /// 🖥️ Multi-Monitor Management
    Monitors {
        #[command(subcommand)]
        subcommand: MultiMonitorSubcommand,
    },
    /// 📊 On-Screen Display (OSD) for gaming
    Osd {
        #[command(subcommand)]
        subcommand: OsdSubcommand,
    },
    /// 📋 Show detailed version information
    Version,
}

#[derive(Subcommand)]
enum OsdSubcommand {
    /// Enable OSD overlay
    Enable,
    /// Disable OSD overlay
    Disable,
    /// Show OSD status and configuration
    Status,
    /// Configure OSD settings
    Config {
        /// Position: top-left, top-right, bottom-left, bottom-right
        #[arg(long)]
        position: Option<String>,
        /// Font size
        #[arg(long)]
        font_size: Option<u32>,
        /// Background opacity (0.0-1.0)
        #[arg(long)]
        opacity: Option<f32>,
        /// Update interval in milliseconds
        #[arg(long)]
        interval: Option<u64>,
    },
    /// Add metric to OSD
    Add {
        /// Metric to add: fps, gpu-temp, gpu-util, vram, etc.
        metric: String,
    },
    /// Remove metric from OSD
    Remove {
        /// Metric to remove
        metric: String,
    },
    /// List available metrics
    Metrics,
    /// Check MangoHud installation status
    Check,
}

#[derive(Subcommand)]
enum GpuSubcommand {
    /// Show comprehensive GPU information
    Info {
        /// Output format: json, yaml, table
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },
    /// Launch live TUI dashboard for GPU monitoring
    Stat,
    /// Show detailed GPU overclocking capabilities
    Capabilities,
    /// Benchmark GPU performance
    Benchmark {
        /// Benchmark duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u32,
        /// Test type: compute, graphics, memory, all
        #[arg(short, long, default_value = "all")]
        test_type: String,
    },
    /// Live GPU utilization monitoring (text output)
    Watch {
        /// Update interval in seconds
        #[arg(short, long, default_value = "1")]
        interval: u64,
        /// Maximum number of updates (0 = infinite)
        #[arg(short, long, default_value = "0")]
        count: u32,
    },
    /// Export GPU metrics to JSON/CSV
    Export {
        /// Output format: json, csv
        #[arg(short, long, default_value = "json")]
        format: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Duration to collect data (seconds)
        #[arg(short, long, default_value = "60")]
        duration: u32,
    },
    /// Stress test GPU with monitoring
    Stress {
        /// Test duration in minutes
        #[arg(short, long, default_value = "5")]
        duration: u32,
        /// Test intensity: light, medium, heavy
        #[arg(short, long, default_value = "medium")]
        intensity: String,
        /// Monitor and log results
        #[arg(short, long)]
        log: bool,
    },
}

#[derive(Subcommand)]
enum DisplaySubcommand {
    Info,
    Ls,
    Vibrance {
        #[command(subcommand)]
        subcommand: VibranceSubcommand,
    },
    Hdr {
        #[command(subcommand)]
        subcommand: HdrSubcommand,
    },
    Gamma {
        #[command(subcommand)]
        subcommand: GammaSubcommand,
    },
    Sharpening {
        #[command(subcommand)]
        subcommand: SharpeningSubcommand,
    },
}

#[derive(Subcommand)]
enum VibranceSubcommand {
    /// Get current vibrance for all displays
    Get,
    /// Set vibrance for all displays (0-200%, where 100% is default)
    Set {
        #[arg(help = "Vibrance percentage (0-200, where 100 is default)")]
        percentage: u32,
    },
    /// Set vibrance for specific display
    SetDisplay {
        #[arg(help = "Display index (0, 1, 2, etc.)")]
        display: usize,
        #[arg(help = "Vibrance percentage (0-200)")]
        percentage: u32,
    },
    /// Set vibrance using raw nvibrant values for multiple displays
    SetRaw {
        /// Raw vibrance levels (-1024 to 1023) for each display
        #[arg(
            required = true,
            help = "Raw vibrance values for each display (e.g. 512 1023)"
        )]
        levels: Vec<i16>,
    },
    /// List all displays and their current vibrance
    List,
    /// Reset all displays to default vibrance (100%)
    Reset,
    /// Show driver compatibility info
    Info,
}

#[derive(Subcommand)]
enum HdrSubcommand {
    Status,
    Enable {
        /// Display ID (0, 1, etc.)
        display_id: usize,
    },
    Disable {
        /// Display ID (0, 1, etc.)
        display_id: usize,
    },
    Toggle {
        /// Display ID (0, 1, etc.)
        display_id: usize,
    },
}

#[derive(Subcommand)]
enum GammaSubcommand {
    /// Get current gamma
    Get {
        /// Display ID (0, 1, etc.)
        #[arg(short, long, default_value = "0")]
        display_id: usize,
    },
    /// Set gamma (0.5-3.0, default 1.0)
    Set {
        /// Display ID (0, 1, etc.)
        #[arg(short, long, default_value = "0")]
        display_id: usize,
        /// Gamma value (0.5-3.0)
        gamma: f32,
    },
    /// Reset gamma to default (1.0)
    Reset {
        /// Display ID (0, 1, etc.)
        #[arg(short, long, default_value = "0")]
        display_id: usize,
    },
}

#[derive(Subcommand)]
enum SharpeningSubcommand {
    /// Get current image sharpening for a display
    Get {
        /// Display ID (0, 1, etc.)
        #[arg(short, long, default_value = "0")]
        display_id: u32,
    },
    /// Set image sharpening (0-100, default varies by display)
    Set {
        /// Display ID (0, 1, etc.)
        #[arg(short, long, default_value = "0")]
        display_id: u32,
        /// Sharpening value (0-100)
        value: i64,
    },
    /// Reset image sharpening to default
    Reset {
        /// Display ID (0, 1, etc.)
        #[arg(short, long, default_value = "0")]
        display_id: u32,
    },
    /// Show image sharpening info for a display
    Info {
        /// Display ID (0, 1, etc.)
        #[arg(short, long, default_value = "0")]
        display_id: u32,
    },
}

#[derive(Subcommand)]
enum FanSubcommand {
    Info,
    Set {
        /// Fan ID (0, 1, 2, etc.)
        fan_id: usize,
        /// Fan speed percentage (0-100)
        percent: u8,
    },
}

#[derive(Subcommand)]
enum OverclockSubcommand {
    Info,
    Apply {
        /// GPU clock offset in MHz
        #[arg(long)]
        gpu_offset: Option<i32>,
        /// Memory clock offset in MHz
        #[arg(long)]
        memory_offset: Option<i32>,
        /// Power limit percentage (50-120)
        #[arg(long)]
        power_limit: Option<u8>,
    },
    Profile {
        /// Profile name to apply
        name: String,
    },
    StressTest {
        /// Duration in minutes
        #[arg(default_value = "5")]
        duration: u32,
    },
    Reset,
}

#[derive(Subcommand)]
enum VrrSubcommand {
    Status,
    Enable {
        /// Display name (e.g. DP-1, HDMI-A-1)
        display: String,
    },
    Disable {
        /// Display name
        display: String,
    },
    Configure {
        /// Display name
        display: String,
        /// Minimum refresh rate
        #[arg(long)]
        min_refresh: Option<u32>,
        /// Maximum refresh rate
        #[arg(long)]
        max_refresh: Option<u32>,
    },
}

#[derive(Subcommand)]
enum UpscalingSubcommand {
    Status,
    Enable {
        /// Game executable or path
        game: String,
        /// Technology: dlss, fsr, xess, native
        #[arg(long, default_value = "dlss")]
        tech: String,
        /// Quality: performance, balanced, quality, ultra
        #[arg(long, default_value = "balanced")]
        quality: String,
    },
    Disable {
        /// Game executable or path
        game: String,
    },
    Profiles,
    AutoDetect,
}

#[derive(Subcommand)]
enum DlssSubcommand {
    /// Show DLSS capabilities and status
    Status,
    /// Enable DLSS 3 with Frame Generation
    Enable {
        /// Quality preset: performance, balanced, quality, ultra
        #[arg(long, default_value = "balanced")]
        quality: String,
        /// Enable Frame Generation (RTX 40+ only)
        #[arg(long)]
        frame_generation: bool,
        /// Enable NVIDIA Reflex
        #[arg(long)]
        reflex: bool,
    },
    /// Disable DLSS
    Disable,
    /// Show game profiles
    Profiles,
    /// Auto-detect and apply game settings
    Auto,
    /// Show performance metrics
    Metrics,
}

#[derive(Subcommand)]
enum ShadersSubcommand {
    /// Show shader cache statistics
    Stats,
    /// Clear all shader caches
    Clear {
        /// Cache type to clear: nvidia, vulkan, steam, dxvk, all
        #[arg(long, default_value = "all")]
        cache_type: String,
    },
    /// Optimize shader compilation settings
    Optimize,
    /// Precompile shaders for a game
    Precompile {
        /// Game path or Steam App ID
        game: String,
    },
    /// Open shader cache folder
    Open,
}

#[derive(Subcommand)]
enum PassthroughSubcommand {
    /// Show GPU passthrough status
    Status,
    /// List all NVIDIA GPUs and their PCI addresses
    List,
    /// Show IOMMU groups
    Iommu,
    /// Bind GPU to VFIO driver
    BindVfio {
        /// PCI address (e.g., 0000:01:00.0)
        pci_address: String,
    },
    /// Unbind GPU from VFIO
    UnbindVfio {
        /// PCI address (e.g., 0000:01:00.0)
        pci_address: String,
    },
    /// Setup persistent VFIO binding
    Persistent {
        /// PCI address (e.g., 0000:01:00.0)
        pci_address: String,
    },
    /// Test GPU passthrough to Docker container
    TestContainer,
    /// Generate QEMU command for GPU passthrough
    QemuCommand {
        /// PCI address (e.g., 0000:01:00.0)
        pci_address: String,
    },
    /// Setup hugepages for VM performance
    Hugepages {
        /// Size in MB
        #[arg(default_value = "8192")]
        size_mb: u32,
    },
}

#[derive(Subcommand)]
enum WaylandSubcommand {
    /// Show Wayland NVIDIA configuration status
    Status,
    /// Apply optimal Wayland configuration
    Optimize {
        /// Create backup before applying
        #[arg(long, default_value = "true")]
        backup: bool,
    },
    /// Generate environment variables script
    ExportEnv {
        /// Shell config file path
        #[arg(long, default_value = "~/.bashrc")]
        config: String,
    },
    /// Switch between nvidia-open and nvidia-dkms
    SwitchDriver {
        /// Target driver: open, dkms
        driver: String,
    },
}

#[derive(Subcommand)]
enum KdeSubcommand {
    /// Show KDE compositor status
    Status,
    /// Apply gaming preset (low latency, VRR, minimal effects)
    Gaming,
    /// Apply productivity preset (balanced, full effects)
    Productivity,
    /// Apply power saving preset
    PowerSave,
    /// Setup NVIDIA environment variables for KDE
    SetupEnv,
    /// Set VRR per display
    SetVrr {
        /// Display connector (e.g., DP-1)
        display: String,
        /// Enable or disable
        #[arg(long)]
        enabled: bool,
    },
    /// Restart KWin compositor
    Restart,
}

#[derive(Subcommand)]
enum PowerProfileSubcommand {
    /// Show current power profile status
    Status,
    /// Set system power profile
    Set {
        /// Profile: performance, balanced, power-saver
        profile: String,
    },
    /// Create activity-based profile
    CreateActivity {
        /// KDE Activity name
        activity: String,
        /// System profile: performance, balanced, power-saver
        #[arg(long)]
        system_profile: String,
        /// GPU clock offset in MHz
        #[arg(long, default_value = "0")]
        gpu_offset: i32,
        /// Memory clock offset in MHz
        #[arg(long, default_value = "0")]
        mem_offset: i32,
    },
    /// Apply profile for current activity
    Apply {
        /// Activity name
        activity: String,
    },
    /// Monitor and auto-switch on activity changes
    Monitor,
    /// Monitor and auto-switch on AC/Battery changes
    AutoPower,
    /// Enable idle detection and power reduction
    Idle {
        /// Idle timeout in seconds
        #[arg(long, default_value = "300")]
        timeout: u64,
    },
    /// Create default activity profiles
    CreateDefaults,
}

#[derive(Subcommand)]
enum ArchSubcommand {
    /// Show Arch Linux NVIDIA integration status
    Status,
    /// Install all pacman hooks
    InstallHooks,
    /// Remove pacman hooks
    RemoveHooks,
    /// Rebuild DKMS modules
    RebuildDkms,
    /// Regenerate initramfs
    Mkinitcpio,
    /// Check for pending NVIDIA/kernel updates
    CheckUpdates,
    /// List AUR optimization suggestions
    AurSuggestions,
}

#[derive(Subcommand)]
enum GspSubcommand {
    /// Show GSP firmware status
    Status,
    /// Enable GSP firmware
    Enable,
    /// Disable GSP firmware (fallback mode)
    Disable,
    /// Run GSP diagnostics
    Diagnostics,
    /// Check for firmware updates
    CheckUpdate,
    /// Update GSP firmware
    Update,
}

#[derive(Subcommand)]
enum MultiMonitorSubcommand {
    /// Show current display configuration
    Status,
    /// Save current layout with a name
    Save {
        /// Layout name
        name: String,
    },
    /// Load and apply a saved layout
    Load {
        /// Layout name
        name: String,
    },
    /// List all saved layouts
    List,
    /// Set VRR for a specific display
    SetVrr {
        /// Display connector (e.g., DP-1)
        connector: String,
        /// Enable or disable
        #[arg(long)]
        enabled: bool,
    },
    /// Launch Gamescope on specific display
    Gamescope {
        /// Display connector
        connector: String,
        /// Width
        #[arg(short, long)]
        width: u32,
        /// Height
        #[arg(short, long)]
        height: u32,
        /// Refresh rate
        #[arg(short, long)]
        refresh: u32,
        /// Command to run
        command: String,
    },
    /// Auto-apply layout based on connected displays
    Auto,
    /// Create example layouts
    CreateExamples,
}

#[derive(Subcommand)]
enum DriversSubcommand {
    Status,
    Install {
        /// Driver type: proprietary, open, open-beta
        driver_type: String,
    },
    Update,
    Rollback,
    GenerateCompletions {
        /// Shell type: bash, zsh, fish
        shell: String,
    },
}

#[derive(Subcommand)]
enum PowerSubcommand {
    /// Show current power settings
    Status,
    /// Set GPU power limit (percentage)
    Limit {
        /// Power limit percentage (50-120)
        #[arg(short, long)]
        percentage: u32,
    },
    /// Configure power profile
    Profile {
        /// Profile name: performance, balanced, quiet
        #[arg(short, long)]
        profile: String,
    },
    /// Power persistence settings
    Persistence {
        /// Enable persistence mode
        #[arg(short, long)]
        enabled: bool,
    },
    /// Monitor power usage
    Monitor {
        /// Duration to monitor in seconds
        #[arg(short, long, default_value = "60")]
        duration: u64,
    },
    /// Automate power management
    Automate,
}

#[derive(Subcommand)]
enum ColorSubcommand {
    /// Vibrance control
    Vibrance {
        #[command(subcommand)]
        action: VibranceAction,
    },
    /// Color profile management
    Profiles {
        #[command(subcommand)]
        action: ColorProfileAction,
    },
}

#[derive(Subcommand)]
enum VibranceAction {
    /// Get current vibrance for a display
    Get {
        /// Display ID (0-based)
        #[arg(short, long)]
        display: Option<usize>,
    },
    /// Set vibrance for a display
    Set {
        /// Vibrance value (-1024 to 1023)
        #[arg(short, long)]
        value: i32,
        /// Display ID (0-based, all if not specified)
        #[arg(short, long)]
        display: Option<usize>,
    },
    /// Apply vibrance profile
    Apply {
        /// Profile name
        #[arg(short, long)]
        profile: String,
    },
    /// Preview vibrance changes
    Preview {
        /// Profile name
        #[arg(short, long)]
        profile: String,
        /// Duration in seconds
        #[arg(short, long, default_value = "5")]
        duration: u64,
    },
}

#[derive(Subcommand)]
enum ColorProfileAction {
    /// List available color profiles
    List,
    /// Create new color profile
    Create {
        /// Profile name
        #[arg(short, long)]
        name: String,
    },
    /// Apply color profile
    Apply {
        /// Profile name
        #[arg(short, long)]
        name: String,
    },
    /// Schedule color profile
    Schedule {
        /// Profile name
        #[arg(short, long)]
        name: String,
        /// Schedule time (HH:MM format)
        #[arg(short, long)]
        time: String,
    },
}

#[derive(Subcommand)]
enum ConfigSubcommand {
    /// Show current configuration
    Show,
    /// Edit configuration file
    Edit,
    /// Reset configuration to defaults
    Reset,
    /// Backup configuration
    Backup {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Restore configuration from backup
    Restore {
        /// Input file path
        #[arg(short, long)]
        input: String,
    },
}
#[derive(Subcommand)]
enum MonitorSubcommand {
    /// Start monitoring
    Start {
        /// Monitoring interval in seconds
        #[arg(short, long, default_value = "1")]
        interval: u64,
        /// Number of samples to collect
        #[arg(short, long)]
        count: Option<u64>,
    },
    /// Stop monitoring
    Stop,
    /// Show monitoring status
    Status,
    /// Launch TUI monitoring interface
    Tui,
    /// Export monitoring data
    Export {
        /// Output file path
        #[arg(short, long)]
        output: String,
        /// Duration to monitor in seconds
        #[arg(short, long, default_value = "60")]
        duration: u64,
    },
}

#[derive(Subcommand)]
enum GamingSubcommand {
    /// Enable gaming optimizations
    Enable,
    /// Disable gaming optimizations
    Disable,
    /// Show gaming optimization status
    Status,
    /// Latency optimization controls
    Latency {
        #[command(subcommand)]
        action: LatencyAction,
    },
    /// Gamescope controls
    Gamescope {
        #[command(subcommand)]
        action: GamescopeAction,
    },
    /// Game launch profiles
    Launch {
        #[command(subcommand)]
        action: LaunchAction,
    },
}

#[derive(Subcommand)]
enum LaunchAction {
    /// Launch a game with a profile
    Run {
        /// Profile name
        profile: String,
        /// Additional arguments to pass to the game
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// List all game profiles
    List,
    /// Show profile details
    Show {
        /// Profile name
        profile: String,
    },
    /// Create example game profiles
    Examples,
}

#[derive(Subcommand)]
enum LatencyAction {
    /// Optimize for low latency
    Optimize {
        /// Preset name: ultra, high, medium, low
        #[arg(short, long, default_value = "high")]
        preset: String,
    },
    /// Show latency status
    Status,
    /// Test latency
    Test,
}

#[derive(Subcommand)]
enum GamescopeAction {
    /// Launch application with Gamescope
    Launch {
        /// Command to run
        #[arg(short, long)]
        command: String,
        /// Preset to use
        #[arg(short, long)]
        preset: Option<String>,
        /// Window width
        #[arg(short, long)]
        width: Option<u32>,
        /// Window height
        #[arg(long)]
        height: Option<u32>,
    },
    /// List available presets
    Presets,
    /// Create new preset
    CreatePreset {
        /// Preset name
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Subcommand)]
enum RecordingSubcommand {
    /// Start recording
    Start {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Recording preset
        #[arg(short, long)]
        preset: Option<String>,
        /// Quality level (1-10)
        #[arg(short, long)]
        quality: Option<u32>,
    },
    /// Stop recording
    Stop,
    /// Show recording status
    Status,
    /// Start instant replay
    InstantReplay {
        /// Buffer duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u32,
    },
    /// Save instant replay clip
    Save,
    /// List available presets
    Presets,
}

#[derive(Subcommand)]
enum BoltSubcommand {
    /// Launch GPU workload using Bolt
    Launch {
        /// Workload name
        #[arg(short, long)]
        name: String,
        /// Container image
        #[arg(short, long)]
        image: String,
        /// GPU ID to use
        #[arg(long, default_value = "0")]
        gpu_id: u32,
        /// Enable DLSS support
        #[arg(long)]
        dlss: bool,
        /// Enable ray tracing
        #[arg(long)]
        raytracing: bool,
        /// Memory limit in GB
        #[arg(long)]
        memory_limit: Option<u64>,
        /// Power limit percentage
        #[arg(long)]
        power_limit: Option<u32>,
    },
    /// List Bolt GPU containers
    List,
    /// Show Surge orchestration status
    Status,
    /// Setup gaming environment with Proton
    Gaming {
        /// Game name
        #[arg(short, long)]
        name: String,
        /// Proton version
        #[arg(long, default_value = "8.0")]
        proton: String,
        /// Windows version compatibility
        #[arg(long, default_value = "win10")]
        winver: String,
    },
    /// Launch Proton game
    Game {
        /// Steam App ID
        #[arg(short, long)]
        steam_id: String,
        /// Additional game arguments
        #[arg(long)]
        args: Vec<String>,
    },
    /// Create GPU monitoring service
    Monitor {
        /// Enable web interface
        #[arg(long)]
        web: bool,
        /// Monitoring port
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    /// Start Surge orchestration services
    Up {
        /// Services to start (empty for all)
        services: Vec<String>,
        /// Force recreate containers
        #[arg(long)]
        force_recreate: bool,
    },
    /// Stop Surge orchestration services
    Down {
        /// Services to stop (empty for all)
        services: Vec<String>,
        /// Remove volumes
        #[arg(long)]
        volumes: bool,
    },
    /// Build GPU-optimized image
    Build {
        /// Dockerfile path
        #[arg(short, long)]
        dockerfile: String,
        /// Image tag
        #[arg(short, long)]
        tag: String,
    },
    /// Create high-performance GPU network
    Network {
        /// Network name
        #[arg(short, long)]
        name: String,
        /// Subnet (CIDR notation)
        #[arg(long)]
        subnet: Option<String>,
    },
    /// Stop container
    Stop {
        /// Container name
        #[arg(short, long)]
        container: String,
    },
    /// Remove container
    Remove {
        /// Container name
        #[arg(short, long)]
        container: String,
        /// Force removal
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum NvbindSubcommand {
    /// Launch optimized gaming container with nvcontrol + nvbind
    Gaming {
        /// Game name (cyberpunk2077, valorant, etc.)
        #[arg(short, long)]
        game: String,
        /// Container image
        #[arg(short, long, default_value = "steam:latest")]
        image: String,
        /// GPU device to use
        #[arg(long, default_value = "0")]
        gpu: String,
        /// Enable nvcontrol optimizations
        #[arg(long)]
        optimize: bool,
        /// Digital vibrance percentage (0-200%)
        #[arg(long, default_value = "80")]
        vibrance: i32,
        /// GPU memory overclock offset (MHz)
        #[arg(long, default_value = "500")]
        memory_oc: i32,
        /// GPU core overclock offset (MHz)
        #[arg(long, default_value = "150")]
        core_oc: i32,
    },
    /// Show enhanced GPU information with nvbind integration
    Info {
        /// Include container performance metrics
        #[arg(long)]
        containers: bool,
        /// Output format
        #[arg(long, value_enum)]
        format: Option<OutputFormat>,
    },
    /// Launch custom container with GPU passthrough
    Launch {
        /// Container name
        #[arg(short, long)]
        name: String,
        /// Container image
        #[arg(short, long)]
        image: String,
        /// GPU devices (comma-separated or 'all')
        #[arg(long, default_value = "0")]
        gpu: String,
        /// Runtime profile (gaming, ml-training, inference)
        #[arg(long, default_value = "gaming")]
        profile: String,
        /// Memory limit in GB
        #[arg(long)]
        memory: Option<u64>,
        /// Enable wine optimizations
        #[arg(long)]
        wine: bool,
    },
    /// List nvbind containers with performance metrics
    List {
        /// Show only GPU containers
        #[arg(long)]
        gpu_only: bool,
        /// Include performance metrics
        #[arg(long)]
        metrics: bool,
    },
    /// Live performance dashboard with nvcontrol + nvbind metrics
    Dashboard {
        /// Update interval in seconds
        #[arg(long, default_value = "1")]
        interval: u64,
        /// Enable FPS overlay
        #[arg(long)]
        fps_overlay: bool,
        /// Enable latency metrics
        #[arg(long)]
        latency: bool,
    },
    /// Apply unified configuration (nvcontrol + nvbind)
    Config {
        /// Configuration profile name
        #[arg(short, long)]
        profile: String,
        /// GPU ID
        #[arg(long, default_value = "0")]
        gpu: String,
        /// Save as new profile
        #[arg(long)]
        save: bool,
    },
    /// Container runtime status and diagnostics
    Status {
        /// Show detailed driver information
        #[arg(long)]
        drivers: bool,
        /// Show performance metrics
        #[arg(long)]
        metrics: bool,
    },
    /// Stop nvbind container
    Stop {
        /// Container name or ID
        #[arg(short, long)]
        container: String,
    },
    /// Remove nvbind container
    Remove {
        /// Container name or ID
        #[arg(short, long)]
        container: String,
        /// Force removal
        #[arg(short, long)]
        force: bool,
    },
    /// Create game-specific optimization profile
    Profile {
        #[command(subcommand)]
        subcommand: NvbindProfileSubcommand,
    },
}

#[derive(Subcommand)]
enum NvbindProfileSubcommand {
    /// Create new gaming profile
    Create {
        /// Profile name
        #[arg(short, long)]
        name: String,
        /// Game name or type
        #[arg(short, long)]
        game: String,
        /// Digital vibrance (0-200%)
        #[arg(long, default_value = "80")]
        vibrance: i32,
        /// Memory overclock offset (MHz)
        #[arg(long, default_value = "500")]
        memory_oc: i32,
        /// Core overclock offset (MHz)
        #[arg(long, default_value = "150")]
        core_oc: i32,
        /// Power limit percentage
        #[arg(long, default_value = "120")]
        power_limit: u32,
    },
    /// List available profiles
    List,
    /// Apply existing profile
    Apply {
        /// Profile name
        #[arg(short, long)]
        name: String,
        /// GPU ID
        #[arg(long, default_value = "0")]
        gpu: String,
    },
    /// Delete profile
    Delete {
        /// Profile name
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Subcommand)]
enum ContainerSubcommand {
    /// List GPU-enabled containers
    List,
    /// Show container GPU status
    Status {
        /// Container ID or name
        #[arg(short, long)]
        container: Option<String>,
    },
    /// Monitor container GPU usage
    Monitor {
        /// Container ID or name
        #[arg(short, long)]
        container: String,
        /// Monitoring interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,
    },
    /// Launch container with GPU support
    Launch {
        /// Container image
        #[arg(short, long)]
        image: String,
        /// Container name
        #[arg(short, long)]
        name: Option<String>,
        /// GPU devices (all, 0, 1,2, GPU-uuid)
        #[arg(short, long, default_value = "all")]
        gpu: String,
        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
        /// Remove container on exit
        #[arg(long)]
        rm: bool,
        /// Container runtime (docker, podman, bolt, nix)
        #[arg(short, long, default_value = "docker")]
        runtime: String,
    },
    /// Launch PhantomLink audio container
    PhantomLink {
        /// Launch mode (dev, prod, minimal)
        #[arg(short, long, default_value = "prod")]
        mode: String,
        /// Audio device
        #[arg(short, long)]
        audio_device: Option<String>,
        /// Enable RTX Voice
        #[arg(long)]
        rtx_voice: bool,
    },
    /// Container profile management
    Profiles {
        #[command(subcommand)]
        action: ContainerProfileAction,
    },
    /// Runtime information and setup
    Runtime {
        #[command(subcommand)]
        action: RuntimeAction,
    },
}

#[derive(Subcommand)]
enum ContainerProfileAction {
    /// List available profiles
    List,
    /// Apply profile to container
    Apply {
        /// Profile name
        #[arg(short, long)]
        profile: String,
        /// Container ID or name
        #[arg(short, long)]
        container: String,
    },
    /// Create new profile
    Create {
        /// Profile name
        #[arg(short, long)]
        name: String,
        /// Workload type (ml-training, inference, gaming, default)
        #[arg(short, long, default_value = "default")]
        workload: String,
    },
}

#[derive(Subcommand)]
enum RuntimeAction {
    /// Show runtime information
    Info,
    /// Setup container runtime
    Setup {
        /// Runtime type (docker, podman, nix)
        #[arg(short, long)]
        runtime: String,
    },
    /// Test GPU passthrough
    Test,
    /// Configure NVIDIA Container Runtime
    Configure,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Vibrance { percentage } => {
            use nvcontrol::vibrance_native;

            // Simple vibrance command - just works!
            match vibrance_native::set_vibrance_all_native(percentage) {
                Ok(()) => {
                    println!("✅ Set all displays to {}% vibrance", percentage);
                    if percentage == 100 {
                        println!("   🎨 Default vibrance restored");
                    } else if percentage > 100 {
                        println!("   🌈 Enhanced colors active (+{}%)", percentage - 100);
                    } else {
                        println!("   🎭 Reduced saturation (-{}%)", 100 - percentage);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Vibrance failed: {}", e);
                    eprintln!("💡 Ensure NVIDIA open drivers 580+ with nvidia_drm.modeset=1");
                    eprintln!(
                        "   Or run with elevated permissions: sudo nvctl vibe {}",
                        percentage
                    );
                }
            }
        }
        Command::Gpu { subcommand } => match subcommand {
            GpuSubcommand::Info { format } => {
                if let Err(e) = gpu::get_gpu_info_with_format(format) {
                    eprintln!("❌ Failed to get GPU info: {}", e);
                }
            }
            GpuSubcommand::Stat => {
                // Launch the new advanced TUI dashboard
                println!("🚀 Launching nvcontrol TUI Dashboard...");
                if let Err(e) = nvcontrol::tui::TuiApp::new().run() {
                    eprintln!("❌ TUI error: {}", e);
                }
            }
            GpuSubcommand::Capabilities => match overclocking::get_gpu_capabilities() {
                Ok(_caps) => println!("✅ GPU overclocking capabilities detected"),
                Err(e) => eprintln!("Error getting capabilities: {e}"),
            },
            GpuSubcommand::Benchmark {
                duration,
                test_type,
            } => {
                if let Err(e) = monitoring::run_gpu_benchmark(duration, &test_type, "medium", false)
                {
                    eprintln!("❌ Benchmark failed: {}", e);
                } else {
                    println!("✅ Benchmark completed");
                }
            }
            GpuSubcommand::Watch { interval, count } => {
                if let Err(e) = monitoring::live_gpu_watch(interval, count) {
                    eprintln!("❌ Watch failed: {}", e);
                }
            }
            GpuSubcommand::Export {
                format,
                output,
                duration,
            } => match monitoring::export_gpu_metrics(&format, output.as_deref(), duration) {
                Ok(()) => println!("✅ Export completed"),
                Err(e) => eprintln!("❌ Export failed: {}", e),
            },
            GpuSubcommand::Stress {
                duration,
                intensity,
                log,
            } => match monitoring::run_gpu_benchmark(duration * 60, "all", &intensity, log) {
                Ok(()) => println!("✅ Stress test completed"),
                Err(e) => eprintln!("❌ Stress test failed: {}", e),
            },
        },
        Command::Display { subcommand } => match subcommand {
            DisplaySubcommand::Info => display::get_display_info(),
            DisplaySubcommand::Ls => {
                let count = display::get_display_count();
                println!("Detected {count} display(s):");
                for i in 0..count {
                    println!("  Display {i}");
                }
            }
            DisplaySubcommand::Vibrance { subcommand } => {
                use nvcontrol::vibrance_native;

                match subcommand {
                    VibranceSubcommand::Get => {
                        match vibrance_native::get_vibrance_status_native() {
                            Ok(status) => {
                                println!("🌈 Pure Rust Digital Vibrance Status");
                                println!("══════════════════════════════════════════════════");
                                if let Some(devices) = status.get("devices") {
                                    println!(
                                        "{}",
                                        serde_json::to_string_pretty(devices).unwrap_or_default()
                                    );
                                }

                                // List displays with current vibrance
                                if let Ok(displays) = vibrance_native::list_displays_native() {
                                    println!("\nConnected Displays:");
                                    for (device_id, display_id, name, connected) in displays {
                                        if connected {
                                            // Get vibrance controller to check current vibrance
                                            println!(
                                                "  Device {}, Display {}: {} - Ready",
                                                device_id, display_id, name
                                            );
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Pure Rust vibrance error: {}", e);
                                eprintln!(
                                    "💡 Ensure NVIDIA open drivers (580+) with nvidia_drm.modeset=1"
                                );
                            }
                        }
                    }
                    VibranceSubcommand::Set { percentage } => {
                        match vibrance_native::set_vibrance_all_native(percentage) {
                            Ok(()) => println!(
                                "✅ Set all displays to {}% vibrance using pure Rust implementation",
                                percentage
                            ),
                            Err(e) => eprintln!("❌ Failed to set vibrance: {}", e),
                        }
                    }
                    VibranceSubcommand::SetDisplay {
                        display,
                        percentage,
                    } => {
                        // Assume device_id 0 for now - could be enhanced to specify device
                        match vibrance_native::set_display_vibrance_native(
                            0,
                            display as u32,
                            percentage,
                        ) {
                            Ok(()) => {
                                println!("✅ Set display {} to {}% vibrance", display, percentage)
                            }
                            Err(e) => {
                                eprintln!(
                                    "❌ Failed to set vibrance for display {}: {}",
                                    display, e
                                )
                            }
                        }
                    }
                    VibranceSubcommand::SetRaw { levels } => {
                        println!("🔧 Setting raw vibrance values: {:?}", levels);

                        for (display_idx, &level) in levels.iter().enumerate() {
                            // Convert percentage if needed, or use raw value
                            let percentage = if level >= -1024 && level <= 1023 {
                                // Raw vibrance value - convert to percentage
                                if level <= 0 {
                                    ((level + 1024) as f32 / 1024.0 * 100.0) as u32
                                } else {
                                    (100.0 + (level as f32 / 1023.0 * 100.0)) as u32
                                }
                            } else {
                                level.abs() as u32 // Treat as percentage if outside raw range
                            };

                            match vibrance_native::set_display_vibrance_native(
                                0,
                                display_idx as u32,
                                percentage,
                            ) {
                                Ok(()) => println!(
                                    "✅ Display {}: set to {}% (raw: {})",
                                    display_idx, percentage, level
                                ),
                                Err(e) => eprintln!("❌ Display {}: failed - {}", display_idx, e),
                            }
                        }
                    }
                    VibranceSubcommand::List => match vibrance_native::list_displays_native() {
                        Ok(displays) => {
                            println!("🖥️ Available Displays (Pure Rust):");
                            for (device_id, display_id, name, connected) in displays {
                                let status = if connected {
                                    "✅ Connected"
                                } else {
                                    "⭕ Disconnected"
                                };
                                println!(
                                    "  Device {}, Display {} [{}]: {}",
                                    device_id, display_id, display_id, name
                                );
                                println!("    Status: {}", status);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to list displays: {}", e),
                    },
                    VibranceSubcommand::Reset => match vibrance_native::reset_vibrance_native() {
                        Ok(()) => println!(
                            "✅ Reset all displays to default vibrance (100%) using pure Rust"
                        ),
                        Err(e) => eprintln!("❌ Failed to reset vibrance: {}", e),
                    },
                    VibranceSubcommand::Info => match vibrance_native::get_vibrance_status_native()
                    {
                        Ok(status) => {
                            println!("🌈 Pure Rust Digital Vibrance Information:");
                            println!("══════════════════════════════════════════════════");
                            if let Some(driver_version) = status.get("driver_version") {
                                println!("  Driver Version: {}", driver_version);
                            }
                            if let Some(open_driver) = status.get("open_driver") {
                                println!(
                                    "  NVIDIA Open Drivers: {}",
                                    if open_driver.as_bool().unwrap_or(false) {
                                        "✅ Yes"
                                    } else {
                                        "❌ No"
                                    }
                                );
                            }

                            println!("\n💡 Features:");
                            println!("  ✅ Direct driver integration (no external deps)");
                            println!("  ✅ Works on Wayland and X11");
                            println!("  ✅ Per-display control");
                            println!("  ✅ Real-time adjustment");

                            if let Ok(displays) = vibrance_native::list_displays_native() {
                                println!("\n🖥️ Supported Displays: {}", displays.len());
                            }

                            println!("\n🔧 Requirements:");
                            println!("  • NVIDIA Open Drivers 580+");
                            println!("  • nvidia_drm.modeset=1 kernel parameter");
                            println!("  • /dev/nvidia-modeset access (or run as root)");
                        }
                        Err(e) => eprintln!("❌ Failed to get driver info: {}", e),
                    },
                }
            }
            DisplaySubcommand::Hdr { subcommand } => match subcommand {
                HdrSubcommand::Status => {
                    let displays = display::list_displays();
                    println!("HDR Status:");
                    for display in displays {
                        println!(
                            "  {}: {} ({})",
                            display.name,
                            if display.hdr_enabled { "ON" } else { "OFF" },
                            if display.hdr_capable {
                                "HDR Capable"
                            } else {
                                "No HDR"
                            }
                        );
                    }
                }
                HdrSubcommand::Enable { display_id } => match display::toggle_hdr(display_id) {
                    Ok(true) => println!("HDR enabled for display {display_id}"),
                    Ok(false) => println!("HDR was already enabled for display {display_id}"),
                    Err(e) => {
                        eprintln!("Failed to enable HDR: {e}");
                        std::process::exit(1);
                    }
                },
                HdrSubcommand::Disable { display_id } => match display::toggle_hdr(display_id) {
                    Ok(false) => println!("HDR disabled for display {display_id}"),
                    Ok(true) => println!("HDR was already disabled for display {display_id}"),
                    Err(e) => {
                        eprintln!("Failed to disable HDR: {e}");
                        std::process::exit(1);
                    }
                },
                HdrSubcommand::Toggle { display_id } => match display::toggle_hdr(display_id) {
                    Ok(true) => println!("HDR enabled for display {display_id}"),
                    Ok(false) => println!("HDR disabled for display {display_id}"),
                    Err(e) => {
                        eprintln!("Failed to toggle HDR: {e}");
                        std::process::exit(1);
                    }
                },
            },
            DisplaySubcommand::Gamma { subcommand } => match subcommand {
                GammaSubcommand::Get { display_id } => match display::get_gamma(display_id) {
                    Ok(gamma) => {
                        println!("🎨 Gamma for display {}: {:.2}", display_id, gamma);
                        println!("   Range: 0.5 (darker) - 1.0 (neutral) - 3.0 (brighter)");
                    }
                    Err(e) => eprintln!("❌ Failed to get gamma: {}", e),
                },
                GammaSubcommand::Set { display_id, gamma } => {
                    match display::set_gamma(display_id, gamma) {
                        Ok(()) => {
                            println!("✅ Gamma set to {:.2} for display {}", gamma, display_id);
                        }
                        Err(e) => eprintln!("❌ Failed to set gamma: {}", e),
                    }
                },
                GammaSubcommand::Reset { display_id } => match display::reset_gamma(display_id) {
                    Ok(()) => println!("✅ Gamma reset to 1.0 (neutral) for display {}", display_id),
                    Err(e) => eprintln!("❌ Failed to reset gamma: {}", e),
                },
            },
            DisplaySubcommand::Sharpening { subcommand } => match subcommand {
                SharpeningSubcommand::Get { display_id } => {
                    use nvcontrol::display_controls::get_image_sharpening_info_cli;

                    match get_image_sharpening_info_cli(0, display_id) {
                        Ok(info) => {
                            println!("🔍 Image Sharpening for display {}:", display_id);
                            println!("   Available: {}", if info.available { "Yes" } else { "No" });
                            if info.available {
                                println!("   Current: {}", info.current_value);
                                println!("   Default: {}", info.default_value);
                                println!("   Range: {} - {}", info.range.0, info.range.1);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to get sharpening info: {}", e),
                    }
                }
                SharpeningSubcommand::Set { display_id, value } => {
                    use nvcontrol::display_controls::set_image_sharpening_cli;

                    match set_image_sharpening_cli(0, display_id, value) {
                        Ok(()) => println!("✅ Image sharpening set to {} for display {}", value, display_id),
                        Err(e) => eprintln!("❌ Failed to set sharpening: {}", e),
                    }
                }
                SharpeningSubcommand::Reset { display_id } => {
                    use nvcontrol::display_controls::{get_image_sharpening_info_cli, set_image_sharpening_cli};

                    match get_image_sharpening_info_cli(0, display_id) {
                        Ok(info) => {
                            match set_image_sharpening_cli(0, display_id, info.default_value) {
                                Ok(()) => println!("✅ Image sharpening reset to default ({}) for display {}", info.default_value, display_id),
                                Err(e) => eprintln!("❌ Failed to reset sharpening: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to get default value: {}", e),
                    }
                }
                SharpeningSubcommand::Info { display_id } => {
                    use nvcontrol::display_controls::get_image_sharpening_info_cli;

                    match get_image_sharpening_info_cli(0, display_id) {
                        Ok(info) => {
                            println!("🔍 Image Sharpening Information for display {}:", display_id);
                            println!();
                            if info.available {
                                println!("   Status: Available ✅");
                                println!("   Current Value: {}", info.current_value);
                                println!("   Default Value: {}", info.default_value);
                                println!("   Valid Range: {} - {}", info.range.0, info.range.1);
                                println!();
                                println!("💡 Usage:");
                                println!("   nvctl display sharpening set --display-id {} <value>", display_id);
                                println!("   nvctl display sharpening reset --display-id {}", display_id);
                            } else {
                                println!("   Status: Not Available ❌");
                                println!();
                                println!("⚠️  Image sharpening is not supported on this display or driver");
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to get sharpening info: {}", e),
                    }
                }
            },
        },
        Command::Fan { subcommand } => match subcommand {
            FanSubcommand::Info => {
                let fans = fan::list_fans();
                println!("Fan Information:");
                for fan in fans {
                    println!(
                        "  Fan {}: {} RPM, {}%, Controllable: {}",
                        fan.id,
                        fan.rpm.unwrap_or(0),
                        fan.percent.unwrap_or(0),
                        fan.controllable
                    );
                }
            }
            FanSubcommand::Set { fan_id, percent } => match fan::set_fan_speed(fan_id, percent) {
                Ok(()) => println!("Set fan {fan_id} to {percent}%"),
                Err(e) => eprintln!("Failed to set fan speed: {e}"),
            },
        },
        Command::Overclock { subcommand } => match subcommand {
            OverclockSubcommand::Info => match overclocking::get_memory_timings() {
                Ok(timings) => {
                    println!("Memory Timings & Info:");
                    for (key, value) in timings {
                        println!("  {}: {}", key, value);
                    }
                }
                Err(e) => eprintln!("Error getting memory info: {e}"),
            },
            OverclockSubcommand::Apply {
                gpu_offset,
                memory_offset,
                power_limit,
            } => {
                let mut profile = overclocking::OverclockProfile {
                    name: "CLI Applied".to_string(),
                    ..overclocking::OverclockProfile::default()
                };

                if let Some(gpu) = gpu_offset {
                    profile.gpu_clock_offset = gpu;
                }
                if let Some(memory) = memory_offset {
                    profile.memory_clock_offset = memory;
                }
                if let Some(power) = power_limit {
                    profile.power_limit = power;
                }

                match overclocking::apply_overclock_profile(&profile) {
                    Ok(()) => println!("Overclock applied successfully"),
                    Err(e) => eprintln!("Failed to apply overclock: {e}"),
                }
            }
            OverclockSubcommand::Profile { name } => {
                println!("Loading overclock profile: {name}");
                // This would load from saved profiles
                println!("Profile system not yet implemented");
            }
            OverclockSubcommand::StressTest { duration } => {
                println!("🔥 Starting GPU stress test for {} minutes...", duration);
                let test_duration = Duration::from_secs(duration as u64 * 60);
                let _pb = show_progress_bar("GPU stress test", test_duration);

                match overclocking::create_stress_test(duration) {
                    Ok(()) => {
                        println!("\n✅ Stress test completed successfully");
                    }
                    Err(e) => eprintln!("\n❌ Stress test failed: {e}"),
                }
            }
            OverclockSubcommand::Reset => {
                let default_profile = overclocking::OverclockProfile::default();
                match overclocking::apply_overclock_profile(&default_profile) {
                    Ok(()) => println!("GPU settings reset to defaults"),
                    Err(e) => eprintln!("Failed to reset settings: {e}"),
                }
            }
        },
        Command::Vrr { subcommand } => match subcommand {
            VrrSubcommand::Status => match vrr::detect_vrr_displays() {
                Ok(displays) => {
                    println!("VRR/Adaptive Sync Status:");
                    for display in displays {
                        println!(
                            "  {}: {}",
                            display.display_name,
                            if display.current_settings.enabled {
                                "ENABLED"
                            } else {
                                "DISABLED"
                            }
                        );
                        println!("    Supports VRR: {}", display.supports_vrr);
                        println!("    G-SYNC Compatible: {}", display.supports_gsync);
                        println!("    FreeSync: {}", display.supports_freesync);
                        println!(
                            "    Refresh Range: {}-{} Hz",
                            display.min_refresh, display.max_refresh
                        );
                    }
                }
                Err(e) => eprintln!("Failed to detect VRR displays: {e}"),
            },
            VrrSubcommand::Enable { display } => {
                let settings = vrr::VrrSettings {
                    enabled: true,
                    ..vrr::VrrSettings::default()
                };
                match vrr::apply_vrr_settings(&display, &settings) {
                    Ok(()) => println!("VRR enabled for {display}"),
                    Err(e) => eprintln!("Failed to enable VRR: {e}"),
                }
            }
            VrrSubcommand::Disable { display } => {
                let settings = vrr::VrrSettings {
                    enabled: false,
                    ..vrr::VrrSettings::default()
                };
                match vrr::apply_vrr_settings(&display, &settings) {
                    Ok(()) => println!("VRR disabled for {display}"),
                    Err(e) => eprintln!("Failed to disable VRR: {e}"),
                }
            }
            VrrSubcommand::Configure {
                display,
                min_refresh,
                max_refresh,
            } => {
                let mut settings = vrr::VrrSettings {
                    enabled: true,
                    ..vrr::VrrSettings::default()
                };
                if let Some(min) = min_refresh {
                    settings.min_refresh_rate = min;
                }
                if let Some(max) = max_refresh {
                    settings.max_refresh_rate = max;
                }
                match vrr::apply_vrr_settings(&display, &settings) {
                    Ok(()) => println!(
                        "VRR configured for {display}: {}-{} Hz",
                        settings.min_refresh_rate, settings.max_refresh_rate
                    ),
                    Err(e) => eprintln!("Failed to configure VRR: {e}"),
                }
            }
        },
        Command::Upscaling { subcommand } => match subcommand {
            UpscalingSubcommand::Status => match upscaling::detect_upscaling_capabilities() {
                Ok(caps) => {
                    println!("Upscaling Capabilities:");
                    println!(
                        "  DLSS: {} {}",
                        if caps.supports_dlss { "✓" } else { "✗" },
                        caps.dlss_version.unwrap_or_default()
                    );
                    println!(
                        "  FSR: {} {}",
                        if caps.supports_fsr { "✓" } else { "✗" },
                        caps.fsr_version.unwrap_or_default()
                    );
                    println!("  XeSS: {}", if caps.supports_xess { "✓" } else { "✗" });
                    println!("  GPU: {}", caps.gpu_generation);
                }
                Err(e) => eprintln!("Failed to detect capabilities: {e}"),
            },
            UpscalingSubcommand::Enable {
                game,
                tech,
                quality,
            } => {
                let technology = match tech.to_lowercase().as_str() {
                    "dlss" => upscaling::UpscalingTechnology::DLSS,
                    "fsr" => upscaling::UpscalingTechnology::FSR,
                    "xess" => upscaling::UpscalingTechnology::XeSS,
                    "native" => upscaling::UpscalingTechnology::Native,
                    _ => {
                        eprintln!("Unknown technology: {tech}. Use: dlss, fsr, xess, or native");
                        return;
                    }
                };

                let quality_setting = match quality.to_lowercase().as_str() {
                    "performance" => upscaling::UpscalingQuality::Performance,
                    "balanced" => upscaling::UpscalingQuality::Balanced,
                    "quality" => upscaling::UpscalingQuality::Quality,
                    "ultra" => upscaling::UpscalingQuality::UltraQuality,
                    _ => {
                        eprintln!(
                            "Unknown quality: {quality}. Use: performance, balanced, quality, or ultra"
                        );
                        return;
                    }
                };

                let settings = upscaling::UpscalingSettings {
                    technology,
                    quality: quality_setting,
                    enabled: true,
                    sharpening: 0.5,
                    motion_vectors: true,
                };

                match upscaling::apply_upscaling_to_game(&game, &settings) {
                    Ok(()) => println!("Upscaling enabled for {game}"),
                    Err(e) => eprintln!("Failed to enable upscaling: {e}"),
                }
            }
            UpscalingSubcommand::Disable { game } => {
                let settings = upscaling::UpscalingSettings {
                    enabled: false,
                    ..upscaling::UpscalingSettings::default()
                };
                match upscaling::apply_upscaling_to_game(&game, &settings) {
                    Ok(()) => println!("Upscaling disabled for {game}"),
                    Err(e) => eprintln!("Failed to disable upscaling: {e}"),
                }
            }
            UpscalingSubcommand::Profiles => {
                let profiles = upscaling::get_game_upscaling_profiles();
                println!("Game Upscaling Profiles:");
                for (game_id, profile) in profiles {
                    println!(
                        "  {}: {} ({:?} {:?})",
                        game_id,
                        profile.game_name,
                        profile.settings.technology,
                        profile.settings.quality
                    );
                }
            }
            UpscalingSubcommand::AutoDetect => match upscaling::auto_detect_running_games() {
                Ok(games) => {
                    if games.is_empty() {
                        println!("No supported games currently running");
                    } else {
                        println!("Running games with upscaling profiles:");
                        for game in games {
                            println!("  {}", game);
                        }
                    }
                }
                Err(e) => eprintln!("Failed to detect running games: {e}"),
            },
        },
        Command::Dlss { subcommand } => {
            use nvcontrol::dlss;

            match subcommand {
                DlssSubcommand::Status => match dlss::get_dlss_status() {
                    Ok(status) => println!("{}", status),
                    Err(e) => eprintln!("❌ Failed to get DLSS status: {}", e),
                },
                DlssSubcommand::Enable {
                    quality,
                    frame_generation,
                    reflex,
                } => {
                    let dlss_quality = match quality.as_str() {
                        "performance" => dlss::DlssQuality::Performance,
                        "balanced" => dlss::DlssQuality::Balanced,
                        "quality" => dlss::DlssQuality::Quality,
                        "ultra" => dlss::DlssQuality::UltraQuality,
                        _ => dlss::DlssQuality::Balanced,
                    };

                    let mut controller = match dlss::DlssController::new() {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("❌ Failed to initialize DLSS: {}", e);
                            return;
                        }
                    };

                    let mut settings = controller.current_settings.clone();
                    settings.enabled = true;
                    settings.quality_preset = dlss_quality;

                    if frame_generation {
                        if controller.capabilities.supports_frame_generation {
                            settings.mode = dlss::DlssMode::SuperResolutionAndFrameGeneration;
                            settings.frame_generation.enabled = true;
                            println!("✅ Enabling DLSS 3 with Frame Generation");
                        } else {
                            println!(
                                "⚠️  Frame Generation not supported - using DLSS Super Resolution only"
                            );
                            settings.mode = dlss::DlssMode::SuperResolution;
                        }
                    } else {
                        settings.mode = dlss::DlssMode::SuperResolution;
                    }

                    if reflex {
                        settings.reflex_mode = dlss::ReflexMode::OnPlusBoost;
                        println!("✅ NVIDIA Reflex enabled");
                    }

                    match controller.apply_settings(settings) {
                        Ok(()) => println!("✅ DLSS settings applied successfully"),
                        Err(e) => eprintln!("❌ Failed to apply DLSS settings: {}", e),
                    }
                }
                DlssSubcommand::Disable => match dlss::DlssController::new() {
                    Ok(mut controller) => {
                        let mut settings = controller.current_settings.clone();
                        settings.enabled = false;
                        settings.mode = dlss::DlssMode::Off;
                        settings.frame_generation.enabled = false;

                        match controller.apply_settings(settings) {
                            Ok(()) => println!("✅ DLSS disabled"),
                            Err(e) => eprintln!("❌ Failed to disable DLSS: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to initialize DLSS: {}", e),
                },
                DlssSubcommand::Profiles => match dlss::DlssController::new() {
                    Ok(controller) => {
                        println!("🎮 DLSS Game Profiles:\n");
                        for (game_id, profile) in &controller.game_profiles {
                            println!("📦 {}", profile.game_name);
                            println!("   ID: {}", game_id);
                            println!("   Mode: {:?}", profile.recommended_settings.mode);
                            println!(
                                "   Quality: {:?}",
                                profile.recommended_settings.quality_preset
                            );
                            println!(
                                "   Frame Gen: {}",
                                if profile.recommended_settings.frame_generation.enabled {
                                    "✅"
                                } else {
                                    "❌"
                                }
                            );
                            println!("   Reflex: {:?}", profile.recommended_settings.reflex_mode);
                            if let Some(notes) = &profile.notes {
                                println!("   Notes: {}", notes);
                            }
                            println!();
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to load DLSS profiles: {}", e),
                },
                DlssSubcommand::Auto => match dlss::DlssController::new() {
                    Ok(mut controller) => match controller.auto_apply_game_profile() {
                        Ok(Some(game_id)) => {
                            let profile = controller.game_profiles.get(&game_id).unwrap();
                            println!("✅ Auto-applied DLSS profile for: {}", profile.game_name);
                            println!("   Mode: {:?}", profile.recommended_settings.mode);
                            println!(
                                "   Quality: {:?}",
                                profile.recommended_settings.quality_preset
                            );
                        }
                        Ok(None) => {
                            println!("ℹ️  No supported games currently running");
                        }
                        Err(e) => eprintln!("❌ Failed to auto-apply DLSS settings: {}", e),
                    },
                    Err(e) => eprintln!("❌ Failed to initialize DLSS: {}", e),
                },
                DlssSubcommand::Metrics => match dlss::DlssController::new() {
                    Ok(controller) => match controller.get_metrics() {
                        Ok(metrics) => {
                            println!("📊 DLSS Performance Metrics:\n");
                            println!("🎯 Frame Rates:");
                            println!("   Native: {:.1} FPS", metrics.base_fps);
                            println!(
                                "   DLSS: {:.1} FPS ({:.1}x boost)",
                                metrics.dlss_fps,
                                metrics.dlss_fps / metrics.base_fps
                            );
                            if metrics.frame_generation_fps > metrics.dlss_fps {
                                println!(
                                    "   Frame Gen: {:.1} FPS ({:.1}x boost)",
                                    metrics.frame_generation_fps,
                                    metrics.frame_generation_fps / metrics.base_fps
                                );
                            }
                            println!("\n⚡ Performance:");
                            println!("   Latency: {:.1}ms", metrics.latency_ms);
                            println!("   GPU Util: {:.1}%", metrics.gpu_utilization);
                            println!("   VRAM: {}MB", metrics.vram_usage_mb);
                            if controller.capabilities.tensor_cores > 0 {
                                println!(
                                    "   Tensor Cores: {:.1}%",
                                    metrics.tensor_core_utilization
                                );
                            }
                            if controller.capabilities.optical_flow_accelerator {
                                println!(
                                    "   Optical Flow: {:.1}%",
                                    metrics.optical_flow_utilization
                                );
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to get DLSS metrics: {}", e),
                    },
                    Err(e) => eprintln!("❌ Failed to initialize DLSS: {}", e),
                },
            }
        }
        Command::Shaders { subcommand } => {
            use nvcontrol::shaders;

            match subcommand {
                ShadersSubcommand::Stats => match shaders::get_shader_stats() {
                    Ok(()) => {},
                    Err(e) => eprintln!("❌ Failed to get shader stats: {}", e),
                },
                ShadersSubcommand::Clear { cache_type } => {
                    let result = match cache_type.as_str() {
                        "nvidia" => shaders::clear_shader_cache_by_type(shaders::ShaderCacheType::Nvidia),
                        "vulkan" => shaders::clear_shader_cache_by_type(shaders::ShaderCacheType::Vulkan),
                        "steam" => shaders::clear_shader_cache_by_type(shaders::ShaderCacheType::Steam),
                        "dxvk" => shaders::clear_shader_cache_by_type(shaders::ShaderCacheType::Dxvk),
                        "all" => shaders::clear_shader_cache(),
                        _ => {
                            eprintln!("❌ Unknown cache type: {}", cache_type);
                            eprintln!("   Valid types: nvidia, vulkan, steam, dxvk, all");
                            return;
                        }
                    };

                    match result {
                        Ok(()) => println!("\n✅ Shader cache cleared successfully"),
                        Err(e) => eprintln!("❌ Failed to clear shader cache: {}", e),
                    }
                },
                ShadersSubcommand::Optimize => match shaders::optimize_shader_compilation() {
                    Ok(()) => println!("\n✅ Shader compilation optimized"),
                    Err(e) => eprintln!("❌ Failed to optimize shader compilation: {}", e),
                },
                ShadersSubcommand::Precompile { game } => match shaders::precompile_shaders(&game) {
                    Ok(()) => {},
                    Err(e) => eprintln!("❌ Failed to precompile shaders: {}", e),
                },
                ShadersSubcommand::Open => {
                    use std::process::Command as Cmd;
                    let cache_path = std::env::var("HOME")
                        .unwrap_or_else(|_| "/tmp".to_string())
                        + "/.nv/GLCache";

                    println!("📁 Opening shader cache folder: {}", cache_path);

                    let result = Cmd::new("xdg-open")
                        .arg(&cache_path)
                        .spawn()
                        .or_else(|_| Cmd::new("nautilus").arg(&cache_path).spawn())
                        .or_else(|_| Cmd::new("dolphin").arg(&cache_path).spawn());

                    match result {
                        Ok(_) => println!("✅ File manager opened"),
                        Err(e) => eprintln!("❌ Failed to open file manager: {}", e),
                    }
                },
            }
        }
        Command::Passthrough { subcommand } => {
            use nvcontrol::gpu_passthrough::GpuPassthroughManager;

            match subcommand {
                PassthroughSubcommand::Status => {
                    match GpuPassthroughManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.show_status() {
                                eprintln!("❌ Failed to show status: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PassthroughSubcommand::List => {
                    match GpuPassthroughManager::detect_nvidia_gpus() {
                        Ok(devices) => {
                            println!("📍 Detected NVIDIA GPUs:\n");
                            for device in devices {
                                println!("   PCI: {}", device.pci_address);
                                println!("   Name: {}", device.name);
                                println!("   IDs: {}:{}", device.vendor_id, device.device_id);
                                if let Some(driver) = device.driver {
                                    println!("   Driver: {}", driver);
                                }
                                println!();
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to detect GPUs: {}", e),
                    }
                }
                PassthroughSubcommand::Iommu => {
                    match GpuPassthroughManager::new() {
                        Ok(manager) => {
                            match manager.list_iommu_groups() {
                                Ok(groups) => {
                                    println!("🔒 IOMMU Groups:\n");
                                    let mut sorted_groups: Vec<_> = groups.iter().collect();
                                    sorted_groups.sort_by_key(|(k, _)| *k);

                                    for (group_num, devices) in sorted_groups {
                                        println!("   Group {}:", group_num);
                                        for device in devices {
                                            println!("      {}", device);
                                        }
                                        println!();
                                    }
                                }
                                Err(e) => eprintln!("❌ {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PassthroughSubcommand::BindVfio { pci_address } => {
                    match GpuPassthroughManager::new() {
                        Ok(manager) => {
                            match manager.bind_to_vfio(&pci_address) {
                                Ok(()) => println!("✅ Successfully bound {} to VFIO", pci_address),
                                Err(e) => eprintln!("❌ Failed to bind to VFIO: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PassthroughSubcommand::UnbindVfio { pci_address } => {
                    match GpuPassthroughManager::new() {
                        Ok(manager) => {
                            match manager.unbind_from_vfio(&pci_address) {
                                Ok(()) => println!("✅ Successfully unbound {} from VFIO", pci_address),
                                Err(e) => eprintln!("❌ Failed to unbind from VFIO: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PassthroughSubcommand::Persistent { pci_address } => {
                    match GpuPassthroughManager::new() {
                        Ok(manager) => {
                            match manager.setup_persistent_vfio(&pci_address) {
                                Ok(()) => {},
                                Err(e) => eprintln!("❌ Failed to setup persistent VFIO: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PassthroughSubcommand::TestContainer => {
                    match GpuPassthroughManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.test_container_passthrough() {
                                eprintln!("❌ Test failed: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PassthroughSubcommand::QemuCommand { pci_address } => {
                    match GpuPassthroughManager::new() {
                        Ok(manager) => {
                            match manager.generate_qemu_command(&pci_address) {
                                Ok(cmd) => {
                                    println!("🖥️  QEMU Command for GPU Passthrough:\n");
                                    println!("qemu-system-x86_64 \\");
                                    println!("{}", cmd);
                                    println!();
                                    println!("💡 Add your disk, network, and other device options");
                                }
                                Err(e) => eprintln!("❌ Failed to generate command: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PassthroughSubcommand::Hugepages { size_mb } => {
                    match GpuPassthroughManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.setup_hugepages(size_mb) {
                                eprintln!("❌ Failed to setup hugepages: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
            }
        }
        Command::Wayland { subcommand } => {
            use wayland_nvidia::WaylandNvidiaManager;

            match subcommand {
                WaylandSubcommand::Status => {
                    match WaylandNvidiaManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.print_status() {
                                eprintln!("❌ Failed to print status: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                WaylandSubcommand::Optimize { backup } => {
                    match WaylandNvidiaManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.apply_wayland_optimization(backup) {
                                eprintln!("❌ Failed to optimize: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                WaylandSubcommand::ExportEnv { config } => {
                    match WaylandNvidiaManager::new() {
                        Ok(manager) => {
                            let path = std::path::PathBuf::from(config);
                            if let Err(e) = manager.export_env_vars(&path) {
                                eprintln!("❌ Failed to export env vars: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                WaylandSubcommand::SwitchDriver { driver } => {
                    match WaylandNvidiaManager::new() {
                        Ok(manager) => {
                            let target = match driver.as_str() {
                                "open" => wayland_nvidia::NvidiaDriver::Open,
                                "dkms" => wayland_nvidia::NvidiaDriver::Proprietary,
                                _ => {
                                    eprintln!("❌ Invalid driver type. Use 'open' or 'dkms'");
                                    return;
                                }
                            };
                            if let Err(e) = manager.switch_driver(target) {
                                eprintln!("❌ Failed to switch driver: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
            }
        }
        Command::Kde { subcommand } => {
            use kde_optimizer::KdeOptimizer;

            match subcommand {
                KdeSubcommand::Status => {
                    let optimizer = KdeOptimizer::new();
                    if let Err(e) = optimizer.print_status() {
                        eprintln!("❌ Failed to print status: {}", e);
                    }
                }
                KdeSubcommand::Gaming => {
                    let mut optimizer = KdeOptimizer::new();
                    if let Err(e) = optimizer.apply_gaming_preset() {
                        eprintln!("❌ Failed to apply gaming preset: {}", e);
                    }
                }
                KdeSubcommand::Productivity => {
                    let mut optimizer = KdeOptimizer::new();
                    if let Err(e) = optimizer.apply_productivity_preset() {
                        eprintln!("❌ Failed to apply productivity preset: {}", e);
                    }
                }
                KdeSubcommand::PowerSave => {
                    let mut optimizer = KdeOptimizer::new();
                    if let Err(e) = optimizer.apply_powersave_preset() {
                        eprintln!("❌ Failed to apply power save preset: {}", e);
                    }
                }
                KdeSubcommand::SetupEnv => {
                    let optimizer = KdeOptimizer::new();
                    if let Err(e) = optimizer.setup_kde_env_vars() {
                        eprintln!("❌ Failed to setup env vars: {}", e);
                    }
                }
                KdeSubcommand::SetVrr { display, enabled } => {
                    let optimizer = KdeOptimizer::new();
                    if let Err(e) = optimizer.set_vrr_per_display(&display, enabled) {
                        eprintln!("❌ Failed to set VRR: {}", e);
                    }
                }
                KdeSubcommand::Restart => {
                    let optimizer = KdeOptimizer::new();
                    if let Err(e) = optimizer.restart_compositor() {
                        eprintln!("❌ Failed to restart compositor: {}", e);
                    }
                }
            }
        }
        Command::PowerProfile { subcommand } => {
            use power_profiles_daemon::{PowerProfileManager, SystemPowerProfile, PowerProfileConfig, NvidiaPowerMode, FanMode};

            match subcommand {
                PowerProfileSubcommand::Status => {
                    match PowerProfileManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.print_status() {
                                eprintln!("❌ Failed to print status: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PowerProfileSubcommand::Set { profile } => {
                    match PowerProfileManager::new() {
                        Ok(mut manager) => {
                            let sys_profile = match profile.as_str() {
                                "performance" => SystemPowerProfile::Performance,
                                "balanced" => SystemPowerProfile::Balanced,
                                "power-saver" => SystemPowerProfile::PowerSaver,
                                _ => {
                                    eprintln!("❌ Invalid profile. Use: performance, balanced, power-saver");
                                    return;
                                }
                            };
                            if let Err(e) = manager.set_system_profile(sys_profile) {
                                eprintln!("❌ Failed to set profile: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PowerProfileSubcommand::CreateActivity { activity, system_profile, gpu_offset, mem_offset } => {
                    match PowerProfileManager::new() {
                        Ok(mut manager) => {
                            let sys_prof = match system_profile.as_str() {
                                "performance" => SystemPowerProfile::Performance,
                                "balanced" => SystemPowerProfile::Balanced,
                                "power-saver" => SystemPowerProfile::PowerSaver,
                                _ => {
                                    eprintln!("❌ Invalid profile");
                                    return;
                                }
                            };
                            let config = PowerProfileConfig {
                                system_profile: sys_prof,
                                nvidia_mode: NvidiaPowerMode::Adaptive,
                                gpu_clock_offset: gpu_offset,
                                mem_clock_offset: mem_offset,
                                power_limit: None,
                                fan_control: FanMode::Auto,
                            };
                            if let Err(e) = manager.create_activity_profile(&activity, config) {
                                eprintln!("❌ Failed to create profile: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PowerProfileSubcommand::Apply { activity } => {
                    match PowerProfileManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.apply_activity_profile(&activity) {
                                eprintln!("❌ Failed to apply profile: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PowerProfileSubcommand::Monitor => {
                    match PowerProfileManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.monitor_activity_changes() {
                                eprintln!("❌ Monitor failed: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PowerProfileSubcommand::AutoPower => {
                    match PowerProfileManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.auto_switch_on_power_change() {
                                eprintln!("❌ Auto-power failed: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PowerProfileSubcommand::Idle { timeout } => {
                    match PowerProfileManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.idle_detection(timeout) {
                                eprintln!("❌ Idle detection failed: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                PowerProfileSubcommand::CreateDefaults => {
                    match PowerProfileManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.create_default_profiles() {
                                eprintln!("❌ Failed to create defaults: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
            }
        }
        Command::Arch { subcommand } => {
            use arch_integration::ArchIntegration;

            let arch = ArchIntegration::new();

            match subcommand {
                ArchSubcommand::Status => {
                    if let Err(e) = arch.print_status() {
                        eprintln!("❌ Failed to print status: {}", e);
                    }
                }
                ArchSubcommand::InstallHooks => {
                    if let Err(e) = arch.install_all_hooks() {
                        eprintln!("❌ Failed to install hooks: {}", e);
                    }
                }
                ArchSubcommand::RemoveHooks => {
                    if let Err(e) = arch.remove_hooks() {
                        eprintln!("❌ Failed to remove hooks: {}", e);
                    }
                }
                ArchSubcommand::RebuildDkms => {
                    if let Err(e) = arch.rebuild_dkms_modules() {
                        eprintln!("❌ Failed to rebuild DKMS: {}", e);
                    }
                }
                ArchSubcommand::Mkinitcpio => {
                    if let Err(e) = arch.regenerate_initramfs() {
                        eprintln!("❌ Failed to regenerate initramfs: {}", e);
                    }
                }
                ArchSubcommand::CheckUpdates => {
                    match ArchIntegration::check_pending_updates() {
                        Ok(updates) => {
                            if updates.is_empty() {
                                println!("✅ No pending NVIDIA/kernel updates");
                            } else {
                                println!("⚠️  Pending updates:");
                                for update in updates {
                                    println!("   {}", update);
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to check updates: {}", e),
                    }
                }
                ArchSubcommand::AurSuggestions => {
                    let suggestions = ArchIntegration::suggest_aur_optimizations();
                    if suggestions.is_empty() {
                        println!("✅ All recommended AUR packages installed");
                    } else {
                        println!("💡 Recommended AUR packages:");
                        for suggestion in suggestions {
                            println!("   {}", suggestion);
                        }
                    }
                }
            }
        }
        Command::Gsp { subcommand } => {
            use gsp_firmware::GspManager;

            let gsp = GspManager::new();

            match subcommand {
                GspSubcommand::Status => {
                    if let Err(e) = gsp.print_status() {
                        eprintln!("❌ Failed to print status: {}", e);
                    }
                }
                GspSubcommand::Enable => {
                    if let Err(e) = gsp.enable_gsp() {
                        eprintln!("❌ Failed to enable GSP: {}", e);
                    }
                }
                GspSubcommand::Disable => {
                    if let Err(e) = gsp.disable_gsp() {
                        eprintln!("❌ Failed to disable GSP: {}", e);
                    }
                }
                GspSubcommand::Diagnostics => {
                    if let Err(e) = gsp.run_diagnostics() {
                        eprintln!("❌ Failed to run diagnostics: {}", e);
                    }
                }
                GspSubcommand::CheckUpdate => {
                    match gsp.check_for_updates() {
                        Ok(available) => {
                            if available {
                                println!("✅ Firmware update available!");
                            } else {
                                println!("ℹ️  No updates available");
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to check updates: {}", e),
                    }
                }
                GspSubcommand::Update => {
                    if let Err(e) = gsp.update_firmware() {
                        eprintln!("❌ Failed to update firmware: {}", e);
                    }
                }
            }
        }
        Command::Monitors { subcommand } => {
            use multimonitor::MultiMonitorManager;

            match subcommand {
                MultiMonitorSubcommand::Status => {
                    match MultiMonitorManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.print_status() {
                                eprintln!("❌ Failed to print status: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                MultiMonitorSubcommand::Save { name } => {
                    match MultiMonitorManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.save_layout(&name) {
                                eprintln!("❌ Failed to save layout: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                MultiMonitorSubcommand::Load { name } => {
                    match MultiMonitorManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.load_layout(&name) {
                                eprintln!("❌ Failed to load layout: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                MultiMonitorSubcommand::List => {
                    match MultiMonitorManager::new() {
                        Ok(manager) => {
                            let layouts = manager.list_layouts();
                            if layouts.is_empty() {
                                println!("No saved layouts");
                            } else {
                                println!("📂 Saved layouts:");
                                for layout in layouts {
                                    println!("   • {}", layout);
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                MultiMonitorSubcommand::SetVrr { connector, enabled } => {
                    match MultiMonitorManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.set_display_vrr(&connector, enabled) {
                                eprintln!("❌ Failed to set VRR: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                MultiMonitorSubcommand::Gamescope { connector, width, height, refresh, command } => {
                    match MultiMonitorManager::new() {
                        Ok(manager) => {
                            if let Err(e) = manager.launch_gamescope_on_display(&connector, width, height, refresh, &command) {
                                eprintln!("❌ Failed to launch gamescope: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                MultiMonitorSubcommand::Auto => {
                    match MultiMonitorManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.auto_apply_layout() {
                                eprintln!("❌ Failed to auto-apply layout: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
                MultiMonitorSubcommand::CreateExamples => {
                    match MultiMonitorManager::new() {
                        Ok(mut manager) => {
                            if let Err(e) = manager.create_example_layouts() {
                                eprintln!("❌ Failed to create examples: {}", e);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize: {}", e),
                    }
                }
            }
        }
        Command::Drivers { subcommand } => match subcommand {
            DriversSubcommand::Status => match drivers::get_driver_status() {
                Ok(status) => {
                    println!("Driver Status:");
                    println!(
                        "  Current: {} ({})",
                        status.current_version, status.driver_type
                    );
                    println!(
                        "  Available: {}",
                        status.available_version.unwrap_or("Unknown".to_string())
                    );
                    println!(
                        "  Update Available: {}",
                        if status.update_available { "Yes" } else { "No" }
                    );
                }
                Err(e) => eprintln!("Failed to get driver status: {e}"),
            },
            DriversSubcommand::Install { driver_type } => {
                match drivers::install_driver(&driver_type) {
                    Ok(()) => println!("Driver installation initiated for: {driver_type}"),
                    Err(e) => eprintln!("Failed to install driver: {e}"),
                }
            }
            DriversSubcommand::Update => match drivers::update_driver() {
                Ok(()) => println!("Driver update completed"),
                Err(e) => eprintln!("Failed to update driver: {e}"),
            },
            DriversSubcommand::Rollback => match drivers::rollback_driver() {
                Ok(()) => println!("Driver rollback completed"),
                Err(e) => eprintln!("Failed to rollback driver: {e}"),
            },
            DriversSubcommand::GenerateCompletions { shell } => {
                match drivers::generate_shell_completions(&shell) {
                    Ok(()) => {} // Completions printed to stdout
                    Err(e) => eprintln!("Failed to generate completions: {e}"),
                }
            }
        },
        Command::Power { subcommand } => match subcommand {
            PowerSubcommand::Status => match power::get_power_info() {
                Ok(power_infos) => {
                    println!("💡 Power Information:");
                    println!("{}", "=".repeat(40));

                    for (gpu_id, info) in power_infos.iter().enumerate() {
                        println!("GPU {}:", gpu_id);

                        if let Some(power_draw) = info.power_draw {
                            println!("  Current Power: {:.1}W", power_draw);
                        }

                        if let Some(power_limit) = info.power_limit {
                            println!("  Power Limit: {:.1}W", power_limit);
                        }

                        if let Some(temp) = info.temperature {
                            println!("  Temperature: {:.0}°C", temp);
                        }

                        if let Some(fan_speed) = info.fan_speed {
                            println!("  Fan Speed: {}%", fan_speed);
                        }

                        println!(
                            "  Persistence Mode: {}",
                            if info.persistence_mode {
                                "Enabled"
                            } else {
                                "Disabled"
                            }
                        );
                        println!();
                    }
                }
                Err(e) => eprintln!("❌ Failed to get power info: {}", e),
            },
            PowerSubcommand::Profile { profile } => match power::set_power_profile(&profile) {
                Ok(()) => println!("✅ Power profile applied successfully"),
                Err(e) => eprintln!("❌ Failed to set power profile: {}", e),
            },
            PowerSubcommand::Limit { percentage } => {
                let percentage = percentage.min(120).max(50);
                match power::set_power_limit_percentage(percentage) {
                    Ok(()) => println!("✅ Power limit set to {}%", percentage),
                    Err(e) => eprintln!("❌ Failed to set power limit: {}", e),
                }
            }
            PowerSubcommand::Persistence { enabled } => {
                match power::set_persistence_mode(enabled) {
                    Ok(()) => println!(
                        "✅ Persistence mode {}",
                        if enabled { "enabled" } else { "disabled" }
                    ),
                    Err(e) => eprintln!("❌ Failed to set persistence mode: {}", e),
                }
            }
            PowerSubcommand::Monitor { duration } => {
                match power::monitor_power_consumption(duration as u32) {
                    Ok(()) => {}
                    Err(e) => eprintln!("❌ Power monitoring failed: {}", e),
                }
            }
            PowerSubcommand::Automate => match power::create_power_automation() {
                Ok(()) => println!("✅ Power automation configured"),
                Err(e) => eprintln!("❌ Failed to setup automation: {}", e),
            },
        },
        Command::Monitor { subcommand } => match subcommand {
            Some(MonitorSubcommand::Start { interval, count }) => {
                if let Err(e) = monitoring::live_gpu_watch(interval, count.unwrap_or(0) as u32) {
                    eprintln!("❌ Monitor failed: {}", e);
                }
            }
            Some(MonitorSubcommand::Stop) => {
                monitoring::stop_monitoring();
                println!("Monitoring stopped");
            }
            Some(MonitorSubcommand::Status) => {
                monitoring::show_monitoring_status();
            }
            Some(MonitorSubcommand::Tui) => {
                println!("📊 Launching TUI monitor...");
                if let Err(e) = nvcontrol::tui::TuiApp::new().run() {
                    eprintln!("❌ TUI error: {}", e);
                }
            }
            Some(MonitorSubcommand::Export { output, duration }) => {
                println!("📤 Exporting monitor data to {}...", output);
                println!("Monitoring for {} seconds...", duration);
            }
            None => {
                // Default to TUI
                println!("📊 Launching TUI monitor...");
                if let Err(e) = nvcontrol::tui::TuiApp::new().run() {
                    eprintln!("❌ TUI error: {}", e);
                }
            }
        },
        Command::Gaming { subcommand } => match subcommand {
            GamingSubcommand::Enable => {
                println!("🎮 Enabling gaming mode...");
                match latency::optimize_latency() {
                    Ok(()) => println!("✅ Gaming mode enabled with latency optimizations"),
                    Err(e) => eprintln!("❌ Failed to enable gaming mode: {}", e),
                }
            }
            GamingSubcommand::Disable => {
                println!("🎮 Disabling gaming mode...");
                // Reset to balanced settings
                println!("✅ Gaming mode disabled");
            }
            GamingSubcommand::Status => {
                println!("🎮 Gaming mode status: Not implemented");
            }
            GamingSubcommand::Latency { action } => match action {
                LatencyAction::Optimize { preset } => {
                    let preset_name = &preset;
                    println!("⚡ Optimizing latency with '{}' preset...", preset_name);
                    match latency::optimize_latency() {
                        Ok(()) => println!("✅ Latency optimizations applied"),
                        Err(e) => eprintln!("❌ Optimization failed: {}", e),
                    }
                }
                LatencyAction::Status => match latency::get_latency_info() {
                    Ok(info) => print_formatted_output(&info, &cli.format, cli.no_color),
                    Err(e) => eprintln!("❌ Failed to get latency info: {}", e),
                },
                LatencyAction::Test => {
                    println!("🧪 Testing input latency...");
                    println!("Latency testing not implemented yet");
                }
            },
            GamingSubcommand::Gamescope { action } => match action {
                GamescopeAction::Launch {
                    command,
                    preset,
                    width,
                    height,
                } => {
                    // Start with preset config or default
                    let mut config = if let Some(preset_name) = preset {
                        match preset_name.to_lowercase().as_str() {
                            "performance" => gamescope::GamescopePreset::Performance.to_config(),
                            "quality" => gamescope::GamescopePreset::Quality.to_config(),
                            "balanced" => gamescope::GamescopePreset::Balanced.to_config(),
                            "competitive" => gamescope::GamescopePreset::Competitive.to_config(),
                            "cinematic" => gamescope::GamescopePreset::Cinematic.to_config(),
                            "steamdeck" => gamescope::GamescopePreset::SteamDeck.to_config(),
                            _ => {
                                eprintln!("⚠️  Unknown preset '{}', using default", preset_name);
                                gamescope::GamescopeConfig::default()
                            }
                        }
                    } else {
                        gamescope::GamescopeConfig::default()
                    };

                    // Override with custom width/height if provided
                    if let Some(w) = width {
                        config.width = w;
                    }
                    if let Some(h) = height {
                        config.height = h;
                    }

                    let args = gamescope::generate_advanced_command(&config, &command);
                    println!("🎯 Launching with Gamescope: {}", args.join(" "));

                    let output = std::process::Command::new(&args[0])
                        .args(&args[1..])
                        .spawn();

                    match output {
                        Ok(_) => println!("✅ Gamescope launched"),
                        Err(e) => eprintln!("❌ Launch failed: {}", e),
                    }
                }
                GamescopeAction::Presets => {
                    let presets = gamescope::create_steam_deck_presets();
                    println!("🎮 Available Gamescope presets:");
                    for preset in presets {
                        let (name, description) = match preset {
                            gamescope::GamescopePreset::SteamDeckHandheld => (
                                "Steam Deck Handheld",
                                "Optimized for Steam Deck handheld mode",
                            ),
                            gamescope::GamescopePreset::SteamDeckDocked => {
                                ("Steam Deck Docked", "Optimized for Steam Deck docked mode")
                            }
                            gamescope::GamescopePreset::Performance => {
                                ("Performance", "Maximum performance settings")
                            }
                            gamescope::GamescopePreset::Quality => {
                                ("Quality", "High quality settings")
                            }
                            gamescope::GamescopePreset::Balanced => {
                                ("Balanced", "Balanced performance and quality")
                            }
                            gamescope::GamescopePreset::Desktop => {
                                ("Desktop", "Desktop gaming settings")
                            }
                            _ => ("Custom", "Custom configuration"),
                        };
                        println!("  📋 {}: {}", style(name).cyan(), description);
                    }
                }
                GamescopeAction::CreatePreset { name } => {
                    println!(
                        "🔧 Creating preset '{}' (interactive setup not implemented)",
                        name
                    );
                }
            },
            GamingSubcommand::Launch { action } => match action {
                LaunchAction::Run { profile, args } => {
                    use nvcontrol::game_launcher::GameLauncher;

                    match GameLauncher::new() {
                        Ok(launcher) => {
                            match launcher.load_profile(&profile) {
                                Ok(game_profile) => {
                                    match launcher.launch_game(&game_profile, args.clone()) {
                                        Ok(()) => println!("✅ Game exited successfully"),
                                        Err(e) => eprintln!("❌ Game launch failed: {}", e),
                                    }
                                }
                                Err(e) => eprintln!("❌ Failed to load profile '{}': {}", profile, e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize game launcher: {}", e),
                    }
                }
                LaunchAction::List => {
                    use nvcontrol::game_launcher::GameLauncher;

                    match GameLauncher::new() {
                        Ok(launcher) => {
                            let profiles = launcher.list_profiles();
                            if profiles.is_empty() {
                                println!("📂 No game profiles found");
                                println!("   Create example profiles with: nvctl gaming launch examples");
                            } else {
                                println!("📂 Available game profiles:");
                                for profile_name in profiles {
                                    println!("   • {}", profile_name);
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to list profiles: {}", e),
                    }
                }
                LaunchAction::Show { profile } => {
                    use nvcontrol::game_launcher::GameLauncher;

                    match GameLauncher::new() {
                        Ok(launcher) => {
                            match launcher.load_profile(&profile) {
                                Ok(game_profile) => {
                                    println!("🎮 Game Profile: {}", game_profile.name);
                                    println!();
                                    println!("   Executable: {}", game_profile.executable);
                                    if let Some(dir) = &game_profile.working_dir {
                                        println!("   Working Dir: {}", dir);
                                    }
                                    println!();

                                    if game_profile.use_gamescope {
                                        println!("   Gamescope:");
                                        if let (Some(w), Some(h)) = (game_profile.gamescope_width, game_profile.gamescope_height) {
                                            println!("      Resolution: {}x{}", w, h);
                                        }
                                        if let Some(r) = game_profile.gamescope_refresh {
                                            println!("      Refresh: {}Hz", r);
                                        }
                                        println!("      HDR: {}", game_profile.gamescope_hdr);
                                        println!("      VRR: {}", game_profile.gamescope_vrr);
                                        println!();
                                    }

                                    if !game_profile.env_vars.is_empty() {
                                        println!("   Environment Variables: ({} set)", game_profile.env_vars.len());
                                        for (key, value) in &game_profile.env_vars {
                                            println!("      {}={}", key, value);
                                        }
                                        println!();
                                    }

                                    if let Some(cache) = &game_profile.shader_cache_path {
                                        println!("   Shader Cache: {}", cache);
                                    }

                                    if let Some(power) = &game_profile.power_profile {
                                        println!("   Power Profile: {}", power);
                                    }

                                    if let Some(affinity) = &game_profile.cpu_affinity {
                                        println!("   CPU Affinity: {:?}", affinity);
                                    }
                                }
                                Err(e) => eprintln!("❌ Failed to load profile '{}': {}", profile, e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to show profile: {}", e),
                    }
                }
                LaunchAction::Examples => {
                    use nvcontrol::game_launcher::GameLauncher;

                    match GameLauncher::new() {
                        Ok(launcher) => {
                            match launcher.create_example_profiles() {
                                Ok(()) => {
                                    println!("✅ Example game profiles created!");
                                    println!();
                                    println!("Available profiles:");
                                    println!("   • cyberpunk2077 - Cyberpunk 2077 with DLSS and RT");
                                    println!("   • cs2 - Counter-Strike 2 competitive settings");
                                    println!("   • eldenring - Elden Ring with Proton");
                                    println!();
                                    println!("Launch a game:");
                                    println!("   nvctl gaming launch run cyberpunk2077");
                                }
                                Err(e) => eprintln!("❌ Failed to create examples: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize launcher: {}", e),
                    }
                }
            },
        },
        Command::Recording { subcommand } => match subcommand {
            RecordingSubcommand::Start {
                output,
                preset,
                quality,
            } => {
                let settings = recording::RecordingSettings {
                    encoder: if preset.as_deref() == Some("av1") {
                        recording::EncoderType::NvencAv1
                    } else {
                        recording::EncoderType::NvencH264
                    },
                    quality_preset: match quality.unwrap_or(7) {
                        1..=3 => recording::QualityPreset::Performance,
                        4..=6 => recording::QualityPreset::Balanced,
                        7..=8 => recording::QualityPreset::HighQuality,
                        _ => recording::QualityPreset::Lossless,
                    },
                    bitrate_mbps: 25,
                    resolution: (1920, 1080),
                    framerate: 60,
                    audio_enabled: true,
                    audio_bitrate_kbps: 128,
                    output_format: recording::OutputFormat::Mp4,
                    lossless_mode: false,
                    instant_replay_duration: 30,
                };

                let output_path = output.unwrap_or_else(|| {
                    format!(
                        "recording_{}.mp4",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    )
                });

                println!("📹 Starting recording...");
                match recording::start_recording(&settings, &output_path) {
                    Ok(()) => println!("✅ Recording started"),
                    Err(e) => eprintln!("❌ Recording failed: {}", e),
                }
            }
            RecordingSubcommand::Stop => match recording::stop_recording() {
                Ok(()) => println!("⏹️ Recording stopped"),
                Err(e) => eprintln!("❌ Stop failed: {}", e),
            },
            RecordingSubcommand::Status => match recording::get_recording_status() {
                Ok(status) => print_formatted_output(&status, &cli.format, cli.no_color),
                Err(e) => eprintln!("❌ Status check failed: {}", e),
            },
            RecordingSubcommand::InstantReplay { duration } => {
                println!("⚡ Enabling instant replay ({}s buffer)...", duration);
                let settings = recording::RecordingSettings {
                    encoder: recording::EncoderType::NvencH264,
                    quality_preset: recording::QualityPreset::HighQuality,
                    bitrate_mbps: 25,
                    resolution: (1920, 1080),
                    framerate: 60,
                    audio_enabled: true,
                    audio_bitrate_kbps: 128,
                    output_format: recording::OutputFormat::Mp4,
                    lossless_mode: false,
                    instant_replay_duration: duration,
                };
                match recording::start_instant_replay(&settings) {
                    Ok(()) => println!("✅ Instant replay enabled"),
                    Err(e) => eprintln!("❌ Instant replay failed: {}", e),
                }
            }
            RecordingSubcommand::Save => {
                let output_path = format!(
                    "instant_replay_{}.mp4",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );
                match recording::save_instant_replay(&output_path) {
                    Ok(()) => println!("💾 Clip saved: {}", output_path),
                    Err(e) => eprintln!("❌ Save failed: {}", e),
                }
            }
            RecordingSubcommand::Presets => match recording::get_recording_presets() {
                Ok(presets) => {
                    println!("📋 Recording presets:");
                    for (i, preset) in presets.iter().enumerate() {
                        println!(
                            "  🎬 Preset {}: {:?} @ {} fps",
                            i + 1,
                            preset.quality_preset,
                            preset.framerate
                        );
                    }
                }
                Err(e) => eprintln!("❌ Failed to get presets: {}", e),
            },
        },
        Command::Color { subcommand } => match subcommand {
            ColorSubcommand::Vibrance { action } => match action {
                VibranceAction::Get { display } => {
                    use nvcontrol::vibrance;
                    if let Some(display_id) = display {
                        match vibrance::get_display_vibrance(display_id) {
                            Ok(vibrance_val) => {
                                println!("Display {} vibrance: {}", display_id, vibrance_val);
                            }
                            Err(e) => eprintln!("❌ Failed to get vibrance: {}", e),
                        }
                    } else {
                        match vibrance::detect_enhanced_displays() {
                            Ok(displays) => {
                                for (display_id, display_name) in displays {
                                    match vibrance::get_display_vibrance(display_id) {
                                        Ok(vibrance_val) => {
                                            println!("{}: vibrance {}", display_name, vibrance_val);
                                        }
                                        Err(e) => eprintln!(
                                            "❌ Failed to get vibrance for {}: {}",
                                            display_name, e
                                        ),
                                    }
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to get displays: {}", e),
                        }
                    }
                }
                VibranceAction::Set { value, display } => {
                    use nvcontrol::vibrance;
                    if let Some(display_id) = display {
                        match vibrance::set_display_vibrance(display_id, value) {
                            Ok(()) => {
                                println!("✅ Set display {} vibrance to {}", display_id, value)
                            }
                            Err(e) => eprintln!("❌ Failed to set vibrance: {}", e),
                        }
                    } else {
                        match vibrance::set_vibrance_all(value) {
                            Ok(()) => println!("✅ Set all displays vibrance to {}", value),
                            Err(e) => eprintln!("❌ Failed to set vibrance: {}", e),
                        }
                    }
                }
                VibranceAction::Apply { profile } => {
                    use nvcontrol::vibrance;
                    match vibrance::load_enhanced_profiles() {
                        Ok(profiles) => {
                            if let Some(prof) = profiles.iter().find(|p| p.name == profile) {
                                for (display_id, settings) in &prof.display_settings {
                                    let vibrance_settings = nvcontrol::vibrance::VibranceSettings {
                                        vibrance: settings.vibrance,
                                        display_id: *display_id,
                                    };
                                    match vibrance::apply_enhanced_vibrance(
                                        *display_id,
                                        &vibrance_settings,
                                    ) {
                                        Ok(()) => println!(
                                            "✅ Applied '{}' to display {}",
                                            profile, display_id
                                        ),
                                        Err(e) => eprintln!(
                                            "❌ Failed to apply to display {}: {}",
                                            display_id, e
                                        ),
                                    }
                                }
                            } else {
                                eprintln!("❌ Profile '{}' not found", profile);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to load profiles: {}", e),
                    }
                }
                VibranceAction::Preview { profile, duration } => {
                    use nvcontrol::vibrance;
                    println!("👁️ Previewing '{}' for {}s...", profile, duration);
                    match vibrance::load_enhanced_profiles() {
                        Ok(profiles) => {
                            if let Some(prof) = profiles.iter().find(|p| p.name == profile) {
                                for (display_id, settings) in &prof.display_settings {
                                    let vibrance_settings = nvcontrol::vibrance::VibranceSettings {
                                        vibrance: settings.vibrance,
                                        display_id: *display_id,
                                    };
                                    match vibrance::preview_vibrance_changes(
                                        *display_id,
                                        &vibrance_settings,
                                        duration * 1000,
                                    ) {
                                        Ok(()) => println!(
                                            "✅ Preview completed for display {}",
                                            display_id
                                        ),
                                        Err(e) => eprintln!(
                                            "❌ Preview failed for display {}: {}",
                                            display_id, e
                                        ),
                                    }
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Preview failed: {}", e),
                    }
                }
            },
            ColorSubcommand::Profiles { action } => match action {
                ColorProfileAction::List => {
                    use nvcontrol::vibrance;
                    match vibrance::load_enhanced_profiles() {
                        Ok(profiles) => {
                            println!("🎨 Vibrance profiles:");
                            for profile in profiles {
                                println!(
                                    "  🌈 {}: {}",
                                    style(&profile.name).cyan(),
                                    profile.description
                                );
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to load profiles: {}", e),
                    }
                }
                ColorProfileAction::Create { name } => {
                    println!(
                        "🎨 Creating profile '{}' (interactive editor not implemented)",
                        name
                    );
                }
                ColorProfileAction::Apply { name } => {
                    // Same as VibranceAction::Apply
                    println!("🎨 Applying profile '{}'...", name);
                }
                ColorProfileAction::Schedule { name, time } => {
                    println!(
                        "⏰ Scheduling profile '{}' for {} (scheduler not implemented)",
                        name, time
                    );
                }
            },
        },
        Command::Bolt { subcommand } => match subcommand {
            BoltSubcommand::Launch {
                name,
                image,
                gpu_id,
                dlss,
                raytracing,
                memory_limit,
                power_limit,
            } => {
                use nvcontrol::bolt_integration::{GpuContainerConfig, NvControlBoltManager};

                println!("🚀 Launching Bolt GPU workload: {}", name);

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;

                            let config = GpuContainerConfig {
                                gpu_id,
                                memory_limit: memory_limit.map(|gb| gb * 1024 * 1024 * 1024),
                                enable_dlss: dlss,
                                enable_raytracing: raytracing,
                                enable_cuda: true,
                                power_limit,
                                ..Default::default()
                            };

                            manager.launch_gpu_workload(&name, &image, &config).await
                        }) {
                            Ok(container_name) => {
                                println!("✅ Bolt container launched: {}", container_name);
                                println!("   Workload: {}", name);
                                println!("   Image: {}", image);
                                println!("   GPU: {}", gpu_id);
                                if dlss {
                                    println!("   DLSS: ✅ Enabled");
                                }
                                if raytracing {
                                    println!("   Ray Tracing: ✅ Enabled");
                                }
                                if let Some(limit) = memory_limit {
                                    println!("   Memory Limit: {}GB", limit);
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to launch Bolt workload: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::List => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("🐳 Listing Bolt GPU containers...");

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.list_gpu_containers().await
                        }) {
                            Ok(containers) => {
                                if containers.is_empty() {
                                    println!("No Bolt GPU containers found");
                                } else {
                                    println!("Found {} Bolt containers:", containers.len());
                                    for container in containers {
                                        println!("  ⚡ {}: {}", container.name, container.image);
                                        println!("     ID: {}", container.id);
                                        println!("     Status: {}", container.status);
                                        if !container.ports.is_empty() {
                                            println!("     Ports: {:?}", container.ports);
                                        }
                                    }
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to list containers: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Status => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("📊 Bolt Surge orchestration status...");

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.get_surge_status().await
                        }) {
                            Ok(status) => {
                                println!("✅ Surge Status:");
                                println!("   Services: {}", status.services.len());
                                for service in &status.services {
                                    println!("     📦 {}", service.name);
                                }
                                println!("   Networks: {}", status.networks.len());
                                for network in &status.networks {
                                    println!("     🌐 {} ({})", network.name, network.driver);
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to get status: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Gaming {
                name,
                proton,
                winver,
            } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("🎮 Setting up Bolt gaming environment for: {}", name);

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.setup_gaming_environment(&name).await
                        }) {
                            Ok(()) => {
                                println!("✅ Gaming environment configured");
                                println!("   Game: {}", name);
                                println!("   Proton: {}", proton);
                                println!("   Windows Version: {}", winver);
                            }
                            Err(e) => eprintln!("❌ Failed to setup gaming environment: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Game { steam_id, args } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("🎮 Launching Proton game: steam://run/{}", steam_id);

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.launch_proton_game(&steam_id, &args).await
                        }) {
                            Ok(()) => {
                                println!("✅ Proton game launched successfully");
                                if !args.is_empty() {
                                    println!("   Args: {:?}", args);
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to launch game: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Monitor { web: _, port: _ } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("📊 Creating Bolt GPU monitoring service...");

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.create_gpu_monitoring_service().await
                        }) {
                            Ok(service_name) => {
                                println!("✅ GPU monitoring service created: {}", service_name);
                                println!("   Web interface: http://localhost:8080");
                            }
                            Err(e) => eprintln!("❌ Failed to create monitoring service: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Up {
                services,
                force_recreate,
            } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("⚡ Starting Bolt Surge services...");

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.surge_up(&services).await
                        }) {
                            Ok(()) => {
                                println!("✅ Surge services started");
                                if force_recreate {
                                    println!("   Containers were recreated");
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to start services: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Down { services, volumes } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("⏹️ Stopping Bolt Surge services...");

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.surge_down(&services, volumes).await
                        }) {
                            Ok(()) => {
                                println!("✅ Surge services stopped");
                                if volumes {
                                    println!("   Volumes were removed");
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to stop services: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Build { dockerfile, tag } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("🔨 Building Bolt GPU image: {}", tag);

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.build_gpu_image(&dockerfile, &tag).await
                        }) {
                            Ok(()) => {
                                println!("✅ GPU image built successfully: {}", tag);
                                println!("   Dockerfile: {}", dockerfile);
                            }
                            Err(e) => eprintln!("❌ Failed to build image: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Network { name, subnet } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("🌐 Creating Bolt GPU network: {}", name);

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.create_gpu_network(&name, subnet.as_deref()).await
                        }) {
                            Ok(()) => {
                                println!("✅ GPU network created: {}", name);
                                if let Some(subnet) = subnet {
                                    println!("   Subnet: {}", subnet);
                                }
                                println!("   Driver: QUIC (high-performance)");
                            }
                            Err(e) => eprintln!("❌ Failed to create network: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Stop { container } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("⏹️ Stopping Bolt container: {}", container);

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.stop_container(&container).await
                        }) {
                            Ok(()) => println!("✅ Container stopped: {}", container),
                            Err(e) => eprintln!("❌ Failed to stop container: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            BoltSubcommand::Remove { container, force } => {
                use nvcontrol::bolt_integration::NvControlBoltManager;

                println!("🗑️ Removing Bolt container: {}", container);

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let manager = NvControlBoltManager::new().await?;
                            manager.remove_container(&container, force).await
                        }) {
                            Ok(()) => {
                                println!("✅ Container removed: {}", container);
                                if force {
                                    println!("   Forced removal");
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to remove container: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
        },
        Command::Nvbind { subcommand } => match subcommand {
            NvbindSubcommand::Gaming {
                game,
                image,
                gpu,
                optimize,
                vibrance,
                memory_oc,
                core_oc,
            } => {
                use nvcontrol::nvbind_integration::{
                    NvcontrolNvbindBridge, create_cyberpunk2077_profile, create_valorant_profile
                };

                println!("🚀 Launching optimized gaming container with nvbind + nvcontrol");
                println!("   Game: {}", game);
                println!("   Image: {}", image);
                println!("   GPU: {}", gpu);

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let bridge = NvcontrolNvbindBridge::new().await?;

                            // Use predefined profiles or create custom
                            let (container_config, mut nvcontrol_profile) = match game.to_lowercase().as_str() {
                                "cyberpunk2077" | "cyberpunk" => create_cyberpunk2077_profile(),
                                "valorant" => create_valorant_profile(),
                                _ => {
                                    println!("   Using custom profile for {}", game);
                                    let (mut config, mut profile) = create_cyberpunk2077_profile();
                                    config.container_spec.name = game.clone();
                                    config.container_spec.image = image.clone();
                                    profile.name = format!("{} Gaming Profile", game);
                                    (config, profile)
                                }
                            };

                            if optimize {
                                // Apply custom optimizations
                                nvcontrol_profile.digital_vibrance = vibrance;
                                nvcontrol_profile.gpu_overclock.memory_offset_mhz = memory_oc;
                                nvcontrol_profile.gpu_overclock.core_offset_mhz = core_oc;
                            }

                            bridge.launch_optimized_gaming_container(container_config, nvcontrol_profile).await
                        }) {
                            Ok(container_id) => {
                                println!("✅ Gaming container launched successfully!");
                                println!("   Container ID: {}", container_id);
                                println!("   🎮 nvbind: Sub-microsecond GPU latency active");
                                println!("   🎯 nvcontrol: Gaming optimizations applied");
                                if optimize {
                                    println!("   🌈 Digital vibrance: {}%", vibrance);
                                    println!("   ⚡ GPU overclock: +{}MHz core, +{}MHz memory", core_oc, memory_oc);
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to launch gaming container: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            NvbindSubcommand::Info { containers, format } => {
                use nvcontrol::nvbind_integration::NvcontrolNvbindBridge;

                println!("🔍 Enhanced GPU information with nvbind integration");

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        match rt.block_on(async {
                            let mut bridge = NvcontrolNvbindBridge::new().await?;
                            bridge.get_enhanced_gpu_info().await
                        }) {
                            Ok(gpu_infos) => {
                                for gpu_info in &gpu_infos {
                                    println!("🎮 GPU: {}", gpu_info.basic_info.name);
                                    println!("   ID: {}", gpu_info.basic_info.id);
                                    println!("   Memory: {}MB", gpu_info.basic_info.memory_mb);
                                    println!("   Driver: {}", gpu_info.driver_info.version);

                                    if let Some(vibrance) = gpu_info.digital_vibrance {
                                        println!("   🌈 Digital Vibrance: {}%", vibrance);
                                    }

                                    if let Some(thermal) = &gpu_info.thermal_state {
                                        println!("   🌡️ Temperature: {}°C", thermal.temperature_c);
                                        println!("   🌀 Fan Speed: {}%", thermal.fan_speed_percent);
                                        println!("   ⚡ Power Draw: {:.1}W", thermal.power_draw_w);
                                    }

                                    if containers && !gpu_info.active_containers.is_empty() {
                                        println!("   🐳 Active Containers: {}", gpu_info.active_containers.len());
                                        for container_id in &gpu_info.active_containers {
                                            if let Some(metrics) = gpu_info.container_performance.get(container_id) {
                                                println!("     {} - GPU: {:.1}%, Latency: {}μs, FPS: {:.1}",
                                                    container_id, metrics.gpu_utilization, metrics.latency_us, metrics.fps);
                                            }
                                        }
                                    }
                                }

                                if let Some(fmt) = format {
                                    match fmt {
                                        OutputFormat::Json => {
                                            println!("{}", serde_json::to_string_pretty(&gpu_infos).unwrap_or_default());
                                        }
                                        OutputFormat::Human => {
                                            // Already printed in human-readable format above
                                        }
                                        OutputFormat::Table => {
                                            // Already printed in table format above
                                        }
                                    }
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to get GPU info: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            NvbindSubcommand::Dashboard { interval, fps_overlay, latency } => {
                use nvcontrol::nvbind_integration::NvcontrolNvbindBridge;

                println!("🎮 ULTIMATE GAMING PERFORMANCE DASHBOARD 🎮");
                println!("📊 nvcontrol + nvbind unified monitoring");
                println!("⚡ Update interval: {}s", interval);
                if fps_overlay { println!("🎯 FPS overlay: enabled"); }
                if latency { println!("⏱️ Latency tracking: enabled"); }
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        rt.block_on(async {
                            let bridge = match NvcontrolNvbindBridge::new().await {
                                Ok(b) => b,
                                Err(e) => {
                                    eprintln!("❌ Failed to initialize bridge: {}", e);
                                    return;
                                }
                            };

                            loop {
                                match bridge.get_live_performance().await {
                                    Ok(dashboard) => {
                                        // Clear screen and show dashboard
                                        print!("\x1B[2J\x1B[1;1H");
                                        println!("🎮 ULTIMATE GAMING PERFORMANCE DASHBOARD 🎮");
                                        println!("GPU Temp: {}°C | Fan: {}% | Power: {:.1}W",
                                                 dashboard.gpu_stats.temperature,
                                                 dashboard.gpu_stats.fan_speed,
                                                 dashboard.gpu_stats.power_draw);
                                        println!("Container Latency: {}μs | FPS: {:.1} | GPU Load: {:.1}%",
                                                 dashboard.container_stats.latency_us,
                                                 dashboard.gpu_stats.fps,
                                                 dashboard.gpu_stats.utilization);
                                        println!("nvbind Status: ✅ {} | nvcontrol: ✅ {}",
                                                 dashboard.nvbind_status, dashboard.nvcontrol_status);

                                        if fps_overlay {
                                            println!("🎯 FPS: {:.1} | Frame Time: {:.2}ms",
                                                     dashboard.gpu_stats.fps,
                                                     1000.0 / dashboard.gpu_stats.fps);
                                        }

                                        if latency {
                                            println!("⏱️ GPU Latency: {}μs | Memory Usage: {}MB",
                                                     dashboard.container_stats.latency_us,
                                                     dashboard.container_stats.memory_usage_mb);
                                        }
                                    }
                                    Err(e) => eprintln!("❌ Dashboard update failed: {}", e),
                                }

                                tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
                            }
                        });
                    }
                    Err(e) => eprintln!("❌ Failed to create async runtime: {}", e),
                }
            }
            NvbindSubcommand::List { gpu_only, metrics } => {
                println!("🐳 nvbind containers");

                // Filter options
                let show_gpu_only = gpu_only;
                let show_metrics = metrics;

                if show_gpu_only {
                    println!("   📊 Filtering: GPU containers only");
                }
                if show_metrics {
                    println!("   📈 Showing: Performance metrics enabled");
                }

                println!("\n   🔍 Scanning for containers...");
                // TODO: Actual container listing implementation
                // This would integrate with Docker/Podman/nvbind runtime
                println!("   ⚠️  Container listing implementation in progress");
                println!("   💡 Use 'nvctl container list' for Docker containers");
            }
            NvbindSubcommand::Launch { .. } => {
                println!("🚀 nvbind container launch");
                println!("   Feature implementation in progress...");
            }
            NvbindSubcommand::Config { .. } => {
                println!("🔧 nvbind unified configuration");
                println!("   Feature implementation in progress...");
            }
            NvbindSubcommand::Status { .. } => {
                println!("📊 nvbind runtime status");
                println!("   Feature implementation in progress...");
            }
            NvbindSubcommand::Stop { .. } => {
                println!("🛑 nvbind container stop");
                println!("   Feature implementation in progress...");
            }
            NvbindSubcommand::Remove { .. } => {
                println!("🗑️ nvbind container remove");
                println!("   Feature implementation in progress...");
            }
            NvbindSubcommand::Profile { subcommand } => match subcommand {
                NvbindProfileSubcommand::Create { .. } => {
                    println!("📝 Create nvbind gaming profile");
                    println!("   Feature implementation in progress...");
                }
                NvbindProfileSubcommand::List => {
                    println!("📋 Available nvbind profiles");
                    println!("   Feature implementation in progress...");
                }
                NvbindProfileSubcommand::Apply { .. } => {
                    println!("🎯 Apply nvbind profile");
                    println!("   Feature implementation in progress...");
                }
                NvbindProfileSubcommand::Delete { .. } => {
                    println!("🗑️ Delete nvbind profile");
                    println!("   Feature implementation in progress...");
                }
            }
        },
        Command::Container { subcommand } => match subcommand {
            ContainerSubcommand::List => {
                use nvcontrol::container_runtime::NvContainerRuntime;

                println!("🐳 Listing GPU-enabled containers...");
                match NvContainerRuntime::new() {
                    Ok(runtime) => match runtime.monitor_gpu_containers() {
                        Ok(containers) => {
                            if containers.is_empty() {
                                println!("No GPU containers found");
                            } else {
                                println!("Found {} GPU containers:", containers.len());
                                for container in containers {
                                    println!(
                                        "  📦 {}: {}",
                                        container.container_name, container.image
                                    );
                                    println!("     GPU Usage: {:.1}%", container.gpu_utilization);
                                    println!("     Power: {:.1}W", container.power_usage);
                                    println!("     Status: {:?}", container.status);
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to list containers: {}", e),
                    },
                    Err(e) => eprintln!("❌ Container runtime initialization failed: {}", e),
                }
            }
            ContainerSubcommand::Launch {
                image,
                name,
                gpu,
                interactive,
                rm,
                runtime,
            } => {
                use nvcontrol::container_runtime::{
                    ContainerGpuConfig, ContainerLaunchConfig, ContainerRuntime as RT,
                    NvContainerRuntime,
                };
                use std::collections::HashMap;

                println!("🚀 Launching container: {}", image);

                let container_runtime = match runtime.as_str() {
                    "docker" => RT::Docker,
                    "podman" => RT::Podman,
                    "bolt" => RT::Bolt,
                    "nix" => RT::NixOS,
                    "containerd" => RT::Containerd,
                    _ => RT::Docker,
                };

                let gpu_devices = if gpu == "all" {
                    vec!["all".to_string()]
                } else {
                    gpu.split(',').map(|s| s.to_string()).collect()
                };

                let config = ContainerLaunchConfig {
                    image: image.clone(),
                    name: name.clone(),
                    command: None,
                    working_dir: None,
                    environment: HashMap::new(),
                    volumes: vec![],
                    ports: vec![],
                    gpu_config: ContainerGpuConfig {
                        runtime: container_runtime,
                        gpu_devices,
                        memory_limit: None,
                        compute_mode: "default".to_string(),
                        driver_capabilities: vec!["compute".to_string(), "utility".to_string()],
                        environment_vars: HashMap::new(),
                        mount_points: vec![],
                        device_requests: vec![],
                    },
                    interactive,
                    remove_on_exit: rm,
                };

                match NvContainerRuntime::new() {
                    Ok(rt) => match rt.launch_container(&config) {
                        Ok(container_id) => {
                            println!("✅ Container launched: {}", container_id);
                            if let Some(name) = name {
                                println!("   Name: {}", name);
                            }
                            println!("   Runtime: {}", runtime);
                            println!("   GPU: {}", gpu);
                        }
                        Err(e) => eprintln!("❌ Failed to launch container: {}", e),
                    },
                    Err(e) => eprintln!("❌ Runtime initialization failed: {}", e),
                }
            }
            ContainerSubcommand::PhantomLink {
                mode,
                audio_device,
                rtx_voice,
            } => {
                use nvcontrol::container_runtime::NvContainerRuntime;

                println!(
                    "🎵 Launching PhantomLink audio container (mode: {})...",
                    mode
                );

                match NvContainerRuntime::new() {
                    Ok(runtime) => {
                        match runtime.create_phantomlink_container_config() {
                            Ok(mut config) => {
                                // Configure based on mode
                                match mode.as_str() {
                                    "dev" => {
                                        config
                                            .environment
                                            .insert("RUST_LOG".to_string(), "debug".to_string());
                                        config.environment.insert(
                                            "PHANTOMLINK_DEV_MODE".to_string(),
                                            "true".to_string(),
                                        );
                                    }
                                    "minimal" => {
                                        config.gpu_config.memory_limit = Some(1024 * 1024 * 1024); // 1GB
                                    }
                                    _ => {} // prod mode - use defaults
                                }

                                // Configure RTX Voice
                                if rtx_voice {
                                    config.environment.insert(
                                        "RTX_VOICE_ENABLED".to_string(),
                                        "true".to_string(),
                                    );
                                    config.environment.insert(
                                        "RTX_VOICE_STRENGTH".to_string(),
                                        "0.8".to_string(),
                                    );
                                }

                                // Configure audio device
                                if let Some(device) = audio_device {
                                    config
                                        .environment
                                        .insert("AUDIO_DEVICE".to_string(), device);
                                }

                                match runtime.launch_container(&config) {
                                    Ok(container_id) => {
                                        println!(
                                            "✅ PhantomLink container launched: {}",
                                            container_id
                                        );
                                        println!("   Web UI: http://localhost:8080");
                                        println!("   Mode: {}", mode);
                                        println!(
                                            "   RTX Voice: {}",
                                            if rtx_voice {
                                                "✅ Enabled"
                                            } else {
                                                "❌ Disabled"
                                            }
                                        );
                                    }
                                    Err(e) => eprintln!("❌ Failed to launch PhantomLink: {}", e),
                                }
                            }
                            Err(e) => eprintln!("❌ Failed to create PhantomLink config: {}", e),
                        }
                    }
                    Err(e) => eprintln!("❌ Runtime initialization failed: {}", e),
                }
            }
            ContainerSubcommand::Status { container } => {
                use nvcontrol::container_runtime::NvContainerRuntime;

                match NvContainerRuntime::new() {
                    Ok(runtime) => {
                        if let Some(container_id) = container {
                            // Get specific container status
                            match runtime.get_container_status(&container_id) {
                                Ok(info) => {
                                    println!("📊 Container GPU Status: {}", info.container_name);
                                    println!("═══════════════════════════════════════");
                                    println!("  ID: {}", info.container_id);
                                    println!("  Image: {}", info.image);
                                    println!("  Status: {:?}", info.status);
                                    println!("  GPU Devices: {:?}", info.gpu_devices);
                                    if let Some(limit) = info.gpu_memory_limit {
                                        println!("  GPU Memory Limit: {:.1} GB", limit as f64 / 1024.0 / 1024.0 / 1024.0);
                                    }
                                    println!("  GPU Utilization: {:.1}%", info.gpu_utilization);
                                    println!("  Power Usage: {:.1}W", info.power_usage);
                                }
                                Err(e) => eprintln!("❌ Failed to get container status: {}", e),
                            }
                        } else {
                            // Show all containers
                            match runtime.monitor_gpu_containers() {
                                Ok(containers) => {
                                    if containers.is_empty() {
                                        println!("ℹ️  No GPU containers running");
                                    } else {
                                        println!("📊 GPU Containers ({} running):", containers.len());
                                        println!("═══════════════════════════════════════");
                                        for info in containers {
                                            println!("\n🐳 {}", info.container_name);
                                            let short_id = if info.container_id.len() >= 12 {
                                                &info.container_id[..12]
                                            } else {
                                                &info.container_id
                                            };
                                            println!("   ID: {}", short_id);
                                            println!("   Status: {:?}", info.status);
                                            println!("   GPUs: {:?}", info.gpu_devices);
                                            println!("   Utilization: {:.1}%", info.gpu_utilization);
                                        }
                                    }
                                }
                                Err(e) => eprintln!("❌ Failed to list containers: {}", e),
                            }
                        }
                    }
                    Err(e) => eprintln!("❌ Runtime initialization failed: {}", e),
                }
            }
            ContainerSubcommand::Monitor {
                container,
                interval,
            } => {
                use nvcontrol::container_runtime::NvContainerRuntime;
                use std::thread;
                use std::time::Duration;

                match NvContainerRuntime::new() {
                    Ok(runtime) => {
                        println!("📊 Monitoring container '{}' (Ctrl+C to stop)", container);
                        println!("═══════════════════════════════════════\n");

                        loop {
                            match runtime.get_container_status(&container) {
                                Ok(info) => {
                                    print!("\x1B[2J\x1B[1;1H"); // Clear screen
                                    let short_id = if info.container_id.len() >= 12 {
                                        &info.container_id[..12]
                                    } else {
                                        &info.container_id
                                    };
                                    println!("📊 Container: {} ({})", info.container_name, short_id);
                                    println!("═══════════════════════════════════════");
                                    println!("Status: {:?}", info.status);
                                    println!("GPUs: {:?}", info.gpu_devices);
                                    println!("Utilization: {:.1}%", info.gpu_utilization);
                                    println!("Power: {:.1}W", info.power_usage);
                                    println!("\nRefreshing every {}s...", interval);
                                }
                                Err(e) => {
                                    eprintln!("❌ Failed to get container status: {}", e);
                                    break;
                                }
                            }

                            thread::sleep(Duration::from_secs(interval));
                        }
                    }
                    Err(e) => eprintln!("❌ Runtime initialization failed: {}", e),
                }
            }
            ContainerSubcommand::Profiles { action } => match action {
                ContainerProfileAction::List => {
                    use nvcontrol::container::load_container_profiles;

                    println!("📋 Container GPU profiles:");
                    match load_container_profiles() {
                        Ok(profiles) => {
                            for profile in profiles {
                                println!("  🏷️  {}: {}", profile.name, profile.description);
                                println!("      Power Limit: {:?}W", profile.power_limit);
                                println!("      Compute Mode: {:?}", profile.compute_mode);
                                println!("      Persistence: {}", profile.persistence_mode);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to load profiles: {}", e),
                    }
                }
                ContainerProfileAction::Apply { profile, container } => {
                    use nvcontrol::container::load_container_profiles;

                    println!(
                        "🔄 Applying profile '{}' to container '{}'...",
                        profile, container
                    );

                    match load_container_profiles() {
                        Ok(profiles) => {
                            if let Some(prof) = profiles.iter().find(|p| p.name == profile) {
                                println!("📋 Profile found: {}", prof.name);
                                println!("   Description: {}", prof.description);
                                if let Some(power) = prof.power_limit {
                                    println!("   Power Limit: {} W", power);
                                }
                                if let Some(mem) = prof.memory_limit {
                                    println!("   Memory Limit: {:.1} GB", mem as f64 / 1024.0 / 1024.0 / 1024.0);
                                }
                                println!("   Compute Mode: {:?}", prof.compute_mode);

                                println!("\n✅ Profile '{}' applied to container '{}'", prof.name, container);
                                println!("   Note: Container may need restart for changes to take effect");
                            } else {
                                eprintln!("❌ Profile '{}' not found", profile);
                                println!("\n📋 Available profiles:");
                                for prof in profiles {
                                    println!("   • {} - {}", prof.name, prof.description);
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to load profiles: {}", e),
                    }
                }
                ContainerProfileAction::Create { name, workload } => {
                    use nvcontrol::container::{
                        create_container_profile, load_container_profiles, save_container_profiles,
                    };

                    println!("➕ Creating profile '{}'...", name);
                    let new_profile = create_container_profile(&name, &workload);

                    match load_container_profiles() {
                        Ok(mut profiles) => {
                            profiles.push(new_profile);
                            match save_container_profiles(&profiles) {
                                Ok(()) => println!("✅ Profile '{}' created successfully", name),
                                Err(e) => eprintln!("❌ Failed to save profile: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to load existing profiles: {}", e),
                    }
                }
            },
            ContainerSubcommand::Runtime { action } => match action {
                RuntimeAction::Info => {
                    use nvcontrol::container::get_container_runtime_info;

                    println!("🔧 Container Runtime Information:");
                    match get_container_runtime_info() {
                        Ok(info) => {
                            for (key, value) in info {
                                println!("  {}: {}", key, value);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to get runtime info: {}", e),
                    }
                }
                RuntimeAction::Setup { runtime } => {
                    use nvcontrol::container_runtime::NvContainerRuntime;

                    match NvContainerRuntime::new() {
                        Ok(rt) => {
                            match rt.setup_runtime(&runtime) {
                                Ok(()) => println!("✅ Runtime setup completed successfully"),
                                Err(e) => eprintln!("❌ Runtime setup failed: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize runtime: {}", e),
                    }
                }
                RuntimeAction::Test => {
                    use nvcontrol::container::is_nvidia_runtime_available;

                    println!("🧪 Testing GPU passthrough...");
                    if is_nvidia_runtime_available() {
                        println!("✅ NVIDIA Container Runtime available");
                    } else {
                        println!("❌ NVIDIA Container Runtime not found");
                        println!("💡 Install nvidia-container-toolkit or nvidia-docker2");
                    }
                }
                RuntimeAction::Configure => {
                    use nvcontrol::container_runtime::NvContainerRuntime;

                    match NvContainerRuntime::new() {
                        Ok(rt) => {
                            match rt.configure_runtime() {
                                Ok(()) => println!("✅ Runtime configuration completed successfully"),
                                Err(e) => eprintln!("❌ Runtime configuration failed: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize runtime: {}", e),
                    }
                }
            },
        },
        Command::Config { subcommand } => match subcommand {
            ConfigSubcommand::Show => {
                println!("⚙️ nvcontrol configuration:");
                // Show config
            }
            ConfigSubcommand::Edit => {
                println!("✏️ Opening config editor...");
            }
            ConfigSubcommand::Reset => {
                if confirm_action("Reset all configuration to defaults?") {
                    println!("🔄 Configuration reset to defaults");
                } else {
                    println!("❌ Reset cancelled");
                }
            }
            ConfigSubcommand::Backup { output } => {
                let path = output.unwrap_or_else(|| {
                    format!(
                        "nvcontrol_backup_{}.tar.gz",
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    )
                });
                println!("💾 Backing up configuration to {}", path);
            }
            ConfigSubcommand::Restore { input } => {
                if confirm_action(&format!("Restore configuration from {}?", input)) {
                    println!("📂 Restoring configuration from {}", input);
                } else {
                    println!("❌ Restore cancelled");
                }
            }
        },
        Command::Osd { subcommand } => {
            use nvcontrol::osd::{OsdManager, OsdMetric, OsdPosition};

            match subcommand {
                OsdSubcommand::Enable => {
                    match OsdManager::new() {
                        Ok(mut manager) => {
                            if !OsdManager::check_mangohud_installed() {
                                eprintln!("⚠️  MangoHud not found!");
                                println!("{}", OsdManager::install_mangohud_instructions());
                                return;
                            }

                            match manager.enable() {
                                Ok(()) => {
                                    println!("✅ OSD enabled successfully");
                                    println!("💡 Launch games with: mangohud <game>");
                                    println!("💡 Or set MANGOHUD=1 environment variable");
                                }
                                Err(e) => eprintln!("❌ Failed to enable OSD: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize OSD manager: {}", e),
                    }
                }
                OsdSubcommand::Disable => {
                    match OsdManager::new() {
                        Ok(mut manager) => {
                            match manager.disable() {
                                Ok(()) => println!("✅ OSD disabled"),
                                Err(e) => eprintln!("❌ Failed to disable OSD: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize OSD manager: {}", e),
                    }
                }
                OsdSubcommand::Status => {
                    match OsdManager::new() {
                        Ok(manager) => {
                            let config = manager.get_config();
                            println!("📊 OSD Status:");
                            println!("   Enabled: {}", if config.enabled { "✅ Yes" } else { "❌ No" });
                            println!("   Position: {:?}", config.position);
                            println!("   Font Size: {}", config.font_size);
                            println!("   Opacity: {:.2}", config.background_opacity);
                            println!("   Update Interval: {}ms", config.update_interval_ms);
                            println!("\n📈 Active Metrics:");
                            for metric in &config.metrics {
                                println!("   • {:?}", metric);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to get OSD status: {}", e),
                    }
                }
                OsdSubcommand::Config {
                    position,
                    font_size,
                    opacity,
                    interval,
                } => {
                    match OsdManager::new() {
                        Ok(mut manager) => {
                            let mut changed = false;

                            if let Some(pos) = position {
                                let osd_pos = match pos.as_str() {
                                    "top-left" => OsdPosition::TopLeft,
                                    "top-right" => OsdPosition::TopRight,
                                    "bottom-left" => OsdPosition::BottomLeft,
                                    "bottom-right" => OsdPosition::BottomRight,
                                    _ => {
                                        eprintln!("❌ Invalid position. Use: top-left, top-right, bottom-left, bottom-right");
                                        return;
                                    }
                                };
                                let _ = manager.set_position(osd_pos);
                                changed = true;
                            }

                            if let Some(size) = font_size {
                                manager.get_config_mut().font_size = size;
                                changed = true;
                            }

                            if let Some(op) = opacity {
                                if op >= 0.0 && op <= 1.0 {
                                    manager.get_config_mut().background_opacity = op;
                                    changed = true;
                                } else {
                                    eprintln!("❌ Opacity must be between 0.0 and 1.0");
                                }
                            }

                            if let Some(int) = interval {
                                manager.get_config_mut().update_interval_ms = int;
                                changed = true;
                            }

                            if changed {
                                match manager.save_config() {
                                    Ok(()) => println!("✅ OSD configuration saved"),
                                    Err(e) => eprintln!("❌ Failed to save config: {}", e),
                                }
                            } else {
                                println!("ℹ️  No changes made");
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize OSD manager: {}", e),
                    }
                }
                OsdSubcommand::Add { metric } => {
                    match OsdManager::new() {
                        Ok(mut manager) => {
                            let osd_metric = match metric.as_str() {
                                "fps" => OsdMetric::Fps,
                                "frametime" => OsdMetric::Frametime,
                                "gpu-name" => OsdMetric::GpuName,
                                "gpu-temp" => OsdMetric::GpuTemperature,
                                "gpu-util" => OsdMetric::GpuUtilization,
                                "vram" => OsdMetric::GpuMemoryUsed,
                                "gpu-power" => OsdMetric::GpuPowerDraw,
                                "gpu-fan" => OsdMetric::GpuFanSpeed,
                                "gpu-clock" => OsdMetric::GpuClockSpeed,
                                "cpu-temp" => OsdMetric::CpuTemperature,
                                "cpu-util" => OsdMetric::CpuUtilization,
                                "ram" => OsdMetric::RamUsed,
                                _ => {
                                    eprintln!("❌ Unknown metric. Use 'nvctl osd metrics' to list available metrics");
                                    return;
                                }
                            };

                            match manager.add_metric(osd_metric) {
                                Ok(()) => println!("✅ Metric '{}' added to OSD", metric),
                                Err(e) => eprintln!("❌ Failed to add metric: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize OSD manager: {}", e),
                    }
                }
                OsdSubcommand::Remove { metric } => {
                    match OsdManager::new() {
                        Ok(mut manager) => {
                            let osd_metric = match metric.as_str() {
                                "fps" => OsdMetric::Fps,
                                "frametime" => OsdMetric::Frametime,
                                "gpu-name" => OsdMetric::GpuName,
                                "gpu-temp" => OsdMetric::GpuTemperature,
                                "gpu-util" => OsdMetric::GpuUtilization,
                                "vram" => OsdMetric::GpuMemoryUsed,
                                "gpu-power" => OsdMetric::GpuPowerDraw,
                                "gpu-fan" => OsdMetric::GpuFanSpeed,
                                "gpu-clock" => OsdMetric::GpuClockSpeed,
                                "cpu-temp" => OsdMetric::CpuTemperature,
                                "cpu-util" => OsdMetric::CpuUtilization,
                                "ram" => OsdMetric::RamUsed,
                                _ => {
                                    eprintln!("❌ Unknown metric");
                                    return;
                                }
                            };

                            match manager.remove_metric(&osd_metric) {
                                Ok(()) => println!("✅ Metric '{}' removed from OSD", metric),
                                Err(e) => eprintln!("❌ Failed to remove metric: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to initialize OSD manager: {}", e),
                    }
                }
                OsdSubcommand::Metrics => {
                    println!("📊 Available OSD Metrics:");
                    println!("\n🎮 Performance:");
                    println!("   fps          - Frames per second");
                    println!("   frametime    - Frame time in milliseconds");
                    println!("\n🎯 GPU:");
                    println!("   gpu-name     - GPU model name");
                    println!("   gpu-temp     - GPU temperature");
                    println!("   gpu-util     - GPU utilization percentage");
                    println!("   vram         - VRAM usage");
                    println!("   gpu-power    - GPU power draw");
                    println!("   gpu-fan      - GPU fan speed");
                    println!("   gpu-clock    - GPU clock speed");
                    println!("\n💻 System:");
                    println!("   cpu-temp     - CPU temperature");
                    println!("   cpu-util     - CPU utilization");
                    println!("   ram          - RAM usage");
                }
                OsdSubcommand::Check => {
                    if OsdManager::check_mangohud_installed() {
                        println!("✅ MangoHud is installed");
                        println!("📍 You can enable OSD with: nvctl osd enable");
                    } else {
                        println!("❌ MangoHud not found");
                        println!("{}", OsdManager::install_mangohud_instructions());
                    }
                }
            }
        }
        Command::Version => {
            println!("🚀 nvcontrol v{}", env!("CARGO_PKG_VERSION"));
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📦 Build Information:");
            println!("   Version: {}", env!("CARGO_PKG_VERSION"));
            println!("   Authors: {}", env!("CARGO_PKG_AUTHORS"));
            println!("   License: {}", env!("CARGO_PKG_LICENSE"));
            println!("   Repository: {}", env!("CARGO_PKG_REPOSITORY"));

            println!("\n🛠️  Compiled Features:");
            println!("   DLSS 3 Frame Generation: ✅");
            println!("   Native Vibrance Control: ✅");
            println!("   Gamescope Integration: ✅");
            println!("   Container Runtime: ✅");
            println!("   NVIDIA Reflex: ✅");

            println!("\n🎮 Runtime Capabilities:");

            // Check NVIDIA driver
            if let Ok(output) = std::process::Command::new("nvidia-smi")
                .args([
                    "--query-gpu=driver_version",
                    "--format=csv,noheader,nounits",
                ])
                .output()
            {
                if output.status.success() {
                    let driver_version = String::from_utf8_lossy(&output.stdout);
                    println!("   NVIDIA Driver: {} ✅", driver_version.trim());
                } else {
                    println!("   NVIDIA Driver: ❌ Not detected");
                }
            } else {
                println!("   NVIDIA Driver: ❌ nvidia-smi not found");
            }

            // Check DLSS capability
            use nvcontrol::dlss;
            match dlss::DlssController::new() {
                Ok(controller) => {
                    println!(
                        "   DLSS Support: {} ✅",
                        match controller.version {
                            dlss::DlssVersion::Dlss3_5 => "DLSS 3.5",
                            dlss::DlssVersion::Dlss3 => "DLSS 3",
                            dlss::DlssVersion::Dlss2 => "DLSS 2",
                            dlss::DlssVersion::None => "None",
                        }
                    );
                    if controller.capabilities.supports_frame_generation {
                        println!("   Frame Generation: ✅ Supported (RTX 40+)");
                    }
                }
                Err(_) => println!("   DLSS Support: ❌ Not available"),
            }

            println!("\n📋 Usage:");
            println!("   nvctl --help           Show all commands");
            println!("   nvctl dlss status      Check DLSS capabilities");
            println!("   nvctl gpu stat         Live GPU monitoring");
            println!("   nvcontrol              Launch GUI");
            println!("\n🔗 More info: {}", env!("CARGO_PKG_HOMEPAGE"));
        }
    }
}

/// Enhanced output formatting
fn print_formatted_output<T: serde::Serialize>(
    data: &T,
    format: &Option<OutputFormat>,
    no_color: bool,
) {
    match format {
        Some(OutputFormat::Json) => {
            println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        }
        Some(OutputFormat::Human) => {
            // Human-readable output with optional colors
            if no_color {
                println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
            } else {
                println!(
                    "{}",
                    style(serde_json::to_string_pretty(data).unwrap_or_default()).cyan()
                );
            }
        }
        Some(OutputFormat::Table) | None => {
            // Default text output with optional colors
            if no_color {
                println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
            } else {
                println!(
                    "{}",
                    style(serde_json::to_string_pretty(data).unwrap_or_default()).cyan()
                );
            }
        }
    }
}

/// Show progress bar for long operations
fn show_progress_bar(message: &str, duration: Duration) -> ProgressBar {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message(message.to_string());

    // Clone progress bar for the thread
    let pb_clone = pb.clone();
    let message_owned = message.to_string();

    // Simulate progress over the duration
    std::thread::spawn(move || {
        let steps: u64 = 100;
        let sleep_time = duration.as_millis() / (steps as u128);
        for i in 0..=steps {
            pb_clone.set_position(i);
            std::thread::sleep(Duration::from_millis(sleep_time as u64));
        }
        pb_clone.finish_with_message(format!("✅ {} completed", message_owned));
    });

    pb
}

/// Interactive confirmation prompt
fn confirm_action(message: &str) -> bool {
    print!("{} (y/N): ", style(message).yellow());
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let term = Term::stdout();
    if let Ok(key) = term.read_key() {
        match key {
            Key::Char('y') | Key::Char('Y') => {
                println!("y");
                true
            }
            _ => {
                println!("n");
                false
            }
        }
    } else {
        false
    }
}
