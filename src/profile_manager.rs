// Profile Import/Export Manager
// Save and load fan curves, overclock profiles, and game profiles

use crate::gui_widgets::{FanCurve, VoltageCurve};
use crate::game_detection::GameProfile;
use crate::overclocking::OverclockProfile;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBundle {
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub fan_curve: Option<FanCurve>,
    pub voltage_curve: Option<VoltageCurve>,
    pub overclock: Option<OverclockProfile>,
    pub game_profiles: Vec<GameProfile>,
    pub vibrance_settings: Option<VibranceSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibranceSettings {
    pub display_levels: Vec<i16>,
    pub per_game_vibrance: bool,
}

pub struct ProfileManager {
    profiles_dir: PathBuf,
}

impl ProfileManager {
    pub fn new() -> NvResult<Self> {
        let profiles_dir = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find config directory".into()))?
            .join("nvcontrol")
            .join("profiles");

        fs::create_dir_all(&profiles_dir)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to create profiles directory: {}", e)))?;

        Ok(Self { profiles_dir })
    }

    /// Export a complete profile bundle to JSON
    pub fn export_profile(&self, bundle: &ProfileBundle, filename: Option<&str>) -> NvResult<PathBuf> {
        let filename = filename.unwrap_or(&bundle.name);
        let safe_filename = self.sanitize_filename(filename);
        let path = self.profiles_dir.join(format!("{}.json", safe_filename));

        let json = serde_json::to_string_pretty(bundle)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to serialize profile: {}", e)))?;

        fs::write(&path, json)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write profile: {}", e)))?;

        println!("✅ Profile exported to: {}", path.display());
        Ok(path)
    }

    /// Import a profile bundle from JSON
    pub fn import_profile(&self, path: &Path) -> NvResult<ProfileBundle> {
        if !path.exists() {
            return Err(NvControlError::ConfigError(format!("Profile file not found: {}", path.display())));
        }

        let json = fs::read_to_string(path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read profile: {}", e)))?;

        let bundle: ProfileBundle = serde_json::from_str(&json)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse profile: {}", e)))?;

        println!("✅ Profile imported: {}", bundle.name);
        Ok(bundle)
    }

    /// List all available profile bundles
    pub fn list_profiles(&self) -> NvResult<Vec<ProfileBundle>> {
        let mut profiles = Vec::new();

        if !self.profiles_dir.exists() {
            return Ok(profiles);
        }

        for entry in fs::read_dir(&self.profiles_dir)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read profiles directory: {}", e)))?
        {
            let entry = entry.map_err(|e| NvControlError::ConfigError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(bundle) = self.import_profile(&path) {
                    profiles.push(bundle);
                }
            }
        }

        // Sort by creation date (newest first)
        profiles.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(profiles)
    }

    /// Delete a profile bundle
    pub fn delete_profile(&self, name: &str) -> NvResult<()> {
        let safe_filename = self.sanitize_filename(name);
        let path = self.profiles_dir.join(format!("{}.json", safe_filename));

        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| NvControlError::ConfigError(format!("Failed to delete profile: {}", e)))?;
            println!("✅ Profile deleted: {}", name);
        }

        Ok(())
    }

    /// Export just a fan curve
    pub fn export_fan_curve(&self, curve: &FanCurve, name: &str) -> NvResult<PathBuf> {
        let bundle = ProfileBundle {
            name: name.to_string(),
            description: "Fan curve profile".to_string(),
            created_at: chrono::Utc::now(),
            fan_curve: Some(curve.clone()),
            voltage_curve: None,
            overclock: None,
            game_profiles: vec![],
            vibrance_settings: None,
        };

        self.export_profile(&bundle, Some(name))
    }

    /// Export just a voltage curve
    pub fn export_voltage_curve(&self, curve: &VoltageCurve, name: &str) -> NvResult<PathBuf> {
        let bundle = ProfileBundle {
            name: name.to_string(),
            description: "Voltage curve profile".to_string(),
            created_at: chrono::Utc::now(),
            fan_curve: None,
            voltage_curve: Some(curve.clone()),
            overclock: None,
            game_profiles: vec![],
            vibrance_settings: None,
        };

        self.export_profile(&bundle, Some(name))
    }

    /// Export just an overclock profile
    pub fn export_overclock(&self, oc: &OverclockProfile, name: &str) -> NvResult<PathBuf> {
        let bundle = ProfileBundle {
            name: name.to_string(),
            description: "Overclock profile".to_string(),
            created_at: chrono::Utc::now(),
            fan_curve: None,
            voltage_curve: None,
            overclock: Some(oc.clone()),
            game_profiles: vec![],
            vibrance_settings: None,
        };

        self.export_profile(&bundle, Some(name))
    }

    /// Quick save current settings
    pub fn quick_save(
        &self,
        name: &str,
        fan_curve: Option<FanCurve>,
        voltage_curve: Option<VoltageCurve>,
        overclock: Option<OverclockProfile>,
    ) -> NvResult<PathBuf> {
        let bundle = ProfileBundle {
            name: name.to_string(),
            description: format!("Quick save - {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")),
            created_at: chrono::Utc::now(),
            fan_curve,
            voltage_curve,
            overclock,
            game_profiles: vec![],
            vibrance_settings: None,
        };

        self.export_profile(&bundle, Some(name))
    }

    fn sanitize_filename(&self, name: &str) -> String {
        name.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                _ => c,
            })
            .collect()
    }

    /// Get the profiles directory path
    pub fn get_profiles_dir(&self) -> &Path {
        &self.profiles_dir
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ProfileManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        let manager = ProfileManager::new().unwrap();
        assert_eq!(manager.sanitize_filename("test<profile>"), "test_profile_");
        assert_eq!(manager.sanitize_filename("my:profile"), "my_profile");
    }
}
