# ECS Architecture Design

Bevy ECS architecture for CLOSM Probe.

---

## States

### AppState (Main State Machine)

```
Empty → Loading → Viewing
```

| State | Description |
|-------|-------------|
| `Empty` | Initial state, no folder selected, startup screen shown |
| `Loading` | Folder selected, reading filesystem |
| `Viewing` | Main visualization state |

### ViewingMode (SubState of Viewing)

```
Idle ↔ Animating
```

| SubState | Description |
|----------|-------------|
| `Idle` | Normal interaction, camera controllable |
| `Animating` | Camera transition in progress, input blocked |

---

## Components

### Celestial Bodies

| Component | Description | Fields |
|-----------|-------------|--------|
| `CelestialBody` | Marker for all celestial entities | `name: String`, `path: PathBuf`, `size_bytes: u64`, `modified: SystemTime` |
| `Star` | Current folder (center) | marker only |
| `Planet` | Child folder or file | `is_directory: bool` |
| `FileType` | File classification | enum: Code, Image, Video, Document, Data, Archive, Directory |

### Interaction

| Component | Description |
|-----------|-------------|
| `Clickable` | Entity can be clicked |
| `Drillable` | Entity supports drill-down (directories only) |
| `Hovered` | Currently under mouse cursor |
| `Selected` | Currently selected by user |

### Visual

| Component | Description | Fields |
|-----------|-------------|--------|
| `Brightness` | Modification time encoding | `value: f32` (0.25 - 1.0) |
| `GrandchildRing` | Grandchild abstraction ring | `count: usize` |
| `AsteroidBelt` | Overflow indicator | `count: usize` (items > 20) |

---

## Resources

### Navigation

| Resource | Description | Fields |
|----------|-------------|--------|
| `CurrentDirectory` | Active directory path | `path: PathBuf` |
| `Breadcrumb` | Navigation path segments | `segments: Vec<PathBuf>` |
| `NavigationHistory` | Back/forward history | `back: Vec<PathBuf>`, `forward: Vec<PathBuf>` |

### Cache

| Resource | Description | Fields |
|----------|-------------|--------|
| `DirectoryCache` | LRU cache for directory contents | `cache: HashMap<PathBuf, CacheEntry>`, `max_size: usize` (50) |
| `CacheEntry` | Single cache entry | `entries: Vec<FileEntry>`, `timestamp: Instant`, `ttl: Duration` (30s) |

### UI State

| Resource | Description | Fields |
|----------|-------------|--------|
| `UiState` | UI interaction state | `hovered_entity: Option<Entity>`, `selected_entity: Option<Entity>`, `sidebar_open: bool` |
| `UiLayout` | Computed layout dimensions | `sidebar_width: f32` (280.0) |

### Configuration

| Resource | Description | Fields |
|----------|-------------|--------|
| `VisualConfig` | Visual encoding parameters | `size_scale`, `brightness_range` |
| `ThemeConfig` | Color theme | `dark_mode: bool`, `colors: ThemeColors` |
| `CameraConfig` | Camera constraints | `zoom_min`, `zoom_max`, `pitch_limit` |

---

## Events

| Event | Description | Payload |
|-------|-------------|---------|
| `FolderSelectedEvent` | User selected a folder | `path: PathBuf` |
| `DrillDownEvent` | Navigate into directory | `entity: Entity`, `path: PathBuf` |
| `DrillUpEvent` | Navigate to parent | (none) |
| `SelectionChangedEvent` | Selection changed | `entity: Option<Entity>` |
| `NavigateToEvent` | Breadcrumb navigation | `path: PathBuf` |

---

## System Scheduling

### Startup Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `setup_camera` | `OnEnter(Empty)` | Initialize camera and lighting |
| `setup_ui_resources` | `Startup` | Initialize UI resources |
| `detect_theme` | `Startup` | Detect OS theme preference |

### State Transition Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `spawn_startup_ui` | `OnEnter(Empty)` | Show "Open Folder" button |
| `cleanup_startup_ui` | `OnExit(Empty)` | Remove startup UI |
| `start_loading` | `OnEnter(Loading)` | Begin directory read |
| `finish_loading` | `Update` in `Loading` | Transition to Viewing |
| `spawn_celestials` | `OnEnter(Viewing)` | Create celestial bodies |
| `cleanup_celestials` | `OnExit(Viewing)` | Despawn all celestials |

### Update Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `handle_folder_dialog` | `Update` in `Empty` | Process folder selection |
| `update_hover` | `Update` in `Viewing` | Detect hovered entity |
| `handle_selection` | `Update` in `Viewing` | Process clicks |
| `handle_drilldown` | `Update` in `Viewing` | Process double-clicks |
| `handle_keyboard` | `Update` in `Viewing` | Esc, Space keys |
| `render_ui` | `Update` | egui rendering |
| `animate_camera` | `Update` in `Animating` | Camera transitions |

---

## Entity Hierarchy

```
World
├── Camera3d
├── AmbientLight (Resource)
├── Star (current folder)
│   └── PointLight
├── Planet[] (children)
│   └── GrandchildRing (optional)
└── AsteroidBelt (optional, if > 20 items)
```

---

## File Structure

```
src/
├── states.rs           # AppState, ViewingMode definitions
├── components/
│   ├── mod.rs
│   ├── celestial.rs    # CelestialBody, Star, Planet, FileType
│   ├── interaction.rs  # Clickable, Drillable, Hovered, Selected
│   └── visual.rs       # Brightness, GrandchildRing, AsteroidBelt
├── resources/
│   ├── mod.rs
│   ├── navigation.rs   # CurrentDirectory, Breadcrumb, NavigationHistory
│   ├── cache.rs        # DirectoryCache, CacheEntry
│   ├── ui_state.rs     # UiState, UiLayout
│   └── config.rs       # VisualConfig, ThemeConfig, CameraConfig
└── events.rs           # All event definitions
```

---

## See Also

- [Scene Graph](./scene-graph.md) - Bundle definitions, spawning
- [Requirements: Metaphor](../requirements/metaphor.md) - Entity mapping rules
- [Requirements: Visual](../requirements/visual.md) - Visual encoding values
