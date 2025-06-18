use clap::{Parser, Subcommand};
use nvcontrol::{display, drivers, fan, gpu, overclocking, upscaling, vibrance, vrr, monitoring, power};

#[derive(Parser)]
#[command(name = "nvctl", version, about = "NVIDIA Control CLI - Advanced GPU Management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Gpu {
        #[command(subcommand)]
        subcommand: GpuSubcommand,
    },
    Display {
        #[command(subcommand)]
        subcommand: DisplaySubcommand,
    },
    Fan {
        #[command(subcommand)]
        subcommand: FanSubcommand,
    },
    Overclock {
        #[command(subcommand)]
        subcommand: OverclockSubcommand,
    },
    Vrr {
        #[command(subcommand)]
        subcommand: VrrSubcommand,
    },
    Upscaling {
        #[command(subcommand)]
        subcommand: UpscalingSubcommand,
    },
    Drivers {
        #[command(subcommand)]
        subcommand: DriversSubcommand,
    },
    Power {
        #[command(subcommand)]
        subcommand: PowerSubcommand,
    },
}

#[derive(Subcommand)]
enum GpuSubcommand {
    /// Show comprehensive GPU information
    Info,
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
        percentage: u32 
    },
    /// Set vibrance for specific display
    SetDisplay { 
        #[arg(help = "Display index (0, 1, 2, etc.)")]
        display: usize,
        #[arg(help = "Vibrance percentage (0-200)")]
        percentage: u32 
    },
    /// Set vibrance using raw nvibrant values for multiple displays
    SetRaw {
        /// Raw vibrance levels (-1024 to 1023) for each display
        #[arg(required = true, help = "Raw vibrance values for each display (e.g. 512 1023)")]
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
    /// Show comprehensive power information
    Info,
    /// Set power management profile
    Profile {
        /// Profile: performance, balanced, powersaver, or custom name
        profile: String,
    },
    /// Set power limit percentage
    Limit {
        /// Power limit percentage (50-120)
        percentage: u32,
    },
    /// Toggle persistence mode
    Persistence {
        /// Enable or disable persistence mode
        #[arg(value_parser = clap::builder::BoolishValueParser::new())]
        enabled: bool,
    },
    /// Monitor power consumption
    Monitor {
        /// Duration in seconds to monitor
        #[arg(short, long, default_value = "60")]
        duration: u32,
    },
    /// Create power management automation
    Automate,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Gpu { subcommand } => match subcommand {
            GpuSubcommand::Info => gpu::get_gpu_info(),
            GpuSubcommand::Stat => {
                // Launch the new advanced TUI dashboard
                println!("🚀 Launching nvcontrol TUI Dashboard...");
                match nvcontrol::tui::TuiApp::new().run() {
                    Ok(()) => {},
                    Err(e) => eprintln!("❌ TUI error: {}", e),
                }
            },
            GpuSubcommand::Capabilities => match overclocking::get_gpu_capabilities() {
                Ok(_caps) => println!("✅ GPU overclocking capabilities detected"),
                Err(e) => eprintln!("Error getting capabilities: {e}"),
            },
            GpuSubcommand::Benchmark { duration, test_type, intensity, log } => {
                match monitoring::run_gpu_benchmark(*duration, test_type, intensity, *log) {
                    Ok(()) => println!("✅ Benchmark completed"),
                    Err(e) => eprintln!("❌ Benchmark failed: {}", e),
                }
            },
            GpuSubcommand::Watch { interval, count } => {
                match monitoring::live_gpu_watch(*interval, *count) {
                    Ok(()) => {},
                    Err(e) => eprintln!("❌ Watch failed: {}", e),
                }
            },
            GpuSubcommand::Export { format, output, duration } => {
                match monitoring::export_gpu_metrics(format, output.as_deref(), *duration) {
                    Ok(()) => println!("✅ Export completed"),
                    Err(e) => eprintln!("❌ Export failed: {}", e),
                }
            },
            GpuSubcommand::Stress { duration, intensity, log } => {
                match monitoring::run_gpu_benchmark(duration * 60, "all", intensity, *log) {
                    Ok(()) => println!("✅ Stress test completed"),
                    Err(e) => eprintln!("❌ Stress test failed: {}", e),
                }
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
                    VibranceSubcommand::Get => {
                        match vibrance::get_displays() {
                            Ok(displays) => {
                                println!("Connected Displays:");
                                for (i, display) in displays.iter().enumerate() {
                                    match vibrance::get_display_vibrance(i) {
                                        Ok(vibrance_val) => {
                                            let percentage = vibrance::vibrance_to_percentage(vibrance_val);
                                            println!("  {}: {}% vibrance", display, percentage);
                                        }
                                        Err(e) => println!("  {}: Error - {}", display, e),
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error getting displays: {}", e);
                                if !vibrance::is_available() {
                                    eprintln!("Note: nvibrant may not be installed or NVIDIA drivers not available");
                                }
                            }
                        }
                    }
                    VibranceSubcommand::Set { percentage } => {
                        let vibrance_val = vibrance::percentage_to_vibrance(*percentage);
                        match vibrance::set_vibrance_all(vibrance_val) {
                            Ok(()) => println!("Set all displays to {}% vibrance", percentage),
                            Err(e) => eprintln!("Failed to set vibrance: {}", e),
                        }
                    }
                    VibranceSubcommand::SetDisplay { display, percentage } => {
                        let vibrance_val = vibrance::percentage_to_vibrance(*percentage);
                        let display_values = vec![(*display, vibrance_val)];
                        match vibrance::set_vibrance(&display_values) {
                            Ok(()) => println!("Set display {} to {}% vibrance", display, percentage),
                            Err(e) => eprintln!("Failed to set vibrance for display {}: {}", display, e),
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
                    VibranceSubcommand::List => {
                        match vibrance::get_displays() {
                            Ok(displays) => {
                                println!("Available Displays:");
                                for (i, display) in displays.iter().enumerate() {
                                    println!("  [{}] {}", i, display);
                                }
                            }
                            Err(e) => eprintln!("Failed to list displays: {}", e),
                        }
                    }
                    VibranceSubcommand::Reset => {
                        match vibrance::set_vibrance_all(0) {
                            Ok(()) => println!("Reset all displays to default vibrance (100%)"),
                            Err(e) => eprintln!("Failed to reset vibrance: {}", e),
                        }
                    }
                    VibranceSubcommand::Info => {
                        match vibrance::get_driver_info() {
                            Ok(info) => {
                                println!("Vibrance Support Information:");
                                println!("  {}", info);
                                println!("  nvibrant available: {}", if vibrance::is_available() { "Yes" } else { "No" });
                                
                                if !vibrance::is_available() {
                                    println!("\nTo install nvibrant:");
                                    println!("  pip install nvibrant");
                                    println!("  # OR");
                                    println!("  uvx nvibrant");
                                }
                            }
                            Err(e) => eprintln!("Failed to get driver info: {}", e),
                        }
                    }
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
            PowerSubcommand::Info => {
                match power::get_power_info() {
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
                            
                            println!("  Persistence Mode: {}", if info.persistence_mode { "Enabled" } else { "Disabled" });
                            println!();
                        }
                    }
                    Err(e) => eprintln!("❌ Failed to get power info: {}", e),
                }
            },
            PowerSubcommand::Profile { profile } => {
                match power::set_power_profile(profile) {
                    Ok(()) => println!("✅ Power profile applied successfully"),
                    Err(e) => eprintln!("❌ Failed to set power profile: {}", e),
                }
            },
            PowerSubcommand::Limit { percentage } => {
                let percentage = (*percentage).min(120).max(50);
                match power::set_power_limit_percentage(percentage) {
                    Ok(()) => println!("✅ Power limit set to {}%", percentage),
                    Err(e) => eprintln!("❌ Failed to set power limit: {}", e),
                }
            },
            PowerSubcommand::Persistence { enabled } => {
                match power::set_persistence_mode(*enabled) {
                    Ok(()) => println!("✅ Persistence mode {}", if *enabled { "enabled" } else { "disabled" }),
                    Err(e) => eprintln!("❌ Failed to set persistence mode: {}", e),
                }
            },
            PowerSubcommand::Monitor { duration } => {
                match power::monitor_power_consumption(*duration) {
                    Ok(()) => {},
                    Err(e) => eprintln!("❌ Power monitoring failed: {}", e),
                }
            },
            PowerSubcommand::Automate => {
                match power::create_power_automation() {
                    Ok(()) => println!("✅ Power automation configured"),
                    Err(e) => eprintln!("❌ Failed to setup automation: {}", e),
                }
            },
        },
    }
}
