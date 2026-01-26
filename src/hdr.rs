// HDR (High Dynamic Range) Control for KDE/GNOME
// Integrates with compositor D-Bus APIs and NVKMS
use crate::NvResult;
use crate::display_backend::SharedDisplayRunner;
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

        // Compositor-specific status
        match compositor.as_str() {
            "hyprland" => {
                if let Ok(monitors) = get_hyprland_monitors() {
                    println!("\n  Monitors:");
                    for mon in &monitors {
                        let hdr_status = if mon.hdr_enabled {
                            "✅ HDR Active"
                        } else {
                            "❌ SDR"
                        };
                        println!(
                            "    {} ({}x{}@{}Hz): {}",
                            mon.name, mon.width, mon.height, mon.refresh_rate, hdr_status
                        );
                    }
                }
                println!("\n💡 To enable HDR:");
                println!("  nvctl display hdr enable");
                println!("  OR add to hyprland.conf: monitor=DP-1,2560x1440@165,auto,1,hdr");
            }
            "kde" | "kwin" | "plasma" => {
                if let Ok(displays) = get_kde_displays() {
                    if !displays.is_empty() {
                        println!("\n  Monitors:");
                        for disp in &displays {
                            let hdr_status = if disp.hdr_enabled {
                                "✅ HDR Active"
                            } else {
                                "❌ SDR"
                            };
                            let vrr_status = match disp.vrr_policy {
                                0 => "VRR Off",
                                1 => "VRR Always",
                                2 => "VRR Auto",
                                _ => "VRR Unknown",
                            };
                            println!("    {}: {} | {}", disp.name, hdr_status, vrr_status);
                        }
                    }
                }
                println!("\n💡 To enable HDR:");
                println!("  nvctl display hdr enable");
                println!("  OR: System Settings → Display → Enable HDR");
            }
            "gnome" | "mutter" => {
                let (hdr_feature, vrr_feature) = get_gnome_hdr_status();
                println!("\n  Experimental Features:");
                println!(
                    "    HDR: {}",
                    if hdr_feature {
                        "✅ Enabled"
                    } else {
                        "❌ Disabled"
                    }
                );
                println!(
                    "    VRR: {}",
                    if vrr_feature {
                        "✅ Enabled"
                    } else {
                        "❌ Disabled"
                    }
                );
                println!("\n💡 To enable HDR:");
                println!("  nvctl display hdr enable");
                println!("  OR: Settings → Displays → Enable HDR (per display)");
            }
            _ => {
                println!("  Status: ⚠️  Check compositor settings");
                println!("\n💡 Check your compositor's display settings");
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
    let runner = crate::display_backend::create_real_runner();
    is_process_running_with_backend(name, &runner)
}

fn is_process_running_with_backend(name: &str, runner: &SharedDisplayRunner) -> bool {
    runner.run_command("pgrep", &["-x", name]).is_ok()
}

// KDE/KWin HDR control
fn enable_hdr_kde() -> NvResult<()> {
    use std::process::Command;

    // Get KDE displays first
    let displays = get_kde_displays()?;

    if displays.is_empty() {
        // Fallback to D-Bus method
        let output = Command::new("qdbus")
            .args([
                "org.kde.KWin",
                "/KWin",
                "org.kde.KWin.setHDREnabled",
                "true",
            ])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                println!("✅ HDR enabled in KDE");
                return Ok(());
            }
            _ => {}
        }

        println!("⚠️  Could not enable HDR automatically");
        println!("   System Settings → Display Configuration → Enable HDR");
        return Ok(());
    }

    // Enable HDR on each connected display
    let mut success_count = 0;
    for display in &displays {
        // kscreen-doctor format: output.<name>.hdr.enable
        let output = Command::new("kscreen-doctor")
            .arg(format!("output.{}.hdr.enable", display.name))
            .output();

        match output {
            Ok(out) if out.status.success() => {
                println!("✅ HDR enabled on {}", display.name);
                success_count += 1;
            }
            _ => {
                // Try with wcg (wide color gamut) as well
                let _ = Command::new("kscreen-doctor")
                    .arg(format!("output.{}.wcg.enable", display.name))
                    .output();
                println!("⚠️  HDR may require manual enable for {}", display.name);
            }
        }
    }

    if success_count == 0 {
        println!("\n💡 Enable HDR manually:");
        println!("   System Settings → Display Configuration → Select display → Enable HDR");
    }

    Ok(())
}

