// NVIDIA NVKMS (Kernel Mode Setting) API Bindings
// Based on NVIDIA Open GPU Kernel Modules 580+
// SPDX-License-Identifier: MIT

use std::os::raw::c_ulong;

// ===== Basic Types from nvtypes.h =====
pub type NvU8 = u8;
pub type NvU16 = u16;
pub type NvU32 = u32;
pub type NvU64 = u64;
pub type NvS8 = i8;
pub type NvS16 = i16;
pub type NvS32 = i32;
pub type NvS64 = i64;
pub type NvBool = NvU8;

// ===== NVKMS API Types from nvkms-api-types.h =====
pub const NVKMS_MAX_SUBDEVICES: usize = 8; // NV_MAX_SUBDEVICES
pub const NVKMS_MAX_HEADS_PER_DISP: usize = 8; // NV_MAX_HEADS

pub type NvKmsDeviceHandle = NvU32;
pub type NvKmsDispHandle = NvU32;
pub type NvKmsConnectorHandle = NvU32;
pub type NvKmsSurfaceHandle = NvU32;

// ===== Display ID (from nv_dpy_id.h) =====
pub type NVDpyId = NvU32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NVDpyIdList {
    pub id: [NvU32; 8], // Array to hold multiple display IDs
}

impl Default for NVDpyIdList {
    fn default() -> Self {
        Self { id: [0; 8] }
    }
}

// ===== NVKMS IOC TL Infrastructure from nvkms-ioctl.h =====
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsIoctlParams {
    pub cmd: NvU32,
    pub size: NvU32,
    pub address: NvU64,
}

pub const NVKMS_IOCTL_MAGIC: u8 = b'm';
pub const NVKMS_IOCTL_CMD: u8 = 0;

// ioctl request code: _IOWR(NVKMS_IOCTL_MAGIC, NVKMS_IOCTL_CMD, NvKmsIoctlParams)
pub const NVKMS_IOCTL_IOWR: c_ulong = nix::request_code_readwrite!(NVKMS_IOCTL_MAGIC, NVKMS_IOCTL_CMD, std::mem::size_of::<NvKmsIoctlParams>());

// ===== NVKMS Ioctl Commands from nvkms-api.h =====
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsIoctlCommand {
    AllocDevice = 0,
    FreeDevice = 1,
    QueryDisp = 2,
    QueryConnectorStaticData = 3,
    QueryConnectorDynamicData = 4,
    QueryDpyStaticData = 5,
    QueryDpyDynamicData = 6,
    ValidateModeIndex = 7,
    ValidateMode = 8,
    SetMode = 9,
    SetCursorImage = 10,
    MoveCursor = 11,
    SetLut = 12,
    CheckLutNotifier = 13,
    IdleBaseChannel = 14,
    Flip = 15,
    DeclareDynamicDpyInterest = 16,
    RegisterSurface = 17,
    UnregisterSurface = 18,
    GrantSurface = 19,
    AcquireSurface = 20,
    ReleaseSurface = 21,
    SetDpyAttribute = 22,
    GetDpyAttribute = 23,
    GetDpyAttributeValidValues = 24,
    SetDispAttribute = 25,
    GetDispAttribute = 26,
    GetDispAttributeValidValues = 27,
}

// ===== Display Attributes from nvkms-api.h =====
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsDpyAttribute {
    BacklightBrightness = 0,
    Scanline = 1,
    HwHead = 2,
    Head = 3,
    RequestedDithering = 4,
    RequestedDitheringMode = 5,
    RequestedDitheringDepth = 6,
    CurrentDithering = 7,
    CurrentDitheringMode = 8,
    CurrentDitheringDepth = 9,
    DigitalVibrance = 10,
    ImageSharpening = 11,
    ImageSharpeningAvailable = 12,
    ImageSharpeningDefault = 13,
    RequestedColorSpace = 14,
    CurrentColorSpace = 15,
    RequestedColorRange = 16,
    CurrentColorRange = 17,
    CurrentColorBpc = 18,
    DigitalSignal = 19,
    DigitalLinkType = 20,
}

// ===== Attribute Type =====
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsAttributeType {
    Range = 0,
    IntBits = 1,
    Bool = 2,
}

// ===== Color Range Values =====
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsDpyAttributeColorRangeValue {
    Full = 0,
    Limited = 1,
}

// ===== Color Space Values =====
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsDpyAttributeColorSpaceValue {
    Rgb = 0,
    YCbCr422 = 1,
    YCbCr444 = 2,
}

// ===== Dithering Values =====
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsDpyAttributeDitheringValue {
    Auto = 0,
    Enabled = 1,
    Disabled = 2,
}

