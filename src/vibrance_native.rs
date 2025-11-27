use crate::nvkms_bindings::*;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::os::unix::io::AsRawFd;

/// Native Digital Vibrance Implementation using NVKMS ioctls
/// Based on nvibrant's approach - iterate connectors, get dpyId from static data
/// Direct interface with NVIDIA Open Drivers (515+)

const NVIDIA_MODESET_DEVICE: &str = "/dev/nvidia-modeset";

// Vibrance range: -1024 (grayscale) to 1023 (200% saturation), 0 = default
const VIBRANCE_MIN: i64 = -1024;
const VIBRANCE_MAX: i64 = 1023;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorInfo {
    pub connector_index: u32,
    pub connector_type: String,
    pub dpy_id: u32,
    pub connected: bool,
    pub current_vibrance: i64,
}

#[derive(Debug)]
pub struct NativeVibranceController {
    pub driver_version: String,
    pub connectors: Vec<ConnectorInfo>,
    device_handle: u32,
    disp_handle: u32,
    modeset_fd: i32,
}

impl NativeVibranceController {
    /// Initialize the native vibrance controller following nvibrant's approach
    pub fn new() -> NvResult<Self> {
        let driver_version = Self::get_driver_version()?;

        // Open nvidia-modeset device
        let modeset_file = Self::open_modeset_device()?;
        let modeset_fd = modeset_file.as_raw_fd();

        // Leak the file handle to keep it open
        std::mem::forget(modeset_file);

        // Allocate NVKMS device (like nvibrant does)
        let (device_handle, disp_handle, connectors) =
            Self::allocate_device_and_enumerate(modeset_fd, &driver_version)?;

        Ok(NativeVibranceController {
            driver_version,
            connectors,
            device_handle,
            disp_handle,
            modeset_fd,
        })
    }

