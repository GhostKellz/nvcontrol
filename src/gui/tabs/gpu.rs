//! GPU Status Tab
//!
//! Main GPU monitoring and information display.

use eframe::egui;
use std::collections::VecDeque;

use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;

/// Draw a mini sparkline graph
fn draw_sparkline(
    ui: &mut egui::Ui,
    data: &VecDeque<f32>,
    color: egui::Color32,
    height: f32,
    max_val: f32,
) {
    let width = ui.available_width().min(120.0);
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());

    if data.len() < 2 {
        return;
    }

    let painter = ui.painter_at(rect);

    // Draw background
    painter.rect_filled(rect, 2.0, egui::Color32::from_gray(30));

    // Calculate points
    let points: Vec<egui::Pos2> = data
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            let x = rect.left() + (i as f32 / (data.len() - 1) as f32) * rect.width();
            let y = rect.bottom() - (val / max_val).clamp(0.0, 1.0) * rect.height();
            egui::pos2(x, y)
        })
        .collect();

    // Draw line
    if points.len() >= 2 {
        painter.add(egui::Shape::line(points, egui::Stroke::new(1.5, color)));
    }
}

/// Render the GPU status tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} GPU Status & Monitoring", icons::GPU));
    ui.add_space(8.0);

    // GPU selector for multi-GPU systems
    if state.available_gpus.len() > 1 {
        Card::new(&colors)
            .title("GPU Selection")
            .icon(icons::TARGET)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_id_salt("gpu_selector")
                        .selected_text(format!(
                            "GPU {} - {}",
                            state.selected_gpu_index,
                            state
                                .available_gpus
                                .get(state.selected_gpu_index as usize)
                                .map(|g| g.name.as_str())
                                .unwrap_or("Unknown")
                        ))
                        .show_ui(ui, |ui| {
                            for gpu in &state.available_gpus {
                                ui.selectable_value(
                                    &mut state.selected_gpu_index,
                                    gpu.index,
                                    format!(
                                        "GPU {} - {} ({:.1}°C, {:.0}%)",
                                        gpu.index, gpu.name, gpu.temperature, gpu.utilization
                                    ),
                                );
                            }
                        });

                    if ui.button(format!("{} Refresh", icons::REFRESH)).clicked() {
                        state.available_gpus = crate::multi_gpu::detect_gpus().unwrap_or_default();
                        state.toasts.info("GPU list refreshed");
                    }
                });
            });

        ui.add_space(8.0);
    }

    // Main GPU info in two columns
    ui.columns(2, |columns| {
        // Left column: GPU Identity
        Card::new(&colors)
            .title("GPU Information")
            .icon(icons::GPU)
            .show(&mut columns[0], |ui| {
                if let Some(ref stats) = state.gpu_stats {
                    // GPU Name - prominent cyan color
                    ui.label(
                        egui::RichText::new(&stats.name)
                            .strong()
                            .size(16.0)
                            .color(colors.cyan.to_egui()),
                    );
                    ui.add_space(4.0);

                    // Architecture badge with compute capability
                    let arch_color = match stats.architecture.as_str() {
                        "Blackwell" => colors.yellow.to_egui(),
                        "Ada Lovelace" => colors.green.to_egui(),
                        "Ampere" => colors.cyan.to_egui(),
                        "Turing" => colors.purple.to_egui(),
                        "Volta" => colors.blue.to_egui(),
                        "Pascal" => colors.orange.to_egui(),
                        "Maxwell" => colors.magenta.to_egui(),
                        _ => colors.yellow.to_egui(), // Unknown gets yellow so it's visible
                    };

                    ui.horizontal(|ui| {
                        egui::Frame::none()
                            .fill(arch_color.gamma_multiply(0.2))
                            .rounding(4.0)
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(&stats.architecture)
                                        .size(12.0)
                                        .strong()
                                        .color(arch_color),
                                );
                            });

                        // Show compute capability
                        egui::Frame::none()
                            .fill(colors.bg_dark.to_egui())
                            .rounding(4.0)
                            .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(&stats.compute_capability)
                                        .size(11.0)
                                        .color(colors.cyan.to_egui()),
                                );
                            });
                    });

                    ui.add_space(8.0);

                    egui::Grid::new("gpu_info_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("CUDA Cores:").color(colors.cyan.to_egui()),
                            );
                            ui.label(
                                egui::RichText::new(format!("{}", stats.cuda_cores))
                                    .strong()
                                    .color(colors.green.to_egui()),
                            );
                            ui.end_row();

                            ui.label(egui::RichText::new("VRAM:").color(colors.cyan.to_egui()));
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.0} GB GDDR7",
                                    stats.memory_total as f64 / 1e9
                                ))
                                .strong()
                                .color(colors.green.to_egui()),
                            );
                            ui.end_row();

                            ui.label(egui::RichText::new("Driver:").color(colors.cyan.to_egui()));
                            ui.label(
                                egui::RichText::new(&stats.driver_version)
                                    .color(colors.green.to_egui()),
                            );
                            ui.end_row();

                            ui.label(egui::RichText::new("PCI Bus:").color(colors.cyan.to_egui()));
                            ui.label(
                                egui::RichText::new(&stats.pci_bus)
                                    .small()
                                    .color(colors.green.to_egui()),
                            );
                            ui.end_row();
                        });
                } else {
                    ui.label(
                        egui::RichText::new("No GPU data available").color(colors.yellow.to_egui()),
                    );
                }
            });

        // Right column: Live stats
        Card::new(&colors)
            .title("Live Statistics")
            .icon(icons::CHART)
            .show(&mut columns[1], |ui| {
                if let Some(ref stats) = state.gpu_stats {
                    // Temperature
                    let temp_color = if stats.temperature > 80.0 {
                        colors.red.to_egui()
                    } else if stats.temperature > 65.0 {
                        colors.yellow.to_egui()
                    } else {
                        colors.green.to_egui()
                    };

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(icons::TEMP)
                                .size(16.0)
                                .color(temp_color),
                        );
                        ui.label(
                            egui::RichText::new(format!("{:.0}°C", stats.temperature))
                                .size(20.0)
                                .strong()
                                .color(temp_color),
                        );
                    });

                    ui.add_space(8.0);

                    // Utilization bar
                    ui.label(
                        egui::RichText::new("GPU Utilization")
                            .small()
                            .color(colors.fg.to_egui()),
                    );
                    let util_bar = egui::ProgressBar::new(stats.utilization / 100.0)
                        .text(format!("{:.0}%", stats.utilization));
                    ui.add(util_bar);

                    ui.add_space(4.0);

                    // Memory bar
                    let mem_percent = stats.memory_used as f32 / stats.memory_total as f32;
                    ui.label(
                        egui::RichText::new("VRAM Usage")
                            .small()
                            .color(colors.fg.to_egui()),
                    );
                    let mem_bar = egui::ProgressBar::new(mem_percent).text(format!(
                        "{:.1} / {:.1} GB",
                        stats.memory_used as f64 / 1e9,
                        stats.memory_total as f64 / 1e9
                    ));
                    ui.add(mem_bar);

                    ui.add_space(8.0);

                    // Power and clocks
                    egui::Grid::new("live_stats_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Power:").color(colors.fg.to_egui()));
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:.0}W / {:.0}W",
                                    stats.power_draw, stats.power_limit
                                ))
                                .strong()
                                .color(colors.cyan.to_egui()),
                            );
                            ui.end_row();

                            ui.label(egui::RichText::new("Core:").color(colors.fg.to_egui()));
                            ui.label(
                                egui::RichText::new(format!("{} MHz", stats.core_clock))
                                    .strong()
                                    .color(colors.green.to_egui()),
                            );
                            ui.end_row();

                            ui.label(egui::RichText::new("Memory:").color(colors.fg.to_egui()));
                            ui.label(
                                egui::RichText::new(format!("{} MHz", stats.memory_clock))
                                    .strong()
                                    .color(colors.purple.to_egui()),
                            );
                            ui.end_row();

                            ui.label(egui::RichText::new("Fan:").color(colors.fg.to_egui()));
                            ui.label(
                                egui::RichText::new(format!("{}%", stats.fan_speed))
                                    .strong()
                                    .color(colors.blue.to_egui()),
                            );
                            ui.end_row();
                        });

                    // Mini sparklines for history
                    if !state.temp_history.is_empty() {
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("History (2 min)")
                                .small()
                                .color(colors.fg_dark.to_egui()),
                        );
                        ui.add_space(4.0);

                        egui::Grid::new("sparklines_grid")
                            .num_columns(2)
                            .spacing([8.0, 4.0])
                            .show(ui, |ui| {
                                // Temperature sparkline
                                ui.label(
                                    egui::RichText::new("Temp")
                                        .small()
                                        .color(colors.fg_dark.to_egui()),
                                );
                                draw_sparkline(ui, &state.temp_history, temp_color, 20.0, 100.0);
                                ui.end_row();

                                // Utilization sparkline
                                ui.label(
                                    egui::RichText::new("GPU")
                                        .small()
                                        .color(colors.fg_dark.to_egui()),
                                );
                                draw_sparkline(
                                    ui,
                                    &state.util_history,
                                    colors.green.to_egui(),
                                    20.0,
                                    100.0,
                                );
                                ui.end_row();

                                // Power sparkline
                                ui.label(
                                    egui::RichText::new("Power")
                                        .small()
                                        .color(colors.fg_dark.to_egui()),
                                );
                                draw_sparkline(
                                    ui,
                                    &state.power_history,
                                    colors.cyan.to_egui(),
                                    20.0,
                                    stats.power_limit.max(450.0),
                                );
                                ui.end_row();
                            });
                    }
                } else {
                    ui.label(
                        egui::RichText::new("Waiting for GPU data...")
                            .color(colors.yellow.to_egui()),
                    );
                }
            });
    });

    ui.add_space(8.0);

    // ASUS Power Monitor+ Card (if available)
    if state.asus_power_detector.is_some() {
        Card::new(&colors)
            .title("ASUS Power Monitor+")
            .icon(icons::POWER)
            .show(ui, |ui| {
                if let Some(ref status) = state.asus_power_status {
                    // Overall status indicator with health
                    let (status_text, status_color) = if status.has_warnings {
                        ("Warning", colors.yellow.to_egui())
                    } else {
                        match status.health {
                            crate::asus_power_detector::PowerHealth::Good => {
                                ("Good", colors.green.to_egui())
                            }
                            crate::asus_power_detector::PowerHealth::Warning => {
                                ("Warning", colors.yellow.to_egui())
                            }
                            crate::asus_power_detector::PowerHealth::Critical => {
                                ("Critical", colors.red.to_egui())
                            }
                            crate::asus_power_detector::PowerHealth::Unknown => {
                                ("Unknown", colors.fg_dark.to_egui())
                            }
                        }
                    };

                    ui.horizontal(|ui| {
                        // Status badge
                        egui::Frame::none()
                            .fill(status_color.gamma_multiply(0.2))
                            .rounding(4.0)
                            .inner_margin(egui::Margin::symmetric(12.0, 6.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(format!("Health: {}", status_text))
                                        .strong()
                                        .color(status_color),
                                );
                            });

                        // Total power if available
                        if let Some(total_power) = status.total_power_w {
                            egui::Frame::none()
                                .fill(colors.bg_dark.to_egui())
                                .rounding(4.0)
                                .inner_margin(egui::Margin::symmetric(12.0, 6.0))
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{:.1}W Total", total_power))
                                            .strong()
                                            .color(colors.cyan.to_egui()),
                                    );
                                });
                        }

                        // Trend indicator from history
                        let trend = state.asus_power_history.trend();
                        let (trend_icon, trend_color) = match trend {
                            crate::asus_power_detector::PowerTrend::Rising => {
                                ("↑", colors.orange.to_egui())
                            }
                            crate::asus_power_detector::PowerTrend::Falling => {
                                ("↓", colors.green.to_egui())
                            }
                            crate::asus_power_detector::PowerTrend::Stable => {
                                ("→", colors.cyan.to_egui())
                            }
                            crate::asus_power_detector::PowerTrend::Unknown => {
                                ("?", colors.fg_dark.to_egui())
                            }
                        };
                        egui::Frame::none()
                            .fill(colors.bg_dark.to_egui())
                            .rounding(4.0)
                            .inner_margin(egui::Margin::symmetric(8.0, 6.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(trend_icon)
                                        .size(14.0)
                                        .color(trend_color),
                                );
                            });
                    });

                    ui.add_space(8.0);

                    // Two-column layout for rails and stats
                    ui.columns(2, |columns| {
                        // Left column: Power rails
                        columns[0].label(
                            egui::RichText::new("12V-2x6 Power Rails")
                                .strong()
                                .color(colors.fg.to_egui()),
                        );
                        columns[0].add_space(4.0);

                        if !status.rails.is_empty() {
                            egui::Grid::new("power_rails_grid")
                                .num_columns(3)
                                .spacing([12.0, 4.0])
                                .show(&mut columns[0], |ui| {
                                    for rail in &status.rails {
                                        ui.label(
                                            egui::RichText::new(format!("Rail {}", rail.rail_id))
                                                .small()
                                                .color(colors.fg_dark.to_egui()),
                                        );

                                        if let Some(current_ma) = rail.current_ma {
                                            ui.label(
                                                egui::RichText::new(format!(
                                                    "{:.2}A",
                                                    current_ma as f32 / 1000.0
                                                ))
                                                .small()
                                                .color(colors.cyan.to_egui()),
                                            );
                                        } else {
                                            ui.label(
                                                egui::RichText::new("-")
                                                    .small()
                                                    .color(colors.fg_dark.to_egui()),
                                            );
                                        }

                                        let rail_color = if rail.warning {
                                            colors.yellow.to_egui()
                                        } else {
                                            colors.green.to_egui()
                                        };
                                        ui.label(
                                            egui::RichText::new(if rail.warning {
                                                "!"
                                            } else {
                                                "✓"
                                            })
                                            .small()
                                            .color(rail_color),
                                        );
                                        ui.end_row();
                                    }
                                });
                        }

                        // Right column: History stats
                        columns[1].label(
                            egui::RichText::new("Power Statistics")
                                .strong()
                                .color(colors.fg.to_egui()),
                        );
                        columns[1].add_space(4.0);

                        let history = &state.asus_power_history;
                        if !history.is_empty() {
                            egui::Grid::new("power_stats_grid")
                                .num_columns(2)
                                .spacing([12.0, 4.0])
                                .show(&mut columns[1], |ui| {
                                    if let Some(avg) = history.average_power() {
                                        ui.label(
                                            egui::RichText::new("Avg:")
                                                .small()
                                                .color(colors.fg_dark.to_egui()),
                                        );
                                        ui.label(
                                            egui::RichText::new(format!("{:.1}W", avg))
                                                .small()
                                                .color(colors.cyan.to_egui()),
                                        );
                                        ui.end_row();
                                    }

                                    if let Some(peak) = history.peak_power() {
                                        ui.label(
                                            egui::RichText::new("Peak:")
                                                .small()
                                                .color(colors.fg_dark.to_egui()),
                                        );
                                        ui.label(
                                            egui::RichText::new(format!("{:.1}W", peak))
                                                .small()
                                                .color(colors.orange.to_egui()),
                                        );
                                        ui.end_row();
                                    }

                                    if let Some(min) = history.min_power() {
                                        ui.label(
                                            egui::RichText::new("Min:")
                                                .small()
                                                .color(colors.fg_dark.to_egui()),
                                        );
                                        ui.label(
                                            egui::RichText::new(format!("{:.1}W", min))
                                                .small()
                                                .color(colors.green.to_egui()),
                                        );
                                        ui.end_row();
                                    }

                                    ui.label(
                                        egui::RichText::new("Samples:")
                                            .small()
                                            .color(colors.fg_dark.to_egui()),
                                    );
                                    ui.label(
                                        egui::RichText::new(format!("{}", history.len()))
                                            .small()
                                            .color(colors.fg_dark.to_egui()),
                                    );
                                    ui.end_row();

                                    let warnings = history.warning_count();
                                    if warnings > 0 {
                                        ui.label(
                                            egui::RichText::new("Warnings:")
                                                .small()
                                                .color(colors.yellow.to_egui()),
                                        );
                                        ui.label(
                                            egui::RichText::new(format!("{}", warnings))
                                                .small()
                                                .color(colors.yellow.to_egui()),
                                        );
                                        ui.end_row();
                                    }
                                });
                        } else {
                            columns[1].label(
                                egui::RichText::new("Collecting data...")
                                    .small()
                                    .italics()
                                    .color(colors.fg_dark.to_egui()),
                            );
                        }
                    });

                    // Model info at bottom
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "{} (I2C bus {})",
                            status.model, status.i2c_bus
                        ))
                        .small()
                        .color(colors.fg_dark.to_egui()),
                    );

                    // Warnings section
                    if status.has_warnings {
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(icons::WARN).color(colors.yellow.to_egui()),
                            );
                            ui.label(
                                egui::RichText::new("Power rail warning - check connector seating")
                                    .color(colors.yellow.to_egui()),
                            );
                        });
                    }
                } else {
                    ui.label(
                        egui::RichText::new("Initializing power monitoring...")
                            .color(colors.fg_dark.to_egui()),
                    );
                }
            });
    }
    // Note: Repaint handled by app.rs for live monitoring tabs
    let _ = ctx; // Suppress unused warning
}
