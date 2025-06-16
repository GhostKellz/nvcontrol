use clap::{Parser, Subcommand};
use nvcontrol::{display, gpu, vibrance, fan};

#[derive(Parser)]
#[command(name = "nvctl", version, about = "NVIDIA Control CLI", long_about = None)]
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
}

#[derive(Subcommand)]
enum GpuSubcommand {
    Info,
    Stat,
    // Future: Clocks, Power, etc.
}

#[derive(Subcommand)]
enum DisplaySubcommand {
    Info,
    Ls,
    Vibrance {
        /// Vibrance levels for each display (e.g. 512 1023)
        #[arg(required = true)]
        levels: Vec<i16>,
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

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Gpu { subcommand } => match subcommand {
            GpuSubcommand::Info => gpu::get_gpu_info(),
            GpuSubcommand::Stat => gpu::monitor_gpu_stat(),
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
            DisplaySubcommand::Vibrance { levels } => {
                if let Err(e) = vibrance::set_vibrance(&levels) {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        },
        Command::Fan { subcommand } => match subcommand {
            FanSubcommand::Info => {
                let fans = fan::list_fans();
                println!("Fan Information:");
                for fan in fans {
                    println!("  Fan {}: {} RPM, {}%, Controllable: {}", 
                        fan.id, 
                        fan.rpm.unwrap_or(0), 
                        fan.percent.unwrap_or(0),
                        fan.controllable
                    );
                }
            }
            FanSubcommand::Set { fan_id, percent } => {
                fan::set_fan_speed(fan_id, percent);
                println!("Set fan {fan_id} to {percent}%");
            }
        },
    }
}
