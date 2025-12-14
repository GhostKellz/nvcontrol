//! Display & Color Management Tab
//!
//! Display settings, ICC profiles, color depth, and HDR status.

use eframe::egui;

use crate::display;
use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;
use crate::vibrance;

/// Render the Display tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} Display & Color Management", icons::DISPLAY));
    ui.add_space(4.0);

    // Digital Vibrance (quick access)
    Card::new(&colors)
        .title("Digital Vibrance")
        .icon(icons::VIBRANCE)
        .show(ui, |ui| {
            let mut changed = false;
            for (i, level) in state.vibrance_levels.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Display {}", i));

                    // Convert from -1024..1023 range to 0..100 percentage
                    let mut percentage = ((*level + 1024) as f32 / 2047.0 * 100.0) as u32;

                    if ui
                        .add(egui::Slider::new(&mut percentage, 0..=100).suffix("%"))
                        .changed()
                    {
                        // Convert back to -1024..1023 range
                        *level = ((percentage as f32 / 100.0 * 2047.0) - 1024.0) as i16;
                        changed = true;
                    }

                    // Show raw value for advanced users
                    ui.label(egui::RichText::new(format!("({})", level)).small().weak());

                    // Quick preset buttons
                    if ui.small_button("Off").clicked() {
                        *level = 0;
                        changed = true;
                    }
                    if ui.small_button("50%").clicked() {
                        *level = 512;
                        changed = true;
                    }
                    if ui.small_button("Max").clicked() {
                        *level = 1023;
                        changed = true;
                    }
                });
            }

            if changed {
                let display_values: Vec<(usize, i32)> = state
                    .vibrance_levels
                    .iter()
                    .enumerate()
                    .map(|(idx, &level)| (idx, level as i32))
                    .collect();
                let _ = vibrance::set_vibrance(&display_values);
                state.config.vibrance_levels = state.vibrance_levels.clone();
                state.config.save();
            }
        });

    ui.add_space(8.0);

    // ICC Profile Management
    Card::new(&colors)
        .title("ICC Profile Management")
        .icon(icons::VIBRANCE)
        .show(ui, |ui| {
            let icc_profiles = display::list_icc_profiles();
            if icc_profiles.is_empty() {
                ui.label(
                    egui::RichText::new("No ICC profiles found")
                        .weak()
                        .italics(),
                );
            } else {
                // Update available profiles
                if state.available_icc_profiles != icc_profiles {
                    state.available_icc_profiles = icc_profiles.clone();
                }

                egui::ComboBox::from_label("ICC Profile")
                    .selected_text(
                        state
                            .available_icc_profiles
                            .get(state.selected_icc_profile_idx)
                            .map(|s| s.as_str())
                            .unwrap_or("None"),
                    )
                    .show_ui(ui, |cb_ui| {
                        for (i, profile) in state.available_icc_profiles.iter().enumerate() {
                            cb_ui.selectable_value(&mut state.selected_icc_profile_idx, i, profile);
                        }
                    });

                if ui.button("Apply ICC Profile").clicked() {
                    if let Some(profile) = state
                        .available_icc_profiles
                        .get(state.selected_icc_profile_idx)
                    {
                        match display::load_icc_profile(0, profile) {
                            Ok(()) => {
                                state.config.selected_icc_profile = profile.clone();
                                state.config.save();
                                state.toasts.success("ICC profile applied successfully");
                            }
                            Err(e) => {
                                state
                                    .toasts
                                    .error(format!("Failed to apply profile: {}", e));
                            }
                        }
                    }
                }
            }

            ui.add_space(4.0);

            if ui.button("Open ICC Folder").clicked() {
                match display::open_icc_folder() {
                    Ok(()) => state.toasts.info("Opened ICC folder"),
                    Err(e) => state.toasts.error(format!("Failed to open folder: {}", e)),
                }
            }
        });

    ui.add_space(8.0);

    // Display HDR Capabilities
    Card::new(&colors)
        .title("Display HDR Status")
        .icon(icons::HDR)
        .show(ui, |ui| {
            let displays = display::list_displays();
            if displays.is_empty() {
                ui.label(egui::RichText::new("No displays detected").weak().italics());
            } else {
                for disp in displays {
                    egui::Frame::none()
                        .fill(colors.bg_dark.to_egui())
                        .rounding(6.0)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("{}: {}", disp.name, disp.kind))
                                        .strong(),
                                );

                                if disp.hdr_capable {
                                    let (status_text, status_color) = if disp.hdr_enabled {
                                        ("✅ HDR ON", colors.green.to_egui())
                                    } else {
                                        ("⚫ HDR OFF", colors.fg_dark.to_egui())
                                    };
                                    ui.colored_label(status_color, status_text);
                                    ui.label(format!("{}bit", disp.color_depth));
                                } else {
                                    ui.colored_label(colors.red.to_egui(), "❌ No HDR");
                                }
                            });
                        });

                    ui.add_space(4.0);
                }
            }

            ui.add_space(8.0);

            // HDR Toggle
            let mut hdr_enabled = state.hdr_enabled;
            if ui.checkbox(&mut hdr_enabled, "Enable HDR").changed() {
                state.toggle_hdr();
            }
        });
}
