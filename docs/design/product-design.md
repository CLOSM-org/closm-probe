# CLOSM Probe Product Design

**Version**: 1.0
**Last Updated**: 2026-01-08
**Status**: Draft

---

## 1. Product Overview

### 1.1 Product Name

**CLOSM Probe** (pronounced "Closm Probe")

### 1.2 Concept

> Visualize storage as a 3D universe space and "explore" files in a new file management experience.

### 1.3 Mission Statement

Provide a next-generation storage interface that transcends traditional hierarchical file systems, allowing users to discover and organize files by intuitively exploring 3D space.

### 1.4 Target Users

- Creators/engineers handling large amounts of files
- Users seeking storage capacity visualization and optimization
- Early adopters seeking new file management experiences

---

## 2. Problems We Solve

### 2.1 Traditional File System Issues

| Issue | Detail |
|-------|--------|
| 1D hierarchical structure | A file can only exist in one location |
| Capacity invisibility | Hard to intuitively see "what's eating space" |
| Lack of relationships | Cannot express semantic connections between files |
| Increasing exploration cost | Deeper hierarchies make finding files harder |
| Missing time axis | Difficult to distinguish old vs recent files |

### 2.2 CLOSM Probe Solutions

| Solution | Detail |
|----------|--------|
| 3D space representation | Files placed as nodes in universe space |
| Capacity visualization | Node size = file/folder capacity |
| Graph structure | Express relationships via edges |
| Intuitive navigation | Drag and zoom to explore space |
| Time axis expression | Update frequency shown via brightness/color |

---

## 3. Physical Space Metaphor

### 3.1 Solar System Model

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

**Key Principles:**
- **2-Level Display**: Only show 2 levels from current "sun" for clarity
- **Flat Orbital Plane**: All nodes on Y=0 (like real solar system)
- **Drill-Down Navigation**: Double-click a planet to make it the new sun

### 3.2 Basic Mapping

| Element | Metaphor | Visual |
|---------|----------|--------|
| Current Root | Sun | Center, glowing |
| 1st Level Directory | Planet | Orbits sun at radius 4 |
| 2nd Level Items | Moon | Orbits parent planet at radius 1.5 |
| File | Satellite | Smaller sphere, color by type |

### 3.3 Navigation

| Action | Result |
|--------|--------|
| Double-click directory | Drill down (directory becomes new sun) |
| Breadcrumb click | Navigate back to that level |
| Single click | Select and show details |
| Drag | Rotate view |
| Scroll | Zoom in/out |

### 3.4 File Type Colors

| Type | Color | Hex |
|------|-------|-----|
| code | Cyan | #61dafb |
| design | Purple | #a855f7 |
| image | Orange | #f59e0b |
| video | Red | #ef4444 |
| pdf | Dark Red | #dc2626 |
| doc | Blue | #3b82f6 |
| data | Teal | #06b6d4 |
| archive | Gray | #6b7280 |
| directory | Violet | #8b5cf6 |

### 3.5 Time Axis Expression

| Update Time | Visual Representation |
|-------------|----------------------|
| Within 24h | Max brightness + pulse animation |
| Within 1 week | High brightness |
| Within 1 month | Medium brightness |
| Over 3 months | Low brightness (dark) |
| Over 1 year | Minimum brightness + grayed out |

---

## 4. Feature Specifications

### 4.1 Core Features

#### 4.1.1 3D Navigation

| Operation | Action |
|-----------|--------|
| Drag | Rotate view |
| Scroll | Zoom in/out |
| Double-click | Focus on selected node |
| Right-drag | Pan (parallel movement) |
| Pinch (touch) | Zoom |

#### 4.1.2 Node Selection & Detail Display

Information displayed on selection:
- File/folder name
- Path (breadcrumbs)
- Size (bytes + percentage of total)
- Last modified date
- File type
- Related files list

#### 4.1.3 Capacity Analysis

- Total capacity summary display
- Directory/file count
- Highlight top capacity nodes
- Progress bar showing percentage of total

#### 4.1.4 Filtering

- Filter by file type
- Filter by update date
- Filter by size
- Filter by tags/labels

