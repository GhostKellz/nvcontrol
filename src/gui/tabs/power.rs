use eframe::egui;

use crate::asus_power_detector::{PowerHealth, PowerTrend};
use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;

pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} Power", icons::POWER));
    ui.add_space(4.0);

    state.refresh_asus_power();

    Card::new(&colors)
        .title("GPU Power")
        .icon(icons::POWER)
        .show(ui, |ui| {
            if let Some(ref stats) = state.gpu_stats {
                egui::Grid::new("gpu_power_grid")
                    .num_columns(2)
                    .spacing([16.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("Current Draw").color(colors.fg_dark.to_egui()),
                        );
                        ui.label(
                            egui::RichText::new(format!("{:.1}W", stats.power_draw))
                                .color(colors.cyan.to_egui()),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("Power Limit").color(colors.fg_dark.to_egui()),
                        );
                        ui.label(
                            egui::RichText::new(format!("{:.1}W", stats.power_limit))
                                .color(colors.fg.to_egui()),
                        );
                        ui.end_row();

                        let avg = if state.power_history.is_empty() {
                            0.0
                        } else {
                            state.power_history.iter().sum::<f32>()
                                / state.power_history.len() as f32
                        };
                        ui.label(
                            egui::RichText::new("Rolling Avg").color(colors.fg_dark.to_egui()),
                        );
                        ui.label(
                            egui::RichText::new(format!("{:.1}W", avg))
                                .color(colors.cyan.to_egui()),
                        );
                        ui.end_row();
                    });
            } else {
                ui.label(
                    egui::RichText::new("Waiting for GPU power data...")
                        .color(colors.fg_dark.to_egui()),
                );
            }
        });

    ui.add_space(8.0);

    Card::new(&colors)
        .title("ASUS Power Monitor+")
        .icon(icons::POWER)
        .show(ui, |ui| {
            if state.asus_power_detector.is_none() {
                ui.label(
                    egui::RichText::new("No supported ASUS Power Detector+ GPU found")
                        .color(colors.fg_dark.to_egui()),
                );
                ui.label(
                    egui::RichText::new("This panel is intended for supported ROG Astral/Matrix cards with readable I2C monitoring")
                        .small()
                        .color(colors.comment.to_egui()),
                );
                return;
            }

            if let Some(ref status) = state.asus_power_status {
                let (status_text, status_color) = match status.health {
                    PowerHealth::Good => ("Healthy", colors.green.to_egui()),
                    PowerHealth::Warning => ("Warning", colors.yellow.to_egui()),
                    PowerHealth::Critical => ("Critical", colors.red.to_egui()),
                    PowerHealth::Unknown => ("Unknown", colors.fg_dark.to_egui()),
                };

                let trend = state.asus_power_history.trend();
                let trend_text = match trend {
                    PowerTrend::Rising => "Rising",
                    PowerTrend::Falling => "Falling",
                    PowerTrend::Stable => "Stable",
                    PowerTrend::Unknown => "Unknown",
                };

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{}", status.model)).strong().color(colors.fg.to_egui()));
                    ui.separator();
                    ui.label(egui::RichText::new(format!("Health: {}", status_text)).strong().color(status_color));
                    if let Some(total_power) = status.total_power_w {
                        ui.separator();
                        ui.label(egui::RichText::new(format!("{:.1}W connector", total_power)).strong().color(colors.cyan.to_egui()));
                    }
                });

                ui.add_space(6.0);

                egui::Grid::new("asus_power_overview_grid")
                    .num_columns(2)
                    .spacing([16.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("Trend").color(colors.fg_dark.to_egui()));
                        ui.label(egui::RichText::new(trend_text).color(colors.fg.to_egui()));
                        ui.end_row();

                        ui.label(egui::RichText::new("I2C Bus").color(colors.fg_dark.to_egui()));
                        ui.label(egui::RichText::new(format!("{}", status.i2c_bus)).color(colors.fg.to_egui()));
                        ui.end_row();

                        ui.label(egui::RichText::new("Source").color(colors.fg_dark.to_egui()));
                        ui.label(egui::RichText::new(&status.source).color(colors.fg.to_egui()));
                        ui.end_row();

                        if let Some(balance) = status.current_balance_percent {
                            ui.label(egui::RichText::new("Pin balance").color(colors.fg_dark.to_egui()));
                            ui.label(egui::RichText::new(format!("{balance:.1}% min/max")).color(colors.fg.to_egui()));
                            ui.end_row();
                        }

                        ui.label(egui::RichText::new("Samples").color(colors.fg_dark.to_egui()));
                        ui.label(egui::RichText::new(format!("{}", state.asus_power_history.len())).color(colors.fg.to_egui()));
                        ui.end_row();

                        ui.label(egui::RichText::new("Warnings").color(colors.fg_dark.to_egui()));
                        ui.label(egui::RichText::new(format!("{}", state.asus_power_history.warning_count())).color(colors.yellow.to_egui()));
                        ui.end_row();
                    });

                ui.add_space(8.0);
                ui.label(egui::RichText::new("12V-2x6 pins").strong().color(colors.fg.to_egui()));
                egui::Grid::new("asus_power_rail_grid")
                    .num_columns(4)
                    .spacing([12.0, 4.0])
                    .show(ui, |ui| {
                        for rail in &status.rails {
                            ui.label(egui::RichText::new(format!("Pin {}", rail.rail_id + 1)).small().color(colors.fg_dark.to_egui()));
                            ui.label(
                                egui::RichText::new(
                                    rail.voltage_mv
                                        .map(|voltage_mv| format!("{:.3}V", voltage_mv as f32 / 1000.0))
                                        .unwrap_or_else(|| "-".to_string()),
                                )
                                .small()
                                .color(colors.fg.to_egui()),
                            );
                            ui.label(
                                egui::RichText::new(
                                    rail.current_ma
                                        .map(|current_ma| format!("{:.2}A", current_ma as f32 / 1000.0))
                                        .unwrap_or_else(|| "-".to_string()),
                                )
                                .small()
                                .color(colors.cyan.to_egui()),
                            );
                            ui.label(
                                egui::RichText::new(if rail.warning { "!" } else { "✓" })
                                    .small()
                                    .color(if rail.warning {
                                        colors.yellow.to_egui()
                                    } else {
                                        colors.green.to_egui()
                                    }),
                            );
                            ui.end_row();
                        }
                    });

                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(
                        "Read-only IT8915 telemetry. Health rules are load-gated snapshots; nvcontrol never performs automatic shutdown or power-limit actions.",
                    )
                    .small()
                    .color(colors.comment.to_egui()),
                );
            } else {
                let message = state
                    .asus_power_error
                    .as_deref()
                    .unwrap_or("Initializing ASUS power monitoring...");
                ui.label(egui::RichText::new(message).color(colors.fg_dark.to_egui()));
                ui.label(
                    egui::RichText::new("Prefer astral-hwmon, or verify access to the GPU's i2c device for the direct SMBus fallback.")
                        .small()
                        .color(colors.comment.to_egui()),
                );
            }
        });
}
