//! DLSS Management Tab
//!
//! Scan games for DLSS DLLs, show versions, and generate Proton launch options.

use eframe::egui;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Instant;

use crate::dlss::{self, DllType, DllVersion, DlssController, DoctorStatus, GameDlssInfo};
use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;

/// Async scan result message
pub enum DlssScanResult {
    Games(Result<Vec<GameDlssInfo>, String>),
    Doctor(Result<dlss::DlssDoctorResult, String>),
}

/// Cached DLSS data to avoid rescanning every frame
pub struct DlssTabState {
    pub games: Vec<GameDlssInfo>,
    pub last_scan: Option<Instant>,
    pub scanning: bool,
    pub doctor_running: bool,
    pub doctor_result: Option<dlss::DlssDoctorResult>,
    pub selected_game: Option<usize>,
    pub show_launch_opts: bool,
    pub include_indicator: bool,

    // Cached controller (GPU capabilities) - doesn't change at runtime
    pub cached_controller: Option<DlssController>,
    pub controller_last_check: Option<Instant>,

    // Async channels for background operations
    scan_tx: Sender<DlssScanResult>,
    scan_rx: Receiver<DlssScanResult>,
}

impl Default for DlssTabState {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            games: Vec::new(),
            last_scan: None,
            scanning: false,
            doctor_running: false,
            doctor_result: None,
            selected_game: None,
            show_launch_opts: false,
            include_indicator: false,
            cached_controller: None,
            controller_last_check: None,
            scan_tx: tx,
            scan_rx: rx,
        }
    }
}

