//! Fan Control Tab
//!
//! Fan speed control with presets, custom curves, and thermal monitoring.

use eframe::egui;

use crate::fan;
use crate::gui::icons;
use crate::gui::state::{FanMode, GuiState};
use crate::gui::widgets::Card;

/// Render the Fan Control tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} Fan Control", icons::FAN_ICON));
    ui.add_space(4.0);

    // Two-column layout
    ui.columns(2, |columns| {
        // Left column: Current fan status
        render_fan_status(&mut columns[0], state, &colors);

        // Right column: Thermal status
        render_thermal_status(&mut columns[1], state, &colors);
    });

    ui.add_space(10.0);

    // Fan curve editor
    render_fan_curve_editor(ui, state, &colors);

    // Request repaint for live updates
    ctx.request_repaint();
}

/// Render current fan status card
fn render_fan_status(
    ui: &mut egui::Ui,
    state: &mut GuiState,
    colors: &crate::themes::ColorPalette,
) {
    Card::new(colors)
        .title("Current Fan Status")
        .icon(icons::FAN_ICON)
        .show(ui, |ui| {
            let fans = fan::list_fans();
            if fans.is_empty() {
                ui.label(
                    egui::RichText::new("No controllable fans detected")
                        .weak()
                        .italics(),
                );
            }

            for fan_info in fans {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("Fan {}:", fan_info.id)).strong());
                    if let Some(rpm) = fan_info.rpm {
                        ui.label(
                            egui::RichText::new(format!("{} RPM", rpm))
                                .color(colors.green.to_egui()),
                        );
                    }
                    if let Some(percent) = fan_info.percent {
                        let fan_color = if percent > 80 {
                            colors.red.to_egui()
                        } else if percent > 50 {
                            colors.yellow.to_egui()
                        } else {
                            colors.green.to_egui()
                        };
                        ui.label(
                            egui::RichText::new(format!("{}%", percent))
                                .strong()
                                .color(fan_color),
                        );
                    }
                });

                if fan_info.controllable {
                    let current_speed = state.fan_speeds.get(&fan_info.id).copied().unwrap_or(50);
                    let mut new_speed = current_speed;
                    ui.horizontal(|ui| {
                        ui.label("Speed:");
                        if ui
                            .add(egui::Slider::new(&mut new_speed, 0..=100).suffix("%"))
                            .changed()
                        {
                            state.fan_speeds.insert(fan_info.id, new_speed);
                            if let Err(e) = fan::set_fan_speed(fan_info.id, new_speed) {
                                state
                                    .toasts
                                    .error(format!("Failed to set fan speed: {}", e));
                            }
                        }
                    });
                } else {
                    ui.label(egui::RichText::new("Read-only (automatic)").small().weak());
                }
                ui.add_space(4.0);
            }

            // Fan mode selector
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Mode:");
                if ui
                    .selectable_label(state.fan_mode == FanMode::Auto, "Auto")
                    .clicked()
                {
                    state.set_fan_mode(FanMode::Auto);
                }
                if ui
                    .selectable_label(state.fan_mode == FanMode::Manual, "Manual")
                    .clicked()
                {
                    state.set_fan_mode(FanMode::Manual);
                }
                if ui
                    .selectable_label(state.fan_mode == FanMode::Curve, "Curve")
                    .clicked()
                {
                    state.set_fan_mode(FanMode::Curve);
                }
            });

            // Quick preset buttons
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Quick Presets")
                    .small()
                    .strong()
                    .color(colors.yellow.to_egui()),
            );
            ui.horizontal(|ui| {
                if ui
                    .button("🔇 Silent")
                    .on_hover_text("30% - Quiet operation")
                    .clicked()
                {
                    for (_, speed) in state.fan_speeds.iter_mut() {
                        *speed = 30;
                    }
                    if let Err(e) = fan::set_fan_speed(0, 30) {
                        state.toasts.error(format!("Failed: {}", e));
                    } else {
                        state.toasts.info("Fan set to silent mode (30%)");
                    }
                }
                if ui
                    .button("⚖️ Balanced")
                    .on_hover_text("50% - Default cooling")
                    .clicked()
                {
                    for (_, speed) in state.fan_speeds.iter_mut() {
                        *speed = 50;
                    }
                    if let Err(e) = fan::set_fan_speed(0, 50) {
                        state.toasts.error(format!("Failed: {}", e));
                    } else {
                        state.toasts.info("Fan set to balanced mode (50%)");
                    }
                }
                if ui
                    .button("❄️ Cool")
                    .on_hover_text("70% - Better cooling")
                    .clicked()
                {
                    for (_, speed) in state.fan_speeds.iter_mut() {
                        *speed = 70;
                    }
                    if let Err(e) = fan::set_fan_speed(0, 70) {
                        state.toasts.error(format!("Failed: {}", e));
                    } else {
                        state.toasts.info("Fan set to cool mode (70%)");
                    }
                }
                if ui
                    .button("🔥 Max")
                    .on_hover_text("100% - Maximum cooling")
                    .clicked()
                {
                    for (_, speed) in state.fan_speeds.iter_mut() {
                        *speed = 100;
                    }
                    if let Err(e) = fan::set_fan_speed(0, 100) {
                        state.toasts.error(format!("Failed: {}", e));
                    } else {
                        state.toasts.info("Fan set to maximum (100%)");
                    }
                }
            });
        });
}

