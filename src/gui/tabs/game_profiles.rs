//! Game Profiles Tab
//!
//! Automatic GPU profile application when games are launched.
//! Monitors running processes and applies configured overclock/fan profiles.

use eframe::egui;

use crate::game_profile_auto;
use crate::gui::icons;
use crate::gui::state::GuiState;
use crate::gui::widgets::Card;

/// Render the Game Profiles tab
pub fn render(ui: &mut egui::Ui, state: &mut GuiState, _ctx: &egui::Context) {
    let colors = state.theme_colors();

    ui.heading(format!("{} Game Profile Auto-Application", icons::GAME));
    ui.add_space(4.0);

    ui.label("Automatically apply GPU profiles when games are launched.");
    ui.add_space(8.0);

    // Configuration Card
    Card::new(&colors)
        .title("Configuration")
        .icon(icons::SETTINGS)
        .show(ui, |ui| {
            // Enable/disable toggle
            ui.horizontal(|ui| {
                let was_enabled = state.game_auto_config.enabled;
                ui.checkbox(&mut state.game_auto_config.enabled, "Enable Auto-Application");
                if state.game_auto_config.enabled != was_enabled {
                    // Status changed
                    if state.game_auto_config.enabled {
                        state.toasts.info("Game profile auto-application enabled");
                    } else {
                        state.toasts.info("Game profile auto-application disabled");
                    }
                }
            });

            ui.add_space(8.0);

            // Poll interval
            ui.horizontal(|ui| {
                ui.label("Poll Interval:");
                let mut poll_secs = state.game_auto_config.poll_interval_secs as i32;
                if ui
                    .add(
                        egui::Slider::new(&mut poll_secs, 1..=10)
                            .suffix(" seconds")
                            .text(""),
                    )
                    .changed()
                {
                    state.game_auto_config.poll_interval_secs = poll_secs as u64;
                }
            });
            ui.label(
                egui::RichText::new("How often to check for running games")
                    .small()
                    .weak(),
            );

            ui.add_space(4.0);

            // Apply delay
            ui.horizontal(|ui| {
                ui.label("Apply Delay:");
                let mut delay_secs = state.game_auto_config.apply_delay_secs as i32;
                if ui
                    .add(
                        egui::Slider::new(&mut delay_secs, 0..=10)
                            .suffix(" seconds")
                            .text(""),
                    )
                    .changed()
                {
                    state.game_auto_config.apply_delay_secs = delay_secs as u64;
                }
            });
            ui.label(
                egui::RichText::new("Wait before applying profile (prevents issues if game crashes on start)")
                    .small()
                    .weak(),
            );

            ui.add_space(8.0);

            // Restore on exit
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut state.game_auto_config.restore_on_exit,
                    "Restore default profile on game exit",
                );
            });
            ui.label(
                egui::RichText::new("Returns GPU to stock/default settings when game closes")
                    .small()
                    .weak(),
            );
        });

    ui.add_space(8.0);

    // Actions Card
    Card::new(&colors)
        .title("Actions")
        .icon(icons::ROCKET)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("💾 Save Configuration").clicked() {
                    match game_profile_auto::save_config(&state.game_auto_config) {
                        Ok(_) => {
                            state.toasts.success("Configuration saved successfully");
                        }
                        Err(e) => {
                            state.toasts.error(format!("Failed to save config: {}", e));
                        }
                    }
                }

                if ui.button("🔄 Reset to Defaults").clicked() {
                    state.game_auto_config = game_profile_auto::AutoProfileConfig::default();
                    state.toasts.info("Configuration reset to defaults");
                }
            });
        });

    ui.add_space(8.0);

    // Profile Directory Info Card
    Card::new(&colors)
        .title("Profile Directory")
        .icon(icons::INFO)
        .show(ui, |ui| {
            ui.label("Game profiles are stored in:");

            let config_dir = dirs::config_dir()
                .map(|p| p.join("nvcontrol").join("profiles"))
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "~/.config/nvcontrol/profiles".to_string());

            egui::Frame::none()
                .fill(colors.bg_dark.to_egui())
                .rounding(4.0)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&config_dir)
                            .monospace()
                            .color(colors.cyan.to_egui()),
                    );
                });

            ui.add_space(8.0);

            if ui.button("📂 Open Profiles Folder").clicked() {
                if let Some(config_path) = dirs::config_dir() {
                    let profiles_dir = config_path.join("nvcontrol").join("profiles");
                    // Create directory if it doesn't exist
                    let _ = std::fs::create_dir_all(&profiles_dir);
                    // Try to open with system file manager
                    #[cfg(target_os = "linux")]
                    {
                        let _ = std::process::Command::new("xdg-open")
                            .arg(&profiles_dir)
                            .spawn();
                    }
                    state.toasts.info("Opening profiles folder...");
                }
            }
        });

    ui.add_space(8.0);

    // How It Works Card
    Card::new(&colors)
        .title("How It Works")
        .icon(icons::BULB)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new("1. Create profile files")
                    .strong()
                    .color(colors.cyan.to_egui()),
            );
            ui.label(
                egui::RichText::new("   Create .toml files in the profiles directory with game executable names")
                    .small(),
            );

            ui.add_space(4.0);

            ui.label(
                egui::RichText::new("2. Configure GPU settings")
                    .strong()
                    .color(colors.cyan.to_egui()),
            );
            ui.label(
                egui::RichText::new("   Each profile contains overclock, fan, and power settings")
                    .small(),
            );

            ui.add_space(4.0);

            ui.label(
                egui::RichText::new("3. Launch your game")
                    .strong()
                    .color(colors.cyan.to_egui()),
            );
            ui.label(
                egui::RichText::new("   nvcontrol detects the game and applies the matching profile")
                    .small(),
            );

            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Example profile (cyberpunk2077.toml):")
                    .small()
                    .color(colors.yellow.to_egui()),
            );

            egui::Frame::none()
                .fill(colors.bg_dark.to_egui())
                .rounding(4.0)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(
                            r#"executable = "cyberpunk2077.exe"
gpu_clock_offset = 150
memory_clock_offset = 500
power_limit = 110
fan_speed = 80"#,
                        )
                        .monospace()
                        .small()
                        .color(colors.fg.to_egui()),
                    );
                });
        });
}
