//! Status Badge Widget
//!
//! Small status indicators for various states.

use crate::themes::ColorPalette;
use eframe::egui;

/// Status badge type
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BadgeType {
    Success,
    Warning,
    Error,
    Info,
    Neutral,
}

/// A small status badge
pub struct StatusBadge<'a> {
    text: &'a str,
    badge_type: BadgeType,
    colors: &'a ColorPalette,
    pulsing: bool,
}

impl<'a> StatusBadge<'a> {
    /// Create a new status badge
    pub fn new(text: &'a str, badge_type: BadgeType, colors: &'a ColorPalette) -> Self {
        Self {
            text,
            badge_type,
            colors,
            pulsing: false,
        }
    }

    /// Make the badge pulse (for active states)
    #[allow(dead_code)]
    pub fn pulsing(mut self) -> Self {
        self.pulsing = true;
        self
    }

    /// Show the badge
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let (bg_color, fg_color) = match self.badge_type {
            BadgeType::Success => (
                self.colors.green.to_egui().gamma_multiply(0.2),
                self.colors.green.to_egui(),
            ),
            BadgeType::Warning => (
                self.colors.yellow.to_egui().gamma_multiply(0.2),
                self.colors.yellow.to_egui(),
            ),
            BadgeType::Error => (
                self.colors.red.to_egui().gamma_multiply(0.2),
                self.colors.red.to_egui(),
            ),
            BadgeType::Info => (
                self.colors.blue.to_egui().gamma_multiply(0.2),
                self.colors.blue.to_egui(),
            ),
            BadgeType::Neutral => (
                self.colors.fg_dark.to_egui().gamma_multiply(0.2),
                self.colors.fg_dark.to_egui(),
            ),
        };

        let frame = egui::Frame::none()
            .fill(bg_color)
            .rounding(4.0)
            .inner_margin(egui::Margin::symmetric(6.0, 2.0));

        frame
            .show(ui, |ui| {
                ui.label(egui::RichText::new(self.text).size(11.0).color(fg_color))
            })
            .response
    }
}

/// Convenience functions for creating badges
impl<'a> StatusBadge<'a> {
    pub fn success(text: &'a str, colors: &'a ColorPalette) -> Self {
        Self::new(text, BadgeType::Success, colors)
    }

    pub fn warning(text: &'a str, colors: &'a ColorPalette) -> Self {
        Self::new(text, BadgeType::Warning, colors)
    }

    pub fn error(text: &'a str, colors: &'a ColorPalette) -> Self {
        Self::new(text, BadgeType::Error, colors)
    }

    pub fn info(text: &'a str, colors: &'a ColorPalette) -> Self {
        Self::new(text, BadgeType::Info, colors)
    }
}
