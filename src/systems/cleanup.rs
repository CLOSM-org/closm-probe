//! Cleanup systems
//!
//! Remove entities and reset state on state transitions.

#![allow(dead_code)]

use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

/// Clean up when exiting Viewing state
pub fn cleanup_viewing(
    mut commands: Commands,
    celestials: Query<Entity, With<CelestialBody>>,
    asteroid_belts: Query<Entity, With<AsteroidBelt>>,
    mut ui_state: ResMut<UiState>,
) {
    // Despawn all celestial bodies
    for entity in celestials.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in asteroid_belts.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Clear UI state
    ui_state.hovered_entity = None;
    ui_state.selected_entity = None;

    info!("Cleaned up viewing state");
}

/// Clean up when entering a new directory
pub fn cleanup_for_navigation(
    mut commands: Commands,
    celestials: Query<Entity, With<CelestialBody>>,
    asteroid_belts: Query<Entity, With<AsteroidBelt>>,
) {
    for entity in celestials.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in asteroid_belts.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
