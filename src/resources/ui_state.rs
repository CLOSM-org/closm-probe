//! UI state resources
//!
//! Track UI interaction state and layout.

use bevy::prelude::*;

/// UI interaction state
#[derive(Resource, Debug, Default)]
pub struct UiState {
    /// Currently hovered entity
    pub hovered_entity: Option<Entity>,
    /// Currently selected entity
    pub selected_entity: Option<Entity>,
    /// Sidebar visibility
    pub sidebar_open: bool,
    /// Show startup screen
    pub show_startup: bool,
}

/// UI layout dimensions
#[derive(Resource, Debug)]
pub struct UiLayout {
    /// Sidebar width in pixels
    pub sidebar_width: f32,
    /// Breadcrumb bar height
    pub breadcrumb_height: f32,
    /// Padding for UI elements
    pub padding: f32,
}

impl Default for UiLayout {
    fn default() -> Self {
        Self {
            sidebar_width: 280.0,
            breadcrumb_height: 40.0,
            padding: 16.0,
        }
    }
}

/// Pending folder selection from async dialog
#[derive(Resource, Debug, Default)]
pub struct PendingFolderSelection {
    /// Selected folder path (set by async dialog)
    pub path: Option<std::path::PathBuf>,
}