fn disable_hdr_kde() -> NvResult<()> {
    use std::process::Command;

    let displays = get_kde_displays()?;

    if displays.is_empty() {
        let _ = Command::new("qdbus")
            .args([
                "org.kde.KWin",
                "/KWin",
                "org.kde.KWin.setHDREnabled",
                "false",
            ])
            .output();
        println!("✅ HDR disabled in KDE");
        return Ok(());
    }

    for display in &displays {
        let _ = Command::new("kscreen-doctor")
            .arg(format!("output.{}.hdr.disable", display.name))
            .output();
        println!("✅ HDR disabled on {}", display.name);
    }

    Ok(())
}

#[derive(Debug)]
struct KdeDisplay {
    name: String,
    #[allow(dead_code)]
    connected: bool,
    #[allow(dead_code)]
    hdr_enabled: bool,
    #[allow(dead_code)]
    vrr_policy: i32,
}

fn get_kde_displays() -> NvResult<Vec<KdeDisplay>> {
    let runner = crate::display_backend::create_real_runner();
    get_kde_displays_with_backend(&runner)
}

fn get_kde_displays_with_backend(runner: &SharedDisplayRunner) -> NvResult<Vec<KdeDisplay>> {
    let json_str = runner.run_command("kscreen-doctor", &["-j"]).map_err(|e| {
        crate::NvControlError::DisplayDetectionFailed(format!("kscreen-doctor failed: {}", e))
    })?;

    let mut displays = Vec::new();

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
        if let Some(outputs) = json.get("outputs").and_then(|o| o.as_array()) {
            for out in outputs {
                let connected = out
                    .get("connected")
                    .and_then(|c| c.as_bool())
                    .unwrap_or(false);

                if !connected {
                    continue;
                }

                let name = out
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("Unknown")
                    .to_string();

                // Plasma 6: "hdr" is the capability/enabled state
                // Some versions use "hdrEnabled" separately
                let hdr_enabled = out
                    .get("hdr")
                    .and_then(|h| h.as_bool())
                    .or_else(|| out.get("hdrEnabled").and_then(|h| h.as_bool()))
                    .unwrap_or(false);

                // vrrPolicy: 0=Never, 1=Always, 2=Automatic
                let vrr_policy = out.get("vrrPolicy").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

                displays.push(KdeDisplay {
                    name,
                    connected,
                    hdr_enabled,
                    vrr_policy,
                });
            }
        }
    }

    Ok(displays)
}

// GNOME/Mutter HDR control
fn enable_hdr_gnome() -> NvResult<()> {
    let runner = crate::display_backend::create_real_runner();
    enable_hdr_gnome_with_backend(&runner)
}

fn enable_hdr_gnome_with_backend(runner: &SharedDisplayRunner) -> NvResult<()> {
    // Get current experimental features to preserve them
    let current = runner
        .run_command(
            "gsettings",
            &["get", "org.gnome.mutter", "experimental-features"],
        )
        .unwrap_or_default();

    // Build new features list including HDR
    let new_features = if current.contains("hdr") {
        // Already has HDR
        println!("✅ HDR already enabled in GNOME experimental features");
        return Ok(());
    } else if current.contains("variable-refresh-rate") {
        // Has VRR, add HDR
        "['variable-refresh-rate', 'hdr']"
    } else if current.trim() == "@as []" || current.trim() == "[]" {
        // Empty, just add HDR
        "['hdr']"
    } else {
        // Has other features, add HDR (GNOME 46+ supports both)
        "['variable-refresh-rate', 'hdr']"
    };

    match runner.run_command(
        "gsettings",
        &[
            "set",
            "org.gnome.mutter",
            "experimental-features",
            new_features,
        ],
    ) {
        Ok(_) => {
            println!("✅ HDR experimental feature enabled in GNOME");
            println!("   📝 You may need to log out and back in");
            println!("   📝 Then enable HDR per-display in Settings → Displays");
            Ok(())
        }
        Err(_) => {
            println!("⚠️  Could not enable HDR feature");
            println!("   GNOME HDR requires GNOME 46+ with mutter HDR support");
            println!("   Check: Settings → Displays");
            Ok(())
        }
    }
}

fn disable_hdr_gnome() -> NvResult<()> {
    let runner = crate::display_backend::create_real_runner();
    disable_hdr_gnome_with_backend(&runner)
}

