//! Celestial body spawning systems
//!
//! Spawns stars, planets, and visual elements.

#![allow(dead_code)]

use crate::bundles::*;
use crate::components::*;
use crate::events::RespawnCelestialsEvent;
use crate::resources::*;
use crate::systems::filesystem::{count_directory_items, read_directory};
use crate::systems::size_calculation::{spawn_size_calculations, SizeCalculationChannel};
use crate::utils::*;
use bevy::prelude::*;

/// Spawn celestial bodies for the current directory
pub fn spawn_celestials(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_dir: Res<CurrentDirectory>,
    mut cache: ResMut<DirectoryCache>,
    config: Res<VisualConfig>,
    size_channel: Res<SizeCalculationChannel>,
    persistent_cache: Option<Res<PersistentCache>>,
) {
    let Some(path) = &current_dir.path else {
        return;
    };

    // Read directory contents
    let entries = read_directory(path, &mut cache);

    // Spawn the central star
    let star_mesh = create_sphere_mesh(config.star_size, &mut meshes);
    let star_material = create_star_material(&mut materials);

    let star_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "/".to_string());

    let star_entity = commands
        .spawn(StarBundle::new(
            star_name,
            path.clone(),
            0, // Size not calculated for current dir
            std::time::UNIX_EPOCH,
            star_mesh,
            star_material,
        ))
        .id();

    // Add point light to star
    commands.entity(star_entity).with_children(|parent| {
        parent.spawn((
            PointLight {
                intensity: 2_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            Transform::default(),
        ));
    });

    // Limit items to max display count
    let display_entries: Vec<_> = entries.iter().take(config.max_display_items).collect();
    let overflow_count = entries.len().saturating_sub(config.max_display_items);
    let total_display = display_entries.len();

    // Collect directory paths for size calculation
    let mut pending_calculations = Vec::new();

    // Spawn planets for each entry
    for (index, entry) in display_entries.iter().enumerate() {
        let position = calculate_orbital_position(index, total_display, ORBIT_RADIUS);
        let brightness = calculate_brightness(entry.modified);

        if entry.is_directory {
            // Check persistent cache for pre-calculated size
            let cached_size = persistent_cache
                .as_ref()
                .and_then(|pc| pc.get_size(&entry.path));

            let (effective_size_bytes, has_cached_size) = match cached_size {
                Some(size) => (size, true),
                None => (entry.size_bytes, false),
            };

            // Directory planet (unit sphere, sized via transform.scale)
            let size = calculate_size(effective_size_bytes, true, &config);
            let mesh = create_sphere_mesh(1.0, &mut meshes);
            let material = create_celestial_material(
                FileType::Directory,
                brightness.value,
                &mut materials,
            );

            let bundle = DirectoryPlanetBundle::new(
                entry.name.clone(),
                entry.path.clone(),
                effective_size_bytes,
                entry.modified,
                brightness,
                position,
                size,
                mesh,
                material,
            );

            let planet_entity = if has_cached_size {
                // Cached: spawn at final size, no pulse animation
                commands.spawn(bundle).id()
            } else {
                // Uncached: spawn with pulse animation, queue calculation
                commands
                    .spawn((bundle, PulseAnimation::default(), PendingSizeCalculation))
                    .id()
            };

            if !has_cached_size {
                pending_calculations.push(entry.path.clone());
            }

            // Check for grandchildren and add ring if any
            let grandchild_count = count_directory_items(&entry.path);
            if grandchild_count > 0 {
                commands.entity(planet_entity).insert(GrandchildRing {
                    count: grandchild_count,
                });
            }
        } else {
            // File planet (octahedron)
            let file_type =
                FileType::from_extension(entry.path.extension().and_then(|e| e.to_str()));
            let size = calculate_size(entry.size_bytes, false, &config);
            let mesh = create_octahedron_mesh(size, &mut meshes);
            let material =
                create_celestial_material(file_type, brightness.value, &mut materials);

            commands.spawn(FilePlanetBundle::new(
                entry.name.clone(),
                entry.path.clone(),
                entry.size_bytes,
                entry.modified,
                file_type,
                brightness,
                position,
                mesh,
                material,
            ));
        }
    }

    // Spawn background size calculations
    if !pending_calculations.is_empty() {
        spawn_size_calculations(pending_calculations, size_channel.sender.clone());
    }

    // Spawn asteroid belt for overflow items
    if overflow_count > 0 {
        commands.spawn((AsteroidBelt { count: overflow_count }, Transform::default()));
        info!("Overflow: {} items in asteroid belt", overflow_count);
    }

    info!(
        "Spawned {} celestials for {}",
        display_entries.len() + 1, // +1 for star
        path.display()
    );
}

