# Scene Graph Design

Celestial body spawning and scene hierarchy.

---

## Entity Hierarchy

```
World
├── Camera3d + PanOrbitCamera
├── AmbientLight (Resource)
├── BackgroundStar (single mesh, 300 quads, permanent)
│   ├── Mesh3d (per-vertex color quads)
│   └── MeshMaterial3d (unlit, double_sided)
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
| `Mesh3d` | Sphere(1.0) — unit sphere |
| `MeshMaterial3d` | White with brightness |
| `Transform` | Orbital position, scale = calculated size |

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

## Visual Encoding

Size, brightness, color rules: see [Visual Encoding](../requirements/visual.md) (single source).

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
| BackgroundStar | Single mesh (300 quads) | Per-vertex color, unlit, 1 draw call |
| Star | `Sphere::new(2.5)` | Fixed size |
| Directory Planet | `Sphere::new(1.0)` | Unit sphere, sized via `transform.scale` |
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
