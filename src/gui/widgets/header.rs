//! Header Bar Widget
//!
//! Top application header with GPU info and status indicators.

use crate::themes::ColorPalette;
use eframe::egui;

use crate::gui::icons;

/// Status indicator state
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatusState {
    Ok,
    Warning,
    Error,
    Unknown,
}

impl StatusState {
    fn color(&self, colors: &ColorPalette) -> egui::Color32 {
        match self {
            StatusState::Ok => colors.green.to_egui(),
            StatusState::Warning => colors.yellow.to_egui(),
            StatusState::Error => colors.red.to_egui(),
            StatusState::Unknown => colors.fg_dark.to_egui(),
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            StatusState::Ok => icons::OK,
            StatusState::Warning => icons::WARN,
            StatusState::Error => icons::ERR,
            StatusState::Unknown => icons::INFO,
        }
    }
}

/// A status pill/badge for the header
pub struct StatusPill {
    pub label: String,
    pub state: StatusState,
}

/// Header bar widget
pub struct HeaderBar<'a> {
    gpu_name: Option<&'a str>,
    gpu_temp: Option<f32>,
    gpu_util: Option<f32>,
    colors: &'a ColorPalette,
    status_pills: Vec<StatusPill>,
}

impl<'a> HeaderBar<'a> {
    /// Create a new header bar
    pub fn new(colors: &'a ColorPalette) -> Self {
        Self {
            gpu_name: None,
            gpu_temp: None,
            gpu_util: None,
            colors,
            status_pills: Vec::new(),
        }
    }

    /// Set GPU name
    pub fn gpu_name(mut self, name: &'a str) -> Self {
        self.gpu_name = Some(name);
        self
    }

    /// Set GPU temperature
    pub fn gpu_temp(mut self, temp: f32) -> Self {
        self.gpu_temp = Some(temp);
        self
    }

    /// Set GPU utilization
    pub fn gpu_util(mut self, util: f32) -> Self {
        self.gpu_util = Some(util);
        self
    }

    /// Add a status pill
    pub fn add_status(mut self, label: impl Into<String>, state: StatusState) -> Self {
        self.status_pills.push(StatusPill {
            label: label.into(),
            state,
        });
        self
    }

    /// Show the header bar
    pub fn show(self, ui: &mut egui::Ui) {
        let frame = egui::Frame::none()
            .fill(self.colors.bg_dark.to_egui())
            .inner_margin(egui::Margin::symmetric(16.0, 8.0));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Left side: Logo and GPU name
                ui.label(
                    egui::RichText::new(icons::GPU)
                        .size(24.0)
                        .color(self.colors.green.to_egui()),
                );
                ui.add_space(8.0);

                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("nvcontrol")
                            .strong()
                            .size(14.0)
                            .color(self.colors.fg.to_egui()),
                    );
                    if let Some(name) = self.gpu_name {
                        ui.label(
                            egui::RichText::new(name)
                                .size(11.0)
                                .color(self.colors.fg_dark.to_egui()),
                        );
                    }
                });

                ui.add_space(24.0);

                // Center: Quick stats
                if let Some(temp) = self.gpu_temp {
                    let temp_color = if temp > 80.0 {
                        self.colors.red.to_egui()
                    } else if temp > 65.0 {
                        self.colors.yellow.to_egui()
                    } else {
                        self.colors.green.to_egui()
                    };

                    ui.label(
                        egui::RichText::new(icons::TEMP)
                            .size(14.0)
                            .color(temp_color),
                    );
                    ui.label(
                        egui::RichText::new(format!("{:.0}°C", temp))
                            .size(12.0)
                            .color(temp_color),
                    );
                    ui.add_space(16.0);
                }

                if let Some(util) = self.gpu_util {
                    let util_color = if util > 90.0 {
                        self.colors.yellow.to_egui()
                    } else {
                        self.colors.cyan.to_egui()
                    };

                    ui.label(
                        egui::RichText::new(icons::SPEED)
                            .size(14.0)
                            .color(util_color),
                    );
                    ui.label(
                        egui::RichText::new(format!("{:.0}%", util))
                            .size(12.0)
                            .color(util_color),
                    );
                }

                // Right side: Status pills
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Version
                    ui.label(
                        egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .size(10.0)
                            .color(self.colors.fg_dark.to_egui()),
                    );

                    ui.add_space(16.0);

                    // Status pills
                    for pill in &self.status_pills {
                        let color = pill.state.color(self.colors);
                        let icon = pill.state.icon();

                        egui::Frame::none()
                            .fill(color.gamma_multiply(0.2))
                            .rounding(4.0)
                            .inner_margin(egui::Margin::symmetric(6.0, 2.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(icon).size(10.0).color(color));
                                    ui.label(
                                        egui::RichText::new(&pill.label).size(10.0).color(color),
                                    );
                                });
                            });
                        ui.add_space(4.0);
                    }
                });
            });
        });
    }
}
