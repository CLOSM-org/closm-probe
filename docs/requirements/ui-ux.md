[← Metaphor](./metaphor.md) | [Index](./index.md) | [Visual →](./visual.md)

# UI/UX Design

画面構成・操作・視覚フィードバックの定義。

---

## Layout

```
┌──────────┬────────────────────────────────────┐
│          │ [/ > Documents > Projects]         │
│  Side    │           (breadcrumb overlay)     │
│  bar     │                                    │
│          │           3D Universe              │
│ [Spatial │                                    │
│    nav]  │         tooltip [file.txt]         │
│          │                                    │
│  ──────  │                                    │
│  Settings│                                    │
└──────────┴────────────────────────────────────┘
```

| Element | Specification |
|---------|---------------|
| Sidebar | **Left side**, always visible, spatial navigation style, semi-transparent |
| 3D View | Main area (right of sidebar). **Star always centered in this area** |
| Breadcrumb | Overlay inside 3D view (semi-transparent) |
| Tooltip | Shows hovered celestial details |

### 3D View Rendering

- **Viewport**: Excludes sidebar area (rendering area = window width - sidebar width)
- **Center**: Star (current folder) is always centered in the 3D viewport
- **HiDPI**: Viewport scales correctly on Retina displays

---

## Sidebar Design (Spatial Navigation Style)

Design philosophy: **Spatial navigation** (VS Code Explorer / Figma Pages pattern), not conversation management.

```
┌─────────────────────┐
│ CLOSM Probe         │  ← Identity
│                     │
│ ┌─────────────────┐ │
│ │ 📂 Open Folder  │ │  ← Primary Action (Fitts's Law)
│ └─────────────────┘ │
│                     │
│  Recent             │  ← Temporal section
│  Documents          │    (Gestalt spacing, no dividers)
│    ~/Work/docs/...  │    ← Path hint (secondary color)
│  Downloads          │
│    ~/Users/dl/...   │
│                     │    ← Spacing = visual group separator
│  Selected           │  ← Context section
│  file.txt           │
│  Size: 1.2 KB       │
│  Modified: 2h ago   │
│                     │
│                     │
│  ⚙ Settings ─────┐ │  ← System (bottom, L1 expand)
│  │ Theme: [Dark]  │ │
│  │ Limit: [10]    │ │
│  │ Hidden: [ ]    │ │
│  └────────────────┘ │
└─────────────────────┘
```

### Sidebar Sections

| Section | Content | Visibility |
|---------|---------|------------|
| Identity | App title | Always |
| Primary Action | Open Folder button (accent, full-width) | Always |
| Temporal | Recent folders with path hints | Always |
| Context | Selected celestial details (name, size, modified) | When selected |
| System | Settings panel (theme, display limit, hidden files) | Click to expand (L1) |

### Section Grouping

Uses **Gestalt spacing** (not explicit dividers): subtle spacing and background differentiation create visual groups without separator lines.

### Sidebar Dimensions

| Property | Value |
|----------|-------|
| Width | 260px (fixed) |
| Background | Dark theme: `#1a1a2e` / Light: `#f5f5f5` |
| Padding | 16px |

### History Entries

| Property | Value |
|----------|-------|
| Default display count | 10 |
| Configurable range | 10 - 30 (via Settings) |
| Entry format | Folder name + shortened path hint (secondary color) |

### Settings Panel

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
- Manual toggle available in Settings panel
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
