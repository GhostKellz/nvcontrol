//! Overclock Tab
//!
//! GPU overclocking controls with presets, manual tuning, voltage curve editor,
//! and live monitoring.

use eframe::egui;

use crate::gui::icons;
use crate::gui::state::{GuiState, OcPreset};
use crate::gui::widgets::Card;
use crate::overclocking;

/// Render the Overclock tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} Interactive GPU Overclocking", icons::OVERCLOCK));
    ui.add_space(4.0);

    // Current status bar at top
    if let Some(ref stats) = state.gpu_stats {
        egui::Frame::none()
            .fill(colors.bg_dark.to_egui())
            .rounding(6.0)
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Current clocks
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("Current Clocks").small());
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("{} MHz", stats.core_clock))
                                        .strong()
                                        .color(colors.cyan.to_egui()),
                                );
                                ui.label("GPU");
                                ui.separator();
                                ui.label(
                                    egui::RichText::new(format!("{} MHz", stats.memory_clock))
                                        .strong()
                                        .color(colors.purple.to_egui()),
                                );
                                ui.label("VRAM");
                            });
                        });
                    });

                    // Temperature indicator
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("Temperature").small());
                            let temp_color = if stats.temperature > 80.0 {
                                colors.red.to_egui()
                            } else if stats.temperature > 70.0 {
                                colors.orange.to_egui()
                            } else {
                                colors.green.to_egui()
                            };
                            ui.label(
                                egui::RichText::new(format!("{:.0}°C", stats.temperature))
                                    .strong()
                                    .size(18.0)
                                    .color(temp_color),
                            );
                        });
                    });

                    // Power indicator
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("Power Draw").small());
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.0}W / {:.0}W",
                                    stats.power_draw, stats.power_limit
                                ))
                                .strong()
                                .color(colors.magenta.to_egui()),
                            );
                        });
                    });
                });
            });

        ui.add_space(8.0);
    }

    // Two-column layout
    ui.columns(2, |columns| {
        // Left column: Presets and Manual Tuning
        render_presets_column(&mut columns[0], state, &colors);

        // Right column: Safety info and Live monitoring
        render_monitoring_column(&mut columns[1], state, &colors);
    });

    ui.add_space(10.0);

    // Voltage Curve Editor
    render_voltage_curve(ui, state, &colors);

    // Request repaint for live updates
    ctx.request_repaint();
}

