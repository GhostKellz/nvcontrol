//! Gamescope Tab
//!
//! Gamescope integration for Wayland gaming, Steam Deck optimization,
//! and FSR/NIS upscaling configuration.

use eframe::egui;

use crate::gamescope::{self, GamescopeConfig, GamescopePreset, GamescopeUpscaling};
use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;

/// Render the Gamescope tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!(
        "{} Gamescope Integration & Steam Deck Optimization",
        icons::ROCKET
    ));
    ui.add_space(4.0);

    // Quick Launch Presets Card
    Card::new(&colors)
        .title("Quick Launch Presets")
        .icon(icons::ROCKET)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button("🎮 Steam Deck (800p)")
                    .on_hover_text("Optimized for Steam Deck display")
                    .clicked()
                {
                    let config = GamescopePreset::SteamDeck.to_config();
                    match gamescope::apply_gamescope_config(&config) {
                        Ok(()) => {
                            state.gamescope_config = Some(config);
                            state.toasts.success("Steam Deck preset applied");
                        }
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }

                if ui
                    .button("📱 Handheld 1080p")
                    .on_hover_text("For handheld PCs with 1080p displays")
                    .clicked()
                {
                    let config = GamescopePreset::Handheld1080p.to_config();
                    match gamescope::apply_gamescope_config(&config) {
                        Ok(()) => {
                            state.gamescope_config = Some(config);
                            state.toasts.success("Handheld 1080p preset applied");
                        }
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .button("🖥️ Desktop Gaming")
                    .on_hover_text("For desktop displays with FSR upscaling")
                    .clicked()
                {
                    let config = GamescopePreset::Desktop.to_config();
                    match gamescope::apply_gamescope_config(&config) {
                        Ok(()) => {
                            state.gamescope_config = Some(config);
                            state.toasts.success("Desktop preset applied");
                        }
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }

                if ui
                    .button("🏆 Performance")
                    .on_hover_text("Maximum FPS, lower quality upscaling")
                    .clicked()
                {
                    let config = GamescopePreset::Performance.to_config();
                    match gamescope::apply_gamescope_config(&config) {
                        Ok(()) => {
                            state.gamescope_config = Some(config);
                            state.toasts.success("Performance preset applied");
                        }
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }

                if ui
                    .button("🎬 Cinematic")
                    .on_hover_text("High quality, capped framerate")
                    .clicked()
                {
                    let config = GamescopePreset::Cinematic.to_config();
                    match gamescope::apply_gamescope_config(&config) {
                        Ok(()) => {
                            state.gamescope_config = Some(config);
                            state.toasts.success("Cinematic preset applied");
                        }
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }
            });
        });

    ui.add_space(8.0);

    // Custom Configuration Card
    Card::new(&colors)
        .title("Custom Gamescope Configuration")
        .icon(icons::SETTINGS)
        .show(ui, |ui| {
            // Get or create default config (4K 240Hz borderless fullscreen)
            let mut config = state
                .gamescope_config
                .clone()
                .unwrap_or_else(|| GamescopeConfig {
                    width: 3840,
                    height: 2160,
                    refresh_rate: Some(240),
                    hdr_enabled: false,
                    adaptive_sync: true,
                    upscaling: GamescopeUpscaling::Fsr,
                    fullscreen: true,
                    borderless: true,
                    nvidia_optimizations: true,
                    ..Default::default()
                });

            // Resolution
            ui.horizontal(|ui| {
                ui.label("Resolution:");
                let mut width = config.width as i32;
                let mut height = config.height as i32;

                ui.add(
                    egui::DragValue::new(&mut width)
                        .range(800..=3840)
                        .prefix("W: "),
                );
                ui.label("×");
                ui.add(
                    egui::DragValue::new(&mut height)
                        .range(600..=2160)
                        .prefix("H: "),
                );

                config.width = width as u32;
                config.height = height as u32;
            });

            // Common resolution presets
            ui.horizontal(|ui| {
                ui.label("     ");
                if ui.small_button("720p").clicked() {
                    config.width = 1280;
                    config.height = 720;
                }
                if ui.small_button("800p").clicked() {
                    config.width = 1280;
                    config.height = 800;
                }
                if ui.small_button("1080p").clicked() {
                    config.width = 1920;
                    config.height = 1080;
                }
                if ui.small_button("1440p").clicked() {
                    config.width = 2560;
                    config.height = 1440;
                }
                if ui.small_button("4K").clicked() {
                    config.width = 3840;
                    config.height = 2160;
                }
            });

            ui.add_space(4.0);

            // Refresh rate
            ui.horizontal(|ui| {
                ui.label("Refresh Rate:");
                let mut refresh = config.refresh_rate.unwrap_or(60) as i32;
                ui.add(
                    egui::DragValue::new(&mut refresh)
                        .range(30..=360)
                        .suffix("Hz"),
                );
                config.refresh_rate = Some(refresh as u32);
            });

            // Refresh rate presets
            ui.horizontal(|ui| {
                ui.label("     ");
                if ui.small_button("60Hz").clicked() {
                    config.refresh_rate = Some(60);
                }
                if ui.small_button("120Hz").clicked() {
                    config.refresh_rate = Some(120);
                }
                if ui.small_button("144Hz").clicked() {
                    config.refresh_rate = Some(144);
                }
                if ui.small_button("165Hz").clicked() {
                    config.refresh_rate = Some(165);
                }
                if ui.small_button("240Hz").clicked() {
                    config.refresh_rate = Some(240);
                }
            });

            ui.add_space(4.0);

            // Upscaling
            ui.horizontal(|ui| {
                ui.label("Upscaling:");
                let upscale_text = match &config.upscaling {
                    GamescopeUpscaling::None => "None",
                    GamescopeUpscaling::Linear => "Linear",
                    GamescopeUpscaling::Nearest => "Nearest",
                    GamescopeUpscaling::Fsr => "FSR",
                    GamescopeUpscaling::Nis => "NIS",
                    GamescopeUpscaling::Integer => "Integer",
                    GamescopeUpscaling::FsrSharpness(_) => "FSR (Custom)",
                    GamescopeUpscaling::Custom(_) => "Custom",
                };

                egui::ComboBox::from_id_salt("upscaling_mode")
                    .selected_text(upscale_text)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(
                                matches!(&config.upscaling, GamescopeUpscaling::None),
                                "None",
                            )
                            .clicked()
                        {
                            config.upscaling = GamescopeUpscaling::None;
                        }
                        if ui
                            .selectable_label(
                                matches!(&config.upscaling, GamescopeUpscaling::Linear),
                                "Linear",
                            )
                            .clicked()
                        {
                            config.upscaling = GamescopeUpscaling::Linear;
                        }
                        if ui
                            .selectable_label(
                                matches!(&config.upscaling, GamescopeUpscaling::Fsr),
                                "FSR (AMD FidelityFX)",
                            )
                            .clicked()
                        {
                            config.upscaling = GamescopeUpscaling::Fsr;
                        }
                        if ui
                            .selectable_label(
                                matches!(&config.upscaling, GamescopeUpscaling::Nis),
                                "NIS (NVIDIA Image Scaling)",
                            )
                            .clicked()
                        {
                            config.upscaling = GamescopeUpscaling::Nis;
                        }
                        if ui
                            .selectable_label(
                                matches!(&config.upscaling, GamescopeUpscaling::Integer),
                                "Integer Scaling",
                            )
                            .clicked()
                        {
                            config.upscaling = GamescopeUpscaling::Integer;
                        }
                    });
            });

            ui.add_space(8.0);

            // Window mode (cleaner than separate checkboxes)
            ui.horizontal(|ui| {
                ui.label("Window Mode:");
                // Determine current mode
                let current_mode = match (config.fullscreen, config.borderless) {
                    (true, true) => "Borderless Fullscreen",
                    (true, false) => "Exclusive Fullscreen",
                    (false, _) => "Windowed",
                };

                egui::ComboBox::from_id_salt("window_mode")
                    .selected_text(current_mode)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(
                                current_mode == "Borderless Fullscreen",
                                "Borderless Fullscreen (Recommended)",
                            )
                            .clicked()
                        {
                            config.fullscreen = true;
                            config.borderless = true;
                        }
                        if ui
                            .selectable_label(
                                current_mode == "Exclusive Fullscreen",
                                "Exclusive Fullscreen",
                            )
                            .clicked()
                        {
                            config.fullscreen = true;
                            config.borderless = false;
                        }
                        if ui
                            .selectable_label(current_mode == "Windowed", "Windowed")
                            .clicked()
                        {
                            config.fullscreen = false;
                            config.borderless = false;
                        }
                    });
            });

            ui.add_space(4.0);

            // Feature toggles
            ui.horizontal(|ui| {
                ui.checkbox(&mut config.hdr_enabled, "HDR");
                ui.checkbox(&mut config.adaptive_sync, "Adaptive Sync");
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut config.nvidia_optimizations, "NVIDIA Optimizations");
                ui.checkbox(&mut config.steam_deck_mode, "Steam Deck Mode");
            });

            ui.add_space(8.0);

            // Apply button
            if ui.button("🚀 Apply Custom Configuration").clicked() {
                match gamescope::apply_gamescope_config(&config) {
                    Ok(()) => {
                        state.gamescope_config = Some(config.clone());
                        state
                            .toasts
                            .success("Custom gamescope configuration applied");
                    }
                    Err(e) => state.toasts.error(format!("Failed: {}", e)),
                }
            }

            // Store updated config
            state.gamescope_config = Some(config);
        });

    ui.add_space(8.0);

    // NVIDIA Optimizations Card
    Card::new(&colors)
        .title("NVIDIA Optimizations for Gamescope")
        .icon(icons::GPU)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new("Environment variables applied when using gamescope:")
                    .small()
                    .color(colors.fg_dark.to_egui()),
            );

            ui.add_space(4.0);

            // These are informational - gamescope sets them automatically when nvidia_optimizations is enabled
            egui::Grid::new("nvidia_env_vars")
                .num_columns(2)
                .spacing([20.0, 4.0])
                .show(ui, |ui| {
                    ui.label("__GL_THREADED_OPTIMIZATIONS=1");
                    ui.label(
                        egui::RichText::new("Enable threaded OpenGL")
                            .small()
                            .color(colors.fg_dark.to_egui()),
                    );
                    ui.end_row();

                    ui.label("__GL_SYNC_TO_VBLANK=0");
                    ui.label(
                        egui::RichText::new("Disable VSync for VRR")
                            .small()
                            .color(colors.fg_dark.to_egui()),
                    );
                    ui.end_row();

                    ui.label("__VK_LAYER_NV_optimus=NVIDIA_only");
                    ui.label(
                        egui::RichText::new("Force NVIDIA GPU (Optimus)")
                            .small()
                            .color(colors.fg_dark.to_egui()),
                    );
                    ui.end_row();

                    ui.label("__GLX_VENDOR_LIBRARY_NAME=nvidia");
                    ui.label(
                        egui::RichText::new("Use NVIDIA GLX")
                            .small()
                            .color(colors.fg_dark.to_egui()),
                    );
                    ui.end_row();
                });
        });

    ui.add_space(8.0);

    // Steam Launch Command Generator
    Card::new(&colors)
        .title("Steam Launch Command Generator")
        .icon(icons::ROCKET)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Copy this to Steam game launch options:").strong());
            ui.add_space(4.0);

            // Generate command from current config
            let config = state.gamescope_config.clone().unwrap_or_default();
            let mut cmd = format!("gamescope -w {} -h {}", config.width, config.height);

            if let Some(rate) = config.refresh_rate {
                cmd.push_str(&format!(" -r {}", rate));
            }

            if config.fullscreen {
                cmd.push_str(" -f");
            }
            if config.borderless {
                cmd.push_str(" -b");
            }
            if config.adaptive_sync {
                cmd.push_str(" --adaptive-sync");
            }

            // Upscaling
            match &config.upscaling {
                GamescopeUpscaling::Fsr => cmd.push_str(" -F fsr"),
                GamescopeUpscaling::Nis => cmd.push_str(" -F nis"),
                GamescopeUpscaling::Integer => cmd.push_str(" -F integer"),
                _ => {}
            }

            if config.hdr_enabled {
                cmd.push_str(" --hdr-enabled");
            }

            cmd.push_str(" -- %command%");

            egui::Frame::none()
                .fill(colors.bg_dark.to_egui())
                .rounding(4.0)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&cmd)
                            .monospace()
                            .color(colors.cyan.to_egui()),
                    );
                });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("📋 Copy Command").clicked() {
                    ui.ctx().copy_text(cmd.clone());
                    state.toasts.success("Command copied to clipboard!");
                }

                if ui.button("📋 Copy (with NVIDIA env vars)").clicked() {
                    let full_cmd = format!(
                        "__GL_THREADED_OPTIMIZATIONS=1 __GL_SHADER_DISK_CACHE=1 __VK_LAYER_NV_optimus=NVIDIA_only {}",
                        cmd
                    );
                    ui.ctx().copy_text(full_cmd);
                    state.toasts.success("Full command with NVIDIA vars copied!");
                }
            });
        });

    ui.add_space(8.0);

    // Tips Card
    Card::new(&colors)
        .title("Gamescope Tips")
        .icon(icons::BULB)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("How to use with Steam:").strong());
            ui.label(egui::RichText::new("1. Right-click game → Properties").small());
            ui.label(
                egui::RichText::new("2. Under 'Launch Options', paste the command above").small(),
            );
            ui.label(
                egui::RichText::new("3. The %command% will be replaced with the game executable")
                    .small(),
            );

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            ui.label(egui::RichText::new("Common flags:").strong());
            ui.label(egui::RichText::new("• -f = Fullscreen").small());
            ui.label(
                egui::RichText::new("• -b = Borderless (use with -f for borderless fullscreen)")
                    .small(),
            );
            ui.label(egui::RichText::new("• -r 144 = Set refresh rate to 144Hz").small());
            ui.label(egui::RichText::new("• -F fsr = Enable AMD FSR upscaling").small());
            ui.label(egui::RichText::new("• --adaptive-sync = Enable VRR/G-Sync").small());
            ui.label(
                egui::RichText::new(
                    "• --hdr-enabled = Enable HDR (requires Wayland + HDR display)",
                )
                .small(),
            );
        });
}
