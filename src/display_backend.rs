//! Display Command Backend Abstraction
//!
//! Provides a trait-based abstraction over display-related shell commands for testability.
//! Real implementation executes actual commands, mock implementation returns configurable data.
//!
//! ## Security: Command Allow-List
//!
//! The `ShellDisplayRunner` only executes binaries from an allow-list of absolute paths.
//! This prevents PATH injection attacks and ensures predictable behavior across environments.
//! See `ALLOWED_COMMANDS` for the full list.

use crate::NvControlError;
use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;

/// Allow-listed commands with their expected absolute paths.
/// Commands not in this list will be rejected with `DisplayError::CommandNotAllowed`.
///
/// Multiple paths are supported per command to handle different distro layouts.
fn get_allowed_commands() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();

    // X11 tools
    map.insert("xrandr", vec!["/usr/bin/xrandr"]);
    map.insert(
        "nvidia-settings",
        vec!["/usr/bin/nvidia-settings", "/usr/local/bin/nvidia-settings"],
    );

    // Wayland tools
    map.insert("wayland-info", vec!["/usr/bin/wayland-info"]);
    map.insert("weston-info", vec!["/usr/bin/weston-info"]);
    map.insert("wlr-randr", vec!["/usr/bin/wlr-randr"]);

    // KDE tools
    map.insert(
        "kscreen-doctor",
        vec!["/usr/bin/kscreen-doctor", "/usr/lib/kf6/bin/kscreen-doctor"],
    );
    map.insert("qdbus", vec!["/usr/bin/qdbus", "/usr/lib/qt6/bin/qdbus"]);
    map.insert("kwriteconfig5", vec!["/usr/bin/kwriteconfig5"]);
    map.insert("kwriteconfig6", vec!["/usr/bin/kwriteconfig6"]);
    map.insert("kreadconfig5", vec!["/usr/bin/kreadconfig5"]);
    map.insert("kreadconfig6", vec!["/usr/bin/kreadconfig6"]);

    // GNOME tools
    map.insert("gsettings", vec!["/usr/bin/gsettings"]);
    map.insert(
        "gnome-monitor-config",
        vec!["/usr/bin/gnome-monitor-config"],
    );

    // Hyprland tools
    map.insert("hyprctl", vec!["/usr/bin/hyprctl"]);

    // Sway tools
    map.insert("swaymsg", vec!["/usr/bin/swaymsg"]);

    // System utilities
    map.insert("which", vec!["/usr/bin/which", "/bin/which"]);
    map.insert("pgrep", vec!["/usr/bin/pgrep"]);

    map
}

/// Resolve a command name to its absolute path from the allow-list.
/// Returns `None` if the command is not allow-listed or not found at any expected path.
fn resolve_allowed_path(cmd: &str) -> Option<String> {
    let allowed = get_allowed_commands();
    if let Some(paths) = allowed.get(cmd) {
        for path in paths {
            if std::path::Path::new(path).exists() {
                return Some((*path).to_string());
            }
        }
    }
    None
}

/// Error types for display operations
#[derive(Debug, Clone)]
pub enum DisplayError {
    /// The required binary is not installed
    BinaryMissing(String),
    /// Command is not in the allow-list (security hardening)
    CommandNotAllowed(String),
    /// Permission denied for the operation
    PermissionDenied(String),
    /// Feature not supported on this system
    Unsupported(String),
    /// Command execution failed
    ExecutionFailed(String),
    /// Parse error in command output
    ParseError(String),
}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::BinaryMissing(cmd) => {
                write!(f, "Command '{}' not found. Please install it.", cmd)
            }
            DisplayError::CommandNotAllowed(cmd) => {
                write!(
                    f,
                    "Command '{}' is not in the allow-list. For security, only approved system utilities can be executed.",
                    cmd
                )
            }
            DisplayError::PermissionDenied(msg) => {
                write!(f, "Permission denied: {}. Try running with sudo.", msg)
            }
            DisplayError::Unsupported(feature) => {
                write!(f, "Feature '{}' not supported on this system.", feature)
            }
            DisplayError::ExecutionFailed(msg) => write!(f, "Command failed: {}", msg),
            DisplayError::ParseError(msg) => write!(f, "Failed to parse output: {}", msg),
        }
    }
}

impl std::error::Error for DisplayError {}

