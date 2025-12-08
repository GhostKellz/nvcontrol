//! HDR Configuration Tab
//!
//! HDR settings, metadata, tone mapping, and color space controls.

use eframe::egui;

use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;
use crate::hdr::{self, ColorSpace, Eotf, ToneMappingMode};

/// Render the HDR tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} HDR Configuration", icons::HDR));
    ui.add_space(4.0);

    // HDR Status
    Card::new(&colors)
        .title("HDR Status")
        .icon(icons::HDR)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let mut enabled = state.hdr_enabled;
                if ui.checkbox(&mut enabled, "Enable HDR").changed() {
                    state.toggle_hdr();
                }
            });

            ui.add_space(8.0);

            // Display capabilities
            if let Ok(caps) = hdr::get_hdr_capabilities() {
                ui.label(
                    egui::RichText::new("Display Capabilities:")
                        .strong()
                        .color(colors.cyan.to_egui()),
                );

                egui::Grid::new("hdr_caps_grid")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Peak Luminance:");
                        ui.label(
                            egui::RichText::new(format!("{} nits", caps.max_luminance))
                                .color(colors.yellow.to_egui()),
                        );
                        ui.end_row();

                        ui.label("Min Luminance:");
                        ui.label(
                            egui::RichText::new(format!("{:.4} nits", caps.min_luminance))
                                .color(colors.fg.to_egui()),
                        );
                        ui.end_row();
                    });

                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    if caps.supports_hdr10 {
                        ui.colored_label(colors.green.to_egui(), "✅ HDR10");
                    }
                    if caps.supports_hdr10_plus {
                        ui.colored_label(colors.green.to_egui(), "✅ HDR10+");
                    }
                    if caps.supports_dolby_vision {
                        ui.colored_label(colors.purple.to_egui(), "✅ Dolby Vision");
                    }
                    if caps.supports_hlg {
                        ui.colored_label(colors.cyan.to_egui(), "✅ HLG");
                    }
                });
            } else {
                ui.colored_label(
                    colors.yellow.to_egui(),
                    "Unable to detect HDR capabilities",
                );
            }
        });

    ui.add_space(8.0);

    // HDR Metadata
    Card::new(&colors)
        .title("HDR Metadata")
        .icon(icons::CHART)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Peak Brightness:");
                let mut peak = state.hdr_config.peak_brightness as f32;
                if ui
                    .add(egui::Slider::new(&mut peak, 100.0..=10000.0).suffix(" nits"))
                    .changed()
                {
                    state.hdr_config.peak_brightness = peak as u32;
                    let _ = state.hdr_config.save();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Min Brightness:");
                let mut min = state.hdr_config.min_brightness;
                if ui
                    .add(
                        egui::Slider::new(&mut min, 0.0001..=0.1)
                            .suffix(" nits")
                            .logarithmic(true),
                    )
                    .changed()
                {
                    state.hdr_config.min_brightness = min;
                    let _ = state.hdr_config.save();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Max Content Light Level:");
                let mut max_cll = state.hdr_config.max_content_light_level as f32;
                if ui
                    .add(egui::Slider::new(&mut max_cll, 100.0..=10000.0).suffix(" nits"))
                    .changed()
                {
                    state.hdr_config.max_content_light_level = max_cll as u32;
                    let _ = state.hdr_config.save();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Max Frame Average:");
                let mut max_fall = state.hdr_config.max_frame_average as f32;
                if ui
                    .add(egui::Slider::new(&mut max_fall, 50.0..=5000.0).suffix(" nits"))
                    .changed()
                {
                    state.hdr_config.max_frame_average = max_fall as u32;
                    let _ = state.hdr_config.save();
                }
            });
        });

    ui.add_space(8.0);

    // Color & Tone Mapping
    Card::new(&colors)
        .title("Color & Tone Mapping")
        .icon(icons::VIBRANCE)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tone Mapping:");
                egui::ComboBox::from_id_salt("tone_mapping")
                    .selected_text(format!("{}", state.hdr_config.tone_mapping))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut state.hdr_config.tone_mapping,
                            ToneMappingMode::None,
                            "None (Clip)",
                        );
                        ui.selectable_value(
                            &mut state.hdr_config.tone_mapping,
                            ToneMappingMode::Reinhard,
                            "Reinhard",
                        );
                        ui.selectable_value(
                            &mut state.hdr_config.tone_mapping,
                            ToneMappingMode::Hable,
                            "Hable (Uncharted 2)",
                        );
                        ui.selectable_value(
                            &mut state.hdr_config.tone_mapping,
                            ToneMappingMode::ACES,
                            "ACES Filmic",
                        );
                        ui.selectable_value(
                            &mut state.hdr_config.tone_mapping,
                            ToneMappingMode::AGX,
                            "AGX",
                        );
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Color Space:");
                egui::ComboBox::from_id_salt("color_space")
                    .selected_text(format!("{}", state.hdr_config.color_space))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut state.hdr_config.color_space,
                            ColorSpace::BT709,
                            "BT.709 (SDR)",
                        );
                        ui.selectable_value(
                            &mut state.hdr_config.color_space,
                            ColorSpace::BT2020,
                            "BT.2020 (HDR)",
                        );
                        ui.selectable_value(
                            &mut state.hdr_config.color_space,
                            ColorSpace::DciP3,
                            "DCI-P3 (Wide Gamut)",
                        );
                    });
            });

            ui.horizontal(|ui| {
                ui.label("EOTF:");
                egui::ComboBox::from_id_salt("eotf")
                    .selected_text(format!("{}", state.hdr_config.eotf))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut state.hdr_config.eotf,
                            Eotf::Gamma22,
                            "Gamma 2.2 (SDR)",
                        );
                        ui.selectable_value(
                            &mut state.hdr_config.eotf,
                            Eotf::PQ,
                            "PQ (HDR10)",
                        );
                        ui.selectable_value(
                            &mut state.hdr_config.eotf,
                            Eotf::HLG,
                            "HLG (HDR10+)",
                        );
                    });
            });

            ui.add_space(8.0);

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("💾 Save Config").clicked() {
                    if let Err(e) = state.hdr_config.save() {
                        state.toasts.error(format!("Failed to save HDR config: {}", e));
                    } else {
                        state.toasts.success("HDR configuration saved");
                    }
                }

                if ui.button("🔄 Reset to Defaults").clicked() {
                    state.hdr_config = hdr::HdrConfig::default();
                    let _ = state.hdr_config.save();
                    state.toasts.info("HDR settings reset to defaults");
                }
            });
        });

    ui.add_space(8.0);

    // Tips
    Card::new(&colors)
        .title("Tips")
        .icon(icons::BULB)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new("• HDR requires a compatible display and content").small(),
            );
            ui.label(
                egui::RichText::new("• Peak brightness should match your display's capability")
                    .small(),
            );
            ui.label(
                egui::RichText::new("• ACES tone mapping is recommended for most content").small(),
            );
            ui.label(
                egui::RichText::new("• BT.2020 color space is used for most HDR content").small(),
            );
        });
}