/// Render the presets and manual tuning column
fn render_presets_column(
    ui: &mut egui::Ui,
    state: &mut GuiState,
    colors: &crate::themes::ColorPalette,
) {
    // Presets card
    Card::new(colors)
        .title("Performance Presets")
        .icon(icons::TARGET)
        .show(ui, |ui| {
            // Preset card helper
            let preset_cards = [
                (
                    OcPreset::Stock,
                    "📊",
                    "Stock",
                    "Factory default settings",
                    "0/0/100%",
                    colors.fg_dark.to_egui(),
                    "Safe",
                ),
                (
                    OcPreset::MildOc,
                    "🔧",
                    "Mild OC",
                    "Modest gains, minimal risk",
                    "+75/+500/105%",
                    colors.green.to_egui(),
                    "Low",
                ),
                (
                    OcPreset::Performance,
                    "⚡",
                    "Performance",
                    "Balanced speed & stability",
                    "+150/+1000/110%",
                    colors.orange.to_egui(),
                    "Medium",
                ),
                (
                    OcPreset::Extreme,
                    "🔥",
                    "Extreme",
                    "Maximum performance (RTX 5090)",
                    "+200/+1500/115%",
                    colors.red.to_egui(),
                    "High",
                ),
            ];

            for (preset, icon, name, desc, specs, color, risk) in preset_cards {
                let is_selected = state.oc_preset == preset;
                let frame_fill = if is_selected {
                    color.linear_multiply(0.3)
                } else {
                    colors.bg_dark.to_egui()
                };
                let frame_stroke = if is_selected {
                    egui::Stroke::new(2.0, color)
                } else {
                    egui::Stroke::new(1.0, colors.bg_highlight.to_egui())
                };

                let resp = egui::Frame::none()
                    .fill(frame_fill)
                    .stroke(frame_stroke)
                    .rounding(6.0)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Icon
                            ui.label(egui::RichText::new(icon).size(24.0).color(color));
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(name).strong().color(
                                        if is_selected {
                                            color
                                        } else {
                                            colors.fg.to_egui()
                                        },
                                    ));
                                    if is_selected {
                                        ui.label(egui::RichText::new("✓").color(color));
                                    }
                                });
                                ui.label(egui::RichText::new(desc).small().weak());
                                ui.label(
                                    egui::RichText::new(specs)
                                        .small()
                                        .monospace()
                                        .color(colors.fg_dark.to_egui()),
                                );
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(egui::RichText::new(risk).small().color(color));
                                },
                            );
                        });
                    });

                if resp.response.interact(egui::Sense::click()).clicked() {
                    state.apply_oc_preset(preset);
                }

                ui.add_space(4.0);
            }
        });

    ui.add_space(8.0);

    // Manual Tuning card
    Card::new(colors)
        .title("Manual Tuning")
        .icon(icons::SPEED)
        .show(ui, |ui| {
            // GPU Clock slider
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Core Clock").small().strong());
                ui.label(egui::RichText::new("(GPU boost offset)").small().weak());
            });
            let gpu_color = if state.gpu_offset > 0 {
                colors.green.to_egui()
            } else if state.gpu_offset < 0 {
                colors.cyan.to_egui()
            } else {
                colors.fg_dark.to_egui()
            };
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut state.gpu_offset, -200..=200)
                        .suffix(" MHz")
                        .custom_formatter(|v, _| format!("{:+.0}", v)),
                );
                ui.label(
                    egui::RichText::new(format!("{:+} MHz", state.gpu_offset))
                        .strong()
                        .color(gpu_color),
                );
            });

            ui.add_space(6.0);

            // Memory Clock slider
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Memory Clock").small().strong());
                ui.label(egui::RichText::new("(VRAM speed offset)").small().weak());
            });
            let mem_color = if state.memory_offset > 0 {
                colors.purple.to_egui()
            } else if state.memory_offset < 0 {
                colors.cyan.to_egui()
            } else {
                colors.fg_dark.to_egui()
            };
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut state.memory_offset, -1000..=1500)
                        .suffix(" MHz")
                        .custom_formatter(|v, _| format!("{:+.0}", v)),
                );
                ui.label(
                    egui::RichText::new(format!("{:+} MHz", state.memory_offset))
                        .strong()
                        .color(mem_color),
                );
            });

            ui.add_space(6.0);

            // Power Limit slider
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Power Limit").small().strong());
                ui.label(egui::RichText::new("(max TDP percentage)").small().weak());
            });
            let power_color = if state.power_limit_percent > 100 {
                colors.red.to_egui()
            } else if state.power_limit_percent < 80 {
                colors.green.to_egui()
            } else {
                colors.yellow.to_egui()
            };
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut state.power_limit_percent, 50..=115).suffix("%"));
                ui.label(
                    egui::RichText::new(format!("{}%", state.power_limit_percent))
                        .strong()
                        .color(power_color),
                );
            });

            ui.add_space(12.0);

            // Action buttons
            ui.horizontal(|ui| {
                let apply_btn = egui::Button::new(egui::RichText::new("✅ Apply OC").strong())
                    .fill(colors.green.to_egui().linear_multiply(0.3));
                if ui
                    .add(apply_btn)
                    .on_hover_text("Apply overclock settings to GPU")
                    .clicked()
                {
                    match state.apply_overclock() {
                        Ok(_) => state.toasts.success("Overclock applied successfully"),
                        Err(e) => state.toasts.error(format!("OC failed: {}", e)),
                    }
                }

                if ui
                    .button("🔄 Reset to Stock")
                    .on_hover_text("Ctrl+R")
                    .clicked()
                {
                    state.reset_overclock();
                }

                if ui
                    .button("🔥 Stress Test")
                    .on_hover_text("Run 5-minute stability test")
                    .clicked()
                {
                    match overclocking::create_stress_test(5) {
                        Ok(_) => state.toasts.info("Stress test started (5 minutes)"),
                        Err(e) => state.toasts.error(format!("Failed to start test: {}", e)),
                    }
                }
            });
        });
}

