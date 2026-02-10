# UI Design

egui overlay UI system with left sidebar (navigation controller) and switchable main content area.

---

## Architecture

### bevy_egui Integration

```
Bevy 3D Render → egui Overlay → Final Frame
```

UI is rendered as a transparent overlay on top of the 3D scene.

### MainView Switching

```
┌──────────┬────────────────────────────────────┐
│          │                                    │
│ Sidebar  │  Main Content Area                 │
│ (260px)  │  (switches based on MainView)      │
│          │                                    │
│  Always  │  Universe: 3D scene + overlays     │
│  same    │  Settings: egui CentralPanel       │
│  struct  │                                    │
│          │                                    │
└──────────┴────────────────────────────────────┘
```

| MainView | Content | UI Elements |
|----------|---------|-------------|
| `Universe` | 3D scene visible | Breadcrumb overlay, tooltip overlay |
| `Settings` | Opaque CentralPanel covers 3D | Settings page (theme, limits, hidden files) |

**Key principle**: Sidebar = fixed navigation controller, Main area = content switching.

**Rendering**: Camera always active (no `is_active` toggle). CentralPanel is fully opaque and painted after 3D in the same frame.

### Single-System Rendering

Each AppState has **one system** that renders both sidebar and main content:

```
render_startup_ui  (Empty)  → SidePanel + CentralPanel (if Settings)
render_sidebar     (Viewing) → SidePanel + CentralPanel (if Settings)
```

Settings page is drawn by `draw_settings_page()` (private helper), called **within** the sidebar system after `SidePanel::show()`. This guarantees sidebar state changes and main content rendering happen in the same frame — no system ordering ambiguity, no flicker.

---

## Sidebar (Left Panel) — 3-Zone Layout

### Structure

```
┌─────────────────────┐
│ CLOSM Probe         │  Zone 1: Fixed Top
│ [Open Folder]       │
│                     │
│ ┌─ ScrollArea ────┐ │  Zone 2: Scrollable Middle
│ │ Recent           │ │    height = available - footer_height
│ │  Documents       │ │
│ │   ~/Work/docs    │ │
│ │  Downloads       │ │
│ │   ~/Users/dl     │ │
│ │                  │ │
│ │ Selected         │ │  (Viewing state only)
│ │  file.txt        │ │
│ │  Size: 1.2 KB    │ │
│ └──────────────────┘ │
│                      │
│ [Settings]           │  Zone 3: Fixed Bottom (44px)
└─────────────────────┘
```

### Zone Implementation

```rust
let ctx = contexts.ctx_mut();

// ── Sidebar ──
egui::SidePanel::left("sidebar").show(ctx, |ui| {
    // Zone 1: Fixed Top
    render_identity(ui);
    render_open_folder_button(ui, ...);

    // Zone 2: Scrollable Middle
    let footer_h = 44.0;
    let scroll_h = (ui.available_height() - footer_h).max(0.0);
    egui::ScrollArea::vertical()
        .max_height(scroll_h)
        .auto_shrink(false)
        .show(ui, |ui| { /* Recent + Selected */ });

    // Zone 3: Fixed Bottom
    render_settings_bar(ui, &mut ui_state);
});

// ── Main Content (same frame, after sidebar) ──
if ui_state.main_view == MainView::Settings {
    draw_settings_page(ctx, &mut sidebar_settings, &mut theme_config);
}
```

### Properties

| Property | Value |
|----------|-------|
| Position | Left edge (SidePanel::left) |
| Width | 260px (fixed) |
| Always visible | Yes (no toggle needed) |
| Background | Dark: `#1a1a2e` / Light: `#f5f5f5` |
| Section grouping | Gestalt spacing (no explicit dividers) |
| Footer height | 44px (fixed) |

### Sections

| Section | Zone | Content | Visibility |
|---------|------|---------|------------|
| Identity | 1 | App title | Always |
| Primary Action | 1 | Open Folder button (accent, full-width) | Always |
| Temporal | 2 | Recent folders with path hints (`SidebarSettings.history_limit`) | Always |
| Context | 2 | Selected celestial details | Viewing + selected |
| Settings bar | 3 | Toggle button for MainView switching | Always |

### History Entry Format

```
 Documents                          <- Folder name (primary color)
  ~/Work/Projects/docs/...          <- Shortened path (secondary color, smaller)
```

Path hint uses `shorten_path()`: replaces home directory with `~`, truncates middle segments with `...` if too long.

### Settings Bar (Zone 3)

Fixed at bottom of sidebar. Toggles `MainView` between `Universe` and `Settings`.

- Active state: highlighted background when `MainView::Settings`
- Inactive state: subtle background when `MainView::Universe`

---

## Settings View (Main Area)

