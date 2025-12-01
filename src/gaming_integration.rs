/// Phase 4.1: Gaming Integration
///
/// Steam integration, Lutris integration, GameMode integration, automatic profile application
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Game detected on the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedGame {
    pub name: String,
    pub executable: PathBuf,
    pub launcher: GameLauncher,
    pub app_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameLauncher {
    Steam,
    Lutris,
    Heroic,
    Native,
    Unknown,
}

/// Steam library scanner
#[allow(dead_code)]
pub struct SteamScanner {
    steam_root: Option<PathBuf>,
    library_folders: Vec<PathBuf>,
}

impl SteamScanner {
    pub fn new() -> Self {
        let steam_root = Self::find_steam_root();
        let library_folders = Self::find_library_folders(&steam_root);

        Self {
            steam_root,
            library_folders,
        }
    }

    fn find_steam_root() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        let candidates = vec![
            PathBuf::from(&home).join(".steam/steam"),
            PathBuf::from(&home).join(".local/share/Steam"),
            PathBuf::from("/usr/share/steam"),
        ];

        candidates.into_iter().find(|p| p.exists())
    }

    fn find_library_folders(steam_root: &Option<PathBuf>) -> Vec<PathBuf> {
        let mut folders = Vec::new();

        if let Some(root) = steam_root {
            folders.push(root.clone());

            // Parse libraryfolders.vdf
            let vdf_path = root.join("steamapps/libraryfolders.vdf");
            if vdf_path.exists() {
                if let Ok(content) = std::fs::read_to_string(vdf_path) {
                    for line in content.lines() {
                        if line.contains("\"path\"") {
                            if let Some(path_str) = line.split('"').nth(3) {
                                let path = PathBuf::from(path_str);
                                if path.exists() {
                                    folders.push(path);
                                }
                            }
                        }
                    }
                }
            }
        }

        folders
    }

    /// Scan for installed Steam games
    pub fn scan_games(&self) -> Vec<DetectedGame> {
        let mut games = Vec::new();

        for library in &self.library_folders {
            let steamapps = library.join("steamapps");
            if !steamapps.exists() {
                continue;
            }

            // Read .acf manifest files
            if let Ok(entries) = std::fs::read_dir(&steamapps) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("acf") {
                        if let Some(game) = self.parse_acf(&path, &steamapps) {
                            games.push(game);
                        }
                    }
                }
            }
        }

        games
    }

    fn parse_acf(&self, acf_path: &Path, steamapps: &Path) -> Option<DetectedGame> {
        let content = std::fs::read_to_string(acf_path).ok()?;

        let mut app_id = None;
        let mut name = None;
        let mut install_dir = None;

        for line in content.lines() {
            if line.contains("\"appid\"") {
                app_id = line.split('"').nth(3).map(String::from);
            } else if line.contains("\"name\"") {
                name = line.split('"').nth(3).map(String::from);
            } else if line.contains("\"installdir\"") {
                install_dir = line.split('"').nth(3).map(String::from);
            }
        }

        let name = name?;
        let install_dir = install_dir?;

        // Find executable (heuristic: look for .exe or binary in install dir)
        let game_dir = steamapps.join("common").join(&install_dir);
        let executable = self.find_game_executable(&game_dir)?;

        Some(DetectedGame {
            name,
            executable,
            launcher: GameLauncher::Steam,
            app_id,
        })
    }

    fn find_game_executable(&self, game_dir: &Path) -> Option<PathBuf> {
        if !game_dir.exists() {
            return None;
        }

        // Look for common patterns
        if let Ok(entries) = std::fs::read_dir(game_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_file() {
                    let name = path.file_name()?.to_str()?;

                    // Windows executables (via Proton)
                    if name.ends_with(".exe") && !name.contains("unins") && !name.contains("crash")
                    {
                        return Some(path);
                    }

                    // Native Linux executables (check if executable bit set)
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if let Ok(metadata) = path.metadata() {
                            if metadata.permissions().mode() & 0o111 != 0 {
                                return Some(path);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Launch Steam game with optimizations
    pub fn launch_game(&self, app_id: &str, env_vars: HashMap<String, String>) -> NvResult<()> {
        let mut cmd = Command::new("steam");
        cmd.arg(format!("steam://run/{}", app_id));

        for (key, value) in env_vars {
            cmd.env(key, value);
        }

        cmd.spawn().map_err(|e| {
            NvControlError::CommandFailed(format!("Failed to launch Steam game: {}", e))
        })?;

        Ok(())
    }
}

impl Default for SteamScanner {
    fn default() -> Self {
        Self::new()
    }
}

/// Lutris integration
#[allow(dead_code)]
pub struct LutrisScanner {
    lutris_db: Option<PathBuf>,
}

impl LutrisScanner {
    pub fn new() -> Self {
        let home = std::env::var("HOME").ok();
        let lutris_db = home.map(|h| PathBuf::from(h).join(".local/share/lutris/pga.db"));

        Self { lutris_db }
    }

    /// Scan for Lutris games
    pub fn scan_games(&self) -> Vec<DetectedGame> {
        // Note: Parsing SQLite DB would require rusqlite dependency
        // For now, we'll scan the games directory
        let home = match std::env::var("HOME") {
            Ok(h) => h,
            Err(_) => return Vec::new(),
        };

        let games_dir = PathBuf::from(home).join(".local/share/lutris/games");
        if !games_dir.exists() {
            return Vec::new();
        }

        let mut games = Vec::new();

        if let Ok(entries) = std::fs::read_dir(games_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                        // Look for executable in game dir
                        if let Some(exe) = self.find_executable(&path) {
                            games.push(DetectedGame {
                                name: name.to_string(),
                                executable: exe,
                                launcher: GameLauncher::Lutris,
                                app_id: None,
                            });
                        }
                    }
                }
            }
        }

        games
    }

    fn find_executable(&self, game_dir: &Path) -> Option<PathBuf> {
        if let Ok(entries) = std::fs::read_dir(game_dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if path.is_file() {
                        if let Ok(metadata) = path.metadata() {
                            if metadata.permissions().mode() & 0o111 != 0 {
                                return Some(path);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Launch Lutris game
    pub fn launch_game(&self, game_id: &str) -> NvResult<()> {
        Command::new("lutris")
            .arg(format!("lutris:rungame/{}", game_id))
            .spawn()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("Failed to launch Lutris game: {}", e))
            })?;

        Ok(())
    }
}

impl Default for LutrisScanner {
    fn default() -> Self {
        Self::new()
    }
}

/// GameMode integration
pub struct GameModeIntegration {
    enabled: bool,
}

impl GameModeIntegration {
    pub fn new() -> Self {
        let enabled = Self::is_gamemode_available();

        Self { enabled }
    }

    fn is_gamemode_available() -> bool {
        Command::new("gamemoded").arg("--version").output().is_ok()
    }

    /// Check if GameMode is available
    pub fn is_available(&self) -> bool {
        self.enabled
    }

    /// Start GameMode for a process
    pub fn start_for_pid(&self, pid: u32) -> NvResult<()> {
        if !self.enabled {
            return Err(NvControlError::UnsupportedFeature(
                "GameMode not available".to_string(),
            ));
        }

        Command::new("gamemoderun")
            .arg(pid.to_string())
            .spawn()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("Failed to start GameMode: {}", e))
            })?;

        Ok(())
    }

    /// Check GameMode status
    pub fn is_active(&self) -> bool {
        if !self.enabled {
            return false;
        }

        Command::new("gamemoded")
            .arg("--status")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Default for GameModeIntegration {
    fn default() -> Self {
        Self::new()
    }
}

/// Automatic profile application on game launch
pub struct GameProfileAutomation {
    active_game: Option<DetectedGame>,
    profiles: HashMap<String, String>, // exe -> profile name
}

impl GameProfileAutomation {
    pub fn new() -> Self {
        Self {
            active_game: None,
            profiles: HashMap::new(),
        }
    }

    /// Register game -> profile mapping
    pub fn register_profile(&mut self, game_exe: String, profile_name: String) {
        self.profiles.insert(game_exe, profile_name);
    }

    /// Detect running game
    pub fn detect_running_game<'a>(
        &mut self,
        games: &'a [DetectedGame],
    ) -> Option<&'a DetectedGame> {
        // Check running processes
        let output = Command::new("ps").arg("aux").output().ok()?;
        let ps_output = String::from_utf8_lossy(&output.stdout);

        for game in games {
            let exe_name = game.executable.file_name()?.to_str()?;

            if ps_output.contains(exe_name) {
                self.active_game = Some(game.clone());
                return Some(game);
            }
        }

        None
    }

    /// Get profile for active game
    pub fn get_active_profile(&self) -> Option<&String> {
        let game = self.active_game.as_ref()?;
        let exe = game.executable.to_str()?;
        self.profiles.get(exe)
    }

    /// Apply profile for detected game
    pub fn apply_profile_for_game(&self, game: &DetectedGame, gpu_id: u32) -> NvResult<()> {
        let exe = game
            .executable
            .to_str()
            .ok_or_else(|| NvControlError::RuntimeError("Invalid executable path".to_string()))?;

        let profile_name = self.profiles.get(exe).ok_or_else(|| {
            NvControlError::ConfigError(format!("No profile for game: {}", game.name))
        })?;

        // Apply overclock profile
        use crate::enhanced_overclock::OverclockProfileManager;
        let mut oc_manager = OverclockProfileManager::new();
        oc_manager.load()?;
        oc_manager.apply_profile(profile_name, gpu_id)?;

        println!("Applied profile '{}' for game: {}", profile_name, game.name);

        Ok(())
    }
}

