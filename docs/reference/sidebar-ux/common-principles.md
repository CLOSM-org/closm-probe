[← Index](./index.md)

# Common Sidebar Design Principles

Shared design philosophy extracted from Claude, ChatGPT, and industry best practices.

---

## 1. Creation-Consumption Loop

The fundamental pattern: every sidebar divides user intent into **Create** (start new) and **Resume** (continue existing).

| Product | Create Action | Resume Action |
|---------|--------------|---------------|
| Claude | New Chat | Starred / Recents |
| ChatGPT | New Chat (+) | History (date groups) |
| Gmail | Compose | Inbox (time-ordered) |
| Slack | New Message | Channels / DMs |
| Twitter/X | Post (FAB) | Timeline |
| Notion | New Page | Recents / Favorites |
| CLOSM Probe | Open Folder | Recent folders |

---

## 2. Sidebar Layer Hierarchy

All products follow a top-to-bottom ordering by **frequency x urgency**:

| Layer | Content | Principle |
|-------|---------|-----------|
| **Identity** (top) | App name, workspace | Cognitive anchor: "where am I?" |
| **Primary Action** | New / Create button | Fitts's Law: most frequent action at most accessible position |
| **Curated** | Starred / Pinned (manual) | User-intentional importance markers |
| **Temporal** | Recents (automatic) | Episodic memory aid (short-term recall) |
| **Structural** | Projects / Workspaces | Active but low-frequency structured work |
| **System** (bottom) | Settings / Profile | Low-frequency meta operations |

**Why this order works:**
- **Fitts's Law**: Targets near screen edges have effectively infinite size (cursor stops at edge)
- **Gutenberg Diagram**: Left-to-right, top-to-bottom reading cultures fixate on upper-left first (Primary Optical Area)
- **NNGroup research**: Users spend 80% of viewing time on the left half of screen

---

## 3. Progressive Disclosure (3 Levels)

| Level | Trigger | Examples | Safety |
|-------|---------|----------|--------|
| **Level 0** (immediate) | Page load | New button, history list, input box | Safe actions only |
| **Level 1** (1 click) | User action | Project details, Artifacts panel, file upload | Reversible actions |
| **Level 2** (2+ clicks) | Deep navigation | Memory reset, permissions, theme settings | Destructive/rare ops |

**Principle**: Destructive operations are intentionally buried deeper in the hierarchy (Safety-Aware UX).

---

## 4. "New + History" Pattern Analysis

```
[+ New Chat]        ← Primary Action (creation)
────────────
Today
  - Conversation A  ← Reverse-chronological feed
  - Conversation B
Yesterday
  - Conversation C
```

**Six UX principles driving this pattern:**

| Principle | Explanation |
|-----------|-------------|
| **Persistent Primary Action** | Creation CTA is always visible throughout the experience, encouraging content creation (same as Twitter FAB) |
| **Cognitive Load Reduction** | Time-ordered history requires zero organizational effort from users |
| **Episodic Memory Alignment** | Users recall "I discussed this yesterday afternoon" - time-based ordering matches natural recall |
| **Reverse Chronological Feed** | "Newest on top" is a learned pattern from social media - zero learning cost |
| **Spatial Stability** | Fixed button position builds motor memory - users physically memorize "click upper-left to start new" |
| **Information Scent** | "New Chat" label clearly signals next action. NNGroup: "One word is worth a thousand images in navigation" |

---

## 5. Recency vs. Organization

Apps choose their balance based on **primary task nature**:

| Task Type | Strategy | Best For |
|-----------|----------|----------|
| **Exploratory** ("find that conversation") | Recency-first | ChatGPT, Claude, Slack DMs |
| **Structural** ("open project A file") | Organization-first | VS Code, Figma |
| **Hybrid** | Both with toggle | Spotify, Gmail, Notion |

**Notion's elegant approach**: User-defined page tree (Organization) + **Favorites** section that implicitly introduces Recency.

---

## 6. Context Switching Patterns

| Pattern | Implementation | Example |
|---------|---------------|---------|
| **Mode Switching** | Icon-based mode selector at top, content changes completely | VS Code Activity Bar, Slack nav rail |
| **Filtering** | Filter chips on same list, narrow without context loss | Spotify type filters, Slack workspace filter |
| **Section Expansion** | Collapsible sections, user controls visibility | Notion teamspace/shared/private, file trees |

**Key insight (Notion)**: Uses **Gestalt principles** (proximity, similarity) with subtle spacing and background color changes to create grouping cognition without explicit dividers.

---

## 7. Mental Models Created by Sidebars

| Product | Mental Model | Metaphor |
|---------|-------------|----------|
| Notion | Digital workspace | "My desk drawers" - nested pages, user-organized |
| Slack | Office hallway | "Room directory" - channels are rooms, DMs are private conversations |
| VS Code | Workbench + toolbox | "Tool belt" - each Activity Bar icon is a different tool |
| Figma | Canvas + inspector | "Layer palette" - inherited from Photoshop/Illustrator |
| Spotify | Record shelf | "My collection" - owned/saved music to browse |
| ChatGPT/Claude | Conversation notebook | "Chat history" - past dialogues in chronological order |

**NNGroup definition**: "Mental models are internal representations of how a system works that users carry in their minds." Good sidebars minimize the gap between mental model and actual UI.

**Critical insight**: Sidebar structure **is** the information architecture revealed to users. Notion's tree tells users "you build the structure." Slack's flat channel list tells users "all conversations have equal importance."

---

## See Also

- [Claude Analysis](./claude-analysis.md) - Claude-specific design philosophy
- [ChatGPT Analysis](./chatgpt-analysis.md) - ChatGPT-specific evolution
- [General Patterns](./general-patterns.md) - Industry patterns from Notion, Slack, VS Code, etc.
