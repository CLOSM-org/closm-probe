//! Interaction components
//!
//! Components for user interaction with celestial bodies.

use bevy::prelude::*;

/// Marker for entities that can be clicked
#[derive(Component, Debug, Default)]
pub struct Clickable;

/// Marker for entities that support drill-down (directories only)
#[derive(Component, Debug, Default)]
pub struct Drillable;

/// Marker for entity currently under mouse cursor
#[derive(Component, Debug, Default)]
pub struct Hovered;

/// Marker for entity currently selected by user
#[derive(Component, Debug, Default)]
pub struct Selected;
