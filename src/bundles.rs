//! Entity bundles for celestial bodies
//!
//! Bundles group components for common entity types.

use crate::components::*;
use bevy::prelude::*;
use std::path::PathBuf;
use std::time::SystemTime;

/// Bundle for the central star (current directory)
#[derive(Bundle)]
pub struct StarBundle {
    pub celestial: CelestialBody,
    pub star: Star,
    pub file_type: FileType,
    pub clickable: Clickable,
    pub drillable: Drillable,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
}

impl StarBundle {
    pub fn new(
        name: String,
        path: PathBuf,
        size_bytes: u64,
        modified: SystemTime,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            celestial: CelestialBody {
                name,
                path,
                size_bytes,
                modified,
            },
            star: Star,
            file_type: FileType::Directory,
            clickable: Clickable,
            drillable: Drillable,
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(material),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
        }
    }
}

/// Bundle for a directory planet
#[derive(Bundle)]
pub struct DirectoryPlanetBundle {
    pub celestial: CelestialBody,
    pub planet: Planet,
    pub file_type: FileType,
    pub brightness: Brightness,
    pub clickable: Clickable,
    pub drillable: Drillable,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
}

impl DirectoryPlanetBundle {
    pub fn new(
        name: String,
        path: PathBuf,
        size_bytes: u64,
        modified: SystemTime,
        brightness: Brightness,
        position: Vec3,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            celestial: CelestialBody {
                name,
                path,
                size_bytes,
                modified,
            },
            planet: Planet { is_directory: true },
            file_type: FileType::Directory,
            brightness,
            clickable: Clickable,
            drillable: Drillable,
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(material),
            transform: Transform::from_translation(position),
        }
    }
}

/// Bundle for a file planet
#[derive(Bundle)]
pub struct FilePlanetBundle {
    pub celestial: CelestialBody,
    pub planet: Planet,
    pub file_type: FileType,
    pub brightness: Brightness,
    pub clickable: Clickable,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub transform: Transform,
}

impl FilePlanetBundle {
    pub fn new(
        name: String,
        path: PathBuf,
        size_bytes: u64,
        modified: SystemTime,
        file_type: FileType,
        brightness: Brightness,
        position: Vec3,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            celestial: CelestialBody {
                name,
                path,
                size_bytes,
                modified,
            },
            planet: Planet { is_directory: false },
            file_type,
            brightness,
            clickable: Clickable,
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(material),
            transform: Transform::from_translation(position),
        }
    }
}