/// Render thermal status card
fn render_thermal_status(
    ui: &mut egui::Ui,
    state: &GuiState,
    colors: &crate::themes::ColorPalette,
) {
    Card::new(colors)
        .title("Thermal Status")
        .icon(icons::TEMP)
        .show(ui, |ui| {
            if let Some(ref stats) = state.gpu_stats {
                let temp_color = if stats.temperature > 80.0 {
                    colors.red.to_egui()
                } else if stats.temperature > 65.0 {
                    colors.yellow.to_egui()
                } else {
                    colors.green.to_egui()
                };

                ui.horizontal(|ui| {
                    ui.label("GPU Temperature:");
                    ui.label(
                        egui::RichText::new(format!("{:.0}°C", stats.temperature))
                            .size(24.0)
                            .strong()
                            .color(temp_color),
                    );
                });

                ui.add_space(4.0);

                let target_speed = state.fan_curve.get_speed_at_temp(stats.temperature as f64);
                ui.horizontal(|ui| {
                    ui.label("Target Fan Speed:");
                    ui.label(
                        egui::RichText::new(format!("{:.0}%", target_speed))
                            .size(18.0)
                            .color(colors.cyan.to_egui()),
                    );
                });

                ui.add_space(8.0);

                // Thermal status indicator
                let (icon, color, desc) = if stats.temperature > 85.0 {
                    ("🔥 CRITICAL", colors.red.to_egui(), "GPU is overheating!")
                } else if stats.temperature > 80.0 {
                    (
                        "⚠️ HOT",
                        colors.orange.to_egui(),
                        "Consider increasing fan speed",
                    )
                } else if stats.temperature > 70.0 {
                    (
                        "🌡️ WARM",
                        colors.yellow.to_egui(),
                        "Normal gaming temperature",
                    )
                } else if stats.temperature > 50.0 {
                    ("✅ GOOD", colors.green.to_egui(), "Healthy operating range")
                } else {
                    ("❄️ COOL", colors.cyan.to_egui(), "Low load temperature")
                };

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(icon).strong().color(color));
                });
                ui.label(egui::RichText::new(desc).small().weak());

                ui.add_space(8.0);

                // Current fan speed from stats
                ui.horizontal(|ui| {
                    ui.label("Current Fan Speed:");
                    ui.label(
                        egui::RichText::new(format!("{}%", stats.fan_speed))
                            .strong()
                            .color(colors.blue.to_egui()),
                    );
                });
            } else {
                ui.label(
                    egui::RichText::new("Waiting for GPU stats...")
                        .weak()
                        .italics(),
                );
            }
        });
}

