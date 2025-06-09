use clap::{Parser, Subcommand};
use nvcontrol::{gpu, display, vibrance};

#[derive(Parser)]
#[command(name = "nvctl", version, about = "NVIDIA Control CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GpuInfo,
    DisplayInfo,
    SetVibrance { level: u8 },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::GpuInfo => gpu::get_gpu_info(),
        Commands::DisplayInfo => display::get_display_info(),
        Commands::SetVibrance { level } => vibrance::set_vibrance(level),
    }
}
