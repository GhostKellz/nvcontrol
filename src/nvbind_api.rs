/// Phase 5.1: nvbind Integration
///
/// Unified GPU management API between nvcontrol and nvbind
/// Container-aware monitoring for Docker, Podman, systemd-nspawn

use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

/// Container runtime type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContainerRuntime {
    Docker,
    Podman,
    SystemdNspawn,
    LXC,
    Unknown,
}

/// Container information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub runtime: ContainerRuntime,
    pub state: ContainerState,
    pub gpu_assigned: Option<u32>,
    pub gpu_utilization: Option<u32>,
    pub vram_usage_mb: Option<u64>,
    pub processes: Vec<ContainerProcess>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
    Paused,
    Restarting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerProcess {
    pub pid: u32,
    pub name: String,
    pub gpu_memory_mb: u64,
}

/// Unified GPU management API
pub struct UnifiedGpuApi {
    containers: HashMap<String, ContainerInfo>,
}

impl UnifiedGpuApi {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
        }
    }

    /// Detect all containers using GPUs
    pub fn detect_containers(&mut self) -> NvResult<Vec<ContainerInfo>> {
        let mut containers = Vec::new();

        // Detect Docker containers
        if let Ok(docker_containers) = self.detect_docker_containers() {
            containers.extend(docker_containers);
        }

        // Detect Podman containers
        if let Ok(podman_containers) = self.detect_podman_containers() {
            containers.extend(podman_containers);
        }

        // Detect systemd-nspawn containers
        if let Ok(nspawn_containers) = self.detect_nspawn_containers() {
            containers.extend(nspawn_containers);
        }

        // Update internal cache
        for container in &containers {
            self.containers.insert(container.id.clone(), container.clone());
        }

        Ok(containers)
    }

    fn detect_docker_containers(&self) -> NvResult<Vec<ContainerInfo>> {
        let output = Command::new("docker")
            .args(&["ps", "--format", "{{.ID}}|{{.Names}}|{{.State}}"])
            .output();

        if output.is_err() {
            return Ok(Vec::new()); // Docker not available
        }

        let output = output.unwrap();
        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut containers = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                let id = parts[0].to_string();
                let name = parts[1].to_string();
                let state = match parts[2] {
                    "running" => ContainerState::Running,
                    "paused" => ContainerState::Paused,
                    "restarting" => ContainerState::Restarting,
                    _ => ContainerState::Stopped,
                };

                // Check if container has GPU access
                let gpu_assigned = self.get_container_gpu_docker(&id)?;

                containers.push(ContainerInfo {
                    id: id.clone(),
                    name,
                    runtime: ContainerRuntime::Docker,
                    state,
                    gpu_assigned,
                    gpu_utilization: None,
                    vram_usage_mb: None,
                    processes: Vec::new(),
                });
            }
        }

        Ok(containers)
    }

    fn detect_podman_containers(&self) -> NvResult<Vec<ContainerInfo>> {
        let output = Command::new("podman")
            .args(&["ps", "--format", "{{.ID}}|{{.Names}}|{{.State}}"])
            .output();

        if output.is_err() {
            return Ok(Vec::new());
        }

        let output = output.unwrap();
        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut containers = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                let id = parts[0].to_string();
                let name = parts[1].to_string();
                let state = match parts[2] {
                    "running" => ContainerState::Running,
                    "paused" => ContainerState::Paused,
                    _ => ContainerState::Stopped,
                };

                let gpu_assigned = self.get_container_gpu_podman(&id)?;

                containers.push(ContainerInfo {
                    id: id.clone(),
                    name,
                    runtime: ContainerRuntime::Podman,
                    state,
                    gpu_assigned,
                    gpu_utilization: None,
                    vram_usage_mb: None,
                    processes: Vec::new(),
                });
            }
        }

        Ok(containers)
    }

    fn detect_nspawn_containers(&self) -> NvResult<Vec<ContainerInfo>> {
        let output = Command::new("machinectl")
            .args(&["list", "--no-legend"])
            .output();

        if output.is_err() {
            return Ok(Vec::new());
        }

        let output = output.unwrap();
        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut containers = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let state = if parts[2] == "running" {
                    ContainerState::Running
                } else {
                    ContainerState::Stopped
                };

                containers.push(ContainerInfo {
                    id: name.clone(),
                    name: name.clone(),
                    runtime: ContainerRuntime::SystemdNspawn,
                    state,
                    gpu_assigned: None,
                    gpu_utilization: None,
                    vram_usage_mb: None,
                    processes: Vec::new(),
                });
            }
        }

        Ok(containers)
    }

    fn get_container_gpu_docker(&self, container_id: &str) -> NvResult<Option<u32>> {
        // Check if container was started with --gpus flag
        let output = Command::new("docker")
            .args(&["inspect", container_id, "--format", "{{.HostConfig.DeviceRequests}}"])
            .output()
            .map_err(|e| {
                NvControlError::ContainerOperationFailed(format!("Docker inspect failed: {}", e))
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.contains("nvidia") || stdout.contains("gpu") {
            // Container has GPU access, try to determine which GPU
            // For now, assume GPU 0 if not specified
            Ok(Some(0))
        } else {
            Ok(None)
        }
    }

    fn get_container_gpu_podman(&self, container_id: &str) -> NvResult<Option<u32>> {
        let output = Command::new("podman")
            .args(&["inspect", container_id, "--format", "{{.HostConfig.Devices}}"])
            .output()
            .map_err(|e| {
                NvControlError::ContainerOperationFailed(format!("Podman inspect failed: {}", e))
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.contains("nvidia") {
            Ok(Some(0))
        } else {
            Ok(None)
        }
    }

    /// Get GPU utilization for container
    pub fn get_container_gpu_stats(&self, container_id: &str) -> NvResult<ContainerGpuStats> {
        let container = self.containers.get(container_id).ok_or_else(|| {
            NvControlError::ContainerOperationFailed(format!(
                "Container not found: {}",
                container_id
            ))
        })?;

        if container.gpu_assigned.is_none() {
            return Err(NvControlError::ContainerOperationFailed(
                "Container has no GPU assigned".to_string(),
            ));
        }

        // Get PIDs in container
        let pids = self.get_container_pids(container_id, container.runtime)?;

        // Query GPU stats for these PIDs
        let mut total_vram = 0u64;
        let mut processes = Vec::new();

        for pid in pids {
            if let Ok(gpu_mem) = self.get_process_gpu_memory(pid) {
                if gpu_mem > 0 {
                    total_vram += gpu_mem;

                    if let Ok(name) = self.get_process_name(pid) {
                        processes.push(ContainerProcess {
                            pid,
                            name,
                            gpu_memory_mb: gpu_mem,
                        });
                    }
                }
            }
        }

        Ok(ContainerGpuStats {
            container_id: container_id.to_string(),
            gpu_id: container.gpu_assigned.unwrap(),
            vram_usage_mb: total_vram,
            gpu_utilization: 0, // Would need nvidia-smi per-process tracking
            processes,
        })
    }

    fn get_container_pids(&self, container_id: &str, runtime: ContainerRuntime) -> NvResult<Vec<u32>> {
        let output = match runtime {
            ContainerRuntime::Docker => Command::new("docker")
                .args(&["top", container_id, "-eo", "pid"])
                .output(),
            ContainerRuntime::Podman => Command::new("podman")
                .args(&["top", container_id, "-eo", "pid"])
                .output(),
            ContainerRuntime::SystemdNspawn => Command::new("machinectl")
                .args(&["status", container_id])
                .output(),
            _ => return Ok(Vec::new()),
        };

        if output.is_err() {
            return Ok(Vec::new());
        }

        let output = output.unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut pids = Vec::new();
        for line in stdout.lines().skip(1) {
            if let Some(pid_str) = line.split_whitespace().next() {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    pids.push(pid);
                }
            }
        }

        Ok(pids)
    }

    fn get_process_gpu_memory(&self, pid: u32) -> NvResult<u64> {
        let output = Command::new("nvidia-smi")
            .args(&[
                "--query-compute-apps=pid,used_memory",
                "--format=csv,noheader,nounits",
            ])
            .output()
            .map_err(|e| {
                NvControlError::CommandFailed(format!("nvidia-smi failed: {}", e))
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                if let Ok(proc_pid) = parts[0].trim().parse::<u32>() {
                    if proc_pid == pid {
                        if let Ok(memory) = parts[1].trim().parse::<u64>() {
                            return Ok(memory);
                        }
                    }
                }
            }
        }

        Ok(0)
    }

    fn get_process_name(&self, pid: u32) -> NvResult<String> {
        let path = PathBuf::from(format!("/proc/{}/comm", pid));

        if path.exists() {
            let name = std::fs::read_to_string(path)
                .map_err(|e| NvControlError::RuntimeError(format!("Failed to read comm: {}", e)))?;
            Ok(name.trim().to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    /// List all containers with GPU access
    pub fn list_gpu_containers(&self) -> Vec<&ContainerInfo> {
        self.containers
            .values()
            .filter(|c| c.gpu_assigned.is_some())
            .collect()
    }

    /// Get container by ID
    pub fn get_container(&self, id: &str) -> Option<&ContainerInfo> {
        self.containers.get(id)
    }
}

impl Default for UnifiedGpuApi {
    fn default() -> Self {
        Self::new()
    }
}

/// Container GPU statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerGpuStats {
    pub container_id: String,
    pub gpu_id: u32,
    pub vram_usage_mb: u64,
    pub gpu_utilization: u32,
    pub processes: Vec<ContainerProcess>,
}

/// Resource allocation tracker
pub struct ResourceAllocationTracker {
    allocations: HashMap<String, GpuAllocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuAllocation {
    pub container_id: String,
    pub gpu_id: u32,
    pub vram_limit_mb: Option<u64>,
    pub compute_limit_percent: Option<u32>,
}

impl ResourceAllocationTracker {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
        }
    }

    /// Assign GPU to container
    pub fn assign_gpu(&mut self, container_id: String, gpu_id: u32, vram_limit_mb: Option<u64>) {
        self.allocations.insert(
            container_id.clone(),
            GpuAllocation {
                container_id,
                gpu_id,
                vram_limit_mb,
                compute_limit_percent: None,
            },
        );
    }

    /// Get allocation for container
    pub fn get_allocation(&self, container_id: &str) -> Option<&GpuAllocation> {
        self.allocations.get(container_id)
    }

    /// List all allocations
    pub fn list_allocations(&self) -> Vec<&GpuAllocation> {
        self.allocations.values().collect()
    }

    /// Check if allocation is within limits
    pub fn check_limits(&self, container_id: &str, current_vram_mb: u64) -> bool {
        if let Some(alloc) = self.allocations.get(container_id) {
            if let Some(limit) = alloc.vram_limit_mb {
                return current_vram_mb <= limit;
            }
        }
        true
    }
}

impl Default for ResourceAllocationTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_api_creation() {
        let api = UnifiedGpuApi::new();
        assert_eq!(api.containers.len(), 0);
    }

    #[test]
    fn test_container_detection() {
        let mut api = UnifiedGpuApi::new();

        // This will succeed or fail gracefully depending on system
        let result = api.detect_containers();
        println!("Container detection result: {:?}", result);
    }

    #[test]
    fn test_resource_tracker() {
        let mut tracker = ResourceAllocationTracker::new();

        tracker.assign_gpu("container1".to_string(), 0, Some(4096));

        assert!(tracker.get_allocation("container1").is_some());
        assert_eq!(tracker.get_allocation("container1").unwrap().gpu_id, 0);

        assert!(tracker.check_limits("container1", 2048));
        assert!(!tracker.check_limits("container1", 8192));
    }

    #[test]
    fn test_container_runtime_types() {
        let docker = ContainerRuntime::Docker;
        let podman = ContainerRuntime::Podman;

        assert_ne!(docker, podman);
    }
}