/// Render fan curve editor
fn render_fan_curve_editor(
    ui: &mut egui::Ui,
    state: &mut GuiState,
    colors: &crate::themes::ColorPalette,
) {
    Card::new(colors)
        .title("Fan Curve Editor")
        .icon(icons::CHART)
        .show(ui, |ui| {
            // Fan curve plot
            use egui_plot::{Line, Plot, PlotPoints, Points};

            let curve_points_vec: Vec<[f64; 2]> = state
                .fan_curve
                .points
                .iter()
                .map(|p| [p.x, p.y])
                .collect();

            Plot::new("fan_curve_plot")
                .height(220.0)
                .width(ui.available_width())
                .x_axis_label("Temperature (°C)")
                .y_axis_label("Fan Speed (%)")
                .allow_drag(true)
                .allow_zoom(true)
                .show_axes([true, true])
                .show(ui, |plot_ui| {
                    // Draw the curve line
                    let curve_line: PlotPoints = curve_points_vec.clone().into();
                    plot_ui.line(Line::new(curve_line).color(colors.cyan.to_egui()));

                    // Draw the control points
                    let curve_pts: PlotPoints = curve_points_vec.into();
                    plot_ui.points(
                        Points::new(curve_pts)
                            .radius(6.0)
                            .color(colors.blue.to_egui())
                            .name("Control Points"),
                    );

                    // Draw current temperature indicator
                    if let Some(ref stats) = state.gpu_stats {
                        let temp = stats.temperature as f64;
                        let speed = state.fan_curve.get_speed_at_temp(temp);
                        let current_point: PlotPoints = vec![[temp, speed]].into();
                        plot_ui.points(
                            Points::new(current_point)
                                .radius(8.0)
                                .color(colors.red.to_egui())
                                .name("Current"),
                        );
                    }
                });

            ui.add_space(8.0);

            // Two columns for point editor and actions
            ui.columns(2, |columns| {
                // Control points column
                columns[0].label(
                    egui::RichText::new("Control Points")
                        .small()
                        .strong()
                        .color(colors.green.to_egui()),
                );
                columns[0].separator();

                let mut point_to_remove = None;

                for (i, point) in state.fan_curve.points.iter().enumerate() {
                    columns[0].horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("{}.", i + 1)).monospace());
                        ui.label(
                            egui::RichText::new(format!("{:.0}°C", point.x))
                                .color(colors.orange.to_egui()),
                        );
                        ui.label("→");
                        ui.label(
                            egui::RichText::new(format!("{:.0}%", point.y))
                                .color(colors.cyan.to_egui()),
                        );

                        if ui
                            .small_button("🗑️")
                            .on_hover_text("Remove point")
                            .clicked()
                            && state.fan_curve.points.len() > 2
                        {
                            point_to_remove = Some(i);
                        }
                    });
                }

                if let Some(i) = point_to_remove {
                    state.fan_curve.remove_point(i);
                }

                // Actions column
                columns[1].label(
                    egui::RichText::new("Actions")
                        .small()
                        .strong()
                        .color(colors.yellow.to_egui()),
                );
                columns[1].separator();

                columns[1].horizontal(|ui| {
                    if ui
                        .button("➕ Add Point")
                        .on_hover_text("Add control point at 60°C")
                        .clicked()
                    {
                        let mid_temp = 60.0;
                        let mid_speed = state.fan_curve.get_speed_at_temp(mid_temp);
                        state.fan_curve.add_point(mid_temp, mid_speed);
                    }

                    if ui
                        .button("🔄 Reset")
                        .on_hover_text("Reset to default curve")
                        .clicked()
                    {
                        state.fan_curve = crate::gui_widgets::FanCurve::default();
                        state.toasts.info("Fan curve reset to default");
                    }

                    if ui
                        .button("💾 Apply")
                        .on_hover_text("Apply fan curve to GPU")
                        .clicked()
                    {
                        state.set_fan_mode(FanMode::Curve);
                    }
                });

                // Preset curves
                columns[1].add_space(8.0);
                columns[1].label(
                    egui::RichText::new("Preset Curves")
                        .small()
                        .color(colors.purple.to_egui()),
                );
                columns[1].horizontal(|ui| {
                    if ui
                        .small_button("Silent")
                        .on_hover_text("Low noise profile")
                        .clicked()
                    {
                        state.fan_curve.points.clear();
                        state.fan_curve.add_point(30.0, 20.0);
                        state.fan_curve.add_point(50.0, 30.0);
                        state.fan_curve.add_point(70.0, 50.0);
                        state.fan_curve.add_point(85.0, 80.0);
                        state.toasts.info("Silent curve loaded");
                    }
                    if ui
                        .small_button("Balanced")
                        .on_hover_text("Default cooling")
                        .clicked()
                    {
                        state.fan_curve.points.clear();
                        state.fan_curve.add_point(30.0, 30.0);
                        state.fan_curve.add_point(50.0, 40.0);
                        state.fan_curve.add_point(65.0, 60.0);
                        state.fan_curve.add_point(80.0, 100.0);
                        state.toasts.info("Balanced curve loaded");
                    }
                    if ui
                        .small_button("Aggressive")
                        .on_hover_text("Maximum cooling")
                        .clicked()
                    {
                        state.fan_curve.points.clear();
                        state.fan_curve.add_point(30.0, 40.0);
                        state.fan_curve.add_point(45.0, 60.0);
                        state.fan_curve.add_point(60.0, 80.0);
                        state.fan_curve.add_point(75.0, 100.0);
                        state.toasts.info("Aggressive curve loaded");
                    }
                });
            });

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(
                    "Tip: Drag points in the graph to adjust the curve, or use preset curves above.",
                )
                .small()
                .weak(),
            );
        });
}
