/// GUI Themes for nvcontrol
///
/// Modern theming system with dark/light modes and GPU manufacturer presets
use serde::{Deserialize, Serialize};

/// Color scheme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    // Primary colors
    pub primary: RgbaColor,
    pub secondary: RgbaColor,
    pub accent: RgbaColor,

    // Background colors
    pub background: RgbaColor,
    pub surface: RgbaColor,
    pub elevated_surface: RgbaColor,

    // Text colors
    pub text_primary: RgbaColor,
    pub text_secondary: RgbaColor,
    pub text_disabled: RgbaColor,

    // Status colors
    pub success: RgbaColor,
    pub warning: RgbaColor,
    pub error: RgbaColor,
    pub info: RgbaColor,

    // Graph colors
    pub graph_temp: RgbaColor,
    pub graph_load: RgbaColor,
    pub graph_power: RgbaColor,
    pub graph_clock: RgbaColor,
    pub graph_memory: RgbaColor,

    // Widget colors
    pub slider_track: RgbaColor,
    pub slider_thumb: RgbaColor,
    pub button_normal: RgbaColor,
    pub button_hover: RgbaColor,
    pub button_pressed: RgbaColor,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Theme presets
pub struct ThemePresets;

impl ThemePresets {
    /// NVIDIA GeForce (Dark Green)
    pub fn nvidia_dark() -> ColorScheme {
        ColorScheme {
            primary: RgbaColor::rgb(118, 185, 0), // NVIDIA Green
            secondary: RgbaColor::rgb(0, 0, 0),
            accent: RgbaColor::rgb(147, 209, 46),

            background: RgbaColor::rgb(18, 18, 18),
            surface: RgbaColor::rgb(28, 28, 28),
            elevated_surface: RgbaColor::rgb(38, 38, 38),

            text_primary: RgbaColor::rgb(255, 255, 255),
            text_secondary: RgbaColor::rgb(180, 180, 180),
            text_disabled: RgbaColor::rgb(100, 100, 100),

            success: RgbaColor::rgb(76, 175, 80),
            warning: RgbaColor::rgb(255, 193, 7),
            error: RgbaColor::rgb(244, 67, 54),
            info: RgbaColor::rgb(33, 150, 243),

            graph_temp: RgbaColor::rgb(244, 67, 54),
            graph_load: RgbaColor::rgb(118, 185, 0),
            graph_power: RgbaColor::rgb(255, 193, 7),
            graph_clock: RgbaColor::rgb(33, 150, 243),
            graph_memory: RgbaColor::rgb(156, 39, 176),

            slider_track: RgbaColor::rgb(60, 60, 60),
            slider_thumb: RgbaColor::rgb(118, 185, 0),
            button_normal: RgbaColor::rgb(38, 38, 38),
            button_hover: RgbaColor::rgb(48, 48, 48),
            button_pressed: RgbaColor::rgb(118, 185, 0),
        }
    }

    /// ASUS ROG (Red/Black)
    pub fn asus_rog() -> ColorScheme {
        ColorScheme {
            primary: RgbaColor::rgb(255, 0, 0), // ROG Red
            secondary: RgbaColor::rgb(0, 0, 0),
            accent: RgbaColor::rgb(255, 69, 58),

            background: RgbaColor::rgb(10, 10, 10),
            surface: RgbaColor::rgb(20, 20, 20),
            elevated_surface: RgbaColor::rgb(30, 30, 30),

            text_primary: RgbaColor::rgb(255, 255, 255),
            text_secondary: RgbaColor::rgb(200, 200, 200),
            text_disabled: RgbaColor::rgb(100, 100, 100),

            success: RgbaColor::rgb(76, 175, 80),
            warning: RgbaColor::rgb(255, 152, 0),
            error: RgbaColor::rgb(255, 0, 0),
            info: RgbaColor::rgb(33, 150, 243),

            graph_temp: RgbaColor::rgb(255, 0, 0),
            graph_load: RgbaColor::rgb(255, 69, 58),
            graph_power: RgbaColor::rgb(255, 152, 0),
            graph_clock: RgbaColor::rgb(138, 180, 248),
            graph_memory: RgbaColor::rgb(186, 104, 200),

            slider_track: RgbaColor::rgb(50, 50, 50),
            slider_thumb: RgbaColor::rgb(255, 0, 0),
            button_normal: RgbaColor::rgb(30, 30, 30),
            button_hover: RgbaColor::rgb(40, 10, 10),
            button_pressed: RgbaColor::rgb(255, 0, 0),
        }
    }