/// Despawn all celestial bodies
pub fn despawn_celestials(
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

/// Handle respawn event - spawns celestials when navigation occurs
pub fn handle_respawn_celestials(
    mut events: EventReader<RespawnCelestialsEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_dir: Res<CurrentDirectory>,
    mut cache: ResMut<DirectoryCache>,
    config: Res<VisualConfig>,
    size_channel: Res<SizeCalculationChannel>,
    persistent_cache: Option<Res<PersistentCache>>,
) {
    // Only process if there's an event
    if events.read().next().is_none() {
        return;
    }

    // Clear any remaining events
    events.clear();

    let Some(path) = &current_dir.path else {
        return;
    };

    // Read directory contents
    let entries = read_directory(path, &mut cache);

    // Spawn the central star
    let star_mesh = create_sphere_mesh(config.star_size, &mut meshes);
    let star_material = create_star_material(&mut materials);

    let star_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "/".to_string());

    let star_entity = commands
        .spawn(StarBundle::new(
            star_name,
            path.clone(),
            0,
            std::time::UNIX_EPOCH,
            star_mesh,
            star_material,
        ))
        .id();

    // Add point light to star
    commands.entity(star_entity).with_children(|parent| {
        parent.spawn((
            PointLight {
                intensity: 2_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            Transform::default(),
        ));
    });

    // Limit items to max display count
    let display_entries: Vec<_> = entries.iter().take(config.max_display_items).collect();
    let overflow_count = entries.len().saturating_sub(config.max_display_items);
    let total_display = display_entries.len();

    // Collect directory paths for size calculation
    let mut pending_calculations = Vec::new();

    // Spawn planets for each entry
    for (index, entry) in display_entries.iter().enumerate() {
        let position = calculate_orbital_position(index, total_display, ORBIT_RADIUS);
        let brightness = calculate_brightness(entry.modified);

        if entry.is_directory {
            // Check persistent cache for pre-calculated size
            let cached_size = persistent_cache
                .as_ref()
                .and_then(|pc| pc.get_size(&entry.path));

            let (effective_size_bytes, has_cached_size) = match cached_size {
                Some(size) => (size, true),
                None => (entry.size_bytes, false),
            };

            let size = calculate_size(effective_size_bytes, true, &config);
            let mesh = create_sphere_mesh(1.0, &mut meshes);
            let material =
                create_celestial_material(FileType::Directory, brightness.value, &mut materials);

            let bundle = DirectoryPlanetBundle::new(
                entry.name.clone(),
                entry.path.clone(),
                effective_size_bytes,
                entry.modified,
                brightness,
                position,
                size,
                mesh,
                material,
            );

            let planet_entity = if has_cached_size {
                commands.spawn(bundle).id()
            } else {
                commands
                    .spawn((bundle, PulseAnimation::default(), PendingSizeCalculation))
                    .id()
            };

            if !has_cached_size {
                pending_calculations.push(entry.path.clone());
            }

            let grandchild_count = count_directory_items(&entry.path);
            if grandchild_count > 0 {
                commands.entity(planet_entity).insert(GrandchildRing {
                    count: grandchild_count,
                });
            }
        } else {
            let file_type =
                FileType::from_extension(entry.path.extension().and_then(|e| e.to_str()));
            let size = calculate_size(entry.size_bytes, false, &config);
            let mesh = create_octahedron_mesh(size, &mut meshes);
            let material =
                create_celestial_material(file_type, brightness.value, &mut materials);

            commands.spawn(FilePlanetBundle::new(
                entry.name.clone(),
                entry.path.clone(),
                entry.size_bytes,
                entry.modified,
                file_type,
                brightness,
                position,
                mesh,
                material,
            ));
        }
    }

    // Spawn background size calculations
    if !pending_calculations.is_empty() {
        spawn_size_calculations(pending_calculations, size_channel.sender.clone());
    }

    if overflow_count > 0 {
        commands.spawn((AsteroidBelt { count: overflow_count }, Transform::default()));
    }

    info!(
        "Respawned {} celestials for {}",
        display_entries.len() + 1,
        path.display()
    );
}
