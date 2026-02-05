# UI Design

egui overlay UI system.

---

## Architecture

### bevy_egui Integration

```
Bevy 3D Render → egui Overlay → Final Frame
```

UI is rendered as a transparent overlay on top of the 3D scene.

---

## UI Components

### Startup Screen (AppState::Empty)

| Element | Description |
|---------|-------------|
| Title | "CLOSM Probe" centered |
| Subtitle | "3D Storage Visualization" |
| Button | "📂 Open Folder" |

```
┌────────────────────────────────┐
│                                │
│         CLOSM Probe            │
│    3D Storage Visualization    │
│                                │
│       [📂 Open Folder]         │
│                                │
└────────────────────────────────┘
```

### Breadcrumb Navigation

| Property | Value |
|----------|-------|
| Position | Top-left (16px padding) |
| Background | Semi-transparent (30, 30, 45, 200) |
| Separator | " > " |
| Clickable | All segments except current |

```
[/ > Documents > Projects > Current]
```

### Sidebar

| Property | Value |
|----------|-------|
| Position | Right edge |
| Width | 280px |
| Toggle | "≡" hamburger button |
| Sections | Details, Settings |

```
┌──────────────────────────────────┬──────┐
│                                  │  ≡   │
│                                  ├──────┤
│                                  │ Side │
│          3D Scene                │ bar  │
│                                  │      │
└──────────────────────────────────┴──────┘
```

### Tooltip

| Property | Value |
|----------|-------|
| Trigger | Hover over celestial |
| Position | Near hovered entity (3D → 2D projection) |
| Content | Name, size, relative time |
| Background | Dark (20, 20, 30, 230) |

---

## File Dialog

### rfd Integration

- Synchronous dialog (`FileDialog::new().pick_folder()`)
- Triggered by button click (not startup)
- Returns `Option<PathBuf>`

### Flow

```
Click "Open Folder"
    → rfd::FileDialog::pick_folder()
    → Set PendingFolderSelection resource
    → check_folder_selection system detects
    → Update CurrentDirectory
    → Transition to AppState::Viewing
```

---

## Theme System

### OS Theme Detection

Uses `dark-light` crate at startup.

```rust
let dark_mode = match dark_light::detect() {
    Mode::Dark => true,
    Mode::Light => false,
    Mode::Default => true, // Prefer dark for space theme
};
```

### Color Palettes

#### Dark Cosmic (default)

| Element | Color |
|---------|-------|
| Background | rgb(15, 15, 25) |
| Text | rgb(240, 240, 250) |
| Text secondary | rgb(160, 160, 180) |
| Accent | rgb(100, 180, 255) |
| Panel background | rgba(30, 30, 45, 230) |

#### Light Cosmic

| Element | Color |
|---------|-------|
| Background | rgb(240, 245, 255) |
| Text | rgb(20, 25, 40) |
| Accent | rgb(50, 120, 200) |

---

## Coordinate Conversion

### 3D → 2D Projection

```rust
let viewport_pos = camera.world_to_viewport(
    camera_transform,
    entity_transform.translation()
)?;

// Position egui element
egui::Area::new(id)
    .fixed_pos(egui::pos2(viewport_pos.x + 20.0, viewport_pos.y - 10.0))
    .show(ctx, |ui| { /* ... */ });
```

---

## Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `render_startup_ui` | Update in Empty | Show open folder screen |
| `check_folder_selection` | Update in Empty | Detect pending selection |
| `render_breadcrumb` | Update in Viewing | Navigation overlay |
| `render_sidebar` | Update in Viewing | Details panel |
| `render_tooltip` | Update in Viewing | Hover information |

---

## Resources

| Resource | Purpose |
|----------|---------|
| `UiState` | Track hover, selection, sidebar state |
| `UiLayout` | Dimensions (sidebar width, padding) |
| `PendingFolderSelection` | Async dialog result |
| `ThemeConfig` | Colors and dark/light mode |

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Esc | Clear selection |
| Space | Reset view |

---

## See Also

- [ECS Architecture](./ecs-architecture.md) - UI resources
- [Requirements: UI/UX](../requirements/ui-ux.md) - Layout specification
