//! UI state resources
//!
//! Track UI interaction state and layout.

#![allow(dead_code)]

use bevy::prelude::*;
use bevy::tasks::Task;
use std::path::PathBuf;

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
            sidebar_width: 260.0, // Claude/ChatGPT style
            breadcrumb_height: 40.0,
            padding: 16.0,
        }
    }
}

/// Pending folder selection from async dialog
#[derive(Resource, Debug, Default)]
pub struct PendingFolderSelection {
    /// Selected folder path (set by async dialog)
    pub path: Option<PathBuf>,
}

/// Async file dialog task
#[derive(Resource, Default)]
pub struct FileDialogTask {
    /// Running async task for folder picker
    pub task: Option<Task<Option<PathBuf>>>,
}

/// Sidebar settings (user preferences + panel state)
#[derive(Resource, Debug)]
pub struct SidebarSettings {
    /// Whether settings panel is expanded
    pub settings_open: bool,
    /// Max history entries displayed (range: 10-30)
    pub history_limit: usize,
    /// Show hidden files (dotfiles) in visualization
    pub show_hidden_files: bool,
}

impl Default for SidebarSettings {
    fn default() -> Self {
        Self {
            settings_open: false,
            history_limit: 10,
            show_hidden_files: false,
        }
    }
}
