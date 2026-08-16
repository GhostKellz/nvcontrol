/// ASUS Power Detector+ Implementation
///
/// Read-only monitoring of 12V-2x6 power connector voltage and current for
/// supported ASUS ROG Astral cards.
///
/// Supported cards:
/// The IT8915 telemetry format was independently documented by the Linux
/// community and verified against HWiNFO. Prefer the standard `astral12vhpwr`
/// hwmon interface when available; otherwise use the same read-only SMBus
/// transaction directly.
///
/// Safety: This module ONLY performs READ operations on I2C.
/// No writes are ever performed to prevent hardware damage.
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// ASUS vendor ID
pub const ASUS_VENDOR_ID: u16 = 0x1043;

/// Known ASUS ROG GPU subsystem IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsusRogModel {
    /// ROG Astral RTX 5090 OC (verified on Linux)
    AstralRtx5090,
    AstralRtx5090D,
    AstralRtx5090Lc,
    AstralRtx5090White,
    AstralRtx5080,
    AstralRtx5080White,
    AstralRtx5080Miku,
    /// ROG Astral RTX 5090 Matrix
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
            0x89ea => Self::AstralRtx5090D,
            0x89ec => Self::AstralRtx5090Lc,
            0x8a2e => Self::AstralRtx5090White,
            0x89de => Self::AstralRtx5080,
            0x8a2b => Self::AstralRtx5080White,
            0x8a45 => Self::AstralRtx5080Miku,
            0x8a61 => Self::MatrixRtx5090,
            _ => Self::UnknownAsus,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::AstralRtx5090 => "ROG Astral RTX 5090",
            Self::AstralRtx5090D => "ROG Astral RTX 5090D OC",
            Self::AstralRtx5090Lc => "ROG Astral RTX 5090 LC",
            Self::AstralRtx5090White => "ROG Astral RTX 5090 OC White",
            Self::AstralRtx5080 => "ROG Astral RTX 5080 OC",
            Self::AstralRtx5080White => "ROG Astral RTX 5080 OC White",
            Self::AstralRtx5080Miku => "ROG Astral RTX 5080 OC Hatsune Miku",
            Self::MatrixRtx5090 => "ROG Matrix RTX 5090",
            Self::UnknownAsus => "Unknown ASUS ROG",
            Self::NotAsus => "Not ASUS",
        }
    }

    pub fn supports_power_detector(&self) -> bool {
        matches!(
            self,
            Self::AstralRtx5090
                | Self::AstralRtx5090D
                | Self::AstralRtx5090Lc
                | Self::AstralRtx5090White
                | Self::AstralRtx5080
                | Self::AstralRtx5080White
                | Self::AstralRtx5080Miku
                | Self::MatrixRtx5090
        )
    }

    pub fn capability_note(&self) -> &'static str {
        match self {
            Self::AstralRtx5090 => "Power Detector+ verified on the read-only IT8915 path",
            Self::AstralRtx5090D
            | Self::AstralRtx5090Lc
            | Self::AstralRtx5090White
            | Self::AstralRtx5080
            | Self::AstralRtx5080White
            | Self::AstralRtx5080Miku
            | Self::MatrixRtx5090 => {
                "Known Astral subsystem ID; telemetry layout is inherited but not yet independently verified on this SKU"
            }
            Self::UnknownAsus => {
                "ASUS GPU detected, but board-specific Power Detector+ support is not yet confirmed"
            }
            Self::NotAsus => "Not an ASUS GPU",
        }
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
            Self::Good => "\x1b[32m",     // Green
            Self::Warning => "\x1b[33m",  // Yellow
            Self::Critical => "\x1b[31m", // Red
            Self::Unknown => "\x1b[90m",  // Gray
        }
    }
}

/// Power rail measurement from a single 12V-2x6 pin/sense point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerRailReading {
    /// Rail identifier (0-5 for 6-rail monitoring)
    pub rail_id: u8,
    /// Raw current field, retained for serialization compatibility
    pub raw_value: u16,
    /// Measured voltage in millivolts
    pub voltage_mv: Option<u32>,
    /// Measured current in milliamps
    pub current_ma: Option<u32>,
    /// Measured power for this pin in watts
    pub power_w: Option<f32>,
    /// Warning flag if current exceeds safe threshold
    pub warning: bool,
}

/// Complete power connector status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerConnectorStatus {
    /// Card model
    pub model: String,
    /// I2C bus used
    pub i2c_bus: u32,
    /// Telemetry backend in use (`hwmon` or direct `SMBus`)
    pub source: String,
    /// Individual rail readings
    pub rails: Vec<PowerRailReading>,
    /// Total estimated power draw from connector (watts)
    pub total_power_w: Option<f32>,
    /// Minimum-to-maximum pin-current ratio under meaningful connector load
    pub current_balance_percent: Option<f32>,
    /// Any warnings active
    pub has_warnings: bool,
    /// Overall health status
    pub health: PowerHealth,
    /// Timestamp
    pub timestamp: u64,
}

