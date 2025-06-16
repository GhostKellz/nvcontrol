#[cfg(feature = "gui")]
use eframe::egui;
#[cfg(feature = "gui")]
use nvcontrol::{display, fan, vibrance, config};

#[cfg(feature = "gui")]
enum Tab {
    Display,
    Fan,
}

#[cfg(feature = "gui")]
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "nvcontrol",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(NvControlApp::new())),
    )
}

#[cfg(not(feature = "gui"))]
fn main() {
    eprintln!("GUI feature not enabled. Build with --features gui to enable the GUI.");
    std::process::exit(1);
}

#[cfg(feature = "gui")]
struct NvControlApp {
    vibrance_levels: Vec<i16>,
    tab: Tab,
    config: config::Config,
    hdr_enabled: bool,
    selected_icc_profile_idx: usize,
}

#[cfg(feature = "gui")]
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

#[cfg(feature = "gui")]
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
                            
                            // Convert from -1024..1023 range to 0..100 percentage
                            let mut percentage = ((*level + 1024) as f32 / 2047.0 * 100.0) as u32;
                            
                            if ui.add(egui::Slider::new(&mut percentage, 0..=100).suffix("%")).changed() {
                                // Convert back to -1024..1023 range
                                *level = ((percentage as f32 / 100.0 * 2047.0) - 1024.0) as i16;
                                changed = true;
                            }
                            
                            // Show raw value for advanced users
                            ui.label(format!("({level})"));
                            
                            // Show raw value for advanced users
                            ui.label(format!("({level})"));
                            
                            // Quick preset buttons
                            if ui.small_button("Off").clicked() {
                                *level = 0;
                                changed = true;
                            }
                            if ui.small_button("50%").clicked() {
                                *level = 512;
                                changed = true;
                            }
                            if ui.small_button("Max").clicked() {
                                *level = 1023;
                                changed = true;
                            }
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
                            match display::load_icc_profile(0, &icc_profiles[self.selected_icc_profile_idx]) {
                                Ok(()) => {
                                    self.config.selected_icc_profile = icc_profiles[self.selected_icc_profile_idx].clone();
                                    self.config.save();
                                    ui.label("✅ Profile applied successfully");
                                }
                                Err(e) => {
                                    ui.label(format!("❌ Profile error: {e}"));
                                }
                            }
                        }
                    }
                    if ui.button("Open ICC Folder").clicked() {
                        match display::open_icc_folder() {
                            Ok(()) => ui.label("✅ Opened ICC folder"),
                            Err(e) => ui.label(format!("❌ Error: {e}")),
                        };
                    }
                    ui.separator();
                    // Display HDR capabilities
                    ui.label("HDR Status:");
                    let displays = display::list_displays();
                    for display in displays {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}: {}", display.name, display.kind));
                            if display.hdr_capable {
                                let status = if display.hdr_enabled { "✅ HDR ON" } else { "⚫ HDR OFF" };
                                ui.label(status);
                                ui.label(format!("{}bit", display.color_depth));
                            } else {
                                ui.label("❌ No HDR");
                            }
                        });
                    }
                    ui.separator();
                    // HDR Toggle
                    if ui.checkbox(&mut self.hdr_enabled, "Enable HDR").changed() {
                        match display::toggle_hdr(0) {
                            Ok(new_status) => {
                                self.hdr_enabled = new_status;
                                self.config.hdr_enabled = self.hdr_enabled;
                                self.config.save();
                                if self.hdr_enabled {
                                    ui.label("HDR Enabled");
                                } else {
                                    ui.label("HDR Disabled");
                                }
                            }
                            Err(e) => {
                                ui.label(format!("HDR Error: {e}"));
                                // Revert checkbox state on error
                                self.hdr_enabled = !self.hdr_enabled;
                            }
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
