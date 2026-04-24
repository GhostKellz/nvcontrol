// Profile Import/Export Manager
// Save and load fan curves, overclock profiles, and game profiles

use crate::game_launcher::GameProfile;
use crate::gui_widgets::{FanCurve, VoltageCurve};
use crate::multimonitor::{DisplayConfig, DisplayLayout, MultiMonitorManager};
use crate::overclocking::OverclockProfile;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBundle {
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub display_layout: Option<DisplayLayout>,
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

fn point_pairs_to_fan_curve(points: &[(u8, u8)]) -> FanCurve {
    FanCurve {
        points: points
            .iter()
            .map(|(temp, speed)| crate::gui_widgets::CurvePoint::new(*temp as f64, *speed as f64))
            .collect(),
        selected_point: None,
    }
}

fn fan_curve_to_point_pairs(curve: &FanCurve) -> Vec<(u8, u8)> {
    curve
        .points
        .iter()
        .map(|point| {
            (
                point.x.clamp(0.0, 100.0) as u8,
                point.y.clamp(0.0, 100.0) as u8,
            )
        })
        .collect()
}

fn fallback_display_layout() -> DisplayLayout {
    let displays = crate::display::list_displays()
        .into_iter()
        .map(|display| DisplayConfig {
            name: display.name.clone(),
            connector: display.name,
            enabled: true,
            resolution: (1920, 1080),
            refresh_rate: 60,
            position: (0, 0),
            scale: 1.0,
            rotation: crate::multimonitor::Rotation::Normal,
            vrr_enabled: false,
            digital_vibrance: 0,
            color_profile: if display.hdr_capable {
                Some(if display.hdr_enabled {
                    "hdr".to_string()
                } else {
                    "sdr".to_string()
                })
            } else {
                None
            },
        })
        .collect();

    DisplayLayout {
        name: "live".to_string(),
        displays,
    }
}

impl ProfileManager {
    pub fn new() -> NvResult<Self> {
        let profiles_dir = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find config directory".into()))?
            .join("nvcontrol")
            .join("profiles");

        fs::create_dir_all(&profiles_dir).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create profiles directory: {}", e))
        })?;

        Ok(Self { profiles_dir })
    }

    /// Export a complete profile bundle to JSON
    pub fn export_profile(
        &self,
        bundle: &ProfileBundle,
        filename: Option<&str>,
    ) -> NvResult<PathBuf> {
        let filename = filename.unwrap_or(&bundle.name);
        let safe_filename = self.sanitize_filename(filename);
        let path = self.profiles_dir.join(format!("{}.json", safe_filename));

        self.export_profile_to_path(bundle, &path)?;

        println!("✅ Profile exported to: {}", path.display());
        Ok(path)
    }

    pub fn export_profile_to_path(&self, bundle: &ProfileBundle, path: &Path) -> NvResult<()> {
        let parent = path.parent().ok_or_else(|| {
            NvControlError::ConfigError(format!("Invalid export path: {}", path.display()))
        })?;

        fs::create_dir_all(parent).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to create export directory: {}", e))
        })?;

        let json = serde_json::to_string_pretty(bundle).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize profile: {}", e))
        })?;

        fs::write(path, json)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to write profile: {}", e)))?;

        Ok(())
    }

    /// Import a profile bundle from JSON
    pub fn import_profile(&self, path: &Path) -> NvResult<ProfileBundle> {
        if !path.exists() {
            return Err(NvControlError::ConfigError(format!(
                "Profile file not found: {}",
                path.display()
            )));
        }

        let json = fs::read_to_string(path)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to read profile: {}", e)))?;

        let bundle: ProfileBundle = serde_json::from_str(&json)
            .map_err(|e| NvControlError::ConfigError(format!("Failed to parse profile: {}", e)))?;

        println!("✅ Profile imported: {}", bundle.name);
        Ok(bundle)
    }

    pub fn load_profile_by_name(&self, name: &str) -> NvResult<ProfileBundle> {
        let safe_filename = self.sanitize_filename(name);
        let path = self.profiles_dir.join(format!("{}.json", safe_filename));
        self.import_profile(&path)
    }

    pub fn save_imported_profile(
        &self,
        bundle: &ProfileBundle,
        override_name: Option<&str>,
    ) -> NvResult<PathBuf> {
        let mut bundle = bundle.clone();
        if let Some(name) = override_name {
            bundle.name = name.to_string();
        }

        self.export_profile(&bundle, Some(&bundle.name))
    }

    /// List all available profile bundles
    pub fn list_profiles(&self) -> NvResult<Vec<ProfileBundle>> {
        let mut profiles = Vec::new();

        if !self.profiles_dir.exists() {
            return Ok(profiles);
        }

        for entry in fs::read_dir(&self.profiles_dir).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to read profiles directory: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                NvControlError::ConfigError(format!("Failed to read directory entry: {}", e))
            })?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(bundle) = self.import_profile(&path) {
                    profiles.push(bundle);
                }
            }
        }

        // Sort by creation date (newest first)
        profiles.sort_by_key(|profile| std::cmp::Reverse(profile.created_at));

        Ok(profiles)
    }

    /// Delete a profile bundle
    pub fn delete_profile(&self, name: &str) -> NvResult<()> {
        let safe_filename = self.sanitize_filename(name);
        let path = self.profiles_dir.join(format!("{}.json", safe_filename));

        if path.exists() {
            fs::remove_file(&path).map_err(|e| {
                NvControlError::ConfigError(format!("Failed to delete profile: {}", e))
            })?;
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
            display_layout: None,
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
            display_layout: None,
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
            display_layout: None,
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
            description: format!(
                "Quick save - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            ),
            created_at: chrono::Utc::now(),
            display_layout: None,
            fan_curve,
            voltage_curve,
            overclock,
            game_profiles: vec![],
            vibrance_settings: None,
        };

        self.export_profile(&bundle, Some(name))
    }

    pub fn capture_live_bundle(name: impl Into<String>) -> NvResult<ProfileBundle> {
        let name = name.into();

        let (gpu_clock_offset, memory_clock_offset) =
            crate::overclocking::get_current_offsets(0).unwrap_or((0, 0));
        let power_limit = crate::power::get_power_info()
            .ok()
            .and_then(|infos| infos.into_iter().next())
            .and_then(|info| info.power_limit)
            .map(|limit| limit.round().clamp(0.0, 255.0) as u8)
            .unwrap_or(100);
        let temp_limit = crate::power::get_power_info()
            .ok()
            .and_then(|infos| infos.into_iter().next())
            .and_then(|info| info.temperature)
            .map(|temp| temp.round().clamp(0.0, 125.0) as u8)
            .unwrap_or(83);

        let vibrance_settings = crate::vibrance_native::get_vibrance_status_native()
            .ok()
            .and_then(|status| status.get("connectors").cloned())
            .and_then(|value| {
                serde_json::from_value::<Vec<crate::vibrance_native::ConnectorInfo>>(value).ok()
            })
            .map(|connectors| VibranceSettings {
                display_levels: connectors
                    .into_iter()
                    .map(|connector| connector.current_vibrance as i16)
                    .collect(),
                per_game_vibrance: false,
            });

        let display_layout = MultiMonitorManager::new()
            .and_then(|manager| manager.get_current_layout())
            .or_else(|_| -> NvResult<DisplayLayout> { Ok(fallback_display_layout()) })?;

        Ok(ProfileBundle {
            name: name.clone(),
            description: format!(
                "Live state snapshot captured {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            ),
            created_at: chrono::Utc::now(),
            display_layout: Some(display_layout),
            fan_curve: Some(point_pairs_to_fan_curve(
                &OverclockProfile::default().fan_curve,
            )),
            voltage_curve: None,
            overclock: Some(OverclockProfile {
                name: format!("{}-live", name),
                gpu_clock_offset,
                memory_clock_offset,
                voltage_offset: 0,
                power_limit,
                temp_limit,
                fan_curve: OverclockProfile::default().fan_curve,
            }),
            game_profiles: vec![],
            vibrance_settings,
        })
    }

    pub fn apply_bundle(bundle: &ProfileBundle) -> NvResult<Vec<String>> {
        let mut applied = Vec::new();

        if let Some(overclock) = &bundle.overclock {
            crate::overclocking::apply_overclock_profile(overclock)?;
            applied.push(format!(
                "overclock gpu {:+} mem {:+} power {}",
                overclock.gpu_clock_offset, overclock.memory_clock_offset, overclock.power_limit
            ));

            crate::power::set_power_limit_percentage(overclock.power_limit as u32)?;
            applied.push(format!("power limit {}%", overclock.power_limit));
        }

        if let Some(fan_curve) = &bundle.fan_curve {
            let points = fan_curve_to_point_pairs(fan_curve);
            if !points.is_empty() {
                crate::fan::set_fan_curve(0, &points)?;
                applied.push(format!("fan curve ({} points)", points.len()));
            }
        }

        if let Some(display_layout) = &bundle.display_layout {
            let mut manager = MultiMonitorManager::new()?;
            manager.apply_live_layout(display_layout)?;
            applied.push(format!(
                "display layout ({} displays)",
                display_layout.displays.len()
            ));
        }

        if let Some(vibrance) = &bundle.vibrance_settings {
            for (display_id, level) in vibrance.display_levels.iter().enumerate() {
                let percentage = crate::vibrance_native::vibrance_to_percentage(*level as i64);
                crate::vibrance_native::set_display_vibrance_native(
                    0,
                    display_id as u32,
                    percentage,
                )?;
            }
            applied.push(format!(
                "vibrance ({} displays)",
                vibrance.display_levels.len()
            ));
        }

        Ok(applied)
    }

    pub fn preview_live_bundle(name: impl Into<String>) -> NvResult<String> {
        let bundle = Self::capture_live_bundle(name)?;
        Ok(Self::summarize_bundle(&bundle))
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

    pub fn summarize_bundle(bundle: &ProfileBundle) -> String {
        let mut output = String::new();
        let _ = writeln!(&mut output, "Name: {}", bundle.name);
        let _ = writeln!(&mut output, "Description: {}", bundle.description);
        let _ = writeln!(&mut output, "Created: {}", bundle.created_at);
        let _ = writeln!(
            &mut output,
            "Includes: display_layout={}, fan_curve={}, voltage_curve={}, overclock={}, game_profiles={}, vibrance={}",
            bundle.display_layout.is_some(),
            bundle.fan_curve.is_some(),
            bundle.voltage_curve.is_some(),
            bundle.overclock.is_some(),
            bundle.game_profiles.len(),
            bundle.vibrance_settings.is_some()
        );

        if let Some(layout) = &bundle.display_layout {
            let _ = writeln!(
                &mut output,
                "Display Layout: {} displays [{}]",
                layout.displays.len(),
                layout
                    .displays
                    .iter()
                    .map(|display| display.connector.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        if let Some(overclock) = &bundle.overclock {
            let _ = writeln!(
                &mut output,
                "Overclock: gpu={:+}MHz mem={:+}MHz power={} temp={}C",
                overclock.gpu_clock_offset,
                overclock.memory_clock_offset,
                overclock.power_limit,
                overclock.temp_limit
            );
        }

        if let Some(vibrance) = &bundle.vibrance_settings {
            let _ = writeln!(
                &mut output,
                "Vibrance: displays={} per_game={}",
                vibrance.display_levels.len(),
                vibrance.per_game_vibrance
            );
        }

        if !bundle.game_profiles.is_empty() {
            let _ = writeln!(&mut output, "Game Profiles:");
            for profile in &bundle.game_profiles {
                let _ = writeln!(
                    &mut output,
                    "  - {} [{}] gpu={:?} mem={:?} power={:?} fps={:?}",
                    profile.name,
                    profile.executable,
                    profile.gpu_clock_offset,
                    profile.mem_clock_offset,
                    profile.power_limit,
                    profile.fps_limit
                );
            }
        }

        output.trim_end().to_string()
    }

    pub fn diff_bundles(current: &ProfileBundle, target: &ProfileBundle) -> Vec<String> {
        let mut changes = Vec::new();

        if current.description != target.description {
            changes.push(format!(
                "description: {:?} -> {:?}",
                current.description, target.description
            ));
        }

        match (&current.display_layout, &target.display_layout) {
            (Some(left), Some(right)) => {
                let left_connectors: Vec<_> = left.displays.iter().map(|d| &d.connector).collect();
                let right_connectors: Vec<_> =
                    right.displays.iter().map(|d| &d.connector).collect();
                if left_connectors != right_connectors
                    || left.displays.len() != right.displays.len()
                {
                    changes.push(format!(
                        "display_layout: {:?} -> {:?}",
                        left_connectors, right_connectors
                    ));
                }
            }
            (None, Some(_)) => changes.push("display_layout: none -> configured".to_string()),
            (Some(_), None) => changes.push("display_layout: configured -> none".to_string()),
            _ => {}
        }

        if current.fan_curve.is_some() != target.fan_curve.is_some() {
            changes.push(format!(
                "fan_curve: {} -> {}",
                current.fan_curve.is_some(),
                target.fan_curve.is_some()
            ));
        }

        if current.voltage_curve.is_some() != target.voltage_curve.is_some() {
            changes.push(format!(
                "voltage_curve: {} -> {}",
                current.voltage_curve.is_some(),
                target.voltage_curve.is_some()
            ));
        }

        match (&current.overclock, &target.overclock) {
            (Some(left), Some(right))
                if left.gpu_clock_offset != right.gpu_clock_offset
                    || left.memory_clock_offset != right.memory_clock_offset
                    || left.power_limit != right.power_limit
                    || left.temp_limit != right.temp_limit
                    || left.voltage_offset != right.voltage_offset
                    || left.fan_curve != right.fan_curve =>
            {
                changes.push(format!(
                    "overclock: gpu {:+} -> {:+}, mem {:+} -> {:+}, power {} -> {}",
                    left.gpu_clock_offset,
                    right.gpu_clock_offset,
                    left.memory_clock_offset,
                    right.memory_clock_offset,
                    left.power_limit,
                    right.power_limit
                ))
            }
            (None, Some(_)) => changes.push("overclock: none -> configured".to_string()),
            (Some(_), None) => changes.push("overclock: configured -> none".to_string()),
            _ => {}
        }

        match (&current.vibrance_settings, &target.vibrance_settings) {
            (Some(left), Some(right)) if left.display_levels != right.display_levels => {
                changes.push(format!(
                    "vibrance.display_levels: {:?} -> {:?}",
                    left.display_levels, right.display_levels
                ));
            }
            (None, Some(_)) => changes.push("vibrance: none -> configured".to_string()),
            (Some(_), None) => changes.push("vibrance: configured -> none".to_string()),
            _ => {}
        }

        let current_games: Vec<_> = current
            .game_profiles
            .iter()
            .map(|profile| profile.executable.as_str())
            .collect();
        let target_games: Vec<_> = target
            .game_profiles
            .iter()
            .map(|profile| profile.executable.as_str())
            .collect();

        if current_games != target_games {
            changes.push(format!(
                "game_profiles: {:?} -> {:?}",
                current_games, target_games
            ));
        }

        changes
    }

    pub fn resolve_bundle_reference(&self, value: &str) -> NvResult<ProfileBundle> {
        if value.eq_ignore_ascii_case("live") {
            Self::capture_live_bundle("live")
        } else {
            let path = Path::new(value);
            if path.exists() {
                self.import_profile(path)
            } else {
                self.load_profile_by_name(value)
            }
        }
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
    use crate::game_launcher::ProcessPriority;

    fn sample_bundle(name: &str, gpu_offset: i32) -> ProfileBundle {
        ProfileBundle {
            name: name.to_string(),
            description: format!("bundle for {name}"),
            created_at: chrono::Utc::now(),
            display_layout: Some(DisplayLayout {
                name: format!("{name}-layout"),
                displays: vec![DisplayConfig {
                    name: "DP-1".to_string(),
                    connector: "DP-1".to_string(),
                    enabled: true,
                    resolution: (2560, 1440),
                    refresh_rate: 165,
                    position: (0, 0),
                    scale: 1.0,
                    rotation: crate::multimonitor::Rotation::Normal,
                    vrr_enabled: true,
                    digital_vibrance: 250,
                    color_profile: Some("gaming".to_string()),
                }],
            }),
            fan_curve: None,
            voltage_curve: None,
            overclock: Some(OverclockProfile {
                name: name.to_string(),
                gpu_clock_offset: gpu_offset,
                memory_clock_offset: 200,
                power_limit: 95,
                voltage_offset: 0,
                temp_limit: 85,
                fan_curve: Vec::new(),
            }),
            game_profiles: vec![GameProfile {
                name: name.to_string(),
                executable: format!("{name}.exe"),
                gpu_clock_offset: Some(gpu_offset),
                mem_clock_offset: Some(200),
                power_limit: Some(95),
                vibrance: Some(150),
                fps_limit: Some(144),
                priority: ProcessPriority::High,
                ..Default::default()
            }],
            vibrance_settings: Some(VibranceSettings {
                display_levels: vec![32, 64],
                per_game_vibrance: false,
            }),
        }
    }

    #[test]
    fn test_sanitize_filename() {
        let manager = ProfileManager::new().unwrap();
        assert_eq!(manager.sanitize_filename("test<profile>"), "test_profile_");
        assert_eq!(manager.sanitize_filename("my:profile"), "my_profile");
    }

    #[test]
    fn test_bundle_summary_contains_key_sections() {
        let summary = ProfileManager::summarize_bundle(&sample_bundle("test", 100));
        assert!(summary.contains("Name: test"));
        assert!(summary.contains("Overclock:"));
        assert!(summary.contains("Game Profiles:"));
    }

    #[test]
    fn test_bundle_diff_reports_changes() {
        let current = sample_bundle("current", 75);
        let mut target = sample_bundle("target", 150);
        target.description = "updated".to_string();
        target.vibrance_settings = Some(VibranceSettings {
            display_levels: vec![128],
            per_game_vibrance: true,
        });

        let diff = ProfileManager::diff_bundles(&current, &target);
        assert!(diff.iter().any(|line| line.contains("description")));
        assert!(diff.iter().any(|line| line.contains("overclock")));
        assert!(
            diff.iter()
                .any(|line| line.contains("vibrance.display_levels"))
        );
    }

    #[test]
    fn resolve_live_reference_returns_live_bundle() {
        let manager = ProfileManager::new().unwrap();
        let bundle = manager.resolve_bundle_reference("live").unwrap();
        assert_eq!(bundle.name, "live");
        assert!(bundle.overclock.is_some());
    }
}