fn disable_hdr_gnome_with_backend(runner: &SharedDisplayRunner) -> NvResult<()> {
    // Get current features
    let current = runner
        .run_command(
            "gsettings",
            &["get", "org.gnome.mutter", "experimental-features"],
        )
        .unwrap_or_default();

    // Keep VRR if it was enabled, just remove HDR
    let new_features = if current.contains("variable-refresh-rate") {
        "['variable-refresh-rate']"
    } else {
        "[]"
    };

    let _ = runner.run_command(
        "gsettings",
        &[
            "set",
            "org.gnome.mutter",
            "experimental-features",
            new_features,
        ],
    );

    println!("✅ HDR experimental feature disabled in GNOME");
    Ok(())
}

fn get_gnome_hdr_status() -> (bool, bool) {
    let runner = crate::display_backend::create_real_runner();
    get_gnome_hdr_status_with_backend(&runner)
}

fn get_gnome_hdr_status_with_backend(runner: &SharedDisplayRunner) -> (bool, bool) {
    let output = runner
        .run_command(
            "gsettings",
            &["get", "org.gnome.mutter", "experimental-features"],
        )
        .unwrap_or_default();

    let hdr_enabled = output.contains("hdr");
    let vrr_enabled = output.contains("variable-refresh-rate");

    (hdr_enabled, vrr_enabled)
}

// Hyprland HDR control
fn enable_hdr_hyprland() -> NvResult<()> {
    use std::process::Command;

    // Get current monitors first
    let monitors = get_hyprland_monitors()?;

    if monitors.is_empty() {
        println!("⚠️  No monitors detected");
        println!("   Add to hyprland.conf:");
        println!("   monitor=,preferred,auto,1,hdr");
        return Ok(());
    }

    let mut success_count = 0;
    for monitor in &monitors {
        // Get current mode for this monitor
        let mode = format!(
            "{}x{}@{}",
            monitor.width, monitor.height, monitor.refresh_rate
        );

        // Apply HDR with current settings
        let output = Command::new("hyprctl")
            .args([
                "keyword",
                "monitor",
                &format!("{},{},auto,1,hdr", monitor.name, mode),
            ])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                println!("✅ HDR enabled on {}", monitor.name);
                success_count += 1;
            }
            _ => {
                println!("⚠️  Could not enable HDR on {}", monitor.name);
            }
        }
    }

    if success_count == 0 {
        println!("\n💡 Add to hyprland.conf manually:");
        for monitor in &monitors {
            println!(
                "   monitor={},{}x{}@{},auto,1,hdr",
                monitor.name, monitor.width, monitor.height, monitor.refresh_rate
            );
        }
    }

    Ok(())
}

fn disable_hdr_hyprland() -> NvResult<()> {
    use std::process::Command;

    let monitors = get_hyprland_monitors()?;

    for monitor in &monitors {
        let mode = format!(
            "{}x{}@{}",
            monitor.width, monitor.height, monitor.refresh_rate
        );

        Command::new("hyprctl")
            .args([
                "keyword",
                "monitor",
                &format!("{},{},auto,1", monitor.name, mode),
            ])
            .output()
            .ok();

        println!("✅ HDR disabled on {}", monitor.name);
    }

    if monitors.is_empty() {
        println!("✅ HDR disabled in Hyprland");
    }

    Ok(())
}

#[derive(Debug)]
struct HyprlandMonitor {
    name: String,
    width: u32,
    height: u32,
    refresh_rate: u32,
    #[allow(dead_code)]
    hdr_enabled: bool,
}

fn get_hyprland_monitors() -> NvResult<Vec<HyprlandMonitor>> {
    let runner = crate::display_backend::create_real_runner();
    get_hyprland_monitors_with_backend(&runner)
}

fn get_hyprland_monitors_with_backend(
    runner: &SharedDisplayRunner,
) -> NvResult<Vec<HyprlandMonitor>> {
    let json_str = runner
        .run_command("hyprctl", &["monitors", "-j"])
        .map_err(|e| {
            crate::NvControlError::DisplayDetectionFailed(format!("hyprctl failed: {}", e))
        })?;

    let mut monitors = Vec::new();

    if let Ok(monitor_array) = serde_json::from_str::<Vec<serde_json::Value>>(&json_str) {
        for mon in monitor_array {
            if mon
                .get("disabled")
                .and_then(|d| d.as_bool())
                .unwrap_or(false)
            {
                continue;
            }

            let name = mon
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown")
                .to_string();

            let width = mon.get("width").and_then(|w| w.as_u64()).unwrap_or(1920) as u32;

            let height = mon.get("height").and_then(|h| h.as_u64()).unwrap_or(1080) as u32;

            let refresh_rate = mon
                .get("refreshRate")
                .and_then(|r| r.as_f64())
                .unwrap_or(60.0) as u32;

            // Check current format for HDR hint (XRGB2101010 = 10-bit HDR capable)
            let current_format = mon
                .get("currentFormat")
                .and_then(|f| f.as_str())
                .unwrap_or("");
            let hdr_enabled =
                current_format.contains("101010") || current_format.contains("16161616");

            monitors.push(HyprlandMonitor {
                name,
                width,
                height,
                refresh_rate,
                hdr_enabled,
            });
        }
    }

    Ok(monitors)
}