/// Render the DLSS tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    // Poll for async results (non-blocking)
    while let Ok(result) = state.dlss_state.scan_rx.try_recv() {
        match result {
            DlssScanResult::Games(Ok(games)) => {
                let count = games.len();
                state.dlss_state.games = games;
                state.dlss_state.last_scan = Some(Instant::now());
                state.dlss_state.scanning = false;
                state.toasts.success(format!("Found {} DLSS games", count));
            }
            DlssScanResult::Games(Err(e)) => {
                state.dlss_state.scanning = false;
                state.toasts.error(format!("Scan failed: {}", e));
            }
            DlssScanResult::Doctor(Ok(result)) => {
                state.dlss_state.doctor_result = Some(result);
                state.dlss_state.doctor_running = false;
            }
            DlssScanResult::Doctor(Err(e)) => {
                state.dlss_state.doctor_running = false;
                state.toasts.error(format!("Doctor failed: {}", e));
            }
        }
    }

    // Cache controller on first access or refresh every 60s
    let should_refresh_controller = state.dlss_state.cached_controller.is_none()
        || state
            .dlss_state
            .controller_last_check
            .map(|t| t.elapsed().as_secs() > 60)
            .unwrap_or(true);

    if should_refresh_controller {
        state.dlss_state.cached_controller = DlssController::new().ok();
        state.dlss_state.controller_last_check = Some(Instant::now());
    }

    ui.heading(format!("{} DLSS Management", icons::DLSS));
    ui.add_space(4.0);

    // GPU Capabilities Card
    Card::new(&colors)
        .title("GPU DLSS Capabilities")
        .icon(icons::GPU)
        .show(ui, |ui| {
            if let Some(ref controller) = state.dlss_state.cached_controller {
                egui::Grid::new("dlss_caps_grid")
                    .num_columns(2)
                    .spacing([20.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("GPU:");
                        ui.label(&controller.capabilities.gpu_model);
                        ui.end_row();

                        ui.label("Driver:");
                        ui.label(&controller.capabilities.driver_version);
                        ui.end_row();

                        ui.label("DLSS Version:");
                        ui.label(format!("{:?}", controller.version));
                        ui.end_row();

                        ui.label("Super Resolution:");
                        render_capability(ui, &colors, controller.capabilities.supports_dlss);
                        ui.end_row();

                        ui.label("Frame Generation:");
                        render_capability(
                            ui,
                            &colors,
                            controller.capabilities.supports_frame_generation,
                        );
                        ui.end_row();

                        ui.label("Ray Reconstruction:");
                        render_capability(
                            ui,
                            &colors,
                            controller.capabilities.supports_ray_reconstruction,
                        );
                        ui.end_row();

                        ui.label("Multi-Frame Gen:");
                        render_capability(
                            ui,
                            &colors,
                            controller.capabilities.supports_multi_frame_generation,
                        );
                        ui.end_row();
                    });
            } else {
                ui.colored_label(colors.red.to_egui(), "Failed to detect DLSS capabilities");
            }
        });

    ui.add_space(8.0);

    // Doctor Diagnostics Card
    Card::new(&colors)
        .title("DLSS Doctor")
        .icon(icons::INFO)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let doctor_btn = ui.add_enabled(
                    !state.dlss_state.doctor_running,
                    egui::Button::new(if state.dlss_state.doctor_running {
                        "Running..."
                    } else {
                        "Run Diagnostics"
                    }),
                );
                if doctor_btn.clicked() && !state.dlss_state.doctor_running {
                    state.dlss_state.doctor_running = true;
                    let tx = state.dlss_state.scan_tx.clone();
                    std::thread::spawn(move || {
                        let result = DlssController::doctor().map_err(|e| e.to_string());
                        let _ = tx.send(DlssScanResult::Doctor(result));
                    });
                }
            });

            if let Some(ref result) = state.dlss_state.doctor_result {
                ui.add_space(4.0);

                let status_text = match result.status {
                    DoctorStatus::Healthy => ("Healthy", colors.green.to_egui()),
                    DoctorStatus::Warning => ("Warning", colors.yellow.to_egui()),
                    DoctorStatus::Error => ("Error", colors.red.to_egui()),
                };
                ui.colored_label(status_text.1, format!("Status: {}", status_text.0));

                ui.add_space(4.0);

                for check in [
                    &result.gpu_check,
                    &result.driver_check,
                    &result.proton_check,
                ] {
                    let icon = match check.status {
                        DoctorStatus::Healthy => "✅",
                        DoctorStatus::Warning => "⚠️",
                        DoctorStatus::Error => "❌",
                    };
                    ui.label(format!("{} {}: {}", icon, check.name, check.message));
                }

                if !result.recommendations.is_empty() {
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Recommendations:").strong());
                    for rec in &result.recommendations {
                        ui.label(format!("• {}", rec));
                    }
                }
            }
        });

    ui.add_space(8.0);

    // Games List Card
    Card::new(&colors)
        .title("DLSS-Enabled Games")
        .icon(icons::GAME)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let scan_btn = ui.add_enabled(
                    !state.dlss_state.scanning,
                    egui::Button::new(if state.dlss_state.scanning {
                        "Scanning..."
                    } else {
                        "Scan Games"
                    }),
                );
                if scan_btn.clicked() && !state.dlss_state.scanning {
                    state.dlss_state.scanning = true;
                    let tx = state.dlss_state.scan_tx.clone();
                    std::thread::spawn(move || {
                        let result = DlssController::scan_games().map_err(|e| e.to_string());
                        let _ = tx.send(DlssScanResult::Games(result));
                    });
                }

                if state.dlss_state.scanning {
                    ui.spinner();
                }

                if let Some(last) = state.dlss_state.last_scan {
                    let ago = last.elapsed().as_secs();
                    ui.label(format!("Last scan: {}s ago", ago));
                }
            });

            ui.add_space(8.0);

            if state.dlss_state.games.is_empty() {
                ui.label("No games scanned yet. Click 'Scan Games' to find DLSS-enabled games.");
            } else {
                let latest = DllVersion::new(310, 5, 0, 0);

                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for (idx, game) in state.dlss_state.games.iter().enumerate() {
                            let is_selected = state.dlss_state.selected_game == Some(idx);

                            let frame = egui::Frame::none()
                                .fill(if is_selected {
                                    colors.bg_highlight.to_egui()
                                } else {
                                    egui::Color32::TRANSPARENT
                                })
                                .inner_margin(4.0)
                                .rounding(4.0);

                            frame.show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Game name and launcher
                                    let launcher = game.launcher.display_name();
                                    if ui
                                        .selectable_label(
                                            is_selected,
                                            format!("{} ({})", game.game_name, launcher),
                                        )
                                        .clicked()
                                    {
                                        state.dlss_state.selected_game = Some(idx);
                                    }

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            // Show DLSS versions
                                            for dll in &game.dlls {
                                                if dll.dll_type == DllType::SuperResolution {
                                                    let version =
                                                        dll.version.as_deref().unwrap_or("?");
                                                    let needs_update = dll
                                                        .parsed_version
                                                        .as_ref()
                                                        .map(|v| v < &latest)
                                                        .unwrap_or(true);

                                                    let color = if needs_update {
                                                        colors.yellow.to_egui()
                                                    } else {
                                                        colors.green.to_egui()
                                                    };

                                                    ui.colored_label(
                                                        color,
                                                        format!("v{}", version),
                                                    );
                                                }
                                            }
                                        },
                                    );
                                });
                            });
                        }
                    });
            }
        });

    // Selected Game Details
    if let Some(idx) = state.dlss_state.selected_game {
        if let Some(game) = state.dlss_state.games.get(idx) {
            ui.add_space(8.0);

            Card::new(&colors)
                .title(&game.game_name)
                .icon(icons::INFO)
                .show(ui, |ui| {
                    egui::Grid::new("game_details_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Launcher:");
                            ui.label(game.launcher.display_name());
                            ui.end_row();

                            if let Some(ref app_id) = game.app_id {
                                ui.label("App ID:");
                                ui.label(app_id);
                                ui.end_row();
                            }

                            ui.label("Proton/Wine:");
                            ui.label(if game.is_proton { "Yes" } else { "No" });
                            ui.end_row();
                        });

                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("DLSS DLLs:").strong());

                    for dll in &game.dlls {
                        let version = dll.version.as_deref().unwrap_or("Unknown");
                        let size_mb = dll.file_size / (1024 * 1024);
                        let transformer = if dll.is_transformer_model {
                            " [Transformer]"
                        } else {
                            ""
                        };

                        ui.label(format!(
                            "• {} v{} ({}MB){}",
                            dll.dll_type.display_name(),
                            version,
                            size_mb,
                            transformer
                        ));
                    }

                    if game.is_proton {
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(4.0);

                        ui.label(egui::RichText::new("Steam Launch Options:").strong());

                        ui.horizontal(|ui| {
                            ui.checkbox(
                                &mut state.dlss_state.include_indicator,
                                "Include version indicator",
                            );
                        });

                        let opts = if state.dlss_state.include_indicator {
                            dlss::ProtonLaunchOptions::with_indicator()
                        } else {
                            dlss::ProtonLaunchOptions::default_upgrade()
                        };

                        let launch_str = opts.to_steam_launch_options();

                        ui.horizontal(|ui| {
                            ui.monospace(&launch_str);
                            if ui.button("Copy").clicked() {
                                ui.output_mut(|o| o.copied_text = launch_str.clone());
                                state.toasts.success("Copied to clipboard");
                            }
                        });
                    }
                });
        }
    }

    ui.add_space(8.0);

    // Proton Tips Card
    Card::new(&colors)
        .title("Proton DLSS Tips")
        .icon(icons::BULB)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Environment Variables:").strong());
            ui.label("• PROTON_DLSS_UPGRADE=1 - Force latest DLSS version");
            ui.label("• PROTON_DLSS_INDICATOR=1 - Show version overlay in-game");
            ui.label("• PROTON_DLSS_UPGRADE=310.5.0 - Use specific version");
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Recommended Proton:").strong());
            ui.label("• Proton-GE or Proton-CachyOS for best DLSS support");
            ui.label("• Stock Proton 8+ also works for most games");
        });
}

fn render_capability(ui: &mut egui::Ui, colors: &crate::themes::ColorPalette, supported: bool) {
    if supported {
        ui.colored_label(colors.green.to_egui(), "✅ Supported");
    } else {
        ui.colored_label(colors.comment.to_egui(), "❌ Not Available");
    }
}