    /// MSI Gaming (Red/Black)
    pub fn msi_gaming() -> ColorScheme {
        ColorScheme {
            primary: RgbaColor::rgb(227, 6, 19), // MSI Red
            secondary: RgbaColor::rgb(0, 0, 0),
            accent: RgbaColor::rgb(255, 50, 50),

            background: RgbaColor::rgb(15, 15, 15),
            surface: RgbaColor::rgb(25, 25, 25),
            elevated_surface: RgbaColor::rgb(35, 35, 35),

            text_primary: RgbaColor::rgb(255, 255, 255),
            text_secondary: RgbaColor::rgb(190, 190, 190),
            text_disabled: RgbaColor::rgb(100, 100, 100),

            success: RgbaColor::rgb(76, 175, 80),
            warning: RgbaColor::rgb(255, 193, 7),
            error: RgbaColor::rgb(227, 6, 19),
            info: RgbaColor::rgb(33, 150, 243),

            graph_temp: RgbaColor::rgb(227, 6, 19),
            graph_load: RgbaColor::rgb(255, 50, 50),
            graph_power: RgbaColor::rgb(255, 152, 0),
            graph_clock: RgbaColor::rgb(100, 181, 246),
            graph_memory: RgbaColor::rgb(186, 104, 200),

            slider_track: RgbaColor::rgb(55, 55, 55),
            slider_thumb: RgbaColor::rgb(227, 6, 19),
            button_normal: RgbaColor::rgb(35, 35, 35),
            button_hover: RgbaColor::rgb(45, 15, 15),
            button_pressed: RgbaColor::rgb(227, 6, 19),
        }
    }

    /// EVGA Precision (Orange/Black)
    pub fn evga_precision() -> ColorScheme {
        ColorScheme {
            primary: RgbaColor::rgb(255, 102, 0), // EVGA Orange
            secondary: RgbaColor::rgb(0, 0, 0),
            accent: RgbaColor::rgb(255, 140, 0),

            background: RgbaColor::rgb(12, 12, 12),
            surface: RgbaColor::rgb(22, 22, 22),
            elevated_surface: RgbaColor::rgb(32, 32, 32),

            text_primary: RgbaColor::rgb(255, 255, 255),
            text_secondary: RgbaColor::rgb(200, 200, 200),
            text_disabled: RgbaColor::rgb(100, 100, 100),

            success: RgbaColor::rgb(76, 175, 80),
            warning: RgbaColor::rgb(255, 193, 7),
            error: RgbaColor::rgb(244, 67, 54),
            info: RgbaColor::rgb(33, 150, 243),

            graph_temp: RgbaColor::rgb(255, 87, 34),
            graph_load: RgbaColor::rgb(255, 152, 0),
            graph_power: RgbaColor::rgb(255, 193, 7),
            graph_clock: RgbaColor::rgb(3, 169, 244),
            graph_memory: RgbaColor::rgb(171, 71, 188),

            slider_track: RgbaColor::rgb(60, 60, 60),
            slider_thumb: RgbaColor::rgb(255, 102, 0),
            button_normal: RgbaColor::rgb(32, 32, 32),
            button_hover: RgbaColor::rgb(42, 32, 22),
            button_pressed: RgbaColor::rgb(255, 102, 0),
        }
    }

    /// AMD Radeon (Red/Black) - for comparison/compatibility
    pub fn amd_radeon() -> ColorScheme {
        ColorScheme {
            primary: RgbaColor::rgb(237, 28, 36), // AMD Red
            secondary: RgbaColor::rgb(0, 0, 0),
            accent: RgbaColor::rgb(255, 60, 60),

            background: RgbaColor::rgb(16, 16, 16),
            surface: RgbaColor::rgb(26, 26, 26),
            elevated_surface: RgbaColor::rgb(36, 36, 36),

            text_primary: RgbaColor::rgb(255, 255, 255),
            text_secondary: RgbaColor::rgb(190, 190, 190),
            text_disabled: RgbaColor::rgb(100, 100, 100),

            success: RgbaColor::rgb(76, 175, 80),
            warning: RgbaColor::rgb(255, 193, 7),
            error: RgbaColor::rgb(237, 28, 36),
            info: RgbaColor::rgb(33, 150, 243),

            graph_temp: RgbaColor::rgb(237, 28, 36),
            graph_load: RgbaColor::rgb(255, 60, 60),
            graph_power: RgbaColor::rgb(255, 152, 0),
            graph_clock: RgbaColor::rgb(66, 165, 245),
            graph_memory: RgbaColor::rgb(186, 104, 200),

            slider_track: RgbaColor::rgb(56, 56, 56),
            slider_thumb: RgbaColor::rgb(237, 28, 36),
            button_normal: RgbaColor::rgb(36, 36, 36),
            button_hover: RgbaColor::rgb(46, 26, 26),
            button_pressed: RgbaColor::rgb(237, 28, 36),
        }
    }

