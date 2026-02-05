//! Application state machine
//!
//! Defines the main state flow: Empty → Loading → Viewing

use bevy::prelude::*;

/// Main application state
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    /// Initial state - no folder selected
    #[default]
    Empty,
    /// Reading filesystem after folder selection
    Loading,
    /// Main visualization state
    Viewing,
}

/// Sub-state for Viewing mode
#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::Viewing)]
pub enum ViewingMode {
    /// Normal interaction - camera controllable
    #[default]
    Idle,
    /// Camera transition in progress - input blocked
    Animating,
}
