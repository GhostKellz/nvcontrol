use clap::{Parser, Subcommand};
use nvcontrol::{gpu, display, vibrance};

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
                println!("Detected {} display(s):", count);
                for i in 0..count {
                    println!("  Display {}", i);
                }
            }
            DisplaySubcommand::Vibrance { levels } => vibrance::set_vibrance(&levels),
        },
    }
}
