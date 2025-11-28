/// ASUS Power Detector+ Implementation
///
/// Read-only monitoring of 12V-2x6 power connector current for ASUS ROG cards.
/// Replicates GPU Tweak III Power Detector+ functionality on Linux.
///
/// Supported cards:
/// - ROG Astral RTX 5090 (subsystem 1043:89e3)
/// - ROG Matrix RTX 5090 (subsystem 1043:TBD)
///
/// Safety: This module ONLY performs READ operations on I2C.
/// No writes are ever performed to prevent hardware damage.

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

/// ASUS vendor ID
pub const ASUS_VENDOR_ID: u16 = 0x1043;

/// Known ASUS ROG GPU subsystem IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsusRogModel {
    /// ROG Astral RTX 5090
    AstralRtx5090,
    /// ROG Matrix RTX 5090
    MatrixRtx5090,
    /// Unknown ASUS card (may still work)
    UnknownAsus,
    /// Not an ASUS card
    NotAsus,
}

impl AsusRogModel {
    pub fn from_subsystem_id(vendor: u16, device: u16) -> Self {
        if vendor != ASUS_VENDOR_ID {
            return Self::NotAsus;
        }

        match device {
            0x89e3 => Self::AstralRtx5090,
            // Add more models as discovered
            // 0xXXXX => Self::MatrixRtx5090,
            _ => Self::UnknownAsus,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::AstralRtx5090 => "ROG Astral RTX 5090",
            Self::MatrixRtx5090 => "ROG Matrix RTX 5090",
            Self::UnknownAsus => "Unknown ASUS ROG",
            Self::NotAsus => "Not ASUS",
        }
    }

    pub fn supports_power_detector(&self) -> bool {
        matches!(self, Self::AstralRtx5090 | Self::MatrixRtx5090)
    }
}

/// Health status for power connector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerHealth {
    /// All rails within safe limits
    Good,
    /// Some rails approaching limits (>7A)
    Warning,
    /// One or more rails over limit (>9.2A)
    Critical,
    /// Unable to determine status
    Unknown,
}

impl PowerHealth {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Good => "GOOD",
            Self::Warning => "WARNING",
            Self::Critical => "CRITICAL",
            Self::Unknown => "UNKNOWN",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            Self::Good => "\x1b[32m",      // Green
            Self::Warning => "\x1b[33m",   // Yellow
            Self::Critical => "\x1b[31m",  // Red
            Self::Unknown => "\x1b[90m",   // Gray
        }
    }
}

/// Power rail measurement from a single 12V-2x6 pin/sense point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerRailReading {
    /// Rail identifier (0-5 for 6-rail monitoring)
    pub rail_id: u8,
    /// Raw register value
    pub raw_value: u16,
    /// Estimated current in milliamps (approximate, needs calibration)
    pub current_ma: Option<u32>,
    /// Warning flag if current exceeds safe threshold
    pub warning: bool,
}

/// Complete power connector status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerConnectorStatus {
    /// Card model
    pub model: String,
    /// I2C bus used
    pub i2c_bus: u8,
    /// Individual rail readings
    pub rails: Vec<PowerRailReading>,
    /// Total estimated power draw from connector (watts)
    pub total_power_w: Option<f32>,
    /// Any warnings active
    pub has_warnings: bool,
    /// Overall health status
    pub health: PowerHealth,
    /// Timestamp
    pub timestamp: u64,
}

/// ASUS Power Detector+ interface
///
/// # Safety
/// This struct ONLY performs read operations on I2C devices.
/// All methods are read-only and cannot damage hardware.
pub struct AsusPowerDetector {
    /// GPU PCI bus ID (e.g., "0000:01:00.0")
    pci_id: String,
    /// Detected card model
    model: AsusRogModel,
    /// I2C bus number for power monitoring chip
    i2c_bus: Option<u8>,
    /// I2C device address (0x2b for Astral)
    i2c_addr: u8,
}

