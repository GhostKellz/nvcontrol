use serde::{Deserialize, Serialize};

/// Modern theming system with Tokyo Night and Dracula variants
/// Designed for TUI (Ratatui) and GUI (egui) consistency
/// Optimized for NVIDIA GPU monitoring dashboards

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ThemeVariant {
    TokyoNightNight,
    TokyoNightStorm,
    TokyoNightMoon,
    Dracula,
    RogRed,        // ASUS ROG red theme
    MatrixGreen,   // Classic green on black
    Cyberpunk,     // Pink/cyan neon
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub variant: ThemeVariant,
    pub colors: ColorPalette,
    pub nerd_font: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    // Backgrounds
    pub bg: Color,
    pub bg_dark: Color,
    pub bg_highlight: Color,
    pub bg_popup: Color,
    pub bg_statusline: Color,

    // Foregrounds
    pub fg: Color,
    pub fg_dark: Color,
    pub fg_gutter: Color,

    // Primary colors
    pub blue: Color,
    pub cyan: Color,
    pub green: Color,
    pub teal: Color,
    pub magenta: Color,
    pub purple: Color,
    pub red: Color,
    pub orange: Color,
    pub yellow: Color,

    // Semantic colors (for GPU monitoring)
    pub temp_cold: Color,    // < 50°C - Blue/Cyan
    pub temp_normal: Color,  // 50-70°C - Green
    pub temp_warm: Color,    // 70-80°C - Yellow/Orange
    pub temp_hot: Color,     // > 80°C - Red

    pub usage_low: Color,    // < 30% - Green
    pub usage_medium: Color, // 30-70% - Yellow
    pub usage_high: Color,   // > 70% - Red

    pub power_efficient: Color, // < 50% TDP - Green
    pub power_normal: Color,    // 50-85% TDP - Blue
    pub power_high: Color,      // > 85% TDP - Orange

    // UI elements
    pub border: Color,
    pub border_highlight: Color,
    pub selection: Color,
    pub comment: Color,

    // Graph colors
    pub graph_line1: Color,  // Primary metric
    pub graph_line2: Color,  // Secondary metric
    pub graph_line3: Color,  // Tertiary metric
    pub graph_fill: Color,   // Area fill
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Convert to ratatui::Color
    pub fn to_ratatui(&self) -> ratatui::style::Color {
        ratatui::style::Color::Rgb(self.r, self.g, self.b)
    }

    /// Convert to egui::Color32
    #[cfg(feature = "gui")]
    pub fn to_egui(&self) -> eframe::egui::Color32 {
        eframe::egui::Color32::from_rgb(self.r, self.g, self.b)
    }

    /// Lighten color by percentage
    pub fn lighten(&self, amount: f32) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        Self {
            r: (self.r as f32 + (255.0 - self.r as f32) * amount) as u8,
            g: (self.g as f32 + (255.0 - self.g as f32) * amount) as u8,
            b: (self.b as f32 + (255.0 - self.b as f32) * amount) as u8,
        }
    }

    /// Darken color by percentage
    pub fn darken(&self, amount: f32) -> Self {
        let amount = amount.clamp(0.0, 1.0);
        Self {
            r: (self.r as f32 * (1.0 - amount)) as u8,
            g: (self.g as f32 * (1.0 - amount)) as u8,
            b: (self.b as f32 * (1.0 - amount)) as u8,
        }
    }
}

