//! Tab Modules
//!
//! Each tab is a separate module for maintainability.

pub mod display;
pub mod dlss;
pub mod fan;
pub mod game_profiles;
pub mod gamescope;
pub mod gpu;
pub mod hdr;
pub mod latency;
pub mod osd;
// Note: overclock tab removed from GUI (v0.8.5) - OC is CLI-only on Wayland
// pub mod overclock;
pub mod recording;
pub mod settings;
pub mod system;
pub mod vibrance;
pub mod vrr;

/// All available tabs in the GUI
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    // Core GPU tabs
    Gpu,
    Fan,
    // Display tabs
    Display,
    Vibrance,
    Hdr,
    Vrr,
    // Gaming tabs
    GameProfiles,
    Dlss,
    Osd,
    Latency,
    Gamescope,
    Recording,
    // System
    System,
    Settings,
}

impl Tab {
    /// Get the keyboard shortcut for this tab (1-9)
    pub fn shortcut(&self) -> Option<u8> {
        match self {
            Tab::Gpu => Some(1),
            Tab::Fan => Some(2),
            Tab::Display => Some(3),
            Tab::Vibrance => Some(4),
            Tab::Hdr => Some(5),
            Tab::GameProfiles => Some(6),
            Tab::Osd => Some(7),
            Tab::Settings => Some(8),
            _ => None,
        }
    }

    /// Get all tabs in sidebar order
    pub fn sidebar_tabs() -> Vec<(Tab, &'static str, &'static str)> {
        vec![
            // Core GPU
            (Tab::Gpu, super::icons::GPU, "GPU Status"),
            (Tab::Fan, super::icons::FAN_ICON, "Fan Control"),
            // Display
            (Tab::Display, super::icons::DISPLAY, "Display"),
            (Tab::Vibrance, super::icons::VIBRANCE, "Vibrance"),
            (Tab::Hdr, super::icons::HDR, "HDR"),
            (Tab::Vrr, super::icons::VRR, "VRR/G-Sync"),
            // Gaming
            (Tab::GameProfiles, super::icons::GAME, "Profiles"),
            (Tab::Dlss, super::icons::DLSS, "DLSS"),
            (Tab::Osd, super::icons::BENCHMARK, "OSD"),
            (Tab::Latency, super::icons::LATENCY, "Latency"),
            (Tab::Gamescope, super::icons::ROCKET, "Gamescope"),
            (Tab::Recording, super::icons::RECORD, "Recording"),
            // System
            (Tab::System, super::icons::SYSTEM, "System"),
            (Tab::Settings, super::icons::SETTINGS, "Settings"),
        ]
    }
}