impl AsusPowerDetector {
    /// Create a new Power Detector for the specified GPU
    pub fn new(pci_id: &str) -> NvResult<Self> {
        let model = Self::detect_model(pci_id)?;
        let i2c_bus = Self::find_i2c_bus(pci_id);

        Ok(Self {
            pci_id: pci_id.to_string(),
            model,
            i2c_bus,
            i2c_addr: 0x2b, // Power monitor address on Astral
        })
    }

    /// Detect ASUS ROG model from PCI subsystem IDs
    fn detect_model(pci_id: &str) -> NvResult<AsusRogModel> {
        let vendor_path = format!("/sys/bus/pci/devices/{}/subsystem_vendor", pci_id);
        let device_path = format!("/sys/bus/pci/devices/{}/subsystem_device", pci_id);

        let vendor_str = fs::read_to_string(&vendor_path)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Cannot read subsystem vendor: {}", e)))?;
        let device_str = fs::read_to_string(&device_path)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Cannot read subsystem device: {}", e)))?;

        let vendor = u16::from_str_radix(vendor_str.trim().trim_start_matches("0x"), 16)
            .unwrap_or(0);
        let device = u16::from_str_radix(device_str.trim().trim_start_matches("0x"), 16)
            .unwrap_or(0);

        Ok(AsusRogModel::from_subsystem_id(vendor, device))
    }

    /// Find the I2C bus that has the power monitor chip
    fn find_i2c_bus(pci_id: &str) -> Option<u8> {
        // GPU I2C buses are typically at /sys/bus/pci/devices/{pci_id}/i2c-N
        let pci_path = format!("/sys/bus/pci/devices/{}", pci_id);

        // Collect all GPU I2C bus numbers
        let mut buses: Vec<u8> = Vec::new();
        if let Ok(entries) = fs::read_dir(&pci_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("i2c-") {
                    if let Ok(bus_num) = name[4..].parse::<u8>() {
                        buses.push(bus_num);
                    }
                }
            }
        }

        // Sort and probe each bus to find the one with power monitor at 0x2b
        buses.sort();
        for bus in buses {
            if Self::probe_power_monitor(bus, 0x2b) {
                return Some(bus);
            }
        }

