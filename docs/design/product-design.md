# CLOSM Probe Product Design

**Version**: 2.0
**Last Updated**: 2026-01-08
**Status**: Draft

> **Note**: For current implementation values (colors, radii, thresholds), see [Metaphor Mapping Reference](../specifications/metaphor-mapping.md). This document describes design rationale and concepts.

---

# Part I: The Universe Metaphor

The foundation of CLOSM Probe: representing storage as explorable space.

---

## 1. Vision: Storage as Space

### 1.1 Product Name

**CLOSM Probe** (pronounced "Closm Probe")

### 1.2 Core Concept

> Visualize storage as a 3D universe space and "explore" files in a new file management experience.

### 1.3 Mission Statement

Provide a next-generation storage interface that transcends traditional hierarchical file systems, allowing users to discover and organize files by intuitively exploring 3D space.

### 1.4 Why Space Metaphor Works

Traditional file systems suffer from fundamental limitations:

| Problem | Detail |
|---------|--------|
| 1D hierarchical structure | A file can only exist in one location |
| Capacity invisibility | Hard to intuitively see "what's eating space" |
| Lack of relationships | Cannot express semantic connections between files |
| Increasing exploration cost | Deeper hierarchies make finding files harder |
| Missing time axis | Difficult to distinguish old vs recent files |

The **Physical Space Metaphor** solves these by mapping to intuitive spatial concepts:

| Solution | Metaphor | Detail |
|----------|----------|--------|
| 3D space representation | Universe | Files placed as celestial bodies in space |
| Capacity visualization | Size | Larger planets = larger directories |
| Graph structure | Orbits & Edges | Express relationships via orbital paths |
| Intuitive navigation | Exploration | Drag and zoom to explore space |
| Time axis expression | Brightness | Recent = bright stars, old = dim |

---

## 2. Solar System Model

### 2.1 Hierarchical Mapping

The visualization follows a **solar system metaphor** with flat orbital planes:

```
        ☉ Sun (Current Root Directory)
       /|\
      / | \
     🪐 🪐 🪐  Planets (1st level - directories/files)
     |
    /|\
   🌙🌙🌙  Moons (2nd level - children of planets)
```

### 2.2 Element Mapping

| Storage Element | Celestial Body | Visual Representation |
|-----------------|----------------|----------------------|
| Current Root | Sun | Center, glowing, pulsing |
| 1st Level Directory | Planet | Orbits sun at radius 4 |
| 2nd Level Items | Moon | Orbits parent planet at radius 1.5 |
| File | Satellite | Smaller sphere, colored by type |

### 2.3 Key Principles

| Principle | Description |
|-----------|-------------|
| **2-Level Display** | Only show 2 levels from current "sun" for clarity |
| **Flat Orbital Plane** | All celestial bodies on Y=0 (like real solar system) |
| **Drill-Down Navigation** | Double-click a planet to make it the new sun |

### 2.4 State Transitions

```
[Root View]           [After Drill-Down]
    ☉ Sun                 ☉ New Sun (was Planet A)
   /|\                   /|\
  🪐🪐🪐                 🌙🌙🌙 (former children of Planet A)
  A B C
```

When drilling down:
1. Selected planet becomes the new sun (center)
2. Its children become the new planets
3. Grandchildren become the new moons
4. Breadcrumb trail shows navigation path

---

## 3. Visual Encoding System

Multiple dimensions encode information using **polar coordinates** for richer expression.

### 3.1 Polar Coordinate Mapping