Displayed as `CentralPanel` when `MainView::Settings`. Replaces 3D scene visibility.

```
┌─────────────────────────────────────┐
│                                     │
│     Settings                        │
│                                     │
│     Appearance                      │
│       Theme    [Dark] / [Light]     │
│                                     │
│     Display                         │
│       History limit  ═══○═══  10    │
│       [ ] Show hidden files         │
│                                     │
│                                     │
└─────────────────────────────────────┘
```

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
| Visible | Only when `MainView::Universe` |

```
[/ > Documents > Projects > Current]
```

---

## Tooltip

| Property | Value |
|----------|-------|
| Trigger | Hover over celestial |
| Position | Near hovered entity (3D -> 2D projection) |
| Content | Name, size, relative time |
| Background | Dark `rgba(20, 20, 30, 230)` |
| Visible | Only when `MainView::Universe` |

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
    -> Spawn async rfd::AsyncFileDialog
    -> poll_file_dialog system polls each frame
    -> On completion: Set PendingFolderSelection
    -> check_folder_selection detects
    -> Update CurrentDirectory, NavigationHistory
    -> Transition to AppState::Viewing
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

### 3D -> 2D Projection

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

| System | Schedule | Condition | Purpose |
|--------|----------|-----------|---------|
| `render_startup_ui` | Update in Empty | — | Sidebar + Settings page (via `draw_settings_page`) |
| `render_sidebar` | Update in Viewing | — | Sidebar + Settings page (via `draw_settings_page`) |
| `sync_main_view_camera` | Update (global) | — | Toggle PanOrbitCamera.enabled per MainView |
| `handle_keyboard` | Update (global) | — | Esc (close Settings / clear selection), Space (reset view) |
| `poll_file_dialog` | Update in Empty+Viewing | — | Poll async dialog |
| `check_folder_selection` | Update in Empty+Viewing | — | Detect pending selection, reset MainView |
| `render_breadcrumb` | Update in Viewing | `MainView::Universe` | Navigation overlay |
| `render_tooltip` | Update in Viewing | `MainView::Universe` | Hover information |
| `update_hover` | Update in Viewing | `MainView::Universe` | Hover detection (skip in Settings) |
| `handle_selection` | Update in Viewing | `MainView::Universe` | Click selection (skip in Settings) |
| `handle_navigate_to` | Update in Viewing | — | Breadcrumb/history nav, resets MainView |

### MainView Auto-Reset Rule

**Any navigation action resets `main_view` to `Universe`:**

| Trigger | System | Mechanism |
|---------|--------|-----------|
| Sidebar history click | `render_startup_ui` / `render_sidebar` | Direct set |
| Folder dialog completion | `check_folder_selection` | Direct set |
| Breadcrumb/history navigation | `handle_navigate_to` | Direct set |
| Esc key | `handle_keyboard` | Direct set |

---

## Resources

| Resource | Purpose |
|----------|---------|
| `UiState` | Track hover, selection, main_view state |
| `UiLayout` | Dimensions (sidebar width, padding) |
| `SidebarSettings` | User preferences (history limit, hidden files) |
| `PendingFolderSelection` | Async dialog result |
| `FileDialogTask` | Running async dialog task |
| `NavigationHistory` | Recent folders list |
| `ThemeConfig` | Colors and dark/light mode |

### UiState Resource

```rust
#[derive(Resource)]
pub struct UiState {
    pub hovered_entity: Option<Entity>,
    pub selected_entity: Option<Entity>,
    pub main_view: MainView,  // Universe (default) or Settings
}
```

### SidebarSettings Resource

```rust
#[derive(Resource)]
pub struct SidebarSettings {
    pub history_limit: usize,    // Display limit (default: 10, range: 10-30)
    pub show_hidden_files: bool, // Include dotfiles (default: false)
}
```

---

## Shared UI Components

| Function | Purpose |
|----------|---------|
| `sidebar_frame()` | SidePanel frame config (background, stroke) |
| `render_identity(ui)` | App title heading |
| `render_open_folder_button(ui, ...)` | Primary action button |
| `render_history_entries(ui, ...)` | Recent entries list with click callback |
| `render_settings_bar(ui, ui_state)` | Fixed bottom bar, toggles MainView |
| `draw_settings_page(ctx, ...)` | Settings CentralPanel (private, called from sidebar systems) |
| `section_label(ui, text)` | Section heading |
| `shorten_path(path)` | Path hint formatting |

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Esc | Close Settings (if open), else clear selection |
| Space | Reset view (Viewing + Universe only) |
| Backspace | Navigate to parent |

---

## See Also

- [ECS Architecture](./ecs-architecture.md) - UI resources
- [Requirements: UI/UX](../requirements/ui-ux.md) - Layout specification