impl From<DisplayError> for NvControlError {
    fn from(err: DisplayError) -> Self {
        NvControlError::DisplayDetectionFailed(err.to_string())
    }
}

/// Result type for display operations
pub type DisplayResult<T> = Result<T, DisplayError>;

/// Display command runner trait
pub trait DisplayCommandRunner: Send + Sync {
    /// Run xrandr with given arguments
    fn run_xrandr(&self, args: &[&str]) -> DisplayResult<String>;

    /// Run nvidia-settings with given arguments
    fn run_nvidia_settings(&self, args: &[&str]) -> DisplayResult<String>;

    /// Run wayland-info or wlr-randr
    fn run_wayland_info(&self) -> DisplayResult<String>;

    /// Run wlr-randr with given arguments
    fn run_wlr_randr(&self, args: &[&str]) -> DisplayResult<String>;

    /// Check if a command is available
    fn command_available(&self, cmd: &str) -> bool;

    /// Get the display server type (X11 or Wayland)
    fn get_display_server(&self) -> DisplayResult<DisplayServer>;

    /// Run arbitrary command (for less common operations)
    fn run_command(&self, cmd: &str, args: &[&str]) -> DisplayResult<String>;

    /// Check if display runner is available (has access to display server)
    fn is_available(&self) -> bool;
}

/// Display server type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayServer {
    X11,
    Wayland,
    Unknown,
}

/// Real display command runner executing actual shell commands.
///
/// Uses an allow-list of absolute paths for security hardening.
/// Only commands in `get_allowed_commands()` can be executed.
pub struct ShellDisplayRunner;

impl ShellDisplayRunner {
    pub fn new() -> Self {
        Self
    }

    fn run(&self, cmd: &str, args: &[&str]) -> DisplayResult<String> {
        // Resolve command to absolute path from allow-list
        let abs_path = match resolve_allowed_path(cmd) {
            Some(path) => path,
            None => {
                // Check if the command is not allow-listed vs just not installed
                let allowed = get_allowed_commands();
                if allowed.contains_key(cmd) {
                    // Command is allow-listed but binary not found at expected paths
                    return Err(DisplayError::BinaryMissing(format!(
                        "{} (expected at: {})",
                        cmd,
                        allowed.get(cmd).unwrap().join(" or ")
                    )));
                } else {
                    // Command is not in the allow-list at all
                    return Err(DisplayError::CommandNotAllowed(cmd.to_string()));
                }
            }
        };

        let output = Command::new(&abs_path).args(args).output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                DisplayError::PermissionDenied(format!("{} {}", abs_path, args.join(" ")))
            } else {
                DisplayError::ExecutionFailed(format!("{}: {}", abs_path, e))
            }
        })?;

        if output.status.success() {
            String::from_utf8(output.stdout)
                .map_err(|e| DisplayError::ParseError(format!("Invalid UTF-8: {}", e)))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("permission denied") || stderr.contains("Permission denied") {
                Err(DisplayError::PermissionDenied(stderr.to_string()))
            } else {
                Err(DisplayError::ExecutionFailed(stderr.to_string()))
            }
        }
    }
}

impl Default for ShellDisplayRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayCommandRunner for ShellDisplayRunner {
    fn run_xrandr(&self, args: &[&str]) -> DisplayResult<String> {
        self.run("xrandr", args)
    }

    fn run_nvidia_settings(&self, args: &[&str]) -> DisplayResult<String> {
        self.run("nvidia-settings", args)
    }

    fn run_wayland_info(&self) -> DisplayResult<String> {
        // Try wayland-info first, fall back to weston-info
        if self.command_available("wayland-info") {
            self.run("wayland-info", &[])
        } else if self.command_available("weston-info") {
            self.run("weston-info", &[])
        } else {
            Err(DisplayError::BinaryMissing(
                "wayland-info or weston-info".to_string(),
            ))
        }
    }

    fn run_wlr_randr(&self, args: &[&str]) -> DisplayResult<String> {
        self.run("wlr-randr", args)
    }

    fn command_available(&self, cmd: &str) -> bool {
        // Only return true if command is allow-listed AND exists at an expected path
        resolve_allowed_path(cmd).is_some()
    }

    fn get_display_server(&self) -> DisplayResult<DisplayServer> {
        // Check XDG_SESSION_TYPE first
        if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
            return Ok(match session_type.to_lowercase().as_str() {
                "wayland" => DisplayServer::Wayland,
                "x11" => DisplayServer::X11,
                _ => DisplayServer::Unknown,
            });
        }

