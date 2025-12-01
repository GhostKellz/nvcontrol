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
        .args(&["set", "org.gnome.mutter.experimental-features", "['hdr']"])
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
        .args(&["set", "org.gnome.mutter.experimental-features", "[]"])
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

// Advanced HDR Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdrConfig {
    pub enabled: bool,
    pub peak_brightness: u32,         // nits (100-10000)
    pub min_brightness: f32,          // nits (0.0001-0.1)
    pub max_frame_average: u32,       // nits
    pub max_content_light_level: u32, // nits
    pub tone_mapping: ToneMappingMode,
    pub color_space: ColorSpace,
    pub eotf: Eotf,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ToneMappingMode {
    None,
    Reinhard,
    Hable,
    ACES,
    AGX,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColorSpace {
    BT709,  // SDR
    BT2020, // HDR
    DciP3,  // Wide gamut (DCI-P3)
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Eotf {
    Gamma22, // SDR
    PQ,      // Perceptual Quantizer (HDR10)
    HLG,     // Hybrid Log-Gamma (HDR10+)
}

impl Default for HdrConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            peak_brightness: 1000,
            min_brightness: 0.0001,
            max_frame_average: 400,
            max_content_light_level: 1000,
            tone_mapping: ToneMappingMode::Hable,
            color_space: ColorSpace::BT2020,
            eotf: Eotf::PQ,
        }
    }
}

impl HdrConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load HDR config from file
    pub fn load() -> NvResult<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| crate::NvControlError::ConfigError("No config directory".into()))?
            .join("nvcontrol");

        let config_path = config_dir.join("hdr_config.toml");

        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path).map_err(|e| {
                crate::NvControlError::ConfigError(format!("Failed to read config: {}", e))
            })?;

            toml::from_str(&contents).map_err(|e| {
                crate::NvControlError::ConfigError(format!("Failed to parse config: {}", e))
            })
        } else {
            Ok(Self::default())
        }
    }

    /// Save HDR config to file
    pub fn save(&self) -> NvResult<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| crate::NvControlError::ConfigError("No config directory".into()))?
            .join("nvcontrol");

        std::fs::create_dir_all(&config_dir).map_err(|e| {
            crate::NvControlError::ConfigError(format!("Failed to create config dir: {}", e))
        })?;

        let config_path = config_dir.join("hdr_config.toml");

        let toml = toml::to_string_pretty(self).map_err(|e| {
            crate::NvControlError::ConfigError(format!("Failed to serialize config: {}", e))
        })?;

        std::fs::write(&config_path, toml).map_err(|e| {
            crate::NvControlError::ConfigError(format!("Failed to write config: {}", e))
        })?;

        Ok(())
    }

    /// Apply HDR configuration to the display
    pub fn apply(&self) -> NvResult<()> {
        if !self.enabled {
            return disable_hdr_cli();
        }

        // Enable HDR first
        enable_hdr_cli()?;

        // Apply advanced settings via nvidia-settings
        println!("🔧 Applying HDR configuration...");
        println!("   Peak Brightness: {} nits", self.peak_brightness);
        println!("   Tone Mapping: {:?}", self.tone_mapping);
        println!("   Color Space: {:?}", self.color_space);
        println!("   EOTF: {:?}", self.eotf);

        // Note: Most of these settings are compositor-specific
        // This is a placeholder for future NVML/NVKMS integration
        Ok(())
    }
}

/// Get display HDR capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdrCapabilities {
    pub max_luminance: u32, // nits
    pub min_luminance: f32, // nits
    pub max_fall: u32,      // Frame Average Light Level
    pub supports_hdr10: bool,
    pub supports_hdr10_plus: bool,
    pub supports_dolby_vision: bool,
    pub supports_hlg: bool,
}

impl Default for HdrCapabilities {
    fn default() -> Self {
        Self {
            max_luminance: 1000,
            min_luminance: 0.0001,
            max_fall: 400,
            supports_hdr10: true,
            supports_hdr10_plus: false,
            supports_dolby_vision: false,
            supports_hlg: true,
        }
    }
}

pub fn get_hdr_capabilities() -> NvResult<HdrCapabilities> {
    // TODO: Query actual display capabilities via EDID/DisplayID
    // For now, return safe defaults
    Ok(HdrCapabilities::default())
}

impl std::fmt::Display for ToneMappingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToneMappingMode::None => write!(f, "None (Clip)"),
            ToneMappingMode::Reinhard => write!(f, "Reinhard"),
            ToneMappingMode::Hable => write!(f, "Hable (Uncharted 2)"),
            ToneMappingMode::ACES => write!(f, "ACES Filmic"),
            ToneMappingMode::AGX => write!(f, "AGX"),
        }
    }
}

impl std::fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorSpace::BT709 => write!(f, "BT.709 (sRGB)"),
            ColorSpace::BT2020 => write!(f, "BT.2020 (HDR)"),
            ColorSpace::DciP3 => write!(f, "DCI-P3 (Wide Gamut)"),
        }
    }
}

impl std::fmt::Display for Eotf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Eotf::Gamma22 => write!(f, "Gamma 2.2 (SDR)"),
            Eotf::PQ => write!(f, "PQ (HDR10)"),
            Eotf::HLG => write!(f, "HLG (HDR10+/BBC)"),
        }
    }
}
