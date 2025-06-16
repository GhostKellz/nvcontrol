use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub vibrance_levels: Vec<i16>,
    pub hdr_enabled: bool,
    pub selected_icc_profile: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => eprintln!("Failed to parse config: {e}"),
                },
                Err(e) => eprintln!("Failed to read config: {e}"),
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        match toml::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(&config_path, content) {
                    eprintln!("Failed to save config: {e}");
                }
            }
            Err(e) => eprintln!("Failed to serialize config: {e}"),
        }
    }

    fn config_path() -> PathBuf {
        if let Some(config_dir) = directories::ProjectDirs::from("com", "ghostkellz", "nvcontrol") {
            config_dir.config_dir().join("config.toml")
        } else {
            PathBuf::from("nvcontrol_config.toml")
        }
    }
}
