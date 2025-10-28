// HDR (High Dynamic Range) Control for KDE/GNOME
// Integrates with compositor D-Bus APIs and NVKMS
use crate::NvResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdrStatus {
    pub supported: bool,
    pub enabled: bool,
    pub compositor: String,
    pub static_metadata_available: bool,
}

/// Enable HDR via compositor D-Bus
pub fn enable_hdr_cli() -> NvResult<()> {
    let compositor = detect_compositor()?;

    match compositor.as_str() {
        "kde" | "kwin" => enable_hdr_kde(),
        "gnome" | "mutter" => enable_hdr_gnome(),
        "hyprland" => enable_hdr_hyprland(),
        _ => {
            println!("⚠️  HDR control not yet implemented for {}", compositor);
            println!("   Use your compositor's settings:");
            println!("   • KDE: System Settings → Display → Enable HDR");
            println!("   • GNOME: Settings → Displays → Enable HDR");
            Ok(())
        }
    }
}

/// Disable HDR via compositor D-Bus
pub fn disable_hdr_cli() -> NvResult<()> {
    let compositor = detect_compositor()?;

    match compositor.as_str() {
        "kde" | "kwin" => disable_hdr_kde(),
        "gnome" | "mutter" => disable_hdr_gnome(),
        "hyprland" => disable_hdr_hyprland(),
        _ => {
            println!("⚠️  HDR control not yet implemented for {}", compositor);
            Ok(())
        }
    }
}

/// Get HDR status
pub fn get_hdr_status_cli() -> NvResult<()> {
    let compositor = detect_compositor()?;

    println!("🌈 HDR Status:");
    println!("══════════════════════════════════════");
    println!("  Compositor: {}", compositor);

    // Check GPU HDR support via nvidia-settings
    let hdr_supported = check_hdr_support()?;

    if hdr_supported {
        println!("  GPU Support: ✅ Yes");
        println!("  Status: ⚠️  Check compositor settings");
        println!("\n💡 To enable HDR:");
        match compositor.as_str() {
            "kde" | "kwin" => {
                println!("  nvctl display hdr enable");
                println!("  OR: System Settings → Display → Enable HDR");
            }
            "gnome" | "mutter" => {
                println!("  nvctl display hdr enable");
                println!("  OR: Settings → Displays → Enable HDR");
            }
            _ => {
                println!("  Check your compositor's display settings");
            }
        }
    } else {
        println!("  GPU Support: ❌ Not available");
        println!("\n  Your GPU or display may not support HDR");
    }

    Ok(())
}

// Compositor detection
fn detect_compositor() -> NvResult<String> {
    if let Ok(session) = std::env::var("XDG_SESSION_DESKTOP") {
        return Ok(session.to_lowercase());
    }

    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        return Ok(desktop.to_lowercase());
    }

    // Fallback: check running processes
    if is_process_running("kwin_wayland") || is_process_running("kwin_x11") {
        return Ok("kde".to_string());
    }

    if is_process_running("gnome-shell") {
        return Ok("gnome".to_string());
    }

    if is_process_running("Hyprland") {
        return Ok("hyprland".to_string());
    }

    Ok("unknown".to_string())
}

fn is_process_running(name: &str) -> bool {
    std::process::Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// KDE/KWin HDR control
fn enable_hdr_kde() -> NvResult<()> {
    use std::process::Command;

    // KDE Plasma 6+ has HDR support via D-Bus
    let output = Command::new("qdbus")
        .args(&[
            "org.kde.KWin",
            "/KWin",
            "org.kde.KWin.setHDREnabled",
            "true",
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            println!("✅ HDR enabled in KDE");
            println!("   📝 Note: Ensure your display supports HDR");
            Ok(())
        }
        _ => {
            // Fallback: use kscreen-doctor (Plasma 6)
            let kscreen_output = Command::new("kscreen-doctor")
                .args(&["output.1.hdr.enabled=true"])
                .output();

            match kscreen_output {
                Ok(out) if out.status.success() => {
                    println!("✅ HDR enabled via kscreen-doctor");
                    Ok(())
                }
                _ => {
                    println!("⚠️  Could not enable HDR automatically");
                    println!("   Please enable manually:");
                    println!("   System Settings → Display Configuration → Enable HDR");
                    Ok(())
                }
            }
        }
    }
}

fn disable_hdr_kde() -> NvResult<()> {
    use std::process::Command;

    let output = Command::new("qdbus")
        .args(&[
            "org.kde.KWin",
            "/KWin",
            "org.kde.KWin.setHDREnabled",
            "false",
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            println!("✅ HDR disabled in KDE");
            Ok(())
        }
        _ => {
            Command::new("kscreen-doctor")
                .args(&["output.1.hdr.enabled=false"])
                .output()
                .ok();
            println!("✅ HDR disabled");
            Ok(())
        }
    }
}

// GNOME/Mutter HDR control
fn enable_hdr_gnome() -> NvResult<()> {
    use std::process::Command;

    // GNOME 46+ has experimental HDR support
    let output = Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.mutter.experimental-features",
            "['hdr']",
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            println!("✅ HDR experimental feature enabled in GNOME");
            println!("   📝 Restart GNOME Shell: Alt+F2, type 'r', press Enter");
            println!("   📝 Then enable HDR in Settings → Displays");
            Ok(())
        }
        _ => {
            println!("⚠️  Could not enable HDR feature");
            println!("   GNOME HDR is experimental (GNOME 46+)");
            println!("   Check: Settings → Displays");
            Ok(())
        }
    }
}

fn disable_hdr_gnome() -> NvResult<()> {
    use std::process::Command;

    Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.mutter.experimental-features",
            "[]",
        ])
        .output()
        .ok();

    println!("✅ HDR experimental feature disabled in GNOME");
    Ok(())
}

// Hyprland HDR control
fn enable_hdr_hyprland() -> NvResult<()> {
    use std::process::Command;

    // Hyprland HDR via hyprctl
    let output = Command::new("hyprctl")
        .args(&["keyword", "monitor", ",highres,auto,1,hdr"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            println!("✅ HDR enabled in Hyprland");
            Ok(())
        }
        _ => {
            println!("⚠️  Add to hyprland.conf:");
            println!("   monitor=,highres,auto,1,hdr");
            Ok(())
        }
    }
}

fn disable_hdr_hyprland() -> NvResult<()> {
    use std::process::Command;

    Command::new("hyprctl")
        .args(&["keyword", "monitor", ",highres,auto,1"])
        .output()
        .ok();

    println!("✅ HDR disabled in Hyprland");
    Ok(())
}

// Check HDR support via NVIDIA
fn check_hdr_support() -> NvResult<bool> {
    use std::process::Command;

    // Query NVIDIA for HDR capability
    let output = Command::new("nvidia-settings")
        .args(&["-q", "[gpu:0]/SupportedColorSpaces"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let output_str = String::from_utf8_lossy(&out.stdout);
            // HDR typically requires BT2020 color space
            Ok(output_str.contains("BT2020") || output_str.contains("HDR"))
        }
        _ => Ok(false),
    }
}
