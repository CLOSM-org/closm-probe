//! Events for inter-system communication
//!
//! Event-driven communication between systems.

#![allow(dead_code)]

use bevy::prelude::*;
use std::path::PathBuf;

/// User selected a folder via dialog
#[derive(Event, Debug)]
pub struct FolderSelectedEvent {
    pub path: PathBuf,
}

/// Navigate into a directory (drill-down)
#[derive(Event, Debug)]
pub struct DrillDownEvent {
    pub entity: Entity,
    pub path: PathBuf,
}

/// Navigate to parent directory
#[derive(Event, Debug, Default)]
pub struct DrillUpEvent;

/// Selection changed
#[derive(Event, Debug)]
pub struct SelectionChangedEvent {
    pub entity: Option<Entity>,
}

/// Navigate to specific path (breadcrumb click)
#[derive(Event, Debug)]
pub struct NavigateToEvent {
    pub path: PathBuf,
}

/// Request view reset to default position
#[derive(Event, Debug, Default)]
pub struct ViewResetEvent;

/// Request to respawn celestial bodies for current directory
#[derive(Event, Debug, Default)]
pub struct RespawnCelestialsEvent;
