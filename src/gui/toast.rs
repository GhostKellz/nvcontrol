//! Toast/Snackbar Notification System
//!
//! Provides non-blocking notifications for user feedback.
//! Replaces println! calls with visible UI notifications.

use eframe::egui;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Toast notification severity/type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    /// Success message (green)
    Success,
    /// Informational message (blue)
    Info,
    /// Warning message (yellow)
    Warning,
    /// Error message (red)
    Error,
}

impl ToastKind {
    /// Get the icon for this toast kind
    pub fn icon(&self) -> &'static str {
        match self {
            ToastKind::Success => super::icons::OK,
            ToastKind::Info => super::icons::INFO,
            ToastKind::Warning => super::icons::WARN,
            ToastKind::Error => super::icons::ERR,
        }
    }

    /// Get the color for this toast kind
    pub fn color(&self, colors: &crate::themes::ColorPalette) -> egui::Color32 {
        match self {
            ToastKind::Success => colors.green.to_egui(),
            ToastKind::Info => colors.blue.to_egui(),
            ToastKind::Warning => colors.yellow.to_egui(),
            ToastKind::Error => colors.red.to_egui(),
        }
    }
}

/// A single toast notification
#[derive(Debug, Clone)]
pub struct Toast {
    /// The message to display
    pub message: String,
    /// The kind of toast
    pub kind: ToastKind,
    /// When the toast was created
    pub created_at: Instant,
    /// How long to display the toast
    pub duration: Duration,
    /// Optional action button text
    pub action: Option<String>,
}

impl Toast {
    /// Create a new toast with default duration (4 seconds)
    pub fn new(kind: ToastKind, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind,
            created_at: Instant::now(),
            duration: Duration::from_secs(4),
            action: None,
        }
    }

    /// Create a success toast
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(ToastKind::Success, message)
    }

    /// Create an info toast
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(ToastKind::Info, message)
    }

    /// Create a warning toast
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(ToastKind::Warning, message)
    }

    /// Create an error toast
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(ToastKind::Error, message)
    }

    /// Set a custom duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Add an action button
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Check if the toast has expired
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration
    }

    /// Get remaining time as a fraction (1.0 = full, 0.0 = expired)
    pub fn remaining_fraction(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration.as_secs_f32();
        (1.0 - elapsed / total).max(0.0)
    }
}

/// Manages a queue of toast notifications
#[derive(Debug, Default)]
pub struct ToastManager {
    /// Queue of active toasts
    toasts: VecDeque<Toast>,
    /// Maximum number of visible toasts
    max_visible: usize,
}

impl ToastManager {
    /// Create a new toast manager
    pub fn new() -> Self {
        Self {
            toasts: VecDeque::new(),
            max_visible: 5,
        }
    }

    /// Add a toast to the queue
    pub fn push(&mut self, toast: Toast) {
        self.toasts.push_back(toast);
        // Limit queue size
        while self.toasts.len() > 20 {
            self.toasts.pop_front();
        }
    }

    /// Add a success toast
    pub fn success(&mut self, message: impl Into<String>) {
        self.push(Toast::success(message));
    }

    /// Add an info toast
    pub fn info(&mut self, message: impl Into<String>) {
        self.push(Toast::info(message));
    }

    /// Add a warning toast
    pub fn warning(&mut self, message: impl Into<String>) {
        self.push(Toast::warning(message));
    }

    /// Add an error toast
    pub fn error(&mut self, message: impl Into<String>) {
        self.push(Toast::error(message));
    }

    /// Remove expired toasts
    pub fn cleanup(&mut self) {
        self.toasts.retain(|t| !t.is_expired());
    }

    /// Render all visible toasts
    /// Returns true if any action button was clicked (with the toast index)
    pub fn show(&mut self, ctx: &egui::Context, colors: &crate::themes::ColorPalette) -> Option<usize> {
        self.cleanup();

        let mut clicked_action = None;
        let mut to_remove = Vec::new();

        // Show toasts from bottom-right corner
        let screen_rect = ctx.screen_rect();
        let toast_width = 350.0;
        let toast_height = 60.0;
        let margin = 16.0;
        let spacing = 8.0;

        for (i, toast) in self.toasts.iter().enumerate().take(self.max_visible) {
            let y_offset = (toast_height + spacing) * (i as f32);
            let rect = egui::Rect::from_min_size(
                egui::pos2(
                    screen_rect.max.x - toast_width - margin,
                    screen_rect.max.y - toast_height - margin - y_offset,
                ),
                egui::vec2(toast_width, toast_height),
            );

            // Calculate fade based on remaining time
            let alpha = if toast.remaining_fraction() < 0.2 {
                (toast.remaining_fraction() / 0.2 * 255.0) as u8
            } else {
                255
            };

            let toast_color = toast.kind.color(colors);
            let bg_color = egui::Color32::from_rgba_unmultiplied(
                colors.bg_highlight.r,
                colors.bg_highlight.g,
                colors.bg_highlight.b,
                alpha,
            );
            let border_color = egui::Color32::from_rgba_unmultiplied(
                toast_color.r(),
                toast_color.g(),
                toast_color.b(),
                alpha,
            );

            egui::Area::new(egui::Id::new(format!("toast_{}", i)))
                .fixed_pos(rect.min)
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .fill(bg_color)
                        .stroke(egui::Stroke::new(2.0, border_color))
                        .rounding(8.0)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.set_min_size(egui::vec2(toast_width - 24.0, toast_height - 24.0));
                            ui.horizontal(|ui| {
                                // Icon
                                ui.label(
                                    egui::RichText::new(toast.kind.icon())
                                        .size(20.0)
                                        .color(toast_color),
                                );
                                ui.add_space(8.0);

                                // Message
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(&toast.message)
                                            .color(egui::Color32::from_rgba_unmultiplied(
                                                colors.fg.r,
                                                colors.fg.g,
                                                colors.fg.b,
                                                alpha,
                                            )),
                                    );

                                    // Progress bar
                                    let progress_rect = ui.available_rect_before_wrap();
                                    let progress_width = progress_rect.width() * toast.remaining_fraction();
                                    ui.painter().rect_filled(
                                        egui::Rect::from_min_size(
                                            progress_rect.min,
                                            egui::vec2(progress_width, 2.0),
                                        ),
                                        1.0,
                                        border_color,
                                    );
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    // Close button
                                    if ui
                                        .add(egui::Button::new("×").small())
                                        .clicked()
                                    {
                                        to_remove.push(i);
                                    }

                                    // Action button if present
                                    if let Some(ref action) = toast.action {
                                        if ui.small_button(action).clicked() {
                                            clicked_action = Some(i);
                                            to_remove.push(i);
                                        }
                                    }
                                });
                            });
                        });
                });
        }

        // Remove clicked/closed toasts
        for i in to_remove.into_iter().rev() {
            self.toasts.remove(i);
        }

        // Request repaint if there are active toasts (for animations)
        if !self.toasts.is_empty() {
            ctx.request_repaint();
        }

        clicked_action
    }

    /// Check if there are any active toasts
    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    /// Clear all toasts
    pub fn clear(&mut self) {
        self.toasts.clear();
    }
}
