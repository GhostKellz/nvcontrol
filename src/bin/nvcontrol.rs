//! nvcontrol - NVIDIA GPU Control Panel GUI
//!
//! A modern GUI for managing NVIDIA GPUs on Linux.
//! Uses the modular GUI system from src/gui/.

#[cfg(feature = "gui")]
fn main() -> eframe::Result<()> {
    nvcontrol::gui::run()
}

#[cfg(not(feature = "gui"))]
fn main() {
    eprintln!("Error: nvcontrol requires the 'gui' feature.");
    eprintln!("Build with: cargo build --bin nvcontrol --features gui");
    std::process::exit(1);
}