/// Maximum number of historical samples to keep
pub const POWER_HISTORY_SIZE: usize = 60;

/// Trend direction for power readings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerTrend {
    /// Power draw is increasing
    Rising,
    /// Power draw is stable
    Stable,
    /// Power draw is decreasing
    Falling,
    /// Not enough data to determine trend
    Unknown,
}

impl PowerTrend {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Rising => "↑ Rising",
            Self::Stable => "→ Stable",
            Self::Falling => "↓ Falling",
            Self::Unknown => "? Unknown",
        }
    }
}

/// Historical power reading with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerHistorySample {
    /// Time since start of monitoring
    pub elapsed_ms: u64,
    /// Per-rail current readings (mA)
    pub rail_currents: Vec<u32>,
    /// Total estimated power (W)
    pub total_power_w: f32,
    /// Health status at this sample
    pub health: PowerHealth,
}

/// Power history buffer with trend analysis
#[derive(Debug, Clone)]
pub struct PowerHistory {
    /// Circular buffer of power samples
    samples: VecDeque<PowerHistorySample>,
    /// Start time of monitoring
    start_time: Instant,
    /// Last sample time (for rate limiting)
    last_sample: Option<Instant>,
    /// Minimum interval between samples
    sample_interval: Duration,
}

impl Default for PowerHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerHistory {
    /// Create a new power history buffer
    pub fn new() -> Self {
        Self {
            samples: VecDeque::with_capacity(POWER_HISTORY_SIZE),
            start_time: Instant::now(),
            last_sample: None,
            sample_interval: Duration::from_secs(1),
        }
    }

    /// Create with custom sample interval
    pub fn with_interval(interval: Duration) -> Self {
        Self {
            samples: VecDeque::with_capacity(POWER_HISTORY_SIZE),
            start_time: Instant::now(),
            last_sample: None,
            sample_interval: interval,
        }
    }

    /// Add a new sample from a PowerConnectorStatus reading
    pub fn record(&mut self, status: &PowerConnectorStatus) {
        // Rate limit samples
        if let Some(last) = self.last_sample {
            if last.elapsed() < self.sample_interval {
                return;
            }
        }

        let rail_currents: Vec<u32> = status.rails.iter().filter_map(|r| r.current_ma).collect();

        let sample = PowerHistorySample {
            elapsed_ms: self.start_time.elapsed().as_millis() as u64,
            rail_currents,
            total_power_w: status.total_power_w.unwrap_or(0.0),
            health: status.health,
        };

        // Maintain buffer size
        if self.samples.len() >= POWER_HISTORY_SIZE {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);
        self.last_sample = Some(Instant::now());
    }

    /// Get number of samples in buffer
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Get all samples for analysis
    pub fn samples(&self) -> &VecDeque<PowerHistorySample> {
        &self.samples
    }

    /// Get the most recent sample
    pub fn latest(&self) -> Option<&PowerHistorySample> {
        self.samples.back()
    }

    /// Calculate average power over the history
    pub fn average_power(&self) -> Option<f32> {
        if self.samples.is_empty() {
            return None;
        }
        let sum: f32 = self.samples.iter().map(|s| s.total_power_w).sum();
        Some(sum / self.samples.len() as f32)
    }

