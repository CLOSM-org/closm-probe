//! Interaction systems
//!
//! Handle user input: hover, selection, drilldown.

use crate::components::*;
use crate::events::{
    DrillDownEvent, NavigateToEvent, RespawnCelestialsEvent, SelectionChangedEvent, ViewResetEvent,
};
use crate::resources::*;
use crate::states::*;
use crate::systems::camera::CameraAnimation;
use crate::utils::window_to_viewport_cursor;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::PanOrbitCamera;

/// Track double-click timing
#[derive(Resource, Default)]
pub struct ClickState {
    pub last_click_time: Option<f64>,
    pub last_click_entity: Option<Entity>,
}

const DOUBLE_CLICK_THRESHOLD: f64 = 0.3; // 300ms

/// Handle hover detection via raycasting
pub fn update_hover(
    mut ui_state: ResMut<UiState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    celestials: Query<(Entity, &GlobalTransform, &CelestialBody), With<Clickable>>,
) {
    // No hover detection when 3D scene is hidden
    if ui_state.main_view != MainView::Universe {
        ui_state.hovered_entity = None;
        return;
    }

    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let Some(window_cursor) = window.cursor_position() else {
        ui_state.hovered_entity = None;
        return;
    };

    // Convert window coordinates to viewport coordinates
    // Returns None if cursor is over sidebar (outside viewport)
    let Some(viewport_cursor) = window_to_viewport_cursor(window_cursor, camera, window) else {
        ui_state.hovered_entity = None;
        return;
    };

    // Cast ray from cursor (using viewport-local coordinates)
    let Ok(ray) = camera.viewport_to_world(camera_transform, viewport_cursor) else {
        ui_state.hovered_entity = None;
        return;
    };

    // Find closest intersecting celestial body
    let mut closest: Option<(Entity, f32)> = None;

    for (entity, transform, _celestial) in celestials.iter() {
        let center = transform.translation();
        // Approximate radius based on transform scale
        let radius = transform.compute_transform().scale.x * 1.2; // Small margin

        // Ray-sphere intersection
        if let Some(distance) = ray_sphere_intersection(ray.origin, ray.direction, center, radius) {
            if closest.map_or(true, |(_, d)| distance < d) {
                closest = Some((entity, distance));
            }
        }
    }

    ui_state.hovered_entity = closest.map(|(e, _)| e);
}

/// Ray-sphere intersection test
fn ray_sphere_intersection(
    ray_origin: Vec3,
    ray_direction: Dir3,
    sphere_center: Vec3,
    sphere_radius: f32,
) -> Option<f32> {
    let oc = ray_origin - sphere_center;
    let a = ray_direction.dot(*ray_direction);
    let b = 2.0 * oc.dot(*ray_direction);
    let c = oc.dot(oc) - sphere_radius * sphere_radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t > 0.0 {
            Some(t)
        } else {
            None
        }
    }
}

/// Handle click/selection
pub fn handle_selection(
    mut ui_state: ResMut<UiState>,
    mut click_state: ResMut<ClickState>,
    mut selection_events: EventWriter<SelectionChangedEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    // No selection when 3D scene is hidden
    if ui_state.main_view != MainView::Universe {
        return;
    }

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let current_time = time.elapsed_secs_f64();
    let hovered = ui_state.hovered_entity;

    // Check for double-click
    if let (Some(last_time), Some(last_entity)) =
        (click_state.last_click_time, click_state.last_click_entity)
    {
        if current_time - last_time < DOUBLE_CLICK_THRESHOLD {
            if Some(last_entity) == hovered {
                // Double-click detected - will be handled by drilldown system
                click_state.last_click_time = None;
                click_state.last_click_entity = None;
                return;
            }
        }
    }

    // Single click - select
    if ui_state.selected_entity != hovered {
        ui_state.selected_entity = hovered;
        selection_events.send(SelectionChangedEvent { entity: hovered });
    }

    click_state.last_click_time = Some(current_time);
    click_state.last_click_entity = hovered;
}

