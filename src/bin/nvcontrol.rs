use eframe::egui;
use nvcontrol::{display, vibrance, fan};

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
}

impl NvControlApp {
    fn new() -> Self {
        let display_count = display::get_display_count();
        Self {
            vibrance_levels: vec![0; display_count],
            tab: Tab::Display,
        }
    }
}

impl eframe::App for NvControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(matches!(self.tab, Tab::Display), "Display & Color").clicked() {
                    self.tab = Tab::Display;
                }
                if ui.selectable_label(matches!(self.tab, Tab::Fan), "Fan Control").clicked() {
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
                            ui.label(format!("Display {}", i));
                            changed |= ui.add(egui::Slider::new(level, -1024..=1023).suffix("%"))
                                .changed();
                        });
                    }
                    if changed {
                        vibrance::set_vibrance(&self.vibrance_levels);
                    }
                    ui.separator();
                    // ICC Profile Management
                    ui.label("ICC Profile Management");
                    let icc_profiles = display::list_icc_profiles();
                    let mut selected_profile = 0;
                    if icc_profiles.is_empty() {
                        ui.label("No ICC profiles found");
                    } else {
                        egui::ComboBox::from_label("ICC Profile")
                            .selected_text(&icc_profiles[selected_profile])
                            .show_ui(ui, |cb_ui| {
                                for (i, profile) in icc_profiles.iter().enumerate() {
                                    cb_ui.selectable_value(&mut selected_profile, i, profile);
                                }
                            });
                        if ui.button("Apply ICC Profile").clicked() {
                            display::load_icc_profile(0, &icc_profiles[selected_profile]);
                            ui.label("Profile applied (stub)");
                        }
                    }
                    if ui.button("Open ICC Folder").clicked() {
                        // Stub: just show a message
                        ui.label("ICC folder: ~/.icc (stub)");
                    }
                    ui.separator();
                    // HDR Toggle
                    static mut HDR_ENABLED: bool = false;
                    let mut hdr_enabled = unsafe { HDR_ENABLED };
                    if ui.checkbox(&mut hdr_enabled, "Enable HDR").changed() {
                        display::toggle_hdr(0);
                        unsafe { HDR_ENABLED = hdr_enabled; }
                        if hdr_enabled {
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
                        vec![fan::FanInfo { id: 0, rpm: Some(1500), percent: Some(40), controllable: false }]
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