    /// Light Mode (for daytime use)
    pub fn light_mode() -> ColorScheme {
        ColorScheme {
            primary: RgbaColor::rgb(25, 118, 210),
            secondary: RgbaColor::rgb(255, 255, 255),
            accent: RgbaColor::rgb(66, 165, 245),

            background: RgbaColor::rgb(250, 250, 250),
            surface: RgbaColor::rgb(255, 255, 255),
            elevated_surface: RgbaColor::rgb(245, 245, 245),

            text_primary: RgbaColor::rgb(33, 33, 33),
            text_secondary: RgbaColor::rgb(117, 117, 117),
            text_disabled: RgbaColor::rgb(189, 189, 189),

            success: RgbaColor::rgb(56, 142, 60),
            warning: RgbaColor::rgb(245, 124, 0),
            error: RgbaColor::rgb(211, 47, 47),
            info: RgbaColor::rgb(25, 118, 210),

            graph_temp: RgbaColor::rgb(211, 47, 47),
            graph_load: RgbaColor::rgb(67, 160, 71),
            graph_power: RgbaColor::rgb(245, 124, 0),
            graph_clock: RgbaColor::rgb(25, 118, 210),
            graph_memory: RgbaColor::rgb(142, 36, 170),

            slider_track: RgbaColor::rgb(224, 224, 224),
            slider_thumb: RgbaColor::rgb(25, 118, 210),
            button_normal: RgbaColor::rgb(245, 245, 245),
            button_hover: RgbaColor::rgb(238, 238, 238),
            button_pressed: RgbaColor::rgb(25, 118, 210),
        }
    }

    /// Cyberpunk (Neon Blue/Purple)
    pub fn cyberpunk() -> ColorScheme {
        ColorScheme {
            primary: RgbaColor::rgb(0, 255, 255),    // Cyan
            secondary: RgbaColor::rgb(138, 43, 226), // Purple
            accent: RgbaColor::rgb(255, 0, 255),     // Magenta

            background: RgbaColor::rgb(8, 8, 16),
            surface: RgbaColor::rgb(16, 16, 32),
            elevated_surface: RgbaColor::rgb(24, 24, 48),

            text_primary: RgbaColor::rgb(0, 255, 255),
            text_secondary: RgbaColor::rgb(138, 180, 248),
            text_disabled: RgbaColor::rgb(100, 100, 120),

            success: RgbaColor::rgb(0, 255, 128),
            warning: RgbaColor::rgb(255, 255, 0),
            error: RgbaColor::rgb(255, 0, 128),
            info: RgbaColor::rgb(0, 192, 255),

            graph_temp: RgbaColor::rgb(255, 0, 128),
            graph_load: RgbaColor::rgb(0, 255, 255),
            graph_power: RgbaColor::rgb(255, 255, 0),
            graph_clock: RgbaColor::rgb(138, 43, 226),
            graph_memory: RgbaColor::rgb(255, 0, 255),

            slider_track: RgbaColor::rgb(40, 40, 80),
            slider_thumb: RgbaColor::rgb(0, 255, 255),
            button_normal: RgbaColor::rgb(24, 24, 48),
            button_hover: RgbaColor::rgb(34, 34, 68),
            button_pressed: RgbaColor::rgb(0, 255, 255),
        }
    }
}

/// Theme manager
pub struct ThemeManager {
    current_theme: ColorScheme,
    theme_name: String,
}

