//! Setup systems
//!
//! Initialize camera, lighting, and theme.

use crate::resources::*;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

/// Setup camera and basic lighting
pub fn setup_camera(mut commands: Commands, config: Res<CameraConfig>) {
    // Spawn camera with orbit controls
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera {
            radius: Some(20.0),
            pitch: Some(0.4), // ~23 degrees
            yaw: Some(0.0),
            pitch_lower_limit: Some(-config.pitch_limit.to_radians()),
            pitch_upper_limit: Some(config.pitch_limit.to_radians()),
            zoom_lower_limit: config.zoom_min,
            zoom_upper_limit: Some(config.zoom_max),
            ..default()
        },
    ));

    // Ambient light for overall visibility
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.4),
        brightness: 200.0,
    });

    info!("Camera and lighting setup complete");
}

/// Detect OS theme and setup theme config
pub fn setup_theme(mut commands: Commands) {
    // Use dark-light crate to detect OS theme
    let dark_mode = match dark_light::detect() {
        dark_light::Mode::Dark => true,
        dark_light::Mode::Light => false,
        dark_light::Mode::Default => true, // Default to dark for space theme
    };

    let mut theme = ThemeConfig::default();
    theme.dark_mode = dark_mode;
    theme.apply_mode();

    commands.insert_resource(theme);

    info!("Theme detected: {}", if dark_mode { "dark" } else { "light" });
}