    /// Get NVIDIA driver version from /sys/module/nvidia/version
    fn get_driver_version() -> NvResult<String> {
        // First try sysfs (most reliable)
        if let Ok(version) = std::fs::read_to_string("/sys/module/nvidia/version") {
            return Ok(version.trim().to_string());
        }

        // Fallback to nvidia-smi
        use std::process::Command;
        let output = Command::new("nvidia-smi")
            .args(["--query-gpu=driver_version", "--format=csv,noheader,nounits"])
            .output()
            .map_err(|e| {
                NvControlError::VibranceControlFailed(format!("Failed to get driver version: {}", e))
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(NvControlError::VibranceControlFailed(
                "Could not detect NVIDIA driver version".to_string(),
            ))
        }
    }

    /// Open /dev/nvidia-modeset device
    fn open_modeset_device() -> NvResult<File> {
        use std::fs::OpenOptions;

        if !std::path::Path::new(NVIDIA_MODESET_DEVICE).exists() {
            return Err(NvControlError::VibranceControlFailed(
                "NVIDIA modeset device not found. Ensure nvidia_drm.modeset=1".to_string(),
            ));
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(NVIDIA_MODESET_DEVICE)
            .map_err(|e| {
                NvControlError::VibranceControlFailed(format!(
                    "Failed to open nvidia-modeset device: {}",
                    e
                ))
            })?;

        Ok(file)
    }

    /// Allocate NVKMS device and enumerate connectors (nvibrant approach)
    fn allocate_device_and_enumerate(
        fd: i32,
        driver_version: &str,
    ) -> NvResult<(u32, u32, Vec<ConnectorInfo>)> {
        // Prepare version string (must match driver version exactly)
        let mut version_string = [0u8; NVKMS_NVIDIA_DRIVER_VERSION_STRING_LENGTH];
        let version_bytes = driver_version.as_bytes();
        let copy_len = version_bytes.len().min(NVKMS_NVIDIA_DRIVER_VERSION_STRING_LENGTH - 1);
        version_string[..copy_len].copy_from_slice(&version_bytes[..copy_len]);

        // Prepare registry keys (empty, like nvibrant)
        let registry_keys = [RegistryKey {
            name: [0u8; NVKMS_MAX_DEVICE_REGISTRY_KEYNAME_LEN],
            value: 0,
        }; NVKMS_MAX_DEVICE_REGISTRY_KEYS];

        // GPU index (default 0, could be configurable)
        let gpu_index: u32 = std::env::var("NVIDIA_GPU")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        // Allocate NVKMS device
        let mut alloc_params = NvKmsAllocDeviceParams {
            request: NvKmsAllocDeviceRequest {
                version_string,
                device_id: NvKmsDeviceId {
                    rm_device_id: gpu_index,
                    mig_device: MIGDeviceId { value: 0 },
                },
                sli_mosaic: 0,                                // NV_FALSE
                try_infer_sli_mosaic_from_existing_device: 0, // NV_FALSE
                no3d: 1,                                      // NV_TRUE (like nvibrant)
                enable_console_hotplug_handling: 0,           // NV_FALSE
                registry_keys,
            },
            reply: unsafe { std::mem::zeroed() },
        };

        unsafe {
            nvkms_ioctl(fd, NvKmsIoctlCommand::AllocDevice, &mut alloc_params).map_err(|e| {
                NvControlError::VibranceControlFailed(format!("AllocDevice ioctl failed: {}", e))
            })?;
        }

        // Check status
        if alloc_params.reply.status != NvKmsAllocDeviceStatus::Success {
            let status_msg = match alloc_params.reply.status {
                NvKmsAllocDeviceStatus::VersionMismatch => "Driver version mismatch - try rebooting",
                NvKmsAllocDeviceStatus::BadDeviceId => "Bad device ID",
                NvKmsAllocDeviceStatus::AlreadyAllocated => "Device already allocated",
                _ => "Unknown error",
            };
            return Err(NvControlError::VibranceControlFailed(format!(
                "AllocDevice failed: {}",
                status_msg
            )));
        }

        let device_handle = alloc_params.reply.device_handle;
        let disp_handle = alloc_params.reply.disp_handles[0]; // First display

        // Query display info to get connectors
        let mut query_disp_params = NvKmsQueryDispParams {
            request: NvKmsQueryDispRequest {
                device_handle,
                disp_handle,
            },
            reply: unsafe { std::mem::zeroed() },
        };

        unsafe {
            nvkms_ioctl(fd, NvKmsIoctlCommand::QueryDisp, &mut query_disp_params).map_err(|e| {
                NvControlError::VibranceControlFailed(format!("QueryDisp failed: {}", e))
            })?;
        }

        let num_connectors = query_disp_params.reply.num_connectors;
        let mut connectors = Vec::new();

        // Iterate through all connectors (like nvibrant does)
        for connector_idx in 0..num_connectors as usize {
            let connector_handle = query_disp_params.reply.connector_handles[connector_idx];

            // Query connector static data to get dpyId and type
            let mut static_params = NvKmsQueryConnectorStaticDataParams {
                request: NvKmsQueryConnectorStaticDataRequest {
                    device_handle,
                    disp_handle,
                    connector_handle,
                },
                reply: unsafe { std::mem::zeroed() },
            };

            if unsafe { nvkms_ioctl(fd, NvKmsIoctlCommand::QueryConnectorStaticData, &mut static_params) }.is_err() {
                continue;
            }

            let dpy_id = static_params.reply.dpy_id;
            let connector_type = match static_params.reply.connector_type {
                NvKmsConnectorType::Dp => "DP",
                NvKmsConnectorType::Hdmi => "HDMI",
                NvKmsConnectorType::DviI => "DVI-I",
                NvKmsConnectorType::DviD => "DVI-D",
                NvKmsConnectorType::Vga => "VGA",
                NvKmsConnectorType::Usbc => "USB-C",
                NvKmsConnectorType::Lvds => "LVDS",
                NvKmsConnectorType::Dsi => "DSI",
                NvKmsConnectorType::Adc => "ADC",
                _ => "Unknown",
            };

            // Query dpy dynamic data to check if connected
            // Use zeroed() for the whole struct since it has padding fields
            let mut dynamic_params: NvKmsQueryDpyDynamicDataParams = unsafe { std::mem::zeroed() };
            dynamic_params.request.device_handle = device_handle;
            dynamic_params.request.disp_handle = disp_handle;
            dynamic_params.request.dpy_id = dpy_id;

            let connected = if unsafe { nvkms_ioctl(fd, NvKmsIoctlCommand::QueryDpyDynamicData, &mut dynamic_params) }.is_ok() {
                dynamic_params.reply.connected != 0
            } else {
                false
            };

            connectors.push(ConnectorInfo {
                connector_index: connector_idx as u32,
                connector_type: connector_type.to_string(),
                dpy_id,
                connected,
                current_vibrance: 0,
            });
        }

        Ok((device_handle, disp_handle, connectors))
    }

    /// Set vibrance for a specific connector by index
    pub fn set_vibrance(&mut self, connector_index: usize, vibrance_value: i64) -> NvResult<()> {
        let connector = self.connectors.get(connector_index).ok_or_else(|| {
            NvControlError::VibranceControlFailed(format!(
                "Connector {} not found",
                connector_index
            ))
        })?;

        if !connector.connected {
            return Err(NvControlError::VibranceControlFailed(
                "Cannot set vibrance on disconnected display".to_string(),
            ));
        }

        let clamped_value = vibrance_value.clamp(VIBRANCE_MIN, VIBRANCE_MAX);

        let mut params = NvKmsSetDpyAttributeParams {
            request: NvKmsSetDpyAttributeRequest {
                device_handle: self.device_handle,
                disp_handle: self.disp_handle,
                dpy_id: connector.dpy_id,
                attribute: NvKmsDpyAttribute::DigitalVibrance,
                value: clamped_value,
            },
            reply: NvKmsSetDpyAttributeReply { padding: 0 },
        };

        unsafe {
            nvkms_ioctl(self.modeset_fd, NvKmsIoctlCommand::SetDpyAttribute, &mut params).map_err(
                |e| NvControlError::VibranceControlFailed(format!("Failed to set vibrance: {}", e)),
            )?;
        }

        // Update stored value
        if let Some(conn) = self.connectors.get_mut(connector_index) {
            conn.current_vibrance = clamped_value;
        }

        Ok(())
    }

    /// Set vibrance for all connected displays
    pub fn set_vibrance_all(&mut self, vibrance_percentage: u32) -> NvResult<()> {
        let vibrance_value = percentage_to_vibrance(vibrance_percentage);

        let mut success_count = 0;
        let mut last_error = None;

        for i in 0..self.connectors.len() {
            if self.connectors[i].connected {
                match self.set_vibrance(i, vibrance_value) {
                    Ok(()) => success_count += 1,
                    Err(e) => last_error = Some(e),
                }
            }
        }

        if success_count > 0 {
            println!("✅ Set {} displays to {}% vibrance", success_count, vibrance_percentage);
            Ok(())
        } else if let Some(e) = last_error {
            Err(e)
        } else {
            Err(NvControlError::VibranceControlFailed(
                "No connected displays found".to_string(),
            ))
        }
    }

    /// Convert percentage (0-200%) to vibrance range (-1024 to 1023)
    pub fn percentage_to_vibrance(&self, percentage: u32) -> i64 {
        percentage_to_vibrance(percentage)
    }

    /// Convert vibrance range to percentage
    pub fn vibrance_to_percentage(&self, vibrance: i64) -> u32 {
        vibrance_to_percentage(vibrance)
    }

    /// List all connectors
    pub fn list_displays(&self) -> Vec<(u32, u32, String, bool)> {
        self.connectors
            .iter()
            .map(|c| (0, c.connector_index, format!("{}: {}", c.connector_index, c.connector_type), c.connected))
            .collect()
    }

    /// Get vibrance status
    pub fn get_vibrance_status(&self) -> HashMap<String, serde_json::Value> {
        let mut status = HashMap::new();
        status.insert("driver_version".to_string(), serde_json::Value::String(self.driver_version.clone()));
        status.insert("connectors".to_string(), serde_json::to_value(&self.connectors).unwrap_or(serde_json::Value::Null));
        status
    }

    /// Reset all to default (100%)
    pub fn reset_all_vibrance(&mut self) -> NvResult<()> {
        self.set_vibrance_all(100)
    }
}

impl Drop for NativeVibranceController {
    fn drop(&mut self) {
        // Free the NVKMS device handle
        let mut free_params = NvKmsFreeDeviceParams {
            request: NvKmsFreeDeviceRequest {
                device_handle: self.device_handle,
            },
            reply: NvKmsFreeDeviceReply { padding: 0 },
        };

        unsafe {
            let _ = nvkms_ioctl(self.modeset_fd, NvKmsIoctlCommand::FreeDevice, &mut free_params);
        }
    }
}

// ===== Module-level functions =====

/// Convert percentage (0-200%) to vibrance range (-1024 to 1023)
pub fn percentage_to_vibrance(percentage: u32) -> i64 {
    let percentage = percentage.min(200);

    if percentage <= 100 {
        // 0-100% maps to -1024 to 0
        let ratio = percentage as f64 / 100.0;
        ((ratio - 1.0) * 1024.0) as i64
    } else {
        // 100-200% maps to 0 to 1023
        let ratio = (percentage - 100) as f64 / 100.0;
        (ratio * 1023.0) as i64
    }
}

/// Convert vibrance range (-1024 to 1023) to percentage (0-200%)
pub fn vibrance_to_percentage(vibrance: i64) -> u32 {
    if vibrance <= 0 {
        (((vibrance + 1024) as f64 / 1024.0) * 100.0) as u32
    } else {
        (100.0 + (vibrance as f64 / 1023.0 * 100.0)) as u32
    }
}

// ===== Global controller instance =====
use std::sync::Mutex;

static VIBRANCE_CONTROLLER: Mutex<Option<NativeVibranceController>> = Mutex::new(None);

/// Get or initialize the vibrance controller
pub fn get_vibrance_controller() -> NvResult<std::sync::MutexGuard<'static, Option<NativeVibranceController>>> {
    let mut guard = VIBRANCE_CONTROLLER.lock().map_err(|_| {
        NvControlError::VibranceControlFailed("Failed to acquire vibrance lock".to_string())
    })?;

    if guard.is_none() {
        *guard = Some(NativeVibranceController::new()?);
    }

    Ok(guard)
}

// ===== CLI interface functions =====

pub fn set_vibrance_all_native(percentage: u32) -> NvResult<()> {
    let mut guard = get_vibrance_controller()?;
    let controller = guard.as_mut().ok_or_else(|| {
        NvControlError::VibranceControlFailed("Controller not initialized".to_string())
    })?;
    controller.set_vibrance_all(percentage)
}

pub fn set_display_vibrance_native(device_id: u32, display_id: u32, percentage: u32) -> NvResult<()> {
    let _ = device_id; // Ignored for now, single GPU support
    let mut guard = get_vibrance_controller()?;
    let controller = guard.as_mut().ok_or_else(|| {
        NvControlError::VibranceControlFailed("Controller not initialized".to_string())
    })?;
    let vibrance_value = percentage_to_vibrance(percentage);
    controller.set_vibrance(display_id as usize, vibrance_value)
}

pub fn get_vibrance_status_native() -> NvResult<HashMap<String, serde_json::Value>> {
    let guard = get_vibrance_controller()?;
    let controller = guard.as_ref().ok_or_else(|| {
        NvControlError::VibranceControlFailed("Controller not initialized".to_string())
    })?;
    Ok(controller.get_vibrance_status())
}

pub fn list_displays_native() -> NvResult<Vec<(u32, u32, String, bool)>> {
    let guard = get_vibrance_controller()?;
    let controller = guard.as_ref().ok_or_else(|| {
        NvControlError::VibranceControlFailed("Controller not initialized".to_string())
    })?;
    Ok(controller.list_displays())
}

pub fn reset_vibrance_native() -> NvResult<()> {
    let mut guard = get_vibrance_controller()?;
    let controller = guard.as_mut().ok_or_else(|| {
        NvControlError::VibranceControlFailed("Controller not initialized".to_string())
    })?;
    controller.reset_all_vibrance()
}