/// Handle double-click drilldown
pub fn handle_drilldown(
    mut commands: Commands,
    ui_state: Res<UiState>,
    click_state: Res<ClickState>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    drillables: Query<(&CelestialBody, &GlobalTransform), With<Drillable>>,
    mut current_dir: ResMut<CurrentDirectory>,
    mut breadcrumb: ResMut<Breadcrumb>,
    mut history: ResMut<NavigationHistory>,
    mut drilldown_events: EventWriter<DrillDownEvent>,
    camera_query: Query<(Entity, &PanOrbitCamera)>,
    config: Res<CameraConfig>,
    mut next_state: ResMut<NextState<ViewingMode>>,
    celestials: Query<Entity, With<CelestialBody>>,
    asteroid_belts: Query<Entity, With<AsteroidBelt>>,
    persistent_cache: Option<Res<PersistentCache>>,
) {
    // No drilldown when 3D scene is hidden
    if ui_state.main_view != MainView::Universe {
        return;
    }

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let current_time = time.elapsed_secs_f64();

    // Check for double-click
    if let (Some(last_time), Some(last_entity)) =
        (click_state.last_click_time, click_state.last_click_entity)
    {
        if current_time - last_time < DOUBLE_CLICK_THRESHOLD {
            if ui_state.hovered_entity == Some(last_entity) {
                // Double-click on drillable entity
                if let Ok((celestial, transform)) = drillables.get(last_entity) {
                    // Push current directory to history
                    if let Some(current_path) = &current_dir.path {
                        history.push(current_path.clone());

                        // Persist history
                        if let Some(ref cache) = persistent_cache {
                            cache.write_history(&history.entries);
                        }
                    }

                    // Update current directory
                    current_dir.path = Some(celestial.path.clone());
                    *breadcrumb = Breadcrumb::from_path(&celestial.path);

                    // Send drilldown event
                    drilldown_events.send(DrillDownEvent {
                        entity: last_entity,
                        path: celestial.path.clone(),
                    });

                    // Start camera animation
                    for (cam_entity, camera) in camera_query.iter() {
                        let current_radius = camera.radius.unwrap_or(20.0);
                        commands.entity(cam_entity).insert(CameraAnimation::drilldown(
                            transform.translation(),
                            current_radius,
                            config.drilldown_duration,
                        ));
                    }
                    next_state.set(ViewingMode::Animating);

                    // Cleanup existing celestials (will respawn on animation complete)
                    for entity in celestials.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    for entity in asteroid_belts.iter() {
                        commands.entity(entity).despawn_recursive();
                    }

                    info!("Drilldown to: {}", celestial.path.display());
                }
            }
        }
    }
}

/// Handle keyboard shortcuts (runs globally — all states)
pub fn handle_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UiState>,
    mut selection_events: EventWriter<SelectionChangedEvent>,
    mut view_reset_events: EventWriter<ViewResetEvent>,
    state: Res<State<AppState>>,
) {
    // Esc: close Settings first, then clear selection
    if keyboard.just_pressed(KeyCode::Escape) {
        if ui_state.main_view == MainView::Settings {
            ui_state.main_view = MainView::Universe;
        } else if *state.get() == AppState::Viewing && ui_state.selected_entity.is_some() {
            ui_state.selected_entity = None;
            selection_events.send(SelectionChangedEvent { entity: None });
        }
    }

    // Space - reset view (Viewing + Universe only)
    if keyboard.just_pressed(KeyCode::Space)
        && *state.get() == AppState::Viewing
        && ui_state.main_view == MainView::Universe
    {
        view_reset_events.send(ViewResetEvent);
    }
}

/// Handle breadcrumb navigation
pub fn handle_navigate_to(
    mut commands: Commands,
    mut events: EventReader<NavigateToEvent>,
    mut current_dir: ResMut<CurrentDirectory>,
    mut breadcrumb: ResMut<Breadcrumb>,
    mut history: ResMut<NavigationHistory>,
    celestials: Query<Entity, With<CelestialBody>>,
    asteroid_belts: Query<Entity, With<AsteroidBelt>>,
    mut respawn_events: EventWriter<RespawnCelestialsEvent>,
    persistent_cache: Option<Res<PersistentCache>>,
    mut ui_state: ResMut<UiState>,
) {
    for event in events.read() {
        // Navigation → always return to Universe
        ui_state.main_view = MainView::Universe;
        // Push current directory to history
        if let Some(current_path) = &current_dir.path {
            history.push(current_path.clone());

            // Persist history
            if let Some(ref cache) = persistent_cache {
                cache.write_history(&history.entries);
            }
        }

        // Update current directory
        current_dir.path = Some(event.path.clone());
        *breadcrumb = Breadcrumb::from_path(&event.path);

        // Cleanup existing celestials
        for entity in celestials.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in asteroid_belts.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Trigger respawn
        respawn_events.send(RespawnCelestialsEvent);

        info!("Navigated to: {}", event.path.display());
    }
}
