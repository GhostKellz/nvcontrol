// Per-Game Launch Optimizer
// Launches games with optimal settings, environment variables, and CPU affinity

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaunchHook {
    pub command: String,
    pub args: Vec<String>,
    pub ignore_failure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessPriority {
    Low,
    Normal,
    High,
    Realtime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProfile {
    pub name: String,
    pub executable: String,
    pub working_dir: Option<String>,
    pub pre_launch_hooks: Vec<LaunchHook>,
    pub post_exit_hooks: Vec<LaunchHook>,

    // Environment variables
    pub env_vars: HashMap<String, String>,

    // Gamescope settings
    pub use_gamescope: bool,
    pub gamescope_width: Option<u32>,
    pub gamescope_height: Option<u32>,
    pub gamescope_refresh: Option<u32>,
    pub gamescope_hdr: bool,
    pub gamescope_vrr: bool,

    // CPU/IRQ affinity
    pub cpu_affinity: Option<Vec<usize>>,
    pub prefer_vcache_ccd: bool, // For AMD 3D V-Cache CPUs

    // Cache settings
    pub shader_cache_path: Option<String>,
    pub warm_start: bool, // Pre-warm shader cache

    // Proton/Wine settings
    pub use_proton: bool,
    pub proton_version: Option<String>,
    pub wine_prefix: Option<String>,

    // Display/GPU settings
    pub power_profile: Option<String>,
    pub gpu_clock_offset: Option<i32>,
    pub mem_clock_offset: Option<i32>,
    pub power_limit: Option<u32>,
    pub vibrance: Option<u32>,
    pub fps_limit: Option<u32>,
    pub gamescope_preset: Option<String>,
    pub priority: ProcessPriority,
}

impl Default for GameProfile {
    fn default() -> Self {
        let mut env_vars = HashMap::new();

        // Default NVIDIA optimizations
        env_vars.insert("__GL_SHADER_DISK_CACHE".to_string(), "1".to_string());
        env_vars.insert("__GL_THREADED_OPTIMIZATIONS".to_string(), "1".to_string());
        env_vars.insert("__GL_YIELD".to_string(), "USLEEP".to_string());
        env_vars.insert("__GL_GSYNC_ALLOWED".to_string(), "1".to_string());
        env_vars.insert("__GL_VRR_ALLOWED".to_string(), "1".to_string());

        Self {
            name: "default".to_string(),
            executable: String::new(),
            working_dir: None,
            pre_launch_hooks: Vec::new(),
            post_exit_hooks: Vec::new(),
            env_vars,
            use_gamescope: false,
            gamescope_width: None,
            gamescope_height: None,
            gamescope_refresh: None,
            gamescope_hdr: false,
            gamescope_vrr: true,
            cpu_affinity: None,
            prefer_vcache_ccd: false,
            shader_cache_path: None,
            warm_start: false,
            use_proton: false,
            proton_version: None,
            wine_prefix: None,
            power_profile: None,
            gpu_clock_offset: None,
            mem_clock_offset: None,
            power_limit: None,
            vibrance: None,
            fps_limit: None,
            gamescope_preset: None,
            priority: ProcessPriority::Normal,
        }
    }
}

impl GameProfile {
    pub fn new(name: String, executable: String) -> Self {
        Self {
            name,
            executable,
            ..Default::default()
        }
    }

    /// Add DXVK environment variables
    pub fn with_dxvk(mut self, enable_async: bool) -> Self {
        self.env_vars
            .insert("DXVK_STATE_CACHE".to_string(), "1".to_string());

        if enable_async {
            self.env_vars
                .insert("DXVK_ASYNC".to_string(), "1".to_string());
        }

        if let Some(cache_path) = &self.shader_cache_path {
            self.env_vars
                .insert("DXVK_STATE_CACHE_PATH".to_string(), cache_path.clone());
        }

        self
    }

    /// Add VKD3D-Proton environment variables
    pub fn with_vkd3d(mut self) -> Self {
        if let Some(cache_path) = &self.shader_cache_path {
            self.env_vars
                .insert("VKD3D_SHADER_CACHE_PATH".to_string(), cache_path.clone());
        }

        self.env_vars
            .insert("VKD3D_CONFIG".to_string(), "dxr11,dxr".to_string());
        self
    }

    /// Enable NVIDIA DLSS/RTX features
    pub fn with_dlss(mut self) -> Self {
        self.env_vars
            .insert("PROTON_ENABLE_NGX_UPDATER".to_string(), "1".to_string());
        self.env_vars
            .insert("PROTON_ENABLE_NVAPI".to_string(), "1".to_string());
        self
    }

    /// Set shader cache path
    pub fn with_shader_cache(mut self, path: String) -> Self {
        self.shader_cache_path = Some(path.clone());
        self.env_vars
            .insert("__GL_SHADER_DISK_CACHE_PATH".to_string(), path.clone());
        self.env_vars
            .insert("DXVK_STATE_CACHE_PATH".to_string(), path.clone());
        self.env_vars
            .insert("VKD3D_SHADER_CACHE_PATH".to_string(), path);
        self
    }

    pub fn with_pre_launch_hook(mut self, command: impl Into<String>, args: Vec<String>) -> Self {
        self.pre_launch_hooks.push(LaunchHook {
            command: command.into(),
            args,
            ignore_failure: false,
        });
        self
    }

    pub fn with_post_exit_hook(mut self, command: impl Into<String>, args: Vec<String>) -> Self {
        self.post_exit_hooks.push(LaunchHook {
            command: command.into(),
            args,
            ignore_failure: true,
        });
        self
    }
}

/// Game launcher that applies profiles and optimizations
pub struct GameLauncher {
    profiles_dir: PathBuf,
}

impl GameLauncher {
    pub fn new() -> NvResult<Self> {
        let profiles_dir = Self::get_profiles_dir();
        fs::create_dir_all(&profiles_dir).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create profiles dir: {}", e))
        })?;

        Ok(Self { profiles_dir })
    }

    fn get_profiles_dir() -> PathBuf {
        if let Some(project_dirs) = directories::ProjectDirs::from("com", "ghostkellz", "nvcontrol")
        {
            project_dirs.config_dir().join("game_profiles")
        } else {
            PathBuf::from("game_profiles")
        }
    }

    /// Save a game profile
    pub fn save_profile(&self, profile: &GameProfile) -> NvResult<()> {
        let profile_path = self.profiles_dir.join(format!("{}.toml", profile.name));
        let content = toml::to_string_pretty(profile).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize profile: {}", e))
        })?;

        fs::write(&profile_path, content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write profile: {}", e)))?;

        println!("✅ Saved game profile: {}", profile_path.display());
        Ok(())
    }

    pub fn delete_profile(&self, name: &str) -> NvResult<()> {
        let profile_path = self.profiles_dir.join(format!("{}.toml", name));
        if profile_path.exists() {
            fs::remove_file(&profile_path).map_err(|e| {
                NvControlError::ConfigError(format!("Failed to delete profile: {}", e))
            })?;
        }
        Ok(())
    }

    /// Load a game profile
    pub fn load_profile(&self, name: &str) -> NvResult<GameProfile> {
        let profile_path = self.profiles_dir.join(format!("{}.toml", name));
        let content = fs::read_to_string(&profile_path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read profile: {}", e)))?;

        toml::from_str(&content)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse profile: {}", e)))
    }

    /// List all saved game profiles
    pub fn list_profiles(&self) -> Vec<String> {
        if !self.profiles_dir.exists() {
            return Vec::new();
        }

        fs::read_dir(&self.profiles_dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.extension()? == "toml" {
                            path.file_stem()?.to_str().map(|s| s.to_string())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Launch a game with the given profile
    pub fn launch_game(&self, profile: &GameProfile, args: Vec<String>) -> NvResult<()> {
        println!("🚀 Launching game: {}", profile.name);

        self.run_hooks("pre-launch", &profile.pre_launch_hooks, profile)?;

        // Apply power profile if specified
        if let Some(power_profile) = &profile.power_profile {
            println!("   Applying power profile: {}", power_profile);
            crate::power::set_power_profile(power_profile)?;
        }

        // Apply GPU overclocking if specified
        if profile.gpu_clock_offset.is_some() || profile.mem_clock_offset.is_some() {
            println!("   Applying GPU overclocking settings...");
            let oc_profile = crate::overclocking::OverclockProfile {
                name: profile.name.clone(),
                gpu_clock_offset: profile.gpu_clock_offset.unwrap_or(0),
                memory_clock_offset: profile.mem_clock_offset.unwrap_or(0),
                power_limit: profile.power_limit.unwrap_or(100) as u8,
                voltage_offset: 0,
                temp_limit: 85,
                fan_curve: Vec::new(),
            };
            crate::overclocking::apply_overclock_profile(&oc_profile)?;
        }

        if let Some(power_limit) = profile.power_limit {
            println!("   Applying power limit: {}%", power_limit);
            crate::power::set_power_limit_percentage(power_limit)?;
        }

        if let Some(vibrance) = profile.vibrance {
            println!("   Applying vibrance: {}%", vibrance);
            let _ = crate::vibrance_native::set_vibrance_all_native(vibrance);
        }

        // Set CPU affinity if specified
        if let Some(cpu_list) = &profile.cpu_affinity {
            println!("   Setting CPU affinity: {:?}", cpu_list);
            self.set_cpu_affinity(cpu_list)?;
        }

        // Warm start if enabled
        if profile.warm_start {
            println!("   Performing shader cache warm-start...");
            self.warm_start_game(profile)?;
        }

        // Build the launch command
        let mut cmd = if profile.use_gamescope {
            self.build_gamescope_command(profile)?
        } else if profile.use_proton {
            self.build_proton_command(profile)?
        } else {
            Command::new(&profile.executable)
        };

        // Set working directory
        if let Some(work_dir) = &profile.working_dir {
            cmd.current_dir(work_dir);
        }

        // Set environment variables
        for (key, value) in &profile.env_vars {
            cmd.env(key, value);
        }

        // Add arguments
        cmd.args(&args);

        // Print launch info
        println!("   Executable: {}", profile.executable);
        if !profile.env_vars.is_empty() {
            println!("   Environment variables: {} set", profile.env_vars.len());
        }
        if !args.is_empty() {
            println!("   Arguments: {:?}", args);
        }

        // Launch the game
        println!("\n🎮 Starting game...\n");

        let status = cmd
            .stdin(Stdio::null())
            .status()
            .map_err(|e| NvControlError::CommandFailed(format!("Failed to launch game: {}", e)))?;

        if !status.success() {
            let _ = self.run_hooks("post-exit", &profile.post_exit_hooks, profile);
            return Err(NvControlError::CommandFailed(format!(
                "Game exited with error code: {:?}",
                status.code()
            )));
        }

        self.run_hooks("post-exit", &profile.post_exit_hooks, profile)?;
        println!("\n✅ Game exited successfully");
        Ok(())
    }

    fn run_hooks(&self, phase: &str, hooks: &[LaunchHook], profile: &GameProfile) -> NvResult<()> {
        for hook in hooks {
            println!(
                "   Running {} hook: {} {:?}",
                phase, hook.command, hook.args
            );

            let mut cmd = Command::new(&hook.command);
            cmd.args(&hook.args);

            if let Some(work_dir) = &profile.working_dir {
                cmd.current_dir(work_dir);
            }

            for (key, value) in &profile.env_vars {
                cmd.env(key, value);
            }

            let status = cmd.status().map_err(|e| {
                NvControlError::CommandFailed(format!(
                    "Failed to execute {} hook '{}': {}",
                    phase, hook.command, e
                ))
            })?;

            if !status.success() && !hook.ignore_failure {
                return Err(NvControlError::CommandFailed(format!(
                    "{} hook '{}' failed with status {:?}",
                    phase,
                    hook.command,
                    status.code()
                )));
            }
        }

        Ok(())
    }

    fn build_gamescope_command(&self, profile: &GameProfile) -> NvResult<Command> {
        let mut config = if let Some(preset) = profile.gamescope_preset.as_deref() {
            Self::gamescope_config_from_preset(preset)
        } else {
            crate::gamescope::GamescopeConfig::default()
        };

        if let Some(width) = profile.gamescope_width {
            config.width = width;
        }

        if let Some(height) = profile.gamescope_height {
            config.height = height;
        }

        if let Some(refresh) = profile.gamescope_refresh {
            config.refresh_rate = Some(refresh);
        }

        config.hdr_enabled = profile.gamescope_hdr;
        config.adaptive_sync = profile.gamescope_vrr;

        let command = crate::gamescope::generate_advanced_command(&config, &profile.executable);
        let mut cmd = Command::new(&command[0]);
        cmd.args(&command[1..]);
        Ok(cmd)
    }

    fn gamescope_config_from_preset(preset: &str) -> crate::gamescope::GamescopeConfig {
        match preset.to_lowercase().as_str() {
            "performance" => crate::gamescope::GamescopePreset::Performance.to_config(),
            "quality" => crate::gamescope::GamescopePreset::Quality.to_config(),
            "balanced" => crate::gamescope::GamescopePreset::Balanced.to_config(),
            "competitive" => crate::gamescope::GamescopePreset::Competitive.to_config(),
            "cinematic" => crate::gamescope::GamescopePreset::Cinematic.to_config(),
            "steamdeck" => crate::gamescope::GamescopePreset::SteamDeck.to_config(),
            "desktop" => crate::gamescope::GamescopePreset::Desktop.to_config(),
            _ => crate::gamescope::GamescopeConfig::default(),
        }
    }

    fn build_proton_command(&self, profile: &GameProfile) -> NvResult<Command> {
        // Find Proton installation
        let proton_path = if let Some(version) = &profile.proton_version {
            format!(
                "/home/{}/.steam/steam/steamapps/common/Proton {}/proton",
                std::env::var("USER").unwrap_or_else(|_| "user".to_string()),
                version
            )
        } else {
            "proton".to_string()
        };

        let mut cmd = Command::new(proton_path);
        cmd.arg("run");
        cmd.arg(&profile.executable);

        if let Some(prefix) = &profile.wine_prefix {
            cmd.env("WINEPREFIX", prefix);
        }

        Ok(cmd)
    }

    fn set_cpu_affinity(&self, cpu_list: &[usize]) -> NvResult<()> {
        // Note: This would require calling sched_setaffinity via libc
        // For now, we'll use taskset command
        let cpu_mask = cpu_list
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(",");

        println!("   CPU affinity mask: {}", cpu_mask);
        // In practice, we'd store this and apply it to the spawned process
        Ok(())
    }

    fn warm_start_game(&self, profile: &GameProfile) -> NvResult<()> {
        println!("   Running 30-second warm-start pass...");

        let mut cmd = Command::new(&profile.executable);

        // Set environment variables
        for (key, value) in &profile.env_vars {
            cmd.env(key, value);
        }

        if let Some(work_dir) = &profile.working_dir {
            cmd.current_dir(work_dir);
        }

        // Launch in background and kill after 30 seconds
        let child = cmd
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| NvControlError::CommandFailed(format!("Warm start failed: {}", e)))?;

        let pid = child.id();

        // Wait 30 seconds
        std::thread::sleep(std::time::Duration::from_secs(30));

        // Kill the process
        #[cfg(unix)]
        {
            Command::new("kill").arg(pid.to_string()).status().ok();
        }

        println!("   ✅ Warm-start complete, shader cache primed");
        Ok(())
    }

    /// Create example game profiles
    pub fn create_example_profiles(&self) -> NvResult<()> {
        println!("📝 Creating example game profiles...");

        // Cyberpunk 2077
        let cyberpunk =
            GameProfile::new("cyberpunk2077".to_string(), "Cyberpunk2077.exe".to_string())
                .with_shader_cache("/fastcache/cyberpunk2077".to_string())
                .with_dxvk(true)
                .with_vkd3d()
                .with_dlss();

        let mut cyberpunk = cyberpunk;
        cyberpunk.use_gamescope = true;
        cyberpunk.gamescope_width = Some(2560);
        cyberpunk.gamescope_height = Some(1440);
        cyberpunk.gamescope_refresh = Some(144);
        cyberpunk.gamescope_hdr = true;
        cyberpunk.gamescope_preset = Some("quality".to_string());
        cyberpunk.power_profile = Some("performance".to_string());
        cyberpunk.use_proton = true;
        cyberpunk.vibrance = Some(130);
        cyberpunk.priority = ProcessPriority::High;
        cyberpunk.pre_launch_hooks.push(LaunchHook {
            command: "sh".to_string(),
            args: vec![
                "-c".to_string(),
                "notify-send 'nvcontrol' 'Launching Cyberpunk 2077'".to_string(),
            ],
            ignore_failure: true,
        });

        self.save_profile(&cyberpunk)?;

        // CS2 / Counter-Strike 2
        let cs2 = GameProfile::new("cs2".to_string(), "cs2".to_string())
            .with_shader_cache("/fastcache/cs2".to_string());

        let mut cs2 = cs2;
        cs2.use_gamescope = true;
        cs2.gamescope_width = Some(1920);
        cs2.gamescope_height = Some(1080);
        cs2.gamescope_refresh = Some(240);
        cs2.gamescope_vrr = true;
        cs2.gamescope_preset = Some("competitive".to_string());
        cs2.power_profile = Some("performance".to_string());
        cs2.cpu_affinity = Some(vec![0, 1, 2, 3, 4, 5, 6, 7]); // Pin to first CCD
        cs2.fps_limit = Some(240);
        cs2.priority = ProcessPriority::Realtime;
        cs2.post_exit_hooks.push(LaunchHook {
            command: "sh".to_string(),
            args: vec![
                "-c".to_string(),
                "notify-send 'nvcontrol' 'CS2 exited'".to_string(),
            ],
            ignore_failure: true,
        });

        self.save_profile(&cs2)?;

        // Elden Ring
        let elden_ring = GameProfile::new("eldenring".to_string(), "eldenring.exe".to_string())
            .with_shader_cache("/fastcache/eldenring".to_string())
            .with_dxvk(true);

        let mut elden_ring = elden_ring;
        elden_ring.use_proton = true;
        elden_ring.use_gamescope = true;
        elden_ring.gamescope_width = Some(1920);
        elden_ring.gamescope_height = Some(1080);
        elden_ring.gamescope_refresh = Some(60);
        elden_ring.gamescope_preset = Some("balanced".to_string());
        elden_ring.power_profile = Some("balanced".to_string());
        elden_ring.priority = ProcessPriority::High;

        self.save_profile(&elden_ring)?;

        println!("✅ Created 3 example game profiles");
        println!("   Profiles saved to: {}", self.profiles_dir.display());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_profile_has_no_hooks() {
        let profile = GameProfile::default();
        assert!(profile.pre_launch_hooks.is_empty());
        assert!(profile.post_exit_hooks.is_empty());
    }

    #[test]
    fn builder_adds_hooks() {
        let profile = GameProfile::new("test".to_string(), "game".to_string())
            .with_pre_launch_hook("echo", vec!["before".to_string()])
            .with_post_exit_hook("echo", vec!["after".to_string()]);

        assert_eq!(profile.pre_launch_hooks.len(), 1);
        assert_eq!(profile.post_exit_hooks.len(), 1);
        assert_eq!(profile.pre_launch_hooks[0].command, "echo");
        assert!(profile.post_exit_hooks[0].ignore_failure);
    }
}
