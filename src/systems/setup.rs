//! Setup systems
//!
//! Initialize camera, lighting, and theme.

use crate::components::BackgroundStar;
use crate::resources::*;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy_panorbit_camera::PanOrbitCamera;

/// Setup camera and basic lighting
pub fn setup_camera(mut commands: Commands, config: Res<CameraConfig>) {
    // Spawn camera with orbit controls, HDR + Bloom for star glow
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
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

/// Spawn background starfield as a single mesh with per-vertex colors
pub fn spawn_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const STAR_COUNT: usize = 300;
    const RADIUS: f32 = 150.0;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(STAR_COUNT * 4);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(STAR_COUNT * 4);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(STAR_COUNT * 4);
    let mut indices: Vec<u32> = Vec::with_capacity(STAR_COUNT * 6);

    for i in 0..STAR_COUNT {
        let seed = i as u32;

        // Random spherical distribution (natural, non-uniform look)
        let theta = std::f32::consts::TAU * hash_f32(seed, 0);
        let cos_phi = 1.0 - 2.0 * hash_f32(seed, 1);
        let sin_phi = (1.0 - cos_phi * cos_phi).sqrt();

        let center = Vec3::new(
            RADIUS * sin_phi * theta.cos(),
            RADIUS * sin_phi * theta.sin(),
            RADIUS * cos_phi,
        );

        // ~10% bright stars
        let is_bright = hash_u32(seed, 2) % 10 == 0;

        let size = if is_bright {
            0.5 + 0.5 * hash_f32(seed, 3)
        } else {
            0.15 + 0.25 * hash_f32(seed, 3)
        };

        // Color: white to faint blue/warm
        let tint = hash_f32(seed, 4);
        let (r, g, b) = if tint < 0.6 {
            // White-blue (majority)
            (0.85 - tint * 0.2, 0.85 - tint * 0.05, 0.95)
        } else if tint < 0.85 {
            // Warm white
            (0.95, 0.88, 0.8)
        } else {
            // Faint yellow
            (0.95, 0.92, 0.7)
        };

        let brightness = if is_bright {
            0.8 + 0.2 * hash_f32(seed, 5)
        } else {
            0.35 + 0.5 * hash_f32(seed, 5)
        };

        let color = [r * brightness, g * brightness, b * brightness, 1.0];

        // Quad facing origin
        let normal = center.normalize();
        let up = if normal.y.abs() > 0.99 { Vec3::X } else { Vec3::Y };
        let tangent = normal.cross(up).normalize();
        let bitangent = normal.cross(tangent).normalize();
        let half = size * 0.5;

        let base = (i as u32) * 4;
        for &offset in &[
            -tangent - bitangent,
            tangent - bitangent,
            tangent + bitangent,
            -tangent + bitangent,
        ] {
            let pos = center + offset * half;
            positions.push(pos.into());
            normals.push(normal.into());
            colors.push(color);
        }
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    commands.spawn((
        BackgroundStar,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::WHITE,
            double_sided: true,
            cull_mode: None,
            ..default()
        })),
        Transform::default(),
    ));

    info!("Spawned starfield ({STAR_COUNT} stars, single mesh)");
}

/// Deterministic hash → u32
fn hash_u32(seed: u32, offset: u32) -> u32 {
    let mut h = seed.wrapping_mul(2654435761).wrapping_add(offset.wrapping_mul(2246822519));
    h = (h ^ (h >> 16)).wrapping_mul(0x45d9f3b);
    h = (h ^ (h >> 16)).wrapping_mul(0x45d9f3b);
    h ^ (h >> 16)
}

/// Deterministic hash → f32 in [0.0, 1.0)
fn hash_f32(seed: u32, offset: u32) -> f32 {
    hash_u32(seed, offset) as f32 / u32::MAX as f32
}

/// Initialize persistent cache and load persisted navigation history
pub fn initialize_persistent_cache(
    mut commands: Commands,
    mut history: ResMut<NavigationHistory>,
) {
    if let Some(cache) = PersistentCache::new(3600) {
        // Load persisted history entries (filter to paths that still exist)
        let persisted = cache.load_history();
        for entry in persisted {
            if entry.exists() && !history.entries.contains(&entry) {
                history.entries.push(entry);
            }
        }
        let max = history.max_entries;
        history.entries.truncate(max);

        if !history.entries.is_empty() {
            info!(
                "Loaded {} history entries from persistent cache",
                history.entries.len()
            );
        }

        commands.insert_resource(cache);
    } else {
        warn!("Persistent cache unavailable, running without persistence");
    }
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
