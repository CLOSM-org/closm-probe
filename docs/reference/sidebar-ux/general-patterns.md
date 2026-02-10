[← ChatGPT](./chatgpt-analysis.md) | [Index](./index.md) | [Gap Analysis →](./closm-gap-analysis.md)

# General Sidebar UX Patterns

Industry patterns from Notion, Slack, VS Code, Figma, Spotify, and UX research.

---

## 1. Six Sidebar Design Patterns

### A. Persistent Sidebar (Always Visible)

- **Examples**: Notion, Gmail, Slack (desktop)
- **Width**: Typically 224-280px
- **Principle**: NNGroup research - "users fixate on left half 80% of time", left placement has highest discoverability

### B. Navigation Rail (Icon Bar)

- **Examples**: Gmail (compact), VS Code Activity Bar
- **Width**: 48-72px (icons only)
- **Principle**: Saves space while maintaining constant access to primary functions. Click opens secondary panel

### C. Collapsible Sidebar

- **Examples**: Notion, Figma, Spotify
- **Principle**: Supports "work mode" (sidebar hidden) vs. "explore mode" (sidebar expanded) switching

### D. Slide-in Drawer

- **Examples**: Mobile apps (hamburger menu)
- **Principle**: Optimized for limited screen space. NNGroup warns: "Don't hide behind hamburger on desktop"

### E. Two-Panel Selector (Dual Sidebar)

- **Examples**: Slack (nav bar + sidebar), VS Code (Activity Bar + Primary Sidebar)
- **Principle**: First column = category selection, second column = detail list. Browse overview while viewing details without page transitions

### F. Context Panel

- **Examples**: Figma (right sidebar: properties), VS Code (Secondary Sidebar)
- **Principle**: Content changes dynamically based on selected object. UX Planet: "Static menus don't adapt to user actions, hindering navigation"

---

## 2. Product Sidebar Breakdowns

### Notion

```
[Workspace name + account switch]   -- Identity
[Search / AI / Home / Inbox]        -- Quick Actions
[Teamspaces]                        -- Shared (team space)
[Shared]                            -- Collaborative
[Private]                           -- Personal
[Favorites]                         -- Pinned (frequent access)
[Settings / Trash]                  -- System (meta operations)
```

**Key insight**: Structure itself is user-delegated. Notion tells users "you build the structure" through its tree model. **Favorites** elegantly introduces Recency into an Organization-first design.

**Design trick**: Uses **Gestalt principles** (proximity, similarity) with subtle spacing and background color changes for grouping cognition - no explicit dividers needed.

### Slack

```
[Workspace icon]                    -- Identity
[Home/DMs/Activity/Later/More]      -- Navigation Rail (mode switch)
  +-- [Channels section]            -- Organization (structured conversations)
  +-- [Direct Messages section]     -- People (person-based conversations)
  +-- [Apps section]                -- Tools (tool integration)
  +-- [Custom sections]             -- User-defined
```

**Key insight**: New design uses "focus-driven design" - departed from old design where all badges and notifications were always visible. Navigation rail switches sidebar content entirely per mode.

### VS Code

```
[Activity Bar (icon rail)]          -- Mode Selector
  +-- Explorer                      -- Files (file tree)
  +-- Search                        -- Find
  +-- Source Control                 -- Versioning
  +-- Debug                         -- Runtime
  +-- Extensions                    -- Ecosystem
  +-- [Extension-added views]       -- Extensible
```

**Key insight**: Two-panel architecture. Activity Bar (48px icon rail) + Primary Sidebar (detail panel). Each icon represents a completely different tool context.

### Figma

```
[Left sidebar]
  +-- Pages                         -- Structure (file structure)
  +-- Layers                        -- Hierarchy (layer hierarchy)
[Right sidebar]
  +-- Design                        -- Properties (attribute editing)
  +-- Prototype                     -- Interaction
  +-- Inspect                       -- Handoff (implementation info)
```

**Key insight**: Split left/right sidebars. Left = structure/navigation, Right = context-sensitive properties of selected object.

### Spotify

