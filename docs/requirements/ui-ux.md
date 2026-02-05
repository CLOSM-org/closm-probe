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
│  [Chat   │                                    │
│   style] │         tooltip [file.txt]         │
│          │                                    │
│  ──────  │                                    │
│  Settings│                                    │
└──────────┴────────────────────────────────────┘
```

| Element | Specification |
|---------|---------------|
| Sidebar | **Left side**, always visible, Claude/ChatGPT style, semi-transparent |
| 3D View | Main area (right of sidebar). **Star always centered in this area** |
| Breadcrumb | Overlay inside 3D view (semi-transparent) |
| Tooltip | Shows hovered celestial details |

### 3D View Rendering

- **Viewport**: Excludes sidebar area (rendering area = window width - sidebar width)
- **Center**: Star (current folder) is always centered in the 3D viewport
- **HiDPI**: Viewport scales correctly on Retina displays

---

## Sidebar Design (Claude/ChatGPT Style)

```
┌─────────────────────┐
│ CLOSM Probe         │  ← App title
│                     │
│ ┌─────────────────┐ │
│ │ 📂 Open Folder  │ │  ← Primary action button
│ └─────────────────┘ │
│                     │
│ Recent              │  ← Section label
│ ├─ Documents        │  ← Recent folder history
│ ├─ Downloads        │
│ └─ Projects         │
│                     │
│ ─────────────────── │  ← Divider
│                     │
│ Selected: file.txt  │  ← Selection info (when selected)
│ Size: 1.2 KB        │
│ Modified: 2h ago    │
│                     │
│ ─────────────────── │
│                     │
│ ⚙️ Settings         │  ← Settings at bottom
└─────────────────────┘
```

### Sidebar Sections

| Section | Content |
|---------|---------|
| Header | App title + Open Folder button |
| History | Recent folders (max 10, clickable) |
| Selection | Details of selected celestial body |
| Footer | Settings toggle |

### Sidebar Dimensions

| Property | Value |
|----------|-------|
| Width | 260px (fixed) |
| Background | Dark theme: `#1a1a2e` / Light: `#f5f5f5` |
| Padding | 16px |

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

- OS-aware (follows macOS dark/light setting)
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