        // Fall back to checking WAYLAND_DISPLAY and DISPLAY
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            Ok(DisplayServer::Wayland)
        } else if std::env::var("DISPLAY").is_ok() {
            Ok(DisplayServer::X11)
        } else {
            Ok(DisplayServer::Unknown)
        }
    }

    fn run_command(&self, cmd: &str, args: &[&str]) -> DisplayResult<String> {
        self.run(cmd, args)
    }

    fn is_available(&self) -> bool {
        // Check if we have access to a display server
        matches!(
            self.get_display_server(),
            Ok(DisplayServer::X11) | Ok(DisplayServer::Wayland)
        )
    }
}

/// Mock display command runner for testing
#[derive(Debug, Clone)]
pub struct MockDisplayRunner {
    pub display_server: DisplayServer,
    pub available_commands: Vec<String>,
    pub xrandr_output: String,
    pub nvidia_settings_output: String,
    pub wayland_info_output: String,
    pub wlr_randr_output: String,
    /// Compositor type for HDR/VRR mocking (kde, gnome, hyprland, etc.)
    pub compositor: String,
    /// Mock outputs for compositor-specific commands
    pub command_outputs: std::collections::HashMap<String, String>,
}

impl MockDisplayRunner {
    /// Create a mock for X11 environment
    pub fn x11() -> Self {
        Self {
            display_server: DisplayServer::X11,
            available_commands: vec!["xrandr".to_string(), "nvidia-settings".to_string()],
            xrandr_output: MOCK_XRANDR_OUTPUT.to_string(),
            nvidia_settings_output: MOCK_NVIDIA_SETTINGS_OUTPUT.to_string(),
            wayland_info_output: String::new(),
            wlr_randr_output: String::new(),
            compositor: "unknown".to_string(),
            command_outputs: std::collections::HashMap::new(),
        }
    }

    /// Create a mock for Wayland environment
    pub fn wayland() -> Self {
        Self {
            display_server: DisplayServer::Wayland,
            available_commands: vec![
                "wlr-randr".to_string(),
                "wayland-info".to_string(),
                "nvidia-settings".to_string(),
            ],
            xrandr_output: String::new(),
            nvidia_settings_output: MOCK_NVIDIA_SETTINGS_OUTPUT.to_string(),
            wayland_info_output: MOCK_WAYLAND_INFO_OUTPUT.to_string(),
            wlr_randr_output: MOCK_WLR_RANDR_OUTPUT.to_string(),
            compositor: "unknown".to_string(),
            command_outputs: std::collections::HashMap::new(),
        }
    }

    /// Create a mock for KDE Plasma environment
    pub fn kde() -> Self {
        let mut mock = Self::wayland();
        mock.compositor = "kde".to_string();
        mock.available_commands.push("kscreen-doctor".to_string());
        mock.available_commands.push("qdbus".to_string());
        mock.command_outputs.insert(
            "kscreen-doctor".to_string(),
            MOCK_KSCREEN_DOCTOR_OUTPUT.to_string(),
        );
        mock
    }

    /// Create a mock for GNOME environment
    pub fn gnome() -> Self {
        let mut mock = Self::wayland();
        mock.compositor = "gnome".to_string();
        mock.available_commands.push("gsettings".to_string());
        mock.command_outputs.insert(
            "gsettings".to_string(),
            "['variable-refresh-rate']".to_string(),
        );
        mock
    }

    /// Create a mock for Hyprland environment
    pub fn hyprland() -> Self {
        let mut mock = Self::wayland();
        mock.compositor = "hyprland".to_string();
        mock.available_commands.push("hyprctl".to_string());
        mock.command_outputs.insert(
            "hyprctl".to_string(),
            MOCK_HYPRCTL_MONITORS_OUTPUT.to_string(),
        );
        mock
    }

    /// Create a mock with no display tools
    pub fn headless() -> Self {
        Self {
            display_server: DisplayServer::Unknown,
            available_commands: vec![],
            xrandr_output: String::new(),
            nvidia_settings_output: String::new(),
            wayland_info_output: String::new(),
            wlr_randr_output: String::new(),
            compositor: "unknown".to_string(),
            command_outputs: std::collections::HashMap::new(),
        }
    }

