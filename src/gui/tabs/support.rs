use eframe::egui;

use crate::companion;
use crate::drivers;
use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::tabs::system::DriverInfo;
use crate::gui::widgets::Card;
use std::sync::{Mutex, OnceLock};

fn support_bundle_path_state() -> &'static Mutex<String> {
    static PATH: OnceLock<Mutex<String>> = OnceLock::new();
    PATH.get_or_init(|| Mutex::new(drivers::default_support_bundle_path("gui-support.tar.gz")))
}

pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} Support", icons::BENCHMARK));
    ui.add_space(4.0);

    state.refresh_system_info();
    let driver_info = match state.get_driver_info() {
        Some(info) => info.clone(),
        None => DriverInfo::default(),
    };

    Card::new(&colors)
        .title("Support Status")
        .icon(icons::SYSTEM)
        .show(ui, |ui| {
            let severity_color = match driver_info.diagnostic_severity.as_str() {
                "Healthy" => colors.green.to_egui(),
                "Broken" => colors.red.to_egui(),
                _ => colors.yellow.to_egui(),
            };

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Status:")
                        .strong()
                        .color(colors.comment.to_egui()),
                );
                ui.label(
                    egui::RichText::new(&driver_info.diagnostic_severity)
                        .strong()
                        .color(severity_color),
                );
            });

            ui.add_space(6.0);
            for message in driver_info.diagnostic_messages.iter().take(4) {
                ui.label(egui::RichText::new(format!("• {}", message)).color(colors.fg.to_egui()));
            }

            if !driver_info.suggested_fixes.is_empty() {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Suggested fixes")
                        .strong()
                        .color(colors.cyan.to_egui()),
                );
                for fix in driver_info.suggested_fixes.iter().take(4) {
                    ui.label(egui::RichText::new(format!("→ {}", fix)).color(colors.fg.to_egui()));
                }
            }
        });

    ui.add_space(8.0);

    Card::new(&colors)
        .title("Support Actions")
        .icon(icons::BENCHMARK)
        .show(ui, |ui| {
            let mut path_guard = support_bundle_path_state().lock().unwrap();

            if let Some(ref last_bundle) = driver_info.last_bundle_path {
                ui.label(
                    egui::RichText::new(format!("Last bundle: {}", last_bundle))
                        .small()
                        .color(colors.comment.to_egui()),
                );
                if ui.button("Open Last Bundle").clicked() {
                    match companion::open_path(last_bundle) {
                        Ok(()) => state.toasts.success("Opened last support bundle"),
                        Err(e) => state.toasts.error(format!("Open failed: {}", e)),
                    }
                }
                ui.add_space(6.0);
            }

            ui.label(
                egui::RichText::new("Bundle path")
                    .strong()
                    .color(colors.comment.to_egui()),
            );
            ui.text_edit_singleline(&mut *path_guard);
            ui.label(
                egui::RichText::new("Use a tar.gz path if you want a single shareable archive")
                    .small()
                    .color(colors.comment.to_egui()),
            );
            ui.horizontal(|ui| {
                if ui.button("Use Temp").clicked() {
                    *path_guard = drivers::default_support_bundle_path("gui-support.tar.gz");
                }
                if ui.button("Use Home").clicked() {
                    if let Some(home) = dirs::home_dir() {
                        *path_guard = home
                            .join(".local/state/nvcontrol/support/gui-support.tar.gz")
                            .display()
                            .to_string();
                    }
                }
                if ui
                    .add_enabled(!state.support_job_running, egui::Button::new("Refresh"))
                    .clicked()
                {
                    state.spawn_support_refresh();
                }
            });

            if let Some(status) = &state.support_job_status {
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(status)
                        .small()
                        .color(colors.comment.to_egui()),
                );
                if state.support_job_running {
                    ui.spinner();
                }
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        !state.support_job_running,
                        egui::Button::new("Create Support Bundle"),
                    )
                    .clicked()
                {
                    let path_str = path_guard.clone();
                    state.spawn_support_bundle(path_str);
                }
            });

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button("Copy Support Workflow").clicked() {
                    let cmd = "nvctl driver diagnose-release && nvctl driver check && nvctl driver support-bundle --tarball --redact-paths --redact-ids --log-tail 80 --output ~/.local/state/nvcontrol/support/support.tar.gz";
                    ui.ctx().copy_text(cmd.to_string());
                    state.toasts.success("Support workflow copied");
                }

                if ui.button("Copy Summary").clicked() {
                    let summary = format!(
                        "{}: {}",
                        driver_info.diagnostic_severity,
                        driver_info
                            .diagnostic_messages
                            .first()
                            .cloned()
                            .unwrap_or_else(|| "No diagnostics available".to_string())
                    );
                    ui.ctx().copy_text(summary);
                    state.toasts.success("Diagnostic summary copied");
                }
            });
        });
}
