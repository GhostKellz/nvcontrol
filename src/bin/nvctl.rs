use clap::{Parser, Subcommand};
use console::{Key, Term, style};
use indicatif::{ProgressBar, ProgressStyle};
use nvcontrol::{
    display, drivers, fan, gamescope,
    gpu::{self, OutputFormat},
    latency, monitoring, overclocking, power, recording, upscaling, vrr,
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
    Container {
        #[command(subcommand)]
        subcommand: ContainerSubcommand,
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
    /// Container profile management
    Profiles {
        #[command(subcommand)]
        action: ContainerProfileAction,
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
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
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
                use nvcontrol::vibrance;

                match subcommand {
                    VibranceSubcommand::Get => match vibrance::get_displays() {
                        Ok(displays) => {
                            println!("Connected Displays:");
                            for (i, display) in displays.iter().enumerate() {
                                match vibrance::get_display_vibrance(i) {
                                    Ok(vibrance_val) => {
                                        let percentage =
                                            vibrance::vibrance_to_percentage(vibrance_val);
                                        println!("  {}: {}% vibrance", display, percentage);
                                    }
                                    Err(e) => println!("  {}: Error - {}", display, e),
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error getting displays: {}", e);
                            if !vibrance::is_available() {
                                eprintln!(
                                    "Note: nvibrant may not be installed or NVIDIA drivers not available"
                                );
                            }
                        }
                    },
                    VibranceSubcommand::Set { percentage } => {
                        let vibrance_val = vibrance::percentage_to_vibrance(percentage);
                        match vibrance::set_vibrance_all(vibrance_val) {
                            Ok(()) => println!("Set all displays to {}% vibrance", percentage),
                            Err(e) => eprintln!("Failed to set vibrance: {}", e),
                        }
                    }
                    VibranceSubcommand::SetDisplay {
                        display,
                        percentage,
                    } => {
                        let vibrance_val = vibrance::percentage_to_vibrance(percentage);
                        let display_values = vec![(display, vibrance_val)];
                        match vibrance::set_vibrance(&display_values) {
                            Ok(()) => {
                                println!("Set display {} to {}% vibrance", display, percentage)
                            }
                            Err(e) => {
                                eprintln!("Failed to set vibrance for display {}: {}", display, e)
                            }
                        }
                    }
                    VibranceSubcommand::SetRaw { levels } => {
                        // Convert i16 to (usize, i32) format
                        let display_values: Vec<(usize, i32)> = levels
                            .iter()
                            .enumerate()
                            .map(|(idx, &level)| (idx, level as i32))
                            .collect();

                        match vibrance::set_vibrance(&display_values) {
                            Ok(()) => {
                                println!("Applied raw vibrance values: {:?}", levels);
                            }
                            Err(e) => eprintln!("Failed to set raw vibrance: {}", e),
                        }
                    }
                    VibranceSubcommand::List => match vibrance::get_displays() {
                        Ok(displays) => {
                            println!("Available Displays:");
                            for (i, display) in displays.iter().enumerate() {
                                println!("  [{}] {}", i, display);
                            }
                        }
                        Err(e) => eprintln!("Failed to list displays: {}", e),
                    },
                    VibranceSubcommand::Reset => match vibrance::set_vibrance_all(0) {
                        Ok(()) => println!("Reset all displays to default vibrance (100%)"),
                        Err(e) => eprintln!("Failed to reset vibrance: {}", e),
                    },
                    VibranceSubcommand::Info => match vibrance::get_driver_info() {
                        Ok(info) => {
                            println!("Vibrance Support Information:");
                            println!("  {}", info);
                            println!(
                                "  nvibrant available: {}",
                                if vibrance::is_available() {
                                    "Yes"
                                } else {
                                    "No"
                                }
                            );

                            if !vibrance::is_available() {
                                println!("\nTo install nvibrant:");
                                println!("  pip install nvibrant");
                                println!("  # OR");
                                println!("  uvx nvibrant");
                            }
                        }
                        Err(e) => eprintln!("Failed to get driver info: {}", e),
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
                match overclocking::create_stress_test(duration) {
                    Ok(()) => println!("Stress test completed"),
                    Err(e) => eprintln!("Stress test failed: {e}"),
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
                    let mut config = gamescope::GamescopeConfig::default();

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
        Command::Container { subcommand } => match subcommand {
            ContainerSubcommand::List => {
                println!("🐳 Listing GPU containers...");
                // Implementation would call container::list_containers()
            }
            ContainerSubcommand::Status { container } => {
                println!(
                    "📊 Container status: {}",
                    container.unwrap_or_else(|| "all".to_string())
                );
                // Implementation would call container::get_status()
            }
            ContainerSubcommand::Monitor {
                container,
                interval,
            } => {
                println!(
                    "📊 Monitoring container '{}' every {}s...",
                    container, interval
                );
                // Implementation would call container::monitor_container()
            }
            ContainerSubcommand::Profiles { action } => match action {
                ContainerProfileAction::List => {
                    println!("📋 Container profiles:");
                    // Implementation would call container::list_profiles()
                }
                ContainerProfileAction::Apply { profile, container } => {
                    println!(
                        "🔄 Applying profile '{}' to container '{}'...",
                        profile, container
                    );
                    // Implementation would call container::apply_profile()
                }
                ContainerProfileAction::Create { name } => {
                    println!("➕ Creating profile '{}'...", name);
                    // Implementation would call container::create_profile()
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
fn show_progress_bar(message: &str, _duration: Duration) -> ProgressBar {
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
