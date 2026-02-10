# Scene Graph Design

Celestial body spawning and scene hierarchy.

---

## Entity Hierarchy

```
World
├── Camera3d + PanOrbitCamera
├── AmbientLight (Resource)
├── BackgroundStar[] (200, permanent)
│   ├── Mesh3d (sphere, 0.03-0.12)
│   └── MeshMaterial3d (unlit, emissive)
├── Star (current folder)
│   ├── Mesh3d (sphere)
│   ├── MeshMaterial3d (emissive)
│   └── PointLight
├── Planet[] (children, max 20)
│   ├── Mesh3d (sphere or octahedron)
│   ├── MeshMaterial3d
│   └── GrandchildRing (optional)
└── AsteroidBelt (if > 20 items)
```

---

## Bundles

### StarBundle

Central star representing the current directory.

| Component | Value |
|-----------|-------|
| `CelestialBody` | Current folder metadata |
| `Star` | Marker |
| `FileType` | Directory |
| `Clickable` | Yes |
| `Drillable` | Yes (to parent) |
| `Mesh3d` | Sphere(2.5) |
| `MeshMaterial3d` | Emissive yellow/orange |
| `Transform` | Origin (0, 0, 0) |
| `PointLight` | intensity: 2,000,000 |

### DirectoryPlanetBundle

Planet representing a child directory.

| Component | Value |
|-----------|-------|
| `CelestialBody` | Folder metadata |
| `Planet` | is_directory: true |
| `FileType` | Directory |
| `Brightness` | From modified time |
| `Clickable` | Yes |
| `Drillable` | Yes |
| `Mesh3d` | Sphere (size from visual encoding) |
| `MeshMaterial3d` | White with brightness |
| `Transform` | Orbital position |

### FilePlanetBundle

Planet representing a file.

| Component | Value |
|-----------|-------|
| `CelestialBody` | File metadata |
| `Planet` | is_directory: false |
| `FileType` | From extension |
| `Brightness` | From modified time |
| `Clickable` | Yes |
| `Drillable` | No |
| `Mesh3d` | Octahedron (size from visual encoding) |
| `MeshMaterial3d` | FileType color with brightness |
| `Transform` | Orbital position |

---

## Orbital Layout

Planets are arranged in a circular orbit around the star.

```
Layout Algorithm:
1. Sort items: directories first, then by name
2. Calculate radius: 8.0 units from center
3. Distribute evenly: angle = 2π × i / count
4. Position: (radius × cos(angle), 0, radius × sin(angle))
5. Add slight Y variation for visual interest
```

| Parameter | Value |
|-----------|-------|
| Orbit radius | 8.0 |
| Y variation | ±0.5 random |
| Max items | 20 |

---

## Visual Encoding Functions

### Size Calculation

```rust
fn calculate_size(size_bytes: u64, is_directory: bool, config: &VisualConfig) -> f32 {
    let log_size = (size_bytes as f64 + 1.0).log10() as f32;
    let normalized = log_size / 12.0; // ~1TB max

    if is_directory {
        config.dir_size_min + normalized * (config.dir_size_max - config.dir_size_min)
    } else {
        config.file_size_min + normalized * (config.file_size_max - config.file_size_min)
    }
}
```

### Brightness Calculation

```rust
fn calculate_brightness(modified: SystemTime) -> Brightness {
    let age = modified
        .elapsed()
        .map(|d| d.as_secs())
        .unwrap_or(u64::MAX);
    Brightness::from_age_seconds(age)
}
```

### Material Creation

```rust
fn create_material(file_type: FileType, brightness: f32) -> StandardMaterial {
    let base_color = file_type.color();
    StandardMaterial {
        base_color: base_color.with_alpha(1.0),
        emissive: base_color * brightness * 2.0,
        ..default()
    }
}
```

---

## Spawning Flow

```
OnEnter(Viewing):
  1. Read CurrentDirectory path
  2. Check DirectoryCache
  3. If miss: read filesystem, populate cache
  4. Sort entries (dirs first, then name)
  5. Spawn Star at origin
  6. For each entry (max 20):
     - Calculate orbital position
     - Calculate size, brightness
     - Spawn appropriate bundle
  7. If count > 20:
     - Spawn AsteroidBelt with overflow count
  8. For each directory planet:
     - Count grandchildren
     - If > 0: attach GrandchildRing
```

---

## Cleanup Flow

```
OnExit(Viewing):
  1. Query all entities with CelestialBody
  2. Despawn all (recursive for children)
  3. Clear UiState selections
```

---

## Meshes

| Entity | Mesh Type | Note |
|--------|-----------|------|
| BackgroundStar | `Sphere::new(0.03-0.12)` | Fibonacci sphere distribution, unlit |
| Star | `Sphere::new(2.5)` | Fixed size |
| Directory Planet | `Sphere::new(size)` | Size from encoding |
| File Planet | Octahedron | Custom mesh, size from encoding |
| GrandchildRing | Torus or 2D ring | Flat ring around planet |
| AsteroidBelt | Particles (bevy_hanabi) | GPU particles |

### Octahedron Mesh

```rust
fn create_octahedron_mesh(size: f32) -> Mesh {
    // 6 vertices: ±x, ±y, ±z
    // 8 triangular faces
}
```

---

## File Structure

```
src/
├── bundles.rs              # StarBundle, PlanetBundles
├── utils/
│   ├── mod.rs
│   ├── file_type.rs        # Extension → FileType
│   └── visual_encoding.rs  # Size, brightness, material
└── systems/
    ├── filesystem.rs       # Directory reading
    └── spawning.rs         # Celestial spawning
```

---

## See Also

- [ECS Architecture](./ecs-architecture.md) - Component definitions
- [Requirements: Visual](../requirements/visual.md) - Encoding values
- [Requirements: Metaphor](../requirements/metaphor.md) - Entity mapping
