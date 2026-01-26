// NVIDIA NVKMS (Kernel Mode Setting) API Bindings
// Based on NVIDIA Open GPU Kernel Modules 580+
// SPDX-License-Identifier: MIT

use bytemuck::{Pod, Zeroable};
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

// NVDpyIdList is actually just a bitmask (single NvU32), not an array
pub type NVDpyIdList = NvU32;

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
pub const NVKMS_IOCTL_IOWR: c_ulong = nix::request_code_readwrite!(
    NVKMS_IOCTL_MAGIC,
    NVKMS_IOCTL_CMD,
    std::mem::size_of::<NvKmsIoctlParams>()
);

// ===== NVKMS Ioctl Commands from nvkms-api.h =====
// These MUST match the enum order in the NVIDIA open source driver
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsIoctlCommand {
    AllocDevice = 0,               // NVKMS_IOCTL_ALLOC_DEVICE
    FreeDevice = 1,                // NVKMS_IOCTL_FREE_DEVICE
    QueryDisp = 2,                 // NVKMS_IOCTL_QUERY_DISP
    QueryConnectorStaticData = 3,  // NVKMS_IOCTL_QUERY_CONNECTOR_STATIC_DATA
    QueryConnectorDynamicData = 4, // NVKMS_IOCTL_QUERY_CONNECTOR_DYNAMIC_DATA
    QueryDpyStaticData = 5,        // NVKMS_IOCTL_QUERY_DPY_STATIC_DATA
    QueryDpyDynamicData = 6,       // NVKMS_IOCTL_QUERY_DPY_DYNAMIC_DATA
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
    SetDpyAttribute = 22,            // NVKMS_IOCTL_SET_DPY_ATTRIBUTE
    GetDpyAttribute = 23,            // NVKMS_IOCTL_GET_DPY_ATTRIBUTE
    GetDpyAttributeValidValues = 24, // NVKMS_IOCTL_GET_DPY_ATTRIBUTE_VALID_VALUES
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
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
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
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct NvKmsAttributeRange {
    pub min: NvS64,
    pub max: NvS64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
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
// Total size: Request=8, Reply=164, Params=172
pub const NVKMS_MAX_CONNECTORS_PER_DISP: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsQueryDispRequest {
    pub device_handle: NvKmsDeviceHandle,
    pub disp_handle: NvKmsDispHandle,
}

// QueryDispReply: 164 bytes total
// numConnectors at offset 16, connectorHandles at offset 20
#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct NvKmsQueryDispReply {
    pub valid_dpys: NVDpyIdList,  // offset 0
    pub boot_dpys: NVDpyIdList,   // offset 4
    pub mux_dpys: NVDpyIdList,    // offset 8
    pub frame_lock_handle: NvU32, // offset 12
    pub num_connectors: NvU32,    // offset 16
    pub connector_handles: [NvKmsConnectorHandle; NVKMS_MAX_CONNECTORS_PER_DISP], // offset 20
    // Remaining fields (gpu_string is actually 80 bytes starting at 84)
    pub _padding: [u8; 164 - 20 - (NVKMS_MAX_CONNECTORS_PER_DISP * 4)], // 164 - 20 - 64 = 80
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsQueryDispParams {
    pub request: NvKmsQueryDispRequest,
    pub reply: NvKmsQueryDispReply,
}

// ===== Connector Types =====
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsConnectorType {
    Dp = 0,
    Vga = 1,
    DviI = 2,
    DviD = 3,
    Adc = 4,
    Lvds = 5,
    Hdmi = 6,
    Usbc = 7,
    Dsi = 8,
    Unknown = 255,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NvKmsConnectorSignalFormat {
    Unknown = 0,
    Vga = 1,
    Lvds = 2,
    Tmds = 3,
    Dp = 4,
    Dsi = 5,
}

// ===== Query Connector Static Data =====
// Total: Request=12, Reply=32, Params=44
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsQueryConnectorStaticDataRequest {
    pub device_handle: NvKmsDeviceHandle,
    pub disp_handle: NvKmsDispHandle,
    pub connector_handle: NvKmsConnectorHandle,
}

// QueryConnectorStaticDataReply: 32 bytes
// dpyId at 0, type at 12, signalFormat at 20, physicalIndex at 24
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsQueryConnectorStaticDataReply {
    pub dpy_id: NVDpyId,                           // offset 0
    pub is_dp: NvBool,                             // offset 4
    pub is_lvds: NvBool,                           // offset 5
    pub location_on_chip: NvBool,                  // offset 6
    pub _pad1: NvBool,                             // offset 7 (alignment)
    pub legacy_type_index: NvU32,                  // offset 8
    pub connector_type: NvKmsConnectorType,        // offset 12
    pub type_index: NvU32,                         // offset 16
    pub signal_format: NvKmsConnectorSignalFormat, // offset 20
    pub physical_index: NvU32,                     // offset 24
    pub physical_location: NvU32,                  // offset 28
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsQueryConnectorStaticDataParams {
    pub request: NvKmsQueryConnectorStaticDataRequest,
    pub reply: NvKmsQueryConnectorStaticDataReply,
}

// ===== Query Dpy Dynamic Data =====
// Exact sizes from driver 580.105.08:
// - Request: 2072 bytes (has EDID buffer and many flags)
// - Reply: 35088 bytes (connected at offset 132)
// - Total Params: 37160 bytes

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct NvKmsQueryDpyDynamicDataRequest {
    pub device_handle: NvKmsDeviceHandle,
    pub disp_handle: NvKmsDispHandle,
    pub dpy_id: NVDpyId,
    // Padding for additional fields (forceConnected, forceDisconnected, overrideEdid,
    // ignoreEdid, ignoreEdidChecksum, allowDVISpecPClkOverride, dpInbandStereoSignaling,
    // disableACPIBrightnessHotkeys, edid buffer, etc.)
    pub _padding: [u8; 2072 - 12], // 12 = 4 + 4 + 4 for the three handles
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct NvKmsQueryDpyDynamicDataReply {
    // Fields before 'connected' (offset 132)
    pub _pre_connected: [u8; 132],
    pub connected: NvBool,
    // Rest of the struct
    pub _post_connected: [u8; 35088 - 132 - 1],
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
pub struct NvKmsQueryDpyDynamicDataParams {
    pub request: NvKmsQueryDpyDynamicDataRequest,
    pub reply: NvKmsQueryDpyDynamicDataReply,
}

// ===== Alloc Device Structures =====
// These constants must match the NVIDIA driver exactly (from nvkms-api.h)
pub const NVKMS_NVIDIA_DRIVER_VERSION_STRING_LENGTH: usize = 32;
pub const NVKMS_MAX_DEVICE_REGISTRY_KEYS: usize = 16;
pub const NVKMS_MAX_DEVICE_REGISTRY_KEYNAME_LEN: usize = 32;

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

// AllocDeviceReply: 888 bytes total, 8-byte alignment (due to NvKmsLayerCapabilities)
// Key fields: status (offset 0), deviceHandle (offset 4), numDisps (offset 16), dispHandles (offset 20)
#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct NvKmsAllocDeviceReply {
    pub status: NvKmsAllocDeviceStatus,   // offset 0
    pub device_handle: NvKmsDeviceHandle, // offset 4
    pub sub_device_mask: NvU32,           // offset 8
    pub num_heads: NvU32,                 // offset 12
    pub num_disps: NvU32,                 // offset 16
    pub disp_handles: [NvKmsDispHandle; NVKMS_MAX_SUBDEVICES], // offset 20
    // Remaining fields (caps, layer info, etc.)
    pub _padding: [u8; 888 - 20 - (NVKMS_MAX_SUBDEVICES * 4)], // 888 - 20 - 32 = 836
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsAllocDeviceParams {
    pub request: NvKmsAllocDeviceRequest,
    pub reply: NvKmsAllocDeviceReply,
}

// ===== Free Device Structures =====
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsFreeDeviceRequest {
    pub device_handle: NvKmsDeviceHandle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsFreeDeviceReply {
    pub padding: NvU32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvKmsFreeDeviceParams {
    pub request: NvKmsFreeDeviceRequest,
    pub reply: NvKmsFreeDeviceReply,
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
///
/// # Safety
/// - `fd` must be a valid file descriptor for /dev/nvidia-modeset
/// - `params` must be the correct parameter type for `cmd`
/// - The caller must ensure params fields are properly initialized for the command
pub unsafe fn nvkms_ioctl<T>(
    fd: std::os::unix::io::RawFd,
    cmd: NvKmsIoctlCommand,
    params: &mut T,
) -> Result<i32, nix::Error> {
    let ioctl_params = create_ioctl_params(cmd, params);

    // SAFETY: Caller guarantees fd is valid and params matches the ioctl command.
    // NVKMS uses indirect parameter passing: ioctl_params contains a pointer to params.
    let result = unsafe { libc::ioctl(fd, NVKMS_IOCTL_IOWR as c_ulong, &ioctl_params as *const _) };

    nix::errno::Errno::result(result)
}

// SAFETY: All these types are valid when zero-initialized:
// - Enums have discriminant 0 as a valid variant (Success, Dp, Unknown, Range)
// - Unions: all-zero bytes is valid for both range (two i64s) and bits (one u32)
// - All fields are primitive integers, bools, or arrays of such
// - #[repr(C)] structs with all-zero bytes are valid FFI types
unsafe impl Zeroable for NvKmsAllocDeviceStatus {}
unsafe impl Zeroable for NvKmsConnectorType {}
unsafe impl Zeroable for NvKmsConnectorSignalFormat {}
unsafe impl Zeroable for NvKmsAttributeType {}
unsafe impl Zeroable for NvKmsAttributeValidValuesUnion {}
unsafe impl Zeroable for NvKmsAttributeValidValuesCommonReply {}
unsafe impl Zeroable for NvKmsAllocDeviceReply {}
unsafe impl Zeroable for NvKmsQueryConnectorStaticDataReply {}
