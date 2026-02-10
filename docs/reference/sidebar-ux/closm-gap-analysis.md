[← General Patterns](./general-patterns.md) | [Index](./index.md)

# CLOSM Probe Gap Analysis

Current sidebar state vs. Claude/ChatGPT design philosophy best practices.

---

## Current Sidebar Structure

```
+-----------------------------+
| CLOSM Probe                 |  <- Identity (app title)
| 3D Storage Visualization    |  <- Subtitle
|                             |
| [ Open Folder ]             |  <- Primary Action
|                             |
| --- Recent ---              |  <- Temporal (auto history)
| Documents                   |
| Downloads                   |
| Projects                    |
|                             |
| --- Selected ---            |  <- Context (selection details)
| file.txt                    |
| Size: 1.2 KB               |
| Modified: 2h ago            |
|                             |
| --- Settings ---            |  <- System (TODO)
+-----------------------------+
```

---

## Layer-by-Layer Comparison

| Layer | Best Practice | CLOSM Current | Status | Notes |
|-------|--------------|---------------|--------|-------|
| Identity | App name + brief context | "CLOSM Probe" + "3D Storage Visualization" | OK | Appropriate |
| Primary Action | Most prominent button at top | "Open Folder" (accent color, full width) | OK | Good CTA placement |
| Curated | Starred / Pinned (user-intentional) | None | **Gap** | No bookmark/pin mechanism |
| Temporal | Recents (auto chronological) | Recent folders (max 8 shown) | OK | Spec says max 10, shows 8 |
| Context | Selection details | Selected celestial (name, size, modified) | OK | |
| Structural | Projects / Workspaces | N/A | N/A | Single-folder scope, not needed for MVP |
| System | Settings (theme, prefs) | Button only (TODO) | **Gap** | No functional settings |

---

## Identified Gaps

### Gap 1: No "Curated" Layer (Pinned/Starred Folders)

**Current**: Only automatic recent history. No way for users to intentionally mark important folders.

**Best practice**: Claude uses Starred, Notion uses Favorites. This separates "intentional importance" from "temporal recency."

**Impact**: Low for MVP. Users with 2-3 favorite folders would benefit, but Recent history partially covers this need.

**Recommendation**: Consider for future iteration. Add pin icon next to history entries.

### Gap 2: Settings Not Implemented

**Current**: Settings button exists but `// TODO: Settings panel`.

**Best practice**: Settings at sidebar bottom (lowest frequency, deepest operation).

**Potential settings:**

| Setting | Type | Purpose |
|---------|------|---------|
| Theme toggle | Dark/Light switch | Currently OS-detected only |
| Display limit | Slider (10-30) | Max celestials shown |
| Show hidden files | Toggle | Include dotfiles |

### Gap 3: History Count Mismatch

**Current**: `take(8)` in implementation. Spec: max 10.

**Fix**: Simple code change to align with spec.

---

## Design Pattern Alignment

### What CLOSM Probe Does Well

| Pattern | Implementation |
|---------|---------------|
| Creation-Consumption Loop | Open Folder (create) + Recent (resume) |
| Fitts's Law | Open Folder button at top, full-width, accent color |
| Progressive Disclosure | L0: sidebar always visible. L1: selection details on click |
| Spatial Stability | Fixed sidebar width (260px), consistent section positions |
| Context Panel | Selected section dynamically shows celestial details |

### What Could Be Improved

| Pattern | Current Gap | Improvement |
|---------|------------|-------------|
| Information Scent | History entries show folder name only | Add path hint or icon for file type |
| Visual Hierarchy | All sections look similar weight | Current folder could have more visual emphasis |
| Gestalt Grouping | Uses explicit separators | Consider subtle background/spacing instead |
| Progressive Disclosure depth | Settings is L0 visible but non-functional | Either implement or hide until ready |

---

## Comparison with Model Products

| Aspect | Claude | ChatGPT | CLOSM Probe |
|--------|--------|---------|-------------|
| Primary metaphor | Conversation notebook | Everything app | Universe explorer |
| Sidebar role | Switch work contexts | Select use case | Navigate folder space |
| Primary action | New Chat | New Chat (+) | Open Folder |
| History model | Starred + Recents | Date-grouped history | Recent folders |
| Context display | (in conversation) | (in conversation) | Selected celestial panel |
| Unique element | Projects hierarchy | Specialized sections | 3D view + breadcrumb |

### Key Difference

Claude/ChatGPT sidebars serve **conversation management** (many parallel threads, need organization).

CLOSM Probe sidebar serves **spatial navigation** (one active folder, need to switch targets). This is closer to **VS Code's Explorer** or **Figma's Pages** than to chat history.

**Implication**: CLOSM Probe should not over-index on chat-app patterns. The sidebar's primary job is **wayfinding in a spatial metaphor**, not thread management.

---

## Recommendations Summary

| Priority | Item | Effort |
|----------|------|--------|
| P0 | Fix history count (8 -> 10) | Trivial |
| P1 | Implement basic Settings (theme toggle) | Small |
| P2 | Add path hint to history entries | Small |
| P2 | Visual emphasis on current folder section | Small |
| P3 | Pinned/starred folders | Medium |
| P3 | Hidden files toggle in settings | Small |

---

## See Also

- [Requirements: UI/UX](../../requirements/ui-ux.md) - Current specification
- [Design: UI](../../design/ui.md) - Technical design
- [Common Principles](./common-principles.md) - Design philosophy reference
