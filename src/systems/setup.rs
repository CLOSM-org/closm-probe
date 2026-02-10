//! Setup systems
//!
//! Initialize camera, lighting, and theme.

use crate::components::BackgroundStar;
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

    // Deep dark blue background
    commands.insert_resource(ClearColor(Color::srgb_u8(3, 3, 8)));

    // Ambient light for overall visibility
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.3, 0.3, 0.4),
        brightness: 200.0,
    });

    info!("Camera and lighting setup complete");
}

/// Spawn background starfield using Fibonacci sphere distribution
pub fn spawn_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const STAR_COUNT: usize = 200;
    const RADIUS: f32 = 150.0;
    const GOLDEN_RATIO: f32 = 1.618_033_9;

    for i in 0..STAR_COUNT {
        // Fibonacci sphere distribution for even spacing
        let theta = 2.0 * std::f32::consts::PI * (i as f32) / GOLDEN_RATIO;
        let phi = (1.0 - 2.0 * (i as f32 + 0.5) / STAR_COUNT as f32).acos();

        let x = RADIUS * phi.sin() * theta.cos();
        let y = RADIUS * phi.sin() * theta.sin();
        let z = RADIUS * phi.cos();

        // Size variation based on index (deterministic)
        let size = 0.03 + 0.09 * ((i * 7 + 13) % 100) as f32 / 100.0;

        // Slight color variation: white to faint blue
        let blue_shift = ((i * 11 + 3) % 100) as f32 / 100.0;
        let r = 0.8 - blue_shift * 0.1;
        let g = 0.82 - blue_shift * 0.05;
        let b = 0.9 + blue_shift * 0.1;

        // Brightness variation
        let brightness = 0.3 + 0.7 * ((i * 17 + 7) % 100) as f32 / 100.0;

        commands.spawn((
            BackgroundStar,
            Mesh3d(meshes.add(Sphere::new(size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(r * brightness, g * brightness, b * brightness, 1.0),
                emissive: LinearRgba::new(r * brightness * 2.0, g * brightness * 2.0, b * brightness * 2.0, 1.0),
                unlit: true,
                ..default()
            })),
            Transform::from_xyz(x, y, z),
        ));
    }

    info!("Spawned {} background stars", STAR_COUNT);
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
