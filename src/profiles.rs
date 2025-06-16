use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::{NvResult, NvControlError};

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub vibrance_levels: Vec<i16>,
    pub hdr_enabled: bool,
    pub fan_speeds: HashMap<usize, u8>,
    pub icc_profile: Option<String>,
}

impl Profile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            vibrance_levels: vec![0; 2], // Default for 2 displays
            hdr_enabled: false,
            fan_speeds: HashMap::new(),
            icc_profile: None,
        }
    }
}

pub fn save_profile(profile: &Profile) -> NvResult<()> {
    let profiles_dir = get_profiles_dir();
    fs::create_dir_all(&profiles_dir).map_err(|e| 
        NvControlError::DisplayDetectionFailed(format!("Failed to create profiles directory: {e}")))?;
    
    let profile_path = profiles_dir.join(format!("{}.toml", profile.name));
    let content = toml::to_string_pretty(profile).map_err(|e|
        NvControlError::DisplayDetectionFailed(format!("Failed to serialize profile: {e}")))?;
    
    fs::write(profile_path, content).map_err(|e|
        NvControlError::DisplayDetectionFailed(format!("Failed to save profile: {e}")))?;
    
    Ok(())
}

pub fn load_profile(name: &str) -> NvResult<Profile> {
    let profile_path = get_profiles_dir().join(format!("{name}.toml"));
    let content = fs::read_to_string(profile_path).map_err(|e|
        NvControlError::DisplayDetectionFailed(format!("Failed to read profile: {e}")))?;
    
    toml::from_str(&content).map_err(|e|
        NvControlError::DisplayDetectionFailed(format!("Failed to parse profile: {e}")))
}

pub fn list_profiles() -> Vec<String> {
    let profiles_dir = get_profiles_dir();
    if !profiles_dir.exists() {
        return Vec::new();
    }
    
    fs::read_dir(&profiles_dir)
        .unwrap_or_else(|_| fs::read_dir(".").unwrap())
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
}

fn get_profiles_dir() -> PathBuf {
    if let Some(project_dirs) = directories::ProjectDirs::from("com", "ghostkellz", "nvcontrol") {
        project_dirs.config_dir().join("profiles")
    } else {
        PathBuf::from("profiles")
    }
}