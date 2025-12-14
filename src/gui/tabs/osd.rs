//! OSD (On-Screen Display) Tab
//!
//! MangoHud configuration for in-game performance overlay.
//! Provides real-time FPS, temperatures, and GPU metrics during gameplay.

use eframe::egui;

use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;

/// All available OSD metrics with their display names
const OSD_METRICS: &[(&str, &str)] = &[
    ("fps", "FPS Counter"),
    ("frametime", "Frame Time Graph"),
    ("gpu_temp", "GPU Temperature"),
    ("gpu_load", "GPU Utilization"),
    ("gpu_core_clock", "GPU Clock Speed"),
    ("gpu_power", "GPU Power Draw"),
    ("vram", "VRAM Usage"),
    ("fan", "Fan Speed"),
    ("cpu_temp", "CPU Temperature"),
    ("cpu_load", "CPU Utilization"),
    ("ram", "RAM Usage"),
    ("gpu_name", "GPU Name"),
];

/// Render the OSD tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} On-Screen Display (OSD)", icons::BENCHMARK));
    ui.label(
        egui::RichText::new("Configure performance overlay via MangoHud")
            .small()
            .color(colors.fg_dark.to_egui()),
    );
    ui.add_space(8.0);

    // Backend Status Card
    Card::new(&colors)
        .title("OSD Backend")
        .icon(icons::INFO)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if state.mangohud_installed {
                    ui.colored_label(colors.green.to_egui(), "✅ MangoHud detected");
                } else {
                    ui.colored_label(colors.red.to_egui(), "❌ MangoHud not installed");
                }
            });

            if !state.mangohud_installed {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("Install MangoHud for OSD support:")
                        .small()
                        .color(colors.yellow.to_egui()),
                );
                egui::Frame::none()
                    .fill(colors.bg_dark.to_egui())
                    .rounding(4.0)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("# Arch Linux\nsudo pacman -S mangohud\n\n# Fedora\nsudo dnf install mangohud\n\n# Ubuntu\nsudo apt install mangohud")
                                .monospace()
                                .small()
                                .color(colors.fg.to_egui()),
                        );
                    });
            }
        });

    ui.add_space(8.0);

    // OSD Status Card
    Card::new(&colors)
        .title("OSD Status")
        .icon(icons::GAME)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Enable OSD:");
                if ui.checkbox(&mut state.osd_enabled, "").changed() {
                    save_mangohud_config(state);
                    if state.osd_enabled {
                        state.toasts.success("OSD enabled");
                    } else {
                        state.toasts.info("OSD disabled");
                    }
                }
            });

            if state.osd_enabled {
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("Launch games with: mangohud %command%")
                        .small()
                        .color(colors.cyan.to_egui()),
                );
                ui.label(
                    egui::RichText::new("Or set MANGOHUD=1 in Steam launch options")
                        .small()
                        .color(colors.fg_dark.to_egui()),
                );
            }
        });

    ui.add_space(8.0);

    // Position Card
    Card::new(&colors)
        .title("Position")
        .icon(icons::DISPLAY)
        .show(ui, |ui| {
            let positions = [
                ("top-left", "Top Left"),
                ("top-right", "Top Right"),
                ("bottom-left", "Bottom Left"),
                ("bottom-right", "Bottom Right"),
            ];

            egui::ComboBox::from_label("OSD Position")
                .selected_text(
                    positions
                        .iter()
                        .find(|(id, _)| *id == state.osd_position)
                        .map(|(_, name)| *name)
                        .unwrap_or("Top Left"),
                )
                .show_ui(ui, |ui| {
                    for (id, name) in positions {
                        if ui
                            .selectable_label(state.osd_position == id, name)
                            .clicked()
                        {
                            state.osd_position = id.to_string();
                            save_mangohud_config(state);
                        }
                    }
                });
        });

    ui.add_space(8.0);

    // Metrics Card
    Card::new(&colors)
        .title("Metrics to Display")
        .icon(icons::CHART)
        .show(ui, |ui| {
            ui.columns(2, |columns| {
                for (i, (metric_id, metric_name)) in OSD_METRICS.iter().enumerate() {
                    let col = i % 2;
                    let mut enabled = state.osd_metrics.contains(&metric_id.to_string());
                    if columns[col].checkbox(&mut enabled, *metric_name).changed() {
                        if enabled {
                            if !state.osd_metrics.contains(&metric_id.to_string()) {
                                state.osd_metrics.push(metric_id.to_string());
                            }
                        } else {
                            state.osd_metrics.retain(|m| m != *metric_id);
                        }
                        save_mangohud_config(state);
                    }
                }
            });
        });

    ui.add_space(8.0);

    // Appearance Card
    Card::new(&colors)
        .title("Appearance")
        .icon(icons::VIBRANCE)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Font Size:");
                let mut font_size = state.osd_font_size as i32;
                if ui
                    .add(egui::Slider::new(&mut font_size, 12..=48).suffix("px"))
                    .changed()
                {
                    state.osd_font_size = font_size as u32;
                    save_mangohud_config(state);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Background Opacity:");
                if ui
                    .add(egui::Slider::new(&mut state.osd_opacity, 0.0..=1.0))
                    .changed()
                {
                    save_mangohud_config(state);
                }
            });
        });

    ui.add_space(8.0);

    // Quick Presets Card
    Card::new(&colors)
        .title("Quick Presets")
        .icon(icons::ROCKET)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Minimal").on_hover_text("FPS only").clicked() {
                    state.osd_metrics = vec!["fps".to_string()];
                    state.osd_font_size = 24;
                    state.osd_opacity = 0.5;
                    save_mangohud_config(state);
                    state.toasts.info("Minimal preset applied");
                }

                if ui
                    .button("Standard")
                    .on_hover_text("FPS, frametime, GPU temp/load, VRAM")
                    .clicked()
                {
                    state.osd_metrics = vec![
                        "fps".to_string(),
                        "frametime".to_string(),
                        "gpu_temp".to_string(),
                        "gpu_load".to_string(),
                        "vram".to_string(),
                    ];
                    state.osd_font_size = 24;
                    state.osd_opacity = 0.8;
                    save_mangohud_config(state);
                    state.toasts.info("Standard preset applied");
                }

                if ui
                    .button("Full")
                    .on_hover_text("All GPU and CPU metrics")
                    .clicked()
                {
                    state.osd_metrics = vec![
                        "fps".to_string(),
                        "frametime".to_string(),
                        "gpu_temp".to_string(),
                        "gpu_load".to_string(),
                        "gpu_core_clock".to_string(),
                        "gpu_power".to_string(),
                        "vram".to_string(),
                        "fan".to_string(),
                        "cpu_temp".to_string(),
                        "cpu_load".to_string(),
                        "ram".to_string(),
                    ];
                    state.osd_font_size = 20;
                    state.osd_opacity = 0.9;
                    save_mangohud_config(state);
                    state.toasts.info("Full preset applied");
                }

                if ui
                    .button("Benchmark")
                    .on_hover_text("Optimized for benchmarking/reviews")
                    .clicked()
                {
                    state.osd_metrics = vec![
                        "fps".to_string(),
                        "frametime".to_string(),
                        "gpu_temp".to_string(),
                        "gpu_load".to_string(),
                        "gpu_core_clock".to_string(),
                        "gpu_power".to_string(),
                        "cpu_load".to_string(),
                    ];
                    state.osd_font_size = 28;
                    state.osd_opacity = 0.9;
                    save_mangohud_config(state);
                    state.toasts.info("Benchmark preset applied");
                }
            });
        });

    ui.add_space(8.0);

    // Config Preview Card
    Card::new(&colors)
        .title("Config Preview (MangoHud.conf)")
        .icon(icons::INFO)
        .show(ui, |ui| {
            let config_preview = generate_mangohud_config(state);

            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut config_preview.as_str())
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .interactive(false),
                    );
                });

            ui.add_space(4.0);

            let config_path = dirs::config_dir()
                .map(|p| p.join("MangoHud").join("MangoHud.conf"))
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "~/.config/MangoHud/MangoHud.conf".to_string());

            ui.label(
                egui::RichText::new(format!("Saved to: {}", config_path))
                    .small()
                    .weak(),
            );
        });
}

/// Generate MangoHud configuration content
fn generate_mangohud_config(state: &GuiState) -> String {
    let mut lines = vec![
        "# nvcontrol MangoHud configuration".to_string(),
        "# Auto-generated by nvcontrol GUI".to_string(),
        "".to_string(),
        format!("position={}", state.osd_position),
        "".to_string(),
    ];

    // Add enabled metrics
    for metric in &state.osd_metrics {
        lines.push(metric.clone());
    }

    lines.push("".to_string());
    lines.push(format!("font_size={}", state.osd_font_size));
    lines.push(format!("background_alpha={:.2}", state.osd_opacity));

    lines.join("\n")
}

/// Save MangoHud configuration to disk
fn save_mangohud_config(state: &GuiState) {
    if let Some(config_dir) = dirs::config_dir() {
        let mangohud_dir = config_dir.join("MangoHud");
        if std::fs::create_dir_all(&mangohud_dir).is_ok() {
            let config_path = mangohud_dir.join("MangoHud.conf");
            let config_content = generate_mangohud_config(state);
            if let Err(e) = std::fs::write(&config_path, &config_content) {
                eprintln!("Failed to save MangoHud config: {}", e);
            }
        }
    }
}