impl Default for GameProfileAutomation {
    fn default() -> Self {
        Self::new()
    }
}

/// Launch parameter optimizer
pub struct LaunchOptimizer {
    gamemode_enabled: bool,
    mangohud_enabled: bool,
    fsync_enabled: bool,
}

impl LaunchOptimizer {
    pub fn new() -> Self {
        Self {
            gamemode_enabled: true,
            mangohud_enabled: false,
            fsync_enabled: true,
        }
    }

    /// Build optimized environment variables
    pub fn build_env_vars(&self) -> HashMap<String, String> {
        let mut env = HashMap::new();

        // NVIDIA specific
        env.insert("__GL_THREADED_OPTIMIZATIONS".to_string(), "1".to_string());
        env.insert("__GL_SHADER_DISK_CACHE".to_string(), "1".to_string());
        env.insert(
            "__GL_SHADER_DISK_CACHE_SKIP_CLEANUP".to_string(),
            "1".to_string(),
        );

        // Proton/Wine optimizations
        if self.fsync_enabled {
            env.insert("PROTON_NO_ESYNC".to_string(), "0".to_string());
            env.insert("PROTON_NO_FSYNC".to_string(), "0".to_string());
        }

        // MangoHud
        if self.mangohud_enabled {
            env.insert("MANGOHUD".to_string(), "1".to_string());
        }

        // GameMode
        if self.gamemode_enabled {
            env.insert("LD_PRELOAD".to_string(), "libgamemodeauto.so".to_string());
        }

        env
    }

