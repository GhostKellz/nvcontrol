use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModernTheme {
    pub primary: String,    // Bright cyan for NVIDIA branding
    pub secondary: String,  // Orange accent for highlights
    pub background: String, // Deep black background
    pub surface: String,    // Dark gray for cards/panels
    pub on_surface: String, // Light gray text
    pub accent: String,     // Purple for special elements
    pub success: String,    // Green for positive states
    pub warning: String,    // Amber for warnings
    pub error: String,      // Red for errors
    pub glass_alpha: f32,   // Glass morphism transparency
}

impl Default for ModernTheme {
    fn default() -> Self {
        Self {
            primary: "#00D9FF".to_string(),
            secondary: "#FF6B35".to_string(),
            background: "#0F0F0F".to_string(),
            surface: "#1A1A1A".to_string(),
            on_surface: "#E0E0E0".to_string(),
            accent: "#7C3AED".to_string(),
            success: "#10B981".to_string(),
            warning: "#F59E0B".to_string(),
            error: "#EF4444".to_string(),
            glass_alpha: 0.8,
        }
    }
}

impl ModernTheme {
    pub fn nvidia_dark() -> Self {
        Self::default()
    }

    pub fn nvidia_light() -> Self {
        Self {
            background: "#F8F9FA".to_string(),
            surface: "#FFFFFF".to_string(),
            on_surface: "#1F2937".to_string(),
            ..Self::default()
        }
    }

    pub fn gaming() -> Self {
        Self {
            primary: "#00FF41".to_string(), // Matrix green
            accent: "#FF0080".to_string(),  // Cyberpunk pink
            ..Self::default()
        }
    }
}

#[cfg(feature = "gui")]
impl ModernTheme {
    pub fn to_egui_visuals(&self) -> eframe::egui::Visuals {
        use eframe::egui::{Color32, Stroke, Visuals};

        let mut visuals = Visuals::dark();

        visuals.window_fill = Color32::from_hex(&self.surface).unwrap_or(Color32::from_gray(26));
        visuals.panel_fill = Color32::from_hex(&self.background).unwrap_or(Color32::from_gray(15));
        visuals.extreme_bg_color =
            Color32::from_hex(&self.background).unwrap_or(Color32::from_gray(15));

        // Customize widget colors
        visuals.widgets.noninteractive.bg_fill =
            Color32::from_hex(&self.surface).unwrap_or(Color32::from_gray(26));
        visuals.widgets.inactive.bg_fill =
            Color32::from_hex(&self.surface).unwrap_or(Color32::from_gray(26));
        visuals.widgets.hovered.bg_fill =
            Color32::from_hex(&self.accent).unwrap_or(Color32::from_rgb(124, 58, 237));
        visuals.widgets.active.bg_fill =
            Color32::from_hex(&self.primary).unwrap_or(Color32::from_rgb(0, 217, 255));

        // Modern stroke styles
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(
            1.0,
            Color32::from_hex(&self.on_surface).unwrap_or(Color32::from_gray(224)),
        );
        visuals.widgets.hovered.bg_stroke = Stroke::new(
            2.0,
            Color32::from_hex(&self.primary).unwrap_or(Color32::from_rgb(0, 217, 255)),
        );

        visuals
    }
}
