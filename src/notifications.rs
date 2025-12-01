// Desktop Notifications & Alert System
// Temperature warnings, fan failures, GPU alerts

use crate::{NvControlError, NvResult};
use notify_rust::{Notification, Timeout, Urgency};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

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
        Notification::new()
            .summary(summary)
            .body(body)
            .icon("nvcontrol")
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
}

/// Background monitoring thread that continuously checks GPU metrics
pub struct AlertMonitorThread {
    notification_manager: NotificationManager,
    check_interval_ms: u64,
}

impl AlertMonitorThread {
    pub fn new(check_interval_ms: u64) -> NvResult<Self> {
        Ok(Self {
            notification_manager: NotificationManager::new()?,
            check_interval_ms,
        })
    }

    /// Run the monitoring loop (blocking)
    pub fn run(&mut self) -> NvResult<()> {
        println!("🔔 Alert monitoring started");

        loop {
            // Get GPU stats
            if let Ok(nvml) = nvml_wrapper::Nvml::init() {
                if let Ok(device) = nvml.device_by_index(0) {
                    let temp = device
                        .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                        .unwrap_or(0) as f32;

                    let power = device
                        .power_usage()
                        .map(|p| p as f32 / 1000.0)
                        .unwrap_or(0.0);

                    let fan_rpm = device.fan_speed(0).unwrap_or(0);

                    // Check all metrics
                    let _ = self
                        .notification_manager
                        .check_all_metrics(temp, power, fan_rpm);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(self.check_interval_ms));
        }
    }
}
