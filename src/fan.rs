/// Represents a GPU fan
pub struct FanInfo {
    pub id: usize,
    pub rpm: Option<u32>,
    pub percent: Option<u8>,
    pub controllable: bool,
}

/// List all fans (stubbed)
pub fn list_fans() -> Vec<FanInfo> {
    // TODO: Query real fan info via NVML/nvidia-smi
    vec![FanInfo { id: 0, rpm: Some(1500), percent: Some(40), controllable: true }]
}

/// Get info for a specific fan (stub)
pub fn get_fan_info(fan_id: usize) -> Option<FanInfo> {
    list_fans().into_iter().find(|f| f.id == fan_id)
}

/// Set fan speed (stub)
pub fn set_fan_speed(_fan_id: usize, _speed_percent: u8) {
    // TODO: Implement via NVML/nvidia-smi
}