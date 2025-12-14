//! Settings Tab
//!
//! Application settings and keyboard shortcuts reference.

use eframe::egui;

use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;
use crate::themes::ThemeVariant;

/// Render the settings tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} Settings", icons::SETTINGS));
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        // Left column: Theme and preferences
        Card::new(&colors)
            .title("Appearance")
            .icon(icons::VIBRANCE)
            .show(&mut columns[0], |ui| {
                ui.label("Theme:");
                ui.horizontal(|ui| {
                    let themes = [
                        (ThemeVariant::TokyoNightNight, "Tokyo Night Night"),
                        (ThemeVariant::TokyoNightStorm, "Tokyo Night Storm"),
                        (ThemeVariant::TokyoNightMoon, "Tokyo Night Moon"),
                        (ThemeVariant::Dracula, "Dracula"),
                        (ThemeVariant::RogRed, "ROG Red"),
                        (ThemeVariant::MatrixGreen, "Matrix Green"),
                        (ThemeVariant::Cyberpunk, "Cyberpunk"),
                    ];

                    egui::ComboBox::from_id_salt("theme_selector")
                        .selected_text(format!(
                            "{} {}",
                            state.theme_icon(),
                            state.current_theme.name()
                        ))
                        .show_ui(ui, |ui| {
                            for (variant, name) in themes {
                                if ui
                                    .selectable_value(&mut state.current_theme, variant, name)
                                    .changed()
                                {
                                    crate::gui::theme::apply_theme(ctx, variant);
                                    state.save_config();
                                }
                            }
                        });
                });

                ui.add_space(8.0);

                if ui.button(format!("{} Cycle Theme (Ctrl+T)", icons::REFRESH)).clicked() {
                    state.cycle_theme();
                    crate::gui::theme::apply_theme(ctx, state.current_theme);
                }

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                // UI Scale setting for 4K displays
                ui.label(egui::RichText::new("UI Scale (for 4K/HiDPI):").strong());
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    let scale_text = format!("{:.0}%", state.ui_scale * 100.0);
                    ui.add(
                        egui::Slider::new(&mut state.ui_scale, 0.75..=2.5)
                            .text(scale_text)
                            .step_by(0.25)
                    );
                });

                ui.horizontal(|ui| {
                    if ui.button("100%").clicked() {
                        state.ui_scale = 1.0;
                        ctx.set_pixels_per_point(state.ui_scale);
                    }
                    if ui.button("125%").clicked() {
                        state.ui_scale = 1.25;
                        ctx.set_pixels_per_point(state.ui_scale);
                    }
                    if ui.button("150%").clicked() {
                        state.ui_scale = 1.5;
                        ctx.set_pixels_per_point(state.ui_scale);
                    }
                    if ui.button("175% (4K)").clicked() {
                        state.ui_scale = 1.75;
                        ctx.set_pixels_per_point(state.ui_scale);
                    }
                    if ui.button("200%").clicked() {
                        state.ui_scale = 2.0;
                        ctx.set_pixels_per_point(state.ui_scale);
                    }
                });

                // Apply scale when slider changes
                if ui.input(|i| i.pointer.any_released()) {
                    ctx.set_pixels_per_point(state.ui_scale);
                }
            });

        columns[0].add_space(8.0);

        // System info
        Card::new(&colors)
            .title("System Information")
            .icon(icons::INFO)
            .show(&mut columns[0], |ui| {
                if let Some(ref stats) = state.gpu_stats {
                    egui::Grid::new("system_info")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("GPU:");
                            ui.label(
                                egui::RichText::new(&stats.name)
                                    .strong()
                                    .color(colors.green.to_egui()),
                            );
                            ui.end_row();

                            ui.label("Architecture:");
                            ui.label(
                                egui::RichText::new(format!("{} ({})", &stats.architecture, &stats.compute_capability))
                                    .color(colors.yellow.to_egui()),
                            );
                            ui.end_row();

                            ui.label("Driver:");
                            ui.label(&stats.driver_version);
                            ui.end_row();

                            ui.label("VRAM:");
                            ui.label(format!("{:.0} GB", stats.memory_total as f64 / 1e9));
                            ui.end_row();
                        });
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                // About section
                ui.label(
                    egui::RichText::new("📦 About nvcontrol")
                        .strong()
                        .color(colors.blue.to_egui()),
                );

                egui::Grid::new("about_info")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Version:");
                        ui.label(
                            egui::RichText::new(env!("CARGO_PKG_VERSION"))
                                .strong()
                                .color(colors.green.to_egui()),
                        );
                        ui.end_row();

                        ui.label("Theme:");
                        ui.label(format!("{} {}", state.theme_icon(), state.current_theme.name()));
                        ui.end_row();

                        ui.label("Config:");
                        ui.label(egui::RichText::new("~/.config/nvcontrol/").small());
                        ui.end_row();
                    });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui
                        .button("📋 Copy System Info")
                        .on_hover_text("Copy system info to clipboard")
                        .clicked()
                    {
                        if let Some(ref stats) = state.gpu_stats {
                            let info = format!(
                                "nvcontrol v{}\n\nGPU: {}\nArchitecture: {}\nDriver: {}\nVRAM: {:.0} GB\nTheme: {}",
                                env!("CARGO_PKG_VERSION"),
                                stats.name,
                                stats.architecture,
                                stats.driver_version,
                                stats.memory_total as f64 / 1e9,
                                state.current_theme.name()
                            );
                            ctx.copy_text(info);
                            state.toasts.success("System info copied to clipboard");
                        }
                    }

                    if ui
                        .button("🔗 GitHub")
                        .on_hover_text("Open project on GitHub")
                        .clicked()
                    {
                        let _ = std::process::Command::new("xdg-open")
                            .arg("https://github.com/ghostkellz/nvcontrol")
                            .spawn();
                    }
                });
            });

        // Right column: Keyboard shortcuts
        let kb_colors = state.theme_colors();

        Card::new(&kb_colors)
            .title("Keyboard Shortcuts")
            .icon("⌨️")
            .show(&mut columns[1], |ui| {
                egui::Grid::new("shortcuts_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("1-9")
                                .monospace()
                                .strong()
                                .color(kb_colors.cyan.to_egui()),
                        );
                        ui.label(
                            egui::RichText::new("Quick tab navigation")
                                .small()
                                .color(kb_colors.fg.to_egui()),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("Ctrl+S")
                                .monospace()
                                .strong()
                                .color(kb_colors.cyan.to_egui()),
                        );
                        ui.label(
                            egui::RichText::new("Save configuration")
                                .small()
                                .color(kb_colors.fg.to_egui()),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("Ctrl+R")
                                .monospace()
                                .strong()
                                .color(kb_colors.cyan.to_egui()),
                        );
                        ui.label(
                            egui::RichText::new("Reset OC to stock")
                                .small()
                                .color(kb_colors.fg.to_egui()),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("Ctrl+T")
                                .monospace()
                                .strong()
                                .color(kb_colors.cyan.to_egui()),
                        );
                        ui.label(
                            egui::RichText::new("Cycle themes")
                                .small()
                                .color(kb_colors.fg.to_egui()),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new("F5")
                                .monospace()
                                .strong()
                                .color(kb_colors.cyan.to_egui()),
                        );
                        ui.label(
                            egui::RichText::new("Refresh display")
                                .small()
                                .color(kb_colors.fg.to_egui()),
                        );
                        ui.end_row();
                    });
            });

        columns[1].add_space(8.0);

        Card::new(&kb_colors)
            .title("Tab Shortcuts")
            .icon("🔢")
            .show(&mut columns[1], |ui| {
                let tab_shortcuts = [
                    ("1", "GPU Status"),
                    ("2", "Overclock"),
                    ("3", "Fan Control"),
                    ("4", "Display"),
                    ("5", "Vibrance"),
                    ("6", "HDR"),
                    ("7", "Profiles"),
                    ("8", "OSD"),
                    ("9", "Settings"),
                ];

                egui::Grid::new("tab_shortcuts")
                    .num_columns(2)
                    .spacing([20.0, 2.0])
                    .show(ui, |ui| {
                        for (key, tab) in tab_shortcuts {
                            ui.label(
                                egui::RichText::new(key)
                                    .monospace()
                                    .color(kb_colors.purple.to_egui()),
                            );
                            ui.label(
                                egui::RichText::new(tab).color(kb_colors.fg.to_egui()),
                            );
                            ui.end_row();
                        }
                    });
            });
    });
}