    /// Set custom xrandr output
    pub fn with_xrandr_output(mut self, output: &str) -> Self {
        self.xrandr_output = output.to_string();
        self
    }

    /// Set custom nvidia-settings output
    pub fn with_nvidia_settings_output(mut self, output: &str) -> Self {
        self.nvidia_settings_output = output.to_string();
        self
    }

    /// Set compositor type
    pub fn with_compositor(mut self, compositor: &str) -> Self {
        self.compositor = compositor.to_string();
        self
    }

    /// Set custom command output
    pub fn with_command_output(mut self, cmd: &str, output: &str) -> Self {
        self.available_commands.push(cmd.to_string());
        self.command_outputs
            .insert(cmd.to_string(), output.to_string());
        self
    }

    /// Get compositor type
    pub fn get_compositor(&self) -> &str {
        &self.compositor
    }
}

impl DisplayCommandRunner for MockDisplayRunner {
    fn run_xrandr(&self, _args: &[&str]) -> DisplayResult<String> {
        if !self.available_commands.contains(&"xrandr".to_string()) {
            return Err(DisplayError::BinaryMissing("xrandr".to_string()));
        }
        Ok(self.xrandr_output.clone())
    }

    fn run_nvidia_settings(&self, _args: &[&str]) -> DisplayResult<String> {
        if !self
            .available_commands
            .contains(&"nvidia-settings".to_string())
        {
            return Err(DisplayError::BinaryMissing("nvidia-settings".to_string()));
        }
        Ok(self.nvidia_settings_output.clone())
    }

    fn run_wayland_info(&self) -> DisplayResult<String> {
        if !self
            .available_commands
            .contains(&"wayland-info".to_string())
        {
            return Err(DisplayError::BinaryMissing("wayland-info".to_string()));
        }
        Ok(self.wayland_info_output.clone())
    }

    fn run_wlr_randr(&self, _args: &[&str]) -> DisplayResult<String> {
        if !self.available_commands.contains(&"wlr-randr".to_string()) {
            return Err(DisplayError::BinaryMissing("wlr-randr".to_string()));
        }
        Ok(self.wlr_randr_output.clone())
    }

    fn command_available(&self, cmd: &str) -> bool {
        self.available_commands.contains(&cmd.to_string())
    }

    fn get_display_server(&self) -> DisplayResult<DisplayServer> {
        Ok(self.display_server)
    }

    fn run_command(&self, cmd: &str, _args: &[&str]) -> DisplayResult<String> {
        if !self.command_available(cmd) {
            return Err(DisplayError::BinaryMissing(cmd.to_string()));
        }
        // Return stored output if available, otherwise empty
        Ok(self.command_outputs.get(cmd).cloned().unwrap_or_default())
    }

    fn is_available(&self) -> bool {
        // Mock is available if we have a known display server
        matches!(
            self.display_server,
            DisplayServer::X11 | DisplayServer::Wayland
        )
    }
}

// Mock output constants
const MOCK_XRANDR_OUTPUT: &str = r#"Screen 0: minimum 8 x 8, current 2560 x 1440, maximum 32767 x 32767
DP-0 connected primary 2560x1440+0+0 (normal left inverted right x axis y axis) 597mm x 336mm
   2560x1440     59.95*+ 143.91   119.88
   1920x1080    119.88    60.00
HDMI-0 disconnected (normal left inverted right x axis y axis)
"#;

const MOCK_NVIDIA_SETTINGS_OUTPUT: &str = r#"Attribute 'DigitalVibrance' (host:0[DPY:DP-0]): 0.
  Valid values for 'DigitalVibrance' are integers in the range [-1024, 1023].
"#;

const MOCK_WAYLAND_INFO_OUTPUT: &str = r#"interface: 'wl_output', version: 4, name: 1
	x: 0, y: 0, scale: 1,
	physical_width: 597 mm, physical_height: 336 mm,
	make: 'LG Electronics', model: '27GP850-B',
	subpixel_orientation: unknown, output_transform: normal,
	mode:
		width: 2560 px, height: 1440 px, refresh: 143.912 Hz,
		flags: current preferred
"#;

const MOCK_WLR_RANDR_OUTPUT: &str = r#"DP-1 "LG Electronics 27GP850-B"
  Enabled: yes
  Modes:
    2560x1440 px, 143.912000 Hz (preferred, current)
    2560x1440 px, 119.880000 Hz
    1920x1080 px, 60.000000 Hz
  Position: 0,0
  Transform: normal
  Scale: 1.000000
  Adaptive Sync: disabled
