// Enhanced Display Controls - Image Sharpening, Color Range/Space, Dithering
// Native implementation using NVKMS ioctls
use crate::nvkms_bindings::*;
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

const NVIDIA_MODESET_DEVICE: &str = "/dev/nvidia-modeset";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayControls {
    pub fd: i32,
    pub device_handle: u32,
    pub disp_handle: u32,
    pub dpy_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSharpeningInfo {
    pub available: bool,
    pub default_value: i64,
    pub current_value: i64,
    pub range: (i64, i64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorRangeInfo {
    pub current: ColorRange,
    pub supported: Vec<ColorRange>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorRange {
    Full,
    Limited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSpaceInfo {
    pub current: ColorSpace,
    pub supported: Vec<ColorSpace>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorSpace {
    RGB,
    YCbCr422,
    YCbCr444,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DitheringInfo {
    pub enabled: bool,
    pub mode: DitheringMode,
    pub depth: DitheringDepth,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DitheringMode {
    Auto,
    Dynamic2x2,
    Static2x2,
    Temporal,
    None,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DitheringDepth {
    Auto,
    SixBits,
    EightBits,
    None,
}

impl DisplayControls {
    pub fn new(device_handle: u32, disp_handle: u32, dpy_id: u32) -> NvResult<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(NVIDIA_MODESET_DEVICE)
            .map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!(
                    "Failed to open nvidia-modeset device: {}",
                    e
                ))
            })?;

        Ok(DisplayControls {
            fd: file.as_raw_fd(),
            device_handle,
            disp_handle,
            dpy_id,
        })
    }

    // ===== Image Sharpening =====

    pub fn get_image_sharpening_info(&self) -> NvResult<ImageSharpeningInfo> {
        // Check if image sharpening is available
        let available =
            self.get_display_attribute_i64(NvKmsDpyAttribute::ImageSharpeningAvailable)?;

        let default_value = if available != 0 {
            self.get_display_attribute_i64(NvKmsDpyAttribute::ImageSharpeningDefault)?
        } else {
            0
        };

        let current_value = if available != 0 {
            self.get_display_attribute_i64(NvKmsDpyAttribute::ImageSharpening)?
        } else {
            0
        };

        // Get valid range
        let range = self
            .get_attribute_range(NvKmsDpyAttribute::ImageSharpening)
            .unwrap_or((0, 100));

        Ok(ImageSharpeningInfo {
            available: available != 0,
            default_value,
            current_value,
            range,
        })
    }

    pub fn set_image_sharpening(&self, value: i64) -> NvResult<()> {
        self.set_display_attribute_i64(NvKmsDpyAttribute::ImageSharpening, value)
    }

    pub fn reset_image_sharpening(&self) -> NvResult<()> {
        let info = self.get_image_sharpening_info()?;
        self.set_image_sharpening(info.default_value)
    }

    // ===== Color Range =====

    pub fn get_color_range(&self) -> NvResult<ColorRangeInfo> {
        let current_value = self.get_display_attribute_i64(NvKmsDpyAttribute::CurrentColorRange)?;

        let current = match current_value {
            0 => ColorRange::Full,
            1 => ColorRange::Limited,
            _ => ColorRange::Full,
        };

        Ok(ColorRangeInfo {
            current,
            supported: vec![ColorRange::Full, ColorRange::Limited],
        })
    }

    pub fn set_color_range(&self, range: ColorRange) -> NvResult<()> {
        let value = match range {
            ColorRange::Full => 0,
            ColorRange::Limited => 1,
        };
        self.set_display_attribute_i64(NvKmsDpyAttribute::RequestedColorRange, value)
    }

    // ===== Color Space =====

    pub fn get_color_space(&self) -> NvResult<ColorSpaceInfo> {
        let current_value = self.get_display_attribute_i64(NvKmsDpyAttribute::CurrentColorSpace)?;

        let current = match current_value {
            0 => ColorSpace::RGB,
            1 => ColorSpace::YCbCr422,
            2 => ColorSpace::YCbCr444,
            _ => ColorSpace::RGB,
        };

        Ok(ColorSpaceInfo {
            current,
            supported: vec![ColorSpace::RGB, ColorSpace::YCbCr422, ColorSpace::YCbCr444],
        })
    }

    pub fn set_color_space(&self, space: ColorSpace) -> NvResult<()> {
        let value = match space {
            ColorSpace::RGB => 0,
            ColorSpace::YCbCr422 => 1,
            ColorSpace::YCbCr444 => 2,
        };
        self.set_display_attribute_i64(NvKmsDpyAttribute::RequestedColorSpace, value)
    }

    // ===== Dithering =====

    pub fn get_dithering_info(&self) -> NvResult<DitheringInfo> {
        let enabled_value = self.get_display_attribute_i64(NvKmsDpyAttribute::CurrentDithering)?;
        let mode_value = self.get_display_attribute_i64(NvKmsDpyAttribute::CurrentDitheringMode)?;
        let depth_value =
            self.get_display_attribute_i64(NvKmsDpyAttribute::CurrentDitheringDepth)?;

        let mode = match mode_value {
            0 => DitheringMode::None,
            1 => DitheringMode::Dynamic2x2,
            2 => DitheringMode::Static2x2,
            3 => DitheringMode::Temporal,
            _ => DitheringMode::Auto,
        };

        let depth = match depth_value {
            0 => DitheringDepth::None,
            1 => DitheringDepth::SixBits,
            2 => DitheringDepth::EightBits,
            _ => DitheringDepth::Auto,
        };

        Ok(DitheringInfo {
            enabled: enabled_value != 0,
            mode,
            depth,
        })
    }

    pub fn set_dithering(
        &self,
        enabled: bool,
        mode: DitheringMode,
        depth: DitheringDepth,
    ) -> NvResult<()> {
        // Set dithering enabled/disabled
        let enabled_value = if enabled { 1 } else { 2 };
        self.set_display_attribute_i64(NvKmsDpyAttribute::RequestedDithering, enabled_value)?;

        // Set dithering mode
        let mode_value = match mode {
            DitheringMode::Auto => 0,
            DitheringMode::Dynamic2x2 => 1,
            DitheringMode::Static2x2 => 2,
            DitheringMode::Temporal => 3,
            DitheringMode::None => 0,
        };
        self.set_display_attribute_i64(NvKmsDpyAttribute::RequestedDitheringMode, mode_value)?;

        // Set dithering depth
        let depth_value = match depth {
            DitheringDepth::Auto => 0,
            DitheringDepth::SixBits => 1,
            DitheringDepth::EightBits => 2,
            DitheringDepth::None => 0,
        };
        self.set_display_attribute_i64(NvKmsDpyAttribute::RequestedDitheringDepth, depth_value)?;

        Ok(())
    }

    // ===== Helper Methods =====

    fn get_display_attribute_i64(&self, attribute: NvKmsDpyAttribute) -> NvResult<i64> {
        let mut params = NvKmsGetDpyAttributeParams {
            request: NvKmsGetDpyAttributeRequest {
                device_handle: self.device_handle,
                disp_handle: self.disp_handle,
                dpy_id: self.dpy_id,
                attribute,
            },
            reply: unsafe { std::mem::zeroed() },
        };

        unsafe {
            nvkms_ioctl(self.fd, NvKmsIoctlCommand::GetDpyAttribute, &mut params).map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!(
                    "Failed to get display attribute: {}",
                    e
                ))
            })?;
        }

        Ok(params.reply.value)
    }

    fn set_display_attribute_i64(&self, attribute: NvKmsDpyAttribute, value: i64) -> NvResult<()> {
        let mut params = NvKmsSetDpyAttributeParams {
            request: NvKmsSetDpyAttributeRequest {
                device_handle: self.device_handle,
                disp_handle: self.disp_handle,
                dpy_id: self.dpy_id,
                attribute,
                value,
            },
            reply: NvKmsSetDpyAttributeReply { padding: 0 },
        };

        unsafe {
            nvkms_ioctl(self.fd, NvKmsIoctlCommand::SetDpyAttribute, &mut params).map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!(
                    "Failed to set display attribute: {}",
                    e
                ))
            })?;
        }

        Ok(())
    }

    fn get_attribute_range(&self, attribute: NvKmsDpyAttribute) -> NvResult<(i64, i64)> {
        let mut params = NvKmsGetDpyAttributeValidValuesParams {
            request: NvKmsGetDpyAttributeValidValuesRequest {
                device_handle: self.device_handle,
                disp_handle: self.disp_handle,
                dpy_id: self.dpy_id,
                attribute,
            },
            reply: unsafe { std::mem::zeroed() },
        };

        unsafe {
            nvkms_ioctl(
                self.fd,
                NvKmsIoctlCommand::GetDpyAttributeValidValues,
                &mut params,
            )
            .map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!(
                    "Failed to get attribute range: {}",
                    e
                ))
            })?;

            if params.reply.attr_type == NvKmsAttributeType::Range {
                Ok((params.reply.u.range.min, params.reply.u.range.max))
            } else {
                Err(NvControlError::DisplayDetectionFailed(
                    "Attribute is not a range type".to_string(),
                ))
            }
        }
    }
}

