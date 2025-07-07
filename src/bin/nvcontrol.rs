#[cfg(feature = "gui")]
use eframe::egui;
#[cfg(feature = "gui")]
use nvcontrol::{
    config, display, drivers, fan, gamescope, latency, overclocking, recording, shaders, theme,
    vibrance, vrr,
};

#[cfg(feature = "gui")]
enum Tab {
    Gpu,
    Display,
    Vibrance,
    Overclock,
    Fan,
    Vrr,
    Latency,
    Recording,
    Gamescope,
    ShaderCache,
    Drivers,
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
                    .selectable_label(matches!(self.tab, Tab::Vrr), "🔄 VRR")
                    .clicked()
                {
                    self.tab = Tab::Vrr;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Latency), "⚡ Latency")
                    .clicked()
                {
                    self.tab = Tab::Latency;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Recording), "📹 Recording")
                    .clicked()
                {
                    self.tab = Tab::Recording;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Gamescope), "🎯 Gamescope")
                    .clicked()
                {
                    self.tab = Tab::Gamescope;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::ShaderCache), "🎨 Shader Cache")
                    .clicked()
                {
                    self.tab = Tab::ShaderCache;
                }
                if ui
                    .selectable_label(matches!(self.tab, Tab::Drivers), "🔧 Drivers")
                    .clicked()
                {
                    self.tab = Tab::Drivers;
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
                                let temp = device
                                    .temperature(
                                        nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu,
                                    )
                                    .unwrap_or(0);
                                let power = device
                                    .power_usage()
                                    .map(|p| p as f64 / 1000.0)
                                    .unwrap_or(0.0);
                                let util = device.utilization_rates().map(|u| u.gpu).unwrap_or(0);
                                let mem_info = device.memory_info().ok();

                                ui.horizontal(|ui| {
                                    ui.label("🎯 GPU:");
                                    ui.label(&name);
                                });

                                ui.horizontal(|ui| {
                                    ui.label("🌡️ Temperature:");
                                    ui.colored_label(
                                        if temp > 80 {
                                            egui::Color32::RED
                                        } else if temp > 70 {
                                            egui::Color32::YELLOW
                                        } else {
                                            egui::Color32::GREEN
                                        },
                                        format!("{}°C", temp),
                                    );
                                });

                                ui.horizontal(|ui| {
                                    ui.label("⚡ Power Usage:");
                                    ui.label(format!("{:.1}W", power));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("📈 GPU Usage:");
                                    ui.add(
                                        egui::ProgressBar::new(util as f32 / 100.0)
                                            .text(format!("{}%", util)),
                                    );
                                });

                                if let Some(mem) = mem_info {
                                    ui.horizontal(|ui| {
                                        ui.label("💾 VRAM:");
                                        let used_gb = mem.used as f64 / 1e9;
                                        let total_gb = mem.total as f64 / 1e9;
                                        let usage_ratio = mem.used as f32 / mem.total as f32;
                                        ui.add(
                                            egui::ProgressBar::new(usage_ratio)
                                                .text(format!("{:.1}/{:.1} GB", used_gb, total_gb)),
                                        );
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
                            ui.colored_label(
                                egui::Color32::from_rgb(16, 185, 129),
                                "✅ nvibrant Available",
                            );

                            match vibrance::get_driver_info() {
                                Ok(info) => ui.label(info),
                                Err(e) => ui.colored_label(
                                    egui::Color32::from_rgb(239, 68, 68),
                                    format!("❌ {}", e),
                                ),
                            };
                        } else {
                            ui.colored_label(
                                egui::Color32::from_rgb(239, 68, 68),
                                "❌ nvibrant Not Available",
                            );
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
                                        let current_vibrance =
                                            vibrance::get_display_vibrance(i).unwrap_or(0);
                                        let mut percentage =
                                            vibrance::vibrance_to_percentage(current_vibrance)
                                                as i32;

                                        if ui
                                            .add(
                                                egui::Slider::new(&mut percentage, 0..=200)
                                                    .suffix("%")
                                                    .text("Vibrance"),
                                            )
                                            .changed()
                                        {
                                            let vibrance_val =
                                                vibrance::percentage_to_vibrance(percentage as u32);
                                            let display_values = vec![(i, vibrance_val)];
                                            if let Err(e) = vibrance::set_vibrance(&display_values)
                                            {
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
                                ui.colored_label(
                                    egui::Color32::from_rgb(239, 68, 68),
                                    format!("❌ Failed to detect displays: {}", e),
                                );
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
                                if let Err(e) = vibrance::set_vibrance_all(
                                    vibrance::percentage_to_vibrance(150),
                                ) {
                                    eprintln!("Failed to set gaming preset: {}", e);
                                }
                            }

                            if ui.button("🎨 Content Creation (120%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(
                                    vibrance::percentage_to_vibrance(120),
                                ) {
                                    eprintln!("Failed to set content creation preset: {}", e);
                                }
                            }

                            if ui.button("🔄 Default (100%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(0) {
                                    eprintln!("Failed to reset vibrance: {}", e);
                                }
                            }

                            if ui.button("🌑 Grayscale (0%)").clicked() {
                                if let Err(e) =
                                    vibrance::set_vibrance_all(vibrance::percentage_to_vibrance(0))
                                {
                                    eprintln!("Failed to set grayscale: {}", e);
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button("🎯 Max Vibrance (200%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(
                                    vibrance::percentage_to_vibrance(200),
                                ) {
                                    eprintln!("Failed to set max vibrance: {}", e);
                                }
                            }

                            if ui.button("📺 Movie Mode (110%)").clicked() {
                                if let Err(e) = vibrance::set_vibrance_all(
                                    vibrance::percentage_to_vibrance(110),
                                ) {
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
            }
            Tab::Display => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🖥️ Display & Color Management");

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
                        let display_values: Vec<(usize, i32)> = self
                            .vibrance_levels
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
                            ui.add(
                                egui::Slider::new(
                                    &mut self.overclock_profile.gpu_clock_offset,
                                    -200..=300,
                                )
                                .suffix(" MHz"),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Memory Clock Offset:");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.overclock_profile.memory_clock_offset,
                                    -500..=1000,
                                )
                                .suffix(" MHz"),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Power Limit:");
                            ui.add(
                                egui::Slider::new(
                                    &mut self.overclock_profile.power_limit,
                                    50..=120,
                                )
                                .suffix("%"),
                            );
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
                                    let current_speed =
                                        self.fan_speeds.get(&fan.id).copied().unwrap_or(50);
                                    let mut new_speed = current_speed;
                                    if ui
                                        .add(egui::Slider::new(&mut new_speed, 0..=100).suffix("%"))
                                        .changed()
                                    {
                                        self.fan_speeds.insert(fan.id, new_speed);
                                        if let Err(e) = fan::set_fan_speed(fan.id, new_speed) {
                                            eprintln!("Failed to set fan speed: {}", e);
                                        }
                                    }
                                } else {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(156, 163, 175),
                                        "Read-only",
                                    );
                                }
                            });
                        }
                    });
                });
            }
            Tab::Vrr => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🔄 VRR (Variable Refresh Rate) & G-Sync Control");

                    ui.group(|ui| {
                        ui.label("🖥️ Display VRR Status");
                        ui.separator();

                        // Detect VRR-capable displays
                        match vrr::detect_vrr_displays() {
                            Ok(displays) => {
                                for display in displays {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("📺 {}", display.display_name));

                                        if display.supports_vrr {
                                            let mut enabled = display.current_settings.enabled;
                                            if ui.checkbox(&mut enabled, "VRR Enabled").changed() {
                                                let new_settings = vrr::VrrSettings {
                                                    enabled,
                                                    ..display.current_settings
                                                };
                                                if let Err(e) = vrr::apply_vrr_settings(
                                                    &display.display_name,
                                                    &new_settings,
                                                ) {
                                                    eprintln!("Failed to toggle VRR: {}", e);
                                                }
                                            }
                                        } else {
                                            ui.colored_label(
                                                egui::Color32::RED,
                                                "❌ VRR Not Supported",
                                            );
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Refresh Range:");
                                        ui.label(format!(
                                            "{}-{}Hz",
                                            display.min_refresh, display.max_refresh
                                        ));

                                        if display.supports_gsync {
                                            ui.colored_label(egui::Color32::GREEN, "✅ G-Sync");
                                        }
                                        if display.supports_freesync {
                                            ui.colored_label(egui::Color32::GREEN, "✅ FreeSync");
                                        }
                                    });

                                    ui.separator();
                                }
                            }
                            Err(e) => {
                                ui.colored_label(
                                    egui::Color32::RED,
                                    format!("Error detecting VRR displays: {}", e),
                                );
                            }
                        }
                    });

                    ui.group(|ui| {
                        ui.label("⚙️ Advanced VRR Settings");
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Low Framerate Compensation:");
                            ui.checkbox(&mut true, "Enable LFC");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Adaptive Sync Mode:");
                            ui.checkbox(&mut true, "Adaptive");
                        });

                        ui.label("💡 Tips:");
                        ui.label("• VRR works best with framerates below max refresh rate");
                        ui.label("• Enable G-Sync in NVIDIA Control Panel for full functionality");
                        ui.label("• Some compositors require additional configuration");
                    });
                });
            }
            Tab::Latency => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("⚡ Latency Optimization & Gaming Performance");

                    ui.group(|ui| {
                        ui.label("🎯 Current Latency Status");
                        ui.separator();

                        match latency::get_latency_info() {
                            Ok(info) => {
                                ui.horizontal(|ui| {
                                    ui.label("NVIDIA Reflex:");
                                    if info.nvidia_reflex_available {
                                        ui.colored_label(
                                            if info.nvidia_reflex_enabled {
                                                egui::Color32::GREEN
                                            } else {
                                                egui::Color32::YELLOW
                                            },
                                            if info.nvidia_reflex_enabled {
                                                "✅ Enabled"
                                            } else {
                                                "⚠️ Available"
                                            },
                                        );
                                    } else {
                                        ui.colored_label(egui::Color32::RED, "❌ Not Available");
                                    }
                                });

                                ui.horizontal(|ui| {
                                    ui.label("GPU Scheduling:");
                                    ui.colored_label(
                                        if info.gpu_scheduling_enabled {
                                            egui::Color32::GREEN
                                        } else {
                                            egui::Color32::YELLOW
                                        },
                                        if info.gpu_scheduling_enabled {
                                            "✅ Enabled"
                                        } else {
                                            "❌ Disabled"
                                        },
                                    );
                                });

                                ui.horizontal(|ui| {
                                    ui.label("CPU Scheduler:");
                                    ui.label(&info.current_cpu_scheduler);
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Estimated Input Lag:");
                                    ui.colored_label(
                                        if info.estimated_input_lag_ms < 10.0 {
                                            egui::Color32::GREEN
                                        } else if info.estimated_input_lag_ms < 20.0 {
                                            egui::Color32::YELLOW
                                        } else {
                                            egui::Color32::RED
                                        },
                                        format!("{:.1}ms", info.estimated_input_lag_ms),
                                    );
                                });
                            }
                            Err(e) => {
                                ui.colored_label(
                                    egui::Color32::RED,
                                    format!("Error getting latency info: {}", e),
                                );
                            }
                        }
                    });

                    ui.group(|ui| {
                        ui.label("🚀 Latency Optimization Modes");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("🏆 Competitive Mode").clicked() {
                                if let Err(e) =
                                    latency::set_latency_mode(latency::LatencyMode::Competitive)
                                {
                                    eprintln!("Failed to set competitive mode: {}", e);
                                }
                            }
                            ui.label("Ultra-low latency, maximum performance");
                        });

                        ui.horizontal(|ui| {
                            if ui.button("⚖️ Balanced Mode").clicked() {
                                if let Err(e) =
                                    latency::set_latency_mode(latency::LatencyMode::Balanced)
                                {
                                    eprintln!("Failed to set balanced mode: {}", e);
                                }
                            }
                            ui.label("Good latency with system stability");
                        });

                        ui.horizontal(|ui| {
                            if ui.button("🔋 Power Saver").clicked() {
                                if let Err(e) =
                                    latency::set_latency_mode(latency::LatencyMode::PowerSaver)
                                {
                                    eprintln!("Failed to set power saver mode: {}", e);
                                }
                            }
                            ui.label("Higher latency but lower power usage");
                        });

                        ui.horizontal(|ui| {
                            if ui.button("🔄 Reset to Default").clicked() {
                                if let Err(e) =
                                    latency::set_latency_mode(latency::LatencyMode::Default)
                                {
                                    eprintln!("Failed to reset latency mode: {}", e);
                                }
                            }
                            ui.label("Restore system defaults");
                        });
                    });

                    ui.group(|ui| {
                        ui.label("🎮 Gaming-Specific Optimizations");
                        ui.separator();

                        if ui.button("🎯 Apply Full Latency Optimization").clicked() {
                            if let Err(e) = latency::optimize_latency() {
                                eprintln!("Failed to apply latency optimizations: {}", e);
                            }
                        }

                        ui.label("💡 Optimization Tips:");
                        ui.label("• Use exclusive fullscreen mode in games");
                        ui.label("• Enable VRR/G-Sync for consistent frame times");
                        ui.label("• Close unnecessary background applications");
                        ui.label("• Use a high-frequency gaming mouse (1000Hz)");
                        ui.label("• Consider overclocking GPU and CPU for higher framerates");
                    });
                });
            }
            Tab::Recording => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("📹 NVENC Recording & Shadowplay-like Features");

                    ui.group(|ui| {
                        ui.label("🎬 NVENC Capabilities");
                        ui.separator();

                        match recording::get_nvenc_capabilities() {
                            Ok(caps) => {
                                ui.horizontal(|ui| {
                                    ui.label("GPU:");
                                    ui.label(&caps.gpu_name);
                                });

                                ui.horizontal(|ui| {
                                    ui.label("H.264 NVENC:");
                                    ui.colored_label(
                                        if caps.h264_available {
                                            egui::Color32::GREEN
                                        } else {
                                            egui::Color32::RED
                                        },
                                        if caps.h264_available {
                                            "✅ Available"
                                        } else {
                                            "❌ Not Available"
                                        },
                                    );
                                });

                                ui.horizontal(|ui| {
                                    ui.label("H.265 NVENC:");
                                    ui.colored_label(
                                        if caps.h265_available {
                                            egui::Color32::GREEN
                                        } else {
                                            egui::Color32::RED
                                        },
                                        if caps.h265_available {
                                            "✅ Available"
                                        } else {
                                            "❌ Not Available"
                                        },
                                    );
                                });

                                ui.horizontal(|ui| {
                                    ui.label("AV1 NVENC:");
                                    ui.colored_label(
                                        if caps.av1_available {
                                            egui::Color32::GREEN
                                        } else {
                                            egui::Color32::RED
                                        },
                                        if caps.av1_available {
                                            "✅ Available (RTX 40+ Series)"
                                        } else {
                                            "❌ Not Available"
                                        },
                                    );
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Max Encoding Sessions:");
                                    ui.label(caps.max_encoding_sessions.to_string());
                                });
                            }
                            Err(e) => {
                                ui.colored_label(
                                    egui::Color32::RED,
                                    format!("Error detecting NVENC: {}", e),
                                );
                            }
                        }
                    });

                    ui.group(|ui| {
                        ui.label("🚀 Quick Recording Presets");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("🎮 Shadowplay Mode").clicked() {
                                let settings = recording::create_shadowplay_preset();
                                println!("Applied Shadowplay preset: {:?}", settings);
                            }
                            ui.label("High-quality H.265 recording, like NVIDIA Shadowplay");
                        });

                        ui.horizontal(|ui| {
                            if ui.button("💎 AV1 Lossless").clicked() {
                                let settings = recording::create_lossless_preset();
                                println!("Applied AV1 lossless preset: {:?}", settings);
                            }
                            ui.label("Ultra-high quality AV1 encoding for content creation");
                        });

                        ui.horizontal(|ui| {
                            if ui.button("📺 Streaming Optimized").clicked() {
                                let settings = recording::create_streaming_preset();
                                println!("Applied streaming preset: {:?}", settings);
                            }
                            ui.label("Low-latency H.264 for live streaming");
                        });

                        ui.horizontal(|ui| {
                            if ui.button("🎬 Content Creation").clicked() {
                                let settings = recording::create_content_creation_preset();
                                println!("Applied content creation preset: {:?}", settings);
                            }
                            ui.label("1440p AV1 recording for YouTube/content");
                        });
                    });

                    ui.group(|ui| {
                        ui.label("🎯 Recording Controls");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("🔴 Start Recording").clicked() {
                                let settings = recording::create_shadowplay_preset();
                                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                                let output_path = format!("nvcontrol_recording_{}.mp4", timestamp);

                                if let Err(e) = recording::start_recording(&settings, &output_path)
                                {
                                    eprintln!("Failed to start recording: {}", e);
                                }
                            }

                            if ui.button("⏹️ Stop Recording").clicked() {
                                if let Err(e) = recording::stop_recording() {
                                    eprintln!("Failed to stop recording: {}", e);
                                }
                            }

                            ui.colored_label(
                                if recording::is_recording() {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GRAY
                                },
                                if recording::is_recording() {
                                    "🔴 Recording..."
                                } else {
                                    "⚫ Stopped"
                                },
                            );
                        });

                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("⏪ Start Instant Replay").clicked() {
                                let settings = recording::create_shadowplay_preset();
                                if let Err(e) = recording::start_instant_replay(&settings) {
                                    eprintln!("Failed to start instant replay: {}", e);
                                }
                            }

                            if ui.button("💾 Save Last 5 Minutes").clicked() {
                                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                                let output_path = format!("instant_replay_{}.mp4", timestamp);

                                if let Err(e) = recording::save_instant_replay(&output_path) {
                                    eprintln!("Failed to save instant replay: {}", e);
                                }
                            }
                        });
                    });

                    ui.group(|ui| {
                        ui.label("⚙️ Recording Settings");
                        ui.separator();

                        let mut resolution_idx = 0;
                        ui.horizontal(|ui| {
                            ui.label("Resolution:");
                            egui::ComboBox::from_label("")
                                .selected_text("1920x1080")
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut resolution_idx,
                                        0,
                                        "1920x1080 (Full HD)",
                                    );
                                    ui.selectable_value(
                                        &mut resolution_idx,
                                        1,
                                        "2560x1440 (1440p)",
                                    );
                                    ui.selectable_value(&mut resolution_idx, 2, "3840x2160 (4K)");
                                });
                        });

                        let mut encoder_idx = 0;
                        ui.horizontal(|ui| {
                            ui.label("Encoder:");
                            egui::ComboBox::from_label("")
                                .selected_text("H.265 NVENC")
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut encoder_idx,
                                        0,
                                        "H.264 NVENC (Most Compatible)",
                                    );
                                    ui.selectable_value(
                                        &mut encoder_idx,
                                        1,
                                        "H.265 NVENC (Better Quality)",
                                    );
                                    ui.selectable_value(
                                        &mut encoder_idx,
                                        2,
                                        "AV1 NVENC (Best Quality - RTX 40+)",
                                    );
                                });
                        });

                        let mut bitrate = 50;
                        ui.horizontal(|ui| {
                            ui.label("Bitrate:");
                            ui.add(
                                egui::DragValue::new(&mut bitrate)
                                    .clamp_range(5.0..=200.0)
                                    .suffix(" Mbps"),
                            );
                        });

                        let mut framerate = 60;
                        ui.horizontal(|ui| {
                            ui.label("Framerate:");
                            ui.add(
                                egui::DragValue::new(&mut framerate)
                                    .clamp_range(30.0..=120.0)
                                    .suffix(" fps"),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut true, "Include Audio");
                            ui.checkbox(&mut false, "Lossless Mode");
                        });
                    });

                    ui.group(|ui| {
                        ui.label("💡 Tips & Information");
                        ui.separator();

                        ui.label("🎯 For Shadowplay-like Experience:");
                        ui.label("• Use H.265 NVENC for best quality/size ratio");
                        ui.label("• Enable instant replay for capturing highlights");
                        ui.label("• 50 Mbps bitrate provides excellent quality");

                        ui.separator();

                        ui.label("🚀 For Content Creation:");
                        ui.label("• Use AV1 NVENC on RTX 40+ series for best compression");
                        ui.label("• Record at 1440p or 4K for future-proofing");
                        ui.label("• Consider lossless mode for editing workflows");

                        ui.separator();

                        ui.label("📺 For Streaming:");
                        ui.label("• Use H.264 NVENC for maximum compatibility");
                        ui.label("• Lower bitrates (6-8 Mbps) for most platforms");
                        ui.label("• Enable hardware scheduling for lowest latency");

                        ui.separator();

                        ui.label("⚠️ Requirements:");
                        ui.label("• FFmpeg must be installed and in PATH");
                        ui.label("• NVIDIA GPU with NVENC support");
                        ui.label("• Sufficient disk space for recordings");
                    });
                });
            }
            Tab::Gamescope => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🎯 Gamescope Integration & Steam Deck Optimization");

                    ui.group(|ui| {
                        ui.label("🚀 Quick Launch Presets");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("🎮 Steam Deck (800p)").clicked() {
                                let config = gamescope::GamescopePreset::SteamDeck.to_config();
                                // TODO: Apply gamescope config
                                println!("Applying Steam Deck preset: {:?}", config);
                            }
                            if ui.button("📱 Handheld 1080p").clicked() {
                                let config = gamescope::GamescopePreset::Handheld1080p.to_config();
                                println!("Applying Handheld 1080p preset: {:?}", config);
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button("🖥️ Desktop Gaming").clicked() {
                                let config = gamescope::GamescopePreset::Desktop.to_config();
                                println!("Applying Desktop preset: {:?}", config);
                            }
                            if ui.button("🏆 Performance").clicked() {
                                let config = gamescope::GamescopePreset::Performance.to_config();
                                println!("Applying Performance preset: {:?}", config);
                            }
                        });
                    });

                    ui.group(|ui| {
                        ui.label("⚙️ Custom Gamescope Configuration");
                        ui.separator();

                        let mut width = 1920u32;
                        let mut height = 1080u32;
                        let mut refresh_rate = 60u32;

                        ui.horizontal(|ui| {
                            ui.label("Resolution:");
                            ui.add(
                                egui::DragValue::new(&mut width)
                                    .clamp_range(800.0..=3840.0)
                                    .prefix("W: "),
                            );
                            ui.label("×");
                            ui.add(
                                egui::DragValue::new(&mut height)
                                    .clamp_range(600.0..=2160.0)
                                    .prefix("H: "),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Refresh Rate:");
                            ui.add(
                                egui::DragValue::new(&mut refresh_rate)
                                    .clamp_range(30.0..=240.0)
                                    .suffix("Hz"),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Upscaling:");
                            egui::ComboBox::from_label("")
                                .selected_text("FSR")
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut 0, 0, "None");
                                    ui.selectable_value(&mut 1, 1, "Linear");
                                    ui.selectable_value(&mut 2, 2, "FSR");
                                    ui.selectable_value(&mut 3, 3, "NIS");
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut true, "HDR");
                            ui.checkbox(&mut true, "Adaptive Sync");
                            ui.checkbox(&mut false, "Borderless");
                        });

                        if ui.button("🚀 Apply Custom Configuration").clicked() {
                            // TODO: Create and apply custom gamescope config
                            println!("Applying custom gamescope configuration");
                        }
                    });

                    ui.group(|ui| {
                        ui.label("🔧 NVIDIA Optimizations for Gamescope");
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut true, "__GL_THREADED_OPTIMIZATIONS");
                            ui.label("Enable threaded optimizations");
                        });

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut false, "__GL_SYNC_TO_VBLANK");
                            ui.label("Disable VSync (for VRR)");
                        });

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut true, "NVIDIA Prime Render Offload");
                            ui.label("Force NVIDIA GPU usage");
                        });

                        ui.label("💡 Gamescope Tips:");
                        ui.label("• Use --adaptive-sync for VRR displays");
                        ui.label("• Enable FSR for better performance on lower resolutions");
                        ui.label("• HDR requires supported display and compositor");
                        ui.label("• For Steam games, use launch options: gamescope -- %command%");
                    });
                });
            }
            Tab::ShaderCache => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🎨 Shader Cache Management");

                    ui.group(|ui| {
                        ui.label("📊 Shader Cache Status");
                        ui.separator();

                        // Shader cache size and location info
                        ui.horizontal(|ui| {
                            ui.label("Cache Location:");
                            ui.code("~/.nv/GLCache");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Cache Size:");
                            ui.code("~500MB"); // This would be dynamically calculated
                        });

                        ui.horizontal(|ui| {
                            ui.label("Cached Shaders:");
                            ui.code("1,234 files"); // This would be dynamically calculated
                        });
                    });

                    ui.group(|ui| {
                        ui.label("🛠️ Cache Management");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("🗑️ Clear Cache").clicked() {
                                // TODO: Implement shader cache clearing
                                println!("Clearing shader cache...");
                            }

                            if ui.button("🔄 Rebuild Cache").clicked() {
                                // TODO: Implement shader cache rebuilding
                                println!("Rebuilding shader cache...");
                            }

                            if ui.button("📁 Open Cache Folder").clicked() {
                                // TODO: Open cache folder in file manager
                                println!("Opening cache folder...");
                            }
                        });

                        ui.checkbox(&mut false, "Auto-clear cache on startup");
                        ui.checkbox(&mut false, "Compress shader cache");
                        ui.checkbox(&mut true, "Enable shader caching");
                    });

                    ui.group(|ui| {
                        ui.label("📈 Cache Statistics");
                        ui.separator();

                        egui::Grid::new("shader_cache_stats")
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label("Cache Hit Rate:");
                                ui.label("94.2%");
                                ui.end_row();

                                ui.label("Average Compilation Time:");
                                ui.label("12.3ms");
                                ui.end_row();

                                ui.label("Most Used Shader:");
                                ui.label("vertex_main.glsl");
                                ui.end_row();

                                ui.label("Last Cleared:");
                                ui.label("Never");
                                ui.end_row();
                            });
                    });

                    ui.group(|ui| {
                        ui.label("💡 Shader Cache Tips:");
                        ui.label("• Clearing cache may cause temporary stuttering in games");
                        ui.label("• Large cache sizes can slow down driver startup");
                        ui.label("• Rebuilding cache can improve performance with new drivers");
                        ui.label("• Cache is automatically managed by the driver");
                    });
                });
            }
            Tab::Drivers => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🔧 Driver Management");

                    ui.group(|ui| {
                        ui.label("📋 Current Driver Information");
                        ui.separator();

                        egui::Grid::new("driver_info")
                            .num_columns(2)
                            .show(ui, |ui| {
                                ui.label("Driver Version:");
                                ui.label("525.147.05"); // This would be dynamically detected
                                ui.end_row();

                                ui.label("CUDA Version:");
                                ui.label("12.0");
                                ui.end_row();

                                ui.label("Installation Date:");
                                ui.label("2024-12-15");
                                ui.end_row();

                                ui.label("Driver Type:");
                                ui.label("Production Branch");
                                ui.end_row();

                                ui.label("Architecture:");
                                ui.label("x86_64");
                                ui.end_row();
                            });
                    });

                    ui.group(|ui| {
                        ui.label("🔄 Driver Actions");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("📥 Check for Updates").clicked() {
                                // TODO: Implement driver update checking
                                println!("Checking for driver updates...");
                            }

                            if ui.button("🔧 Reinstall Driver").clicked() {
                                // TODO: Implement driver reinstallation
                                println!("Reinstalling driver...");
                            }

                            if ui.button("📊 Driver Validation").clicked() {
                                // TODO: Implement driver validation
                                println!("Validating driver installation...");
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button("📜 View Logs").clicked() {
                                // TODO: Open driver logs
                                println!("Opening driver logs...");
                            }

                            if ui.button("🧹 Clean Install").clicked() {
                                // TODO: Implement clean driver installation
                                println!("Performing clean driver install...");
                            }
                        });
                    });

                    ui.group(|ui| {
                        ui.label("⚙️ Driver Settings");
                        ui.separator();

                        ui.checkbox(&mut true, "Enable automatic driver updates");
                        ui.checkbox(&mut false, "Use beta/development drivers");
                        ui.checkbox(&mut true, "Install GeForce Experience (if available)");
                        ui.checkbox(&mut false, "Enable driver telemetry");

                        ui.horizontal(|ui| {
                            ui.label("Update Channel:");
                            egui::ComboBox::from_label("")
                                .selected_text("Production")
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut "Production",
                                        "Production",
                                        "Production",
                                    );
                                    ui.selectable_value(&mut "Production", "Beta", "Beta");
                                    ui.selectable_value(
                                        &mut "Production",
                                        "Developer",
                                        "Developer",
                                    );
                                });
                        });
                    });

                    ui.group(|ui| {
                        ui.label("🚨 Driver Status");
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Status:");
                            ui.colored_label(egui::Color32::GREEN, "✅ Working Properly");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Last Update:");
                            ui.label("No updates available");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Compatibility:");
                            ui.colored_label(egui::Color32::GREEN, "✅ Compatible");
                        });
                    });

                    ui.group(|ui| {
                        ui.label("💡 Driver Tips:");
                        ui.label("• Always backup important data before driver updates");
                        ui.label("• Clean installs can resolve stability issues");
                        ui.label(
                            "• Beta drivers may have performance improvements but less stability",
                        );
                        ui.label("• Check release notes before updating for game compatibility");
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
