//! nvcontrol GUI Module
//!
//! Modular architecture for the egui-based GUI application.
//! Split from the monolithic nvcontrol.rs for maintainability.

pub mod app;
pub mod icons;
pub mod state;
pub mod tabs;
pub mod theme;
pub mod toast;
pub mod widgets;

// Re-export main types
pub use app::{run, NvControlApp};
pub use state::GuiState;
pub use toast::{Toast, ToastKind, ToastManager};
