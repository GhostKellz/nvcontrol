use crate::{NvControlError, NvResult};
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderCacheInfo {
    pub path: PathBuf,
    pub size_mb: f64,
    pub file_count: usize,
    pub cache_type: ShaderCacheType,
    pub last_modified: Option<std::time::SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderCacheType {
    Nvidia,       // NVIDIA driver cache
    Mesa,         // Mesa/RadeonSI cache (for hybrid setups)
    Vulkan,       // Vulkan pipeline cache
    OpenGL,       // OpenGL shader cache
    Steam,        // Steam shader pre-caching
    Dxvk,         // DXVK cache
    Vkd3d,        // VKD3D cache
    Game(String), // Game-specific cache
}

impl ShaderCacheType {
    pub fn as_str(&self) -> &str {
        match self {
            ShaderCacheType::Nvidia => "NVIDIA",
            ShaderCacheType::Mesa => "Mesa",
            ShaderCacheType::Vulkan => "Vulkan",
            ShaderCacheType::OpenGL => "OpenGL",
            ShaderCacheType::Steam => "Steam",
            ShaderCacheType::Dxvk => "DXVK",
            ShaderCacheType::Vkd3d => "VKD3D",
            ShaderCacheType::Game(name) => name,
        }
    }
}

/// Get all shader cache locations and their information
pub fn get_shader_caches() -> NvResult<Vec<ShaderCacheInfo>> {
    let mut caches = Vec::new();

    if let Some(user_dirs) = UserDirs::new() {
        let home = user_dirs.home_dir();

        // NVIDIA shader cache locations
        let nvidia_cache_paths = vec![
            home.join(".nv/GLCache"),
            home.join(".cache/nvidia/GLCache"),
            PathBuf::from("/tmp/nvidia-cache"),
        ];

        for path in nvidia_cache_paths {
            if let Ok(info) = get_cache_info(&path, ShaderCacheType::Nvidia) {
                caches.push(info);
            }
        }

        // Vulkan pipeline cache
        let vulkan_cache = home.join(".cache/mesa_shader_cache");
        if let Ok(info) = get_cache_info(&vulkan_cache, ShaderCacheType::Vulkan) {
            caches.push(info);
        }

        // Steam shader cache
        let steam_cache = home.join(".steam/steam/steamapps/shadercache");
        if let Ok(info) = get_cache_info(&steam_cache, ShaderCacheType::Steam) {
            caches.push(info);
        }

        // DXVK cache
        let dxvk_cache = home.join(".cache/dxvk");
        if let Ok(info) = get_cache_info(&dxvk_cache, ShaderCacheType::Dxvk) {
            caches.push(info);
        }

        // VKD3D cache
        let vkd3d_cache = home.join(".cache/vkd3d");
        if let Ok(info) = get_cache_info(&vkd3d_cache, ShaderCacheType::Vkd3d) {
            caches.push(info);
        }

        // Game-specific caches (common locations)
        let game_cache_patterns = vec![
            (home.join(".cache"), "game cache"),
            (home.join(".local/share"), "local game data"),
        ];

        for (base_path, _desc) in game_cache_patterns {
            if let Ok(entries) = fs::read_dir(&base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                        // Check for common game shader cache indicators
                        if name.contains("shader") || name.contains("cache") {
                            if let Ok(info) =
                                get_cache_info(&path, ShaderCacheType::Game(name.to_string()))
                            {
                                caches.push(info);
                            }
                        }
                    }
                }
            }
        }
    }

    // System-wide caches
    let system_caches = vec![
        (PathBuf::from("/var/cache/nvidia"), ShaderCacheType::Nvidia),
        (PathBuf::from("/tmp/.nvidia_cache"), ShaderCacheType::Nvidia),
    ];

    for (path, cache_type) in system_caches {
        if let Ok(info) = get_cache_info(&path, cache_type) {
            caches.push(info);
        }
    }

    Ok(caches)
}

/// Get information about a specific cache directory
fn get_cache_info(path: &Path, cache_type: ShaderCacheType) -> NvResult<ShaderCacheInfo> {
    if !path.exists() {
        return Err(NvControlError::DisplayDetectionFailed(
            "Cache path does not exist".to_string(),
        ));
    }

    let metadata = fs::metadata(path).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to read cache metadata: {}", e))
    })?;

    let (size_bytes, file_count) = calculate_directory_size(path)?;
    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);

    Ok(ShaderCacheInfo {
        path: path.to_path_buf(),
        size_mb,
        file_count,
        cache_type,
        last_modified: metadata.modified().ok(),
    })
}

