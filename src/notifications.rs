// Desktop Notifications & Alert System
// Temperature warnings, fan failures, GPU alerts

use crate::nvml_backend::SharedNvmlBackend;
use crate::{NvControlError, NvResult};
use notify_rust::{Notification, Timeout, Urgency};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const SUPPORT_BUNDLE_DEDUPE_SECS: u64 = 8;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SupportBundleNotificationState {
    last_path: Option<String>,
    last_sent_unix_secs: Option<u64>,
}

fn notification_icon() -> String {
    let candidate_paths = [
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/nvctl_logo.png"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons/icon-256x256.png"),
        "/usr/share/icons/hicolor/256x256/apps/nvcontrol.png",
        "/usr/share/icons/hicolor/128x128/apps/nvcontrol.png",
        "/usr/share/pixmaps/nvcontrol.png",
        "/usr/share/icons/hicolor/256x256/apps/nvidia.png",
        "/usr/share/pixmaps/nvidia.png",
    ];

    candidate_paths
        .iter()
        .find(|path| Path::new(path).exists())
        .map(|path| (*path).to_string())
        .unwrap_or_else(|| "nvidia".to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub enabled: bool,
    pub temp_warning: u32,          // °C
    pub temp_critical: u32,         // °C
    pub power_warning: u32,         // Watts
    pub fan_failure_threshold: u32, // RPM (if below this, alert)
    pub cooldown_seconds: u64,      // Minimum time between same alerts
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            temp_warning: 85,
            temp_critical: 90,
            power_warning: 350,
            fan_failure_threshold: 500,
            cooldown_seconds: 300, // 5 minutes
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlertType {
    TempWarning,
    TempCritical,
    PowerWarning,
    FanFailure,
    GpuError,
    ProfileApplied,
    OverclockApplied,
}

pub struct NotificationManager {
    config: AlertConfig,
    config_path: PathBuf,
    last_alerts: std::collections::HashMap<AlertType, Instant>,
}

impl NotificationManager {
    pub fn new() -> NvResult<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| NvControlError::ConfigError("Could not find config directory".into()))?
            .join("nvcontrol");

        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("alerts.toml");

        let config = if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            toml::from_str(&contents).unwrap_or_default()
        } else {
            AlertConfig::default()
        };

        Ok(Self {
            config,
            config_path,
            last_alerts: std::collections::HashMap::new(),
        })
    }

    pub fn save_config(&self) -> NvResult<()> {
        let toml_str = toml::to_string_pretty(&self.config).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize config: {}", e))
        })?;
        fs::write(&self.config_path, toml_str)?;
        Ok(())
    }

    /// Check if we should send an alert (respects cooldown)
    fn should_alert(&mut self, alert_type: AlertType) -> bool {
        if !self.config.enabled {
            return false;
        }

        let now = Instant::now();
        let cooldown = Duration::from_secs(self.config.cooldown_seconds);

        if let Some(last_time) = self.last_alerts.get(&alert_type) {
            if now.duration_since(*last_time) < cooldown {
                return false; // Still in cooldown
            }
        }

        self.last_alerts.insert(alert_type, now);
        true
    }

    /// Send a desktop notification
    fn send_notification(
        &self,
        summary: &str,
        body: &str,
        urgency: Urgency,
        timeout: Timeout,
    ) -> NvResult<()> {
        let icon = notification_icon();

        Notification::new()
            .summary(summary)
            .body(body)
            .icon(&icon)
            .appname("nvcontrol")
            .urgency(urgency)
            .timeout(timeout)
            .show()
            .map_err(|e| {
                NvControlError::RuntimeError(format!("Failed to send notification: {}", e))
            })?;

        Ok(())
    }

    /// Check GPU temperature and send alerts if needed
    pub fn check_temperature(&mut self, temp: f32) -> NvResult<()> {
        let temp_u32 = temp as u32;

        if temp_u32 >= self.config.temp_critical {
            if self.should_alert(AlertType::TempCritical) {
                self.send_notification(
                    "🔥 CRITICAL GPU Temperature!",
                    &format!(
                        "GPU temperature is {}°C! Critical threshold: {}°C\n\
                         Consider stopping intensive tasks or improving cooling.",
                        temp_u32, self.config.temp_critical
                    ),
                    Urgency::Critical,
                    Timeout::Never,
                )?;
            }
        } else if temp_u32 >= self.config.temp_warning {
            if self.should_alert(AlertType::TempWarning) {
                self.send_notification(
                    "⚠️  High GPU Temperature",
                    &format!(
                        "GPU temperature is {}°C. Warning threshold: {}°C\n\
                         Monitor your system closely.",
                        temp_u32, self.config.temp_warning
                    ),
                    Urgency::Normal,
                    Timeout::Milliseconds(5000),
                )?;
            }
        }

        Ok(())
    }

    /// Check GPU power draw
    pub fn check_power(&mut self, power: f32) -> NvResult<()> {
        let power_u32 = power as u32;

        if power_u32 >= self.config.power_warning {
            if self.should_alert(AlertType::PowerWarning) {
                self.send_notification(
                    "⚡ High GPU Power Draw",
                    &format!(
                        "GPU is drawing {} watts. Warning threshold: {} watts\n\
                         This may indicate heavy load or inefficient settings.",
                        power_u32, self.config.power_warning
                    ),
                    Urgency::Normal,
                    Timeout::Milliseconds(5000),
                )?;
            }
        }

        Ok(())
    }

    /// Check fan status
    pub fn check_fan(&mut self, fan_rpm: u32) -> NvResult<()> {
        if fan_rpm > 0 && fan_rpm < self.config.fan_failure_threshold {
            if self.should_alert(AlertType::FanFailure) {
                self.send_notification(
                    "🌀 Fan Speed Warning",
                    &format!(
                        "GPU fan is running at {} RPM (very low!)\n\
                         Check for fan failure or obstruction.\n\
                         GPU may be at risk of overheating.",
                        fan_rpm
                    ),
                    Urgency::Critical,
                    Timeout::Never,
                )?;
            }
        }

        Ok(())
    }

    /// Send GPU error notification
    pub fn notify_gpu_error(&mut self, error: &str) -> NvResult<()> {
        if self.should_alert(AlertType::GpuError) {
            self.send_notification(
                "❌ GPU Error",
                &format!("An error occurred with your GPU:\n{}", error),
                Urgency::Critical,
                Timeout::Never,
            )?;
        }
        Ok(())
    }

    /// Send profile applied notification
    pub fn notify_profile_applied(&mut self, profile_name: &str) -> NvResult<()> {
        if self.should_alert(AlertType::ProfileApplied) {
            self.send_notification(
                "🎮 Profile Applied",
                &format!("Game profile \"{}\" has been applied", profile_name),
                Urgency::Low,
                Timeout::Milliseconds(3000),
            )?;
        }
        Ok(())
    }

    /// Notify that a support bundle was created
    pub fn notify_support_bundle_created(&mut self, path: &str) -> NvResult<()> {
        if std::env::var("NVCONTROL_SUPPRESS_SUPPORT_NOTIFICATIONS")
            .ok()
            .as_deref()
            == Some("1")
        {
            return Ok(());
        }

        if !self.should_notify_support_bundle(path) {
            return Ok(());
        }

        let display_name = Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(path);

        self.send_notification(
            "nvcontrol Support Bundle Ready",
            &format!("Saved: {}", display_name),
            Urgency::Low,
            Timeout::Milliseconds(4000),
        )
    }

    /// Send overclock applied notification
    pub fn notify_overclock_applied(&mut self, gpu_offset: i32, mem_offset: i32) -> NvResult<()> {
        if self.should_alert(AlertType::OverclockApplied) {
            self.send_notification(
                "⚡ Overclock Applied",
                &format!("GPU: {:+} MHz, Memory: {:+} MHz", gpu_offset, mem_offset),
                Urgency::Low,
                Timeout::Milliseconds(3000),
            )?;
        }
        Ok(())
    }

    /// Check all GPU metrics at once
    pub fn check_all_metrics(&mut self, temp: f32, power: f32, fan_rpm: u32) -> NvResult<()> {
        self.check_temperature(temp)?;
        self.check_power(power)?;
        self.check_fan(fan_rpm)?;
        Ok(())
    }

    pub fn set_temp_warning(&mut self, temp: u32) -> NvResult<()> {
        self.config.temp_warning = temp;
        self.save_config()
    }

    pub fn set_temp_critical(&mut self, temp: u32) -> NvResult<()> {
        self.config.temp_critical = temp;
        self.save_config()
    }

    pub fn set_power_warning(&mut self, power: u32) -> NvResult<()> {
        self.config.power_warning = power;
        self.save_config()
    }

    pub fn enable(&mut self) -> NvResult<()> {
        self.config.enabled = true;
        self.save_config()
    }

    pub fn disable(&mut self) -> NvResult<()> {
        self.config.enabled = false;
        self.save_config()
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    pub fn get_config(&self) -> &AlertConfig {
        &self.config
    }

    /// Send a test notification
    pub fn send_test_notification(&self) -> NvResult<()> {
        self.send_notification(
            "nvcontrol Test Notification",
            "Desktop notifications are working correctly! 🎉",
            Urgency::Normal,
            Timeout::Milliseconds(5000),
        )
    }

    fn support_bundle_state_path(&self) -> PathBuf {
        self.config_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("support-notify.toml")
    }

    fn load_support_bundle_state(&self) -> SupportBundleNotificationState {
        let path = self.support_bundle_state_path();
        fs::read_to_string(path)
            .ok()
            .and_then(|contents| toml::from_str(&contents).ok())
            .unwrap_or_default()
    }

    fn save_support_bundle_state(&self, state: &SupportBundleNotificationState) -> NvResult<()> {
        let path = self.support_bundle_state_path();
        let contents = toml::to_string(state).map_err(|e| {
            NvControlError::ConfigError(format!("Failed to serialize support notify state: {}", e))
        })?;
        fs::write(path, contents)?;
        Ok(())
    }

    fn should_notify_support_bundle(&self, path: &str) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let state = self.load_support_bundle_state();

        if state.last_path.as_deref() == Some(path)
            && state
                .last_sent_unix_secs
                .map(|last| now.saturating_sub(last) < SUPPORT_BUNDLE_DEDUPE_SECS)
                .unwrap_or(false)
        {
            return false;
        }

        let new_state = SupportBundleNotificationState {
            last_path: Some(path.to_string()),
            last_sent_unix_secs: Some(now),
        };
        let _ = self.save_support_bundle_state(&new_state);
        true
    }
}

