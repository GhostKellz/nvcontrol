#[cfg(feature = "gui")]
use eframe::egui;
#[cfg(feature = "gui")]
use nvcontrol::{config, displ                if ui
                    .selectable_label(matches!(self.tab, Tab::Display), "🖥️ Display")
                    .clicked()
                {
                    self.tab = Tab::Display;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Vibrance), "🌈 Vibrance")
                    .clicked()
                {
                    self.tab = Tab::Vibrance;
                }n, gpu, vibrance, theme, overclocking};
#[cfg(feature = "gui")]
use nvml_wrapper::Nvml;

#[cfg(feature = "gui")]
enum Tab {
    Gpu,
    Display,
    Vibrance,
    Overclock,
    Fan,
    Settings,
}

#[cfg(feature = "gui")]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([800.0, 500.0])
            .with_icon(
                // Load icon from bytes if available
                eframe::icon_data::from_png_bytes(&[]).unwrap_or_default(),
            ),
        ..Default::default()
    };
    
    eframe::run_native(
        "nvcontrol - NVIDIA Settings Manager",
        options,
        Box::new(|cc| {
            // Apply modern theme
            let theme = theme::ModernTheme::nvidia_dark();
            cc.egui_ctx.set_visuals(theme.to_egui_visuals());
            
            // Enable better fonts and styling
            cc.egui_ctx.set_pixels_per_point(1.2);
            
            Box::new(NvControlApp::new())
        }),
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
    theme: theme::ModernTheme,
    overclock_profile: overclocking::OverclockProfile,
    // Missing fields that are used in the update() method
    fan_speeds: std::collections::HashMap<usize, u8>,
    gpu_stats: Option<GpuStats>,
}