    /// Get peak power in the history
    pub fn peak_power(&self) -> Option<f32> {
        self.samples
            .iter()
            .map(|s| s.total_power_w)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Get minimum power in the history
    pub fn min_power(&self) -> Option<f32> {
        self.samples
            .iter()
            .map(|s| s.total_power_w)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Analyze power trend over recent samples
    pub fn trend(&self) -> PowerTrend {
        // Need at least 5 samples for meaningful trend
        if self.samples.len() < 5 {
            return PowerTrend::Unknown;
        }

        // Compare last 5 samples with previous 5
        let len = self.samples.len();
        let recent: Vec<f32> = self
            .samples
            .iter()
            .skip(len.saturating_sub(5))
            .map(|s| s.total_power_w)
            .collect();
        let older: Vec<f32> = self
            .samples
            .iter()
            .skip(len.saturating_sub(10))
            .take(5)
            .map(|s| s.total_power_w)
            .collect();

        if older.is_empty() || recent.is_empty() {
            return PowerTrend::Unknown;
        }

        let recent_avg: f32 = recent.iter().sum::<f32>() / recent.len() as f32;
        let older_avg: f32 = older.iter().sum::<f32>() / older.len() as f32;

        // 5% threshold for trend detection
        let threshold = older_avg * 0.05;

        if recent_avg > older_avg + threshold {
            PowerTrend::Rising
        } else if recent_avg < older_avg - threshold {
            PowerTrend::Falling
        } else {
            PowerTrend::Stable
        }
    }

    /// Get per-rail current averages
    pub fn rail_averages(&self) -> Vec<f32> {
        if self.samples.is_empty() {
            return Vec::new();
        }

        // Find max number of rails across samples
        let max_rails = self
            .samples
            .iter()
            .map(|s| s.rail_currents.len())
            .max()
            .unwrap_or(0);

        (0..max_rails)
            .map(|rail_idx| {
                let sum: u32 = self
                    .samples
                    .iter()
                    .filter_map(|s| s.rail_currents.get(rail_idx))
                    .sum();
                let count = self
                    .samples
                    .iter()
                    .filter(|s| s.rail_currents.get(rail_idx).is_some())
                    .count();
                if count > 0 {
                    sum as f32 / count as f32
                } else {
                    0.0
                }
            })
            .collect()
    }

    /// Check if any warning conditions occurred in history
    pub fn had_warnings(&self) -> bool {
        self.samples
            .iter()
            .any(|s| matches!(s.health, PowerHealth::Warning | PowerHealth::Critical))
    }

    /// Count of warning samples
    pub fn warning_count(&self) -> usize {
        self.samples
            .iter()
            .filter(|s| matches!(s.health, PowerHealth::Warning | PowerHealth::Critical))
            .count()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.samples.clear();
        self.start_time = Instant::now();
        self.last_sample = None;
    }

    /// Export history to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.samples.iter().collect::<Vec<_>>())
    }
}

/// Thread-safe power history wrapper
pub type SharedPowerHistory = Arc<Mutex<PowerHistory>>;

/// Create a new shared power history buffer
pub fn create_shared_history() -> SharedPowerHistory {
    Arc::new(Mutex::new(PowerHistory::new()))
}

const IT8915_ADDRESS: u8 = 0x2b;
const IT8915_REGISTER: u8 = 0x80;
const IT8915_FRAME_LEN: usize = 24;
const I2C_SLAVE: libc::c_ulong = 0x0703;
const I2C_SMBUS: libc::c_ulong = 0x0720;
const I2C_SMBUS_READ: u8 = 1;
const I2C_SMBUS_I2C_BLOCK_DATA: u32 = 8;
const I2C_SMBUS_BLOCK_MAX: usize = 32;

#[derive(Debug, Clone)]
enum TelemetrySource {
    Hwmon { path: PathBuf, bus: u32 },
    DirectSmbus { bus: u32 },
}

impl TelemetrySource {
    fn bus(&self) -> u32 {
        match self {
            Self::Hwmon { bus, .. } | Self::DirectSmbus { bus } => *bus,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Hwmon { .. } => "hwmon (astral12vhpwr)",
            Self::DirectSmbus { .. } => "direct SMBus",
        }
    }
}

#[repr(C)]
union I2cSmbusData {
    byte: u8,
    word: u16,
    block: [u8; I2C_SMBUS_BLOCK_MAX + 2],
}

#[repr(C)]
struct I2cSmbusIoctlData {
    read_write: u8,
    command: u8,
    size: u32,
    data: *mut I2cSmbusData,
}

/// ASUS Power Detector+ interface
///
/// # Safety
/// This struct ONLY performs read operations on I2C devices.
/// All methods are read-only and cannot damage hardware.
pub struct AsusPowerDetector {
    /// GPU PCI bus ID (e.g., "0000:01:00.0")
    #[allow(dead_code)]
    pci_id: String,
    /// Detected card model
    model: AsusRogModel,
    /// Preferred telemetry interface for the power monitoring chip
    source: Option<TelemetrySource>,
    /// I2C device address (0x2b for Astral)
    i2c_addr: u8,
}

impl AsusPowerDetector {
    /// Create a new Power Detector for the specified GPU
    pub fn new(pci_id: &str) -> NvResult<Self> {
        let model = Self::detect_model(pci_id)?;
        let source = Self::find_hwmon_source(pci_id)
            .or_else(|| Self::find_i2c_bus(pci_id).map(|bus| TelemetrySource::DirectSmbus { bus }));

        Ok(Self {
            pci_id: pci_id.to_string(),
            model,
            source,
            i2c_addr: IT8915_ADDRESS,
        })
    }

