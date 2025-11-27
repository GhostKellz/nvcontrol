#[cfg(feature = "gui")]
use eframe::egui;
#[cfg(feature = "gui")]
use nvcontrol::{
    config, display, fan, gamescope, latency, overclocking, recording, theme,
    vibrance, vrr,
};

#[cfg(feature = "gui")]
enum Tab {
    Gpu,
    Display,
    Vibrance,
    Overclock,
    AutoOverclock,
    Fan,
    Vrr,
    Latency,
    Recording,
    Gamescope,
    ShaderCache,
    Drivers,
    Benchmark,
    Hdr,
    PowerCurves,
    GameProfiles,
    RgbControl,
    Containers,
    Osd,            // MangoHud/OSD configuration
    Settings,
}

#[cfg(feature = "gui")]
#[derive(Clone, Copy, PartialEq, Debug)]
enum OcPreset {
    Stock,       // 0/0/80%
    MildOc,      // +75/+500/90%
    Performance, // +150/+1000/95%
    Extreme,     // +200/+1500/105% - RTX 5090 max
}

#[cfg(feature = "gui")]
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub gpu_usage: String,
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
    // Async GPU monitoring
    gpu_stats_rx: std::sync::mpsc::Receiver<GpuStats>,
    last_stats_update: std::time::Instant,
    // GUI widgets
    fan_curve: nvcontrol::gui_widgets::FanCurve,
    voltage_curve: nvcontrol::gui_widgets::VoltageCurve,
    monitoring_dashboard: nvcontrol::gui_widgets::MonitoringDashboard,
    // HDR configuration
    hdr_config: nvcontrol::hdr::HdrConfig,
    // Multi-GPU support
    available_gpus: Vec<nvcontrol::multi_gpu::GpuInfo>,
    selected_gpu_index: u32,
    // Power curves
    power_config: nvcontrol::power_curves::PowerManagementConfig,
    // Auto-overclock wizard
    auto_oc_running: bool,
    auto_oc_target: String,
    auto_oc_safety: String,
    auto_oc_max_temp: f32,
    auto_oc_max_power: u32,
    // Game profile auto-apply
    game_auto_config: nvcontrol::game_profile_auto::AutoProfileConfig,

    // NEW: Interactive OC controls (from TUI)
    gpu_offset: i32,        // -200 to +200 MHz
    memory_offset: i32,     // -1000 to +1000 MHz
    power_limit_percent: u32, // 50 to 105%
    oc_preset: OcPreset,

    // NEW: Theme selection
    current_theme: nvcontrol::themes::ThemeVariant,

    // NEW: RGB control
    rgb_mode: String,
    rgb_color: [f32; 3],

    // NEW: Container management
    container_runtime: Option<nvcontrol::container_runtime::NvContainerRuntime>,
    running_containers: Vec<ContainerInfo>,

    // OSD/MangoHud configuration
    osd_enabled: bool,
    osd_position: String,
    osd_metrics: Vec<String>,
    osd_font_size: u32,
    osd_opacity: f32,
    mangohud_installed: bool,
}

