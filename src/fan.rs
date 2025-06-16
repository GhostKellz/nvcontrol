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
    vec![FanInfo {
        id: 0,
        rpm: Some(1500),
        percent: Some(40),
        controllable: true,
    }]
}

/// Get info for a specific fan (stub)
pub fn get_fan_info(fan_id: usize) -> Option<FanInfo> {
    list_fans().into_iter().find(|f| f.id == fan_id)
}

/// Set fan speed (stub)
pub fn set_fan_speed(_fan_id: usize, _speed_percent: u8) {
    // TODO: Implement via NVML/nvidia-smi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fan_info_creation() {
        let fan = FanInfo {
            id: 0,
            rpm: Some(1500),
            percent: Some(40),
            controllable: true,
        };
        assert_eq!(fan.id, 0);
        assert_eq!(fan.rpm, Some(1500));
        assert_eq!(fan.percent, Some(40));
        assert!(fan.controllable);
    }

    #[test]
    fn test_list_fans() {
        let fans = list_fans();
        assert!(!fans.is_empty());
        assert_eq!(fans[0].id, 0);
    }

    #[test]
    fn test_get_fan_info() {
        let fan = get_fan_info(0);
        assert!(fan.is_some());
        if let Some(fan) = fan {
            assert_eq!(fan.id, 0);
        }
    }
}