#[cfg(feature = "gui")]
#[derive(Debug, Clone)]
struct GpuStats {
    pub name: String,
    pub temperature: f32,
    pub utilization: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub power_draw: f32,
    pub fan_speed: u32,
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
            tab: Tab::Gpu,
            hdr_enabled: config.hdr_enabled,
            selected_icc_profile_idx: 0,
            config,
            theme: theme::ModernTheme::nvidia_dark(),
            overclock_profile: overclocking::OverclockProfile::default(),
            fan_speeds: std::collections::HashMap::new(),
            gpu_stats: None,
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for NvControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(matches!(self.tab, Tab::Gpu), "🎮 GPU")
                    .clicked()
                {
                    self.tab = Tab::Gpu;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Display), "🖥️ Display & Color")
                    .clicked()
                {
                    self.tab = Tab::Display;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Overclock), "⚡ Overclock")
                    .clicked()
                {
                    self.tab = Tab::Overclock;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Fan), "🌀 Fan Control")
                    .clicked()
                {
                    self.tab = Tab::Fan;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Settings), "⚙️ Settings")
                    .clicked()
                {
                    self.tab = Tab::Settings;
                }
            });
        });
        match self.tab {
            Tab::Gpu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🎮 GPU Status & Monitoring");
                    
                    // Get GPU info via NVML or fallback
                    ui.group(|ui| {
                        ui.label("📊 Real-time Stats");
                        ui.separator();
                        
                        // Try to get live GPU stats
                        if let Ok(nvml) = nvml_wrapper::Nvml::init() {
                            if let Ok(device) = nvml.device_by_index(0) {
                                let name = device.name().unwrap_or("Unknown GPU".to_string());
                                let temp = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).unwrap_or(0);
                                let power = device.power_usage().map(|p| p as f64 / 1000.0).unwrap_or(0.0);
                                let util = device.utilization_rates().map(|u| u.gpu).unwrap_or(0);
                                let mem_info = device.memory_info().ok();
                                
                                ui.horizontal(|ui| {
                                    ui.label("🎯 GPU:");
                                    ui.label(&name);
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label("🌡️ Temperature:");
                                    ui.colored_label(
                                        if temp > 80 { egui::Color32::RED } 
                                        else if temp > 70 { egui::Color32::YELLOW }
                                        else { egui::Color32::GREEN },
                                        format!("{}°C", temp)
                                    );
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label("⚡ Power Usage:");
                                    ui.label(format!("{:.1}W", power));
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label("📈 GPU Usage:");
                                    ui.add(egui::ProgressBar::new(util as f32 / 100.0).text(format!("{}%", util)));
                                });
                                
                                if let Some(mem) = mem_info {
                                    ui.horizontal(|ui| {
                                        ui.label("💾 VRAM:");
                                        let used_gb = mem.used as f64 / 1e9;
                                        let total_gb = mem.total as f64 / 1e9;
                                        let usage_ratio = mem.used as f32 / mem.total as f32;
                                        ui.add(egui::ProgressBar::new(usage_ratio).text(format!("{:.1}/{:.1} GB", used_gb, total_gb)));
                                    });
                                }
                            } else {
                                ui.label("❌ No NVIDIA GPU detected");
                            }
                        } else {
                            ui.label("⚠️ NVML not available - install NVIDIA drivers");
                            ui.horizontal(|ui| {
                                ui.label("🎯 GPU:");
                                ui.label("Unknown (fallback mode)");
                            });
                        }
                    });
                    
                    ui.separator();
                    
                    // Quick actions
                    ui.group(|ui| {
                        ui.label("🚀 Quick Actions");
                        ui.separator();
                        
                        ui.horizontal(|ui| {
                            if ui.button("📊 Open Live Monitor (TUI)").clicked() {
                                // Launch the TUI monitor in a new terminal
                                std::thread::spawn(|| {
                                    let _ = std::process::Command::new("x-terminal-emulator")
                                        .args(["-e", "nvctl", "gpu", "stat"])
                                        .spawn();
                                });
                            }
                            
                            if ui.button("🔧 Show Capabilities").clicked() {
                                // Could open a popup or navigate to settings
                            }
                        });
                    });
                });
            }
            Tab::Vibrance => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🌈 Digital Vibrance Control");
                    
                    // nvibrant status
                    ui.group(|ui| {
                        ui.label("📋 nvibrant Status");
                        ui.separator();
                        
                        if vibrance::is_available() {
                            ui.colored_label(egui::Color32::from_rgb(16, 185, 129), "✅ nvibrant Available");
                            
                            match vibrance::get_driver_info() {
                                Ok(info) => ui.label(info),
                                Err(e) => ui.colored_label(egui::Color32::from_rgb(239, 68, 68), format!("❌ {}", e)),
                            };
                        } else {
                            ui.colored_label(egui::Color32::from_rgb(239, 68, 68), "❌ nvibrant Not Available");
                            ui.label("Digital vibrance requires nvibrant for Wayland support");
                            
                            if ui.button("📥 Install nvibrant").clicked() {
                                if let Err(e) = std::process::Command::new("pip3")
                                    .args(&["install", "nvibrant"])
                                    .spawn() 
                                {
                                    eprintln!("Failed to install nvibrant: {}", e);
                                }
                            }
                        }
                    });
                    
                    ui.separator();
                    
                    // Per-display vibrance control
                    ui.group(|ui| {
                        ui.label("🖥️ Per-Display Vibrance Control");
                        ui.separator();
                        
                        match vibrance::get_displays() {
                            Ok(displays) => {
                                let mut changed = false;
                                
                                for (i, display) in displays.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("Display {}: {}", i, display));
                                        
                                        // Get current vibrance
                                        let current_vibrance = vibrance::get_display_vibrance(i).unwrap_or(0);
                                        let mut percentage = vibrance::vibrance_to_percentage(current_vibrance) as i32;
                                        
                                        if ui.add(egui::Slider::new(&mut percentage, 0..=200)
                                            .suffix("%")
                                            .text("Vibrance"))
                                            .changed() 
                                        {
                                            let vibrance_val = vibrance::percentage_to_vibrance(percentage as u32);
                                            let display_values = vec![(i, vibrance_val)];
                                            if let Err(e) = vibrance::set_vibrance(&display_values) {
                                                eprintln!("Failed to set vibrance: {}", e);
                                            } else {
                                                changed = true;
                                            }
                                        }
                                    });
                                }
                                
                                if changed {
                                    // Update config
                                    // TODO: Save vibrance settings to config
                                }
                            }
                            Err(e) => {
                                ui.colored_label(egui::Color32::from_rgb(239, 68, 68), format!("❌ Failed to detect displays: {}", e));
                            }
                        }
                    });
                    
                    ui.separator();
                    
                    // Quick presets
                    ui.group(|ui| {
                        ui.label("🎨 Quick Presets");
                        ui.separator();
                        
                        ui.horizontal(|ui| {
                            if ui.button("🎮 Gaming (150%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(vibrance::percentage_to_vibrance(150)) {
                                    eprintln!("Failed to set gaming preset: {}", e);
                                }
                            }
                            
                            if ui.button("🎨 Content Creation (120%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(vibrance::percentage_to_vibrance(120)) {
                                    eprintln!("Failed to set content creation preset: {}", e);
                                }
                            }
                            
                            if ui.button("🔄 Default (100%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(0) {
                                    eprintln!("Failed to reset vibrance: {}", e);
                                }
                            }
                            
                            if ui.button("🌑 Grayscale (0%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(vibrance::percentage_to_vibrance(0)) {
                                    eprintln!("Failed to set grayscale: {}", e);
                                }
                            }
                        });
                        
                        ui.horizontal(|ui| {
                            if ui.button("🎯 Max Vibrance (200%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(vibrance::percentage_to_vibrance(200)) {
                                    eprintln!("Failed to set max vibrance: {}", e);
                                }
                            }
                            
                            if ui.button("📺 Movie Mode (110%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(vibrance::percentage_to_vibrance(110)) {
                                    eprintln!("Failed to set movie mode: {}", e);
                                }
                            }
                        });
                    });
                    
                    ui.separator();
                    
                    // Advanced settings
                    ui.group(|ui| {
                        ui.label("⚙️ Advanced Settings");
                        ui.separator();
                        
                        ui.horizontal(|ui| {
                            if ui.button("📋 List Displays").clicked() {
                                match vibrance::get_displays() {
                                    Ok(displays) => {
                                        for (i, display) in displays.iter().enumerate() {
                                            println!("Display {}: {}", i, display);
                                        }
                                    }
                                    Err(e) => eprintln!("Failed to list displays: {}", e),
                                }
                            }
                            
                            if ui.button("🔍 Driver Info").clicked() {
                                match vibrance::get_driver_info() {
                                    Ok(info) => println!("Driver Info: {}", info),
                                    Err(e) => eprintln!("Failed to get driver info: {}", e),
                                }
                            }
                        });
                        
                        ui.label("💡 Tip: Changes apply immediately and work on Wayland!");
                        ui.label("🎯 Use Gaming preset for enhanced colors in games");
                        ui.label("🎨 Use Content Creation for color-accurate work");
                    });
                });
            } }
                    ui.label("Digital Vibrance (per display):");
                    let mut changed = false;
                    for (i, level) in self.vibrance_levels.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Display {i}"));

                            // Convert from -1024..1023 range to 0..100 percentage
                            let mut percentage = ((*level + 1024) as f32 / 2047.0 * 100.0) as u32;

                            if ui
                                .add(egui::Slider::new(&mut percentage, 0..=100).suffix("%"))
                                .changed()
                            {
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
                        let display_values: Vec<(usize, i32)> = self.vibrance_levels
                            .iter()
                            .enumerate()
                            .map(|(idx, &level)| (idx, level as i32))
                            .collect();
                        let _ = vibrance::set_vibrance(&display_values);
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
                                    cb_ui.selectable_value(
                                        &mut self.selected_icc_profile_idx,
                                        i,
                                        profile,
                                    );
                                }
                            });
                        if ui.button("Apply ICC Profile").clicked() {
                            match display::load_icc_profile(
                                0,
                                &icc_profiles[self.selected_icc_profile_idx],
                            ) {
                                Ok(()) => {
                                    self.config.selected_icc_profile =
                                        icc_profiles[self.selected_icc_profile_idx].clone();
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
                                let status = if display.hdr_enabled {
                                    "✅ HDR ON"
                                } else {
                                    "⚫ HDR OFF"
                                };
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
            Tab::Overclock => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("⚡ GPU Overclocking");
                    
                    ui.group(|ui| {
                        ui.label("🎯 Current Profile");
                        ui.separator();
                        
                        ui.horizontal(|ui| {
                            ui.label("Profile:");
                            ui.text_edit_singleline(&mut self.overclock_profile.name);
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("GPU Clock Offset:");
                            ui.add(egui::Slider::new(&mut self.overclock_profile.gpu_clock_offset, -200..=300).suffix(" MHz"));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Memory Clock Offset:");
                            ui.add(egui::Slider::new(&mut self.overclock_profile.memory_clock_offset, -500..=1000).suffix(" MHz"));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Power Limit:");
                            ui.add(egui::Slider::new(&mut self.overclock_profile.power_limit, 50..=120).suffix("%"));
                        });
                        
                        if ui.button("Apply Overclock").clicked() {
                            match overclocking::apply_overclock_profile(&self.overclock_profile) {
                                Ok(()) => println!("✅ Overclock applied"),
                                Err(e) => eprintln!("❌ Overclock failed: {}", e),
                            }
                        }
                        
                        ui.horizontal(|ui| {
                            if ui.button("Reset to Default").clicked() {
                                self.overclock_profile = overclocking::OverclockProfile::default();
                            }
                            
                            if ui.button("⚠️ Stress Test").clicked() {
                                let _ = overclocking::create_stress_test(5);
                            }
                        });
                    });
                });
            }
            Tab::Fan => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🌀 Fan Control");
                    
                    ui.group(|ui| {
                        ui.label("🌀 Fan Status");
                        ui.separator();
                        
                        let fans = fan::list_fans();
                        for fan in fans {
                            ui.horizontal(|ui| {
                                ui.label(format!("Fan {}:", fan.id));
                                if let Some(rpm) = fan.rpm {
                                    ui.label(format!("{} RPM", rpm));
                                }
                                if let Some(percent) = fan.percent {
                                    ui.label(format!("{}%", percent));
                                }
                                
                                if fan.controllable {
                                    let current_speed = self.fan_speeds.get(&fan.id).copied().unwrap_or(50);
                                    let mut new_speed = current_speed;
                                    if ui.add(egui::Slider::new(&mut new_speed, 0..=100).suffix("%")).changed() {
                                        self.fan_speeds.insert(fan.id, new_speed);
                                        if let Err(e) = fan::set_fan_speed(fan.id, new_speed) {
                                            eprintln!("Failed to set fan speed: {}", e);
                                        }
                                    }
                                } else {
                                    ui.colored_label(egui::Color32::from_rgb(156, 163, 175), "Read-only");
                                }
                            });
                        }
                    });
                });
            }
            Tab::Settings => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("⚙️ Settings");
                    
                    ui.group(|ui| {
                        ui.label("🎨 Theme Selection");
                        ui.separator();
                        
                        ui.horizontal(|ui| {
                            if ui.button("NVIDIA Dark").clicked() {
                                self.theme = theme::ModernTheme::nvidia_dark();
                                ctx.set_visuals(self.theme.to_egui_visuals());
                            }
                            if ui.button("NVIDIA Light").clicked() {
                                self.theme = theme::ModernTheme::nvidia_light();
                                ctx.set_visuals(self.theme.to_egui_visuals());
                            }
                            if ui.button("Gaming").clicked() {
                                self.theme = theme::ModernTheme::gaming();
                                ctx.set_visuals(self.theme.to_egui_visuals());
                            }
                        });
                    });
                    
                    ui.separator();
                    
                    ui.group(|ui| {
                        ui.label("📊 System Information");
                        ui.separator();
                        
                        if ui.button("🔧 Initialize nvibrant").clicked() {
                            match vibrance::initialize_nvibrant() {
                                Ok(()) => println!("✅ nvibrant initialized"),
                                Err(e) => eprintln!("❌ nvibrant init failed: {}", e),
                            }
                        }
                        
                        if ui.button("🧪 Test nvibrant").clicked() {
                            match vibrance::test_nvibrant() {
                                Ok(()) => println!("✅ nvibrant test passed"),
                                Err(e) => eprintln!("❌ nvibrant test failed: {}", e),
                            }
                        }
                    });
                });
            }
        }
    }
}