| Polar Element | Attribute | Expression |
|---------------|-----------|------------|
| **r (radius)** | Creation date | New = close to sun, Old = outer orbit |
| **θ (angle)** | Last modified (sorted) | Equal spacing, sorted by recency (newest at 12 o'clock) |
| **Size** | File capacity | Larger file = larger body |
| **Color** | File type | Code=cyan, Image=orange, etc. |
| **Brightness** | Last modified | Recent = bright, old = dim |

### 3.2 Equal-Spaced Sorted Placement

Children are placed with **equal angular spacing**, sorted by last modified date:

```javascript
// Sort children by last modified (newest first)
const sortedChildren = [...children].sort((a, b) =>
  (b.lastModified || 0) - (a.lastModified || 0)
);

// Place with equal spacing, starting from 12 o'clock (-π/2)
sortedChildren.forEach((child, index) => {
  const angle = -Math.PI / 2 + (index / sortedChildren.length) * Math.PI * 2;
  // ... position calculation
});
```

This ensures:
- Beautiful equal-spaced arrangement
- Recently updated items start at top (12 o'clock direction)
- Clockwise progression toward older items

### 3.3 Orbital Radius by Creation Date

```javascript
// Sort by creation date, place newer items in inner orbits
const sortedByCreation = [...children].sort((a, b) =>
  (b.createdAt || 0) - (a.createdAt || 0)
);

// Assign orbit bands based on creation order
const orbitIndex = sortedByCreation.findIndex(c => c.id === child.id);
const orbitRadius = baseRadius * (0.6 + (orbitIndex / sortedByCreation.length) * 0.8);
```

### 3.4 Asteroid Belt Model

When a directory contains many small files, they are abstracted as an **asteroid belt** rather than individual nodes:

```
        ☉ Sun
       /|\
      🪐 🪐 🪐  Planets (directories)
       |
      ~~~  Asteroid Belt (small files consolidated)
       |
      🛰️ 🛰️  Large Satellites (significant files only)
```

**Classification Rules:**

| Category | Criteria | Display |
|----------|----------|---------|
| Planet | Directory | Individual sphere |
| Large Satellite | File > threshold size | Individual sphere |
| Asteroid | File ≤ threshold | Consolidated into belt ring |

**Asteroid Belt Behavior:**
- Displayed as a dotted ring around parent
- Shows count badge (e.g., "+12 files")
- Click to expand into individual view
- Reduces visual clutter while preserving information

### 3.5 Size Encoding (Capacity)

Node size represents storage capacity using logarithmic scale:

| Type | Base Size | Scale Factor |
|------|-----------|--------------|
| Planet (directory) | 15px | log10(size) × 3 |
| Satellite (file) | 8px | log10(size) × 2 |

### 3.6 Color Encoding (File Type)

| File Type | Color | Hex Code |
|-----------|-------|----------|
| Code | Cyan | #61dafb |
| Design | Purple | #a855f7 |
| Image | Orange | #f59e0b |
| Video | Red | #ef4444 |
| PDF | Dark Red | #dc2626 |
| Document | Blue | #3b82f6 |
| Data | Teal | #06b6d4 |
| Archive | Gray | #6b7280 |
| Directory | Violet | #8b5cf6 |

### 3.7 Brightness Encoding (Time/Recency)

| Last Modified | Brightness | Visual Effect |
|---------------|------------|---------------|
| Within 24h | 100% | Max brightness + pulse animation |
| Within 1 week | 85% | High brightness |
| Within 1 month | 70% | Medium brightness |
| Within 3 months | 55% | Low brightness |
| Within 1 year | 40% | Dim |
| Over 1 year | 25% | Minimum + grayed out |

### 3.8 UI Color Palette

| Usage | Color | Hex |
|-------|-------|-----|
| Background (deep space) | Dark Navy | #0a0a1a |
| Background (shallow space) | Dark Blue | #1a1a2e |
| Accent (primary) | Purple | #a855f7 |
| Accent (secondary) | Blue | #3b82f6 |
| Text (main) | White | #ffffff |
| Text (sub) | Gray | #888888 |
| Success | Green | #22c55e |
| Warning | Orange | #f59e0b |
| Error | Red | #ef4444 |

---

# Part II: Navigating the Universe

How users explore and interact with the storage universe.

---

## 4. Exploration Mechanics

### 4.1 View Manipulation

| Operation | Action | Notes |
|-----------|--------|-------|
| Drag | Rotate view | Orbit around the sun |
| Scroll | Zoom in/out | Get closer or farther from planets |
| Right-drag | Pan | Parallel movement |
| Pinch (touch) | Zoom | Mobile gesture support |

### 4.2 Navigation Actions

| Action | Result |
|--------|--------|
| Double-click planet | Drill down (planet becomes new sun) |
| Breadcrumb click | Navigate back to that level |
| Double-click sun | Go up one level |

### 4.3 Rotation Limits

- X-axis rotation: -π/2 to π/2 (prevent flipping)
- Y-axis rotation: Unlimited (full orbit)
- Zoom range: 0.5x to 2.0x

---

## 5. Discovery & Selection

### 5.1 Selection Mechanics

| Action | Result |
|--------|--------|
| Single click | Select celestial body, show details |
| Hover | Highlight with glow effect |
| Click empty space | Deselect |

### 5.2 Detail Panel Information

When a celestial body is selected:
- File/folder name
- Path (breadcrumb format)
- Size (bytes + percentage of parent)
- Last modified date
- File type
- Related files list (future)

### 5.3 Search & Focus

1. Enter keyword in search bar
2. Matching celestial bodies highlight
3. Camera animates to focus on first match
4. Navigate between matches with arrows

---

## 6. Information Architecture

### 6.1 Screen Layout

```
┌─────────────────────────────────────────────────────────────┐
│  Header: CLOSM Probe Logo | Search Bar | Settings | User    │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────┐ ┌─────────────────────────────┐ ┌───────────┐  │
│ │         │ │                             │ │           │  │
│ │ Sidebar │ │                             │ │  Detail   │  │
│ │         │ │                             │ │  Panel    │  │
│ │ - Overview│ │      3D Universe Canvas    │ │           │  │
│ │ - Folders│ │                             │ │ - Selected│  │
│ │ - Filters│ │                             │ │   Body    │  │
│ │ - Tags  │ │                             │ │ - Details │  │
│ │         │ │                             │ │ - Related │  │
│ │         │ │                             │ │           │  │
│ └─────────┘ └─────────────────────────────┘ └───────────┘  │
├─────────────────────────────────────────────────────────────┤
│  Footer: Operation Hints | Zoom Level | Body Count          │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Design Principles

| Principle | Detail |
|-----------|--------|
| Dark mode first | Optimal for space representation |
| Minimal | Avoid information overload, show only necessary info |
| Intuitive operation | Natural mouse/touch interaction |
| Responsive | Desktop/tablet/mobile support |
| Accessibility | Keyboard operation, screen reader support |

---

# Part III: Universe Features

Capabilities available within the storage universe.

---

## 7. Core Capabilities

### 7.1 Capacity Analysis

- Total capacity summary display
- Planet/satellite count per level
- Highlight top capacity celestial bodies
- Progress bar showing percentage of total

### 7.2 Filtering

| Filter Type | Options |
|-------------|---------|
| File type | Code, image, video, document, etc. |
| Update date | Within 24h, week, month, year, older |
| Size | Custom min/max range |
| Tags/Labels | User-defined categories |

### 7.3 Visualization Modes

| Mode | Description |
|------|-------------|
| Type view | Color by file type (default) |
| Age view | Color by recency |
| Size view | Color by capacity |

---

## 8. Extended Capabilities (Future)

### 8.1 AI Relationship Analysis

- Analyze file contents via Claude API
- Auto-generate edges between semantically similar files
- Cluster similar satellites into constellations
- Related file recommendations

### 8.2 Physics Simulation

- Gravity model (related files attract)
- Auto-layout optimization
- Natural "galaxy" formation over time

### 8.3 File Operations

- Drag & drop file movement between planets
- Multi-select bulk deletion
- Create new folders (spawn new planets)

### 8.4 Anomaly Detection

- Highlight rapidly growing planets in red
- Gray out long-untouched satellites
- Detect duplicate files

---

# Part IV: Building the Universe

Technical implementation of the storage universe.

---

## 9. Rendering Architecture

### 9.1 Rendering Technologies

| Technology | Purpose | Notes |
|------------|---------|-------|
| Canvas 2D | Initial lightweight 3D | Current implementation |
| Three.js | Full 3D rendering | WebGL-based |
| React Three Fiber | Three.js + React | Declarative approach |

### 9.2 Performance Strategies

| Strategy | Description |
|----------|-------------|
| Level of Detail (LOD) | Reduce detail for distant bodies |
| Virtualization | Hide off-screen celestial bodies |
| Web Workers | Offload position calculations |
| Batch rendering | Group canvas operations |

### 9.3 Detailed Specifications

See [Canvas Rendering Specification](../specifications/canvas-rendering.md) for:
- 3D projection formulas
- Gradient effects
- Animation system
- Hit detection

---

## 10. Data Architecture

### 10.1 CelestialBody (FileNode)

```typescript
interface CelestialBody {
  id: string;
  name: string;
  type: 'planet' | 'satellite';  // directory or file
  path: string;
  size: number;              // bytes
  fileType?: string;         // code, image, video, etc.
  mimeType?: string;
  createdAt: Date;
  updatedAt: Date;
  accessedAt?: Date;

  // 3D space coordinates
  position: {
    x: number;
    y: number;
    z: number;
  };

  // Orbital relationship
  parentId: string | null;
  children?: CelestialBody[];

  // Metadata
  tags?: string[];
  color?: string;
  thumbnail?: string;
}
```

### 10.2 OrbitEdge (FileEdge)

```typescript
interface OrbitEdge {
  id: string;
  sourceId: string;
  targetId: string;
  relationType: 'orbital' | 'semantic' | 'reference' | 'duplicate';
  strength: number;          // 0.0 - 1.0
  createdAt: Date;
  metadata?: {
    reason?: string;         // For AI-generated edges
    confidence?: number;
  };
}
```

### 10.3 ViewConfig

```typescript
interface ViewConfig {
  rotation: { x: number; y: number };
  zoom: number;
  center: { x: number; y: number; z: number };
  currentSunId: string;      // Current root (sun) ID
  breadcrumb: string[];      // Navigation path
  filters: {
    fileTypes?: string[];
    dateRange?: { start: Date; end: Date };
    sizeRange?: { min: number; max: number };
    tags?: string[];
  };
  displayOptions: {
    showLabels: boolean;
    showOrbits: boolean;
    colorScheme: 'type' | 'age' | 'size';
  };
}
```

---

## 11. System Architecture

### 11.1 System Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      CLOSM Probe                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   Frontend  │    │   Backend   │    │  External   │     │
│  │             │    │             │    │  Services   │     │
│  ├─────────────┤    ├─────────────┤    ├─────────────┤     │
│  │ React       │◄──►│ Node.js /   │◄──►│ Google      │     │
│  │ Three.js /  │    │ Python      │    │ Drive API   │     │
│  │ Canvas 2D   │    │             │    │             │     │
│  │             │    │ REST API    │    │ Claude API  │     │
│  │ Tailwind    │    │             │    │             │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│         │                  │                  │             │
│         ▼                  ▼                  ▼             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                    Data Layer                        │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐        │   │
│  │  │ IndexedDB │  │ SQLite /  │  │ Neo4j     │        │   │
│  │  │ (Cache)   │  │ PostgreSQL│  │ (Graph)   │        │   │
│  │  └───────────┘  └───────────┘  └───────────┘        │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 11.2 Tech Stack

#### Frontend

| Technology | Purpose | Notes |
|------------|---------|-------|
| React | UI framework | Next.js |
| Canvas 2D | 3D rendering (lightweight) | Initial implementation |
| Three.js | 3D rendering (full) | WebGL |
| React Three Fiber | Three.js + React integration | Declarative |
| Tailwind CSS | Styling | |

#### Backend

| Technology | Purpose | Notes |
|------------|---------|-------|
| Node.js | API server | Express / Fastify |
| Python | Analysis processing | FastAPI |
| GraphQL | API design | Flexible queries |

#### Data

| Technology | Purpose | Notes |
|------------|---------|-------|
| IndexedDB | Browser cache | Offline support |
| SQLite | Local DB | Electron version |
| PostgreSQL | Cloud DB | SaaS version |
| Neo4j | Graph DB | Relationship storage |

---

## 12. Platform Strategy

| Platform | Technology | Priority | Notes |
|----------|------------|----------|-------|
| Web (SPA) | Next.js | P0 | Primary target |
| Desktop (Mac/Win/Linux) | Electron / Tauri | P1 | Local file system |
| Mobile (iOS/Android) | React Native | P2 | Touch-first |
| VR/AR | WebXR | P3 | Immersive exploration |

---

# Part V: Bringing the Universe to Users

Product strategy and go-to-market approach.

---

## 13. Target Users & Use Cases

### 13.1 Target Users

- Creators/engineers handling large amounts of files
- Users seeking storage capacity visualization and optimization
- Early adopters seeking new file management experiences

### 13.2 Use Case: Storage Capacity Audit

**Persona**: User with "Google Drive is full"

**Flow**:
1. Log in to CLOSM Probe
2. Find the largest planet (folder) in the universe
3. Click to see its moons (contents)
4. Identify massive satellites (files)
5. Delete unnecessary files

**Value**: Visually identify the culprit of capacity usage

### 13.3 Use Case: Finding Related Files

**Persona**: "Where are all files related to that project?"

**Flow**:
1. Enter project name in search bar
2. Camera focuses on matching celestial body
3. Discover surrounding constellation (related files)
4. Understand relationships beyond folder hierarchy

**Value**: Discover scattered files in one place

### 13.4 Use Case: Cleaning Old Files

**Persona**: "I want to clean up files untouched for over a year"

**Flow**:
1. Select "Over 1 year" in filter
2. Dim (low brightness) bodies appear
3. Multi-select and bulk delete
4. Storage capacity recovers

**Value**: Easy filtering by time axis

### 13.5 Use Case: Understanding Project Structure

**Persona**: "What's the structure of this project I just joined?"

**Flow**:
1. Select project folder (planet)
2. Get bird's eye view of the solar system
3. Understand main planets (folder) layout
4. Understand satellite type distribution by color

**Value**: Intuitively understand project structure

---

## 14. Development Roadmap

### Phase 1: MVP (Complete)

**Goal**: Basic 3D universe visualizer

- [x] 3D space celestial body display
- [x] Capacity visualization (body size)
- [x] Drag rotation/zoom
- [x] Body selection/detail panel
- [ ] Sample data demo

**Deliverable**: Web-based prototype

### Phase 2: Storage Integration

**Goal**: Real data integration

- [ ] Google Drive API integration
- [ ] Real-time file structure retrieval
- [ ] Diff detection (new/delete/modify)
- [ ] Local cache (IndexedDB)
- [ ] Browser auth flow

**Deliverable**: Google Drive integrated version

### Phase 3: Analysis & Filtering

**Goal**: Advanced analysis features

- [ ] File type filter
- [ ] Date filter
- [ ] Size filter
- [ ] Search → zoom
- [ ] Top capacity report
- [ ] Duplicate detection

**Deliverable**: Analysis-enabled version

### Phase 4: AI Relationship Analysis

**Goal**: Semantic relationship visualization

- [ ] Claude API integration
- [ ] File content analysis
- [ ] Auto semantic edge generation
- [ ] Constellation clustering
- [ ] Related file recommendations

**Deliverable**: AI analysis version

### Phase 5: Physics Simulation

**Goal**: Natural layout

- [ ] Gravity model implementation
- [ ] Auto-layout optimization
- [ ] Animation enhancement
- [ ] Performance optimization

**Deliverable**: Physics simulation version

### Phase 6: Desktop App

**Goal**: Native app

- [ ] Electron / Tauri support
- [ ] Local file system support
- [ ] Offline operation
- [ ] System tray
- [ ] Auto-start option

**Deliverable**: Desktop application

### Phase 7: VR/AR (Exploration)

**Goal**: Immersive experience

- [ ] WebXR support
- [ ] Hand tracking
- [ ] Spatial audio
- [ ] "Walk through" 3D space

**Deliverable**: VR/AR version (experimental)

---

## 15. Market Position

### 15.1 Competitive Analysis

| Product | Features | CLOSM Probe Differentiation |
|---------|----------|----------------------------|
| Finder / Explorer | Tree structure, 1D | 3D space + relationships |
| WinDirStat / GrandPerspective | 2D treemap, capacity only | 3D + time axis + relationships |
| Notion | Block-based but not spatial | Spatial navigation |
| Obsidian Graph View | Note relationship visualization | Targets entire file system |
| Dropbox / Google Drive | Cloud storage | Visualization/analysis features |

### 15.2 Key Differentiators

1. **3D space representation**: Intuitive visualization beyond 2D
2. **Multi-axis encoding**: Capacity + Time + Relationships simultaneously
3. **AI relationship analysis**: Auto-detect semantic connections
4. **Exploration paradigm**: Game-like universe exploration

---

## 16. Success Metrics (KPI)

### 16.1 User Acquisition

| Metric | Target (6 months) |
|--------|-------------------|
| Registered users | 1,000 |
| MAU | 300 |
| DAU | 50 |

### 16.2 Engagement

| Metric | Target |
|--------|--------|
| Avg session duration | 5+ minutes |
| Sessions per week | 3+ |
| Feature usage (search) | 60%+ |

### 16.3 Business

| Metric | Target |
|--------|--------|
| Paid conversion rate | 5% |
| NPS | 40+ |
| Churn rate | Under 5% |

---

# Appendices

---

## A. Mathematical Formulas

### A.1 3D Projection Formula

```javascript
// Y-axis rotation → X-axis rotation → Perspective projection
const project = (x, y, z, rotation, scale, center) => {
  const cosX = Math.cos(rotation.x);
  const sinX = Math.sin(rotation.x);
  const cosY = Math.cos(rotation.y);
  const sinY = Math.sin(rotation.y);

  // Y-axis rotation
  const x1 = x * cosY - z * sinY;
  const z1 = x * sinY + z * cosY;

  // X-axis rotation
  const y1 = y * cosX - z1 * sinX;
  const z2 = y * sinX + z1 * cosX;

  // Perspective projection
  const perspective = 8;
  const factor = perspective / (perspective + z2);

  return {
    screenX: center.x + x1 * scale * factor,
    screenY: center.y - y1 * scale * factor,
    depth: z2,
    scale: factor
  };
};
```

### A.2 Logarithmic Size Scale

```javascript
// Log scale to make huge files reasonable size
const calculateBodyRadius = (size, type) => {
  const baseSize = type === 'planet' ? 15 : 8;
  const scaleFactor = type === 'planet' ? 3 : 2;
  const minSize = type === 'planet' ? 1000 : 100;

  return baseSize + Math.log10(Math.max(size, minSize)) * scaleFactor;
};
```

### A.3 Brightness Calculation (Time-Based)

```javascript
// Calculate brightness from last modified date
const calculateBrightness = (lastModified) => {
  const now = Date.now();
  const age = now - lastModified;
  const dayInMs = 24 * 60 * 60 * 1000;

  if (age < dayInMs) return 1.0;           // Within 24h
  if (age < 7 * dayInMs) return 0.85;      // Within 1 week
  if (age < 30 * dayInMs) return 0.7;      // Within 1 month
  if (age < 90 * dayInMs) return 0.55;     // Within 3 months
  if (age < 365 * dayInMs) return 0.4;     // Within 1 year
  return 0.25;                              // Over 1 year
};
```

---

## B. Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Performance issues (large files) | UX degradation | LOD implementation, virtualization, Web Worker |
| Google API limits | Feature restriction | Caching, diff retrieval, rate limit handling |
| 3D operation learning curve | Drop-off | Tutorial, 2D mode option |
| Trademark issues | Legal risk | Pre-investigation by patent attorney |
| Security (file access) | Trust | OAuth2.0, principle of least privilege |

---

## C. References

- NASA Voyager Program: https://science.nasa.gov/mission/voyager/
- Three.js Documentation: https://threejs.org/docs/
- Google Drive API: https://developers.google.com/drive
- Claude API: https://docs.anthropic.com/
- Neo4j Graph Database: https://neo4j.com/docs/