"#;

const MOCK_KSCREEN_DOCTOR_OUTPUT: &str = r#"{
  "outputs": [
    {
      "name": "DP-1",
      "connected": true,
      "enabled": true,
      "hdr": false,
      "vrrPolicy": 0,
      "modes": [
        {"width": 2560, "height": 1440, "refreshRate": 165.0},
        {"width": 2560, "height": 1440, "refreshRate": 144.0},
        {"width": 1920, "height": 1080, "refreshRate": 60.0}
      ]
    }
  ]
}"#;

const MOCK_HYPRCTL_MONITORS_OUTPUT: &str = r#"[
  {
    "id": 0,
    "name": "DP-1",
    "width": 2560,
    "height": 1440,
    "refreshRate": 165.0,
    "vrr": false,
    "disabled": false,
    "currentFormat": "XRGB8888",
    "availableModes": ["2560x1440@165.00Hz", "2560x1440@144.00Hz", "1920x1080@60.00Hz"]
  }
]"#;

/// Shared backend type for use across modules
pub type SharedDisplayRunner = Arc<dyn DisplayCommandRunner>;

/// Create a shared real display runner
pub fn create_real_runner() -> SharedDisplayRunner {
    Arc::new(ShellDisplayRunner::new())
}

/// Create a shared mock display runner for testing
pub fn create_mock_runner_x11() -> SharedDisplayRunner {
    Arc::new(MockDisplayRunner::x11())
}

/// Create a shared mock display runner for Wayland testing
pub fn create_mock_runner_wayland() -> SharedDisplayRunner {
    Arc::new(MockDisplayRunner::wayland())
}

/// Create a shared mock display runner for KDE Plasma testing
pub fn create_mock_runner_kde() -> SharedDisplayRunner {
    Arc::new(MockDisplayRunner::kde())
}

/// Create a shared mock display runner for GNOME testing
pub fn create_mock_runner_gnome() -> SharedDisplayRunner {
    Arc::new(MockDisplayRunner::gnome())
}