impl Theme {
    /// Tokyo Night - Night variant (darkest)
    pub fn tokyonight_night() -> Self {
        Self {
            name: "Tokyo Night".to_string(),
            variant: ThemeVariant::TokyoNightNight,
            colors: ColorPalette {
                // Backgrounds
                bg: Color::hex(0x1a1b26),
                bg_dark: Color::hex(0x16161e),
                bg_highlight: Color::hex(0x292e42),
                bg_popup: Color::hex(0x1f2335),
                bg_statusline: Color::hex(0x1f2335),

                // Foregrounds
                fg: Color::hex(0xc0caf5),
                fg_dark: Color::hex(0xa9b1d6),
                fg_gutter: Color::hex(0x3b4261),

                // Primary colors
                blue: Color::hex(0x7aa2f7),
                cyan: Color::hex(0x7dcfff),
                green: Color::hex(0x9ece6a),
                teal: Color::hex(0x1abc9c),
                magenta: Color::hex(0xbb9af7),
                purple: Color::hex(0x9d7cd8),
                red: Color::hex(0xf7768e),
                orange: Color::hex(0xff9e64),
                yellow: Color::hex(0xe0af68),

                // Temperature colors
                temp_cold: Color::hex(0x7dcfff),    // Cyan
                temp_normal: Color::hex(0x9ece6a),  // Green
                temp_warm: Color::hex(0xe0af68),    // Yellow
                temp_hot: Color::hex(0xf7768e),     // Red

                // Usage colors
                usage_low: Color::hex(0x9ece6a),     // Green
                usage_medium: Color::hex(0xe0af68),  // Yellow
                usage_high: Color::hex(0xf7768e),    // Red

                // Power colors
                power_efficient: Color::hex(0x73daca), // Green teal
                power_normal: Color::hex(0x7aa2f7),    // Blue
                power_high: Color::hex(0xff9e64),      // Orange

                // UI
                border: Color::hex(0x3b4261),
                border_highlight: Color::hex(0x7aa2f7),
                selection: Color::hex(0x364a82),
                comment: Color::hex(0x565f89),

                // Graphs
                graph_line1: Color::hex(0x7aa2f7),  // Blue - GPU temp
                graph_line2: Color::hex(0x9ece6a),  // Green - FPS
                graph_line3: Color::hex(0xbb9af7),  // Magenta - Power
                graph_fill: Color::hex(0x292e42),   // Dark blue
            },
            nerd_font: true,
        }
    }

    /// Tokyo Night - Storm variant (balanced)
    pub fn tokyonight_storm() -> Self {
        Self {
            name: "Tokyo Night Storm".to_string(),
            variant: ThemeVariant::TokyoNightStorm,
            colors: ColorPalette {
                bg: Color::hex(0x24283b),
                bg_dark: Color::hex(0x1f2335),
                bg_highlight: Color::hex(0x292e42),
                bg_popup: Color::hex(0x1f2335),
                bg_statusline: Color::hex(0x1f2335),

                fg: Color::hex(0xc0caf5),
                fg_dark: Color::hex(0xa9b1d6),
                fg_gutter: Color::hex(0x3b4261),

                blue: Color::hex(0x7aa2f7),
                cyan: Color::hex(0x7dcfff),
                green: Color::hex(0x9ece6a),
                teal: Color::hex(0x1abc9c),
                magenta: Color::hex(0xbb9af7),
                purple: Color::hex(0x9d7cd8),
                red: Color::hex(0xf7768e),
                orange: Color::hex(0xff9e64),
                yellow: Color::hex(0xe0af68),

                temp_cold: Color::hex(0x7dcfff),
                temp_normal: Color::hex(0x9ece6a),
                temp_warm: Color::hex(0xe0af68),
                temp_hot: Color::hex(0xf7768e),

                usage_low: Color::hex(0x9ece6a),
                usage_medium: Color::hex(0xe0af68),
                usage_high: Color::hex(0xf7768e),

                power_efficient: Color::hex(0x73daca),
                power_normal: Color::hex(0x7aa2f7),
                power_high: Color::hex(0xff9e64),

                border: Color::hex(0x3b4261),
                border_highlight: Color::hex(0x7aa2f7),
                selection: Color::hex(0x364a82),
                comment: Color::hex(0x565f89),

                graph_line1: Color::hex(0x7aa2f7),
                graph_line2: Color::hex(0x9ece6a),
                graph_line3: Color::hex(0xbb9af7),
                graph_fill: Color::hex(0x292e42),
            },
            nerd_font: true,
        }
    }

