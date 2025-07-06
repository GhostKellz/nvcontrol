use crate::{NvControlError, NvResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingSettings {
    pub encoder: EncoderType,
    pub quality_preset: QualityPreset,
    pub bitrate_mbps: u32,
    pub resolution: (u32, u32),
    pub framerate: u32,
    pub audio_enabled: bool,
    pub audio_bitrate_kbps: u32,
    pub output_format: OutputFormat,
    pub lossless_mode: bool,
    pub instant_replay_duration: u32, // seconds
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EncoderType {
    NvencH264,      // Hardware H.264 (most compatible)
    NvencH265,      // Hardware H.265/HEVC (better quality/size)
    NvencAv1,       // Hardware AV1 (latest, best quality) - RTX 40 series+
    SoftwareX264,   // Software fallback
    SoftwareX265,   // Software fallback
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QualityPreset {
    Lossless,       // For perfect quality recordings
    HighQuality,    // Near-lossless, good for content creation
    Balanced,       // Good quality/size ratio
    Performance,    // Lower quality, better performance
    Streaming,      // Optimized for live streaming
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Mp4,
    Mkv,
    Mov,
    Avi,
}

impl Default for RecordingSettings {
    fn default() -> Self {
        Self {
            encoder: EncoderType::NvencH265,
            quality_preset: QualityPreset::HighQuality,
            bitrate_mbps: 50,
            resolution: (1920, 1080),
            framerate: 60,
            audio_enabled: true,
            audio_bitrate_kbps: 192,
            output_format: OutputFormat::Mp4,
            lossless_mode: false,
            instant_replay_duration: 300, // 5 minutes
        }
    }
}

/// Get available NVIDIA encoding capabilities
pub fn get_nvenc_capabilities() -> NvResult<NvencCapabilities> {
    let mut capabilities = NvencCapabilities {
        h264_available: false,
        h265_available: false,
        av1_available: false,
        max_encoding_sessions: 0,
        supported_resolutions: Vec::new(),
        gpu_name: "Unknown".to_string(),
    };

    // Use NVML to check GPU capabilities
    if let Ok(nvml) = nvml_wrapper::Nvml::init() {
        if let Ok(device) = nvml.device_by_index(0) {
            if let Ok(name) = device.name() {
                capabilities.gpu_name = name.clone();
                
                // Determine encoding capabilities based on GPU generation
                if name.contains("RTX 40") || name.contains("RTX 50") {
                    // Ada Lovelace architecture - supports all modern codecs
                    capabilities.h264_available = true;
                    capabilities.h265_available = true;
                    capabilities.av1_available = true;
                    capabilities.max_encoding_sessions = 3;
                } else if name.contains("RTX 30") || name.contains("RTX 20") {
                    // Ampere/Turing - supports H.264 and H.265
                    capabilities.h264_available = true;
                    capabilities.h265_available = true;
                    capabilities.av1_available = false;
                    capabilities.max_encoding_sessions = 2;
                } else if name.contains("GTX 16") || name.contains("GTX 10") {
                    // Turing/Pascal - basic H.264 support
                    capabilities.h264_available = true;
                    capabilities.h265_available = name.contains("GTX 16"); // Only GTX 16 series
                    capabilities.av1_available = false;
                    capabilities.max_encoding_sessions = 1;
                } else {
                    // Older or unknown GPU
                    capabilities.h264_available = true; // Most NVIDIA GPUs support H.264
                    capabilities.h265_available = false;
                    capabilities.av1_available = false;
                    capabilities.max_encoding_sessions = 1;
                }
            }
        }
    }

    // Common supported resolutions for NVENC
    capabilities.supported_resolutions = vec![
        (1920, 1080),
        (2560, 1440),
        (3840, 2160),
        (1280, 720),
        (1600, 900),
    ];

    Ok(capabilities)
}

#[derive(Debug, Clone)]
pub struct NvencCapabilities {
    pub h264_available: bool,
    pub h265_available: bool,
    pub av1_available: bool,
    pub max_encoding_sessions: u32,
    pub supported_resolutions: Vec<(u32, u32)>,
    pub gpu_name: String,
}

/// Create optimized recording settings for different use cases
pub fn create_shadowplay_preset() -> RecordingSettings {
    RecordingSettings {
        encoder: EncoderType::NvencH265,
        quality_preset: QualityPreset::HighQuality,
        bitrate_mbps: 50,
        resolution: (1920, 1080),
        framerate: 60,
        audio_enabled: true,
        audio_bitrate_kbps: 192,
        output_format: OutputFormat::Mp4,
        lossless_mode: false,
        instant_replay_duration: 300,
    }
}

pub fn create_lossless_preset() -> RecordingSettings {
    RecordingSettings {
        encoder: EncoderType::NvencAv1, // Use AV1 for best lossless compression
        quality_preset: QualityPreset::Lossless,
        bitrate_mbps: 200, // Higher bitrate for lossless
        resolution: (1920, 1080),
        framerate: 60,
        audio_enabled: true,
        audio_bitrate_kbps: 320, // Higher audio quality
        output_format: OutputFormat::Mkv, // Better container for lossless
        lossless_mode: true,
        instant_replay_duration: 180, // Shorter due to large file sizes
    }
}

pub fn create_streaming_preset() -> RecordingSettings {
    RecordingSettings {
        encoder: EncoderType::NvencH264, // H.264 for maximum compatibility
        quality_preset: QualityPreset::Streaming,
        bitrate_mbps: 6, // Typical streaming bitrate
        resolution: (1920, 1080),
        framerate: 60,
        audio_enabled: true,
        audio_bitrate_kbps: 128,
        output_format: OutputFormat::Mp4,
        lossless_mode: false,
        instant_replay_duration: 60, // Short clips for streaming highlights
    }
}

pub fn create_content_creation_preset() -> RecordingSettings {
    RecordingSettings {
        encoder: EncoderType::NvencAv1, // AV1 for best quality/size ratio
        quality_preset: QualityPreset::HighQuality,
        bitrate_mbps: 100,
        resolution: (2560, 1440), // 1440p for content creation
        framerate: 60,
        audio_enabled: true,
        audio_bitrate_kbps: 256,
        output_format: OutputFormat::Mkv,
        lossless_mode: false,
        instant_replay_duration: 600, // 10 minutes for longer clips
    }
}

/// Start recording with the specified settings
pub fn start_recording(settings: &RecordingSettings, output_path: &str) -> NvResult<()> {
    // Check if we have the necessary capabilities
    let capabilities = get_nvenc_capabilities()?;
    
    if !is_encoder_supported(&settings.encoder, &capabilities) {
        return Err(NvControlError::DisplayDetectionFailed(
            format!("Encoder {:?} not supported on this GPU", settings.encoder)
        ));
    }

    // Build FFmpeg command for recording
    let mut cmd = Command::new("ffmpeg");
    
    // Input: capture display
    cmd.args(&["-f", "x11grab"]);
    cmd.args(&["-r", &settings.framerate.to_string()]);
    cmd.args(&["-s", &format!("{}x{}", settings.resolution.0, settings.resolution.1)]);
    cmd.args(&["-i", ":0.0"]);
    
    // Audio input if enabled
    if settings.audio_enabled {
        cmd.args(&["-f", "pulse"]);
        cmd.args(&["-i", "default"]);
    }
    
    // Video encoding settings
    match settings.encoder {
        EncoderType::NvencH264 => {
            cmd.args(&["-c:v", "h264_nvenc"]);
        }
        EncoderType::NvencH265 => {
            cmd.args(&["-c:v", "hevc_nvenc"]);
        }
        EncoderType::NvencAv1 => {
            cmd.args(&["-c:v", "av1_nvenc"]);
        }
        EncoderType::SoftwareX264 => {
            cmd.args(&["-c:v", "libx264"]);
        }
        EncoderType::SoftwareX265 => {
            cmd.args(&["-c:v", "libx265"]);
        }
    }
    
    // Quality settings
    match settings.quality_preset {
        QualityPreset::Lossless => {
            if settings.lossless_mode {
                cmd.args(&["-preset", "lossless"]);
            } else {
                cmd.args(&["-crf", "0"]);
            }
        }
        QualityPreset::HighQuality => {
            cmd.args(&["-preset", "slow"]);
            cmd.args(&["-crf", "18"]);
        }
        QualityPreset::Balanced => {
            cmd.args(&["-preset", "medium"]);
            cmd.args(&["-crf", "23"]);
        }
        QualityPreset::Performance => {
            cmd.args(&["-preset", "fast"]);
            cmd.args(&["-crf", "28"]);
        }
        QualityPreset::Streaming => {
            cmd.args(&["-preset", "ultrafast"]);
            cmd.args(&["-b:v", &format!("{}M", settings.bitrate_mbps)]);
        }
    }
    
    // Audio encoding
    if settings.audio_enabled {
        cmd.args(&["-c:a", "aac"]);
        cmd.args(&["-b:a", &format!("{}k", settings.audio_bitrate_kbps)]);
    }
    
    // Output format
    let extension = match settings.output_format {
        OutputFormat::Mp4 => "mp4",
        OutputFormat::Mkv => "mkv",
        OutputFormat::Mov => "mov",
        OutputFormat::Avi => "avi",
    };
    
    let full_output_path = if output_path.ends_with(&format!(".{}", extension)) {
        output_path.to_string()
    } else {
        format!("{}.{}", output_path, extension)
    };
    
    cmd.arg(&full_output_path);
    
    // Start recording
    println!("Starting recording with command: {:?}", cmd);
    
    let child = cmd.spawn()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to start recording: {}", e)))?;
    
    // Store process ID for later termination
    save_recording_pid(child.id())?;
    
    println!("Recording started successfully. Output: {}", full_output_path);
    Ok(())
}

/// Stop current recording
pub fn stop_recording() -> NvResult<()> {
    if let Some(pid) = get_recording_pid()? {
        // Send SIGTERM to gracefully stop FFmpeg
        Command::new("kill")
            .args(&["-TERM", &pid.to_string()])
            .output()
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to stop recording: {}", e)))?;
        
        // Clear stored PID
        clear_recording_pid()?;
        
        println!("Recording stopped successfully");
        Ok(())
    } else {
        Err(NvControlError::DisplayDetectionFailed("No active recording found".to_string()))
    }
}

/// Check if recording is currently active
pub fn is_recording() -> bool {
    get_recording_pid().unwrap_or(None).is_some()
}

/// Start instant replay (continuously record and keep last N seconds)
pub fn start_instant_replay(settings: &RecordingSettings) -> NvResult<()> {
    // Create a circular buffer recording that keeps only the last N seconds
    let temp_dir = get_recording_temp_dir();
    let segment_duration = 30; // 30 second segments
    let total_segments = (settings.instant_replay_duration + segment_duration - 1) / segment_duration;
    
    let mut cmd = Command::new("ffmpeg");
    
    // Input: capture display
    cmd.args(&["-f", "x11grab"]);
    cmd.args(&["-r", &settings.framerate.to_string()]);
    cmd.args(&["-s", &format!("{}x{}", settings.resolution.0, settings.resolution.1)]);
    cmd.args(&["-i", ":0.0"]);
    
    // Segmented output for circular buffer
    cmd.args(&["-f", "segment"]);
    cmd.args(&["-segment_time", &segment_duration.to_string()]);
    cmd.args(&["-segment_wrap", &total_segments.to_string()]);
    cmd.args(&["-reset_timestamps", "1"]);
    
    // Video encoding (use fast preset for instant replay)
    match settings.encoder {
        EncoderType::NvencH264 => cmd.args(&["-c:v", "h264_nvenc"]),
        EncoderType::NvencH265 => cmd.args(&["-c:v", "hevc_nvenc"]),
        _ => cmd.args(&["-c:v", "h264_nvenc"]), // Fallback to H.264
    };
    
    cmd.args(&["-preset", "fast"]);
    cmd.args(&["-b:v", &format!("{}M", settings.bitrate_mbps)]);
    
    let output_pattern = temp_dir.join("replay_%03d.mp4");
    cmd.arg(output_pattern.to_string_lossy().as_ref());
    
    println!("Starting instant replay with command: {:?}", cmd);
    
    let child = cmd.spawn()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to start instant replay: {}", e)))?;
    
    save_instant_replay_pid(child.id())?;
    
    println!("Instant replay started successfully");
    Ok(())
}

/// Save the current instant replay buffer
pub fn save_instant_replay(output_path: &str) -> NvResult<()> {
    let temp_dir = get_recording_temp_dir();
    
    // Find all replay segments
    let mut segments = Vec::new();
    for i in 0..100 {
        let segment_path = temp_dir.join(format!("replay_{:03}.mp4", i));
        if segment_path.exists() {
            segments.push(segment_path);
        }
    }
    
    if segments.is_empty() {
        return Err(NvControlError::DisplayDetectionFailed("No replay segments found".to_string()));
    }
    
    // Sort by modification time to get correct order
    segments.sort_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH));
    
    // Concatenate segments into final video
    let mut cmd = Command::new("ffmpeg");
    
    // Create input list
    let input_list = temp_dir.join("replay_list.txt");
    let list_content = segments.iter()
        .map(|p| format!("file '{}'", p.to_string_lossy()))
        .collect::<Vec<_>>()
        .join("\n");
    
    fs::write(&input_list, list_content)
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to create input list: {}", e)))?;
    
    cmd.args(&["-f", "concat"]);
    cmd.args(&["-safe", "0"]);
    cmd.args(&["-i", input_list.to_string_lossy().as_ref()]);
    cmd.args(&["-c", "copy"]); // Copy without re-encoding
    cmd.arg(output_path);
    
    cmd.output()
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to save instant replay: {}", e)))?;
    
    println!("Instant replay saved to: {}", output_path);
    Ok(())
}

// Helper functions for process management
fn save_recording_pid(pid: u32) -> NvResult<()> {
    let config_dir = get_config_dir();
    fs::create_dir_all(&config_dir)
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to create config dir: {}", e)))?;
    
    let pid_file = config_dir.join("recording.pid");
    fs::write(pid_file, pid.to_string())
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to save PID: {}", e)))?;
    
    Ok(())
}

fn get_recording_pid() -> NvResult<Option<u32>> {
    let pid_file = get_config_dir().join("recording.pid");
    
    if pid_file.exists() {
        let content = fs::read_to_string(pid_file)
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to read PID: {}", e)))?;
        
        let pid: u32 = content.trim().parse()
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Invalid PID: {}", e)))?;
        
        // Check if process is still running
        if process_exists(pid) {
            Ok(Some(pid))
        } else {
            clear_recording_pid()?;
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn clear_recording_pid() -> NvResult<()> {
    let pid_file = get_config_dir().join("recording.pid");
    if pid_file.exists() {
        fs::remove_file(pid_file)
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to clear PID: {}", e)))?;
    }
    Ok(())
}

fn save_instant_replay_pid(pid: u32) -> NvResult<()> {
    let config_dir = get_config_dir();
    let pid_file = config_dir.join("instant_replay.pid");
    fs::write(pid_file, pid.to_string())
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to save instant replay PID: {}", e)))?;
    Ok(())
}

fn process_exists(pid: u32) -> bool {
    Command::new("kill")
        .args(&["-0", &pid.to_string()])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn is_encoder_supported(encoder: &EncoderType, capabilities: &NvencCapabilities) -> bool {
    match encoder {
        EncoderType::NvencH264 => capabilities.h264_available,
        EncoderType::NvencH265 => capabilities.h265_available,
        EncoderType::NvencAv1 => capabilities.av1_available,
        EncoderType::SoftwareX264 | EncoderType::SoftwareX265 => true, // Software always available
    }
}

fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("nvcontrol")
}

fn get_recording_temp_dir() -> PathBuf {
    let temp_dir = get_config_dir().join("temp_recordings");
    fs::create_dir_all(&temp_dir).ok();
    temp_dir
}

/// Load recording presets from configuration
pub fn load_recording_presets() -> NvResult<Vec<RecordingSettings>> {
    let config_path = get_config_dir().join("recording_presets.json");
    
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to read recording config: {}", e)))?;
        
        let presets: Vec<RecordingSettings> = serde_json::from_str(&content)
            .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to parse recording config: {}", e)))?;
        
        Ok(presets)
    } else {
        // Return default presets
        Ok(vec![
            create_shadowplay_preset(),
            create_lossless_preset(),
            create_streaming_preset(),
            create_content_creation_preset(),
        ])
    }
}

/// Save recording presets to configuration
pub fn save_recording_presets(presets: &[RecordingSettings]) -> NvResult<()> {
    let config_path = get_config_dir().join("recording_presets.json");
    
    let content = serde_json::to_string_pretty(presets)
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to serialize presets: {}", e)))?;
    
    fs::write(&config_path, content)
        .map_err(|e| NvControlError::DisplayDetectionFailed(format!("Failed to save recording config: {}", e)))?;
    
    println!("Saved {} recording presets", presets.len());
    Ok(())
}