// ===== Set Display Attribute Structures =====
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsSetDpyAttributeRequest {
    pub device_handle: NvKmsDeviceHandle,
    pub disp_handle: NvKmsDispHandle,
    pub dpy_id: NVDpyId,
    pub attribute: NvKmsDpyAttribute,
    pub value: NvS64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsSetDpyAttributeReply {
    pub padding: NvU32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsSetDpyAttributeParams {
    pub request: NvKmsSetDpyAttributeRequest,
    pub reply: NvKmsSetDpyAttributeReply,
}

// ===== Get Display Attribute Structures =====
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsGetDpyAttributeRequest {
    pub device_handle: NvKmsDeviceHandle,
    pub disp_handle: NvKmsDispHandle,
    pub dpy_id: NVDpyId,
    pub attribute: NvKmsDpyAttribute,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsGetDpyAttributeReply {
    pub value: NvS64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsGetDpyAttributeParams {
    pub request: NvKmsGetDpyAttributeRequest,
    pub reply: NvKmsGetDpyAttributeReply,
}

// ===== Get Attribute Valid Values =====
#[repr(C)]
#[derive(Copy, Clone)]
pub union NvKmsAttributeValidValuesUnion {
    pub range: NvKmsAttributeRange,
    pub bits: NvKmsAttributeBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsAttributeRange {
    pub min: NvS64,
    pub max: NvS64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NvKmsAttributeBits {
    pub ints: NvU32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsAttributeValidValuesCommonReply {
    pub readable: NvBool,
    pub writable: NvBool,
    pub attr_type: NvKmsAttributeType,
    pub u: NvKmsAttributeValidValuesUnion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsGetDpyAttributeValidValuesRequest {
    pub device_handle: NvKmsDeviceHandle,
    pub disp_handle: NvKmsDispHandle,
    pub dpy_id: NVDpyId,
    pub attribute: NvKmsDpyAttribute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsGetDpyAttributeValidValuesParams {
    pub request: NvKmsGetDpyAttributeValidValuesRequest,
    pub reply: NvKmsAttributeValidValuesCommonReply,
}

// ===== Query Display Structures =====
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsQueryDispRequest {
    pub device_handle: NvKmsDeviceHandle,
    pub disp_handle: NvKmsDispHandle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsQueryDispReply {
    pub valid_dpys: NVDpyIdList,
    pub boot_dpys: NVDpyIdList,
    pub mux_dpys: NVDpyIdList,
    pub connector_handles: [NvKmsConnectorHandle; 32],
    pub num_connectors: NvU32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsQueryDispParams {
    pub request: NvKmsQueryDispRequest,
    pub reply: NvKmsQueryDispReply,
}

// ===== Alloc Device Structures =====
pub const NVKMS_NVIDIA_DRIVER_VERSION_STRING_LENGTH: usize = 64;
pub const NVKMS_MAX_DEVICE_REGISTRY_KEYS: usize = 16;
pub const NVKMS_MAX_DEVICE_REGISTRY_KEYNAME_LEN: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsDeviceId {
    pub rm_device_id: NvU32,
    pub mig_device: MIGDeviceId,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIGDeviceId {
    pub value: NvU32, // 0 = NO_MIG_DEVICE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegistryKey {
    pub name: [u8; NVKMS_MAX_DEVICE_REGISTRY_KEYNAME_LEN],
    pub value: NvU32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsAllocDeviceRequest {
    pub version_string: [u8; NVKMS_NVIDIA_DRIVER_VERSION_STRING_LENGTH],
    pub device_id: NvKmsDeviceId,
    pub sli_mosaic: NvBool,
    pub try_infer_sli_mosaic_from_existing_device: NvBool,
    pub no3d: NvBool,
    pub enable_console_hotplug_handling: NvBool,
    pub registry_keys: [RegistryKey; NVKMS_MAX_DEVICE_REGISTRY_KEYS],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsAllocDeviceStatus {
    Success = 0,
    VersionMismatch = 1,
    BadDeviceId = 2,
    AlreadyAllocated = 3,
    Unspecified = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsAllocDeviceReply {
    pub device_handle: NvKmsDeviceHandle,
    pub disp_handles: [NvKmsDispHandle; NVKMS_MAX_SUBDEVICES],
    pub num_disps: NvU32,
    pub status: NvKmsAllocDeviceStatus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsAllocDeviceParams {
    pub request: NvKmsAllocDeviceRequest,
    pub reply: NvKmsAllocDeviceReply,
}

// ===== Helper Functions =====

/// Helper to create an ioctl params structure
pub fn create_ioctl_params<T>(cmd: NvKmsIoctlCommand, params: &T) -> NvKmsIoctlParams {
    NvKmsIoctlParams {
        cmd: cmd as NvU32,
        size: std::mem::size_of::<T>() as NvU32,
        address: params as *const T as NvU64,
    }
}

/// Perform NVKMS ioctl
pub unsafe fn nvkms_ioctl<T>(
    fd: std::os::unix::io::RawFd,
    cmd: NvKmsIoctlCommand,
    params: &mut T,
) -> Result<i32, std::io::Error> {
    let ioctl_params = create_ioctl_params(cmd, params);

    // SAFETY: Caller ensures fd is valid and params matches the ioctl command
    let result = unsafe {
        libc::ioctl(fd, NVKMS_IOCTL_IOWR as c_ulong, &ioctl_params as *const _)
    };

    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(result)
    }
}
