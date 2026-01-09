# Metaphor Mapping Reference

Single source of truth for directory ↔ universe metaphor mappings in CLOSM Probe.

---

## 1. Core Hierarchy

| Storage Element | Celestial Body | Depth | Component | Description |
|-----------------|----------------|-------|-----------|-------------|
| Current Root | Sun | 0 | (center point) | Origin at (0,0,0), stationary |
| Directory | Planet | 1 | DirectoryNode | Direct children of root |
| Subdirectory | Moon | 2+ | DirectoryNode | Children of planets |
| File | Satellite | 1+ | FileNode | Files orbiting directories |
| Small File (<100KB) | Asteroid | - | AsteroidBelt | Clustered in rings |
| Background | Star Field | - | StarField | Distant twinkling stars |

---

## 2. Visual Encoding

| Attribute | Visual Representation | Formula/Location |
|-----------|----------------------|------------------|
| **Size** | Sphere radius (log scale) | `types.ts:calculateNodeRadius()` |
| **File Type** | Color | `types.ts:typeColors` |
| **Creation Date** | Orbit radius (inner=newer) | `types.ts:calculateOrbitRadiusByOrder()` |
| **Last Modified** | Angular position (12 o'clock=newest) | `types.ts:calculateEqualSpacedAngle()` |
| **Recency** | Brightness/Glow | `types.ts:calculateBrightness()` |

---

## 3. Orbit Line Styles

| Hierarchy | Style | Color | Opacity | Rationale |
|-----------|-------|-------|---------|-----------|
| Depth 1 (Planet orbits) | Solid | `#888888` | 0.5 | Primary navigation level |
| Depth 2+ (Moon orbits) | Dashed | `#555555` | 0.3 | Secondary, less prominent |

---

## 4. Color Palette

| File Type | Color | Hex Code |
|-----------|-------|----------|
| Code | Cyan | `#61dafb` |
| Design | Purple | `#a855f7` |
| Image | Orange | `#f59e0b` |
| Video | Red | `#ef4444` |
| PDF | Dark Red | `#dc2626` |
| Document | Blue | `#3b82f6` |
| Text | Green | `#22c55e` |
| Data | Teal | `#06b6d4` |
| Archive | Gray | `#6b7280` |
| Directory | Violet | `#8b5cf6` |

---

## 5. Interaction Metaphors

| User Action | Metaphor | Effect |
|-------------|----------|--------|
| Drag | Orbit around sun | Rotate camera view |
| Scroll | Travel toward/away | Zoom in/out |
| Click | Select celestial body | Show details panel |
| Double-click directory | Enter planet | Drill down (planet becomes new sun) |

---

## 6. Spatial Parameters

| Parameter | Value | Location |
|-----------|-------|----------|
| Camera minDistance | 3 | `CameraController.tsx` |
| Camera maxDistance | 50 | `CameraController.tsx` |
| StarField radius | 200 | `UniverseCanvas.tsx` |
| Planet orbit baseRadius | 25 | `PhysicalStorageUniverse.tsx` |
| Moon orbit baseRadius | 8 | `PhysicalStorageUniverse.tsx` |
| Asteroid belt radius (root) | 40 | `PhysicalStorageUniverse.tsx` |
| Asteroid size threshold | 100KB | `types.ts:ASTEROID_SIZE_THRESHOLD` |

---

## 7. File Reference Map

| Topic | Primary File |
|-------|--------------|
| Type definitions & helpers | `src/components/universe/types.ts` |
| Scene composition | `src/components/universe/UniverseCanvas.tsx` |
| Planet (directory) rendering | `src/components/universe/nodes/DirectoryNode.tsx` |
| Satellite (file) rendering | `src/components/universe/nodes/FileNode.tsx` |
| Asteroid belt rendering | `src/components/universe/nodes/AsteroidBelt.tsx` |
| Orbit visualization | `src/components/universe/effects/OrbitLines.tsx` |
| Star field background | `src/components/universe/effects/StarField.tsx` |
| Camera controls | `src/components/universe/controls/CameraController.tsx` |
| Position calculations | `src/components/PhysicalStorageUniverse.tsx` |

---

## Related Documents

- [Product Design](../design/product-design.md) - Design rationale and specifications
- [Canvas Rendering](canvas-rendering.md) - 2D rendering details (if applicable)