    /// Tokyo Night - Moon variant (blue-tinted)
    pub fn tokyonight_moon() -> Self {
        Self {
            name: "Tokyo Night Moon".to_string(),
            variant: ThemeVariant::TokyoNightMoon,
            colors: ColorPalette {
                bg: Color::hex(0x222436),
                bg_dark: Color::hex(0x1e2030),
                bg_highlight: Color::hex(0x2f334d),
                bg_popup: Color::hex(0x1e2030),
                bg_statusline: Color::hex(0x1e2030),

                fg: Color::hex(0xc8d3f5),
                fg_dark: Color::hex(0xb4bedc),
                fg_gutter: Color::hex(0x3b4261),

                blue: Color::hex(0x82aaff),
                cyan: Color::hex(0x86e1fc),
                green: Color::hex(0xc3e88d),
                teal: Color::hex(0x4fd6be),
                magenta: Color::hex(0xc099ff),
                purple: Color::hex(0xfca7ea),
                red: Color::hex(0xff757f),
                orange: Color::hex(0xff966c),
                yellow: Color::hex(0xffc777),

                temp_cold: Color::hex(0x86e1fc),
                temp_normal: Color::hex(0xc3e88d),
                temp_warm: Color::hex(0xffc777),
                temp_hot: Color::hex(0xff757f),

                usage_low: Color::hex(0xc3e88d),
                usage_medium: Color::hex(0xffc777),
                usage_high: Color::hex(0xff757f),

                power_efficient: Color::hex(0x4fd6be),
                power_normal: Color::hex(0x82aaff),
                power_high: Color::hex(0xff966c),

                border: Color::hex(0x3b4261),
                border_highlight: Color::hex(0x82aaff),
                selection: Color::hex(0x3e4963),
                comment: Color::hex(0x636da6),

                graph_line1: Color::hex(0x82aaff),
                graph_line2: Color::hex(0xc3e88d),
                graph_line3: Color::hex(0xc099ff),
                graph_fill: Color::hex(0x2f334d),
            },
            nerd_font: true,
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            variant: ThemeVariant::Dracula,
            colors: ColorPalette {
                bg: Color::hex(0x282a36),
                bg_dark: Color::hex(0x21222c),
                bg_highlight: Color::hex(0x44475a),
                bg_popup: Color::hex(0x282a36),
                bg_statusline: Color::hex(0x44475a),

                fg: Color::hex(0xf8f8f2),
                fg_dark: Color::hex(0xe6e6e6),
                fg_gutter: Color::hex(0x44475a),

                blue: Color::hex(0x6272a4),
                cyan: Color::hex(0x8be9fd),
                green: Color::hex(0x50fa7b),
                teal: Color::hex(0x1abc9c),
                magenta: Color::hex(0xff79c6),
                purple: Color::hex(0xbd93f9),
                red: Color::hex(0xff5555),
                orange: Color::hex(0xffb86c),
                yellow: Color::hex(0xf1fa8c),

                temp_cold: Color::hex(0x8be9fd),
                temp_normal: Color::hex(0x50fa7b),
                temp_warm: Color::hex(0xf1fa8c),
                temp_hot: Color::hex(0xff5555),

                usage_low: Color::hex(0x50fa7b),
                usage_medium: Color::hex(0xf1fa8c),
                usage_high: Color::hex(0xff5555),

                power_efficient: Color::hex(0x50fa7b),
                power_normal: Color::hex(0x6272a4),
                power_high: Color::hex(0xffb86c),

                border: Color::hex(0x44475a),
                border_highlight: Color::hex(0xbd93f9),
                selection: Color::hex(0x44475a),
                comment: Color::hex(0x6272a4),

                graph_line1: Color::hex(0xbd93f9),
                graph_line2: Color::hex(0x50fa7b),
                graph_line3: Color::hex(0xff79c6),
                graph_fill: Color::hex(0x44475a),
            },
            nerd_font: true,
        }
    }

    /// ASUS ROG Red theme
    pub fn rog_red() -> Self {
        Self {
            name: "ROG Red".to_string(),
            variant: ThemeVariant::RogRed,
            colors: ColorPalette {
                bg: Color::hex(0x0d0d0d),
                bg_dark: Color::hex(0x000000),
                bg_highlight: Color::hex(0x1a0000),
                bg_popup: Color::hex(0x0d0d0d),
                bg_statusline: Color::hex(0x1a0000),

                fg: Color::hex(0xff0032),
                fg_dark: Color::hex(0xcc0028),
                fg_gutter: Color::hex(0x330008),

                blue: Color::hex(0xff006e),
                cyan: Color::hex(0xff0050),
                green: Color::hex(0x00ff88),
                teal: Color::hex(0x00cc70),
                magenta: Color::hex(0xff0064),
                purple: Color::hex(0xff005a),
                red: Color::hex(0xff0032),
                orange: Color::hex(0xff3200),
                yellow: Color::hex(0xffaa00),

                temp_cold: Color::hex(0x00ccff),
                temp_normal: Color::hex(0x00ff88),
                temp_warm: Color::hex(0xffaa00),
                temp_hot: Color::hex(0xff0032),

                usage_low: Color::hex(0x00ff88),
                usage_medium: Color::hex(0xffaa00),
                usage_high: Color::hex(0xff0032),

                power_efficient: Color::hex(0x00cc70),
                power_normal: Color::hex(0xff006e),
                power_high: Color::hex(0xff3200),

                border: Color::hex(0x330008),
                border_highlight: Color::hex(0xff0032),
                selection: Color::hex(0x330008),
                comment: Color::hex(0x660010),

                graph_line1: Color::hex(0xff0032),
                graph_line2: Color::hex(0x00ff88),
                graph_line3: Color::hex(0xff006e),
                graph_fill: Color::hex(0x1a0000),
            },
            nerd_font: true,
        }
    }