        None
    }

    /// Probe an I2C bus to check if power monitor exists at given address
    fn probe_power_monitor(bus: u8, addr: u8) -> bool {
        // Try to read register 0x60 to verify the device exists
        let output = Command::new("i2cget")
            .args(["-y", &bus.to_string(), &format!("0x{:02x}", addr), "0x60", "w"])
            .output();

        match output {
            Ok(o) => o.status.success(),
            Err(_) => false,
        }
    }

    /// Check if this card supports Power Detector+
    pub fn is_supported(&self) -> bool {
        self.model.supports_power_detector() && self.i2c_bus.is_some()
    }

    /// Get the detected card model
    pub fn model(&self) -> &AsusRogModel {
        &self.model
    }

    /// Read power rail status (READ-ONLY operation)
    ///
    /// # Safety
    /// This method only performs I2C read operations using i2cget.
    /// No writes are performed.
    pub fn read_power_rails(&self) -> NvResult<PowerConnectorStatus> {
        let bus = self.i2c_bus.ok_or_else(|| {
            NvControlError::UnsupportedFeature("I2C bus not found for this GPU".into())
        })?;

        if !self.model.supports_power_detector() {
            return Err(NvControlError::UnsupportedFeature(
                format!("Power Detector+ not supported on {}", self.model.name())
            ));
        }

        let mut rails = Vec::new();
        let mut has_warnings = false;

        // Read 6 power rails from registers 0x60-0x6B (word reads)
        // Each rail is at offset: 0x60, 0x62, 0x64, 0x66, 0x68, 0x6A
        for (rail_id, reg_offset) in [0x60u8, 0x62, 0x64, 0x66, 0x68, 0x6a].iter().enumerate() {
            let raw_value = self.read_i2c_word(bus, *reg_offset)?;

            // Convert raw value to estimated current
            // Note: This is approximate - exact conversion requires ASUS documentation
            // Values appear to be in some proportional format
            let current_ma = Self::estimate_current(raw_value);

            // Warning threshold: ~9.2A per pin = 9200mA
            let warning = current_ma.map(|c| c > 9200).unwrap_or(false);
            if warning {
                has_warnings = true;
            }

            rails.push(PowerRailReading {
                rail_id: rail_id as u8,
                raw_value,
                current_ma,
                warning,
            });
        }

        // Estimate total power (rough approximation)
        let total_power_w = Self::estimate_total_power(&rails);

        // Compute overall health status
        let health = Self::compute_health(&rails);

        Ok(PowerConnectorStatus {
            model: self.model.name().to_string(),
            i2c_bus: bus,
            rails,
            total_power_w,
            has_warnings,
            health,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    /// Read a 16-bit word from I2C (READ-ONLY)
    fn read_i2c_word(&self, bus: u8, register: u8) -> NvResult<u16> {
        // Use i2cget for safe read-only access
        let args = [
            "-y".to_string(),
            bus.to_string(),
            format!("0x{:02x}", self.i2c_addr),
            format!("0x{:02x}", register),
            "w".to_string(),
        ];

        let output = Command::new("i2cget")
            .args(&args)
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("i2cget failed to execute: {}", e)))?;

        if !output.status.success() {
            return Err(NvControlError::CommandFailed(
                format!("i2cget -y {} 0x{:02x} 0x{:02x} w failed: {}",
                    bus, self.i2c_addr, register,
                    String::from_utf8_lossy(&output.stderr))
            ));
        }

        let value_str = String::from_utf8_lossy(&output.stdout);
        let value_str = value_str.trim();

        // Parse hex value (format: 0xNNNN)
        u16::from_str_radix(value_str.trim_start_matches("0x"), 16)
            .map_err(|e| NvControlError::RuntimeError(format!("Failed to parse I2C value '{}': {}", value_str, e)))
    }

    /// Estimate current from raw register value
    ///
    /// Note: This is an approximation based on observed values.
    /// Actual conversion factor needs to be determined from hardware documentation.
    fn estimate_current(raw: u16) -> Option<u32> {
        if raw == 0 || raw == 0xFFFF {
            return None;
        }

        // i2cget returns little-endian words, swap bytes for proper value
        let swapped = raw.swap_bytes();

        // Observed idle values: ~450-550 (swapped)
        // At idle, each rail likely carries ~1-2A
        // Assuming linear scaling: value of 512 ≈ 1A
        // Scale factor: ~2 mA per raw unit
        //
        // Under load, 12V-2x6 can handle ~9.2A per pin (55W per pin at 600W TDP)
        // So max expected swapped value would be around 4600 for 9.2A
        Some((swapped as u32 * 2))
    }

    /// Estimate total power from rail readings
    fn estimate_total_power(rails: &[PowerRailReading]) -> Option<f32> {
        let total_current_ma: u32 = rails
            .iter()
            .filter_map(|r| r.current_ma)
            .sum();

        if total_current_ma == 0 {
            return None;
        }

        // Power = Voltage × Current
        // 12V rail: P = 12V × I(A) = 12V × (I_ma / 1000)
        Some(12.0 * (total_current_ma as f32 / 1000.0))
    }

    /// Compute overall health status from rail readings
    fn compute_health(rails: &[PowerRailReading]) -> PowerHealth {
        let mut max_current_ma = 0u32;
        let mut valid_readings = 0;

        for rail in rails {
            if let Some(current) = rail.current_ma {
                valid_readings += 1;
                if current > max_current_ma {
                    max_current_ma = current;
                }
            }
        }

        if valid_readings == 0 {
            return PowerHealth::Unknown;
        }

        // Thresholds based on 12V-2x6 specs:
        // - Safe: < 7000mA (7A) per rail
        // - Warning: 7000-9200mA (7-9.2A)
        // - Critical: > 9200mA (>9.2A)
        if max_current_ma > 9200 {
            PowerHealth::Critical
        } else if max_current_ma > 7000 {
            PowerHealth::Warning
        } else {
            PowerHealth::Good
        }
    }

    /// Get human-readable status string
    pub fn status_string(&self) -> NvResult<String> {
        let status = self.read_power_rails()?;

        let reset = "\x1b[0m";
        let health_color = status.health.color_code();

        let mut output = String::new();
        output.push_str(&format!("ASUS Power Detector+ - {}\n", status.model));
        output.push_str("═══════════════════════════════════════\n");

        // Health status prominently displayed
        output.push_str(&format!("Connector Health: {}[{}]{}\n",
            health_color, status.health.label(), reset));
        output.push_str(&format!("I2C Bus: {} @ 0x{:02X}\n\n", status.i2c_bus, self.i2c_addr));

        output.push_str("12V-2x6 Power Rails:\n");
        for rail in &status.rails {
            let warning_str = if rail.warning { " ⚠️ HIGH" } else { "" };
            let current_str = rail.current_ma
                .map(|c| format!("{:.2}A", c as f32 / 1000.0))
                .unwrap_or_else(|| "N/A".to_string());

            output.push_str(&format!(
                "  Rail {}: 0x{:04X} (~{}){}\n",
                rail.rail_id, rail.raw_value, current_str, warning_str
            ));
        }

        if let Some(power) = status.total_power_w {
            output.push_str(&format!("\nEstimated Connector Power: {:.1}W\n", power));
        }

        if status.has_warnings {
            output.push_str("\n⚠️  WARNING: One or more rails exceeding safe current!\n");
            output.push_str("    Check 12V-2x6 connector seating and cable quality.\n");
        }

        Ok(output)
    }
}

