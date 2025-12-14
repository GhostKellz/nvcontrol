//! Main GUI Application
//!
//! The NvControlApp struct and eframe::App implementation.

use eframe::egui;

use super::state::GuiState;
use super::tabs::Tab;
use super::theme;
use super::widgets::{HeaderBar, StatusState};

/// Run the modern modular GUI application
pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([800.0, 500.0])
            .with_title("nvcontrol - NVIDIA Settings Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "nvcontrol - NVIDIA Settings Manager",
        options,
        Box::new(|cc| {
            let app = NvControlApp::new();
            app.init(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

/// Main application struct
pub struct NvControlApp {
    /// GUI state
    pub state: GuiState,
    /// Last update timestamp
    last_update: std::time::Instant,
    /// Update interval
    update_interval: std::time::Duration,
}

impl NvControlApp {
    /// Create a new app with default theme
    pub fn new() -> Self {
        Self {
            state: GuiState::new(),
            last_update: std::time::Instant::now(),
            update_interval: std::time::Duration::from_millis(500),
        }
    }

    /// Create a new app with a specific theme
    pub fn with_theme(theme: crate::themes::ThemeVariant) -> Self {
        Self {
            state: GuiState::with_theme(theme),
            last_update: std::time::Instant::now(),
            update_interval: std::time::Duration::from_millis(500),
        }
    }

    /// Initialize app with egui context (apply theme, fonts, etc.)
    pub fn init(&self, ctx: &egui::Context) {
        // Apply the theme from state
        theme::apply_theme(ctx, self.state.current_theme);

        // Setup custom fonts: Phosphor Icons
        let mut fonts = egui::FontDefinitions::default();

        // Add Phosphor icons (regular variant) for clean icons
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

        ctx.set_fonts(fonts);
    }

    /// Handle keyboard shortcuts
    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        // Don't process number key shortcuts if a text field has focus
        let has_focus = ctx.memory(|m| m.focused().is_some());

        ctx.input(|i| {
            // Number keys for tab switching (only when no text input is focused)
            if !has_focus {
                if i.key_pressed(egui::Key::Num1) {
                    self.state.tab = Tab::Gpu;
                }
                if i.key_pressed(egui::Key::Num2) {
                    self.state.tab = Tab::Overclock;
                }
                if i.key_pressed(egui::Key::Num3) {
                    self.state.tab = Tab::Fan;
                }
                if i.key_pressed(egui::Key::Num4) {
                    self.state.tab = Tab::Display;
                }
                if i.key_pressed(egui::Key::Num5) {
                    self.state.tab = Tab::Vibrance;
                }
                if i.key_pressed(egui::Key::Num6) {
                    self.state.tab = Tab::Hdr;
                }
                if i.key_pressed(egui::Key::Num7) {
                    self.state.tab = Tab::GameProfiles;
                }
                if i.key_pressed(egui::Key::Num8) {
                    self.state.tab = Tab::Osd;
                }
                if i.key_pressed(egui::Key::Num9) {
                    self.state.tab = Tab::Settings;
                }
            }

            // Ctrl+T to cycle themes
            if i.modifiers.ctrl && i.key_pressed(egui::Key::T) {
                self.state.cycle_theme();
                theme::apply_theme(ctx, self.state.current_theme);
            }

            // Ctrl+S to save config
            if i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                self.state.save_config();
            }

            // F5 to refresh
            if i.key_pressed(egui::Key::F5) {
                self.state.refresh_gpu_stats();
                ctx.request_repaint();
            }
        });
    }

    /// Render the sidebar navigation
    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        let colors = self.state.theme_colors();

        egui::Frame::none()
            .fill(colors.bg_dark.to_egui())
            .show(ui, |ui| {
                ui.set_min_width(180.0);
                ui.set_max_width(180.0);

                ui.add_space(8.0);

                // Tab buttons
                for (tab, icon, label) in Tab::sidebar_tabs() {
                    let selected = self.state.tab == tab;
                    let response = ui.add(
                        egui::Button::new(
                            egui::RichText::new(format!("{} {}", icon, label)).color(if selected {
                                colors.cyan.to_egui()
                            } else {
                                colors.fg.to_egui()
                            }),
                        )
                        .fill(if selected {
                            colors.selection.to_egui()
                        } else {
                            egui::Color32::TRANSPARENT
                        })
                        .stroke(egui::Stroke::NONE)
                        .min_size(egui::vec2(170.0, 28.0)),
                    );

                    if response.clicked() {
                        self.state.tab = tab;
                    }
                }

                // Version at bottom
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .small()
                            .weak(),
                    );
                });
            });
    }

    /// Render the header bar
    fn render_header(&self, ui: &mut egui::Ui) {
        let colors = self.state.theme_colors();

        let mut header = HeaderBar::new(&colors);

        if let Some(ref stats) = self.state.gpu_stats {
            header = header
                .gpu_name(&stats.name)
                .gpu_temp(stats.temperature)
                .gpu_util(stats.utilization);
        }

        // Add status indicators
        if self
            .state
            .driver_validation
            .as_ref()
            .map_or(false, |d| d.passed)
        {
            header = header.add_status("Driver OK", StatusState::Ok);
        }

        if self.state.asus_power_detector.is_some() {
            let power_state = match &self.state.asus_power_status {
                Some(status) if status.has_warnings => StatusState::Warning,
                Some(_) => StatusState::Ok,
                None => StatusState::Unknown,
            };
            header = header.add_status("Power Monitor+", power_state);
        }

        header.show(ui);
    }
}

impl Default for NvControlApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for NvControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update GPU stats periodically
        if self.last_update.elapsed() >= self.update_interval {
            self.state.refresh_gpu_stats();
            self.state.refresh_asus_power();
            self.last_update = std::time::Instant::now();
        }

        // Handle keyboard shortcuts
        self.handle_keyboard(ctx);

        // Render header
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            self.render_header(ui);
        });

        // Render sidebar
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(180.0)
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        // Render main content
        egui::CentralPanel::default().show(ctx, |ui| match self.state.tab {
            Tab::Gpu => super::tabs::gpu::render(ui, &mut self.state, ctx),
            Tab::Overclock => super::tabs::overclock::render(ui, &mut self.state, ctx),
            Tab::Fan => super::tabs::fan::render(ui, &mut self.state, ctx),
            Tab::Display => super::tabs::display::render(ui, &mut self.state, ctx),
            Tab::Vibrance => super::tabs::vibrance::render(ui, &mut self.state, ctx),
            Tab::Hdr => super::tabs::hdr::render(ui, &mut self.state, ctx),
            Tab::Vrr => super::tabs::vrr::render(ui, &mut self.state, ctx),
            Tab::GameProfiles => super::tabs::game_profiles::render(ui, &mut self.state, ctx),
            Tab::Osd => super::tabs::osd::render(ui, &mut self.state, ctx),
            Tab::Latency => super::tabs::latency::render(ui, &mut self.state, ctx),
            Tab::Gamescope => super::tabs::gamescope::render(ui, &mut self.state, ctx),
            Tab::Recording => super::tabs::recording::render(ui, &mut self.state, ctx),
            Tab::Settings => super::tabs::settings::render(ui, &mut self.state, ctx),
        });

        // Render toasts
        let colors = self.state.theme_colors();
        self.state.toasts.show(ctx, &colors);

        // Request repaint for animations
        ctx.request_repaint();
    }
}
