//! Platform-specific directory size calculation
//!
//! - macOS: Uses `du` command for fast calculation (with pure Rust fallback)
//! - Other platforms: Uses jwalk crate for parallel traversal

mod macos_du; // macOS implementation (du command)

#[cfg(not(target_os = "macos"))]
mod jwalk_calc;

use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::path::PathBuf;

use crate::components::{CelestialBody, Planet, PendingSizeCalculation, PulseAnimation};
use crate::resources::VisualConfig;
use crate::utils::calculate_size;

/// Result of a size calculation
#[derive(Debug, Clone)]
pub struct SizeResult {
    pub path: PathBuf,
    pub size: u64,
}

/// Channel for receiving size calculation results
#[derive(Resource)]
pub struct SizeCalculationChannel {
    pub sender: Sender<SizeResult>,
    pub receiver: Receiver<SizeResult>,
}

impl Default for SizeCalculationChannel {
    fn default() -> Self {
        let (sender, receiver) = bounded(100);
        Self { sender, receiver }
    }
}

/// Spawn background size calculations for directory entries
pub fn spawn_size_calculations(paths: Vec<PathBuf>, sender: Sender<SizeResult>) {
    if paths.is_empty() {
        return;
    }

    #[cfg(target_os = "macos")]
    macos_du::spawn_calculations(paths, sender);

    #[cfg(not(target_os = "macos"))]
    jwalk_calc::spawn_calculations(paths, sender);
}

/// System: Update celestial body sizes from background calculations
pub fn update_celestial_sizes(
    mut commands: Commands,
    channel: Res<SizeCalculationChannel>,
    mut celestials: Query<
        (Entity, &mut CelestialBody, &mut Transform, &Planet),
        With<PendingSizeCalculation>,
    >,
    config: Res<VisualConfig>,
) {
    // Process all available results (non-blocking)
    while let Ok(result) = channel.receiver.try_recv() {
        info!(
            "Received size result: {} = {} bytes",
            result.path.display(),
            result.size
        );
        for (entity, mut body, mut transform, planet) in celestials.iter_mut() {
            if body.path == result.path && planet.is_directory {
                // Update size
                body.size_bytes = result.size;

                // Recalculate visual size
                let new_size = calculate_size(result.size, true, &config);
                transform.scale = Vec3::splat(new_size);

                // Stop animation
                commands
                    .entity(entity)
                    .remove::<PulseAnimation>()
                    .remove::<PendingSizeCalculation>();

                info!(
                    "Size calculated: {} = {} bytes",
                    body.name, result.size
                );

                break;
            }
        }
    }
}

/// System: Animate pulsing effect for pending calculations
pub fn animate_pulse(
    time: Res<Time>,
    mut query: Query<(&mut PulseAnimation, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut pulse, material_handle) in query.iter_mut() {
        pulse.elapsed += time.delta_secs();

        // Sin wave for pulsing effect
        let t = (pulse.elapsed * pulse.frequency * std::f32::consts::TAU).sin();
        let alpha = pulse.min_alpha + (pulse.max_alpha - pulse.min_alpha) * (t * 0.5 + 0.5);

        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color.set_alpha(alpha);
        }
    }
}