    /// Matrix Green theme
    pub fn matrix_green() -> Self {
        Self {
            name: "Matrix Green".to_string(),
            variant: ThemeVariant::MatrixGreen,
            colors: ColorPalette {
                bg: Color::hex(0x000000),
                bg_dark: Color::hex(0x000000),
                bg_highlight: Color::hex(0x001a00),
                bg_popup: Color::hex(0x000000),
                bg_statusline: Color::hex(0x001a00),

                fg: Color::hex(0x00ff00),
                fg_dark: Color::hex(0x00cc00),
                fg_gutter: Color::hex(0x003300),

                blue: Color::hex(0x00ff88),
                cyan: Color::hex(0x00ffcc),
                green: Color::hex(0x00ff00),
                teal: Color::hex(0x00cc88),
                magenta: Color::hex(0x00ff66),
                purple: Color::hex(0x00ff44),
                red: Color::hex(0x88ff00),
                orange: Color::hex(0xccff00),
                yellow: Color::hex(0xffff00),

                temp_cold: Color::hex(0x00ffcc),
                temp_normal: Color::hex(0x00ff00),
                temp_warm: Color::hex(0xccff00),
                temp_hot: Color::hex(0xffff00),

                usage_low: Color::hex(0x00ff00),
                usage_medium: Color::hex(0xccff00),
                usage_high: Color::hex(0xffff00),

                power_efficient: Color::hex(0x00cc88),
                power_normal: Color::hex(0x00ff88),
                power_high: Color::hex(0x88ff00),

                border: Color::hex(0x003300),
                border_highlight: Color::hex(0x00ff00),
                selection: Color::hex(0x003300),
                comment: Color::hex(0x006600),

                graph_line1: Color::hex(0x00ff00),
                graph_line2: Color::hex(0x00ffcc),
                graph_line3: Color::hex(0xccff00),
                graph_fill: Color::hex(0x001a00),
            },
            nerd_font: true,
        }
    }

    /// Cyberpunk theme (pink/cyan neon)
    pub fn cyberpunk() -> Self {
        Self {
            name: "Cyberpunk".to_string(),
            variant: ThemeVariant::Cyberpunk,
            colors: ColorPalette {
                bg: Color::hex(0x0a0e27),
                bg_dark: Color::hex(0x050813),
                bg_highlight: Color::hex(0x1a1f3a),
                bg_popup: Color::hex(0x0a0e27),
                bg_statusline: Color::hex(0x1a1f3a),

                fg: Color::hex(0xffffff),
                fg_dark: Color::hex(0xe0e0e0),
                fg_gutter: Color::hex(0x2a2f4a),

                blue: Color::hex(0x00d9ff),
                cyan: Color::hex(0x00ffff),
                green: Color::hex(0x00ff9f),
                teal: Color::hex(0x00ffcc),
                magenta: Color::hex(0xff00ff),
                purple: Color::hex(0xcc00ff),
                red: Color::hex(0xff006e),
                orange: Color::hex(0xff9900),
                yellow: Color::hex(0xffff00),

                temp_cold: Color::hex(0x00ffff),
                temp_normal: Color::hex(0x00ff9f),
                temp_warm: Color::hex(0xff9900),
                temp_hot: Color::hex(0xff006e),

                usage_low: Color::hex(0x00ff9f),
                usage_medium: Color::hex(0xff9900),
                usage_high: Color::hex(0xff006e),

                power_efficient: Color::hex(0x00ffcc),
                power_normal: Color::hex(0x00d9ff),
                power_high: Color::hex(0xff9900),

                border: Color::hex(0x2a2f4a),
                border_highlight: Color::hex(0xff00ff),
                selection: Color::hex(0x2a2f4a),
                comment: Color::hex(0x4a4f6a),

                graph_line1: Color::hex(0xff00ff),
                graph_line2: Color::hex(0x00ffff),
                graph_line3: Color::hex(0x00ff9f),
                graph_fill: Color::hex(0x1a1f3a),
            },
            nerd_font: true,
        }
    }