    /// Detect ASUS ROG model from PCI subsystem IDs
    fn detect_model(pci_id: &str) -> NvResult<AsusRogModel> {
        let vendor_path = format!("/sys/bus/pci/devices/{}/subsystem_vendor", pci_id);
        let device_path = format!("/sys/bus/pci/devices/{}/subsystem_device", pci_id);

        let vendor_str = fs::read_to_string(&vendor_path).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Cannot read subsystem vendor: {}", e))
        })?;
        let device_str = fs::read_to_string(&device_path).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Cannot read subsystem device: {}", e))
        })?;

        let vendor =
            u16::from_str_radix(vendor_str.trim().trim_start_matches("0x"), 16).unwrap_or(0);
        let device =
            u16::from_str_radix(device_str.trim().trim_start_matches("0x"), 16).unwrap_or(0);

        Ok(AsusRogModel::from_subsystem_id(vendor, device))
    }

    fn find_hwmon_source(pci_id: &str) -> Option<TelemetrySource> {
        let mut candidates = Vec::new();
        for entry in fs::read_dir("/sys/class/hwmon").ok()?.flatten() {
            let path = entry.path();
            let Ok(name) = fs::read_to_string(path.join("name")) else {
                continue;
            };
            if name.trim() != "astral12vhpwr" {
                continue;
            }
            let Ok(canonical) = fs::canonicalize(&path) else {
                continue;
            };
            candidates.push((path, canonical));
        }
        candidates.sort_by(|left, right| left.0.cmp(&right.0));

        let selected = candidates
            .iter()
            .find(|(_, canonical)| canonical.to_string_lossy().contains(pci_id))
            .or_else(|| (candidates.len() == 1).then(|| &candidates[0]))?;
        let bus = Self::bus_from_hwmon_path(&selected.1)?;
        Some(TelemetrySource::Hwmon {
            path: selected.0.clone(),
            bus,
        })
    }

    fn bus_from_hwmon_path(path: &Path) -> Option<u32> {
        path.ancestors().find_map(|ancestor| {
            let name = ancestor.file_name()?.to_str()?;
            let (bus, address) = name.split_once('-')?;
            (address == "002b").then(|| bus.parse().ok()).flatten()
        })
    }

    /// Locate adapter index 1 belonging to this GPU without probing unrelated buses.
    fn find_i2c_bus(pci_id: &str) -> Option<u32> {
        let pci_path = format!("/sys/bus/pci/devices/{}", pci_id);
        fs::read_dir(pci_path).ok()?.flatten().find_map(|entry| {
            let file_name = entry.file_name();
            let bus = file_name.to_str()?.strip_prefix("i2c-")?.parse().ok()?;
            let adapter_name = fs::read_to_string(entry.path().join("name")).ok()?;
            let index = adapter_name
                .trim()
                .strip_prefix("NVIDIA i2c adapter ")?
                .split_whitespace()
                .next()?
                .parse::<u32>()
                .ok()?;
            (index == 1).then_some(bus)
        })
    }

    /// Check if this card supports Power Detector+
    pub fn is_supported(&self) -> bool {
        self.model.supports_power_detector() && self.source.is_some()
    }

    pub fn support_status_message(&self) -> String {
        if !self.model.supports_power_detector() {
            return format!("{}: {}", self.model.name(), self.model.capability_note());
        }

        if self.source.is_none() {
            return format!(
                "{} detected, but neither an astral12vhpwr hwmon device nor NVIDIA I2C adapter 1 was found.",
                self.model.name()
            );
        }

        let source = self.source.as_ref().expect("source checked above");
        format!(
            "{}: Power Detector+ available via {}",
            self.model.name(),
            source.label()
        )
    }

    /// Get the detected card model
    pub fn model(&self) -> &AsusRogModel {
        &self.model
    }

    /// Read power rail status (READ-ONLY operation)
    ///
    /// # Safety
    /// This method only requests telemetry reads. The direct fallback sends the
    /// register selector required by the SMBus read, but never writes chip data.
    pub fn read_power_rails(&self) -> NvResult<PowerConnectorStatus> {
        if !self.model.supports_power_detector() {
            return Err(NvControlError::UnsupportedFeature(format!(
                "Power Detector+ not supported on {}",
                self.model.name()
            )));
        }

        let source = self.source.as_ref().ok_or_else(|| {
            NvControlError::UnsupportedFeature(
                "No astral12vhpwr hwmon device or NVIDIA I2C adapter 1 found".into(),
            )
        })?;
        let rails = match source {
            TelemetrySource::Hwmon { path, .. } => Self::read_hwmon_rails(path)?,
            TelemetrySource::DirectSmbus { bus } => {
                let frame = self.read_smbus_frame(*bus)?;
                Self::decode_frame(&frame)?
            }
        };
        let total_power_w = Self::total_power(&rails);
        let current_balance_percent = Self::current_balance(&rails);
        let health = Self::compute_health(&rails);
        let has_warnings = matches!(health, PowerHealth::Warning | PowerHealth::Critical);

        Ok(PowerConnectorStatus {
            model: self.model.name().to_string(),
            i2c_bus: source.bus(),
            source: source.label().to_string(),
            rails,
            total_power_w,
            current_balance_percent,
            has_warnings,
            health,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    fn read_hwmon_rails(path: &Path) -> NvResult<Vec<PowerRailReading>> {
        (0..6)
            .map(|pin| {
                let voltage_mv = Self::read_hwmon_value(&path.join(format!("in{pin}_input")))?;
                let current_ma =
                    Self::read_hwmon_value(&path.join(format!("curr{}_input", pin + 1)))?;
                Self::validated_rail(pin as u8, voltage_mv, current_ma)
            })
            .collect()
    }

    fn read_hwmon_value(path: &Path) -> NvResult<u32> {
        let value = fs::read_to_string(path).map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Cannot read {}: {e}", path.display()))
        })?;
        value.trim().parse().map_err(|e| {
            NvControlError::RuntimeError(format!("Invalid value in {}: {e}", path.display()))
        })
    }

    fn read_smbus_frame(&self, bus: u32) -> NvResult<[u8; IT8915_FRAME_LEN]> {
        let device = format!("/dev/i2c-{bus}");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&device)
            .map_err(|e| {
                let guidance = if e.kind() == std::io::ErrorKind::PermissionDenied {
                    " (grant this user access to the device's owning group, normally i2c, or install astral-hwmon)"
                } else {
                    ""
                };
                NvControlError::GpuQueryFailed(format!("Cannot open {device}: {e}{guidance}"))
            })?;
        let fd = file.as_raw_fd();

        // I2C_SLAVE selects an address; it does not transfer data to the device.
        let result = unsafe { libc::ioctl(fd, I2C_SLAVE, self.i2c_addr as libc::c_ulong) };
        if result < 0 {
            return Err(NvControlError::GpuQueryFailed(format!(
                "Cannot select address 0x{:02x} on {device}: {}",
                self.i2c_addr,
                std::io::Error::last_os_error()
            )));
        }

        let mut smbus_data = I2cSmbusData {
            block: [0; I2C_SMBUS_BLOCK_MAX + 2],
        };
        unsafe {
            smbus_data.block[0] = IT8915_FRAME_LEN as u8;
        }
        let mut ioctl_data = I2cSmbusIoctlData {
            read_write: I2C_SMBUS_READ,
            command: IT8915_REGISTER,
            size: I2C_SMBUS_I2C_BLOCK_DATA,
            data: &mut smbus_data,
        };
        let result = unsafe { libc::ioctl(fd, I2C_SMBUS, &mut ioctl_data) };
        if result < 0 {
            return Err(NvControlError::GpuQueryFailed(format!(
                "SMBus block read from {device} address 0x{:02x} register 0x{IT8915_REGISTER:02x} failed: {}",
                self.i2c_addr,
                std::io::Error::last_os_error()
            )));
        }

        let block = unsafe { smbus_data.block };
        if block[0] as usize != IT8915_FRAME_LEN {
            return Err(NvControlError::RuntimeError(format!(
                "IT8915 returned {} bytes; expected {IT8915_FRAME_LEN}",
                block[0]
            )));
        }
        let mut frame = [0; IT8915_FRAME_LEN];
        frame.copy_from_slice(&block[1..=IT8915_FRAME_LEN]);
        Ok(frame)
    }

    fn decode_frame(frame: &[u8; IT8915_FRAME_LEN]) -> NvResult<Vec<PowerRailReading>> {
        if frame.iter().all(|byte| *byte == 0) {
            return Err(NvControlError::RuntimeError(
                "IT8915 returned an all-zero frame; the required SMBus block transaction did not succeed"
                    .to_string(),
            ));
        }

        (0..6)
            .map(|pin| {
                let offset = (5 - pin) * 4;
                let voltage_mv = u16::from_be_bytes([frame[offset], frame[offset + 1]]) as u32;
                let current_ma = u16::from_be_bytes([frame[offset + 2], frame[offset + 3]]) as u32;
                Self::validated_rail(pin as u8, voltage_mv, current_ma)
            })
            .collect()
    }

    fn validated_rail(rail_id: u8, voltage_mv: u32, current_ma: u32) -> NvResult<PowerRailReading> {
        if !(6000..=13000).contains(&voltage_mv) || current_ma > 30000 {
            return Err(NvControlError::RuntimeError(format!(
                "Implausible IT8915 pin {} reading: {voltage_mv} mV, {current_ma} mA",
                rail_id + 1
            )));
        }
        Ok(PowerRailReading {
            rail_id,
            raw_value: current_ma as u16,
            voltage_mv: Some(voltage_mv),
            current_ma: Some(current_ma),
            power_w: Some(voltage_mv as f32 * current_ma as f32 / 1_000_000.0),
            warning: current_ma >= 9200,
        })
    }

    fn total_power(rails: &[PowerRailReading]) -> Option<f32> {
        let values: Vec<f32> = rails.iter().filter_map(|rail| rail.power_w).collect();
        (values.len() == rails.len() && !values.is_empty()).then(|| values.iter().sum())
    }

    fn current_balance(rails: &[PowerRailReading]) -> Option<f32> {
        let currents: Vec<u32> = rails.iter().filter_map(|rail| rail.current_ma).collect();
        let total: u32 = currents.iter().sum();
        if currents.len() != 6 || total < 20_000 {
            return None;
        }
        let min = *currents.iter().min()?;
        let max = *currents.iter().max()?;
        (max > 0).then_some(min as f32 / max as f32 * 100.0)
    }

    /// Compute overall health status from rail readings
    fn compute_health(rails: &[PowerRailReading]) -> PowerHealth {
        if rails.len() != 6
            || rails
                .iter()
                .any(|rail| rail.current_ma.is_none() || rail.voltage_mv.is_none())
        {
            return PowerHealth::Unknown;
        }
        let currents: Vec<u32> = rails.iter().filter_map(|rail| rail.current_ma).collect();
        let voltages: Vec<u32> = rails.iter().filter_map(|rail| rail.voltage_mv).collect();
        let total_current: u32 = currents.iter().sum();
        let max_current = *currents.iter().max().unwrap_or(&0);
        let min_current = *currents.iter().min().unwrap_or(&0);
        let min_voltage = *voltages.iter().min().unwrap_or(&0);
        let balance = if max_current > 0 {
            min_current as f32 / max_current as f32 * 100.0
        } else {
            100.0
        };

        if max_current >= 9500
            || min_voltage < 11_000
            || (total_current >= 10_000 && min_current < 500)
            || (total_current >= 20_000 && balance < 60.0)
        {
            PowerHealth::Critical
        } else if max_current >= 9200
            || min_voltage < 11_400
            || (total_current >= 20_000 && balance < 70.0)
        {
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
        output.push_str(&format!(
            "Connector Health: {}[{}]{}\n",
            health_color,
            status.health.label(),
            reset
        ));
        output.push_str(&format!(
            "Source: {} (I2C bus {} @ 0x{:02X})\n\n",
            status.source, status.i2c_bus, self.i2c_addr
        ));

        output.push_str("12V-2x6 Power Rails:\n");
        for rail in &status.rails {
            let warning_str = if rail.warning { " ⚠️ HIGH" } else { "" };
            let current_str = rail
                .current_ma
                .map(|c| format!("{:.2}A", c as f32 / 1000.0))
                .unwrap_or_else(|| "N/A".to_string());
            let voltage_str = rail
                .voltage_mv
                .map(|v| format!("{:.3}V", v as f32 / 1000.0))
                .unwrap_or_else(|| "N/A".to_string());

            output.push_str(&format!(
                "  Pin {}: {} × {}{}\n",
                rail.rail_id + 1,
                voltage_str,
                current_str,
                warning_str
            ));
        }

        if let Some(power) = status.total_power_w {
            output.push_str(&format!("\nMeasured Connector Power: {:.1}W\n", power));
        }
        if let Some(balance) = status.current_balance_percent {
            output.push_str(&format!("Pin Current Balance (min/max): {:.1}%\n", balance));
        }

        if status.has_warnings {
            output.push_str("\n⚠️  WARNING: One or more rails exceeding safe current!\n");
            output.push_str("    Check 12V-2x6 connector seating and cable quality.\n");
        }

        Ok(output)
    }

    /// Get status string with history statistics
    pub fn status_string_with_history(&self, history: &PowerHistory) -> NvResult<String> {
        let status = self.read_power_rails()?;

        let reset = "\x1b[0m";
        let health_color = status.health.color_code();

        let mut output = String::new();
        output.push_str(&format!("ASUS Power Detector+ - {}\n", status.model));
        output.push_str("═══════════════════════════════════════\n");

        // Health status prominently displayed
        output.push_str(&format!(
            "Connector Health: {}[{}]{}\n",
            health_color,
            status.health.label(),
            reset
        ));
        output.push_str(&format!(
            "Source: {} (I2C bus {} @ 0x{:02X})\n\n",
            status.source, status.i2c_bus, self.i2c_addr
        ));

        output.push_str("12V-2x6 Power Rails:\n");
        for rail in &status.rails {
            let warning_str = if rail.warning { " ⚠️ HIGH" } else { "" };
            let current_str = rail
                .current_ma
                .map(|c| format!("{:.2}A", c as f32 / 1000.0))
                .unwrap_or_else(|| "N/A".to_string());
            let voltage_str = rail
                .voltage_mv
                .map(|v| format!("{:.3}V", v as f32 / 1000.0))
                .unwrap_or_else(|| "N/A".to_string());

            output.push_str(&format!(
                "  Pin {}: {} × {}{}\n",
                rail.rail_id + 1,
                voltage_str,
                current_str,
                warning_str
            ));
        }

        if let Some(power) = status.total_power_w {
            output.push_str(&format!("\nCurrent Power: {:.1}W\n", power));
        }
        if let Some(balance) = status.current_balance_percent {
            output.push_str(&format!("Pin Current Balance (min/max): {:.1}%\n", balance));
        }

        // Add history statistics if available
        if !history.is_empty() {
            output.push_str(&format!("\n─── History ({} samples) ───\n", history.len()));

            if let Some(avg) = history.average_power() {
                output.push_str(&format!("  Average: {:.1}W\n", avg));
            }
            if let Some(peak) = history.peak_power() {
                output.push_str(&format!("  Peak:    {:.1}W\n", peak));
            }
            if let Some(min) = history.min_power() {
                output.push_str(&format!("  Min:     {:.1}W\n", min));
            }

            let trend = history.trend();
            output.push_str(&format!("  Trend:   {}\n", trend.label()));

            let warnings = history.warning_count();
            if warnings > 0 {
                output.push_str(&format!(
                    "  ⚠️  {} warning{} in history\n",
                    warnings,
                    if warnings == 1 { "" } else { "s" }
                ));
            }

            // Show per-rail averages
            let rail_avgs = history.rail_averages();
            if !rail_avgs.is_empty() {
                output.push_str("\n  Rail Averages:\n");
                for (i, avg) in rail_avgs.iter().enumerate() {
                    output.push_str(&format!("    Rail {}: {:.2}A\n", i, avg / 1000.0));
                }
            }
        }

        if status.has_warnings {
            output.push_str("\n⚠️  WARNING: One or more rails exceeding safe current!\n");
            output.push_str("    Check 12V-2x6 connector seating and cable quality.\n");
        }

        Ok(output)
    }

    /// Read power rails and record to history buffer
    pub fn read_and_record(&self, history: &mut PowerHistory) -> NvResult<PowerConnectorStatus> {
        let status = self.read_power_rails()?;
        history.record(&status);
        Ok(status)
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
        for id in [
            0x89e3, 0x89ea, 0x8a61, 0x89ec, 0x89de, 0x8a2e, 0x8a2b, 0x8a45,
        ] {
            assert!(AsusRogModel::from_subsystem_id(0x1043, id).supports_power_detector());
        }
    }

    #[test]
    fn decode_it8915_frame_uses_big_endian_reverse_pin_order() {
        let expected = [
            (12_180u16, 540u16),
            (12_160, 560),
            (12_140, 580),
            (12_120, 600),
            (12_100, 620),
            (12_080, 640),
        ];
        let mut frame = [0u8; IT8915_FRAME_LEN];
        for (pin, (voltage, current)) in expected.iter().enumerate() {
            let offset = (5 - pin) * 4;
            frame[offset..offset + 2].copy_from_slice(&voltage.to_be_bytes());
            frame[offset + 2..offset + 4].copy_from_slice(&current.to_be_bytes());
        }

        let rails = AsusPowerDetector::decode_frame(&frame).unwrap();
        assert_eq!(rails.len(), 6);
        for (rail, (voltage, current)) in rails.iter().zip(expected) {
            assert_eq!(rail.voltage_mv, Some(voltage as u32));
            assert_eq!(rail.current_ma, Some(current as u32));
        }
        assert!((AsusPowerDetector::total_power(&rails).unwrap() - 42.93).abs() < 0.1);
    }

    #[test]
    fn decode_it8915_frame_rejects_zero_and_implausible_data() {
        assert!(AsusPowerDetector::decode_frame(&[0; IT8915_FRAME_LEN]).is_err());

        let mut frame = [0u8; IT8915_FRAME_LEN];
        for pin in 0..6 {
            let offset = (5 - pin) * 4;
            frame[offset..offset + 2].copy_from_slice(&5000u16.to_be_bytes());
            frame[offset + 2..offset + 4].copy_from_slice(&500u16.to_be_bytes());
        }
        assert!(AsusPowerDetector::decode_frame(&frame).is_err());
    }

    #[test]
    fn hwmon_reader_pairs_zero_indexed_voltage_with_one_indexed_current() {
        let scratch = Path::new(env!("CARGO_MANIFEST_DIR")).join(".scratch");
        fs::create_dir_all(&scratch).unwrap();
        let directory = tempfile::tempdir_in(scratch).unwrap();

        for pin in 0..6 {
            fs::write(
                directory.path().join(format!("in{pin}_input")),
                format!("{}\n", 12_000 + pin),
            )
            .unwrap();
            fs::write(
                directory.path().join(format!("curr{}_input", pin + 1)),
                format!("{}\n", 500 + pin * 20),
            )
            .unwrap();
        }

        let rails = AsusPowerDetector::read_hwmon_rails(directory.path()).unwrap();
        assert_eq!(rails[0].voltage_mv, Some(12_000));
        assert_eq!(rails[0].current_ma, Some(500));
        assert_eq!(rails[5].voltage_mv, Some(12_005));
        assert_eq!(rails[5].current_ma, Some(600));
    }

    fn rails_with(values: &[(u32, u32); 6]) -> Vec<PowerRailReading> {
        values
            .iter()
            .enumerate()
            .map(|(pin, (voltage, current))| {
                AsusPowerDetector::validated_rail(pin as u8, *voltage, *current).unwrap()
            })
            .collect()
    }

    #[test]
    fn connector_health_is_load_gated() {
        let idle = rails_with(&[(12_100, 100); 6]);
        assert_eq!(AsusPowerDetector::compute_health(&idle), PowerHealth::Good);
        assert_eq!(AsusPowerDetector::current_balance(&idle), None);

        let unbalanced = rails_with(&[
            (12_000, 100),
            (12_000, 4_500),
            (12_000, 4_500),
            (12_000, 4_500),
            (12_000, 4_500),
            (12_000, 4_500),
        ]);
        assert_eq!(
            AsusPowerDetector::compute_health(&unbalanced),
            PowerHealth::Critical
        );
        assert!(AsusPowerDetector::current_balance(&unbalanced).unwrap() < 60.0);

        let low_voltage = rails_with(&[(10_900, 1_000); 6]);
        assert_eq!(
            AsusPowerDetector::compute_health(&low_voltage),
            PowerHealth::Critical
        );
    }

    #[test]
    fn test_power_history() {
        let mut history = PowerHistory::with_interval(Duration::from_millis(0)); // No rate limiting for test

        // Create mock status readings
        for i in 0..10 {
            let status = PowerConnectorStatus {
                model: "Test".to_string(),
                i2c_bus: 0,
                rails: vec![PowerRailReading {
                    rail_id: 0,
                    raw_value: 0x0200,
                    voltage_mv: Some(12_000),
                    current_ma: Some(1000 + i * 100),
                    power_w: Some(12.0 + i as f32 * 1.2),
                    warning: false,
                }],
                total_power_w: Some(12.0 + i as f32),
                source: "test".to_string(),
                current_balance_percent: None,
                has_warnings: false,
                health: PowerHealth::Good,
                timestamp: i as u64,
            };
            history.record(&status);
        }

        assert_eq!(history.len(), 10);
        assert!(!history.is_empty());

        // Check statistics
        let avg = history.average_power().unwrap();
        assert!(avg > 16.0 && avg < 17.5); // Average of 12..22

        let peak = history.peak_power().unwrap();
        assert!((peak - 21.0).abs() < 0.1); // Last value: 12 + 9 = 21

        let min = history.min_power().unwrap();
        assert!((min - 12.0).abs() < 0.1); // First value

        // Trend should be rising
        assert_eq!(history.trend(), PowerTrend::Rising);

        // Rail averages
        let rail_avgs = history.rail_averages();
        assert_eq!(rail_avgs.len(), 1);
        assert!(rail_avgs[0] > 1400.0 && rail_avgs[0] < 1500.0); // Average of 1000..1900

        // No warnings
        assert!(!history.had_warnings());
        assert_eq!(history.warning_count(), 0);
    }

    #[test]
    fn test_power_history_buffer_limit() {
        let mut history = PowerHistory::with_interval(Duration::from_millis(0));

        // Add more than POWER_HISTORY_SIZE samples
        for i in 0..70 {
            let status = PowerConnectorStatus {
                model: "Test".to_string(),
                i2c_bus: 0,
                source: "test".to_string(),
                rails: vec![],
                total_power_w: Some(i as f32),
                current_balance_percent: None,
                has_warnings: false,
                health: PowerHealth::Good,
                timestamp: i,
            };
            history.record(&status);
        }

        // Should only keep POWER_HISTORY_SIZE (60) samples
        assert_eq!(history.len(), POWER_HISTORY_SIZE);

        // First sample should be 10 (oldest after dropping 0-9)
        let first = history.samples().front().unwrap();
        assert!((first.total_power_w - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_power_trend_detection() {
        let mut history = PowerHistory::with_interval(Duration::from_millis(0));

        // Add stable samples
        for _ in 0..15 {
            let status = PowerConnectorStatus {
                model: "Test".to_string(),
                i2c_bus: 0,
                source: "test".to_string(),
                rails: vec![],
                total_power_w: Some(100.0),
                current_balance_percent: None,
                has_warnings: false,
                health: PowerHealth::Good,
                timestamp: 0,
            };
            history.record(&status);
        }

        assert_eq!(history.trend(), PowerTrend::Stable);

        // Add falling samples
        history.clear();
        for i in 0..15 {
            let status = PowerConnectorStatus {
                model: "Test".to_string(),
                i2c_bus: 0,
                source: "test".to_string(),
                rails: vec![],
                total_power_w: Some(200.0 - i as f32 * 10.0),
                current_balance_percent: None,
                has_warnings: false,
                health: PowerHealth::Good,
                timestamp: i as u64,
            };
            history.record(&status);
        }

        assert_eq!(history.trend(), PowerTrend::Falling);
    }
}