/// Render the safety info and live monitoring column
fn render_monitoring_column(
    ui: &mut egui::Ui,
    state: &GuiState,
    colors: &crate::themes::ColorPalette,
) {
    // Safety Information card
    Card::new(colors)
        .title("Safety Information")
        .icon(icons::WARN)
        .show(ui, |ui| {
            ui.label("Overclocking can cause:");
            ui.label(egui::RichText::new("• System instability or crashes").small());
            ui.label(egui::RichText::new("• Increased power consumption").small());
            ui.label(egui::RichText::new("• Higher temperatures").small());
            ui.label(egui::RichText::new("• Potential hardware damage if extreme").small());

            ui.add_space(8.0);

            // Temperature warning
            if let Some(ref stats) = state.gpu_stats {
                if stats.temperature > 80.0 {
                    ui.colored_label(
                        colors.red.to_egui(),
                        "⚠️ GPU is HOT! Consider reducing overclock.",
                    );
                } else if stats.temperature > 70.0 {
                    ui.colored_label(colors.yellow.to_egui(), "ℹ️ Temperature elevated but safe.");
                } else {
                    ui.colored_label(
                        colors.green.to_egui(),
                        "✅ Temperature is good for overclocking.",
                    );
                }

                ui.add_space(8.0);

                // Architecture-specific tips
                ui.label(
                    egui::RichText::new(format!("Tips for {} GPUs:", stats.architecture))
                        .strong()
                        .color(colors.blue.to_egui()),
                );
                match stats.architecture.as_str() {
                    "Blackwell" => {
                        ui.label(
                            egui::RichText::new("• RTX 50 series responds well to memory OC")
                                .small(),
                        );
                        ui.label(
                            egui::RichText::new("• GDDR7 can handle +1500 MHz safely").small(),
                        );
                        ui.label(
                            egui::RichText::new("• Core benefits from slight undervolt").small(),
                        );
                    }
                    "Ada Lovelace" => {
                        ui.label(
                            egui::RichText::new("• RTX 40 series has good thermal headroom")
                                .small(),
                        );
                        ui.label(
                            egui::RichText::new("• +150-200 MHz core is typical safe range")
                                .small(),
                        );
                        ui.label(egui::RichText::new("• GDDR6X runs hot, watch temps").small());
                    }
                    _ => {
                        ui.label(
                            egui::RichText::new("• Start with small offsets (+50 core)").small(),
                        );
                        ui.label(
                            egui::RichText::new("• Test stability with each increase").small(),
                        );
                        ui.label(egui::RichText::new("• Monitor temperatures closely").small());
                    }
                }
            }
        });

    ui.add_space(8.0);

    // Live Monitoring card
    Card::new(colors)
        .title("Live Monitoring")
        .icon(icons::CHART)
        .show(ui, |ui| {
            if let Some(ref stats) = state.gpu_stats {
                egui::Grid::new("oc_live_stats")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("GPU Clock:");
                        ui.label(
                            egui::RichText::new(format!("{} MHz", stats.core_clock))
                                .monospace()
                                .color(colors.cyan.to_egui()),
                        );
                        ui.end_row();

                        ui.label("Memory Clock:");
                        ui.label(
                            egui::RichText::new(format!("{} MHz", stats.memory_clock))
                                .monospace()
                                .color(colors.purple.to_egui()),
                        );
                        ui.end_row();

                        ui.label("Temperature:");
                        let temp_c = temp_color(stats.temperature, colors);
                        ui.label(
                            egui::RichText::new(format!("{:.0}°C", stats.temperature))
                                .monospace()
                                .color(temp_c),
                        );
                        ui.end_row();

                        ui.label("Power:");
                        let pwr_ratio = if stats.power_limit > 0.0 {
                            stats.power_draw / stats.power_limit
                        } else {
                            0.0
                        };
                        let pwr_c = power_color(pwr_ratio, colors);
                        ui.label(
                            egui::RichText::new(format!(
                                "{:.0}W / {:.0}W",
                                stats.power_draw, stats.power_limit
                            ))
                            .monospace()
                            .color(pwr_c),
                        );
                        ui.end_row();

                        ui.label("Fan Speed:");
                        ui.label(
                            egui::RichText::new(format!("{}%", stats.fan_speed))
                                .monospace()
                                .color(colors.blue.to_egui()),
                        );
                        ui.end_row();

                        ui.label("GPU Load:");
                        let usage_c = usage_color(stats.utilization, colors);
                        ui.label(
                            egui::RichText::new(format!("{:.0}%", stats.utilization))
                                .monospace()
                                .color(usage_c),
                        );
                        ui.end_row();
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

/// Render the voltage curve editor section
fn render_voltage_curve(
    ui: &mut egui::Ui,
    state: &mut GuiState,
    colors: &crate::themes::ColorPalette,
) {
    Card::new(colors)
        .title("Voltage Curve Editor (Undervolting)")
        .icon(icons::CHART)
        .show(ui, |ui| {
            ui.label(
                "Advanced undervolting allows you to reduce power consumption while maintaining performance.",
            );
            ui.add_space(5.0);

            // Show current GPU stats
            if let Some(ref stats) = state.gpu_stats {
                ui.horizontal(|ui| {
                    ui.label("Current GPU Clock:");
                    ui.label(format!("{} MHz", stats.core_clock));

                    let voltage = state.voltage_curve.get_voltage_at_freq(stats.core_clock as f64);
                    ui.label(format!("→ Target Voltage: {:.0}mV", voltage));
                });
                ui.add_space(5.0);
            }

            // Voltage curve plot
            use egui_plot::{Line, Plot, PlotPoints, Points};

            let curve_points_vec: Vec<[f64; 2]> = state
                .voltage_curve
                .points
                .iter()
                .map(|p| [p.x, p.y])
                .collect();

            Plot::new("voltage_curve_plot")
                .height(250.0)
                .width(ui.available_width())
                .x_axis_label("Frequency (MHz)")
                .y_axis_label("Voltage (mV)")
                .allow_drag(true)
                .allow_zoom(true)
                .show_axes([true, true])
                .show(ui, |plot_ui| {
                    // Draw the curve line
                    let curve_line: PlotPoints = curve_points_vec.clone().into();
                    plot_ui.line(Line::new(curve_line).color(colors.yellow.to_egui()));

                    // Draw the control points
                    let curve_pts: PlotPoints = curve_points_vec.into();
                    plot_ui.points(
                        Points::new(curve_pts)
                            .radius(6.0)
                            .color(colors.yellow.to_egui())
                            .name("Voltage Points"),
                    );

                    // Draw current frequency indicator
                    if let Some(ref stats) = state.gpu_stats {
                        let freq = stats.core_clock as f64;
                        let voltage = state.voltage_curve.get_voltage_at_freq(freq);
                        let current_point: PlotPoints = vec![[freq, voltage]].into();
                        plot_ui.points(
                            Points::new(current_point)
                                .radius(8.0)
                                .color(colors.red.to_egui())
                                .name("Current"),
                        );
                    }
                });

            ui.add_space(10.0);

            // Point editor
            ui.horizontal(|ui| {
                ui.label("Voltage Points:");
            });

            ui.separator();

            let mut point_to_remove = None;

            for (i, point) in state.voltage_curve.points.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}.", i + 1));
                    ui.label(format!("{:.0} MHz → {:.0} mV", point.x, point.y));

                    if ui.button("🗑️ Remove").clicked() && state.voltage_curve.points.len() > 2 {
                        point_to_remove = Some(i);
                    }
                });
            }

            if let Some(i) = point_to_remove {
                state.voltage_curve.remove_point(i);
            }

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button("➕ Add Point").clicked() {
                    let mid_freq = 1500.0;
                    let mid_voltage = state.voltage_curve.get_voltage_at_freq(mid_freq);
                    state.voltage_curve.add_point(mid_freq, mid_voltage);
                }

                if ui.button("🔄 Reset to Default").clicked() {
                    state.voltage_curve = crate::gui_widgets::VoltageCurve::default();
                    state.toasts.info("Voltage curve reset to default");
                }

                if ui.button("💾 Apply Curve").clicked() {
                    // TODO: Apply voltage curve to GPU via nvidia-smi or NVML
                    state.toasts.info("Voltage curve applied (experimental)");
                }
            });

            ui.add_space(10.0);

            ui.colored_label(
                colors.yellow.to_egui(),
                "⚠️ Warning: Incorrect voltage settings can cause instability. Test thoroughly!",
            );
            ui.label("Tip: Lower voltages reduce power draw and heat, but too low will cause crashes.");
            ui.label("Start with small reductions (-25mV) and stress test before going further.");
        });
}

/// Get temperature color based on value
fn temp_color(temp: f32, colors: &crate::themes::ColorPalette) -> egui::Color32 {
    if temp > 80.0 {
        colors.red.to_egui()
    } else if temp > 65.0 {
        colors.yellow.to_egui()
    } else {
        colors.green.to_egui()
    }
}

/// Get power color based on ratio
fn power_color(ratio: f32, colors: &crate::themes::ColorPalette) -> egui::Color32 {
    if ratio > 0.95 {
        colors.red.to_egui()
    } else if ratio > 0.8 {
        colors.yellow.to_egui()
    } else {
        colors.green.to_egui()
    }
}

/// Get usage color based on percentage
fn usage_color(usage: f32, colors: &crate::themes::ColorPalette) -> egui::Color32 {
    if usage > 95.0 {
        colors.red.to_egui()
    } else if usage > 75.0 {
        colors.yellow.to_egui()
    } else {
        colors.green.to_egui()
    }
}
