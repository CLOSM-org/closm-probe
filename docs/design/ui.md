# UI Design

egui overlay UI system with left sidebar (spatial navigation style).

---

## Architecture

### bevy_egui Integration

```
Bevy 3D Render → egui Overlay → Final Frame
```

UI is rendered as a transparent overlay on top of the 3D scene.

### Layout

```
┌──────────┬────────────────────────────────────┐
│          │ [/ > Documents > Projects]         │
│  Sidebar │           (breadcrumb overlay)     │
│  (left)  │                                    │
│          │           3D Universe              │
│  260px   │                                    │
│  fixed   │         tooltip [file.txt]         │
│          │                                    │
└──────────┴────────────────────────────────────┘
```

---

## Sidebar (Left Panel)

### Structure

```
┌─────────────────────┐
│ CLOSM Probe         │  ← Identity
│                     │
│ [📂 Open Folder]    │  ← Primary Action
│                     │
│                     │  ← Gestalt spacing (no divider)
│ Recent              │  ← Temporal section
│  Documents          │     (NavigationHistory)
│   ~/Work/docs/...   │     ← Path hint (secondary color)
│  Downloads          │
│   ~/Users/dl/...    │
│                     │
│                     │  ← Gestalt spacing
│ Selected            │  ← Context section
│  file.txt           │     (when entity selected)
│  Size: 1.2 KB       │
│  Modified: 2h ago   │
│                     │
│                     │
│ ⚙ Settings ──────┐ │  ← System (bottom, L1 expand)
│ │ Theme: [Dark]   │ │
│ │ Limit: [10]     │ │
│ │ Hidden: [ ]     │ │
│ └─────────────────┘ │
└─────────────────────┘
```

### Properties

| Property | Value |
|----------|-------|
| Position | Left edge (SidePanel::left) |
| Width | 260px (fixed) |
| Always visible | Yes (no toggle needed) |
| Background | Dark: `#1a1a2e` / Light: `#f5f5f5` |
| Section grouping | Gestalt spacing (no explicit dividers) |

### Sections

| Section | Content | Visibility |
|---------|---------|------------|
| Identity | App title | Always |
| Primary Action | Open Folder button (accent, full-width) | Always |
| Temporal | Recent folders with path hints (SidebarSettings.history_limit) | Always |
| Context | Selected celestial details | When selected |
| System | Settings panel (collapsible) | Always (click to expand) |

### History Entry Format

```
 Documents                          ← Folder name (primary color)
  ~/Work/Projects/docs/...          ← Shortened path (secondary color, smaller)
```

Path hint uses `shorten_path()`: replaces home directory with `~`, truncates middle segments with `...` if too long.

### Settings Panel (Progressive Disclosure L1)

Collapsed by default. Click gear icon to expand/collapse.

| Setting | Widget | Resource Field |
|---------|--------|----------------|
| Theme | Dark/Light toggle | `ThemeConfig.dark_mode` |
| Display limit | Slider (10-30) | `SidebarSettings.history_limit` |
| Show hidden files | Checkbox | `SidebarSettings.show_hidden_files` |

---

## Breadcrumb Navigation

| Property | Value |
|----------|-------|
| Position | Top of 3D area (inside, not sidebar) |
| Background | Semi-transparent `rgba(30, 30, 45, 200)` |
| Separator | ` > ` |
| Clickable | All segments except current |

```
[/ > Documents > Projects > Current]
```

---

## Tooltip

| Property | Value |
|----------|-------|
| Trigger | Hover over celestial |
| Position | Near hovered entity (3D → 2D projection) |
| Content | Name, size, relative time |
| Background | Dark `rgba(20, 20, 30, 230)` |

---

## File Dialog (Async)

### rfd Integration

Uses `rfd::AsyncFileDialog` with `IoTaskPool` for non-blocking operation.

```rust
// Spawn async task
let task = IoTaskPool::get().spawn(async move {
    let handle = rfd::AsyncFileDialog::new().pick_folder().await;
    handle.map(|h| h.path().to_path_buf())
});
dialog_task.task = Some(task);
```

### Flow

```
Click "Open Folder"
    → Spawn async rfd::AsyncFileDialog
    → poll_file_dialog system polls each frame
    → On completion: Set PendingFolderSelection
    → check_folder_selection detects
    → Update CurrentDirectory, NavigationHistory
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
| Sidebar background | `#1a1a2e` |
| Panel background | `rgba(30, 30, 45, 230)` |
| Text primary | `rgb(240, 240, 250)` |
| Text secondary | `rgb(160, 160, 180)` |
| Accent | `rgb(100, 180, 255)` |

#### Light Cosmic

| Element | Color |
|---------|-------|
| Sidebar background | `#f5f5f5` |
| Text primary | `rgb(20, 25, 40)` |
| Accent | `rgb(50, 120, 200)` |

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
| `render_sidebar` | Update (always) | Left panel with history/selection |
| `poll_file_dialog` | Update in Empty | Poll async dialog |
| `check_folder_selection` | Update in Empty | Detect pending selection |
| `render_breadcrumb` | Update in Viewing | Navigation overlay |
| `render_tooltip` | Update in Viewing | Hover information |

---

## Resources

| Resource | Purpose |
|----------|---------|
| `UiState` | Track hover, selection state |
| `UiLayout` | Dimensions (sidebar width, padding) |
| `SidebarSettings` | Settings panel state and user preferences |
| `PendingFolderSelection` | Async dialog result |
| `FileDialogTask` | Running async dialog task |
| `NavigationHistory` | Recent folders list |
| `ThemeConfig` | Colors and dark/light mode |

### SidebarSettings Resource

```rust
#[derive(Resource)]
pub struct SidebarSettings {
    pub settings_open: bool,     // Panel expanded (default: false)
    pub history_limit: usize,    // Display limit (default: 10, range: 10-30)
    pub show_hidden_files: bool, // Include dotfiles (default: false)
}
```

---

## New Resource: NavigationHistory Extension

```rust
// Add to resources/navigation.rs
#[derive(Resource, Default)]
pub struct NavigationHistory {
    pub entries: Vec<PathBuf>,  // Recent folders, newest first
    pub max_entries: usize,     // Default: 10
}

impl NavigationHistory {
    pub fn push(&mut self, path: PathBuf) {
        // Remove if already exists (move to top)
        self.entries.retain(|p| p != &path);
        // Insert at front
        self.entries.insert(0, path);
        // Trim to max
        self.entries.truncate(self.max_entries);
    }
}
```

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Esc | Clear selection |
| Space | Reset view |
| Backspace | Navigate to parent |

---

## See Also

- [ECS Architecture](./ecs-architecture.md) - UI resources
- [Requirements: UI/UX](../requirements/ui-ux.md) - Layout specification
