//! Tab Modules
//!
//! Each tab is a separate module for maintainability.

pub mod display;
pub mod fan;
pub mod game_profiles;
pub mod gamescope;
pub mod gpu;
pub mod hdr;
pub mod latency;
pub mod osd;
pub mod overclock;
pub mod recording;
pub mod settings;
pub mod vibrance;
pub mod vrr;

/// All available tabs in the GUI
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    // Core GPU tabs
    Gpu,
    Overclock,
    Fan,
    // Display tabs
    Display,
    Vibrance,
    Hdr,
    Vrr,
    // Gaming tabs
    GameProfiles,
    Osd,
    Latency,
    Gamescope,
    Recording,
    // System
    Settings,
}

impl Tab {
    /// Get the keyboard shortcut for this tab (1-9)
    pub fn shortcut(&self) -> Option<u8> {
        match self {
            Tab::Gpu => Some(1),
            Tab::Overclock => Some(2),
            Tab::Fan => Some(3),
            Tab::Display => Some(4),
            Tab::Vibrance => Some(5),
            Tab::Hdr => Some(6),
            Tab::GameProfiles => Some(7),
            Tab::Osd => Some(8),
            Tab::Settings => Some(9),
            _ => None,
        }
    }

    /// Get all tabs in sidebar order
    pub fn sidebar_tabs() -> Vec<(Tab, &'static str, &'static str)> {
        vec![
            // Core GPU
            (Tab::Gpu, super::icons::GPU, "GPU Status"),
            (Tab::Overclock, super::icons::OVERCLOCK, "Overclock"),
            (Tab::Fan, super::icons::FAN_ICON, "Fan Control"),
            // Display
            (Tab::Display, super::icons::DISPLAY, "Display"),
            (Tab::Vibrance, super::icons::VIBRANCE, "Vibrance"),
            (Tab::Hdr, super::icons::HDR, "HDR"),
            (Tab::Vrr, super::icons::VRR, "VRR/G-Sync"),
            // Gaming
            (Tab::GameProfiles, super::icons::GAME, "Profiles"),
            (Tab::Osd, super::icons::BENCHMARK, "OSD"),
            (Tab::Latency, super::icons::LATENCY, "Latency"),
            (Tab::Gamescope, super::icons::ROCKET, "Gamescope"),
            (Tab::Recording, super::icons::RECORD, "Recording"),
            // System
            (Tab::Settings, super::icons::SETTINGS, "Settings"),
        ]
    }
}