impl ThemeManager {
    pub fn new(theme_name: &str) -> Self {
        let current_theme = match theme_name {
            "nvidia_dark" => ThemePresets::nvidia_dark(),
            "asus_rog" => ThemePresets::asus_rog(),
            "msi_gaming" => ThemePresets::msi_gaming(),
            "evga_precision" => ThemePresets::evga_precision(),
            "amd_radeon" => ThemePresets::amd_radeon(),
            "light" => ThemePresets::light_mode(),
            "cyberpunk" => ThemePresets::cyberpunk(),
            _ => ThemePresets::nvidia_dark(),
        };

        Self {
            current_theme,
            theme_name: theme_name.to_string(),
        }
    }

    pub fn get_theme(&self) -> &ColorScheme {
        &self.current_theme
    }

    pub fn set_theme(&mut self, theme_name: &str) {
        self.current_theme = match theme_name {
            "nvidia_dark" => ThemePresets::nvidia_dark(),
            "asus_rog" => ThemePresets::asus_rog(),
            "msi_gaming" => ThemePresets::msi_gaming(),
            "evga_precision" => ThemePresets::evga_precision(),
            "amd_radeon" => ThemePresets::amd_radeon(),
            "light" => ThemePresets::light_mode(),
            "cyberpunk" => ThemePresets::cyberpunk(),
            _ => ThemePresets::nvidia_dark(),
        };
        self.theme_name = theme_name.to_string();
    }

    pub fn theme_name(&self) -> &str {
        &self.theme_name
    }

    pub fn available_themes() -> Vec<&'static str> {
        vec![
            "nvidia_dark",
            "asus_rog",
            "msi_gaming",
            "evga_precision",
            "amd_radeon",
            "light",
            "cyberpunk",
        ]
    }

    /// Export theme to CSS for web-based GUI
    pub fn export_css(&self) -> String {
        let theme = &self.current_theme;
        format!(
            r#":root {{
    --color-primary: {};
    --color-secondary: {};
    --color-accent: {};

    --color-background: {};
    --color-surface: {};
    --color-elevated-surface: {};

    --color-text-primary: {};
    --color-text-secondary: {};
    --color-text-disabled: {};

    --color-success: {};
    --color-warning: {};
    --color-error: {};
    --color-info: {};

    --color-graph-temp: {};
    --color-graph-load: {};
    --color-graph-power: {};
    --color-graph-clock: {};
    --color-graph-memory: {};

    --color-slider-track: {};
    --color-slider-thumb: {};
    --color-button-normal: {};
    --color-button-hover: {};
    --color-button-pressed: {};
}}
"#,
            theme.primary.to_hex(),
            theme.secondary.to_hex(),
            theme.accent.to_hex(),
            theme.background.to_hex(),
            theme.surface.to_hex(),
            theme.elevated_surface.to_hex(),
            theme.text_primary.to_hex(),
            theme.text_secondary.to_hex(),
            theme.text_disabled.to_hex(),
            theme.success.to_hex(),
            theme.warning.to_hex(),
            theme.error.to_hex(),
            theme.info.to_hex(),
            theme.graph_temp.to_hex(),
            theme.graph_load.to_hex(),
            theme.graph_power.to_hex(),
            theme.graph_clock.to_hex(),
            theme.graph_memory.to_hex(),
            theme.slider_track.to_hex(),
            theme.slider_thumb.to_hex(),
            theme.button_normal.to_hex(),
            theme.button_hover.to_hex(),
            theme.button_pressed.to_hex(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_presets() {
        let nvidia = ThemePresets::nvidia_dark();
        assert_eq!(nvidia.primary.to_hex(), "#76B900");

        let asus = ThemePresets::asus_rog();
        assert_eq!(asus.primary.to_hex(), "#FF0000");

        let msi = ThemePresets::msi_gaming();
        assert_eq!(msi.primary.to_hex(), "#E30613");
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new("nvidia_dark");
        assert_eq!(manager.theme_name(), "nvidia_dark");

        manager.set_theme("asus_rog");
        assert_eq!(manager.theme_name(), "asus_rog");
    }

    #[test]
    fn test_css_export() {
        let manager = ThemeManager::new("nvidia_dark");
        let css = manager.export_css();
        assert!(css.contains("--color-primary: #76B900"));
    }

    #[test]
    fn test_available_themes() {
        let themes = ThemeManager::available_themes();
        assert!(themes.contains(&"nvidia_dark"));
        assert!(themes.contains(&"asus_rog"));
        assert!(themes.contains(&"cyberpunk"));
    }
}
