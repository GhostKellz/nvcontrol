use crate::NvResult;
/// Interactive CLI Mode for nvcontrol
///
/// Menu-driven interface for users who prefer guided workflows
use crate::nvml_backend::SharedNvmlBackend;
use console::{Key, Term, style};

pub struct InteractiveCli {
    term: Term,
    backend: SharedNvmlBackend,
}

impl InteractiveCli {
    pub fn new(backend: SharedNvmlBackend) -> Self {
        Self {
            term: Term::stdout(),
            backend,
        }
    }

    pub fn run(&mut self) -> NvResult<()> {
        loop {
            self.term.clear_screen().ok();
            self.show_main_menu()?;

            match self.get_choice(1, 8)? {
                1 => self.gpu_info_menu()?,
                2 => self.monitoring_menu()?,
                3 => self.overclocking_menu()?,
                4 => self.fan_control_menu()?,
                5 => self.profile_menu()?,
                6 => self.gaming_menu()?,
                7 => self.settings_menu()?,
                8 => {
                    self.term.write_line("👋 Goodbye!").ok();
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn show_main_menu(&mut self) -> NvResult<()> {
        let menu = format!(
            r#"
{}
║      {}           ║
╠════════════════════════════════════════════════════╣
║                                                    ║
║  {}  GPU Information                             ║
║  {}  Monitor (Live TUI)                          ║
║  {}  Overclocking                                ║
║  {}  Fan Control                                 ║
║  {}  Profiles                                    ║
║  {}  Gaming Optimization                         ║
║  {}  Settings                                    ║
║  {}  Exit                                        ║
║                                                    ║
╚════════════════════════════════════════════════════╝

Select option (1-8): "#,
            style("╔════════════════════════════════════════════════════╗").bold(),
            style("NVIDIA Control Center").cyan().bold(),
            style("[1]").green().bold(),
            style("[2]").green().bold(),
            style("[3]").green().bold(),
            style("[4]").green().bold(),
            style("[5]").green().bold(),
            style("[6]").green().bold(),
            style("[7]").green().bold(),
            style("[8]").red().bold(),
        );

        self.term.write_line(&menu).ok();
        Ok(())
    }

    fn gpu_info_menu(&mut self) -> NvResult<()> {
        self.term.clear_screen().ok();
        self.term
            .write_line(&format!(
                "\n{}\n",
                style("📊 GPU Information").cyan().bold()
            ))
            .ok();

        // Call GPU info command
        if let Err(e) =
            crate::gpu::get_gpu_info_with_format(crate::gpu::OutputFormat::Human, &self.backend)
        {
            self.term.write_line(&format!("❌ Error: {}", e)).ok();
        }

        self.wait_for_key()?;
        Ok(())
    }

    fn monitoring_menu(&mut self) -> NvResult<()> {
        self.term.clear_screen().ok();
        self.term
            .write_line(&format!(
                "\n{}\n",
                style("📊 Monitoring Options").cyan().bold()
            ))
            .ok();

        self.term.write_line("[1] Launch TUI Monitor").ok();
        self.term.write_line("[2] Quick Stats (1 second)").ok();
        self.term.write_line("[3] Back to Main Menu").ok();
        self.term.write_str("\nSelect: ").ok();
        self.term.flush().ok();

        match self.get_choice(1, 3)? {
            1 => {
                // Launch TUI
                self.term.write_line("\n📊 Launching TUI monitor...").ok();
                if let Err(e) = crate::tui::TuiApp::new().run() {
                    self.term.write_line(&format!("❌ TUI error: {}", e)).ok();
                    self.wait_for_key()?;
                }
            }
            2 => {
                // Quick stats
                if let Err(e) = crate::monitoring::live_gpu_watch(1, 1, &self.backend) {
                    self.term
                        .write_line(&format!("❌ Monitor failed: {}", e))
                        .ok();
                    self.wait_for_key()?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn overclocking_menu(&mut self) -> NvResult<()> {
        self.term.clear_screen().ok();
        self.term
            .write_line(&format!("\n{}\n", style("⚡ Overclocking").cyan().bold()))
            .ok();

        self.term.write_line("[1] Show Current Clocks").ok();
        self.term.write_line("[2] Apply Preset (Performance)").ok();
        self.term.write_line("[3] Reset to Stock").ok();
        self.term.write_line("[4] Back to Main Menu").ok();
        self.term.write_str("\nSelect: ").ok();
        self.term.flush().ok();

        match self.get_choice(1, 4)? {
            1 => {
                // Show current GPU capabilities/clocks
                match crate::overclocking::get_gpu_capabilities() {
                    Ok(caps) => {
                        self.term
                            .write_line(&format!(
                                "Max GPU Clock Offset: +{} MHz",
                                caps.max_gpu_clock_offset
                            ))
                            .ok();
                        self.term
                            .write_line(&format!(
                                "Min GPU Clock Offset: {} MHz",
                                caps.min_gpu_clock_offset
                            ))
                            .ok();
                        self.term
                            .write_line(&format!(
                                "Max Memory Clock Offset: +{} MHz",
                                caps.max_memory_clock_offset
                            ))
                            .ok();
                        self.term
                            .write_line(&format!(
                                "Voltage Control: {}",
                                if caps.supports_voltage_control {
                                    "Yes"
                                } else {
                                    "No"
                                }
                            ))
                            .ok();
                        self.term
                            .write_line(&format!(
                                "Power Limit Range: {}% - {}%",
                                caps.min_power_limit, caps.max_power_limit
                            ))
                            .ok();
                        self.term
                            .write_line(&format!(
                                "Default Temp Limit: {}°C",
                                caps.default_temp_limit
                            ))
                            .ok();
                    }
                    Err(e) => {
                        self.term.write_line(&format!("❌ Error: {}", e)).ok();
                    }
                }
                self.wait_for_key()?;
            }
            2 => {
                self.term
                    .write_line("\n⚠️  Applying performance preset...")
                    .ok();
                let profile = crate::overclocking::OverclockProfile {
                    name: "Performance".to_string(),
                    gpu_clock_offset: 100,
                    memory_clock_offset: 200,
                    voltage_offset: 0,
                    power_limit: 100,
                    temp_limit: 83,
                    fan_curve: vec![(30, 30), (50, 50), (70, 80), (85, 100)],
                };
                if let Err(e) = crate::overclocking::apply_overclock_profile(&profile) {
                    self.term.write_line(&format!("❌ Error: {}", e)).ok();
                } else {
                    self.term.write_line("✅ Performance preset applied").ok();
                }
                self.wait_for_key()?;
            }
            3 => {
                self.term
                    .write_line("\n🔄 Resetting to stock clocks...")
                    .ok();
                let stock_profile = crate::overclocking::OverclockProfile {
                    name: "Stock".to_string(),
                    gpu_clock_offset: 0,
                    memory_clock_offset: 0,
                    voltage_offset: 0,
                    power_limit: 100,
                    temp_limit: 83,
                    fan_curve: vec![(30, 20), (50, 40), (70, 70), (85, 100)],
                };
                if let Err(e) = crate::overclocking::apply_overclock_profile(&stock_profile) {
                    self.term.write_line(&format!("❌ Error: {}", e)).ok();
                } else {
                    self.term.write_line("✅ Reset to stock").ok();
                }
                self.wait_for_key()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn fan_control_menu(&mut self) -> NvResult<()> {
        self.term.clear_screen().ok();
        self.term
            .write_line(&format!("\n{}\n", style("🌀 Fan Control").cyan().bold()))
            .ok();

        self.term.write_line("[1] Set to Auto").ok();
        self.term.write_line("[2] Set Manual Speed (%)").ok();
        self.term.write_line("[3] Apply Fan Curve").ok();
        self.term.write_line("[4] Back to Main Menu").ok();
        self.term.write_str("\nSelect: ").ok();
        self.term.flush().ok();

        match self.get_choice(1, 4)? {
            1 => {
                if let Err(e) = crate::fan::reset_fan_to_auto(0) {
                    self.term.write_line(&format!("❌ Error: {}", e)).ok();
                } else {
                    self.term.write_line("✅ Fan control set to Auto").ok();
                }
                self.wait_for_key()?;
            }
            2 => {
                self.term.write_line("\nEnter fan speed (0-100%): ").ok();
                self.term.flush().ok();
                if let Ok(line) = self.term.read_line() {
                    if let Ok(speed) = line.trim().parse::<u8>() {
                        if speed <= 100 {
                            if let Err(e) = crate::fan::set_fan_speed(0, speed) {
                                self.term.write_line(&format!("❌ Error: {}", e)).ok();
                            } else {
                                self.term
                                    .write_line(&format!("✅ Fan speed set to {}%", speed))
                                    .ok();
                            }
                        } else {
                            self.term.write_line("❌ Speed must be 0-100").ok();
                        }
                    }
                }
                self.wait_for_key()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn profile_menu(&mut self) -> NvResult<()> {
        self.term.clear_screen().ok();
        self.term
            .write_line(&format!("\n{}\n", style("📋 Profiles").cyan().bold()))
            .ok();

        self.term.write_line("[1] List Profiles").ok();
        self.term.write_line("[2] Apply Profile").ok();
        self.term.write_line("[3] Create Profile").ok();
        self.term.write_line("[4] Back to Main Menu").ok();
        self.term.write_str("\nSelect: ").ok();
        self.term.flush().ok();

        if self.get_choice(1, 4)? == 1 {
            let profiles = crate::profiles::list_profiles();
            if profiles.is_empty() {
                self.term.write_line("No profiles found.").ok();
            } else {
                self.term.write_line("Available profiles:").ok();
                for profile in profiles {
                    self.term.write_line(&format!("  - {}", profile)).ok();
                }
            }
            self.wait_for_key()?;
        }

        Ok(())
    }

    fn gaming_menu(&mut self) -> NvResult<()> {
        self.term.clear_screen().ok();
        self.term
            .write_line(&format!(
                "\n{}\n",
                style("🎮 Gaming Optimization").cyan().bold()
            ))
            .ok();

        self.term.write_line("[1] Enable Gaming Mode").ok();
        self.term.write_line("[2] Disable Gaming Mode").ok();
        self.term.write_line("[3] Optimize Latency").ok();
        self.term.write_line("[4] Back to Main Menu").ok();
        self.term.write_str("\nSelect: ").ok();
        self.term.flush().ok();

        match self.get_choice(1, 4)? {
            1 => {
                if let Err(e) = crate::latency::optimize_latency() {
                    self.term.write_line(&format!("❌ Error: {}", e)).ok();
                } else {
                    self.term.write_line("✅ Gaming mode enabled").ok();
                }
                self.wait_for_key()?;
            }
            3 => {
                if let Err(e) = crate::latency::optimize_latency() {
                    self.term.write_line(&format!("❌ Error: {}", e)).ok();
                } else {
                    self.term.write_line("✅ Latency optimized").ok();
                }
                self.wait_for_key()?;
            }
            _ => {}
        }

        Ok(())
    }

    fn settings_menu(&mut self) -> NvResult<()> {
        self.term.clear_screen().ok();
        self.term
            .write_line(&format!("\n{}\n", style("⚙️  Settings").cyan().bold()))
            .ok();

        self.term
            .write_line("Settings functionality coming soon!")
            .ok();
        self.wait_for_key()?;
        Ok(())
    }

    fn get_choice(&mut self, min: u32, max: u32) -> NvResult<u32> {
        loop {
            if let Ok(key) = self.term.read_key() {
                match key {
                    Key::Char(c) => {
                        if let Some(digit) = c.to_digit(10) {
                            if digit >= min && digit <= max {
                                return Ok(digit);
                            }
                        }
                    }
                    Key::Escape => return Ok(max), // Treat ESC as "back/exit"
                    _ => {}
                }
            }
        }
    }

    fn wait_for_key(&mut self) -> NvResult<()> {
        self.term.write_line("\nPress any key to continue...").ok();
        self.term.read_key().ok();
        Ok(())
    }
}