#### 4.1.5 Search → Focus

- Keyword search
- Highlight matching nodes
- Camera zooms to matching node

### 4.2 Extended Features (Future)

#### 4.2.1 AI Relationship Analysis

- Analyze file contents via Claude API
- Auto-generate edges between semantically similar files
- Cluster similar files

#### 4.2.2 Physics Simulation

- Gravity model (related files attract)
- Auto-layout optimization
- Natural "galaxy" formation

#### 4.2.3 File Operations

- Drag & drop file movement
- Multi-select bulk deletion
- Create new folders

#### 4.2.4 Anomaly Detection

- Highlight rapidly growing folders in red
- Gray out long-untouched files
- Detect duplicate files

---

## 5. Technical Architecture

### 5.1 System Diagram

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

### 5.2 Tech Stack

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

### 5.3 Platform Deployment

| Platform | Technology | Priority |
|----------|------------|----------|
| Web (SPA) | Next.js | P0 |
| Desktop (Mac/Win/Linux) | Electron / Tauri | P1 |
| Mobile (iOS/Android) | React Native | P2 |
| VR/AR | WebXR | P3 |

---

## 6. Data Models

### 6.1 FileNode

```typescript
interface FileNode {
  id: string;
  name: string;
  type: 'file' | 'directory';
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

  // Parent-child relationship
  parentId: string | null;
  children?: FileNode[];

  // Metadata
  tags?: string[];
  color?: string;
  thumbnail?: string;
}
```

### 6.2 FileEdge

```typescript
interface FileEdge {
  id: string;
  sourceId: string;
  targetId: string;
  relationType: 'parent-child' | 'semantic' | 'reference' | 'duplicate';
  strength: number;          // 0.0 - 1.0
  createdAt: Date;
  metadata?: {
    reason?: string;         // Reason (for AI-generated edges)
    confidence?: number;
  };
}
```

### 6.3 ViewConfig

```typescript
interface ViewConfig {
  rotation: { x: number; y: number };
  zoom: number;
  center: { x: number; y: number; z: number };
  filters: {
    fileTypes?: string[];
    dateRange?: { start: Date; end: Date };
    sizeRange?: { min: number; max: number };
    tags?: string[];
  };
  displayOptions: {
    showLabels: boolean;
    showEdges: boolean;
    showOrbits: boolean;
    colorScheme: 'type' | 'age' | 'size';
  };
}
```

---

## 7. UI/UX Design

### 7.1 Screen Layout

```
┌─────────────────────────────────────────────────────────────┐
│  Header: CLOSM Probe Logo | Search Bar | Settings | User    │
├─────────────────────────────────────────────────────────────┤
│ ┌─────────┐ ┌─────────────────────────────┐ ┌───────────┐  │
│ │         │ │                             │ │           │  │
│ │ Sidebar │ │                             │ │  Detail   │  │
│ │         │ │                             │ │  Panel    │  │
│ │ - Overview│ │      3D Canvas            │ │           │  │
│ │ - Folders│ │                             │ │ - Selected│  │
│ │ - Filters│ │                             │ │   Node    │  │
│ │ - Tags  │ │                             │ │ - Details │  │
│ │         │ │                             │ │ - Related │  │
│ │         │ │                             │ │           │  │
│ └─────────┘ └─────────────────────────────┘ └───────────┘  │
├─────────────────────────────────────────────────────────────┤
│  Footer: Operation Hints | Zoom Level | Node Count          │
└─────────────────────────────────────────────────────────────┘
```

### 7.2 Design Principles

| Principle | Detail |
|-----------|--------|
| Dark mode first | Optimal for space representation |
| Minimal | Avoid information overload, show only necessary info |
| Intuitive operation | Natural mouse/touch interaction |
| Responsive | Desktop/tablet/mobile support |
| Accessibility | Keyboard operation, screen reader support |

### 7.3 Color Palette

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

## 8. Development Roadmap

### Phase 1: MVP (Complete)

**Goal**: Basic 3D visualizer implementation

- [x] 3D space node display
- [x] Capacity visualization (node size)
- [x] Drag rotation/zoom
- [x] Node selection/detail panel
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
- [ ] Duplicate file detection