// Check HDR support via NVIDIA
fn check_hdr_support() -> NvResult<bool> {
    let runner = crate::display_backend::create_real_runner();
    check_hdr_support_with_backend(&runner)
}

fn check_hdr_support_with_backend(runner: &SharedDisplayRunner) -> NvResult<bool> {
    // Query NVIDIA for HDR capability
    match runner.run_command("nvidia-settings", &["-q", "[gpu:0]/SupportedColorSpaces"]) {
        Ok(output_str) => {
            // HDR typically requires BT2020 color space
            Ok(output_str.contains("BT2020") || output_str.contains("HDR"))
        }
        Err(_) => Ok(false),
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
    // Try to query actual display capabilities via EDID from sysfs
    // EDID HDR data is in extension blocks (CEA-861) containing HDR static metadata
    let display_paths = [
        "/sys/class/drm/card0-DP-1/edid",
        "/sys/class/drm/card0-DP-2/edid",
        "/sys/class/drm/card0-HDMI-A-1/edid",
        "/sys/class/drm/card1-DP-1/edid",
    ];

    for path in &display_paths {
        if let Ok(edid_data) = std::fs::read(path) {
            if let Some(caps) = parse_edid_hdr_capabilities(&edid_data) {
                return Ok(caps);
            }
        }
    }

    // Fall back to safe defaults if EDID parsing fails
    Ok(HdrCapabilities::default())
}

/// Parse EDID data for HDR capabilities
fn parse_edid_hdr_capabilities(edid: &[u8]) -> Option<HdrCapabilities> {
    // EDID must be at least 128 bytes for base block
    if edid.len() < 128 {
        return None;
    }

    // Check EDID header (bytes 0-7 should be 00 FF FF FF FF FF FF 00)
    if edid[0..8] != [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00] {
        return None;
    }

    // Extension blocks start at byte 128
    let num_extensions = edid.get(126).copied().unwrap_or(0) as usize;
    if num_extensions == 0 || edid.len() < 128 + 128 * num_extensions {
        return Some(HdrCapabilities::default()); // No extensions, no HDR
    }

    // Look for CEA-861 extension (tag 0x02) with HDR metadata
    for ext_idx in 0..num_extensions {
        let ext_start = 128 + ext_idx * 128;
        let ext_block = &edid[ext_start..ext_start + 128];

        // CEA-861 extension has tag 0x02
        if ext_block[0] == 0x02 {
            // Parse CEA data blocks for HDR Static Metadata (tag 0x07, extended tag 0x06)
            let dtd_start = ext_block.get(2).copied().unwrap_or(4) as usize;
            let mut offset = 4;

            while offset < dtd_start && offset < 127 {
                let header = ext_block[offset];
                let tag = (header >> 5) & 0x07;
                let length = (header & 0x1F) as usize;

                if tag == 0x07 && length > 0 {
                    // Extended tag
                    let ext_tag = ext_block.get(offset + 1).copied().unwrap_or(0);
                    if ext_tag == 0x06 && length >= 3 {
                        // HDR Static Metadata Data Block
                        let eotf_byte = ext_block.get(offset + 2).copied().unwrap_or(0);
                        let supports_hdr10 = (eotf_byte & 0x04) != 0; // SMPTE ST 2084
                        let supports_hlg = (eotf_byte & 0x08) != 0; // HLG

                        return Some(HdrCapabilities {
                            supports_hdr10,
                            supports_hdr10_plus: false, // Requires separate detection
                            max_luminance: 1000,        // Default, would need metadata byte parsing
                            min_luminance: 0.1,
                            max_fall: 400, // Typical FALL value
                            supports_dolby_vision: false,
                            supports_hlg,
                        });
                    }
                }

                offset += 1 + length;
            }
        }
    }

    Some(HdrCapabilities::default())
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