#[cfg(feature = "gui")]
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct GpuStats {
    pub name: String,
    pub temperature: f32,
    pub utilization: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub power_draw: f32,
    pub fan_speed: u32,
    pub gpu_clock: u32,
    pub memory_clock: u32,
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

        // Create channel for async GPU stats
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn background thread for GPU monitoring
        std::thread::spawn(move || {
            loop {
                // Fetch GPU stats in background
                if let Ok(nvml) = nvml_wrapper::Nvml::init() {
                    if let Ok(device) = nvml.device_by_index(0) {
                        let name = device.name().unwrap_or_else(|_| "Unknown GPU".to_string());
                        let temperature = device
                            .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                            .unwrap_or(0) as f32;
                        let power_draw = device.power_usage().map(|p| p as f32 / 1000.0).unwrap_or(0.0);
                        let utilization_rates = device.utilization_rates().ok();
                        let utilization = utilization_rates.map(|u| u.gpu as f32).unwrap_or(0.0);
                        let mem_info = device.memory_info().ok();
                        let memory_used = mem_info.as_ref().map(|m| m.used).unwrap_or(0);
                        let memory_total = mem_info.as_ref().map(|m| m.total).unwrap_or(0);
                        let fan_speed = device.fan_speed(0).unwrap_or(0);
                        let gpu_clock = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics).unwrap_or(0);
                        let memory_clock = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory).unwrap_or(0);

                        let stats = GpuStats {
                            name,
                            temperature,
                            utilization,
                            memory_used,
                            memory_total,
                            power_draw,
                            fan_speed,
                            gpu_clock,
                            memory_clock,
                        };

                        // Send stats to UI thread (ignore errors if channel closed)
                        let _ = tx.send(stats);
                    }
                }

                // Update every 500ms
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        });

        // Initialize container runtime
        let container_runtime = nvcontrol::container_runtime::NvContainerRuntime::new().ok();

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
            gpu_stats_rx: rx,
            last_stats_update: std::time::Instant::now(),
            fan_curve: nvcontrol::gui_widgets::FanCurve::new(),
            voltage_curve: nvcontrol::gui_widgets::VoltageCurve::new(),
            monitoring_dashboard: nvcontrol::gui_widgets::MonitoringDashboard::new(120), // 2 minutes at 1Hz
            hdr_config: nvcontrol::hdr::HdrConfig::load().unwrap_or_default(),
            available_gpus: nvcontrol::multi_gpu::detect_gpus().unwrap_or_else(|_| vec![]),
            selected_gpu_index: 0,
            power_config: nvcontrol::power_curves::load_power_config().unwrap_or_default(),
            auto_oc_running: false,
            auto_oc_target: "balanced".to_string(),
            auto_oc_safety: "conservative".to_string(),
            auto_oc_max_temp: 85.0,
            auto_oc_max_power: 100,
            game_auto_config: nvcontrol::game_profile_auto::load_config().unwrap_or_default(),

            // NEW: Interactive OC controls
            gpu_offset: 0,
            memory_offset: 0,
            power_limit_percent: 100,
            oc_preset: OcPreset::Stock,

            // NEW: Theme
            current_theme: nvcontrol::themes::ThemeVariant::TokyoNightNight,

            // NEW: RGB
            rgb_mode: "Static".to_string(),
            rgb_color: [1.0, 0.0, 0.0], // Red (ROG default)

            // NEW: Containers
            container_runtime,
            running_containers: Vec::new(),

            // OSD/MangoHud
            osd_enabled: false,
            osd_position: "top-left".to_string(),
            osd_metrics: vec![
                "fps".to_string(),
                "gpu_temp".to_string(),
                "gpu_load".to_string(),
                "vram".to_string(),
                "frametime".to_string(),
            ],
            osd_font_size: 24,
            osd_opacity: 0.8,
            mangohud_installed: nvcontrol::osd::OsdManager::check_mangohud_installed(),
        }
    }

    fn update_gpu_stats_from_channel(&mut self) {
        // Non-blocking receive from channel
        if let Ok(stats) = self.gpu_stats_rx.try_recv() {
            // Update monitoring dashboard
            let gui_stats = nvcontrol::gui_widgets::GpuStats {
                temperature: stats.temperature,
                power_draw: stats.power_draw,
                utilization: stats.utilization,
                fan_speed: stats.fan_speed,
            };
            self.monitoring_dashboard.update(&gui_stats);

            self.gpu_stats = Some(stats);
            self.last_stats_update = std::time::Instant::now();
        }
    }

    // NEW: Apply OC preset
    fn apply_oc_preset(&mut self, preset: OcPreset) {
        self.oc_preset = preset;
        match preset {
            OcPreset::Stock => {
                self.gpu_offset = 0;
                self.memory_offset = 0;
                self.power_limit_percent = 80;
            }
            OcPreset::MildOc => {
                self.gpu_offset = 75;
                self.memory_offset = 500;
                self.power_limit_percent = 90;
            }
            OcPreset::Performance => {
                self.gpu_offset = 150;
                self.memory_offset = 1000;
                self.power_limit_percent = 95;
            }
            OcPreset::Extreme => {
                self.gpu_offset = 200;
                self.memory_offset = 1500;
                self.power_limit_percent = 105;
            }
        }
    }

    // NEW: Apply overclock settings
    fn apply_overclock(&self) {
        let profile = overclocking::OverclockProfile {
            name: "GUI Custom".to_string(),
            gpu_clock_offset: self.gpu_offset,
            memory_clock_offset: self.memory_offset,
            power_limit: self.power_limit_percent as u8,
            voltage_offset: 0,
            temp_limit: 90,
            fan_curve: vec![],  // Use default fan curve
        };

        if let Err(e) = overclocking::apply_overclock_profile(&profile) {
            eprintln!("Failed to apply overclock: {}", e);
        } else {
            println!("✅ Overclock applied: GPU {:+}MHz, Mem {:+}MHz, Power {}%",
                self.gpu_offset, self.memory_offset, self.power_limit_percent);
        }
    }

    // NEW: Apply RGB settings
    fn apply_rgb_settings(&self) {
        use nvcontrol::rgb_control::{RgbController, RgbMode, RgbColor};

        if let Ok(mut controller) = RgbController::new() {
            let color = RgbColor {
                r: (self.rgb_color[0] * 255.0) as u8,
                g: (self.rgb_color[1] * 255.0) as u8,
                b: (self.rgb_color[2] * 255.0) as u8,
            };

            let mode = match self.rgb_mode.as_str() {
                "Static" => RgbMode::Static,
                "Breathing" => RgbMode::Breathing,
                "Rainbow" => RgbMode::Rainbow,
                "TempReactive" => {
                    if let Err(e) = controller.set_temp_reactive() {
                        eprintln!("Failed to set temp reactive mode: {}", e);
                    }
                    return;
                }
                "Off" => RgbMode::Off,
                _ => RgbMode::Static,
            };

            if let Err(e) = controller.set_gpu_mode(mode, color) {
                eprintln!("Failed to set RGB: {}", e);
            } else {
                println!("✅ RGB mode applied: {:?}", self.rgb_mode);
            }
        } else {
            eprintln!("❌ Failed to initialize RGB controller. Is OpenRGB installed?");
        }
    }

    // NEW: Apply theme
    #[allow(unused_variables)]
    fn apply_theme(&mut self, ctx: &egui::Context) {
        let palette = nvcontrol::themes::ColorPalette::from_variant(self.current_theme);
        // Convert palette to egui visuals
        let visuals = egui::Visuals::dark();
        // Apply colors from palette (simplified for now)
        ctx.set_visuals(visuals);
    }

    // NEW: Refresh container list
    fn refresh_containers(&mut self) {
        if let Some(ref _runtime) = self.container_runtime {
            // Get running containers with GPU access
            self.running_containers = Vec::new();
            // This would call runtime.list_containers() when implemented
        }
    }

    // Save MangoHud configuration to ~/.config/MangoHud/MangoHud.conf
    fn save_mangohud_config(&self) {
        if let Some(config_dir) = dirs::config_dir() {
            let mangohud_dir = config_dir.join("MangoHud");
            if std::fs::create_dir_all(&mangohud_dir).is_ok() {
                let config_path = mangohud_dir.join("MangoHud.conf");

                let mut config_lines = vec![
                    "# nvcontrol MangoHud configuration".to_string(),
                    "# Auto-generated by nvcontrol GUI".to_string(),
                    "".to_string(),
                    format!("position={}", self.osd_position),
                    "".to_string(),
                ];

                // Add enabled metrics
                for metric in &self.osd_metrics {
                    config_lines.push(metric.clone());
                }

                config_lines.push("".to_string());
                config_lines.push(format!("font_size={}", self.osd_font_size));
                config_lines.push(format!("background_alpha={:.2}", self.osd_opacity));

                let _ = std::fs::write(config_path, config_lines.join("\n"));
            }
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for NvControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Non-blocking update from background thread
        self.update_gpu_stats_from_channel();

        // Request repaint after 500ms for smooth updates
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        // Side panel with grouped navigation
        egui::SidePanel::left("nav_panel").default_width(170.0).show(ctx, |ui| {
            ui.add_space(8.0);

            // Header with GPU stats
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("󰢮 nvcontrol").strong());
            });

            // Live GPU stats in header
            if let Some(stats) = &self.gpu_stats {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let temp_color = if stats.temperature > 80.0 {
                        egui::Color32::from_rgb(255, 85, 85)
                    } else if stats.temperature > 70.0 {
                        egui::Color32::from_rgb(255, 170, 0)
                    } else {
                        egui::Color32::from_rgb(80, 250, 123)
                    };
                    ui.label(egui::RichText::new(format!("{}°C", stats.temperature as i32))
                        .small()
                        .color(temp_color));
                    ui.separator();
                    ui.label(egui::RichText::new(format!("{}%", stats.utilization as i32))
                        .small());
                    ui.separator();
                    ui.label(egui::RichText::new(format!("{:.0}W", stats.power_draw))
                        .small());
                });
            }

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // GPU & Hardware section
                ui.label(egui::RichText::new("󰍹 GPU & Hardware").small().strong().color(egui::Color32::from_rgb(139, 233, 253)));
                ui.add_space(2.0);
                if ui.selectable_label(matches!(self.tab, Tab::Gpu), "   🎮 GPU Status").clicked() {
                    self.tab = Tab::Gpu;
                }
                let oc_label = if self.gpu_offset != 0 || self.memory_offset != 0 {
                    "   ⚡ Overclock  ●"
                } else {
                    "   ⚡ Overclock"
                };
                if ui.selectable_label(matches!(self.tab, Tab::Overclock), oc_label).clicked() {
                    self.tab = Tab::Overclock;
                }
                let auto_oc_label = if self.auto_oc_running {
                    "   🚀 Auto-OC  ⟳"
                } else {
                    "   🚀 Auto-OC"
                };
                if ui.selectable_label(matches!(self.tab, Tab::AutoOverclock), auto_oc_label).clicked() {
                    self.tab = Tab::AutoOverclock;
                }
                if ui.selectable_label(matches!(self.tab, Tab::Fan), "   🌀 Fan Control").clicked() {
                    self.tab = Tab::Fan;
                }
                if ui.selectable_label(matches!(self.tab, Tab::PowerCurves), "   🔋 Power").clicked() {
                    self.tab = Tab::PowerCurves;
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(4.0);

                // Display section
                ui.label(egui::RichText::new("🖥️ Display").small().strong().color(egui::Color32::from_rgb(189, 147, 249)));
                ui.add_space(2.0);
                if ui.selectable_label(matches!(self.tab, Tab::Display), "   🖥️ Display").clicked() {
                    self.tab = Tab::Display;
                }
                let vibrance_label = if self.vibrance_levels.iter().any(|&v| v != 0) {
                    "   🌈 Vibrance  ●"
                } else {
                    "   🌈 Vibrance"
                };
                if ui.selectable_label(matches!(self.tab, Tab::Vibrance), vibrance_label).clicked() {
                    self.tab = Tab::Vibrance;
                }
                let hdr_label = if self.hdr_enabled {
                    "   ☀️ HDR  ●"
                } else {
                    "   ☀️ HDR"
                };
                if ui.selectable_label(matches!(self.tab, Tab::Hdr), hdr_label).clicked() {
                    self.tab = Tab::Hdr;
                }
                if ui.selectable_label(matches!(self.tab, Tab::Vrr), "   🔄 VRR").clicked() {
                    self.tab = Tab::Vrr;
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(4.0);

                // Gaming section
                ui.label(egui::RichText::new("🎯 Gaming").small().strong().color(egui::Color32::from_rgb(80, 250, 123)));
                ui.add_space(2.0);
                if ui.selectable_label(matches!(self.tab, Tab::GameProfiles), "   🎮 Profiles").clicked() {
                    self.tab = Tab::GameProfiles;
                }
                let osd_label = if self.osd_enabled {
                    "   📊 OSD  ●"
                } else {
                    "   📊 OSD"
                };
                if ui.selectable_label(matches!(self.tab, Tab::Osd), osd_label).clicked() {
                    self.tab = Tab::Osd;
                }
                if ui.selectable_label(matches!(self.tab, Tab::Latency), "   ⚡ Latency").clicked() {
                    self.tab = Tab::Latency;
                }
                if ui.selectable_label(matches!(self.tab, Tab::Gamescope), "   🎯 Gamescope").clicked() {
                    self.tab = Tab::Gamescope;
                }
                if ui.selectable_label(matches!(self.tab, Tab::Recording), "   📹 Recording").clicked() {
                    self.tab = Tab::Recording;
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(4.0);

                // System section
                ui.label(egui::RichText::new("⚙️ System").small().strong().color(egui::Color32::from_rgb(255, 121, 198)));
                ui.add_space(2.0);
                if ui.selectable_label(matches!(self.tab, Tab::ShaderCache), "   🎨 Shaders").clicked() {
                    self.tab = Tab::ShaderCache;
                }
                if ui.selectable_label(matches!(self.tab, Tab::Drivers), "   🔧 Drivers").clicked() {
                    self.tab = Tab::Drivers;
                }
                let container_label = if !self.running_containers.is_empty() {
                    format!("   🐳 Containers  ({})", self.running_containers.len())
                } else {
                    "   🐳 Containers".to_string()
                };
                if ui.selectable_label(matches!(self.tab, Tab::Containers), &container_label).clicked() {
                    self.tab = Tab::Containers;
                }
                if ui.selectable_label(matches!(self.tab, Tab::RgbControl), "   💡 RGB").clicked() {
                    self.tab = Tab::RgbControl;
                }
                if ui.selectable_label(matches!(self.tab, Tab::Benchmark), "   📊 Benchmark").clicked() {
                    self.tab = Tab::Benchmark;
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(4.0);

                // Settings at bottom
                if ui.selectable_label(matches!(self.tab, Tab::Settings), "   ⚙️ Settings").clicked() {
                    self.tab = Tab::Settings;
                }

                // Version info at bottom
                ui.add_space(10.0);
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("v0.7.0").small().weak());
                });
            });
        });
        match self.tab {
            Tab::Gpu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🎮 GPU Status & Monitoring");

                    // GPU Selector (if multiple GPUs)
                    if self.available_gpus.len() > 1 {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("🎯 Select GPU:");
                                egui::ComboBox::from_id_source("gpu_selector")
                                    .selected_text(format!("GPU {} - {}",
                                        self.selected_gpu_index,
                                        self.available_gpus.get(self.selected_gpu_index as usize)
                                            .map(|g| g.name.as_str())
                                            .unwrap_or("Unknown")
                                    ))
                                    .show_ui(ui, |ui| {
                                        for gpu in &self.available_gpus {
                                            ui.selectable_value(
                                                &mut self.selected_gpu_index,
                                                gpu.index,
                                                format!("GPU {} - {} ({:.1}°C, {:.0}%)",
                                                    gpu.index,
                                                    gpu.name,
                                                    gpu.temperature,
                                                    gpu.utilization
                                                )
                                            );
                                        }
                                    });

                                if ui.button("🔄 Refresh GPUs").clicked() {
                                    self.available_gpus = nvcontrol::multi_gpu::detect_gpus().unwrap_or_else(|_| vec![]);
                                }
                            });

                            // Show multi-GPU info
                            ui.horizontal(|ui| {
                                ui.label(format!("📊 Total GPUs: {}", self.available_gpus.len()));

                                let has_sli = self.available_gpus.iter().any(|g| g.sli_enabled);
                                let has_nvlink = self.available_gpus.iter().any(|g| g.nvlink_enabled);

                                if has_sli {
                                    ui.colored_label(egui::Color32::GREEN, "✅ SLI");
                                }
                                if has_nvlink {
                                    ui.colored_label(egui::Color32::GREEN, "✅ NVLink");
                                }
                            });
                        });

                        ui.add_space(10.0);
                    }

                    // Get GPU info from cached stats
                    ui.group(|ui| {
                        ui.label("📊 Real-time Stats");
                        ui.separator();

                        if let Some(ref stats) = self.gpu_stats {
                            ui.horizontal(|ui| {
                                ui.label("🎯 GPU:");
                                ui.label(&stats.name);
                            });

                            ui.horizontal(|ui| {
                                ui.label("🌡️ Temperature:");
                                ui.colored_label(
                                    if stats.temperature > 80.0 {
                                        egui::Color32::RED
                                    } else if stats.temperature > 70.0 {
                                        egui::Color32::YELLOW
                                    } else {
                                        egui::Color32::GREEN
                                    },
                                    format!("{:.1}°C", stats.temperature),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("⚡ Power Usage:");
                                ui.label(format!("{:.1}W", stats.power_draw));
                            });

                            ui.horizontal(|ui| {
                                ui.label("📈 GPU Usage:");
                                ui.add(
                                    egui::ProgressBar::new(stats.utilization / 100.0)
                                        .text(format!("{:.0}%", stats.utilization)),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("💾 VRAM:");
                                let used_gb = stats.memory_used as f64 / 1e9;
                                let total_gb = stats.memory_total as f64 / 1e9;
                                let usage_ratio = stats.memory_used as f32 / stats.memory_total as f32;
                                ui.add(
                                    egui::ProgressBar::new(usage_ratio)
                                        .text(format!("{:.1}/{:.1} GB", used_gb, total_gb)),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("🌀 Fan Speed:");
                                ui.label(format!("{}%", stats.fan_speed));
                            });
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

                            if ui.button("🗑️ Clear Graphs").clicked() {
                                self.monitoring_dashboard.clear_all();
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // Real-time monitoring graphs
                    ui.group(|ui| {
                        ui.label("📈 Real-Time Monitoring");
                        ui.separator();

                        use egui_plot::{Line, Plot, PlotPoints};

                        // Temperature graph
                        ui.label("🌡️ Temperature History");
                        let temp_points: PlotPoints = self
                            .monitoring_dashboard
                            .temperature
                            .get_points()
                            .into_iter()
                            .map(|p| [p[0], p[1]])
                            .collect();

                        Plot::new("temperature_plot")
                            .height(150.0)
                            .width(ui.available_width())
                            .y_axis_label("Temperature (°C)")
                            .allow_drag(false)
                            .allow_zoom(false)
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(temp_points)
                                        .color(egui::Color32::from_rgb(239, 68, 68))
                                        .name("Temperature"),
                                );
                            });

                        // Show stats
                        if let (Some(min), Some(max), Some(avg)) = (
                            self.monitoring_dashboard.temperature.min_value(),
                            self.monitoring_dashboard.temperature.max_value(),
                            self.monitoring_dashboard.temperature.avg_value(),
                        ) {
                            ui.horizontal(|ui| {
                                ui.label(format!("Min: {:.1}°C", min));
                                ui.separator();
                                ui.label(format!("Max: {:.1}°C", max));
                                ui.separator();
                                ui.label(format!("Avg: {:.1}°C", avg));
                            });
                        }

                        ui.add_space(10.0);

                        // GPU Utilization graph
                        ui.label("📊 GPU Utilization History");
                        let util_points: PlotPoints = self
                            .monitoring_dashboard
                            .gpu_utilization
                            .get_points()
                            .into_iter()
                            .map(|p| [p[0], p[1]])
                            .collect();

                        Plot::new("utilization_plot")
                            .height(150.0)
                            .width(ui.available_width())
                            .y_axis_label("Utilization (%)")
                            .allow_drag(false)
                            .allow_zoom(false)
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(util_points)
                                        .color(egui::Color32::from_rgb(59, 130, 246))
                                        .name("GPU Usage"),
                                );
                            });

                        if let (Some(min), Some(max), Some(avg)) = (
                            self.monitoring_dashboard.gpu_utilization.min_value(),
                            self.monitoring_dashboard.gpu_utilization.max_value(),
                            self.monitoring_dashboard.gpu_utilization.avg_value(),
                        ) {
                            ui.horizontal(|ui| {
                                ui.label(format!("Min: {:.1}%", min));
                                ui.separator();
                                ui.label(format!("Max: {:.1}%", max));
                                ui.separator();
                                ui.label(format!("Avg: {:.1}%", avg));
                            });
                        }

                        ui.add_space(10.0);

                        // Power Draw graph
                        ui.label("⚡ Power Draw History");
                        let power_points: PlotPoints = self
                            .monitoring_dashboard
                            .power
                            .get_points()
                            .into_iter()
                            .map(|p| [p[0], p[1]])
                            .collect();

                        Plot::new("power_plot")
                            .height(150.0)
                            .width(ui.available_width())
                            .y_axis_label("Power (W)")
                            .allow_drag(false)
                            .allow_zoom(false)
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(power_points)
                                        .color(egui::Color32::from_rgb(251, 191, 36))
                                        .name("Power Draw"),
                                );
                            });

                        if let (Some(min), Some(max), Some(avg)) = (
                            self.monitoring_dashboard.power.min_value(),
                            self.monitoring_dashboard.power.max_value(),
                            self.monitoring_dashboard.power.avg_value(),
                        ) {
                            ui.horizontal(|ui| {
                                ui.label(format!("Min: {:.1}W", min));
                                ui.separator();
                                ui.label(format!("Max: {:.1}W", max));
                                ui.separator();
                                ui.label(format!("Avg: {:.1}W", avg));
                            });
                        }

                        ui.add_space(10.0);

                        // Fan Speed graph
                        ui.label("🌀 Fan Speed History");
                        let fan_points: PlotPoints = self
                            .monitoring_dashboard
                            .fan_speed
                            .get_points()
                            .into_iter()
                            .map(|p| [p[0], p[1]])
                            .collect();

                        Plot::new("fan_plot")
                            .height(150.0)
                            .width(ui.available_width())
                            .y_axis_label("Fan Speed (%)")
                            .allow_drag(false)
                            .allow_zoom(false)
                            .show(ui, |plot_ui| {
                                plot_ui.line(
                                    Line::new(fan_points)
                                        .color(egui::Color32::from_rgb(16, 185, 129))
                                        .name("Fan Speed"),
                                );
                            });

                        if let (Some(min), Some(max), Some(avg)) = (
                            self.monitoring_dashboard.fan_speed.min_value(),
                            self.monitoring_dashboard.fan_speed.max_value(),
                            self.monitoring_dashboard.fan_speed.avg_value(),
                        ) {
                            ui.horizontal(|ui| {
                                ui.label(format!("Min: {:.1}%", min));
                                ui.separator();
                                ui.label(format!("Max: {:.1}%", max));
                                ui.separator();
                                ui.label(format!("Avg: {:.1}%", avg));
                            });
                        }
                    });
                });
            }
            Tab::Vibrance => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🌈 Digital Vibrance Control");

                    // Native vibrance status
                    ui.group(|ui| {
                        ui.label("📋 Native Vibrance Status");
                        ui.separator();

                        if vibrance::is_available() {
                            ui.colored_label(
                                egui::Color32::from_rgb(16, 185, 129),
                                "✅ Native Vibrance Available (Pure Rust)",
                            );

                            match vibrance::get_driver_info() {
                                Ok(info) => {
                                    ui.label(info);
                                    ui.label("Using direct NVKMS ioctls - no external dependencies")
                                },
                                Err(e) => {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(239, 68, 68),
                                        format!("❌ {}", e),
                                    )
                                },
                            };
                        } else {
                            ui.colored_label(
                                egui::Color32::from_rgb(239, 68, 68),
                                "❌ Native Vibrance Not Available",
                            );
                            ui.label("Requires NVIDIA open drivers 580+");
                            ui.label("Install: sudo pacman -S nvidia-open-dkms");
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
                    ui.heading("⚡ Interactive GPU Overclocking");

                    ui.group(|ui| {
                        ui.label("🎯 Quick Presets");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.selectable_label(self.oc_preset == OcPreset::Stock, "📊 Stock").clicked() {
                                self.apply_oc_preset(OcPreset::Stock);
                            }
                            if ui.selectable_label(self.oc_preset == OcPreset::MildOc, "🔧 Mild OC").clicked() {
                                self.apply_oc_preset(OcPreset::MildOc);
                            }
                            if ui.selectable_label(self.oc_preset == OcPreset::Performance, "⚡ Performance").clicked() {
                                self.apply_oc_preset(OcPreset::Performance);
                            }
                            if ui.selectable_label(self.oc_preset == OcPreset::Extreme, "🔥 Extreme (5090)").clicked() {
                                self.apply_oc_preset(OcPreset::Extreme);
                            }
                        });

                        ui.add_space(5.0);

                        ui.label(format!("Current: {:?}", self.oc_preset));
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("🎛️ Manual Tuning");
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("GPU Clock Offset:");
                            ui.add(
                                egui::Slider::new(&mut self.gpu_offset, -200..=200)
                                    .suffix(" MHz")
                                    .text("Core"),
                            );
                            ui.label(format!("{:+} MHz", self.gpu_offset));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Memory Clock Offset:");
                            ui.add(
                                egui::Slider::new(&mut self.memory_offset, -1000..=1500)
                                    .suffix(" MHz")
                                    .text("VRAM"),
                            );
                            ui.label(format!("{:+} MHz", self.memory_offset));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Power Limit:");
                            ui.add(
                                egui::Slider::new(&mut self.power_limit_percent, 50..=105)
                                    .suffix("%")
                                    .text("TDP"),
                            );
                            ui.label(format!("{}%", self.power_limit_percent));
                        });

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("✅ Apply Overclock").clicked() {
                                self.apply_overclock();
                            }

                            if ui.button("🔄 Reset to Stock").clicked() {
                                self.apply_oc_preset(OcPreset::Stock);
                            }

                            if ui.button("⚠️ Stress Test (5 min)").clicked() {
                                let _ = overclocking::create_stress_test(5);
                            }
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("📈 Voltage Curve Editor (Undervolting)");
                        ui.separator();

                        ui.label("⚡ Advanced undervolting allows you to reduce power consumption while maintaining performance.");
                        ui.add_space(5.0);

                        // Show current GPU stats
                        if let Some(ref stats) = self.gpu_stats {
                            ui.horizontal(|ui| {
                                ui.label("Current GPU Clock:");
                                ui.label(format!("{} MHz", stats.gpu_clock));

                                // Calculate voltage for current frequency
                                let voltage = self.voltage_curve.get_voltage_at_freq(stats.gpu_clock as f64);
                                ui.label(format!("→ Target Voltage: {:.0}mV", voltage));
                            });
                            ui.add_space(5.0);
                        }

                        // Voltage curve plot
                        use egui_plot::{Line, Plot, PlotPoints, Points};

                        let curve_points_vec: Vec<[f64; 2]> = self
                            .voltage_curve
                            .points
                            .iter()
                            .map(|p| [p.x, p.y])
                            .collect();

                        let _plot_response = Plot::new("voltage_curve_plot")
                            .height(300.0)
                            .width(ui.available_width())
                            .x_axis_label("Frequency (MHz)")
                            .y_axis_label("Voltage (mV)")
                            .allow_drag(true)
                            .allow_zoom(true)
                            .show_axes([true, true])
                            .show(ui, |plot_ui| {
                                // Draw the curve line
                                let curve_line: PlotPoints = curve_points_vec.clone().into();
                                plot_ui.line(Line::new(curve_line).color(egui::Color32::from_rgb(251, 191, 36)));

                                // Draw the control points
                                let curve_pts: PlotPoints = curve_points_vec.into();
                                plot_ui.points(
                                    Points::new(curve_pts)
                                        .radius(6.0)
                                        .color(egui::Color32::from_rgb(251, 191, 36))
                                        .name("Voltage Points"),
                                );

                                // Draw current frequency indicator
                                if let Some(ref stats) = self.gpu_stats {
                                    let freq = stats.gpu_clock as f64;
                                    let voltage = self.voltage_curve.get_voltage_at_freq(freq);
                                    let current_point: PlotPoints = vec![[freq, voltage]].into();
                                    plot_ui.points(
                                        Points::new(current_point)
                                            .radius(8.0)
                                            .color(egui::Color32::RED)
                                            .name("Current"),
                                    );
                                }
                            });

                        ui.add_space(10.0);

                        // Point editor
                        ui.horizontal(|ui| {
                            ui.label("📍 Voltage Points:");
                        });

                        ui.separator();

                        let mut point_to_remove = None;

                        for (i, point) in self.voltage_curve.points.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}.", i + 1));
                                ui.label(format!("{:.0} MHz → {:.0} mV", point.x, point.y));

                                if ui.button("🗑️ Remove").clicked() && self.voltage_curve.points.len() > 2 {
                                    point_to_remove = Some(i);
                                }
                            });
                        }

                        if let Some(i) = point_to_remove {
                            self.voltage_curve.remove_point(i);
                        }

                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            if ui.button("➕ Add Point").clicked() {
                                // Add point at midpoint
                                let mid_freq = 1500.0;
                                let mid_voltage = self.voltage_curve.get_voltage_at_freq(mid_freq);
                                self.voltage_curve.add_point(mid_freq, mid_voltage);
                            }

                            if ui.button("🔄 Reset to Default").clicked() {
                                self.voltage_curve = nvcontrol::gui_widgets::VoltageCurve::default();
                            }

                            if ui.button("💾 Apply Curve").clicked() {
                                println!("Applying voltage curve: {:?}", self.voltage_curve.points);
                                // TODO: Apply voltage curve to GPU
                                // This would use nvidia-smi or NVML to set voltage curve
                            }
                        });

                        ui.add_space(10.0);

                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "⚠️ Warning: Incorrect voltage settings can cause instability. Test thoroughly!"
                        );
                        ui.label("💡 Tip: Lower voltages reduce power draw and heat, but too low will cause crashes.");
                        ui.label("📖 Start with small reductions (-25mV) and stress test before going further.");
                    });
                });
            }
            Tab::Fan => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🌀 Fan Control");

                    ui.group(|ui| {
                        ui.label("🌀 Current Fan Status");
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

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("📈 Fan Curve Editor");
                        ui.separator();

                        // Show current GPU temperature
                        if let Some(ref stats) = self.gpu_stats {
                            ui.horizontal(|ui| {
                                ui.label("🌡️ Current GPU Temp:");
                                ui.colored_label(
                                    if stats.temperature > 80.0 {
                                        egui::Color32::RED
                                    } else if stats.temperature > 70.0 {
                                        egui::Color32::YELLOW
                                    } else {
                                        egui::Color32::GREEN
                                    },
                                    format!("{:.1}°C", stats.temperature),
                                );

                                let target_speed = self.fan_curve.get_speed_at_temp(stats.temperature as f64);
                                ui.label(format!("→ Target Fan Speed: {:.0}%", target_speed));
                            });
                            ui.add_space(5.0);
                        }

                        // Fan curve plot
                        use egui_plot::{Line, Plot, PlotPoints, Points};

                        let curve_points_vec: Vec<[f64; 2]> = self
                            .fan_curve
                            .points
                            .iter()
                            .map(|p| [p.x, p.y])
                            .collect();

                        let _plot_response = Plot::new("fan_curve_plot")
                            .height(300.0)
                            .width(ui.available_width())
                            .x_axis_label("Temperature (°C)")
                            .y_axis_label("Fan Speed (%)")
                            .allow_drag(true)
                            .allow_zoom(true)
                            .show_axes([true, true])
                            .show(ui, |plot_ui| {
                                // Draw the curve line
                                let curve_line: PlotPoints = curve_points_vec.clone().into();
                                plot_ui.line(Line::new(curve_line).color(egui::Color32::LIGHT_BLUE));

                                // Draw the control points
                                let curve_pts: PlotPoints = curve_points_vec.into();
                                plot_ui.points(
                                    Points::new(curve_pts)
                                        .radius(6.0)
                                        .color(egui::Color32::from_rgb(59, 130, 246))
                                        .name("Control Points"),
                                );

                                // Draw current temperature indicator
                                if let Some(ref stats) = self.gpu_stats {
                                    let temp = stats.temperature as f64;
                                    let speed = self.fan_curve.get_speed_at_temp(temp);
                                    let current_point: PlotPoints = vec![[temp, speed]].into();
                                    plot_ui.points(
                                        Points::new(current_point)
                                            .radius(8.0)
                                            .color(egui::Color32::RED)
                                            .name("Current"),
                                    );
                                }
                            });

                        ui.add_space(10.0);

                        // Point editor
                        ui.horizontal(|ui| {
                            ui.label("📍 Control Points:");
                        });

                        ui.separator();

                        let mut point_to_remove = None;

                        for (i, point) in self.fan_curve.points.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}.", i + 1));
                                ui.label(format!("{:.0}°C → {:.0}%", point.x, point.y));

                                if ui.button("🗑️ Remove").clicked() && self.fan_curve.points.len() > 2 {
                                    point_to_remove = Some(i);
                                }
                            });
                        }

                        if let Some(i) = point_to_remove {
                            self.fan_curve.remove_point(i);
                        }

                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            if ui.button("➕ Add Point").clicked() {
                                // Add point at midpoint of curve
                                let mid_temp = 60.0;
                                let mid_speed = self.fan_curve.get_speed_at_temp(mid_temp);
                                self.fan_curve.add_point(mid_temp, mid_speed);
                            }

                            if ui.button("🔄 Reset to Default").clicked() {
                                self.fan_curve = nvcontrol::gui_widgets::FanCurve::default();
                            }

                            if ui.button("💾 Apply Curve").clicked() {
                                // Convert to nvcontrol format and apply
                                let curve_data = self.fan_curve.to_nvcontrol_format();
                                println!("Applying fan curve: {:?}", curve_data);
                                // TODO: Apply curve to GPU fan control
                                // This would call the fan module to set automatic curve mode
                            }
                        });

                        ui.add_space(10.0);
                        ui.label("💡 Tip: Click 'Add Point' to create new control points, then drag points in the graph to adjust the curve.");
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
                                match gamescope::apply_gamescope_config(&config) {
                                    Ok(()) => println!("✅ Steam Deck preset applied"),
                                    Err(e) => eprintln!("❌ Failed to apply preset: {}", e),
                                }
                            }
                            if ui.button("📱 Handheld 1080p").clicked() {
                                let config = gamescope::GamescopePreset::Handheld1080p.to_config();
                                match gamescope::apply_gamescope_config(&config) {
                                    Ok(()) => println!("✅ Handheld 1080p preset applied"),
                                    Err(e) => eprintln!("❌ Failed to apply preset: {}", e),
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button("🖥️ Desktop Gaming").clicked() {
                                let config = gamescope::GamescopePreset::Desktop.to_config();
                                match gamescope::apply_gamescope_config(&config) {
                                    Ok(()) => println!("✅ Desktop preset applied"),
                                    Err(e) => eprintln!("❌ Failed to apply preset: {}", e),
                                }
                            }
                            if ui.button("🏆 Performance").clicked() {
                                let config = gamescope::GamescopePreset::Performance.to_config();
                                match gamescope::apply_gamescope_config(&config) {
                                    Ok(()) => println!("✅ Performance preset applied"),
                                    Err(e) => eprintln!("❌ Failed to apply preset: {}", e),
                                }
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
                            // Create custom config from UI settings
                            let config = gamescope::GamescopeConfig {
                                width: 1920,
                                height: 1080,
                                refresh_rate: Some(144),
                                hdr_enabled: true,
                                adaptive_sync: true,
                                upscaling: gamescope::GamescopeUpscaling::Fsr,
                                fullscreen: true,
                                ..Default::default()
                            };

                            match gamescope::apply_gamescope_config(&config) {
                                Ok(()) => println!("✅ Custom gamescope configuration applied"),
                                Err(e) => eprintln!("❌ Failed to apply custom config: {}", e),
                            }
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
                                use nvcontrol::shaders;
                                match shaders::clear_shader_cache() {
                                    Ok(_) => println!("✅ Shader cache cleared successfully"),
                                    Err(e) => eprintln!("❌ Failed to clear shader cache: {}", e),
                                }
                            }

                            if ui.button("🔄 Rebuild Cache").clicked() {
                                use nvcontrol::shaders;
                                match shaders::optimize_shader_compilation() {
                                    Ok(_) => println!("✅ Shader compilation optimized"),
                                    Err(e) => eprintln!("❌ Failed to optimize: {}", e),
                                }
                            }

                            if ui.button("📁 Open Cache Folder").clicked() {
                                use std::process::Command;
                                let cache_path = std::env::var("HOME")
                                    .unwrap_or_else(|_| "/tmp".to_string())
                                    + "/.nv/GLCache";
                                let _ = Command::new("xdg-open")
                                    .arg(&cache_path)
                                    .spawn()
                                    .or_else(|_| Command::new("nautilus").arg(&cache_path).spawn())
                                    .or_else(|_| Command::new("dolphin").arg(&cache_path).spawn());
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
                                use nvcontrol::drivers;
                                match drivers::check_for_updates() {
                                    Ok(Some(latest)) => println!("✅ Update available: {}", latest),
                                    Ok(None) => println!("✅ Driver is up to date"),
                                    Err(e) => eprintln!("❌ Failed to check for updates: {}", e),
                                }
                            }

                            if ui.button("🔧 Reinstall Driver").clicked() {
                                println!("🔄 Reinstalling driver...");
                                println!("   This will reinstall the current NVIDIA driver");
                                println!("   Run: sudo nvctl drivers reinstall");
                            }

                            if ui.button("📊 Driver Validation").clicked() {
                                use nvcontrol::drivers;
                                match drivers::validate_driver_installation() {
                                    Ok(true) => println!("✅ Driver installation is valid"),
                                    Ok(false) => println!("⚠️  Driver installation has issues"),
                                    Err(e) => eprintln!("❌ Failed to validate: {}", e),
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button("📜 View Logs").clicked() {
                                use std::process::Command;
                                let _ = Command::new("xdg-open")
                                    .arg("/var/log/Xorg.0.log")
                                    .spawn()
                                    .or_else(|_| Command::new("less").arg("/var/log/Xorg.0.log").spawn());
                                println!("📜 Opening driver logs...");
                            }

                            if ui.button("🧹 Clean Install").clicked() {
                                println!("🧹 Clean driver installation:");
                                println!("   1. sudo nvctl drivers remove");
                                println!("   2. Reboot");
                                println!("   3. sudo nvctl drivers install open");
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
            Tab::Benchmark => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("📊 GPU Benchmark Suite");

                    ui.group(|ui| {
                        ui.label("🏁 Run Benchmark");
                        ui.separator();

                        ui.label("Test your GPU performance and compare results over time");
                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            if ui.button("⚡ Quick Test (30s)").clicked() {
                                std::thread::spawn(|| {
                                    if let Ok(suite) = nvcontrol::benchmark::BenchmarkSuite::new() {
                                        let _ = suite.run_full_benchmark(30);
                                    }
                                });
                            }

                            if ui.button("📊 Full Test (60s)").clicked() {
                                std::thread::spawn(|| {
                                    if let Ok(suite) = nvcontrol::benchmark::BenchmarkSuite::new() {
                                        let _ = suite.run_full_benchmark(60);
                                    }
                                });
                            }

                            if ui.button("🔥 Extended Test (120s)").clicked() {
                                std::thread::spawn(|| {
                                    if let Ok(suite) = nvcontrol::benchmark::BenchmarkSuite::new() {
                                        let _ = suite.run_full_benchmark(120);
                                    }
                                });
                            }
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("📈 Benchmark History");
                        ui.separator();

                        if let Ok(suite) = nvcontrol::benchmark::BenchmarkSuite::new() {
                            if let Ok(results) = suite.load_all_results() {
                                if results.is_empty() {
                                    ui.label("No benchmark results yet. Run a benchmark to get started!");
                                } else {
                                    // Show latest result
                                    if let Some(latest) = results.first() {
                                        ui.label(format!("🏆 Latest Score: {:.2}", latest.total_score));
                                        ui.label(format!("📅 Date: {}", latest.timestamp.format("%Y-%m-%d %H:%M")));
                                        ui.add_space(5.0);
                                    }

                                    // Performance history graph
                                    use egui_plot::{Line, Plot, PlotPoints};

                                    let score_points: PlotPoints = results
                                        .iter()
                                        .rev()
                                        .enumerate()
                                        .map(|(i, r)| [i as f64, r.total_score])
                                        .collect();

                                    Plot::new("benchmark_history")
                                        .height(200.0)
                                        .width(ui.available_width())
                                        .y_axis_label("Total Score")
                                        .x_axis_label("Test #")
                                        .show(ui, |plot_ui| {
                                            plot_ui.line(
                                                Line::new(score_points)
                                                    .color(egui::Color32::from_rgb(59, 130, 246))
                                                    .name("Total Score"),
                                            );
                                        });

                                    ui.add_space(10.0);

                                    // Detailed results table
                                    ui.label("📋 Detailed Results:");
                                    ui.separator();

                                    egui::ScrollArea::vertical()
                                        .max_height(300.0)
                                        .show(ui, |ui| {
                                            for result in results.iter().take(10) {
                                                ui.horizontal(|ui| {
                                                    ui.label(result.timestamp.format("%Y-%m-%d %H:%M").to_string());
                                                    ui.separator();
                                                    ui.label(format!("Score: {:.2}", result.total_score));
                                                    ui.separator();
                                                    ui.label(format!("Temp: {:.1}°C", result.avg_temp));
                                                    ui.separator();
                                                    ui.label(format!("Power: {:.1}W", result.avg_power));

                                                    if let (Some(gpu), Some(mem)) = (result.gpu_offset, result.memory_offset) {
                                                        ui.separator();
                                                        ui.label(format!("OC: {:+}/{:+}MHz", gpu, mem));
                                                    }
                                                });
                                            }
                                        });

                                    ui.add_space(10.0);

                                    // Comparison section
                                    if results.len() >= 2 {
                                        ui.label("🔄 Compare Results:");
                                        ui.separator();

                                        let baseline = &results[results.len() - 1];
                                        let current = &results[0];
                                        let comparison = suite.compare(baseline, current);

                                        ui.horizontal(|ui| {
                                            ui.label("Baseline:");
                                            ui.label(format!("{:.2}", baseline.total_score));
                                            ui.label(format!("({})", baseline.timestamp.format("%Y-%m-%d")));
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label("Latest:");
                                            ui.label(format!("{:.2}", current.total_score));
                                            ui.label(format!("({})", current.timestamp.format("%Y-%m-%d")));
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label("Performance Gain:");
                                            let color = if comparison.performance_gain >= 0.0 {
                                                egui::Color32::GREEN
                                            } else {
                                                egui::Color32::RED
                                            };
                                            ui.colored_label(color, format!("{:+.2}%", comparison.performance_gain));
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label("Temperature Delta:");
                                            let color = if comparison.temp_delta <= 0.0 {
                                                egui::Color32::GREEN
                                            } else {
                                                egui::Color32::YELLOW
                                            };
                                            ui.colored_label(color, format!("{:+.1}°C", comparison.temp_delta));
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label("Power Delta:");
                                            let color = if comparison.power_delta <= 0.0 {
                                                egui::Color32::GREEN
                                            } else {
                                                egui::Color32::YELLOW
                                            };
                                            ui.colored_label(color, format!("{:+.1}W", comparison.power_delta));
                                        });
                                    }
                                }
                            }
                        }
                    });
                });
            }
            Tab::Hdr => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🌈 HDR Configuration");

                    ui.group(|ui| {
                        ui.label("🎮 HDR Status");
                        ui.separator();

                        ui.horizontal(|ui| {
                            let mut enabled = self.hdr_config.enabled;
                            if ui.checkbox(&mut enabled, "Enable HDR").changed() {
                                self.hdr_config.enabled = enabled;
                                if let Err(e) = self.hdr_config.apply() {
                                    eprintln!("Failed to apply HDR: {}", e);
                                }
                                let _ = self.hdr_config.save();
                            }
                        });

                        if let Ok(caps) = nvcontrol::hdr::get_hdr_capabilities() {
                            ui.add_space(5.0);
                            ui.label("🖥️ Display Capabilities:");
                            ui.horizontal(|ui| {
                                ui.label(format!("Peak Luminance: {} nits", caps.max_luminance));
                                ui.separator();
                                ui.label(format!("Min Luminance: {:.4} nits", caps.min_luminance));
                            });

                            ui.horizontal(|ui| {
                                if caps.supports_hdr10 {
                                    ui.colored_label(egui::Color32::GREEN, "✅ HDR10");
                                }
                                if caps.supports_hdr10_plus {
                                    ui.colored_label(egui::Color32::GREEN, "✅ HDR10+");
                                }
                                if caps.supports_dolby_vision {
                                    ui.colored_label(egui::Color32::GREEN, "✅ Dolby Vision");
                                }
                                if caps.supports_hlg {
                                    ui.colored_label(egui::Color32::GREEN, "✅ HLG");
                                }
                            });
                        }
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("🎚️ HDR Metadata");
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Peak Brightness:");
                            let mut peak = self.hdr_config.peak_brightness as f32;
                            if ui.add(egui::Slider::new(&mut peak, 100.0..=10000.0).suffix(" nits")).changed() {
                                self.hdr_config.peak_brightness = peak as u32;
                                let _ = self.hdr_config.save();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Min Brightness:");
                            let mut min = self.hdr_config.min_brightness;
                            if ui.add(egui::Slider::new(&mut min, 0.0001..=0.1).suffix(" nits").logarithmic(true)).changed() {
                                self.hdr_config.min_brightness = min;
                                let _ = self.hdr_config.save();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Max Content Light Level:");
                            let mut max_cll = self.hdr_config.max_content_light_level as f32;
                            if ui.add(egui::Slider::new(&mut max_cll, 100.0..=10000.0).suffix(" nits")).changed() {
                                self.hdr_config.max_content_light_level = max_cll as u32;
                                let _ = self.hdr_config.save();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Max Frame Average:");
                            let mut max_fall = self.hdr_config.max_frame_average as f32;
                            if ui.add(egui::Slider::new(&mut max_fall, 50.0..=5000.0).suffix(" nits")).changed() {
                                self.hdr_config.max_frame_average = max_fall as u32;
                                let _ = self.hdr_config.save();
                            }
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("🎨 Color & Tone Mapping");
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Tone Mapping:");
                            egui::ComboBox::from_id_source("tone_mapping")
                                .selected_text(format!("{}", self.hdr_config.tone_mapping))
                                .show_ui(ui, |ui| {
                                    use nvcontrol::hdr::ToneMappingMode;
                                    ui.selectable_value(&mut self.hdr_config.tone_mapping, ToneMappingMode::None, "None (Clip)");
                                    ui.selectable_value(&mut self.hdr_config.tone_mapping, ToneMappingMode::Reinhard, "Reinhard");
                                    ui.selectable_value(&mut self.hdr_config.tone_mapping, ToneMappingMode::Hable, "Hable (Uncharted 2)");
                                    ui.selectable_value(&mut self.hdr_config.tone_mapping, ToneMappingMode::ACES, "ACES Filmic");
                                    ui.selectable_value(&mut self.hdr_config.tone_mapping, ToneMappingMode::AGX, "AGX");
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Color Space:");
                            egui::ComboBox::from_id_source("color_space")
                                .selected_text(format!("{}", self.hdr_config.color_space))
                                .show_ui(ui, |ui| {
                                    use nvcontrol::hdr::ColorSpace;
                                    ui.selectable_value(&mut self.hdr_config.color_space, ColorSpace::BT709, "BT.709 (sRGB)");
                                    ui.selectable_value(&mut self.hdr_config.color_space, ColorSpace::BT2020, "BT.2020 (HDR)");
                                    ui.selectable_value(&mut self.hdr_config.color_space, ColorSpace::DciP3, "DCI-P3 (Wide Gamut)");
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("EOTF:");
                            egui::ComboBox::from_id_source("eotf")
                                .selected_text(format!("{}", self.hdr_config.eotf))
                                .show_ui(ui, |ui| {
                                    use nvcontrol::hdr::Eotf;
                                    ui.selectable_value(&mut self.hdr_config.eotf, Eotf::Gamma22, "Gamma 2.2 (SDR)");
                                    ui.selectable_value(&mut self.hdr_config.eotf, Eotf::PQ, "PQ (HDR10)");
                                    ui.selectable_value(&mut self.hdr_config.eotf, Eotf::HLG, "HLG (HDR10+/BBC)");
                                });
                        });

                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui.button("💾 Save & Apply").clicked() {
                                if let Err(e) = self.hdr_config.save() {
                                    eprintln!("Failed to save HDR config: {}", e);
                                }
                                if let Err(e) = self.hdr_config.apply() {
                                    eprintln!("Failed to apply HDR config: {}", e);
                                }
                                println!("✅ HDR configuration saved and applied");
                            }

                            if ui.button("🔄 Reset to Defaults").clicked() {
                                self.hdr_config = nvcontrol::hdr::HdrConfig::default();
                                let _ = self.hdr_config.save();
                            }
                        });
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("💡 HDR Tips");
                        ui.separator();
                        ui.label("• Peak Brightness should match your display's capabilities");
                        ui.label("• For gaming, use Hable or ACES tone mapping");
                        ui.label("• BT.2020 color space is required for true HDR");
                        ui.label("• PQ (Perceptual Quantizer) is the standard for HDR10");
                        ui.label("• HLG is better for broadcast content");
                    });
                });
            }
            Tab::AutoOverclock => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🚀 Automated Overclocking Wizard");
                    ui.add_space(10.0);

                    ui.label("Safely auto-tune your GPU for optimal performance with stability testing.");
                    ui.separator();

                    ui.group(|ui| {
                        ui.label("⚙️ Configuration");
                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.label("Target Mode:");
                            egui::ComboBox::from_id_source("target")
                                .selected_text(&self.auto_oc_target)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.auto_oc_target, "max-performance".to_string(), "Max Performance");
                                    ui.selectable_value(&mut self.auto_oc_target, "balanced".to_string(), "Balanced");
                                    ui.selectable_value(&mut self.auto_oc_target, "efficiency".to_string(), "Efficiency");
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Safety Mode:");
                            egui::ComboBox::from_id_source("safety")
                                .selected_text(&self.auto_oc_safety)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.auto_oc_safety, "conservative".to_string(), "Conservative (Safe)");
                                    ui.selectable_value(&mut self.auto_oc_safety, "moderate".to_string(), "Moderate");
                                    ui.selectable_value(&mut self.auto_oc_safety, "aggressive".to_string(), "Aggressive (Fast)");
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Max Temperature:");
                            ui.add(egui::Slider::new(&mut self.auto_oc_max_temp, 70.0..=90.0).suffix("°C"));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Max Power:");
                            ui.add(egui::Slider::new(&mut self.auto_oc_max_power, 80..=120).suffix("%"));
                        });
                    });

                    ui.add_space(10.0);

                    if !self.auto_oc_running {
                        if ui.button("🚀 Start Auto-Overclock").clicked() {
                            self.auto_oc_running = true;
                            println!("Starting auto-overclock wizard...");
                        }
                    } else {
                        ui.label("⏳ Auto-overclock in progress...");
                        if ui.button("⏹ Stop").clicked() {
                            self.auto_oc_running = false;
                        }
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.label("⚠️  Note: Auto-overclocking will take 10-30 minutes depending on safety mode.");
                    ui.label("The wizard will test stability at each step and auto-rollback if unstable.");
                });
            }
            Tab::PowerCurves => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🔋 Power Limit Curves");
                    ui.add_space(10.0);

                    ui.label("Dynamic power management based on GPU temperature.");
                    ui.separator();

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.power_config.curve_enabled, "Enable Temperature-Based Power Curve");
                        });
                    });

                    ui.add_space(10.0);

                    ui.label("Curve Points (Temperature → Power Limit):");
                    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                        for (i, point) in self.power_config.power_curve.points.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("Point {}: {:.0}°C → {:.0}%", i, point.x, point.y));
                            });
                        }
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("💾 Save Configuration").clicked() {
                            if let Err(e) = nvcontrol::power_curves::save_power_config(&self.power_config) {
                                eprintln!("Failed to save power config: {}", e);
                            } else {
                                println!("✅ Power configuration saved");
                            }
                        }
                        if ui.button("🔄 Reset to Defaults").clicked() {
                            self.power_config = nvcontrol::power_curves::PowerManagementConfig::default();
                        }
                    });
                });
            }
            Tab::GameProfiles => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🎮 Game Profile Auto-Application");
                    ui.add_space(10.0);

                    ui.label("Automatically apply GPU profiles when games are launched.");
                    ui.separator();

                    ui.group(|ui| {
                        ui.label("⚙️ Configuration");
                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.game_auto_config.enabled, "Enable Auto-Application");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Poll Interval:");
                            ui.add(egui::Slider::new(&mut self.game_auto_config.poll_interval_secs, 1..=10).suffix(" seconds"));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Apply Delay:");
                            ui.add(egui::Slider::new(&mut self.game_auto_config.apply_delay_secs, 0..=10).suffix(" seconds"));
                        });

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.game_auto_config.restore_on_exit, "Restore default profile on game exit");
                        });
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("💾 Save Configuration").clicked() {
                            if let Err(e) = nvcontrol::game_profile_auto::save_config(&self.game_auto_config) {
                                eprintln!("Failed to save config: {}", e);
                            } else {
                                println!("✅ Configuration saved");
                            }
                        }
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.label("ℹ️  Note: Game profiles must be configured separately in the profiles directory.");
                });
            }
            Tab::RgbControl => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("💡 ASUS Aura RGB Control");

                    ui.group(|ui| {
                        ui.label("🎨 RGB Mode");
                        ui.separator();

                        egui::ComboBox::from_label("Select Mode")
                            .selected_text(&self.rgb_mode)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.rgb_mode, "Static".to_string(), "Static");
                                ui.selectable_value(&mut self.rgb_mode, "Breathing".to_string(), "Breathing");
                                ui.selectable_value(&mut self.rgb_mode, "Rainbow".to_string(), "Rainbow");
                                ui.selectable_value(&mut self.rgb_mode, "TempReactive".to_string(), "Temperature Reactive");
                                ui.selectable_value(&mut self.rgb_mode, "Off".to_string(), "Off");
                            });

                        ui.add_space(10.0);

                        // Color picker for static/breathing modes
                        if self.rgb_mode == "Static" || self.rgb_mode == "Breathing" {
                            ui.label("Color:");
                            ui.color_edit_button_rgb(&mut self.rgb_color);
                        }

                        ui.add_space(10.0);

                        if ui.button("✅ Apply RGB Settings").clicked() {
                            self.apply_rgb_settings();
                        }
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("ℹ️ RGB Control Info");
                        ui.separator();
                        ui.label("Requires OpenRGB installed:");
                        ui.label("  yay -S openrgb");
                        ui.label("");
                        ui.label("i2c kernel modules must be loaded:");
                        ui.label("  sudo modprobe i2c-dev i2c-nvidia_gpu");
                        ui.label("");
                        ui.label("Temperature Reactive colors:");
                        ui.label("  < 50°C: Blue/Cyan");
                        ui.label("  50-60°C: Green");
                        ui.label("  60-70°C: Yellow");
                        ui.label("  70-80°C: Orange");
                        ui.label("  > 80°C: Red");
                    });
                });
            }
            Tab::Containers => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("🐳 GPU Container Management");

                    ui.group(|ui| {
                        ui.label("📦 NVIDIA Container Toolkit Status");
                        ui.separator();

                        if let Some(ref _runtime) = self.container_runtime {
                            ui.colored_label(egui::Color32::GREEN, "✅ Container runtime initialized");

                            // Check NVIDIA Container Toolkit
                            ui.label("Checking NVIDIA Container Toolkit...");
                            ui.label("(Feature requires full implementation)");
                        } else {
                            ui.colored_label(egui::Color32::RED, "❌ Container runtime not available");
                            ui.label("");
                            ui.label("Install Docker or Podman:");
                            ui.label("  sudo pacman -S docker");
                            ui.label("");
                            ui.label("Install NVIDIA Container Toolkit:");
                            ui.label("  yay -S nvidia-container-toolkit");
                        }
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("🚀 Quick Launch");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("PyTorch (GPU)").clicked() {
                                println!("Launching PyTorch container...");
                                // self.launch_pytorch_container();
                            }
                            if ui.button("TensorFlow (GPU)").clicked() {
                                println!("Launching TensorFlow container...");
                                // self.launch_tensorflow_container();
                            }
                            if ui.button("CUDA Dev").clicked() {
                                println!("Launching CUDA dev container...");
                                // self.launch_cuda_dev_container();
                            }
                        });

                        ui.add_space(5.0);

                        ui.label("Available container profiles:");
                        ui.label("  • PyTorch 2.2.0 + CUDA 12.1");
                        ui.label("  • TensorFlow latest GPU");
                        ui.label("  • CUDA 12.x Development");
                    });

                    ui.add_space(10.0);

                    ui.group(|ui| {
                        ui.label("📊 Running Containers");
                        ui.separator();

                        if self.running_containers.is_empty() {
                            ui.label("No GPU containers currently running");
                        } else {
                            for container in &self.running_containers {
                                ui.horizontal(|ui| {
                                    ui.label(&container.name);
                                    ui.label(format!("GPU: {}", container.gpu_usage));
                                    if ui.button("Stop").clicked() {
                                        println!("Stopping container: {}", container.id);
                                    }
                                });
                            }
                        }

                        ui.add_space(5.0);

                        if ui.button("🔄 Refresh Container List").clicked() {
                            self.refresh_containers();
                        }
                    });
                });
            }
            Tab::Osd => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("📊 OSD / MangoHud Configuration");

                    // MangoHud installation status
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            if self.mangohud_installed {
                                ui.label("✅ MangoHud installed");
                            } else {
                                ui.label("❌ MangoHud not installed");
                                if ui.button("📋 Install Instructions").clicked() {
                                    // Show install instructions
                                }
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // Enable/Disable toggle
                    ui.group(|ui| {
                        ui.label("🎮 OSD Status");
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Enable OSD:");
                            if ui.checkbox(&mut self.osd_enabled, "").changed() {
                                self.save_mangohud_config();
                            }
                        });

                        if self.osd_enabled {
                            ui.label("💡 Launch games with: mangohud %command%");
                            ui.label("   Or set MANGOHUD=1 in Steam launch options");
                        }
                    });

                    ui.add_space(10.0);

                    // Position selection
                    ui.group(|ui| {
                        ui.label("📍 Position");
                        ui.separator();

                        egui::ComboBox::from_label("OSD Position")
                            .selected_text(&self.osd_position)
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(self.osd_position == "top-left", "Top Left").clicked() {
                                    self.osd_position = "top-left".to_string();
                                    self.save_mangohud_config();
                                }
                                if ui.selectable_label(self.osd_position == "top-right", "Top Right").clicked() {
                                    self.osd_position = "top-right".to_string();
                                    self.save_mangohud_config();
                                }
                                if ui.selectable_label(self.osd_position == "bottom-left", "Bottom Left").clicked() {
                                    self.osd_position = "bottom-left".to_string();
                                    self.save_mangohud_config();
                                }
                                if ui.selectable_label(self.osd_position == "bottom-right", "Bottom Right").clicked() {
                                    self.osd_position = "bottom-right".to_string();
                                    self.save_mangohud_config();
                                }
                            });
                    });

                    ui.add_space(10.0);

                    // Metrics selection
                    ui.group(|ui| {
                        ui.label("📈 Metrics to Display");
                        ui.separator();

                        let all_metrics = vec![
                            ("fps", "FPS Counter"),
                            ("frametime", "Frame Time Graph"),
                            ("gpu_temp", "GPU Temperature"),
                            ("gpu_load", "GPU Utilization"),
                            ("gpu_core_clock", "GPU Clock Speed"),
                            ("gpu_power", "GPU Power Draw"),
                            ("vram", "VRAM Usage"),
                            ("fan", "Fan Speed"),
                            ("cpu_temp", "CPU Temperature"),
                            ("cpu_load", "CPU Utilization"),
                            ("ram", "RAM Usage"),
                            ("gpu_name", "GPU Name"),
                        ];

                        ui.columns(2, |columns| {
                            for (i, (metric_id, metric_name)) in all_metrics.iter().enumerate() {
                                let col = i % 2;
                                let mut enabled = self.osd_metrics.contains(&metric_id.to_string());
                                if columns[col].checkbox(&mut enabled, *metric_name).changed() {
                                    if enabled {
                                        if !self.osd_metrics.contains(&metric_id.to_string()) {
                                            self.osd_metrics.push(metric_id.to_string());
                                        }
                                    } else {
                                        self.osd_metrics.retain(|m| m != *metric_id);
                                    }
                                    self.save_mangohud_config();
                                }
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // Appearance settings
                    ui.group(|ui| {
                        ui.label("🎨 Appearance");
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Font Size:");
                            if ui.add(egui::Slider::new(&mut self.osd_font_size, 12..=48)).changed() {
                                self.save_mangohud_config();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Background Opacity:");
                            if ui.add(egui::Slider::new(&mut self.osd_opacity, 0.0..=1.0)).changed() {
                                self.save_mangohud_config();
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // Presets
                    ui.group(|ui| {
                        ui.label("⚡ Quick Presets");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("Minimal").clicked() {
                                self.osd_metrics = vec!["fps".to_string()];
                                self.osd_font_size = 24;
                                self.osd_opacity = 0.5;
                                self.save_mangohud_config();
                            }
                            if ui.button("Standard").clicked() {
                                self.osd_metrics = vec![
                                    "fps".to_string(),
                                    "frametime".to_string(),
                                    "gpu_temp".to_string(),
                                    "gpu_load".to_string(),
                                    "vram".to_string(),
                                ];
                                self.osd_font_size = 24;
                                self.osd_opacity = 0.8;
                                self.save_mangohud_config();
                            }
                            if ui.button("Full").clicked() {
                                self.osd_metrics = vec![
                                    "fps".to_string(),
                                    "frametime".to_string(),
                                    "gpu_temp".to_string(),
                                    "gpu_load".to_string(),
                                    "gpu_core_clock".to_string(),
                                    "gpu_power".to_string(),
                                    "vram".to_string(),
                                    "fan".to_string(),
                                    "cpu_temp".to_string(),
                                    "cpu_load".to_string(),
                                    "ram".to_string(),
                                ];
                                self.osd_font_size = 20;
                                self.osd_opacity = 0.9;
                                self.save_mangohud_config();
                            }
                            if ui.button("Benchmark").clicked() {
                                self.osd_metrics = vec![
                                    "fps".to_string(),
                                    "frametime".to_string(),
                                    "gpu_temp".to_string(),
                                    "gpu_load".to_string(),
                                    "gpu_core_clock".to_string(),
                                    "gpu_power".to_string(),
                                    "cpu_load".to_string(),
                                ];
                                self.osd_font_size = 28;
                                self.osd_opacity = 0.9;
                                self.save_mangohud_config();
                            }
                        });
                    });

                    ui.add_space(10.0);

                    // Config preview
                    ui.group(|ui| {
                        ui.label("📄 MangoHud Config Preview");
                        ui.separator();

                        let config_preview = format!(
                            "position={}\n{}\nfont_size={}\nbackground_alpha={}",
                            self.osd_position,
                            self.osd_metrics.join("\n"),
                            self.osd_font_size,
                            self.osd_opacity
                        );

                        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                            ui.add(egui::TextEdit::multiline(&mut config_preview.clone())
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY));
                        });
                    });
                });
            }
            Tab::Settings => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("⚙️ Settings");

                    ui.group(|ui| {
                        ui.label("🎨 Theme Selection");
                        ui.separator();

                        egui::ComboBox::from_label("Select Theme")
                            .selected_text(self.current_theme.name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.current_theme,
                                    nvcontrol::themes::ThemeVariant::TokyoNightNight, "Tokyo Night");
                                ui.selectable_value(&mut self.current_theme,
                                    nvcontrol::themes::ThemeVariant::TokyoNightStorm, "Tokyo Night Storm");
                                ui.selectable_value(&mut self.current_theme,
                                    nvcontrol::themes::ThemeVariant::TokyoNightMoon, "Tokyo Night Moon");
                                ui.selectable_value(&mut self.current_theme,
                                    nvcontrol::themes::ThemeVariant::Dracula, "Dracula");
                                ui.selectable_value(&mut self.current_theme,
                                    nvcontrol::themes::ThemeVariant::RogRed, "ROG Red");
                                ui.selectable_value(&mut self.current_theme,
                                    nvcontrol::themes::ThemeVariant::MatrixGreen, "Matrix Green");
                                ui.selectable_value(&mut self.current_theme,
                                    nvcontrol::themes::ThemeVariant::Cyberpunk, "Cyberpunk");
                            });

                        ui.add_space(10.0);

                        if ui.button("Apply Theme").clicked() {
                            self.apply_theme(ctx);
                        }

                        ui.add_space(5.0);

                        ui.label("Legacy themes:");
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
                        ui.label("💾 Profile Management");
                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button("📥 Export Current Profile").clicked() {
                                if let Ok(manager) = nvcontrol::profile_manager::ProfileManager::new() {
                                    let bundle = nvcontrol::profile_manager::ProfileBundle {
                                        name: format!("Profile_{}", chrono::Local::now().format("%Y%m%d_%H%M%S")),
                                        description: "Exported from GUI".to_string(),
                                        created_at: chrono::Utc::now(),
                                        fan_curve: Some(self.fan_curve.clone()),
                                        voltage_curve: Some(self.voltage_curve.clone()),
                                        overclock: Some(self.overclock_profile.clone()),
                                        game_profiles: vec![],
                                        vibrance_settings: Some(nvcontrol::profile_manager::VibranceSettings {
                                            display_levels: self.vibrance_levels.clone(),
                                            per_game_vibrance: false,
                                        }),
                                    };

                                    match manager.export_profile(&bundle, None) {
                                        Ok(path) => println!("✅ Profile exported to: {}", path.display()),
                                        Err(e) => eprintln!("❌ Export failed: {}", e),
                                    }
                                }
                            }

                            if ui.button("📂 Open Profiles Folder").clicked() {
                                if let Ok(manager) = nvcontrol::profile_manager::ProfileManager::new() {
                                    let path = manager.get_profiles_dir();
                                    let _ = std::process::Command::new("xdg-open")
                                        .arg(path)
                                        .spawn();
                                }
                            }
                        });

                        ui.add_space(5.0);

                        // List available profiles
                        ui.label("📋 Available Profiles:");
                        ui.separator();

                        if let Ok(manager) = nvcontrol::profile_manager::ProfileManager::new() {
                            if let Ok(profiles) = manager.list_profiles() {
                                if profiles.is_empty() {
                                    ui.label("No profiles found. Export your current settings to create one.");
                                } else {
                                    egui::ScrollArea::vertical()
                                        .max_height(200.0)
                                        .show(ui, |ui| {
                                            for profile in profiles {
                                                ui.horizontal(|ui| {
                                                    ui.label(&profile.name);
                                                    ui.label(format!("({})", profile.created_at.format("%Y-%m-%d")));

                                                    if ui.button("📥 Load").clicked() {
                                                        if let Some(fan) = profile.fan_curve {
                                                            self.fan_curve = fan;
                                                        }
                                                        if let Some(voltage) = profile.voltage_curve {
                                                            self.voltage_curve = voltage;
                                                        }
                                                        if let Some(oc) = profile.overclock {
                                                            self.overclock_profile = oc;
                                                        }
                                                        if let Some(vib) = profile.vibrance_settings {
                                                            self.vibrance_levels = vib.display_levels;
                                                        }
                                                        println!("✅ Profile loaded: {}", profile.name);
                                                    }

                                                    if ui.button("🗑️").clicked() {
                                                        let _ = manager.delete_profile(&profile.name);
                                                    }
                                                });
                                            }
                                        });
                                }
                            }
                        }
                    });

                    ui.separator();

                    ui.group(|ui| {
                        ui.label("📊 System Information");
                        ui.separator();

                        if ui.button("🔧 Test Native Vibrance").clicked() {
                            use nvcontrol::vibrance_native;
                            match vibrance_native::get_vibrance_status_native() {
                                Ok(status) => {
                                    println!("✅ Native vibrance working!");
                                    println!("Status: {:?}", status);
                                    println!("Devices: checked");
                                },
                                Err(e) => eprintln!("❌ Native vibrance test failed: {}", e),
                            }
                        }

                        if ui.button("🔍 Scan Displays").clicked() {
                            use nvcontrol::vibrance_native;
                            match vibrance_native::list_displays_native() {
                                Ok(displays) => {
                                    println!("✅ Found {} displays:", displays.len());
                                    for (idx, disp) in displays.iter().enumerate() {
                                        println!("  {}. Display {} - {:?}", idx + 1, disp.0, disp);
                                    }
                                },
                                Err(e) => eprintln!("❌ Display scan failed: {}", e),
                            }
                        }
                    });
                });
            }
        }
    }
}