    /// Build launch command prefix
    pub fn build_launch_prefix(&self) -> String {
        let mut prefix = Vec::new();

        if self.gamemode_enabled {
            prefix.push("gamemoderun");
        }

        if self.mangohud_enabled {
            prefix.push("mangohud");
        }

        prefix.join(" ")
    }

    pub fn set_gamemode(&mut self, enabled: bool) {
        self.gamemode_enabled = enabled;
    }

    pub fn set_mangohud(&mut self, enabled: bool) {
        self.mangohud_enabled = enabled;
    }

    pub fn set_fsync(&mut self, enabled: bool) {
        self.fsync_enabled = enabled;
    }
}

impl Default for LaunchOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steam_scanner() {
        let scanner = SteamScanner::new();
        println!("Steam root: {:?}", scanner.steam_root);
        println!("Library folders: {:?}", scanner.library_folders);
    }

    #[test]
    fn test_gamemode_detection() {
        let gamemode = GameModeIntegration::new();
        println!("GameMode available: {}", gamemode.is_available());
    }

    #[test]
    fn test_launch_optimizer() {
        let optimizer = LaunchOptimizer::new();
        let env = optimizer.build_env_vars();

        assert!(env.contains_key("__GL_THREADED_OPTIMIZATIONS"));
        assert_eq!(
            env.get("__GL_THREADED_OPTIMIZATIONS"),
            Some(&"1".to_string())
        );
    }

    #[test]
    fn test_launch_prefix() {
        let mut optimizer = LaunchOptimizer::new();
        optimizer.set_gamemode(true);
        optimizer.set_mangohud(true);

        let prefix = optimizer.build_launch_prefix();
        assert!(prefix.contains("gamemoderun"));
        assert!(prefix.contains("mangohud"));
    }

    #[test]
    fn test_game_profile_automation() {
        let mut automation = GameProfileAutomation::new();

        automation.register_profile(
            "/path/to/game.exe".to_string(),
            "High Performance".to_string(),
        );

        assert_eq!(
            automation.profiles.get("/path/to/game.exe"),
            Some(&"High Performance".to_string())
        );
    }
}
