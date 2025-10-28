use crate::nvkms_bindings::*;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::os::unix::io::AsRawFd;

/// Pure Rust Digital Vibrance Implementation using NVKMS ioctls
/// Direct interface with NVIDIA Open Drivers (580+)
/// No external dependencies - built into nvctl

const NVIDIA_MODESET_DEVICE: &str = "/dev/nvidia-modeset";

// Vibrance range: -1024 (grayscale) to 1023 (200% saturation), 0 = default
const VIBRANCE_MIN: i64 = -1024;
const VIBRANCE_MAX: i64 = 1023;
const VIBRANCE_DEFAULT: i64 = 0;

#[derive(Debug)]
pub struct NativeVibranceController {
    pub devices: Vec<NvidiaDevice>,
    pub driver_version: String,
    pub open_driver: bool,
    #[allow(dead_code)]
    modeset_file: Option<File>, // Keep file handle alive
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvidiaDevice {
    pub device_handle: u32,
    pub device_id: u32,
    pub name: String,
    pub displays: Vec<NvidiaDisplay>,
    pub pci_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvidiaDisplay {
    pub disp_handle: u32,
    pub dpy_id: u32,
    pub connector_type: String,
    pub connected: bool,
    pub resolution: (u32, u32),
    pub refresh_rate: Option<f32>,
    pub current_vibrance: i64,
    pub vibrance_supported: bool,
    pub vibrance_range: (i64, i64),
}

impl NativeVibranceController {
    /// Initialize the native vibrance controller
    pub fn new() -> NvResult<Self> {
        // Check if we're running with NVIDIA open drivers
        let driver_version = Self::get_driver_version()?;
        let open_driver = Self::is_open_driver(&driver_version)?;

        if !open_driver {
            return Err(NvControlError::UnsupportedFeature(
                "Pure Rust vibrance requires NVIDIA open drivers (580+)".to_string(),
            ));
        }

        // Open nvidia-modeset device
        let modeset_file = Self::open_modeset_device()?;
        let modeset_fd = modeset_file.as_raw_fd();

        // Detect NVIDIA devices and displays using NVKMS ioctls
        let devices = Self::detect_nvidia_devices_via_nvkms(modeset_fd)?;

        Ok(NativeVibranceController {
            devices,
            driver_version,
            open_driver,
            modeset_file: Some(modeset_file),
        })
    }

    /// Check if we're using NVIDIA open drivers
    fn is_open_driver(version: &str) -> NvResult<bool> {
        let version_num: u32 = version
            .split('.')
            .next()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);

        Ok(version_num >= 580)
    }

    /// Get NVIDIA driver version
    fn get_driver_version() -> NvResult<String> {
        use std::process::Command;

        let output = Command::new("nvidia-smi")
            .args(&["--query-gpu=driver_version", "--format=csv,noheader,nounits"])
            .output()
            .map_err(|e| {
                NvControlError::VibranceControlFailed(format!("Failed to get driver version: {}", e))
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(NvControlError::VibranceControlFailed(
                "Could not detect NVIDIA driver".to_string(),
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
                    "Failed to open nvidia-modeset device: {}. Try running with sudo or add user to nvidia group",
                    e
                ))
            })?;

        Ok(file)
    }

    /// Detect NVIDIA devices and displays using NVKMS ioctls
    fn detect_nvidia_devices_via_nvkms(fd: i32) -> NvResult<Vec<NvidiaDevice>> {
        let mut devices = Vec::new();

        // Get driver version for version string
        let driver_version = Self::get_driver_version()?;

        // Get GPU information from nvidia-smi first
        let gpu_info = Self::get_gpu_info()?;

        for (device_id, gpu_name, pci_id) in gpu_info {
            // Prepare version string
            let mut version_string = [0u8; 64];
            let version_bytes = driver_version.as_bytes();
            let copy_len = version_bytes.len().min(63);
            version_string[..copy_len].copy_from_slice(&version_bytes[..copy_len]);

            // Prepare registry keys (empty)
            let registry_keys = [RegistryKey {
                name: [0u8; 64],
                value: 0,
            }; 16];

            // Allocate NVKMS device
            let mut alloc_params = NvKmsAllocDeviceParams {
                request: NvKmsAllocDeviceRequest {
                    version_string,
                    device_id: NvKmsDeviceId {
                        rm_device_id: device_id,
                        mig_device: MIGDeviceId { value: 0 }, // No MIG
                    },
                    sli_mosaic: 0,                                 // NV_FALSE
                    try_infer_sli_mosaic_from_existing_device: 0,  // NV_FALSE
                    no3d: 1,                                        // NV_TRUE (like nvibrant)
                    enable_console_hotplug_handling: 0,             // NV_FALSE
                    registry_keys,
                },
                reply: unsafe { std::mem::zeroed() },
            };

            unsafe {
                match nvkms_ioctl(fd, NvKmsIoctlCommand::AllocDevice, &mut alloc_params) {
                    Ok(_) => {
                        // Check status
                        if alloc_params.reply.status != NvKmsAllocDeviceStatus::Success {
                            eprintln!(
                                "NVKMS device allocation failed with status: {:?}",
                                alloc_params.reply.status
                            );
                            continue;
                        }

                        let device_handle = alloc_params.reply.device_handle;
                        let num_disps = alloc_params.reply.num_disps as usize;

                        // Query displays for each disp
                        let mut displays = Vec::new();
                        for disp_idx in 0..num_disps {
                            let disp_handle = alloc_params.reply.disp_handles[disp_idx];
                            let disp_displays =
                                Self::query_displays_for_disp(fd, device_handle, disp_handle)?;
                            displays.extend(disp_displays);
                        }

                        devices.push(NvidiaDevice {
                            device_handle,
                            device_id,
                            name: gpu_name.clone(),
                            displays,
                            pci_id: pci_id.clone(),
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to allocate NVKMS device {}: {}", device_id, e);
                        continue;
                    }
                }
            }
        }

        Ok(devices)
    }

    /// Query displays for a specific disp using NVKMS ioctls
    fn query_displays_for_disp(
        fd: i32,
        device_handle: u32,
        disp_handle: u32,
    ) -> NvResult<Vec<NvidiaDisplay>> {
        let mut query_params = NvKmsQueryDispParams {
            request: NvKmsQueryDispRequest {
                device_handle,
                disp_handle,
            },
            reply: unsafe { std::mem::zeroed() },
        };

        unsafe {
            nvkms_ioctl(fd, NvKmsIoctlCommand::QueryDisp, &mut query_params).map_err(|e| {
                NvControlError::VibranceControlFailed(format!("Failed to query displays: {}", e))
            })?;
        }

        let mut displays = Vec::new();

        // Parse valid displays from the reply
        for dpy_id in query_params.reply.valid_dpys.id.iter() {
            if *dpy_id == 0 {
                break; // End of valid displays
            }

            // Get vibrance range for this display
            let vibrance_range = Self::get_vibrance_range(fd, device_handle, disp_handle, *dpy_id)
                .unwrap_or((VIBRANCE_MIN, VIBRANCE_MAX));

            // Get current vibrance value
            let current_vibrance =
                Self::get_vibrance_via_ioctl(fd, device_handle, disp_handle, *dpy_id)
                    .unwrap_or(VIBRANCE_DEFAULT);

            // Query display info from system
            let display_info = Self::query_display_info(*dpy_id);

            displays.push(NvidiaDisplay {
                disp_handle,
                dpy_id: *dpy_id,
                connector_type: display_info.connector_type,
                connected: true,
                resolution: display_info.resolution,
                refresh_rate: display_info.refresh_rate,
                current_vibrance,
                vibrance_supported: true,
                vibrance_range,
            });
        }

        Ok(displays)
    }

    /// Get vibrance range for a display
    fn get_vibrance_range(
        fd: i32,
        device_handle: u32,
        disp_handle: u32,
        dpy_id: u32,
    ) -> NvResult<(i64, i64)> {
        let mut params = NvKmsGetDpyAttributeValidValuesParams {
            request: NvKmsGetDpyAttributeValidValuesRequest {
                device_handle,
                disp_handle,
                dpy_id,
                attribute: NvKmsDpyAttribute::DigitalVibrance,
            },
            reply: unsafe { std::mem::zeroed() },
        };

        unsafe {
            nvkms_ioctl(fd, NvKmsIoctlCommand::GetDpyAttributeValidValues, &mut params).map_err(
                |e| {
                    NvControlError::VibranceControlFailed(format!(
                        "Failed to get vibrance range: {}",
                        e
                    ))
                },
            )?;

            if params.reply.attr_type == NvKmsAttributeType::Range {
                Ok((params.reply.u.range.min, params.reply.u.range.max))
            } else {
                Ok((VIBRANCE_MIN, VIBRANCE_MAX))
            }
        }
    }

    /// Get GPU information from nvidia-smi
    fn get_gpu_info() -> NvResult<Vec<(u32, String, String)>> {
        use std::process::Command;

        let output = Command::new("nvidia-smi")
            .args(&[
                "--query-gpu=index,name,pci.bus_id",
                "--format=csv,noheader,nounits",
            ])
            .output()
            .map_err(|e| NvControlError::CommandFailed(format!("nvidia-smi failed: {}", e)))?;

        let mut gpu_info = Vec::new();
        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split(", ").collect();
            if parts.len() >= 3 {
                let device_id: u32 = parts[0].parse().unwrap_or(0);
                let name = parts[1].to_string();
                let pci_id = parts[2].to_string();
                gpu_info.push((device_id, name, pci_id));
            }
        }

        Ok(gpu_info)
    }

    /// Set vibrance for all connected displays
    pub fn set_vibrance_all(&mut self, vibrance_percentage: u32) -> NvResult<()> {
        let vibrance_value = self.percentage_to_vibrance(vibrance_percentage);
        let fd = self
            .modeset_file
            .as_ref()
            .ok_or_else(|| NvControlError::VibranceControlFailed("Device not initialized".to_string()))?
            .as_raw_fd();

        for device in &mut self.devices {
            for display in &mut device.displays {
                if display.connected && display.vibrance_supported {
                    Self::set_vibrance_via_ioctl(
                        fd,
                        device.device_handle,
                        display.disp_handle,
                        display.dpy_id,
                        vibrance_value,
                    )?;
                    display.current_vibrance = vibrance_value;
                }
            }
        }

        println!("✅ Set all displays to {}% vibrance", vibrance_percentage);
        Ok(())
    }

    /// Set vibrance for a specific display
    pub fn set_display_vibrance(
        &mut self,
        device_id: u32,
        display_id: u32,
        vibrance_percentage: u32,
    ) -> NvResult<()> {
        let vibrance_value = self.percentage_to_vibrance(vibrance_percentage);
        let fd = self
            .modeset_file
            .as_ref()
            .ok_or_else(|| NvControlError::VibranceControlFailed("Device not initialized".to_string()))?
            .as_raw_fd();

        // Find the device and display
        for device in &mut self.devices {
            if device.device_id == device_id {
                for display in &mut device.displays {
                    if display.dpy_id == display_id {
                        Self::set_vibrance_via_ioctl(
                            fd,
                            device.device_handle,
                            display.disp_handle,
                            display.dpy_id,
                            vibrance_value,
                        )?;
                        display.current_vibrance = vibrance_value;
                        println!(
                            "✅ Set device {} display {} to {}% vibrance",
                            device_id, display_id, vibrance_percentage
                        );
                        return Ok(());
                    }
                }
            }
        }

        Err(NvControlError::VibranceControlFailed(format!(
            "Display {} not found on device {}",
            display_id, device_id
        )))
    }

    /// Set vibrance using NVKMS ioctl
    fn set_vibrance_via_ioctl(
        fd: i32,
        device_handle: u32,
        disp_handle: u32,
        dpy_id: u32,
        vibrance_value: i64,
    ) -> NvResult<()> {
        let clamped_value = vibrance_value.clamp(VIBRANCE_MIN, VIBRANCE_MAX);

        let mut params = NvKmsSetDpyAttributeParams {
            request: NvKmsSetDpyAttributeRequest {
                device_handle,
                disp_handle,
                dpy_id,
                attribute: NvKmsDpyAttribute::DigitalVibrance,
                value: clamped_value,
            },
            reply: NvKmsSetDpyAttributeReply { padding: 0 },
        };

        unsafe {
            nvkms_ioctl(fd, NvKmsIoctlCommand::SetDpyAttribute, &mut params).map_err(|e| {
                NvControlError::VibranceControlFailed(format!("Failed to set vibrance: {}", e))
            })?;
        }

        Ok(())
    }

    /// Get vibrance using NVKMS ioctl
    fn get_vibrance_via_ioctl(
        fd: i32,
        device_handle: u32,
        disp_handle: u32,
        dpy_id: u32,
    ) -> NvResult<i64> {
        let mut params = NvKmsGetDpyAttributeParams {
            request: NvKmsGetDpyAttributeRequest {
                device_handle,
                disp_handle,
                dpy_id,
                attribute: NvKmsDpyAttribute::DigitalVibrance,
            },
            reply: unsafe { std::mem::zeroed() },
        };

        unsafe {
            nvkms_ioctl(fd, NvKmsIoctlCommand::GetDpyAttribute, &mut params).map_err(|e| {
                NvControlError::VibranceControlFailed(format!("Failed to get vibrance: {}", e))
            })?;
        }

        Ok(params.reply.value)
    }

    /// Convert percentage (0-200%) to vibrance range (-1024 to 1023)
    pub fn percentage_to_vibrance(&self, percentage: u32) -> i64 {
        let percentage = percentage.min(200);

        if percentage <= 100 {
            // 0-100% maps to -1024 to 0
            let ratio = percentage as f32 / 100.0;
            ((ratio - 1.0) * 1024.0) as i64
        } else {
            // 100-200% maps to 0 to 1023
            let ratio = (percentage - 100) as f32 / 100.0;
            (ratio * 1023.0) as i64
        }
    }

    /// Convert vibrance range (-1024 to 1023) to percentage (0-200%)
    pub fn vibrance_to_percentage(&self, vibrance: i64) -> u32 {
        if vibrance <= 0 {
            // -1024 to 0 maps to 0-100%
            (((vibrance + 1024) as f32 / 1024.0) * 100.0) as u32
        } else {
            // 0 to 1023 maps to 100-200%
            (100.0 + (vibrance as f32 / 1023.0 * 100.0)) as u32
        }
    }

    /// Reset all displays to default vibrance (100%)
    pub fn reset_all_vibrance(&mut self) -> NvResult<()> {
        self.set_vibrance_all(100)
    }

    /// List all available displays
    pub fn list_displays(&self) -> Vec<(u32, u32, String, bool)> {
        let mut displays = Vec::new();

        for device in &self.devices {
            for display in &device.displays {
                displays.push((
                    device.device_id,
                    display.dpy_id,
                    format!("{}:{}", display.dpy_id, display.connector_type),
                    display.connected,
                ));
            }
        }

        displays
    }

    /// Get comprehensive vibrance status
    pub fn get_vibrance_status(&self) -> HashMap<String, serde_json::Value> {
        let mut status = HashMap::new();

        status.insert(
            "driver_version".to_string(),
            serde_json::Value::String(self.driver_version.clone()),
        );
        status.insert(
            "open_driver".to_string(),
            serde_json::Value::Bool(self.open_driver),
        );
        status.insert(
            "devices".to_string(),
            serde_json::to_value(&self.devices).unwrap_or(serde_json::Value::Null),
        );

        status
    }

    /// Query display information from xrandr/wlr-randr
    fn query_display_info(_dpy_id: NvU32) -> DisplayQueryInfo {
        use std::process::Command;

        let mut info = DisplayQueryInfo {
            connector_type: "Unknown".to_string(),
            resolution: (1920, 1080),
            refresh_rate: Some(60.0),
        };

        // Try xrandr (X11)
        if let Ok(output) = Command::new("xrandr").arg("--query").output() {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Parse xrandr output for display info
                for line in output_str.lines() {
                    if line.contains(" connected") {
                        // Extract connector type (HDMI-1, DP-1, etc.)
                        if let Some(connector) = line.split_whitespace().next() {
                            info.connector_type = connector.to_string();
                        }
                        // Extract resolution and refresh rate
                        if let Some(mode) = line.split_whitespace().nth(2) {
                            if let Some((w, h)) = mode.split_once('x') {
                                if let (Ok(width), Ok(height)) = (w.parse(), h.parse()) {
                                    info.resolution = (width, height);
                                }
                            }
                        }
                        // Look for refresh rate
                        if line.contains("*") {
                            for part in line.split_whitespace() {
                                if part.contains("*") || part.contains("+") {
                                    if let Ok(rate) = part.trim_matches(|c| c == '*' || c == '+').parse::<f32>() {
                                        info.refresh_rate = Some(rate);
                                        break;
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        // Try wlr-randr (Wayland)
        if info.connector_type == "Unknown" {
            if let Ok(output) = Command::new("wlr-randr").output() {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    for line in output_str.lines() {
                        if !line.starts_with(' ') && !line.is_empty() {
                            if let Some(connector) = line.split_whitespace().next() {
                                info.connector_type = connector.to_string();
                            }
                        }
                        if line.contains("current") {
                            // Parse: "1920x1080 @ 60.000 Hz"
                            if let Some(res_part) = line.split_whitespace().find(|s| s.contains('x')) {
                                if let Some((w, h)) = res_part.split_once('x') {
                                    if let (Ok(width), Ok(height)) = (w.parse(), h.parse()) {
                                        info.resolution = (width, height);
                                    }
                                }
                            }
                            if let Some(rate_idx) = line.find("@ ") {
                                let rate_str = &line[rate_idx + 2..];
                                if let Some(rate) = rate_str.split_whitespace().next() {
                                    if let Ok(rate_val) = rate.parse::<f32>() {
                                        info.refresh_rate = Some(rate_val);
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        info
    }
}

struct DisplayQueryInfo {
    connector_type: String,
    resolution: (u32, u32),
    refresh_rate: Option<f32>,
}

/// Global static vibrance controller instance
static mut VIBRANCE_CONTROLLER: Option<NativeVibranceController> = None;
static mut CONTROLLER_INITIALIZED: bool = false;

/// Get or initialize the vibrance controller
pub fn get_vibrance_controller() -> NvResult<&'static mut NativeVibranceController> {
    unsafe {
        if !CONTROLLER_INITIALIZED {
            VIBRANCE_CONTROLLER = Some(NativeVibranceController::new()?);
            CONTROLLER_INITIALIZED = true;
        }
        #[allow(static_mut_refs)]
        VIBRANCE_CONTROLLER.as_mut().ok_or_else(|| {
            NvControlError::VibranceControlFailed("Failed to initialize vibrance controller".to_string())
        })
    }
}

/// Pure Rust vibrance commands - CLI interface
pub fn set_vibrance_all_native(percentage: u32) -> NvResult<()> {
    let controller = get_vibrance_controller()?;
    controller.set_vibrance_all(percentage)
}

pub fn set_display_vibrance_native(device_id: u32, display_id: u32, percentage: u32) -> NvResult<()> {
    let controller = get_vibrance_controller()?;
    controller.set_display_vibrance(device_id, display_id, percentage)
}

pub fn get_vibrance_status_native() -> NvResult<HashMap<String, serde_json::Value>> {
    let controller = get_vibrance_controller()?;
    Ok(controller.get_vibrance_status())
}

pub fn list_displays_native() -> NvResult<Vec<(u32, u32, String, bool)>> {
    let controller = get_vibrance_controller()?;
    Ok(controller.list_displays())
}

pub fn reset_vibrance_native() -> NvResult<()> {
    let controller = get_vibrance_controller()?;
    controller.reset_all_vibrance()
}
