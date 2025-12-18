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
                    supports_gsync,
                    supports_freesync,
                ) in &display_info
                {
                    egui::Frame::none()
                        .fill(colors.bg_dark.to_egui())
                        .rounding(6.0)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("📺 {}", display_name)).strong(),
                                );

                                if *supports_vrr {
                                    let mut enabled = *vrr_enabled;
                                    if ui.checkbox(&mut enabled, "VRR Enabled").changed() {
                                        vrr_changes.push((display_name.clone(), enabled));
                                    }
                                } else {
                                    ui.colored_label(colors.red.to_egui(), "❌ VRR Not Supported");
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Refresh Range:");
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{}-{}Hz",
                                        min_refresh, max_refresh
                                    ))
                                    .color(colors.cyan.to_egui()),
                                );

                                if *supports_gsync {
                                    ui.colored_label(colors.green.to_egui(), "✅ G-Sync");
                                }
                                if *supports_freesync {
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
            // These are display-wide settings
            ui.horizontal(|ui| {
                ui.label("Low Framerate Compensation (LFC):");
                ui.label(egui::RichText::new("Enabled").color(colors.green.to_egui()));
            });

            ui.horizontal(|ui| {
                ui.label("Adaptive Sync Mode:");
                ui.label(egui::RichText::new("Active").color(colors.green.to_egui()));
            });

            ui.add_space(8.0);

            // Quick actions
            ui.horizontal(|ui| {
                if ui.button("Enable All VRR").clicked() {
                    let displays: Vec<_> = state
                        .vrr_displays
                        .iter()
                        .filter(|d| d.supports_vrr)
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
                        .filter(|d| d.supports_vrr)
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