    /// Get theme by variant
    pub fn from_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::TokyoNightNight => Self::tokyonight_night(),
            ThemeVariant::TokyoNightStorm => Self::tokyonight_storm(),
            ThemeVariant::TokyoNightMoon => Self::tokyonight_moon(),
            ThemeVariant::Dracula => Self::dracula(),
            ThemeVariant::RogRed => Self::rog_red(),
            ThemeVariant::MatrixGreen => Self::matrix_green(),
            ThemeVariant::Cyberpunk => Self::cyberpunk(),
        }
    }

    /// Get all available themes
    pub fn all_themes() -> Vec<Self> {
        vec![
            Self::tokyonight_night(),
            Self::tokyonight_storm(),
            Self::tokyonight_moon(),
            Self::dracula(),
            Self::rog_red(),
            Self::matrix_green(),
            Self::cyberpunk(),
        ]
    }
}

impl ThemeVariant {
    /// Get the display name of the theme variant
    pub fn name(&self) -> &str {
        match self {
            ThemeVariant::TokyoNightNight => "Tokyo Night",
            ThemeVariant::TokyoNightStorm => "Tokyo Night Storm",
            ThemeVariant::TokyoNightMoon => "Tokyo Night Moon",
            ThemeVariant::Dracula => "Dracula",
            ThemeVariant::RogRed => "ROG Red",
            ThemeVariant::MatrixGreen => "Matrix Green",
            ThemeVariant::Cyberpunk => "Cyberpunk",
        }
    }
}

impl ColorPalette {
    /// Create a color palette from a theme variant
    pub fn from_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::TokyoNightNight => Theme::tokyonight_night().colors,
            ThemeVariant::TokyoNightStorm => Theme::tokyonight_storm().colors,
            ThemeVariant::TokyoNightMoon => Theme::tokyonight_moon().colors,
            ThemeVariant::Dracula => Theme::dracula().colors,
            ThemeVariant::RogRed => Theme::rog_red().colors,
            ThemeVariant::MatrixGreen => Theme::matrix_green().colors,
            ThemeVariant::Cyberpunk => Theme::cyberpunk().colors,
        }
    }

    // Helper methods for commonly accessed colors
    pub fn primary(&self) -> &Color {
        &self.blue
    }

    pub fn text(&self) -> &Color {
        &self.fg
    }

    pub fn text_dim(&self) -> &Color {
        &self.fg_dark
    }

    pub fn accent(&self) -> &Color {
        &self.cyan
    }

    pub fn success(&self) -> &Color {
        &self.green
    }

    pub fn warning(&self) -> &Color {
        &self.yellow
    }

    pub fn error(&self) -> &Color {
        &self.red
    }
}

/// Nerd Font icons for GPU dashboard
pub mod icons {
    pub const GPU: &str = "󰢮";
    pub const TEMP: &str = "";
    pub const FAN: &str = "󰈐";
    pub const POWER: &str = "⚡";
    pub const MEMORY: &str = "";
    pub const CLOCK: &str = "";
    pub const USAGE: &str = "󰓅";
    pub const CHART: &str = "󰄛";
    pub const WARNING: &str = "";
    pub const ERROR: &str = "";
    pub const SUCCESS: &str = "";
    pub const INFO: &str = "";
    pub const SETTINGS: &str = "";
    pub const PROFILE: &str = "";
    pub const OC: &str = "󰥏";
    pub const RGB: &str = "󰉦";
    pub const MONITOR: &str = "󰍹";
    pub const VIBRANCE: &str = "󰌁";
    pub const HDR: &str = "󰃟";
    pub const FULLSCREEN: &str = "󰊓";
    pub const RECORD: &str = "";
    pub const PAUSE: &str = "";
    pub const STOP: &str = "";
    pub const THEME: &str = "󰏘";
    pub const VRR: &str = "󰑙";
    pub const GAMING: &str = "󰊗";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokyo_night_theme() {
        let theme = Theme::tokyonight_night();
        assert_eq!(theme.variant, ThemeVariant::TokyoNightNight);
        assert!(theme.nerd_font);
    }

    #[test]
    fn test_all_themes() {
        let themes = Theme::all_themes();
        assert_eq!(themes.len(), 7);
    }

    #[test]
    fn test_color_lighten() {
        let color = Color::rgb(100, 100, 100);
        let lighter = color.lighten(0.5);
        assert!(lighter.r > color.r);
    }

    #[test]
    fn test_color_darken() {
        let color = Color::rgb(200, 200, 200);
        let darker = color.darken(0.5);
        assert!(darker.r < color.r);
    }
}
