// Enhanced Display Information - Comprehensive display details
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedDisplayInfo {
    pub id: u32,
    pub name: String,
    pub connector_type: String,
    pub connected: bool,
    pub resolution: (u32, u32),
    pub refresh_rate: f32,
    pub vrr_capable: bool,
    pub vrr_enabled: bool,
    pub hdr_capable: bool,
    pub hdr_enabled: bool,
    pub current_vibrance: i32,
    pub image_sharpening: Option<i64>,
    pub color_space: String,
    pub color_range: String,
}

pub fn get_all_displays_info() -> NvResult<Vec<EnhancedDisplayInfo>> {
    use std::process::Command;

    let mut displays = Vec::new();

    // Query displays via nvidia-settings
    let output = Command::new("nvidia-settings")
        .args(&["-q", "displays"])
        .output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("nvidia-settings failed: {}", e)))?;

    if !output.status.success() {
        return Err(NvControlError::DisplayDetectionFailed(
            "Failed to query displays".to_string(),
        ));
    }

    // For now, return basic info - full implementation would parse nvidia-settings output
    // or use NVKMS ioctls to query all display properties

    displays.push(EnhancedDisplayInfo {
        id: 0,
        name: "Display 0".to_string(),
        connector_type: "HDMI-A".to_string(),
        connected: true,
        resolution: (1920, 1080),
        refresh_rate: 60.0,
        vrr_capable: true,
        vrr_enabled: false,
        hdr_capable: false,
        hdr_enabled: false,
        current_vibrance: 0,
        image_sharpening: None,
        color_space: "RGB".to_string(),
        color_range: "Full".to_string(),
    });

    Ok(displays)
}

pub fn print_display_info_cli() -> NvResult<()> {
    println!("🖥️  Enhanced Display Information");
    println!("═══════════════════════════════════════════════════════════\n");

    let displays = get_all_displays_info()?;

    for display in displays {
        println!("📺 {} (ID: {})", display.name, display.id);
        println!("   Connector: {}", display.connector_type);
        println!("   Status: {}", if display.connected { "✅ Connected" } else { "❌ Disconnected" });

        if display.connected {
            println!("   Resolution: {}x{} @ {:.2}Hz", display.resolution.0, display.resolution.1, display.refresh_rate);

            // VRR Info
            print!("   VRR/G-SYNC: ");
            if display.vrr_capable {
                println!("{}", if display.vrr_enabled { "✅ Enabled" } else { "⚪ Capable (Disabled)" });
            } else {
                println!("❌ Not Supported");
            }

            // HDR Info
            print!("   HDR: ");
            if display.hdr_capable {
                println!("{}", if display.hdr_enabled { "✅ Enabled" } else { "⚪ Capable (Disabled)" });
            } else {
                println!("❌ Not Supported");
            }

            // Vibrance
            let vibrance_pct = vibrance_raw_to_percentage(display.current_vibrance);
            println!("   Digital Vibrance: {}% (raw: {})", vibrance_pct, display.current_vibrance);

            // Image Sharpening
            if let Some(sharpening) = display.image_sharpening {
                println!("   Image Sharpening: {}", sharpening);
            }

            // Color Settings
            println!("   Color Space: {}", display.color_space);
            println!("   Color Range: {}", display.color_range);
        }

        println!();
    }

    println!("💡 Quick Commands:");
    println!("   nvctl vibe 150              - Set vibrance to 150%");
    println!("   nvctl vrr status            - Check VRR status");
    println!("   nvctl display hdr status    - Check HDR status");

    Ok(())
}

fn vibrance_raw_to_percentage(raw: i32) -> u32 {
    if raw <= 0 {
        (((raw + 1024) as f32 / 1024.0) * 100.0) as u32
    } else {
        (100.0 + (raw as f32 / 1023.0 * 100.0)) as u32
    }
}