// ===== CLI Functions =====

pub fn set_image_sharpening_cli(_device_id: u32, display_id: u32, value: i64) -> NvResult<()> {
    // TODO: Get device/disp handles from vibrance controller
    // For now, use placeholder values
    let controls = DisplayControls::new(0, 0, display_id)?;
    controls.set_image_sharpening(value)?;
    println!(
        "✅ Set image sharpening to {} for display {}",
        value, display_id
    );
    Ok(())
}

pub fn get_image_sharpening_info_cli(
    _device_id: u32,
    display_id: u32,
) -> NvResult<ImageSharpeningInfo> {
    let controls = DisplayControls::new(0, 0, display_id)?;
    controls.get_image_sharpening_info()
}

pub fn set_color_range_cli(_device_id: u32, display_id: u32, range: ColorRange) -> NvResult<()> {
    let controls = DisplayControls::new(0, 0, display_id)?;
    controls.set_color_range(range)?;
    println!(
        "✅ Set color range to {:?} for display {}",
        range, display_id
    );
    Ok(())
}

pub fn set_color_space_cli(_device_id: u32, display_id: u32, space: ColorSpace) -> NvResult<()> {
    let controls = DisplayControls::new(0, 0, display_id)?;
    controls.set_color_space(space)?;
    println!(
        "✅ Set color space to {:?} for display {}",
        space, display_id
    );
    Ok(())
}

pub fn set_dithering_cli(
    _device_id: u32,
    display_id: u32,
    enabled: bool,
    mode: DitheringMode,
    depth: DitheringDepth,
) -> NvResult<()> {
    let controls = DisplayControls::new(0, 0, display_id)?;
    controls.set_dithering(enabled, mode, depth)?;
    println!(
        "✅ Set dithering (enabled={}, mode={:?}, depth={:?}) for display {}",
        enabled, mode, depth, display_id
    );
    Ok(())
}
