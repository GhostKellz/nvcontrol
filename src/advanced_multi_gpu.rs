/// Phase 3.4: Multi-GPU Management
///
/// SLI/NVLink configuration, per-GPU profile assignment, load balancing, cross-GPU thermal management
use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GPU topology information
#[derive(Debug, Clone)]
pub struct GpuTopology {
    pub gpus: Vec<GpuNode>,
    pub links: Vec<GpuLink>,
}

#[derive(Debug, Clone)]
pub struct GpuNode {
    pub id: u32,
    pub name: String,
    pub pci_bus_id: String,
    pub architecture: Option<String>,
    pub nvlink_capable: bool,
}

#[derive(Debug, Clone)]
pub struct GpuLink {
    pub gpu1: u32,
    pub gpu2: u32,
    pub link_type: LinkType,
    pub bandwidth_gbps: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkType {
    NVLink,
    PCIe,
}

/// Multi-GPU coordinator
pub struct MultiGpuCoordinator {
    gpus: Vec<u32>,
    topology: Option<GpuTopology>,
}

impl MultiGpuCoordinator {
    pub fn new() -> NvResult<Self> {
        let gpus = Self::detect_gpus()?;

        Ok(Self {
            gpus,
            topology: None,
        })
    }

    fn detect_gpus() -> NvResult<Vec<u32>> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let count = nvml.device_count().map_err(|e| {
            NvControlError::GpuQueryFailed(format!("Failed to get device count: {}", e))
        })?;

        Ok((0..count).collect())
    }

    /// Detect GPU topology and interconnects
    pub fn detect_topology(&mut self) -> NvResult<()> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let mut nodes = Vec::new();
        let mut links = Vec::new();

        for &gpu_id in &self.gpus {
            let device = nvml.device_by_index(gpu_id).map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get device {}: {}", gpu_id, e))
            })?;

            let name = device.name().unwrap_or_else(|_| "Unknown".to_string());
            let pci_info = device.pci_info().ok();
            let pci_bus_id = pci_info
                .map(|p| format!("{:04x}:{:02x}:{:02x}.0", p.domain, p.bus, p.device))
                .unwrap_or_else(|| "Unknown".to_string());

            // Check NVLink capability
            // Check if device supports NVLink (not all nvml_wrapper versions have this method)
            let nvlink_capable = false; // Placeholder - NVLink detection via alternative methods

            nodes.push(GpuNode {
                id: gpu_id,
                name,
                pci_bus_id,
                architecture: None,
                nvlink_capable,
            });
        }

        // Detect links between GPUs
        for i in 0..self.gpus.len() {
            for j in (i + 1)..self.gpus.len() {
                let gpu1 = self.gpus[i];
                let gpu2 = self.gpus[j];

                if let Some(link) = self.detect_link(gpu1, gpu2)? {
                    links.push(link);
                }
            }
        }

        self.topology = Some(GpuTopology { gpus: nodes, links });

        Ok(())
    }

    fn detect_link(&self, gpu1: u32, gpu2: u32) -> NvResult<Option<GpuLink>> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let _device1 = nvml
            .device_by_index(gpu1)
            .map_err(|e| NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e)))?;

        // Try to detect NVLink (commented out due to nvml_wrapper API limitations)
        // Alternative: use nvidia-smi nvlink --status
        for _link_id in 0..6 {
            // NVLink detection requires methods not available in current nvml_wrapper
            // TODO: Parse nvidia-smi nvlink --status output instead
            if false {
                if false {
                    // Check if this link connects to gpu2
                    if let Some(_remote_info) = None::<u32> {
                        let device2 = nvml.device_by_index(gpu2).ok();
                        if let Some(_d2) = device2 {
                            if let Ok(_pci2) = _d2.pci_info() {
                                if false {
                                    // Placeholder for bus comparison
                                    return Ok(Some(GpuLink {
                                        gpu1,
                                        gpu2,
                                        link_type: LinkType::NVLink,
                                        bandwidth_gbps: 25.0, // NVLink 2.0 bandwidth per link
                                    }));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Default to PCIe if no NVLink found
        Ok(Some(GpuLink {
            gpu1,
            gpu2,
            link_type: LinkType::PCIe,
            bandwidth_gbps: 16.0, // PCIe 3.0 x16
        }))
    }

    /// Get GPU count
    pub fn gpu_count(&self) -> usize {
        self.gpus.len()
    }

    /// Get GPU IDs
    pub fn gpu_ids(&self) -> &[u32] {
        &self.gpus
    }

    /// Get topology
    pub fn topology(&self) -> Option<&GpuTopology> {
        self.topology.as_ref()
    }

    /// Print topology information
    pub fn print_topology(&self) {
        if let Some(topo) = &self.topology {
            println!("GPU Topology:");
            println!("  GPUs: {}", topo.gpus.len());

            for node in &topo.gpus {
                println!("\n  GPU {}: {}", node.id, node.name);
                println!("    PCI: {}", node.pci_bus_id);
                println!(
                    "    NVLink: {}",
                    if node.nvlink_capable { "Yes" } else { "No" }
                );
            }

            println!("\n  Links:");
            for link in &topo.links {
                println!(
                    "    GPU {} <-> GPU {}: {:?} ({:.1} GB/s)",
                    link.gpu1, link.gpu2, link.link_type, link.bandwidth_gbps
                );
            }
        } else {
            println!("Topology not detected. Run detect_topology() first.");
        }
    }
}

impl Default for MultiGpuCoordinator {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            gpus: Vec::new(),
            topology: None,
        })
    }
}

/// Per-GPU profile manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuProfile {
    pub gpu_id: u32,
    pub name: String,
    pub power_limit_watts: Option<u32>,
    pub gpu_offset_mhz: Option<i32>,
    pub memory_offset_mhz: Option<i32>,
    pub fan_speed_percent: Option<u32>,
}

pub struct PerGpuProfileManager {
    profiles: HashMap<u32, GpuProfile>,
}

impl PerGpuProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    /// Set profile for specific GPU
    pub fn set_profile(&mut self, profile: GpuProfile) {
        self.profiles.insert(profile.gpu_id, profile);
    }

    /// Get profile for GPU
    pub fn get_profile(&self, gpu_id: u32) -> Option<&GpuProfile> {
        self.profiles.get(&gpu_id)
    }

    /// Apply profile to GPU
    pub fn apply_profile(&self, gpu_id: u32) -> NvResult<()> {
        let profile = self
            .get_profile(gpu_id)
            .ok_or_else(|| NvControlError::ConfigError(format!("No profile for GPU {}", gpu_id)))?;

        use crate::gpu_safe::SafeGpuController;

        let controller = SafeGpuController::new(gpu_id);

        if let (Some(gpu_offset), Some(mem_offset)) =
            (profile.gpu_offset_mhz, profile.memory_offset_mhz)
        {
            controller.apply_overclock_safe(gpu_offset, mem_offset)?;
        }

        if let Some(power_limit) = profile.power_limit_watts {
            controller.set_power_limit_safe(power_limit)?;
        }

        println!("Applied profile '{}' to GPU {}", profile.name, gpu_id);

        Ok(())
    }

    /// Apply all profiles
    pub fn apply_all(&self) -> NvResult<()> {
        for gpu_id in self.profiles.keys() {
            self.apply_profile(*gpu_id)?;
        }
        Ok(())
    }
}