/// Create a shared mock display runner for Hyprland testing
pub fn create_mock_runner_hyprland() -> SharedDisplayRunner {
    Arc::new(MockDisplayRunner::hyprland())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_x11() {
        let runner = MockDisplayRunner::x11();

        assert_eq!(runner.get_display_server().unwrap(), DisplayServer::X11);
        assert!(runner.command_available("xrandr"));
        assert!(runner.command_available("nvidia-settings"));
        assert!(!runner.command_available("wlr-randr"));

        let output = runner.run_xrandr(&[]).unwrap();
        assert!(output.contains("2560x1440"));
    }

    #[test]
    fn test_mock_wayland() {
        let runner = MockDisplayRunner::wayland();

        assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Wayland);
        assert!(runner.command_available("wlr-randr"));
        assert!(runner.command_available("wayland-info"));
        assert!(!runner.command_available("xrandr"));

        let output = runner.run_wayland_info().unwrap();
        assert!(output.contains("wl_output"));
    }

    #[test]
    fn test_mock_headless() {
        let runner = MockDisplayRunner::headless();

        assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Unknown);
        assert!(!runner.command_available("xrandr"));
        assert!(!runner.command_available("wlr-randr"));

        assert!(matches!(
            runner.run_xrandr(&[]),
            Err(DisplayError::BinaryMissing(_))
        ));
    }

    #[test]
    fn test_custom_output() {
        let runner = MockDisplayRunner::x11().with_xrandr_output("Custom output here");

        let output = runner.run_xrandr(&[]).unwrap();
        assert_eq!(output, "Custom output here");
    }

    #[test]
    fn test_display_error_formatting() {
        let err = DisplayError::BinaryMissing("foo".to_string());
        assert!(err.to_string().contains("foo"));
        assert!(err.to_string().contains("not found"));

        let err = DisplayError::PermissionDenied("test".to_string());
        assert!(err.to_string().contains("Permission"));

        let err = DisplayError::Unsupported("HDR".to_string());
        assert!(err.to_string().contains("HDR"));
    }

    #[test]
    fn test_real_runner_creation() {
        // Just test that it doesn't panic
        let runner = ShellDisplayRunner::new();
        let _ = runner.get_display_server();
    }

    #[test]
    fn test_shared_runner() {
        let runner = create_mock_runner_x11();

        // Test that it can be cloned and used from multiple places
        let runner2 = Arc::clone(&runner);
        assert_eq!(
            runner.get_display_server().unwrap(),
            runner2.get_display_server().unwrap()
        );
    }

    #[test]
    fn test_mock_kde() {
        let runner = MockDisplayRunner::kde();

        assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Wayland);
        assert!(runner.command_available("kscreen-doctor"));
        assert!(runner.command_available("qdbus"));
        assert_eq!(runner.get_compositor(), "kde");

        let output = runner.run_command("kscreen-doctor", &["-j"]).unwrap();
        assert!(output.contains("DP-1"));
        assert!(output.contains("vrrPolicy"));
    }

    #[test]
    fn test_mock_gnome() {
        let runner = MockDisplayRunner::gnome();

        assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Wayland);
        assert!(runner.command_available("gsettings"));
        assert_eq!(runner.get_compositor(), "gnome");

        let output = runner.run_command("gsettings", &[]).unwrap();
        assert!(output.contains("variable-refresh-rate"));
    }

    #[test]
    fn test_mock_hyprland() {
        let runner = MockDisplayRunner::hyprland();

        assert_eq!(runner.get_display_server().unwrap(), DisplayServer::Wayland);
        assert!(runner.command_available("hyprctl"));
        assert_eq!(runner.get_compositor(), "hyprland");

        let output = runner.run_command("hyprctl", &["monitors", "-j"]).unwrap();
        assert!(output.contains("DP-1"));
        assert!(output.contains("refreshRate"));
    }

    #[test]
    fn test_mock_with_custom_command() {
        let runner = MockDisplayRunner::x11()
            .with_command_output("my-custom-cmd", "custom output")
            .with_compositor("custom");

        assert!(runner.command_available("my-custom-cmd"));
        assert_eq!(runner.get_compositor(), "custom");

        let output = runner.run_command("my-custom-cmd", &[]).unwrap();
        assert_eq!(output, "custom output");
    }

    #[test]
    fn test_allow_list_contains_expected_commands() {
        let allowed = get_allowed_commands();

        // X11 tools
        assert!(allowed.contains_key("xrandr"));
        assert!(allowed.contains_key("nvidia-settings"));

        // Wayland tools
        assert!(allowed.contains_key("wayland-info"));
        assert!(allowed.contains_key("wlr-randr"));

        // KDE tools
        assert!(allowed.contains_key("kscreen-doctor"));
        assert!(allowed.contains_key("qdbus"));

        // GNOME tools
        assert!(allowed.contains_key("gsettings"));

        // Hyprland tools
        assert!(allowed.contains_key("hyprctl"));

        // Sway tools
        assert!(allowed.contains_key("swaymsg"));
    }

    #[test]
    fn test_allow_list_paths_are_absolute() {
        let allowed = get_allowed_commands();

        for (cmd, paths) in allowed.iter() {
            for path in paths {
                assert!(
                    path.starts_with('/'),
                    "Path for '{}' should be absolute: {}",
                    cmd,
                    path
                );
            }
        }
    }

    #[test]
    fn test_command_not_allowed_error() {
        let err = DisplayError::CommandNotAllowed("malicious-script".to_string());
        let msg = err.to_string();
        assert!(msg.contains("malicious-script"));
        assert!(msg.contains("allow-list"));
    }

    #[test]
    fn test_resolve_allowed_path_unknown_command() {
        // Commands not in allow-list should return None
        assert!(resolve_allowed_path("rm").is_none());
        assert!(resolve_allowed_path("wget").is_none());
        assert!(resolve_allowed_path("curl").is_none());
        assert!(resolve_allowed_path("bash").is_none());
    }

    #[test]
    fn test_shell_runner_rejects_unknown_commands() {
        let runner = ShellDisplayRunner::new();

        // These commands should never be allowed
        assert!(!runner.command_available("rm"));
        assert!(!runner.command_available("wget"));
        assert!(!runner.command_available("bash"));

        // run_command should fail for unknown commands
        let result = runner.run_command("malicious-script", &[]);
        assert!(matches!(result, Err(DisplayError::CommandNotAllowed(_))));
    }
}
