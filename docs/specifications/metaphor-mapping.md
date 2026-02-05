# Metaphor Mapping Reference

Single source of truth for directory ↔ universe metaphor mappings in CLOSM Probe.

**Column Key:**
- **Metaphor**: Astronomical concept
- **Product Mapping**: What it represents in the product
- **Details**: Specs, constraints, rationale
- **Implementation**: File path + function

---

## 1. Celestial Body Definitions

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Star (恒星) | Currently open folder | Center fixed at (0,0,0), strong emission, no ring | `PhysicalStorageUniverse.tsx` |
| Planet - Directory (惑星) | Direct child folder of star | White glowing sphere + ring (random tilt), ring thickness = child count | `DirectoryNode.tsx` |
| Planet - File (惑星) | Direct child file of star | Octahedron shape, type-based color | `FileNode.tsx` |
| Satellite (衛星) | Children of planet | Abstracted as ring (not individually rendered) | Ring on `DirectoryNode.tsx` |
| Asteroid Belt | Overflow items (21st+) | Collected when planets exceed 20 | `AsteroidBelt.tsx` |

---

## 2. Size Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Star radius | Current folder | **Fixed**: 2.5 (always largest) | `types.ts:STAR_RADIUS` |
| Directory planet radius | Directory size | **Range**: 0.5 ~ 2.0<br>**Scale**: log10 interpolation<br>**Size range**: 1KB ~ 10GB | `types.ts:calculateNodeRadius()` |
| File planet radius | File size | **Range**: 0.3 ~ 1.8 (~6x difference)<br>**Scale**: log10 interpolation<br>**Size range**: 100B ~ 1GB | `types.ts:calculateNodeRadius()` |
| Ring thickness | Child count | Thicker ring = more children | `DirectoryNode.tsx` |

---

## 3. Position Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Planet orbit | Child elements of star | **Base radius**: 25 units<br>**Range**: 60%~140% of base (15-35 units) | `PhysicalStorageUniverse.tsx` |
| Asteroid belt | Overflow elements (21st+) | **Radius**: 40 units<br>Elements beyond 20 collected into ring | `PhysicalStorageUniverse.tsx` |
| Star field | Background decoration | **Radius**: 120 units<br>**Count**: 1500 particles<br>No mapping to storage data | `UniverseCanvas.tsx` |

---

## 4. Time Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Orbital radius | Creation date (createdAt) | **Rule**: Newer = inner orbit, Older = outer orbit<br>**Range**: 60%~140% of baseRadius<br>**Sort**: By createdAt timestamp | `types.ts:calculateOrbitRadiusByOrder()` |
| Angular position | Last modified date | **Rule**: Most recent at 12 o'clock, clockwise by age<br>**Start**: -π/2 (top)<br>**Direction**: Clockwise | `types.ts:calculateEqualSpacedAngle()` |
| Brightness/Glow | Recency (time since last modified) | **Formula**: max(0.3, 1 - months*0.1)<br>**Range**: 0.3 (old) to 1.0 (recent) | `types.ts:calculateBrightness()` |

---

## 5. Visual Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Star color | Current folder | **Color**: White (#ffffff)<br>**Emission**: High (1.5+) | `DirectoryNode.tsx` (depth=0) |
| Directory planet color | Folder | **Color**: White (#ffffff)<br>**Emission**: None (0) | `DirectoryNode.tsx` |
| File planet color | File type | See Appendix A for color palette | `types.ts:typeColors` |
| File planet shape | File (leaf node) | **Shape**: Octahedron (8-sided) | `FileNode.tsx` |
| Directory ring | Has children | **Tilt**: Random<br>**Thickness**: Proportional to child count | `DirectoryNode.tsx` |
| Orbit line | Planet path | **Style**: Solid<br>**Color**: #888888<br>**Opacity**: 0.5 | `OrbitLines.tsx` |
| Bloom effect | Glow enhancement | **Intensity**: 0.7<br>**Luminance threshold**: 0.15 | `Effects.tsx` |

---

## 6. Display Control

| Attribute | Value | Details | Implementation |
|-----------|-------|---------|----------------|
| Max depth | 1 | Only star (depth 0) and planets (depth 1) displayed<br>Satellites (depth 2+) abstracted as rings | `PhysicalStorageUniverse.tsx` |
| Max planets | 20 | Beyond 20, items go to asteroid belt | `PhysicalStorageUniverse.tsx` |
| Selection method | Sort/Filter | User controls which 20 to display | TBD: UI component |
| Overflow handling | Asteroid belt | 21st+ items collected in belt | `AsteroidBelt.tsx` |
| Satellite preview | Hover expansion | Directory hover shows up to 12 satellites with animation<br>Aligned with ring rotation axis | `DirectoryNode.tsx` |

---

## 7. Interaction Mappings

| Metaphor | Product Mapping | Details | Implementation |
|----------|-----------------|---------|----------------|
| Orbit around star (drag) | Camera rotation | OrbitControls with damping | `CameraController.tsx` |
| Travel toward/away (scroll) | Zoom in/out | **Min distance**: 3<br>**Max distance**: 80 | `CameraController.tsx` |
| Hover directory planet | Satellite preview | Shows up to 12 children with scale animation<br>Aligned with ring tilt | `DirectoryNode.tsx` |
| Click celestial body | Select item | Shows details panel + focus button | `DirectoryNode.tsx`, `FileNode.tsx` |
| Focus button click | Camera zoom | Camera moves to celestial body position | `CameraController.tsx:focusOn()` |
| Double-click directory planet | Drill down | Planet becomes new star (context switch), camera resets | `DirectoryNode.tsx` |
| Double-click file planet | Approach | Camera moves to file position + offset | `CameraController.tsx:focusOn()` |
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
| Directory | White | `#ffffff` |

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
| Asteroid belt radius | 40 | `PhysicalStorageUniverse.tsx` |
| Max displayed planets | 20 | `PhysicalStorageUniverse.tsx` |
| Max depth | 1 | `PhysicalStorageUniverse.tsx` |
| Satellite preview count | 12 | `DirectoryNode.tsx` |

---

## Appendix C: File Reference Map

| Topic | Primary File |
|-------|--------------|
| Type definitions & helpers | `src/components/universe/types.ts` |
| Scene composition | `src/components/universe/UniverseCanvas.tsx` |
| Directory planet rendering | `src/components/universe/nodes/DirectoryNode.tsx` |
| File planet rendering | `src/components/universe/nodes/FileNode.tsx` |
| Asteroid belt rendering | `src/components/universe/nodes/AsteroidBelt.tsx` |
| Orbit visualization | `src/components/universe/effects/OrbitLines.tsx` |
| Star field background | `src/components/universe/effects/StarField.tsx` |
| Camera controls | `src/components/universe/controls/CameraController.tsx` |
| Position calculations | `src/components/PhysicalStorageUniverse.tsx` |
| Post-processing effects | `src/components/universe/postprocessing/Effects.tsx` |

---

## Appendix D: Terminology

| Term | Japanese | Definition |
|------|----------|------------|
| Star | 恒星 | Currently open folder (center of view) |
| Planet | 惑星 | Direct child of star (file or directory) |
| Satellite | 衛星 | Child of planet (abstracted as ring) |
| Asteroid | 小惑星 | Overflow items in belt |
| Ring | リング | Visual indicator of children on directory planet |

---

## Related Documents

- [Product Design](../design/product-design.md) - Design rationale and specifications
- [Canvas Rendering](canvas-rendering.md) - 2D rendering details (if applicable)