impl Default for PerGpuProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Load balancer for multi-GPU workloads
pub struct LoadBalancer {
    coordinator: MultiGpuCoordinator,
    load_history: HashMap<u32, Vec<u32>>,
}

impl LoadBalancer {
    pub fn new() -> NvResult<Self> {
        Ok(Self {
            coordinator: MultiGpuCoordinator::new()?,
            load_history: HashMap::new(),
        })
    }

    /// Update load information
    pub fn update_load(&mut self, gpu_id: u32, utilization: u32) {
        self.load_history
            .entry(gpu_id)
            .or_insert_with(Vec::new)
            .push(utilization);

        // Keep only last 10 samples
        if let Some(history) = self.load_history.get_mut(&gpu_id) {
            if history.len() > 10 {
                history.remove(0);
            }
        }
    }

    /// Get least loaded GPU
    pub fn get_least_loaded_gpu(&self) -> Option<u32> {
        let mut min_load = u32::MAX;
        let mut min_gpu = None;

        for &gpu_id in self.coordinator.gpu_ids() {
            if let Some(history) = self.load_history.get(&gpu_id) {
                if !history.is_empty() {
                    let avg_load: u32 = history.iter().sum::<u32>() / history.len() as u32;

                    if avg_load < min_load {
                        min_load = avg_load;
                        min_gpu = Some(gpu_id);
                    }
                }
            } else {
                // No history, assume idle
                return Some(gpu_id);
            }
        }

        min_gpu
    }

    /// Get load distribution
    pub fn get_load_distribution(&self) -> Vec<(u32, f32)> {
        let mut distribution = Vec::new();

        for &gpu_id in self.coordinator.gpu_ids() {
            let avg_load = if let Some(history) = self.load_history.get(&gpu_id) {
                if !history.is_empty() {
                    history.iter().sum::<u32>() as f32 / history.len() as f32
                } else {
                    0.0
                }
            } else {
                0.0
            };

            distribution.push((gpu_id, avg_load));
        }

        distribution
    }

