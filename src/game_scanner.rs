// Game Library Scanner
// Scans Steam, Lutris, Heroic for installed games and creates profiles

use crate::game_detection::GameProfile;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedGame {
    pub name: String,
    pub executable: String,
    pub launcher: GameLauncher,
    pub install_path: PathBuf,
    pub app_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameLauncher {
    Steam,
    Lutris,
    Heroic,
    Native,
}

pub struct GameLibraryScanner {
    steam_library_paths: Vec<PathBuf>,
    lutris_games_path: PathBuf,
    heroic_config_path: PathBuf,
}

impl GameLibraryScanner {
    pub fn new() -> NvResult<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find home directory".into()))?;

        Ok(Self {
            steam_library_paths: vec![
                home.join(".steam/steam/steamapps"),
                home.join(".local/share/Steam/steamapps"),
            ],
            lutris_games_path: home.join(".local/share/lutris/games"),
            heroic_config_path: home.join(".config/heroic"),
        })
    }

    /// Scan all launchers for installed games
    pub fn scan_all(&self) -> NvResult<Vec<ScannedGame>> {
        let mut games = Vec::new();

        // Scan Steam
        println!("🔍 Scanning Steam library...");
        games.extend(self.scan_steam()?);

        // Scan Lutris
        println!("🔍 Scanning Lutris games...");
        games.extend(self.scan_lutris()?);

        // Scan Heroic
        println!("🔍 Scanning Heroic games...");
        games.extend(self.scan_heroic()?);

        println!("✅ Found {} games total", games.len());
        Ok(games)
    }

    /// Scan Steam library for games
    pub fn scan_steam(&self) -> NvResult<Vec<ScannedGame>> {
        let mut games = Vec::new();

        for library_path in &self.steam_library_paths {
            if !library_path.exists() {
                continue;
            }

            // Read app manifest files
            let manifests = fs::read_dir(library_path)?
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .starts_with("appmanifest_")
                        && e.file_name().to_string_lossy().ends_with(".acf")
                });

            for manifest in manifests {
                if let Ok(game) = self.parse_steam_manifest(&manifest.path()) {
                    games.push(game);
                }
            }
        }

        println!("   📦 Steam: {} games", games.len());
        Ok(games)
    }

    fn parse_steam_manifest(&self, path: &Path) -> NvResult<ScannedGame> {
        let content = fs::read_to_string(path)?;

        // Simple key-value parser for Steam's ACF format
        let mut name = String::new();
        let mut install_dir = String::new();
        let mut app_id = String::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("\"name\"") {
                name = trimmed
                    .split('"')
                    .nth(3)
                    .unwrap_or("")
                    .to_string();
            } else if trimmed.starts_with("\"installdir\"") {
                install_dir = trimmed
                    .split('"')
                    .nth(3)
                    .unwrap_or("")
                    .to_string();
            } else if trimmed.starts_with("\"appid\"") {
                app_id = trimmed
                    .split('"')
                    .nth(3)
                    .unwrap_or("")
                    .to_string();
            }
        }

        if name.is_empty() || install_dir.is_empty() {
            return Err(NvControlError::ConfigError("Invalid manifest".into()));
        }

        // Try to find the executable
        let install_path = path
            .parent()
            .unwrap()
            .join("common")
            .join(&install_dir);

        let executable = self.find_game_executable(&install_path, &name)?;

        Ok(ScannedGame {
            name,
            executable,
            launcher: GameLauncher::Steam,
            install_path,
            app_id: Some(app_id),
        })
    }

    /// Scan Lutris games
    pub fn scan_lutris(&self) -> NvResult<Vec<ScannedGame>> {
        let mut games = Vec::new();

        if !self.lutris_games_path.exists() {
            return Ok(games);
        }

        // Lutris stores game info in YAML files in ~/.local/share/lutris/games/
        let lutris_config = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("No config dir".into()))?
            .join("lutris/games");

        if !lutris_config.exists() {
            return Ok(games);
        }

        for entry in fs::read_dir(&lutris_config)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("yml") {
                if let Ok(game) = self.parse_lutris_game(&path) {
                    games.push(game);
                }
            }
        }

        println!("   🎮 Lutris: {} games", games.len());
        Ok(games)
    }

    fn parse_lutris_game(&self, _path: &Path) -> NvResult<ScannedGame> {
        // For now, return a placeholder
        // TODO: Implement YAML parsing for Lutris configs
        Err(NvControlError::UnsupportedFeature("Lutris parsing not yet implemented".into()))
    }

    /// Scan Heroic Games Launcher
    pub fn scan_heroic(&self) -> NvResult<Vec<ScannedGame>> {
        let mut games = Vec::new();

        if !self.heroic_config_path.exists() {
            return Ok(games);
        }

        // Heroic stores installed games info in JSON files
        let gog_library = self.heroic_config_path.join("gog_store/library.json");
        let epic_library = self.heroic_config_path.join("legendaryConfig/installed.json");

        if gog_library.exists() {
            games.extend(self.parse_heroic_library(&gog_library, GameLauncher::Heroic)?);
        }

        if epic_library.exists() {
            games.extend(self.parse_heroic_library(&epic_library, GameLauncher::Heroic)?);
        }

        println!("   🦸 Heroic: {} games", games.len());
        Ok(games)
    }

    fn parse_heroic_library(&self, _path: &Path, _launcher: GameLauncher) -> NvResult<Vec<ScannedGame>> {
        // TODO: Implement JSON parsing for Heroic configs
        Ok(Vec::new())
    }

    /// Try to find the main executable for a game
    fn find_game_executable(&self, install_path: &Path, game_name: &str) -> NvResult<String> {
        if !install_path.exists() {
            return Ok(game_name.to_lowercase().replace(" ", "_"));
        }

        // Common executable patterns
        let patterns = vec![
            format!("{}.exe", game_name.to_lowercase().replace(" ", "")),
            format!("{}.x86_64", game_name.to_lowercase().replace(" ", "_")),
            format!("{}", game_name.to_lowercase().replace(" ", "_")),
            "start.exe".to_string(),
            "game.exe".to_string(),
        ];

        // Search for executables
        for entry in fs::read_dir(install_path)? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_lowercase();

            for pattern in &patterns {
                if file_name.contains(pattern) {
                    return Ok(entry.file_name().to_string_lossy().to_string());
                }
            }
        }

        // Fallback to game name
        Ok(game_name.to_lowercase().replace(" ", "_"))
    }

    /// Generate recommended profile for a game based on its name/genre
    pub fn generate_recommended_profile(&self, game: &ScannedGame) -> GameProfile {
        use crate::game_detection::ProcessPriority;

        let name = game.name.to_lowercase();

        // Competitive games - high performance
        if name.contains("counter-strike")
            || name.contains("valorant")
            || name.contains("apex")
            || name.contains("fortnite")
            || name.contains("warzone")
        {
            return GameProfile {
                name: game.name.clone(),
                executable: game.executable.clone(),
                gpu_offset: Some(150),
                memory_offset: Some(300),
                power_limit: Some(100),
                fan_curve: Some(vec![(40, 40), (60, 60), (75, 80), (85, 100)]),
                vibrance: Some(175), // High vibrance for competitive
                fps_limit: None,
                priority: ProcessPriority::Realtime,
            };
        }

        // AAA single-player - balanced
        if name.contains("cyberpunk")
            || name.contains("witcher")
            || name.contains("red dead")
            || name.contains("elden ring")
        {
            return GameProfile {
                name: game.name.clone(),
                executable: game.executable.clone(),
                gpu_offset: Some(100),
                memory_offset: Some(200),
                power_limit: Some(95),
                fan_curve: Some(vec![(40, 30), (60, 50), (75, 70), (85, 90)]),
                vibrance: Some(125), // Moderate vibrance
                fps_limit: Some(144),
                priority: ProcessPriority::High,
            };
        }

        // Default profile
        GameProfile {
            name: game.name.clone(),
            executable: game.executable.clone(),
            gpu_offset: Some(75),
            memory_offset: Some(150),
            power_limit: None,
            fan_curve: None,
            vibrance: Some(120),
            fps_limit: None,
            priority: ProcessPriority::High,
        }
    }

    /// Bulk create profiles for all scanned games
    pub fn create_profiles_for_all(&self, games: &[ScannedGame]) -> Vec<GameProfile> {
        games
            .iter()
            .map(|game| self.generate_recommended_profile(game))
            .collect()
    }
}

/// Statistics about scanned games
#[derive(Debug)]
pub struct ScanStatistics {
    pub total_games: usize,
    pub steam_games: usize,
    pub lutris_games: usize,
    pub heroic_games: usize,
    pub native_games: usize,
}

impl ScanStatistics {
    pub fn from_games(games: &[ScannedGame]) -> Self {
        Self {
            total_games: games.len(),
            steam_games: games.iter().filter(|g| g.launcher == GameLauncher::Steam).count(),
            lutris_games: games.iter().filter(|g| g.launcher == GameLauncher::Lutris).count(),
            heroic_games: games.iter().filter(|g| g.launcher == GameLauncher::Heroic).count(),
            native_games: games.iter().filter(|g| g.launcher == GameLauncher::Native).count(),
        }
    }

    pub fn print(&self) {
        println!("\n📊 Scan Statistics:");
        println!("   Total Games: {}", self.total_games);
        println!("   Steam: {}", self.steam_games);
        println!("   Lutris: {}", self.lutris_games);
        println!("   Heroic: {}", self.heroic_games);
        println!("   Native: {}", self.native_games);
    }
}
