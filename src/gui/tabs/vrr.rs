//! VRR (Variable Refresh Rate) & G-Sync Tab
//!
//! Control VRR, G-Sync, and FreeSync settings for connected displays.

use eframe::egui;

use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;

/// Render the VRR tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!(
        "{} VRR (Variable Refresh Rate) & G-Sync Control",
        icons::VRR
    ));
    ui.add_space(4.0);

    // Display VRR Status
    Card::new(&colors)
        .title("Display VRR Status")
        .icon(icons::DISPLAY)
        .show(ui, |ui| {
            // Refresh button
            ui.horizontal(|ui| {
                if ui
                    .button(format!("{} Refresh Displays", icons::REFRESH))
                    .clicked()
                {
                    state.refresh_vrr_displays();
                    state.toasts.info("VRR displays refreshed");
                }
            });

            ui.add_space(8.0);

            if state.vrr_displays.is_empty() {
                ui.label(
                    egui::RichText::new("No VRR-capable displays detected")
                        .weak()
                        .italics(),
                );
            } else {
                // Collect display info to avoid borrow issues
                let display_info: Vec<_> = state
                    .vrr_displays
                    .iter()
                    .map(|d| {
                        (
                            d.display_name.clone(),
                            d.supports_vrr,
                            d.current_settings.enabled,
                            d.min_refresh,
                            d.max_refresh,
                            d.max_mode_refresh,
                            d.supports_gsync,
                            d.supports_freesync,
                        )
                    })
                    .collect();

                let mut vrr_changes: Vec<(String, bool)> = Vec::new();

                for (
                    display_name,
                    supports_vrr,
                    vrr_enabled,
                    min_refresh,
                    max_refresh,
                    max_mode_refresh,
                    supports_gsync,
                    supports_freesync,
                ) in &display_info
                {
                    egui::Frame::new()
                        .fill(colors.bg_dark.to_egui())
                        .corner_radius(6.0)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("📺 {}", display_name)).strong(),
                                );

                                if *supports_vrr == Some(true) {
                                    let mut enabled = *vrr_enabled;
                                    if ui.checkbox(&mut enabled, "VRR Enabled").changed() {
                                        vrr_changes.push((display_name.clone(), enabled));
                                    }
                                } else if *supports_vrr == Some(false) {
                                    ui.colored_label(colors.red.to_egui(), "❌ VRR Not Supported");
                                } else {
                                    ui.label(
                                        egui::RichText::new("VRR capability not reported").weak(),
                                    );
                                }
                            });

                            ui.horizontal(|ui| {
                                if let (Some(min), Some(max)) = (min_refresh, max_refresh) {
                                    ui.label("Reported VRR range:");
                                    ui.label(
                                        egui::RichText::new(format!("{}-{}Hz", min, max))
                                            .color(colors.cyan.to_egui()),
                                    );
                                } else {
                                    ui.label("VRR range: not reported by compositor");
                                }

                                if let Some(max_mode) = max_mode_refresh {
                                    ui.label(format!("Maximum mode: {}Hz", max_mode));
                                }

                                if *supports_gsync == Some(true) {
                                    ui.colored_label(colors.green.to_egui(), "✅ G-Sync");
                                }
                                if *supports_freesync == Some(true) {
                                    ui.colored_label(colors.green.to_egui(), "✅ FreeSync");
                                }
                            });
                        });

                    ui.add_space(4.0);
                }

                // Apply VRR changes after the loop
                for (display_name, enabled) in vrr_changes {
                    state.apply_vrr_to_display(&display_name, enabled);
                }
            }
        });

    ui.add_space(8.0);

    // Advanced VRR Settings
    Card::new(&colors)
        .title("Advanced VRR Settings")
        .icon(icons::SETTINGS)
        .show(ui, |ui| {
            ui.label("VRR policy is reported and configured per output above.");
            ui.label(
                egui::RichText::new(
                    "Panel VRR range, G-SYNC/FreeSync certification, and LFC state are not reported by this compositor.",
                )
                .small()
                .weak(),
            );

            ui.add_space(8.0);

            // Quick actions
            ui.horizontal(|ui| {
                if ui.button("Enable All VRR").clicked() {
                    let displays: Vec<_> = state
                        .vrr_displays
                        .iter()
                        .filter(|d| d.supports_vrr == Some(true))
                        .map(|d| d.display_name.clone())
                        .collect();
                    for display_name in displays {
                        state.apply_vrr_to_display(&display_name, true);
                    }
                }

                if ui.button("Disable All VRR").clicked() {
                    let displays: Vec<_> = state
                        .vrr_displays
                        .iter()
                        .filter(|d| d.supports_vrr == Some(true))
                        .map(|d| d.display_name.clone())
                        .collect();
                    for display_name in displays {
                        state.apply_vrr_to_display(&display_name, false);
                    }
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
                egui::RichText::new("• VRR works best with framerates below max refresh rate")
                    .small(),
            );
            ui.label(
                egui::RichText::new(
                    "• Enable G-Sync in NVIDIA Control Panel for full functionality",
                )
                .small(),
            );
            ui.label(
                egui::RichText::new("• Some compositors require additional configuration").small(),
            );
            ui.label(
                egui::RichText::new("• For competitive gaming, aim for framerates above VRR range")
                    .small(),
            );
        });
}
