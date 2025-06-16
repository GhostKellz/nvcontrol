use eframe::egui;
use nvcontrol::{display, fan, vibrance, config};

enum Tab {
    Display,
    Fan,
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "nvcontrol",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(NvControlApp::new())),
    )
}

struct NvControlApp {
    vibrance_levels: Vec<i16>,
    tab: Tab,
    config: config::Config,
    hdr_enabled: bool,
    selected_icc_profile_idx: usize,
}

impl NvControlApp {
    fn new() -> Self {
        let config = config::Config::load();
        let display_count = display::get_display_count();
        let vibrance_levels = if config.vibrance_levels.len() == display_count {
            config.vibrance_levels.clone()
        } else {
            vec![0; display_count]
        };
        
        Self {
            vibrance_levels,
            tab: Tab::Display,
            hdr_enabled: config.hdr_enabled,
            selected_icc_profile_idx: 0,
            config,
        }
    }
}

impl eframe::App for NvControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(matches!(self.tab, Tab::Display), "Display & Color")
                    .clicked()
                {
                    self.tab = Tab::Display;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Fan), "Fan Control")
                    .clicked()
                {
                    self.tab = Tab::Fan;
                }
            });
        });
        match self.tab {
            Tab::Display => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Display & Color");
                    ui.label("Digital Vibrance (per display):");
                    let mut changed = false;
                    for (i, level) in self.vibrance_levels.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Display {i}"));
                            changed |= ui
                                .add(egui::Slider::new(level, -1024..=1023).suffix("%"))
                                .changed();
                        });
                    }
                    if changed {
                        let _ = vibrance::set_vibrance(&self.vibrance_levels);
                        self.config.vibrance_levels = self.vibrance_levels.clone();
                        self.config.save();
                    }
                    ui.separator();
                    // ICC Profile Management
                    ui.label("ICC Profile Management");
                    let icc_profiles = display::list_icc_profiles();
                    if icc_profiles.is_empty() {
                        ui.label("No ICC profiles found");
                    } else {
                        egui::ComboBox::from_label("ICC Profile")
                            .selected_text(&icc_profiles[self.selected_icc_profile_idx])
                            .show_ui(ui, |cb_ui| {
                                for (i, profile) in icc_profiles.iter().enumerate() {
                                    cb_ui.selectable_value(&mut self.selected_icc_profile_idx, i, profile);
                                }
                            });
                        if ui.button("Apply ICC Profile").clicked() {
                            display::load_icc_profile(0, &icc_profiles[self.selected_icc_profile_idx]);
                            self.config.selected_icc_profile = icc_profiles[self.selected_icc_profile_idx].clone();
                            self.config.save();
                            ui.label("Profile applied (stub)");
                        }
                    }
                    if ui.button("Open ICC Folder").clicked() {
                        // Stub: just show a message
                        ui.label("ICC folder: ~/.icc (stub)");
                    }
                    ui.separator();
                    // HDR Toggle
                    if ui.checkbox(&mut self.hdr_enabled, "Enable HDR").changed() {
                        display::toggle_hdr(0);
                        self.config.hdr_enabled = self.hdr_enabled;
                        self.config.save();
                        if self.hdr_enabled {
                            ui.label("HDR Enabled (stub)");
                        } else {
                            ui.label("HDR Disabled (stub)");
                        }
                    }
                });
            }
            Tab::Fan => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Fan Control");
                    let fans = fan::list_fans();
                    let fans = if fans.is_empty() {
                        vec![fan::FanInfo {
                            id: 0,
                            rpm: Some(1500),
                            percent: Some(40),
                            controllable: false,
                        }]
                    } else {
                        fans
                    };
                    for fan in fans.iter() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Fan {}", fan.id));
                            ui.label(format!("{} RPM", fan.rpm.unwrap_or(0)));
                            ui.label(format!("{}%", fan.percent.unwrap_or(0)));
                            let mut speed = fan.percent.unwrap_or(0) as u32;
                            let slider = egui::Slider::new(&mut speed, 0..=100).text("Set %");
                            let changed = ui.add_enabled(fan.controllable, slider).changed();
                            if changed {
                                fan::set_fan_speed(fan.id, speed as u8);
                                ui.label("Fan control not supported on this device.");
                            }
                        });
                        ui.separator();
                    }
                });
            }
        }
    }
}