```
[Home / Search]                     -- Discovery
[Your Library]                      -- Collection
  +-- Filters: Playlists/Artists    -- Type Filter
  +-- Sort: Recent/Alphabetical     -- Order
  +-- Grid / List view              -- View Mode
```

**Key insight**: Elegant Recency/Organization hybrid. Default sort is "Recently played" but users can switch to alphabetical. Filter chips narrow without losing context.

---

## 3. Universal Section Division Pattern

| Layer | Position | Content | Principle |
|-------|----------|---------|-----------|
| **Identity** | Top | Workspace/account | Cognitive anchor: "who am I, where am I" |
| **Primary Action** | Upper | New/Create button | Most frequent operation at most accessible position |
| **Navigation** | Upper-mid | Mode switch/categories | Top-level structure of information space |
| **Content** | Middle | Actual item lists | Items users operate on daily |
| **System** | Bottom | Settings/Help/Trash | Low-frequency but necessary meta operations |

This follows **Information Scent** principle: labels and structure serve as "signposts" for users to intuitively judge "where to click next."

---

## 4. Primary Action (CTA) Placement

Almost all major apps place the most important action at sidebar top:

| App | Primary Action | Position |
|-----|---------------|----------|
| Gmail | Compose | Top, above navigation items |
| Slack | New Message | Upper sidebar |
| ChatGPT | New Chat | Top of sidebar |
| Claude | New Chat | Top of sidebar |
| Notion | New Page + Search | Quick action area, upper |
| Spotify | + (new playlist) | Top of Your Library section |

**Three reasons for top placement:**

1. **Fitts's Law**: Upper-left corner has effectively infinite target size (cursor stops at screen edge). Research shows CTA optimization alone improved conversion by 34%
2. **Visual Hierarchy**: Gmail's "Compose" uses intentionally different style (color, size, shape) from surrounding nav items. Passes "blur test" (still first visible element when screen is blurred)
3. **Gutenberg Diagram**: Upper-left is "Primary Optical Area" with highest attention in left-to-right reading cultures

---

## 5. Context Switching Deep Dive

### Pattern A: Mode Switching (Activity Bar Style)

- **Examples**: VS Code (Explorer/Search/Git/Debug), Slack (Home/DMs/Activity/Later)
- **Implementation**: Icon-based mode selector at sidebar top. Sidebar content changes completely per selection
- **Merit**: Each mode has independent information space - less cognitive confusion
- **Slack new design**: "Home, DMs, Activity, Later, More" as navigation bar, each button switches sidebar content. "Focus-driven design" departing from always-visible badges

### Pattern B: Filtering (Collection Style)

- **Examples**: Spotify (Playlists/Artists/Albums/Podcasts filter), Slack (All workspaces filter)
- **Implementation**: Filter chips on same list. Narrows displayed items without context switch
- **Merit**: "Same space" feeling preserved, low cognitive cost

### Pattern C: Section Expansion (Tree Style)

- **Examples**: Notion (Teamspace/Shared/Private collapse), VS Code (file tree expand)
- **Implementation**: Each sidebar section is collapsible. Users expand only needed sections
- **Merit**: Embodies **progressive disclosure**: "Break complex tasks into smaller manageable steps, present one at a time" (Jakob Nielsen, 1995)

---

## 6. Research-Backed UX Principles

| Principle | Description | Source |
|-----------|-------------|--------|
| **Left-side dominance** | Users spend 80% of viewing time on left half | NNGroup |
| **Vertical scan efficiency** | Vertical lists require fewer eye fixations than horizontal | Eye tracking research |
| **Information Scent** | Labels and structure intuitively guide "where to go next" | Information Foraging Theory |
| **Fitts's Law** | Selection time depends on target distance and size | Motor control law |
| **Gestalt principles** | Spacing, color, background create visual grouping | Perception psychology |
| **Spatial stability** | Fixed element positions support motor memory formation | Repetition learning theory |
| **Progressive disclosure** | Control cognitive load by staged information reveal | Jakob Nielsen (1995) |

---

## See Also

- [Common Principles](./common-principles.md) - Synthesized design principles
- [CLOSM Gap Analysis](./closm-gap-analysis.md) - Application to CLOSM Probe
