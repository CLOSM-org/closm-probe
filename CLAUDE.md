# CLOSM Probe Development Guidelines

**Last Updated**: 2026-02-05

---

## Project Overview

**CLOSM Probe**: 3D storage visualization tool using universe metaphor.

| Item | Value |
|------|-------|
| Status | **Detailed Design Complete** - Ready for testing |
| Language | Rust (Edition 2024) |
| Engine | Bevy 0.15 |

### Core Metaphor

| Storage | Universe |
|---------|----------|
| Current folder | Star (center) |
| Child folder | Planet (sphere) |
| Child file | Planet (octahedron) |
| Grandchild | Ring around planet |

---

## Tech Stack

```toml
[dependencies]
bevy = "0.15"                    # ECS game engine
bevy_egui = "0.31"               # UI overlay
bevy_panorbit_camera = "0.22"    # Orbital camera
bevy_hanabi = "0.14"             # GPU particles
rfd = "0.15"                     # File dialog
dark-light = "1.0"               # OS theme detection
```

---

## Absolute Rules

| Rule | Description |
|------|-------------|
| **No Auto-Commit** | Never commit without explicit instruction |
| **No Auto-Run** | Never `cargo run` without instruction. `cargo check` is OK |
| **English Docs** | Documentation in English, conversation in Japanese |
| **Metaphor Sync** | Visual changes must align with `docs/requirements/metaphor.md` |

---

## Module Structure

```
src/
├── main.rs           # App entry, plugin registration
├── states.rs         # AppState (Empty → Loading → Viewing)
├── events.rs         # FolderSelectedEvent, DrillDownEvent, etc.
├── bundles.rs        # StarBundle, DirectoryPlanetBundle, FilePlanetBundle
├── components/
│   ├── celestial.rs  # CelestialBody, Star, Planet, FileType
│   ├── interaction.rs # Clickable, Drillable, Hovered, Selected
│   └── visual.rs     # Brightness, GrandchildRing, AsteroidBelt
├── resources/
│   ├── navigation.rs # CurrentDirectory, Breadcrumb, NavigationHistory
│   ├── cache.rs      # DirectoryCache (LRU, 50 entries, 30s TTL)
│   ├── ui_state.rs   # UiState, UiLayout, PendingFolderSelection
│   └── config.rs     # VisualConfig, ThemeConfig, CameraConfig
├── systems/
│   ├── setup.rs      # Camera, lighting, resources initialization
│   ├── cleanup.rs    # State exit cleanup
│   ├── filesystem.rs # Directory reading (sync, std::fs)
│   ├── spawning.rs   # Celestial body spawning
│   ├── camera.rs     # Animation, view reset
│   ├── interaction.rs # Hover, selection, drilldown
│   └── ui.rs         # egui rendering
└── utils/
    └── visual_encoding.rs # Size/color/brightness calculations
```

---

## Documentation Structure

```
docs/
├── requirements/          # Requirements definition
│   ├── index.md           # Requirements index
│   ├── metaphor.md        # Core metaphor mapping (CRITICAL)
│   ├── visual.md          # Size/color/brightness values
│   ├── ui-ux.md           # UI layout, interactions
│   └── tech.md            # Technology decisions
├── design/                # Detailed design
│   ├── index.md           # Design index
│   ├── ecs-architecture.md # Components, Resources, States
│   ├── scene-graph.md     # Bundles, spawning, hierarchy
│   ├── camera.md          # Orbital controls, animation
│   └── ui.md              # egui components, theme
└── reference/
    └── bevy-notes.md      # Bevy 0.15 patterns, rfd gotchas
```

---

## Development Process

```
1. Read relevant design docs first
2. Implement incrementally
3. cargo check frequently
4. Wait for user to test (cargo run)
5. Wait for commit instruction
```

### Key Design Documents

| When | Read |
|------|------|
| Adding visual features | `docs/requirements/visual.md` |
| Changing metaphor | `docs/requirements/metaphor.md` |
| Modifying ECS | `docs/design/ecs-architecture.md` |
| Camera changes | `docs/design/camera.md` |
| UI changes | `docs/design/ui.md` |

---

## Bevy 0.15 Patterns

### Spawning 3D Objects

```rust
commands.spawn((
    Mesh3d(meshes.add(Sphere::new(1.0))),
    MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.2),
        emissive: color.into(),
        ..default()
    })),
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

### State Machine

```rust
#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    #[default]
    Empty,
    Loading,
    Viewing,
}
```

### rfd File Dialog (macOS)

**IMPORTANT**: Don't open dialogs at startup - blocks event loop.

```rust
// Triggered by button click, not at app start
if ui.button("Open Folder").clicked() {
    if let Some(path) = rfd::FileDialog::new().pick_folder() {
        // Process folder
    }
}
```

---

## Commands

```bash
cargo check          # Quick syntax check (allowed anytime)
cargo run            # Run application (wait for instruction)
cargo clippy         # Lint check
cargo fmt            # Format code
```

---

## rust-skills Reference

### When to Use

| Situation | Skill |
|-----------|-------|
| Type-driven design | `/rust-skills:m05-type-driven` |
| Domain modeling | `/rust-skills:m09-domain` |
| Performance issues | `/rust-skills:m10-performance` |
| Lifecycle/RAII | `/rust-skills:m12-lifecycle` |
| Error handling | `/rust-skills:m13-domain-error` |
| Anti-pattern check | `/rust-skills:m15-anti-pattern` |
| Unsafe code review | `/rust-skills:unsafe-checker` |

### Crate Skills

Generate documentation skills for dependencies:

```
/rust-skills:sync-crate-skills          # Sync all from Cargo.toml
/rust-skills:docs bevy                  # Fetch specific crate docs
/rust-skills:cache-status               # Check cache status
```

### Workflow Example

```
1. Encounter unfamiliar crate API
2. /rust-skills:docs {crate_name}
3. Reference generated skill for patterns
```

---

## Cross-Session Memory (claude-mem)

### Quick Reference

```
search(query="spawning", project="closm-probe")
timeline(anchor=123, depth_before=5, depth_after=5)
get_observations(ids=[123, 122, 121])
```

### Observation Types

| Type | When |
|------|------|
| discovery | Code exploration |
| change | File modifications |
| decision | Architecture choices |
| bugfix | Bug fixes |
| feature | New functionality |

---

## Visual Encoding Quick Reference

### Size (log10 scale)

| Entity | Range |
|--------|-------|
| Star | 2.5 (fixed) |
| Directory | 0.5 - 2.0 |
| File | 0.3 - 1.8 |

### Color (FileType)

| Type | Color |
|------|-------|
| Code | Cyan `#61dafb` |
| Image | Orange `#f59e0b` |
| Video | Red `#ef4444` |
| Document | Blue `#3b82f6` |
| Data | Teal `#06b6d4` |
| Archive | Gray `#6b7280` |
| Directory | White |

### Brightness (modification time)

| Age | Brightness |
|-----|------------|
| < 24h | 100% |
| < 1 week | 85% |
| < 1 month | 70% |
| < 3 months | 55% |
| < 1 year | 40% |
| > 1 year | 25% |

---

## Testing Checklist

1. ✅ Startup → Empty universe with "Open Folder" button
2. ✅ Select folder → Celestials spawn
3. ✅ Hover → Tooltip appears
4. ✅ Click → Selection state
5. ✅ Double-click directory → Drilldown animation
6. ✅ Breadcrumb click → Navigate to ancestor
7. ✅ Space key → Reset view
8. ✅ Esc key → Clear selection
