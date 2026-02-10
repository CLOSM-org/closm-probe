[← Claude](./claude-analysis.md) | [Index](./index.md) | [General Patterns →](./general-patterns.md)

# ChatGPT Sidebar Analysis

Design philosophy, evolution, and user flow of ChatGPT's interface.

---

## 1. Use Case Segmentation

| Category | Sidebar Item | Purpose |
|----------|-------------|---------|
| Conversation (core) | New Chat (+) / History | Text-based dialogue |
| Image generation | Images | Dedicated creative studio |
| Video generation | Sora | Text-to-video |
| Third-party integration | Apps | Spotify, Canva, Figma integration |
| Coding | Codex | Coding agent management |
| Custom AI | GPTs | Purpose-customized AI assistants |
| Organization | Projects | Conversation/file grouping |
| Health | Health | Medical records/health data |
| Asset management | Library | Generated image hub |

**Strategic intent**: ChatGPT evolving into "super app" (everything app). After Plugins (2023) and GPT Store (2024), Apps (late 2025) embeds third-party applications directly into conversation flow.

### Three Value Patterns (Official OpenAI)

| Pattern | Description |
|---------|-------------|
| **Knowledge** | Live data, internal analytics, subscription data not available in base ChatGPT |
| **Action** | Record creation, notification sending, scheduling - converting intent to concrete actions |
| **Presentation** | Comparison tables, timelines, charts - information visualization in GUI |

---

## 2. Sidebar Structure (2026-02 estimated)

```
+----------------------------------+
| [=] Sidebar toggle               |
|                                  |
| [+] New Chat                     |  <- Most frequent operation
| [Q] Search                       |  <- Past conversation search
|                                  |
| --- Specialized Sections ---     |
| [IMG] Images                     |  <- Creative studio
| [APP] Apps                       |  <- Third-party apps
| [COD] Codex                      |  <- Coding
| [ATL] Atlas                      |  <- Browser
| [GPT] GPTs                       |  <- Custom GPTs
| [PRJ] Projects                   |  <- Project management
|                                  |
| --- Recent Conversations ---     |
| [Conversation 1]                 |
| [Conversation 2]                 |
| [See more...]                    |  <- Infinite scroll
|                                  |
| --- Footer ---                   |
| [User Profile / Settings]        |
+----------------------------------+
```

### Placement Rationale

| Position | Element | Reason |
|----------|---------|--------|
| Top | New Chat + Search | Fitts's Law: highest frequency at most accessible position |
| Upper-mid | Specialized sections (Images, Apps, Codex) | Use-case entry points: user selects "what I want to do" |
| Mid | Projects + GPTs | Personalized content: user-created/customized assets |
| Lower | Conversation history | Chronological browsing: recent conversations, older ones folded |
| Bottom | Settings/Profile | Low-frequency operations |

**Follows "Intent -> Execute -> Reflect" cognitive flow**: Start new (intent), use specialized tools (execute), reference past conversations (reflect) - top to bottom.

---

## 3. User Journey

```
Landing
  |
  v
[Empty chat screen]  <- Center: prompt input box
  |                     Model selector (Auto/Fast/Thinking)
  |
  +-- Direct input --> [Conversation] --> [Response] --> [Iterate/Deepen]
  |
  +-- Sidebar selection
  |       |
  |       +-- Images --> [Image generation studio]
  |       +-- Apps --> [Third-party integration]
  |       +-- Codex --> [Coding environment]
  |       +-- GPTs --> [Custom GPT selection]
  |       +-- Projects --> [Project context]
  |       +-- Past conversation --> [Resume conversation]
  |
  v
[Task completion]
  |
  +-- Library saves artifacts (automatic)
  +-- Conversation added to sidebar history (automatic)
  +-- Organize into Project (manual)
```

**Design philosophy:**
1. **Zero-step start**: Prompt input available immediately on landing. No menu selection or onboarding required
2. **Conversation is primary interaction**: OpenAI's official principle: "Optimize for Conversation, Not Navigation"
3. **Model controls flow**: Even without explicit feature selection, model auto-judges appropriate tools (Canvas, image gen, etc.) from conversation context

---

## 4. Information Architecture

```
ChatGPT (root)
├── Conversation Layer (primary)
│   ├── Individual conversation threads
│   │   ├── User messages
│   │   ├── AI responses
│   │   │   ├── Inline display (cards, carousels)
│   │   │   ├── Fullscreen display (Canvas, maps)
│   │   │   └── Picture-in-Picture (parallel work)
│   │   └── Action buttons (regenerate, rate)
│   └── Projects (conversation groups)
│       ├── Related conversations
│       ├── Uploaded files
│       └── Project-specific instructions
│
├── Specialized Tool Layer (secondary)
│   ├── Images (creative studio)
│   ├── Apps (third-party integration)
│   ├── Codex (coding environment)
│   ├── Health (health management)
│   ├── GPTs (custom AI assistants)
│   └── Atlas (browser)
│
├── Asset Management Layer (support)
│   ├── Library (generated image storage)
│   ├── Conversation history (chronological)
│   └── Memory (learned preferences)
│
└── Settings Layer (meta)
    ├── Model selection (Auto/Fast/Thinking)
    ├── Personalization
    ├── Connector settings
    └── Privacy management
```

**Core principle**: **"Conversation First"** architecture. All tools/features are invoked from within conversation or supplement it. Official: "Extract, Don't Port" - don't bring entire products into chat, extract atomic actions and integrate into conversation.

---

## 5. Progressive Disclosure

