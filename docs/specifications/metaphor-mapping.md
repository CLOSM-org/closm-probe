# Metaphor Mapping Reference

Single source of truth for directory ↔ universe metaphor mappings in CLOSM Probe.

**Column Key:**
- **Metaphor**: Astronomical concept
- **Product Mapping**: What it represents in the product
- **Details**: Specs, constraints, rationale, TBD items
- **Implementation**: File path + function

---

## 1. Size Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Sun | Current root directory | Fixed center point at (0,0,0), not scaled by size | `PhysicalStorageUniverse.tsx` |
| Planet radius | Directory size | **Scale**: Logarithmic (log10)<br>**Base**: 0.8<br>**Factor**: 0.15<br>**Min clamp**: 1000B<br>**TBD**: Max radius limit, overlap prevention | `types.ts:calculateNodeRadius()` |
| Satellite radius | File size | **Scale**: Logarithmic (log10)<br>**Base**: 0.4<br>**Factor**: 0.08<br>**Min clamp**: 100B<br>**TBD**: Max radius limit | `types.ts:calculateNodeRadius()` |
| Asteroid | Small file (<100KB) | **Threshold**: 100,000 bytes<br>Clustered in belt rings rather than individual orbits | `types.ts:ASTEROID_SIZE_THRESHOLD` |

---

## 2. Position Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Planet orbit (depth 1) | Direct child of root | **Base radius**: 25 units<br>**Range**: 60%~140% of base (15-35 units)<br>Solid orbit line | `PhysicalStorageUniverse.tsx` |
| Moon orbit (depth 2+) | Subdirectory/nested item | **Base radius**: 8 units<br>**Range**: 60%~140% of base (4.8-11.2 units)<br>Dashed orbit line | `PhysicalStorageUniverse.tsx` |
| Asteroid belt (root) | Small files at root level | **Radius**: 40 units<br>Files <100KB collected into ring | `PhysicalStorageUniverse.tsx` |
| Asteroid belt (nested) | Small files in subdirectory | Orbits parent directory position | `PhysicalStorageUniverse.tsx` |
| Star field | Background decoration | **Radius**: 120 units<br>**Count**: 1500 particles<br>No mapping to storage data | `UniverseCanvas.tsx` |

---

## 3. Time Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Orbital radius | Creation date (createdAt) | **Rule**: Newer = inner orbit, Older = outer orbit<br>**Range**: 60%~140% of baseRadius<br>**Sort**: By createdAt timestamp | `types.ts:calculateOrbitRadiusByOrder()` |
| Angular position | Last modified date | **Rule**: Most recent at 12 o'clock, clockwise by age<br>**Start**: -π/2 (top)<br>**Direction**: Clockwise | `types.ts:calculateEqualSpacedAngle()` |
| Brightness/Glow | Recency (time since last modified) | **Formula**: max(0.3, 1 - months*0.1)<br>**Range**: 0.3 (old) to 1.0 (recent) | `types.ts:calculateBrightness()` |

---

## 4. Visual Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Celestial body color | File type | See Appendix A for color palette | `types.ts:typeColors` |
| Emissive glow (directory) | Interaction state | **Normal**: 0.6<br>**Hover**: 0.8<br>**Selected**: 1.0 | `DirectoryNode.tsx` |
| Emissive glow (file) | Interaction state | Similar to directory | `FileNode.tsx` |
| Orbit line (depth 1) | Primary navigation level | **Style**: Solid<br>**Color**: #888888<br>**Opacity**: 0.5 | `OrbitLines.tsx` |
| Orbit line (depth 2+) | Secondary level | **Style**: Dashed<br>**Color**: #555555<br>**Opacity**: 0.3 | `OrbitLines.tsx` |
| Bloom effect | Glow enhancement | **Intensity**: 0.7<br>**Luminance threshold**: 0.15 | `Effects.tsx` |

---

## 5. Interaction Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Orbit around sun (drag) | Camera rotation | OrbitControls with damping | `CameraController.tsx` |
| Travel toward/away (scroll) | Zoom in/out | **Min distance**: 3<br>**Max distance**: 80 | `CameraController.tsx` |
| Click celestial body | Select item | Shows details panel, no camera movement | `DirectoryNode.tsx`, `FileNode.tsx` |
| Double-click directory | Drill down | Planet becomes new sun (context switch), camera resets to overview | `DirectoryNode.tsx` |
| Double-click file | Approach celestial body | Camera moves to file position + offset | `CameraController.tsx:focusOn()` |
| Return to observatory | Reset camera view | Returns camera to initial [0, 12, 22] looking at center | `CameraController.tsx:resetView()` |

---

## Appendix A: Color Palette

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

## Appendix B: Spatial Parameters

| Parameter | Value | Location |
|-----------|-------|----------|
| Camera initial position | [0, 12, 22] | `UniverseCanvas.tsx`, `CameraController.tsx:INITIAL_CAMERA_POSITION` |
| Camera FOV | 55 | `UniverseCanvas.tsx` |
| Camera minDistance | 3 | `CameraController.tsx` |
| Camera maxDistance | 80 | `CameraController.tsx` |
| Reset view position | [0, 12, 22] | `CameraController.tsx:INITIAL_CAMERA_POSITION` |
| Reset view lookAt | [0, 0, 0] | `CameraController.tsx:INITIAL_LOOK_AT` |
| StarField radius | 120 | `UniverseCanvas.tsx` |
| StarField count | 1500 | `UniverseCanvas.tsx` |
| Planet orbit baseRadius | 25 | `PhysicalStorageUniverse.tsx` |
| Moon orbit baseRadius | 8 | `PhysicalStorageUniverse.tsx` |
| Asteroid belt radius (root) | 40 | `PhysicalStorageUniverse.tsx` |
| Asteroid size threshold | 100KB | `types.ts` |

---

## Appendix C: File Reference Map

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
| Post-processing effects | `src/components/universe/postprocessing/Effects.tsx` |

---

## TBD Items (Undefined/Under Discussion)

| Topic | Question | Current Status |
|-------|----------|----------------|
| Max planet radius | What's the upper limit to prevent overlap? | Not constrained |
| Radius vs Volume | Should size map to radius or volume (r^3)? | Currently radius |
| Sun scale | Should sun size reflect root directory size? | Fixed, not scaled |
| Overlap prevention | How to handle planets that would overlap? | Relies on orbital spacing |
| Asteroid density | Max asteroids per belt before performance degrades? | Not limited |
| Satellite definition | Does "satellite" mean "file only" or "all child elements"? | Needs discussion |

---

## Related Documents

- [Product Design](../design/product-design.md) - Design rationale and specifications
- [Canvas Rendering](canvas-rendering.md) - 2D rendering details (if applicable)
