//! Digital Vibrance Tab
//!
//! Per-display digital vibrance control using native NVKMS or nvibrant fallback.

use eframe::egui;

use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;
use crate::vibrance;

/// Render the Vibrance tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} Digital Vibrance Control", icons::VIBRANCE));
    ui.add_space(4.0);

    // Backend Status
    Card::new(&colors)
        .title("Vibrance Backend Status")
        .icon(icons::INFO)
        .show(ui, |ui| {
            // Try native NVKMS controller first (preferred for 580+ drivers)
            match crate::vibrance_native::get_vibrance_controller() {
                Ok(guard) => {
                    if let Some(controller) = guard.as_ref() {
                        ui.colored_label(
                            colors.green.to_egui(),
                            format!("{} Native Digital Vibrance Available", icons::OK),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "Driver: {} (Open)",
                                controller.driver_version
                            ))
                            .small(),
                        );
                        ui.label(
                            egui::RichText::new(
                                "Using direct NVKMS ioctls - no external dependencies",
                            )
                            .small()
                            .weak(),
                        );

                        // Show detected displays
                        let displays = controller.list_displays();
                        if !displays.is_empty() {
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(format!(
                                    "Detected {} display(s)",
                                    displays.len()
                                ))
                                .small()
                                .color(colors.yellow.to_egui()),
                            );
                        }
                    }
                }
                Err(e) => {
                    // Native not available, check for nvibrant fallback
                    if vibrance::is_available() {
                        ui.colored_label(
                            colors.yellow.to_egui(),
                            format!("{} Using nvibrant fallback", icons::WARN),
                        );
                        match vibrance::get_driver_info() {
                            Ok(info) => {
                                ui.label(egui::RichText::new(format!("Driver: {}", info)).small())
                            }
                            Err(_) => {
                                ui.label(egui::RichText::new("Driver version unknown").small().weak())
                            }
                        };
                    } else {
                        ui.colored_label(
                            colors.red.to_egui(),
                            format!("{} Vibrance Not Available", icons::ERR),
                        );
                        ui.label(egui::RichText::new(format!("Error: {}", e)).small().weak());
                        ui.add_space(4.0);
                        ui.label("Requirements:");
                        ui.label(egui::RichText::new("• NVIDIA open drivers 580+").small());
                        ui.label(
                            egui::RichText::new("• nvidia_drm.modeset=1 in kernel params").small(),
                        );
                        ui.label(
                            egui::RichText::new("• User in 'video' group or run as root").small(),
                        );
                    }
                }
            }
        });

    ui.add_space(8.0);

    // Per-Display Vibrance Control
    Card::new(&colors)
        .title("Per-Display Vibrance Control")
        .icon(icons::DISPLAY)
        .show(ui, |ui| {
            // Try native controller first
            match crate::vibrance_native::get_vibrance_controller() {
                Ok(guard) => {
                    if let Some(controller) = guard.as_ref() {
                        let displays = controller.list_displays();
                        if displays.is_empty() {
                            ui.label(
                                egui::RichText::new("No displays detected")
                                    .weak()
                                    .italics(),
                            );
                        } else {
                            for (_device_id, connector_idx, name, connected) in &displays {
                                ui.horizontal(|ui| {
                                    let status_icon = if *connected {
                                        icons::OK
                                    } else {
                                        icons::ERR
                                    };
                                    ui.label(
                                        egui::RichText::new(format!("{} {}", status_icon, name))
                                            .strong(),
                                    );

                                    // Get current vibrance from connectors
                                    let current_pct: i32 = controller
                                        .connectors
                                        .get(*connector_idx as usize)
                                        .map(|c| {
                                            controller.vibrance_to_percentage(c.current_vibrance)
                                                as i32
                                        })
                                        .unwrap_or(100);

                                    let mut percentage = current_pct;
                                    if ui
                                        .add(
                                            egui::Slider::new(&mut percentage, 0..=200)
                                                .suffix("%")
                                                .text("Vibrance"),
                                        )
                                        .changed()
                                    {
                                        if let Err(e) =
                                            crate::vibrance_native::set_display_vibrance_native(
                                                0,
                                                *connector_idx,
                                                percentage as u32,
                                            )
                                        {
                                            state.toasts.error(format!(
                                                "Failed to set vibrance: {}",
                                                e
                                            ));
                                        }
                                    }
                                });
                            }
                        }
                    } else {
                        ui.label(
                            egui::RichText::new("Controller not initialized")
                                .weak()
                                .italics(),
                        );
                    }
                }
                Err(_) => {
                    // Fall back to nvibrant-based control
                    match vibrance::get_displays() {
                        Ok(displays) => {
                            for (i, display) in displays.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("Display {}: {}", i, display));

                                    let current_vibrance =
                                        vibrance::get_display_vibrance(i).unwrap_or(0);
                                    let mut percentage =
                                        vibrance::vibrance_to_percentage(current_vibrance) as i32;

                                    if ui
                                        .add(
                                            egui::Slider::new(&mut percentage, 0..=200)
                                                .suffix("%")
                                                .text("Vibrance"),
                                        )
                                        .changed()
                                    {
                                        let vibrance_val =
                                            vibrance::percentage_to_vibrance(percentage as u32);
                                        let display_values = vec![(i, vibrance_val)];
                                        if let Err(e) = vibrance::set_vibrance(&display_values) {
                                            state.toasts.error(format!(
                                                "Failed to set vibrance: {}",
                                                e
                                            ));
                                        }
                                    }
                                });
                            }
                        }
                        Err(e) => {
                            ui.colored_label(
                                colors.red.to_egui(),
                                format!("{} Failed to detect displays: {}", icons::ERR, e),
                            );
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(
                                    "Try: nvidia-settings -q all | grep -i vibrance",
                                )
                                .small()
                                .monospace(),
                            );
                        }
                    }
                }
            }
        });

    ui.add_space(8.0);

    // Quick Presets
    Card::new(&colors)
        .title("Quick Presets")
        .icon(icons::ROCKET)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button("🎨 Default (100%)")
                    .on_hover_text("Reset to default vibrance")
                    .clicked()
                {
                    apply_vibrance_to_all(state, 100);
                }
                if ui
                    .button("🌈 Vibrant (150%)")
                    .on_hover_text("Enhanced colors for gaming")
                    .clicked()
                {
                    apply_vibrance_to_all(state, 150);
                }
                if ui
                    .button("🎮 Max (200%)")
                    .on_hover_text("Maximum saturation")
                    .clicked()
                {
                    apply_vibrance_to_all(state, 200);
                }
                if ui
                    .button("🖤 Muted (50%)")
                    .on_hover_text("Reduced saturation")
                    .clicked()
                {
                    apply_vibrance_to_all(state, 50);
                }
            });
        });

    ui.add_space(8.0);

    // Tips
    Card::new(&colors)
        .title("Tips")
        .icon(icons::BULB)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("• 100% is the default (no change)").small());
            ui.label(
                egui::RichText::new("• 150-175% is recommended for competitive gaming").small(),
            );
            ui.label(egui::RichText::new("• Higher values may cause color banding").small());
            ui.label(egui::RichText::new("• Settings are saved to config automatically").small());
        });
}

/// Apply vibrance to all displays
fn apply_vibrance_to_all(state: &mut GuiState, percentage: u32) {
    // Try native first
    if let Ok(guard) = crate::vibrance_native::get_vibrance_controller() {
        if let Some(controller) = guard.as_ref() {
            for (_device_id, connector_idx, _name, _connected) in controller.list_displays() {
                let _ = crate::vibrance_native::set_display_vibrance_native(
                    0,
                    connector_idx,
                    percentage,
                );
            }
            state.toasts.success(format!("Vibrance set to {}%", percentage));
            return;
        }
    }

    // Fall back to nvibrant
    if let Ok(displays) = vibrance::get_displays() {
        let vibrance_val = vibrance::percentage_to_vibrance(percentage);
        let display_values: Vec<(usize, i32)> = (0..displays.len())
            .map(|i| (i, vibrance_val))
            .collect();
        if let Err(e) = vibrance::set_vibrance(&display_values) {
            state.toasts.error(format!("Failed to set vibrance: {}", e));
        } else {
            state.toasts.success(format!("Vibrance set to {}%", percentage));
        }
    }
}
