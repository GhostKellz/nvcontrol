//! Latency Optimization Tab
//!
//! Latency modes, Reflex settings, and gaming performance optimization.

use eframe::egui;

use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;
use crate::latency::{self, LatencyMode};

/// Render the Latency tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!(
        "{} Latency Optimization & Gaming Performance",
        icons::LATENCY
    ));
    ui.add_space(4.0);

    // Current Latency Status
    Card::new(&colors)
        .title("Current Latency Status")
        .icon(icons::TARGET)
        .show(ui, |ui| match latency::get_latency_info() {
            Ok(info) => {
                egui::Grid::new("latency_info_grid")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("NVIDIA Reflex:");
                        if info.nvidia_reflex_available {
                            let (text, color) = if info.nvidia_reflex_enabled {
                                ("✅ Enabled", colors.green.to_egui())
                            } else {
                                ("⚠️ Available", colors.yellow.to_egui())
                            };
                            ui.colored_label(color, text);
                        } else {
                            ui.colored_label(colors.red.to_egui(), "❌ Not Available");
                        }
                        ui.end_row();

                        ui.label("GPU Scheduling:");
                        let (text, color) = if info.gpu_scheduling_enabled {
                            ("✅ Enabled", colors.green.to_egui())
                        } else {
                            ("❌ Disabled", colors.yellow.to_egui())
                        };
                        ui.colored_label(color, text);
                        ui.end_row();

                        ui.label("CPU Scheduler:");
                        ui.label(&info.current_cpu_scheduler);
                        ui.end_row();

                        ui.label("Estimated Input Lag:");
                        let lag_color = if info.estimated_input_lag_ms < 10.0 {
                            colors.green.to_egui()
                        } else if info.estimated_input_lag_ms < 20.0 {
                            colors.yellow.to_egui()
                        } else {
                            colors.red.to_egui()
                        };
                        ui.colored_label(
                            lag_color,
                            format!("{:.1}ms", info.estimated_input_lag_ms),
                        );
                        ui.end_row();
                    });
            }
            Err(e) => {
                ui.colored_label(
                    colors.red.to_egui(),
                    format!("Error getting latency info: {}", e),
                );
            }
        });

    ui.add_space(8.0);

    // Latency Optimization Modes
    Card::new(&colors)
        .title("Latency Optimization Modes")
        .icon(icons::ROCKET)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("🏆 Competitive Mode").clicked() {
                    match latency::set_latency_mode(LatencyMode::Competitive) {
                        Ok(_) => state.toasts.success("Competitive latency mode enabled"),
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }
                ui.label("Ultra-low latency, maximum performance");
            });

            ui.horizontal(|ui| {
                if ui.button("⚖️ Balanced Mode").clicked() {
                    match latency::set_latency_mode(LatencyMode::Balanced) {
                        Ok(_) => state.toasts.success("Balanced latency mode enabled"),
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }
                ui.label("Good latency with system stability");
            });

            ui.horizontal(|ui| {
                if ui.button("🔋 Power Saver").clicked() {
                    match latency::set_latency_mode(LatencyMode::PowerSaver) {
                        Ok(_) => state.toasts.success("Power saver mode enabled"),
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }
                ui.label("Higher latency but lower power usage");
            });

            ui.horizontal(|ui| {
                if ui.button("🔄 Reset to Default").clicked() {
                    match latency::set_latency_mode(LatencyMode::Default) {
                        Ok(_) => state.toasts.success("Latency settings reset"),
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }
                ui.label("Restore system defaults");
            });
        });

    ui.add_space(8.0);

    // Gaming Optimizations
    Card::new(&colors)
        .title("Gaming-Specific Optimizations")
        .icon(icons::GAME)
        .show(ui, |ui| {
            if ui.button("🎯 Apply Full Latency Optimization").clicked() {
                match latency::optimize_latency() {
                    Ok(_) => state.toasts.success("Latency optimizations applied"),
                    Err(e) => state.toasts.error(format!("Failed: {}", e)),
                }
            }

            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Optimization Tips:")
                    .strong()
                    .color(colors.cyan.to_egui()),
            );
            ui.label(egui::RichText::new("• Use exclusive fullscreen mode in games").small());
            ui.label(egui::RichText::new("• Enable VRR/G-Sync for consistent frame times").small());
            ui.label(egui::RichText::new("• Close unnecessary background applications").small());
            ui.label(egui::RichText::new("• Use a high-frequency gaming mouse (1000Hz+)").small());
            ui.label(
                egui::RichText::new("• Consider overclocking GPU/CPU for higher framerates")
                    .small(),
            );
        });
}
