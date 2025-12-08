//! GUI Theme Integration
//!
//! Bridges the themes module with egui visuals.

use crate::themes::{ColorPalette, ThemeVariant};
use eframe::egui;

/// Apply a theme variant to egui context
pub fn apply_theme(ctx: &egui::Context, variant: ThemeVariant) {
    let theme_data = crate::themes::Theme::from_variant(variant);
    let colors = &theme_data.colors;

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = colors.bg.to_egui();
    visuals.window_fill = colors.bg_highlight.to_egui();
    visuals.extreme_bg_color = colors.bg_dark.to_egui();
    visuals.widgets.noninteractive.bg_fill = colors.bg_highlight.to_egui();
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, colors.fg.to_egui());

    // Button accent colors from theme palette (each theme defines its own)
    // Buttons use weak_bg_fill, not bg_fill (bg_fill is for sliders/checkboxes)
    visuals.widgets.inactive.weak_bg_fill = colors.button_accent_active.to_egui();
    visuals.widgets.inactive.bg_fill = colors.button_accent_active.to_egui();
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

    visuals.widgets.hovered.weak_bg_fill = colors.button_accent.to_egui();
    visuals.widgets.hovered.bg_fill = colors.button_accent.to_egui();
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);

    visuals.widgets.active.weak_bg_fill = colors.button_accent_hover.to_egui();
    visuals.widgets.active.bg_fill = colors.button_accent_hover.to_egui();
    visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, colors.bg_dark.to_egui());

    // Open state (for dropdown menus)
    visuals.widgets.open.weak_bg_fill = colors.button_accent.to_egui();
    visuals.widgets.open.bg_fill = colors.button_accent.to_egui();
    visuals.widgets.open.fg_stroke = egui::Stroke::new(1.5, egui::Color32::WHITE);

    visuals.selection.bg_fill = colors.button_accent.to_egui();
    visuals.hyperlink_color = colors.blue.to_egui();

    ctx.set_visuals(visuals);
}

/// Get the icon emoji for a theme variant
pub fn theme_icon(variant: ThemeVariant) -> &'static str {
    match variant {
        ThemeVariant::TokyoNightNight => "🌃",
        ThemeVariant::TokyoNightStorm => "⛈️",
        ThemeVariant::TokyoNightMoon => "🌙",
        ThemeVariant::Dracula => "🧛",
        ThemeVariant::RogRed => "🔴",
        ThemeVariant::MatrixGreen => "💚",
        ThemeVariant::Cyberpunk => "🌆",
    }
}

/// Cycle to the next theme variant
pub fn next_theme(current: ThemeVariant) -> ThemeVariant {
    match current {
        ThemeVariant::TokyoNightNight => ThemeVariant::TokyoNightStorm,
        ThemeVariant::TokyoNightStorm => ThemeVariant::TokyoNightMoon,
        ThemeVariant::TokyoNightMoon => ThemeVariant::Dracula,
        ThemeVariant::Dracula => ThemeVariant::RogRed,
        ThemeVariant::RogRed => ThemeVariant::MatrixGreen,
        ThemeVariant::MatrixGreen => ThemeVariant::Cyberpunk,
        ThemeVariant::Cyberpunk => ThemeVariant::TokyoNightNight,
    }
}

/// Consistent spacing constants for the grid system
pub mod spacing {
    /// Base unit (8px)
    pub const UNIT: f32 = 8.0;
    /// Small spacing (4px)
    pub const XS: f32 = 4.0;
    /// Medium spacing (8px)
    pub const SM: f32 = 8.0;
    /// Standard spacing (16px)
    pub const MD: f32 = 16.0;
    /// Large spacing (24px)
    pub const LG: f32 = 24.0;
    /// Extra large spacing (32px)
    pub const XL: f32 = 32.0;
    /// Card padding
    pub const CARD_PADDING: f32 = 12.0;
    /// Card rounding
    pub const CARD_ROUNDING: f32 = 8.0;
    /// Section gap
    pub const SECTION_GAP: f32 = 16.0;
}

/// Draw a card-style container with consistent styling
pub fn card_frame(colors: &ColorPalette) -> egui::Frame {
    egui::Frame::none()
        .fill(colors.bg_highlight.to_egui())
        .stroke(egui::Stroke::new(1.0, colors.border.to_egui()))
        .rounding(spacing::CARD_ROUNDING)
        .inner_margin(spacing::CARD_PADDING)
        .outer_margin(spacing::XS)
}

/// Draw a section header with icon and title
pub fn section_header(ui: &mut egui::Ui, icon: &str, title: &str, colors: &ColorPalette) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(icon)
                .size(18.0)
                .color(colors.cyan.to_egui()),
        );
        ui.label(
            egui::RichText::new(title)
                .strong()
                .size(16.0)
                .color(colors.fg.to_egui()),
        );
    });
    ui.add_space(spacing::SM);
    ui.separator();
    ui.add_space(spacing::SM);
}
