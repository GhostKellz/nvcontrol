//! Recording Tab
//!
//! NVENC hardware recording with Shadowplay-like features.
//! Supports H.264, H.265, and AV1 encoding with instant replay.

use eframe::egui;

use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;
use crate::recording::{self, EncoderType, OutputFormat, QualityPreset, RecordingSettings};

/// Render the Recording tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!(
        "{} NVENC Recording & Shadowplay-like Features",
        icons::RECORD
    ));
    ui.add_space(4.0);

    // NVENC Capabilities Card
    Card::new(&colors)
        .title("NVENC Capabilities")
        .icon(icons::GPU)
        .show(ui, |ui| match recording::get_nvenc_capabilities() {
            Ok(caps) => {
                ui.horizontal(|ui| {
                    ui.label("GPU:");
                    ui.label(
                        egui::RichText::new(&caps.gpu_name)
                            .strong()
                            .color(colors.cyan.to_egui()),
                    );
                });

                ui.add_space(4.0);

                egui::Grid::new("nvenc_caps_grid")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("H.264 NVENC:");
                        ui.colored_label(
                            if caps.h264_available {
                                colors.green.to_egui()
                            } else {
                                colors.red.to_egui()
                            },
                            if caps.h264_available {
                                "✅ Available"
                            } else {
                                "❌ Not Available"
                            },
                        );
                        ui.end_row();

                        ui.label("H.265 NVENC:");
                        ui.colored_label(
                            if caps.h265_available {
                                colors.green.to_egui()
                            } else {
                                colors.red.to_egui()
                            },
                            if caps.h265_available {
                                "✅ Available"
                            } else {
                                "❌ Not Available"
                            },
                        );
                        ui.end_row();

                        ui.label("AV1 NVENC:");
                        ui.colored_label(
                            if caps.av1_available {
                                colors.green.to_egui()
                            } else {
                                colors.red.to_egui()
                            },
                            if caps.av1_available {
                                "✅ Available (RTX 40+ Series)"
                            } else {
                                "❌ Not Available"
                            },
                        );
                        ui.end_row();

                        ui.label("Max Encoding Sessions:");
                        ui.label(
                            egui::RichText::new(caps.max_encoding_sessions.to_string())
                                .color(colors.yellow.to_egui()),
                        );
                        ui.end_row();
                    });
            }
            Err(e) => {
                ui.colored_label(
                    colors.red.to_egui(),
                    format!("Error detecting NVENC: {}", e),
                );
            }
        });

    ui.add_space(8.0);

    // Quick Recording Presets Card
    Card::new(&colors)
        .title("Quick Recording Presets")
        .icon(icons::ROCKET)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button("🎮 Shadowplay Mode")
                    .on_hover_text("High-quality H.265 recording")
                    .clicked()
                {
                    state.recording_settings = recording::create_shadowplay_preset();
                    state.toasts.success("Shadowplay preset applied");
                }

                if ui
                    .button("💎 AV1 Lossless")
                    .on_hover_text("Ultra-high quality for content creation")
                    .clicked()
                {
                    state.recording_settings = recording::create_lossless_preset();
                    state.toasts.success("AV1 lossless preset applied");
                }
            });

            ui.horizontal(|ui| {
                if ui
                    .button("📺 Streaming Optimized")
                    .on_hover_text("Low-latency H.264 for live streaming")
                    .clicked()
                {
                    state.recording_settings = recording::create_streaming_preset();
                    state.toasts.success("Streaming preset applied");
                }

                if ui
                    .button("🎬 Content Creation")
                    .on_hover_text("1440p AV1 for YouTube/content")
                    .clicked()
                {
                    state.recording_settings = recording::create_content_creation_preset();
                    state.toasts.success("Content creation preset applied");
                }
            });
        });

    ui.add_space(8.0);

    // Recording Controls Card
    Card::new(&colors)
        .title("Recording Controls")
        .icon(icons::RECORD)
        .show(ui, |ui| {
            let is_recording = recording::is_recording();

            ui.horizontal(|ui| {
                let record_btn = if is_recording {
                    egui::Button::new(
                        egui::RichText::new("⏹️ Stop Recording")
                            .strong()
                            .color(egui::Color32::WHITE),
                    )
                    .fill(colors.red.to_egui())
                } else {
                    egui::Button::new(
                        egui::RichText::new("🔴 Start Recording")
                            .strong()
                            .color(egui::Color32::WHITE),
                    )
                    .fill(colors.green.to_egui())
                };

                if ui.add(record_btn).clicked() {
                    if is_recording {
                        match recording::stop_recording() {
                            Ok(_) => state.toasts.success("Recording stopped"),
                            Err(e) => state.toasts.error(format!("Failed to stop: {}", e)),
                        }
                    } else {
                        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                        let output_path = format!("nvcontrol_recording_{}.mp4", timestamp);
                        match recording::start_recording(&state.recording_settings, &output_path) {
                            Ok(_) => state
                                .toasts
                                .success(format!("Recording to {}", output_path)),
                            Err(e) => state.toasts.error(format!("Failed to start: {}", e)),
                        }
                    }
                }

                // Status indicator
                ui.colored_label(
                    if is_recording {
                        colors.red.to_egui()
                    } else {
                        colors.fg_dark.to_egui()
                    },
                    if is_recording {
                        "🔴 Recording..."
                    } else {
                        "⚫ Stopped"
                    },
                );
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            ui.label(
                egui::RichText::new("Instant Replay")
                    .strong()
                    .color(colors.cyan.to_egui()),
            );

            ui.horizontal(|ui| {
                if ui
                    .button("⏪ Start Instant Replay")
                    .on_hover_text("Buffer last 5 minutes continuously")
                    .clicked()
                {
                    match recording::start_instant_replay(&state.recording_settings) {
                        Ok(_) => state
                            .toasts
                            .success("Instant replay started (buffering last 5 min)"),
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }

                if ui
                    .button("💾 Save Last 5 Minutes")
                    .on_hover_text("Save instant replay buffer to file")
                    .clicked()
                {
                    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                    let output_path = format!("instant_replay_{}.mp4", timestamp);
                    match recording::save_instant_replay(&output_path) {
                        Ok(_) => state
                            .toasts
                            .success(format!("Saved replay to {}", output_path)),
                        Err(e) => state.toasts.error(format!("Failed: {}", e)),
                    }
                }
            });
        });

    ui.add_space(8.0);

    // Recording Settings Card
    Card::new(&colors)
        .title("Recording Settings")
        .icon(icons::SETTINGS)
        .show(ui, |ui| {
            // Resolution
            ui.horizontal(|ui| {
                ui.label("Resolution:");
                let resolutions = [
                    ((1920, 1080), "1920×1080 (Full HD)"),
                    ((2560, 1440), "2560×1440 (1440p)"),
                    ((3840, 2160), "3840×2160 (4K)"),
                ];

                let current_res = state.recording_settings.resolution;
                let current_text = resolutions
                    .iter()
                    .find(|(r, _)| *r == current_res)
                    .map(|(_, t)| *t)
                    .unwrap_or("Custom");

                egui::ComboBox::from_id_salt("rec_resolution")
                    .selected_text(current_text)
                    .show_ui(ui, |ui| {
                        for (res, text) in resolutions {
                            if ui
                                .selectable_label(state.recording_settings.resolution == res, text)
                                .clicked()
                            {
                                state.recording_settings.resolution = res;
                            }
                        }
                    });
            });

            // Encoder
            ui.horizontal(|ui| {
                ui.label("Encoder:");
                let encoder_text = match state.recording_settings.encoder {
                    EncoderType::NvencH264 => "H.264 NVENC",
                    EncoderType::NvencH265 => "H.265 NVENC",
                    EncoderType::NvencAv1 => "AV1 NVENC",
                    EncoderType::SoftwareX264 => "x264 (Software)",
                    EncoderType::SoftwareX265 => "x265 (Software)",
                };

                egui::ComboBox::from_id_salt("rec_encoder")
                    .selected_text(encoder_text)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(
                                matches!(state.recording_settings.encoder, EncoderType::NvencH264),
                                "H.264 NVENC (Most Compatible)",
                            )
                            .clicked()
                        {
                            state.recording_settings.encoder = EncoderType::NvencH264;
                        }
                        if ui
                            .selectable_label(
                                matches!(state.recording_settings.encoder, EncoderType::NvencH265),
                                "H.265 NVENC (Better Quality)",
                            )
                            .clicked()
                        {
                            state.recording_settings.encoder = EncoderType::NvencH265;
                        }
                        if ui
                            .selectable_label(
                                matches!(state.recording_settings.encoder, EncoderType::NvencAv1),
                                "AV1 NVENC (Best Quality - RTX 40+)",
                            )
                            .clicked()
                        {
                            state.recording_settings.encoder = EncoderType::NvencAv1;
                        }
                    });
            });

            // Bitrate
            ui.horizontal(|ui| {
                ui.label("Bitrate:");
                let mut bitrate = state.recording_settings.bitrate_mbps as i32;
                if ui
                    .add(
                        egui::DragValue::new(&mut bitrate)
                            .range(5..=200)
                            .suffix(" Mbps"),
                    )
                    .changed()
                {
                    state.recording_settings.bitrate_mbps = bitrate as u32;
                }
            });

            // Framerate
            ui.horizontal(|ui| {
                ui.label("Framerate:");
                let mut framerate = state.recording_settings.framerate as i32;
                if ui
                    .add(
                        egui::DragValue::new(&mut framerate)
                            .range(30..=120)
                            .suffix(" fps"),
                    )
                    .changed()
                {
                    state.recording_settings.framerate = framerate as u32;
                }
            });

            // Toggles
            ui.horizontal(|ui| {
                ui.checkbox(&mut state.recording_settings.audio_enabled, "Include Audio");
                ui.checkbox(&mut state.recording_settings.lossless_mode, "Lossless Mode");
            });
        });

    ui.add_space(8.0);

    // Tips Card
    Card::new(&colors)
        .title("Tips & Information")
        .icon(icons::BULB)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new("For Shadowplay-like Experience:")
                    .strong()
                    .color(colors.cyan.to_egui()),
            );
            ui.label(egui::RichText::new("• Use H.265 NVENC for best quality/size ratio").small());
            ui.label(
                egui::RichText::new("• Enable instant replay for capturing highlights").small(),
            );
            ui.label(egui::RichText::new("• 50 Mbps bitrate provides excellent quality").small());

            ui.add_space(4.0);

            ui.label(
                egui::RichText::new("For Content Creation:")
                    .strong()
                    .color(colors.cyan.to_egui()),
            );
            ui.label(
                egui::RichText::new("• Use AV1 NVENC on RTX 40+ for best compression").small(),
            );
            ui.label(egui::RichText::new("• Record at 1440p or 4K for future-proofing").small());
            ui.label(egui::RichText::new("• Consider lossless mode for editing workflows").small());

            ui.add_space(4.0);

            ui.label(
                egui::RichText::new("Requirements:")
                    .strong()
                    .color(colors.yellow.to_egui()),
            );
            ui.label(egui::RichText::new("• FFmpeg must be installed and in PATH").small());
            ui.label(egui::RichText::new("• NVIDIA GPU with NVENC support").small());
            ui.label(egui::RichText::new("• Sufficient disk space for recordings").small());
        });
}