| Level | Immediately Visible | After Interaction |
|-------|-------------------|-------------------|
| **Sidebar** | Recent conversations (few), main section icons | Older conversations (scroll), Project contents (expand) |
| **Chat screen** | Empty prompt box, model selector | Suggestions, Canvas auto-launch |
| **Response** | Text response | Source citations (expand), interactive highlights |
| **Settings** | None (footer icon only) | Full settings menu (click) |
| **Apps** | None (shown as needed in conversation) | Inline cards, fullscreen, PiP |

### Disclosure Layers

```
Layer 1: Empty canvas
  ├── Prompt box only
  └── "Ask anything"

Layer 2: Light hints
  ├── Model selection dropdown
  ├── File attachment button
  └── Voice input button

Layer 3: Context-driven suggestions
  ├── Canvas auto-launch (10+ lines of code/text)
  ├── Health navigation suggestion (health-related queries)
  └── App auto-suggestion (context matching)

Layer 4: Explicit operations
  ├── Sidebar expansion
  ├── Project creation/organization
  ├── Detailed settings changes
  └── Deep history search
```

**Core principle**: OpenAI's **"Minimal UI"** - UI used selectively for "clarifying actions, capturing input, displaying structured results." Decorative components eliminated. Conversation handles history, confirmation, and follow-up.

---

## 6. Sidebar Evolution Timeline

### Phase 1: Minimal Chat List (Nov 2022 - Early 2023)

- **State**: Extremely simple sidebar with conversation list only
- **Design decision**: "Ultimate simplicity" that powered ChatGPT's explosive adoption. Intentionally mimicked WhatsApp/Slack messenger UI for familiarity
- **Insight**: JasperAI/Rytr with GPT-3 existed as "tools"; ChatGPT chose "companion" via conversational interface

### Phase 2: GPTs + Plugins (Mar 2023 - Early 2024)

- **Added**: Plugin support, Custom GPTs, GPT Store
- **Decision**: GPTs section added to sidebar. First platformization attempt
- **Issues**: Low plugin discoverability, poor adoption. No GPT reordering in sidebar

### Phase 3: Canvas + Projects (Late 2024)

- **Added**: Canvas (co-editing screen), Projects (conversation grouping)
- **Decision**: Work spaces beyond conversation. Paradigm shift from "dialogue" to "collaboration"
- **Sidebar change**: Projects section added with icon/color visual distinction

### Phase 4: Floating Sidebar Revolution (Late 2024 - Early 2025)

- **Changed**: Floating sidebar, soft dismiss, infinite scroll
- **Decision**: Previous sidebar "pushed" content aside; floating sidebar "hovers over" content. Maintains immersion while providing navigation
- **Reception**: Mixed. "Can't keep sidebar permanently open" complaints vs. workspace appreciation

### Phase 5: Specialized Section Proliferation (2025)

- **Added**: Images, Sora, Apps, Codex, Health, Library, Atlas
- **Decision**: Super app transformation. Dedicated entry points per use case, "conversation + specialized tools" hybrid navigation
- **Issue**: Section overload. Chrome extension "ChatGPT Sidebar - Hide Unnecessary Sections" emerged to individually hide Images, Apps, Codex, Atlas, GPTs, Projects

### Phase 6: Visual Response Evolution (Early 2026)

- **Added**: Memory source display, interactive highlights, personalization settings enhancement
- **Decision**: Improved response "scannability." Interactive highlights on key entities with side panel details

---

## 7. User Research Data

UX case study (160+ users) revealed:

| Finding | Data |
|---------|------|
| Can "usually" find past conversations | 51.6% |
| Can **always** find past conversations | Only 9.6% |
| Users with 30+ accumulated conversations | 76% |
| Auto-generated titles match content | Often poor |
| Search abandonment threshold | Within 15 seconds |
| Context loss in long conversations | Users start new conversations |

---

## 8. Core Design Philosophy

| Principle | Description |
|-----------|-------------|
| **Conversational Leverage** | Use natural language, thread context, multi-turn guidance to unlock workflows impossible with traditional UI |
| **Extract, Don't Port** | Don't bring entire products in; extract core tasks only and integrate into conversation |
| **Minimal UI** | UI used selectively; conversation handles history, confirmation, follow-up |
| **Optimize for Conversation** | Optimize for conversation flow, not navigation |

### Design Tensions

| Tension | Description |
|---------|-------------|
| Simplicity vs. Feature Growth | Initial simplicity was key to success, but super-app expansion adds sections |
| Conversation-centric vs. Specialized Tools | "Everything from conversation" vs. Images/Codex/Health dedicated entry points |
| Personalization vs. Universality | Projects, pinned GPTs, Library vs. simple onboarding for new users |
| Floating (immersion) vs. Always-on (accessibility) | Workspace width vs. instant navigation access |

---

## Comparison: Claude vs. ChatGPT

| Aspect | Claude | ChatGPT |
|--------|--------|---------|
| Primary focus | Project-based organization | Conversation threads + super app |
| Sidebar content | Projects + conversations | History + specialized section clusters |
| Organization level | High (explicit project hierarchy) | Medium (Projects, pinning) |
| Navigation feel | Hierarchical, team-oriented | Minimalist + extensible |
| Design philosophy | "Thinking space" minimalism | "Everything app" expansion |
| Content display | Artifacts dedicated right panel | Canvas (similar, later addition) |
| Branding | Subtle purple accent | Green/black accent |

---

## See Also

- [Claude Analysis](./claude-analysis.md) - Comparison counterpart
- [General Patterns](./general-patterns.md) - Industry patterns