/// Background monitoring thread that continuously checks GPU metrics
pub struct AlertMonitorThread {
    notification_manager: NotificationManager,
    check_interval_ms: u64,
    backend: SharedNvmlBackend,
}

impl AlertMonitorThread {
    /// Create monitor with shared backend (preferred)
    pub fn with_backend(check_interval_ms: u64, backend: SharedNvmlBackend) -> NvResult<Self> {
        Ok(Self {
            notification_manager: NotificationManager::new()?,
            check_interval_ms,
            backend,
        })
    }

    /// Create monitor (legacy - creates own backend)
    pub fn new(check_interval_ms: u64) -> NvResult<Self> {
        Self::with_backend(
            check_interval_ms,
            crate::nvml_backend::create_real_backend(),
        )
    }

    /// Run the monitoring loop (blocking)
    pub fn run(&mut self) -> NvResult<()> {
        println!("🔔 Alert monitoring started");

        loop {
            // Get GPU stats via backend
            if self.backend.is_available() {
                let temp = self.backend.get_temperature(0).unwrap_or(0) as f32;
                let power = self
                    .backend
                    .get_power_usage(0)
                    .map(|p| p as f32 / 1000.0)
                    .unwrap_or(0.0);
                let fan_rpm = self.backend.get_fan_speed(0, 0).unwrap_or(0);

                // Check all metrics
                let _ = self
                    .notification_manager
                    .check_all_metrics(temp, power, fan_rpm);
            }

            std::thread::sleep(std::time::Duration::from_millis(self.check_interval_ms));
        }
    }
}
