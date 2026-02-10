[← Index](./index.md) | [Common Principles](./common-principles.md) | [ChatGPT →](./chatgpt-analysis.md)

# Claude.ai Sidebar Analysis

Design philosophy, structure, and user flow of Claude.ai's interface.

---

## 1. Use Case Segmentation

| Use Case | UI Element | Description |
|----------|-----------|-------------|
| Start new conversation | "Start a new chat" + prompt menu | Top of sidebar. Main area shows Write/Learn/Code starter prompts |
| Ongoing work | Starred section | Bookmarked conversations and projects |
| Return to recent work | Recents section | Auto-generated chronological list |
| Knowledge base building | Projects | Self-contained workspaces with docs, code, custom instructions |
| Past context search | Chat search + Memory | RAG-based cross-conversation search |
| Structured content creation | Artifacts panel | Code, docs, visualizations in right-side panel |
| Privacy-focused work | Incognito chat (ghost icon) | Not recorded in search or memory |

**Strategic direction**: Transition from "chatbot" to "work platform." Claude Cowork (Jan 2026) is a task-oriented AI coworker, beyond chat interface.

---

## 2. Sidebar Structure

```
+---------------------------+
| [New Chat]                |  <- Highest priority action
+---------------------------+
| * Starred                 |  <- User-intentional importance
|   |- Conversation 1       |
|   |- Project A            |
+---------------------------+
| Recent                    |  <- Auto chronological
|   |- Today's chats        |
|   |- Yesterday's chats    |
+---------------------------+
| Projects                  |  <- Structured workspaces
|   |- Project A            |
|   |  |- Knowledge base    |
|   |  |- Custom instructions|
|   |  |- Chat threads      |
|   |- Project B            |
+---------------------------+
```

### Placement Rationale

| Position | Section | Reason |
|----------|---------|--------|
| Top | New Chat | Most frequent action. Fitts's Law: most accessible position |
| Upper | Starred | **Intentionally** marked as important. Reduces cognitive load for priority work |
| Middle | Recents | **Automatically** generated. Short-term memory aid ("where was I?") |
| Lower | Projects | Deeper structured work. Lower frequency but deep operations when reached |

**Ordering principle**: Frequency x Urgency. New chat = most frequent + immediate. Starred = important + moderate frequency. Recents = passive reference. Projects = active but low-frequency.

---

## 3. User Journey

```
Phase 1: Landing
  Main area: "Ask Claude anything" input box
  + Category starter prompts (Write / Learn / Code)
  + Model selector + Style selector (Normal/Concise/Explanatory/Formal/Custom)

Phase 2: Task Definition
  User enters prompt or selects starter prompt
  + File attachments (PDF, TXT, CSV, images)
  + Tone/style configuration

Phase 3: Dialogue (Iterative Collaboration)
  Claude responds -> Artifacts appear in right panel
  User provides feedback or additional prompts
  Code tab <-> Preview tab switching

Phase 4: Organization
  Add conversation to Starred
  Associate with Project
  Copy/download Artifacts

Phase 5: Continuation (Next Session)
  Return via Recents or Starred
  Search past context via chat search
  Memory function accumulates context automatically
```

**Key design points:**
- **Frictionless start**: Starter prompts solve "what should I ask?" problem
- **No ads, no sponsored content**: Anthropic explicitly states this as a trust principle
- **Pure thinking space**: Interface intentionally recedes to let content shine

---

## 4. Information Architecture

```
Claude.ai
├── Account Layer
│   ├── Personal Preferences (response style)
│   ├── Memory management (view/edit/reset/pause)
│   ├── Feature toggles (search, memory ON/OFF)
│   ├── Subscription management
│   └── Incognito mode
│
├── Organization Layer (Projects)
│   ├── Project A
│   │   ├── Knowledge base (200K context window)
│   │   │   ├── Uploaded documents
│   │   │   ├── Code files
│   │   │   └── Text data
│   │   ├── Custom instructions
│   │   ├── Member management (Can use / Can edit)
│   │   └── Chat threads
│   └── Project B
│
├── Conversation Layer
│   ├── Starred (manual bookmarks)
│   ├── Recents (auto chronological)
│   └── Search results (RAG cross-search)
│
└── Content Layer (per conversation)
    ├── Messages (You / Claude)
    ├── Artifacts (right panel: Code, Preview, HTML, SVG, Mermaid, React)
    └── File attachments
```

**Key principles:**
- **Project isolation**: Each project has independent memory, knowledge base, and custom instructions. Prevents context contamination
- **RAG scalability**: Paid plans expand capacity up to 10x via RAG
- **Nested hierarchy vs. flat list**: Claude offers Projects with nested structure (vs. ChatGPT's flat conversation list). Enterprise workflow differentiator

---

## 5. Progressive Disclosure

### Level 0: Immediately visible (zero clicks)

| Element | Reason |
|---------|--------|
| Input box ("Ask Claude anything") | Most basic action, must be instantly available |
| Model selector | Must choose before conversation starts |
| Style selector (Normal/Concise/Explanatory/Formal) | Affects conversation quality, pre-set important |
| Starred / Recents list | Returning user's top priority need |
| Starter prompts (Write/Learn/Code) | New user onboarding support |

### Level 1: One click to reveal

| Element | Trigger | Reason |
|---------|---------|--------|
| Artifacts panel | Claude generates structured content | Don't occupy space when unneeded |
| Project details (knowledge base, instructions) | Click project | Complex settings only when needed |
| File upload UI | Click attachment button | Not used every time |
| Custom style creation | Style dropdown -> new | Advanced user feature |
| Incognito mode | Click ghost icon | Only when privacy is required |

### Level 2: Multiple steps to reach

| Element | Path | Reason |
|---------|------|--------|
| Memory view/edit | Settings -> Capabilities -> View and edit | Low frequency, careful operation |
| Memory reset | Settings -> Capabilities -> Reset | **Destructive** - intentionally deep |
| Search/Memory toggle | Settings -> Capabilities -> Toggle | Set once, rarely changed |
| Project permissions | Project -> Member settings | Admin-only |

---

## 6. Design Philosophy (5 Keywords)

| Principle | Manifestation |
|-----------|---------------|
| **Content First** | Interface "intentionally recedes to let content shine." Black text + white bg + purple accent minimal palette |
| **Trust through Simplicity** | No ads, no sponsored content. Geist agency defined "trust, collaboration, clarity" as core principles |
| **Function over Decoration** | UI component system designed "function first, without losing brand soul" |
| **Purposeful Minimalism** | "Great design is not about excess, but purpose." Styrene + Tiempos typography: "technically refined yet charmingly quirky" |
| **Safety-Aware UX** | Constitutional AI reflected in UI. Destructive operations buried deep. Incognito for privacy control |

---

## 7. Recent Redesign: "Claudia"

Codenamed "Claudia" redesign changes sidebar from hover-display to toggle-display, adopting ChatGPT-like layout. Indicates convergence toward industry-standard mental models.

**Claude Cowork** (Jan 2026): Beyond chat interface toward "AI coworker" - a goal-tracking system, closer to task-oriented assistant than chatbot. Anthropic's vision: evolve Claude from chatbot to work platform.

---

## See Also

- [Common Principles](./common-principles.md) - Shared patterns
- [ChatGPT Analysis](./chatgpt-analysis.md) - Comparison point