**Deliverable**: Analysis-enabled version

### Phase 4: AI Relationship Analysis

**Goal**: Semantic relationship visualization

- [ ] Claude API integration
- [ ] File content analysis
- [ ] Auto semantic edge generation
- [ ] Clustering display
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

## 9. Use Cases

### 9.1 Storage Capacity Audit

**Persona**: User with "Google Drive is full"

**Flow**:
1. Log in to CLOSM Probe
2. Find the largest "planet" (folder) in 3D space
3. Click to expand contents
4. Identify huge "satellites" (files)
5. Delete unnecessary files

**Value**: Visually identify the culprit of capacity usage

### 9.2 Finding Related Files

**Persona**: "Where are all files related to that project?"

**Flow**:
1. Enter project name in search bar
2. Camera focuses on matching node
3. Discover surrounding cluster (related files)
4. Understand relationships beyond folder hierarchy

**Value**: Discover scattered files in one place

### 9.3 Cleaning Old Files

**Persona**: "I want to clean up files untouched for over a year"

**Flow**:
1. Select "Over 1 year" in filter
2. Dark (low brightness) nodes appear
3. Multi-select and bulk delete
4. Storage capacity recovers

**Value**: Easy filtering by time axis

### 9.4 Understanding Project Structure

**Persona**: "What's the structure of this project I just joined?"

**Flow**:
1. Select project folder
2. Get bird's eye view in 3D space
3. Understand main folder (planet) layout
4. Understand file type distribution by color

**Value**: Intuitively understand project structure

---

## 10. Competitive Analysis

| Product | Features | Differentiation from CLOSM Probe |
|---------|----------|----------------------------------|
| Finder / Explorer | Tree structure, 1D | 3D space + relationships |
| WinDirStat / GrandPerspective | 2D treemap, capacity only | 3D + time axis + relationships |
| Notion | Block-based but not spatial | Spatial navigation |
| Obsidian Graph View | Note relationship visualization | Targets entire file system |
| Dropbox / Google Drive | Cloud storage | Visualization/analysis features |

### CLOSM Probe Differentiation

1. **3D space representation**: Intuitive visualization beyond 2D
2. **Capacity + Time + Relationships**: Express multiple axes simultaneously
3. **AI relationship analysis**: Auto-detect semantic connections
4. **Intuitive operation**: Game-like exploration

---

## 11. Success Metrics (KPI)

### 11.1 User Acquisition

| Metric | Target (6 months) |
|--------|-------------------|
| Registered users | 1,000 |
| MAU | 300 |
| DAU | 50 |

### 11.2 Engagement

| Metric | Target |
|--------|--------|
| Avg session duration | 5+ minutes |
| Sessions per week | 3+ |
| Feature usage (search) | 60%+ |

### 11.3 Business

| Metric | Target |
|--------|--------|
| Paid conversion rate | 5% |
| NPS | 40+ |
| Churn rate | Under 5% |

---

## 12. Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Performance issues (large files) | UX degradation | LOD implementation, virtualization, Web Worker |
| Google API limits | Feature restriction | Caching, diff retrieval, rate limit handling |
| 3D operation learning curve | Drop-off | Tutorial, 2D mode option |
| Trademark issues | Legal risk | Pre-investigation by patent attorney |
| Security (file access) | Trust | OAuth2.0, principle of least privilege |

---

## Appendix

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
const calculateNodeRadius = (size, type) => {
  const baseSize = type === 'directory' ? 15 : 8;
  const scaleFactor = type === 'directory' ? 3 : 2;
  const minSize = type === 'directory' ? 1000 : 100;

  return baseSize + Math.log10(Math.max(size, minSize)) * scaleFactor;
};
```

### A.3 Brightness Calculation (Update Time Based)

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

## References

- NASA Voyager Program: https://science.nasa.gov/mission/voyager/
- Three.js Documentation: https://threejs.org/docs/
- Google Drive API: https://developers.google.com/drive
- Claude API: https://docs.anthropic.com/
- Neo4j Graph Database: https://neo4j.com/docs/
