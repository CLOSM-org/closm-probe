# CLOSM Probe Product Requirements

**Version**: 1.0
**Last Updated**: 2026-02-05
**Status**: Draft

---

## 1. Vision & Mission

### 1.1 Product Name

**CLOSM Probe** (pronounced "Closm Probe")

### 1.2 Core Concept

> Visualize storage as a 3D universe space and "explore" files in a new file management experience.

### 1.3 Mission Statement

Provide a next-generation storage interface that transcends traditional hierarchical file systems, allowing users to discover and organize files by intuitively exploring 3D space.

---

## 2. Problem Statement

Traditional file systems suffer from fundamental limitations:

| Problem | Detail |
|---------|--------|
| 1D hierarchical structure | A file can only exist in one location |
| Capacity invisibility | Hard to intuitively see "what's eating space" |
| Lack of relationships | Cannot express semantic connections between files |
| Increasing exploration cost | Deeper hierarchies make finding files harder |
| Missing time axis | Difficult to distinguish old vs recent files |

---

## 3. Solution: Physical Space Metaphor

The **Physical Space Metaphor** solves these problems by mapping to intuitive spatial concepts:

| Solution | Metaphor | Detail |
|----------|----------|--------|
| 3D space representation | Universe | Files placed as celestial bodies in space |
| Capacity visualization | Size | Larger planets = larger directories |
| Graph structure | Orbits & Edges | Express relationships via orbital paths |
| Intuitive navigation | Exploration | Drag and zoom to explore space |
| Time axis expression | Brightness | Recent = bright stars, old = dim |

---

## 4. Target Users

### 4.1 Primary Users

- Creators/engineers handling large amounts of files
- Users seeking storage capacity visualization and optimization
- Early adopters seeking new file management experiences

### 4.2 Use Cases

#### UC1: Storage Capacity Audit

**Persona**: User with "Google Drive is full"

**Flow**:
1. Log in to CLOSM Probe
2. Find the largest planet (folder) in the universe
3. Click to see its moons (contents)
4. Identify massive satellites (files)
5. Delete unnecessary files

**Value**: Visually identify the culprit of capacity usage

#### UC2: Finding Related Files

**Persona**: "Where are all files related to that project?"

**Flow**:
1. Enter project name in search bar
2. Camera focuses on matching celestial body
3. Discover surrounding constellation (related files)
4. Understand relationships beyond folder hierarchy

**Value**: Discover scattered files in one place

#### UC3: Cleaning Old Files

**Persona**: "I want to clean up files untouched for over a year"

**Flow**:
1. Select "Over 1 year" in filter
2. Dim (low brightness) bodies appear
3. Multi-select and bulk delete
4. Storage capacity recovers

**Value**: Easy filtering by time axis

#### UC4: Understanding Project Structure

**Persona**: "What's the structure of this project I just joined?"

**Flow**:
1. Select project folder (planet)
2. Get bird's eye view of the solar system
3. Understand main planets (folder) layout
4. Understand satellite type distribution by color

**Value**: Intuitively understand project structure

---

## 5. Functional Requirements

### 5.1 Core Features (MVP)

| ID | Feature | Description |
|----|---------|-------------|
| F1 | 3D Universe Display | Display files/folders as celestial bodies in 3D space |
| F2 | Size Visualization | Larger bodies = larger capacity |
| F3 | View Manipulation | Drag rotation, zoom, pan |
| F4 | Selection & Details | Click to select, show details panel |
| F5 | Drill-Down Navigation | Double-click to enter directory |

### 5.2 Extended Features (Future)

| ID | Feature | Description | Priority |
|----|---------|-------------|----------|
| F6 | File Type Filter | Filter by code, image, video, etc. | P1 |
| F7 | Date Filter | Filter by last modified date | P1 |
| F8 | Search & Focus | Search and zoom to matches | P1 |
| F9 | Storage Integration | Connect to local/cloud storage | P1 |
| F10 | AI Relationship Analysis | Auto-detect file relationships | P2 |
| F11 | Duplicate Detection | Find duplicate files | P2 |
| F12 | File Operations | Move, delete, create files | P2 |

---

## 6. User Interaction Requirements

### 6.1 View Manipulation

| Operation | Action | Notes |
|-----------|--------|-------|
| Drag | Rotate view | Orbit around the sun |
| Scroll | Zoom in/out | Get closer or farther from planets |
| Right-drag | Pan | Parallel movement |
| Pinch (touch) | Zoom | Mobile gesture support |

### 6.2 Navigation Actions

| Action | Result |
|--------|--------|
| Double-click planet | Drill down (planet becomes new sun) |
| Breadcrumb click | Navigate back to that level |
| Double-click sun | Go up one level |

### 6.3 Selection Mechanics

| Action | Result |
|--------|--------|
| Single click | Select celestial body, show details |
| Hover | Highlight with glow effect |
| Click empty space | Deselect |

---

## 7. Non-Functional Requirements

### 7.1 Performance

| Metric | Target |
|--------|--------|
| Frame rate | 60fps on modern hardware |
| Max nodes displayed | 1000+ without lag |
| Startup time | < 3 seconds |

### 7.2 Platform Support

| Platform | Priority | Notes |
|----------|----------|-------|
| Desktop (Mac) | P0 | Primary target |
| Desktop (Windows/Linux) | P1 | Cross-platform |
| Web | P2 | Future consideration |

### 7.3 Design Principles

| Principle | Detail |
|-----------|--------|
| Dark mode first | Optimal for space representation |
| Minimal | Avoid information overload |
| Intuitive operation | Natural mouse/touch interaction |
| Accessibility | Keyboard operation support |

---

## 8. Success Metrics (KPI)

### 8.1 User Acquisition (6-month target)

| Metric | Target |
|--------|--------|
| Downloads | 1,000 |
| MAU | 300 |
| DAU | 50 |

### 8.2 Engagement

| Metric | Target |
|--------|--------|
| Avg session duration | 5+ minutes |
| Sessions per week | 3+ |
| Feature usage (search) | 60%+ |

### 8.3 Satisfaction

| Metric | Target |
|--------|--------|
| NPS | 40+ |
| Retention (30-day) | 50%+ |

---

## 9. Development Roadmap

### Phase 1: MVP

- [x] 3D space celestial body display
- [x] Capacity visualization (body size)
- [x] Drag rotation/zoom
- [ ] Body selection/detail panel
- [ ] Local folder integration

### Phase 2: Core Features

- [ ] File type filter
- [ ] Date filter
- [ ] Search & focus
- [ ] Duplicate detection

### Phase 3: Advanced

- [ ] AI relationship analysis
- [ ] File operations (move, delete)
- [ ] Cloud storage integration

---

## 10. Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Performance issues (large files) | UX degradation | LOD implementation, virtualization |
| 3D operation learning curve | Drop-off | Tutorial, simplified controls |
| Security (file access) | Trust issues | Minimal permissions, transparency |
