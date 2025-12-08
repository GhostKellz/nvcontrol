//! Card Widget
//!
//! A card-style container with consistent styling.

use crate::themes::ColorPalette;
use eframe::egui;

/// Card widget for grouping related content
pub struct Card<'a> {
    title: Option<&'a str>,
    icon: Option<&'a str>,
    colors: &'a ColorPalette,
    collapsible: bool,
}

impl<'a> Card<'a> {
    /// Create a new card
    pub fn new(colors: &'a ColorPalette) -> Self {
        Self {
            title: None,
            icon: None,
            colors,
            collapsible: false,
        }
    }

    /// Set the card title
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the card icon
    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Make the card collapsible
    #[allow(dead_code)]
    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }

    /// Show the card with content
    pub fn show<R>(self, ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
        let frame = egui::Frame::none()
            .fill(self.colors.bg_highlight.to_egui())
            .stroke(egui::Stroke::new(1.0, self.colors.border.to_egui()))
            .rounding(8.0)
            .inner_margin(12.0)
            .outer_margin(4.0);

        frame
            .show(ui, |ui| {
                // Header if title is set
                if let Some(title) = self.title {
                    ui.horizontal(|ui| {
                        if let Some(icon) = self.icon {
                            ui.label(
                                egui::RichText::new(icon)
                                    .size(16.0)
                                    .color(self.colors.cyan.to_egui()),
                            );
                        }
                        ui.label(
                            egui::RichText::new(title)
                                .strong()
                                .color(self.colors.fg.to_egui()),
                        );
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);
                }

                add_contents(ui)
            })
            .inner
    }
}