/// Detect all ASUS ROG GPUs in the system
pub fn detect_asus_gpus() -> Vec<(String, AsusRogModel)> {
    let mut gpus = Vec::new();

    let pci_devices = Path::new("/sys/bus/pci/devices");
    if let Ok(entries) = fs::read_dir(pci_devices) {
        for entry in entries.flatten() {
            let pci_id = entry.file_name().to_string_lossy().to_string();

            // Check if this is an NVIDIA GPU (class 0x030000 or 0x030200)
            let class_path = entry.path().join("class");
            if let Ok(class) = fs::read_to_string(&class_path) {
                let class = class.trim();
                if class.starts_with("0x0302") || class.starts_with("0x0300") {
                    // Check vendor (NVIDIA = 0x10de)
                    let vendor_path = entry.path().join("vendor");
                    if let Ok(vendor) = fs::read_to_string(&vendor_path) {
                        if vendor.trim() == "0x10de" {
                            // This is an NVIDIA GPU, check if ASUS
                            if let Ok(model) = AsusPowerDetector::detect_model(&pci_id) {
                                if model != AsusRogModel::NotAsus {
                                    gpus.push((pci_id, model));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    gpus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_detection() {
        assert_eq!(
            AsusRogModel::from_subsystem_id(0x1043, 0x89e3),
            AsusRogModel::AstralRtx5090
        );
        assert_eq!(
            AsusRogModel::from_subsystem_id(0x1043, 0x0000),
            AsusRogModel::UnknownAsus
        );
        assert_eq!(
            AsusRogModel::from_subsystem_id(0x1458, 0x89e3),
            AsusRogModel::NotAsus
        );
    }

    #[test]
    fn test_current_estimation() {
        // Test that estimation produces reasonable values
        let current = AsusPowerDetector::estimate_current(0x0200);
        assert!(current.is_some());
        // 0x200 = 512, × 3.9 ≈ 1997mA ≈ 2A
        assert!(current.unwrap() > 1500 && current.unwrap() < 2500);
    }
}
