[← Metaphor](./metaphor.md) | [Index](./index.md) | [Visual →](./visual.md)

# UI/UX Design

Screen layout, interaction, and visual feedback definitions.

---

## Layout

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

| Element | Specification |
|---------|---------------|
| Sidebar | **Left side**, fixed navigation controller, always same structure |
| Main Content | Switches based on `MainView` state |
| Breadcrumb | Overlay inside 3D view (Universe mode only) |
| Tooltip | Hovered celestial details (Universe mode only) |

### MainView Switching

| View | Content | Condition |
|------|---------|-----------|
| `Universe` | 3D scene + breadcrumb/tooltip overlays | Default view |
| `Settings` | Full-page settings panel (CentralPanel) | Settings bar clicked |

**Key principle**: Sidebar = navigation controller (always same structure), Main area = content switching.

### 3D View Rendering

- **Viewport**: Excludes sidebar area (rendering area = window width - sidebar width)
- **Center**: Star (current folder) is always centered in the 3D viewport
- **HiDPI**: Viewport scales correctly on Retina displays

---

## Sidebar Design (3-Zone Layout)

Design philosophy: **Spatial navigation** (VS Code Explorer / Figma Pages pattern). Sidebar is a fixed navigation controller, not content area.

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

### Sidebar Zones

| Zone | Content | Behavior |
|------|---------|----------|
| Zone 1: Fixed Top | Identity + Open Folder button | Always visible, never scrolls |
| Zone 2: Scrollable Middle | Recent + Selected info | Scrolls when content overflows |
| Zone 3: Fixed Bottom | Settings bar (44px) | Always visible, toggles MainView |

### Sidebar Sections

| Section | Zone | Content | Visibility |
|---------|------|---------|------------|
| Identity | 1 | App title | Always |
| Primary Action | 1 | Open Folder button (accent, full-width) | Always |
| Temporal | 2 | Recent folders with path hints | Always |
| Context | 2 | Selected celestial details (name, size, modified) | Viewing + selected |
| Settings bar | 3 | Navigation toggle to Settings view | Always |

### Section Grouping

Uses **Gestalt spacing** (not explicit dividers): subtle spacing and background differentiation create visual groups without separator lines.

### Sidebar Dimensions

| Property | Value |
|----------|-------|
| Width | 260px (fixed) |
| Background | Dark theme: `#1a1a2e` / Light: `#f5f5f5` |
| Padding | 16px |
| Footer height | 44px |

### History Entries

| Property | Value |
|----------|-------|
| Default display count | 10 |
| Configurable range | 10 - 30 (via Settings) |
| Entry format | Folder name + shortened path hint (secondary color) |

---

## Main Content Area

### Universe View (default)

3D scene with overlay elements:
- Breadcrumb navigation (top of 3D area)
- Hover tooltip (near hovered entity)

### Settings View

Full-page settings panel displayed in CentralPanel when `MainView::Settings`.

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

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| Theme | Toggle (Dark/Light) | OS-detected | Manual dark/light switch |
| Display limit | Slider (10-30) | 10 | Max history entries shown |
| Show hidden files | Toggle | Off | Include dotfiles in visualization |

---

## Startup

- Show empty universe with faint starfield
- Sidebar visible with "Open Folder" button
- No modal dialog - integrated in sidebar

---

## Visual Feedback

| State | Representation |
|-------|----------------|
| Hover | Highlight + name display (tooltip) |
| Selected | Glow effect + sidebar shows details |
| Drilldown | Zoom animation (800ms) |

---

## Theme

- OS-aware at startup (follows macOS dark/light setting)
- Manual toggle available in Settings view
- Dark mode default for "space" feel

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Esc | Clear selection |
| Space | Reset view to center |
| Backspace | Navigate to parent (drill up) |

---

## Mouse Controls

| Action | Behavior |
|--------|----------|
| Left click | Select celestial |
| Double-click (directory) | Drill down into folder |
| Right drag | Orbit camera |
| Scroll | Zoom in/out |

---

## Scope (MVP)

- View-only (no file editing/deletion)
- Single folder navigation at a time

---

## See Also

- [Core Metaphor](./metaphor.md) - Operation mapping definitions
- [Visual Encoding](./visual.md) - Visual encoding rules
- [Tech Stack](./tech.md) - UI implementation (egui)
