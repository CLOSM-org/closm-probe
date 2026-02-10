//! Visual encoding calculations
//!
//! Maps file metadata to visual properties (size, color, brightness).

use crate::components::{Brightness, FileType};
use crate::resources::VisualConfig;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::SystemTime;

/// Calculate celestial body size from file size
///
/// Uses band-based normalization (KB/MB/GB/TB bands) with volume-proportional radius.
pub fn calculate_size(size_bytes: u64, is_directory: bool, config: &VisualConfig) -> f32 {
    let normalized = band_normalize(size_bytes);

    // Volume-proportional mapping: radius = (min³ + t*(max³ - min³))^(1/3)
    // This ensures visual volume (∝ r³) scales linearly with normalized size.
    let (min, max) = if is_directory {
        (config.dir_size_min, config.dir_size_max)
    } else {
        (config.file_size_min, config.file_size_max)
    };
    let min3 = min * min * min;
    let max3 = max * max * max;
    (min3 + normalized * (max3 - min3)).cbrt()
}

/// Normalize byte size using magnitude bands for perceptual differentiation.
///
/// Each band (KB, MB, GB, TB) gets a proportional slice of [0, 1].
/// Log-linear interpolation within each band preserves relative differences.
fn band_normalize(size_bytes: u64) -> f32 {
    const BANDS: &[(u64, f32)] = &[
        (1, 0.00),                    // < 1 KB
        (1_000, 0.10),                // 1 KB
        (1_000_000, 0.35),            // 1 MB
        (1_000_000_000, 0.70),        // 1 GB
        (1_000_000_000_000, 1.00),    // 1 TB
    ];

    let bytes = size_bytes.max(1);

    for i in 1..BANDS.len() {
        if bytes < BANDS[i].0 {
            let (low_bytes, low_norm) = BANDS[i - 1];
            let (high_bytes, high_norm) = BANDS[i];
            let log_low = (low_bytes as f64).log10();
            let log_high = (high_bytes as f64).log10();
            let log_val = (bytes as f64).log10();
            let t = ((log_val - log_low) / (log_high - log_low)) as f32;
            return low_norm + t * (high_norm - low_norm);
        }
    }

    1.0 // >= 1 TB
}

/// Calculate brightness from modification time
pub fn calculate_brightness(modified: SystemTime) -> Brightness {
    let age_secs = modified
        .elapsed()
        .map(|d| d.as_secs())
        .unwrap_or(u64::MAX);
    Brightness::from_age_seconds(age_secs)
}

/// Create material for celestial body
pub fn create_celestial_material(
    file_type: FileType,
    brightness: f32,
    materials: &mut Assets<StandardMaterial>,
) -> Handle<StandardMaterial> {
    let base_color = file_type.color();

    // Directories need alpha blending for pulse animation during size calculation
    let alpha_mode = if file_type == FileType::Directory {
        AlphaMode::Blend
    } else {
        AlphaMode::Opaque
    };

    materials.add(StandardMaterial {
        base_color,
        emissive: LinearRgba::from(base_color) * brightness * 2.0,
        alpha_mode,
        ..default()
    })
}

/// Create emissive material for the star
pub fn create_star_material(materials: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
    let star_color = Color::srgb(1.0, 0.9, 0.6); // Warm yellow

    materials.add(StandardMaterial {
        base_color: star_color,
        emissive: LinearRgba::from(star_color) * 8.0,
        ..default()
    })
}

/// Calculate orbital position for a planet
///
/// Distributes planets evenly around the star.
pub fn calculate_orbital_position(index: usize, total: usize, orbit_radius: f32) -> Vec3 {
    let angle = 2.0 * PI * (index as f32) / (total as f32);

    // Add slight Y variation for visual interest
    let y_offset = ((index as f32 * 1.7).sin() * 0.5).clamp(-0.5, 0.5);

    Vec3::new(
        orbit_radius * angle.cos(),
        y_offset,
        orbit_radius * angle.sin(),
    )
}

/// Orbit layout configuration
pub const ORBIT_RADIUS: f32 = 8.0;

/// Create sphere mesh for planet/star
pub fn create_sphere_mesh(radius: f32, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
    meshes.add(Sphere::new(radius))
}

/// Create octahedron mesh for file representation
pub fn create_octahedron_mesh(size: f32, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
    // Octahedron vertices: 6 points at ±x, ±y, ±z
    let s = size;
    let vertices = [
        [0.0, s, 0.0],   // top
        [0.0, -s, 0.0],  // bottom
        [s, 0.0, 0.0],   // +x
        [-s, 0.0, 0.0],  // -x
        [0.0, 0.0, s],   // +z
        [0.0, 0.0, -s],  // -z
    ];

    // 8 triangular faces
    let indices = vec![
        // top half
        0, 2, 4, // top, +x, +z
        0, 4, 3, // top, +z, -x
        0, 3, 5, // top, -x, -z
        0, 5, 2, // top, -z, +x
        // bottom half
        1, 4, 2, // bottom, +z, +x
        1, 3, 4, // bottom, -x, +z
        1, 5, 3, // bottom, -z, -x
        1, 2, 5, // bottom, +x, -z
    ];

    // Calculate normals for each face
    let mut positions = Vec::new();
    let mut normals = Vec::new();

    for face in indices.chunks(3) {
        let v0 = Vec3::from_array(vertices[face[0] as usize]);
        let v1 = Vec3::from_array(vertices[face[1] as usize]);
        let v2 = Vec3::from_array(vertices[face[2] as usize]);

        let normal = (v1 - v0).cross(v2 - v0).normalize();

        for &idx in face {
            positions.push(vertices[idx as usize]);
            normals.push(normal.to_array());
        }
    }

    let indices: Vec<u32> = (0..24).collect();

    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::MAIN_WORLD
            | bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

    meshes.add(mesh)
}