/// Calculate total size and file count of a directory recursively
fn calculate_directory_size(path: &Path) -> NvResult<(u64, usize)> {
    let mut total_size = 0u64;
    let mut file_count = 0usize;

    if path.is_file() {
        let metadata = fs::metadata(path).map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("Failed to read file metadata: {}", e))
        })?;
        return Ok((metadata.len(), 1));
    }

    let entries = fs::read_dir(path).map_err(|e| {
        NvControlError::DisplayDetectionFailed(format!("Failed to read directory: {}", e))
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("Failed to read directory entry: {}", e))
        })?;

        let path = entry.path();
        if path.is_dir() {
            let (dir_size, dir_files) = calculate_directory_size(&path)?;
            total_size += dir_size;
            file_count += dir_files;
        } else {
            let metadata = fs::metadata(&path).map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!(
                    "Failed to read file metadata: {}",
                    e
                ))
            })?;
            total_size += metadata.len();
            file_count += 1;
        }
    }

    Ok((total_size, file_count))
}

/// Clear all shader caches
pub fn clear_shader_cache() -> NvResult<()> {
    let caches = get_shader_caches()?;
    let mut cleared_count = 0;
    let mut total_size_cleared = 0.0;

    for cache in caches {
        match clear_cache_directory(&cache.path) {
            Ok(()) => {
                cleared_count += 1;
                total_size_cleared += cache.size_mb;
                println!(
                    "Cleared {} cache: {} ({:.1} MB)",
                    cache.cache_type.as_str(),
                    cache.path.display(),
                    cache.size_mb
                );
            }
            Err(e) => {
                eprintln!("Warning: Failed to clear {}: {}", cache.path.display(), e);
            }
        }
    }

    println!(
        "\nCleared {} shader caches, freed {:.1} MB",
        cleared_count, total_size_cleared
    );
    Ok(())
}

/// Clear specific shader cache by type
pub fn clear_shader_cache_by_type(cache_type: ShaderCacheType) -> NvResult<()> {
    let caches = get_shader_caches()?;
    let mut cleared_count = 0;
    let mut total_size_cleared = 0.0;

    for cache in caches {
        if std::mem::discriminant(&cache.cache_type) == std::mem::discriminant(&cache_type) {
            match clear_cache_directory(&cache.path) {
                Ok(()) => {
                    cleared_count += 1;
                    total_size_cleared += cache.size_mb;
                    println!(
                        "Cleared {} cache: {} ({:.1} MB)",
                        cache.cache_type.as_str(),
                        cache.path.display(),
                        cache.size_mb
                    );
                }
                Err(e) => {
                    eprintln!("Warning: Failed to clear {}: {}", cache.path.display(), e);
                }
            }
        }
    }

    if cleared_count == 0 {
        println!("No {} caches found to clear", cache_type.as_str());
    } else {
        println!(
            "Cleared {} {} caches, freed {:.1} MB",
            cleared_count,
            cache_type.as_str(),
            total_size_cleared
        );
    }

    Ok(())
}

/// Clear a specific cache directory
fn clear_cache_directory(path: &Path) -> NvResult<()> {
    if !path.exists() {
        return Ok(());
    }

    // For safety, only clear directories that look like caches
    let path_str = path.to_string_lossy().to_lowercase();
    if !path_str.contains("cache") && !path_str.contains("shader") && !path_str.contains("dxvk") {
        return Err(NvControlError::DisplayDetectionFailed(
            "Path doesn't appear to be a cache directory".to_string(),
        ));
    }

    if path.is_file() {
        fs::remove_file(path).map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("Failed to remove file: {}", e))
        })?;
    } else {
        // Remove contents but keep the directory structure
        let entries = fs::read_dir(path).map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!("Failed to read directory: {}", e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!("Failed to read entry: {}", e))
            })?;

            let entry_path = entry.path();
            if entry_path.is_dir() {
                fs::remove_dir_all(&entry_path).map_err(|e| {
                    NvControlError::DisplayDetectionFailed(format!(
                        "Failed to remove directory: {}",
                        e
                    ))
                })?;
            } else {
                fs::remove_file(&entry_path).map_err(|e| {
                    NvControlError::DisplayDetectionFailed(format!("Failed to remove file: {}", e))
                })?;
            }
        }
    }

    Ok(())
}

