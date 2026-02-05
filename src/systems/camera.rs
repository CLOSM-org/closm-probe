//! Camera systems
//!
//! Camera animation and controls.

use crate::events::{RespawnCelestialsEvent, ViewResetEvent};
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

/// Camera animation state
#[derive(Component)]
pub struct CameraAnimation {
    /// Target focus point
    pub target: Vec3,
    /// Target distance
    pub target_radius: f32,
    /// Animation progress (0.0 - 1.0)
    pub progress: f32,
    /// Animation duration in seconds
    pub duration: f32,
    /// Starting radius
    pub start_radius: f32,
    /// Starting focus
    pub start_focus: Vec3,
}

impl CameraAnimation {
    /// Create new drilldown animation
    pub fn drilldown(target: Vec3, start_radius: f32, duration: f32) -> Self {
        Self {
            target,
            target_radius: 15.0, // Closer view after drilldown
            progress: 0.0,
            duration,
            start_radius,
            start_focus: Vec3::ZERO,
        }
    }

    /// Create return animation
    pub fn return_to_center(current_radius: f32, duration: f32) -> Self {
        Self {
            target: Vec3::ZERO,
            target_radius: 20.0, // Default view distance
            progress: 0.0,
            duration,
            start_radius: current_radius,
            start_focus: Vec3::ZERO,
        }
    }
}

/// Cubic ease-out: 1 - (1-t)³
fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

/// Animate camera transitions
pub fn animate_camera(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut CameraAnimation, &mut PanOrbitCamera)>,
    mut next_state: ResMut<NextState<ViewingMode>>,
    mut respawn_events: EventWriter<RespawnCelestialsEvent>,
) {
    for (entity, mut animation, mut camera) in query.iter_mut() {
        animation.progress += time.delta_secs() / animation.duration;

        if animation.progress >= 1.0 {
            // Animation complete
            camera.radius = Some(animation.target_radius);
            camera.focus = Vec3::ZERO; // Reset focus to origin for new scene
            commands.entity(entity).remove::<CameraAnimation>();
            next_state.set(ViewingMode::Idle);

            // Trigger celestial respawn
            respawn_events.send(RespawnCelestialsEvent);
        } else {
            // Interpolate
            let t = ease_out_cubic(animation.progress);
            camera.radius = Some(animation.start_radius.lerp(animation.target_radius, t));
            camera.focus = animation.start_focus.lerp(animation.target, t);
        }
    }
}

/// Handle view reset request
pub fn handle_view_reset(
    mut commands: Commands,
    mut events: EventReader<ViewResetEvent>,
    query: Query<(Entity, &PanOrbitCamera)>,
    config: Res<CameraConfig>,
    mut next_state: ResMut<NextState<ViewingMode>>,
) {
    for _ in events.read() {
        for (entity, camera) in query.iter() {
            let current_radius = camera.radius.unwrap_or(20.0);
            commands
                .entity(entity)
                .insert(CameraAnimation::return_to_center(
                    current_radius,
                    config.reset_duration,
                ));
            next_state.set(ViewingMode::Animating);
        }
    }
}