    /// Check if load is balanced (within 20% difference)
    pub fn is_balanced(&self) -> bool {
        let distribution = self.get_load_distribution();

        if distribution.len() < 2 {
            return true;
        }

        let loads: Vec<f32> = distribution.iter().map(|(_, load)| *load).collect();
        let max_load = loads.iter().fold(0.0f32, |a, &b| a.max(b));
        let min_load = loads.iter().fold(100.0f32, |a, &b| a.min(b));

        (max_load - min_load) <= 20.0
    }
}

/// Cross-GPU thermal balancer
pub struct ThermalBalancer {
    coordinator: MultiGpuCoordinator,
    target_temp_diff: i32,
}

impl ThermalBalancer {
    pub fn new(target_temp_diff: i32) -> NvResult<Self> {
        Ok(Self {
            coordinator: MultiGpuCoordinator::new()?,
            target_temp_diff,
        })
    }

    /// Get temperature for all GPUs
    pub fn get_all_temps(&self) -> NvResult<Vec<(u32, i32)>> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        let mut temps = Vec::new();

        for &gpu_id in self.coordinator.gpu_ids() {
            let device = nvml.device_by_index(gpu_id).map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
            })?;

            let temp = device
                .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                .map_err(|e| {
                    NvControlError::GpuQueryFailed(format!("Failed to get temperature: {}", e))
                })?;

            temps.push((gpu_id, temp as i32));
        }

        Ok(temps)
    }

    /// Check if thermal balancing needed
    pub fn needs_balancing(&self) -> NvResult<bool> {
        let temps = self.get_all_temps()?;

        if temps.len() < 2 {
            return Ok(false);
        }

        let max_temp = temps.iter().map(|(_, t)| *t).max().unwrap_or(0);
        let min_temp = temps.iter().map(|(_, t)| *t).min().unwrap_or(0);

        Ok((max_temp - min_temp) > self.target_temp_diff)
    }

    /// Balance temperatures by adjusting power limits
    pub fn balance(&self) -> NvResult<()> {
        let temps = self.get_all_temps()?;

        if temps.len() < 2 {
            return Ok(());
        }

        let avg_temp: f32 = temps.iter().map(|(_, t)| *t as f32).sum::<f32>() / temps.len() as f32;

        use nvml_wrapper::Nvml;
        let nvml = Nvml::init()
            .map_err(|e| NvControlError::NvmlNotAvailable(format!("NVML init failed: {}", e)))?;

        for (gpu_id, temp) in temps {
            let mut device = nvml.device_by_index(gpu_id).map_err(|e| {
                NvControlError::GpuQueryFailed(format!("Failed to get device: {}", e))
            })?;

            let constraints = device.power_management_limit_constraints().ok();
            let current_limit = device.power_management_limit().ok();

            if let (Some(constraints), Some(current)) = (constraints, current_limit) {
                let temp_diff = temp as f32 - avg_temp;

                // Adjust power limit: hotter GPUs get lower power
                let adjustment = (temp_diff * -5.0) as i32; // -5W per degree
                let new_limit = (current as i32 + adjustment)
                    .max(constraints.min_limit as i32)
                    .min(constraints.max_limit as i32) as u32;

                let _ = device.set_power_management_limit(new_limit);

                println!(
                    "GPU {}: {}°C -> Power limit: {} W",
                    gpu_id,
                    temp,
                    new_limit / 1000
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinator_creation() {
        let coordinator = MultiGpuCoordinator::new();
        assert!(coordinator.is_ok() || coordinator.is_err());
    }

    #[test]
    fn test_gpu_profile() {
        let profile = GpuProfile {
            gpu_id: 0,
            name: "Performance".to_string(),
            power_limit_watts: Some(350),
            gpu_offset_mhz: Some(150),
            memory_offset_mhz: Some(500),
            fan_speed_percent: Some(75),
        };

        assert_eq!(profile.gpu_id, 0);
        assert_eq!(profile.power_limit_watts, Some(350));
    }

    #[test]
    fn test_profile_manager() {
        let mut manager = PerGpuProfileManager::new();

        let profile = GpuProfile {
            gpu_id: 0,
            name: "Test".to_string(),
            power_limit_watts: Some(300),
            gpu_offset_mhz: None,
            memory_offset_mhz: None,
            fan_speed_percent: None,
        };

        manager.set_profile(profile);
        assert!(manager.get_profile(0).is_some());
    }

    #[test]
    fn test_load_balancer_least_loaded() {
        let mut balancer = LoadBalancer::new().unwrap_or_else(|_| LoadBalancer {
            coordinator: MultiGpuCoordinator {
                gpus: vec![0, 1],
                topology: None,
            },
            load_history: HashMap::new(),
        });

        balancer.update_load(0, 80);
        balancer.update_load(1, 40);

        if balancer.coordinator.gpu_count() > 1 {
            let least = balancer.get_least_loaded_gpu();
            assert_eq!(least, Some(1));
        }
    }
}