/// Precompile shaders for Steam games (if available)
pub fn precompile_shaders(game_path: &str) -> NvResult<()> {
    // Try to trigger Steam's shader pre-caching
    if let Some(app_id) = extract_steam_app_id(game_path) {
        println!(
            "Triggering Steam shader pre-compilation for app ID: {}",
            app_id
        );

        let output = Command::new("steam")
            .args(&["steam://preload/", &app_id])
            .output();

        match output {
            Ok(_) => println!("Steam shader pre-compilation triggered"),
            Err(_) => {
                // Fallback: try with steamcmd if available
                let output = Command::new("steamcmd")
                    .args(&["+app_update", &app_id, "validate", "+quit"])
                    .output();

                match output {
                    Ok(_) => {
                        println!("Steamcmd validation triggered (includes shader compilation)")
                    }
                    Err(_) => {
                        println!("Steam not available, manual shader compilation not possible")
                    }
                }
            }
        }
    } else {
        println!("Non-Steam game detected: {}", game_path);
        println!("Manual shader precompilation requires running the game once");
    }

    Ok(())
}

/// Extract Steam App ID from game path
fn extract_steam_app_id(game_path: &str) -> Option<String> {
    // Check if path contains steamapps structure
    if game_path.contains("steamapps") {
        // Look for appmanifest files
        let steamapps_path = if let Some(pos) = game_path.find("steamapps") {
            let steamapps_end = pos + "steamapps".len();
            &game_path[..steamapps_end]
        } else {
            return None;
        };

        // Read appmanifest files to find the app ID
        if let Ok(entries) = fs::read_dir(steamapps_path) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_name_str.starts_with("appmanifest_") && file_name_str.ends_with(".acf") {
                    if let Some(app_id) = file_name_str
                        .strip_prefix("appmanifest_")
                        .and_then(|s| s.strip_suffix(".acf"))
                    {
                        // Verify this app manifest corresponds to our game
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if content.contains(game_path.split('/').next_back().unwrap_or("")) {
                                return Some(app_id.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Optimize shader compilation settings
pub fn optimize_shader_compilation() -> NvResult<()> {
    println!("Optimizing shader compilation settings...");

    // Set NVIDIA-specific environment variables for better shader compilation
    crate::safe_env::set_vars([
        ("__GL_SHADER_DISK_CACHE", "1"),
        ("__GL_SHADER_DISK_CACHE_PATH", "/tmp/nvidia-shader-cache"),
        ("__GL_SHADER_DISK_CACHE_SIZE", "1073741824"), // 1GB
    ]);

    // Create shader cache directory with proper permissions
    let cache_dir = PathBuf::from("/tmp/nvidia-shader-cache");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).map_err(|e| {
            NvControlError::DisplayDetectionFailed(format!(
                "Failed to create cache directory: {}",
                e
            ))
        })?;

        // Set permissions for user access
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::Permissions::from_mode(0o755);
            fs::set_permissions(&cache_dir, permissions).map_err(|e| {
                NvControlError::DisplayDetectionFailed(format!("Failed to set permissions: {}", e))
            })?;
        }
    }

    println!("Shader compilation optimization applied:");
    println!("  - Disk cache enabled");
    println!("  - Cache location: /tmp/nvidia-shader-cache");
    println!("  - Cache size limit: 1GB");

    Ok(())
}

/// Get shader compilation statistics
pub fn get_shader_stats() -> NvResult<()> {
    let caches = get_shader_caches()?;
    let mut total_size = 0.0;
    let mut total_files = 0;

    println!("Shader Cache Statistics:");
    println!("========================");

    for cache in caches {
        println!(
            "{}: {:.1} MB ({} files)",
            cache.cache_type.as_str(),
            cache.size_mb,
            cache.file_count
        );
        println!("  Location: {}", cache.path.display());

        if let Some(modified) = cache.last_modified {
            if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0);
                if let Some(dt) = datetime {
                    println!("  Last Modified: {}", dt.format("%Y-%m-%d %H:%M:%S"));
                }
            }
        }
        println!();

        total_size += cache.size_mb;
        total_files += cache.file_count;
    }

    println!("Total: {:.1} MB across {} files", total_size, total_files);

    Ok(())
}
