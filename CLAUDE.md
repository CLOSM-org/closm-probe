# CLOSM Probe Development Guidelines

**Last Updated**: 2026-01-08

Project rules and guidelines for Claude Code assistance.

---

## Project Overview

**CLOSM Probe**: 3D storage visualization tool that represents file systems as explorable universe spaces.

| Item | Value |
|------|-------|
| Repository | closm-probe |
| Dev URL | http://localhost:3100 |
| Design Doc | `docs/design/product-design.md` |

### Tech Stack

| Category | Technology |
|----------|------------|
| Framework | Next.js 16 (App Router) |
| Language | TypeScript |
| 3D Rendering | Three.js + React Three Fiber |
| Post-processing | @react-three/postprocessing |
| Styling | Tailwind CSS |
| State | React hooks (useState, useMemo) |

---

## Absolute Rules

### 1. Commit Policy
- **Never commit without explicit instruction**
- Wait for user approval before committing

### 2. Build Policy
- **Never run `npm run build` without instruction**
- Same applies to `npm run dev`

### 3. Documentation Policy
- Keep docs up-to-date when requirements change
- Remove outdated or confusing content
- Docs should be in **English** (for token efficiency)
- Conversations remain in **Japanese**

### 4. Dev Server Management
- **No unauthorized startup**: Only start dev server when instructed
- **Use Port 3100**: Always use port 3100 for dev server

---

## Development Process

```
1. Research & understand existing code
2. Update documentation if needed
3. Clarify requirements (use AskUserQuestion)
4. Implement incrementally (stop at checkpoints)
5. Wait for manual testing
6. Wait for commit instruction
```

### Branch Strategy

| Branch | Purpose |
|--------|---------|
| `main` | Development |
| `feature/*` | New features |
| `fix/*` | Bug fixes |

---

## Technical Guidelines

### Canvas 2D Rendering
- Always validate radius values before `createRadialGradient` (must be >= 0)
- Use `Math.max(1, radius)` for safety
- Z-sort items before rendering for proper depth

### React Components
- Use `'use client'` directive for interactive components
- Canvas components need `useRef` for element access
- Animation loops should use `setInterval` in `useEffect` with cleanup

### Performance Considerations
- Use `useMemo` for expensive calculations (flatten operations)
- Limit animation frame rate (50ms intervals = 20fps)
- Consider LOD (Level of Detail) for large datasets

---

## Key Principles

1. **Ask when uncertain** - Use dialogue for clarification
2. **Keep docs current** - Update as you work
3. **Incremental progress** - Stop at checkpoints
4. **Test-driven** - Wait for manual test results
5. **Commit on instruction** - Never auto-commit
6. **Minimal scope** - Fix only what's necessary; avoid over-correction

---

## Commands

```bash
npm run dev      # Start dev server (port 3100)
npm run build    # Production build
npm run lint     # Run ESLint
```

---

## Repository Structure

```
closm-probe/
├── src/
│   ├── app/                    # Next.js App Router
│   │   ├── layout.tsx          # Root layout
│   │   ├── page.tsx            # Home page
│   │   └── globals.css         # Global styles
│   └── components/             # React components
│       └── PhysicalStorageUniverse.tsx  # Main 3D visualizer
├── docs/                       # Documentation
│   ├── README.md               # Documentation index
│   ├── design/
│   │   └── product-design.md   # Product design spec
│   └── specifications/
│       ├── metaphor-mapping.md # Directory ↔ Universe reference
│       └── canvas-rendering.md # Canvas 2D rendering spec
├── public/                     # Static assets
└── CLAUDE.md                   # This file
```

---

## Cross-Session Memory (claude-mem)

This project uses **claude-mem** plugin for persistent memory across sessions.

### Basic Workflow

```
1. Search → Get index with IDs
2. Timeline → Understand context around results
3. Fetch → Get full details by ID
```

### Common Commands

**Search past work**:
```
search(query="canvas rendering", limit=20, project="closm-probe")
```

**Get timeline context**:
```
timeline(anchor=123, depth_before=5, depth_after=5, project="closm-probe")
```

**Fetch observations (batch)**:
```
get_observations(ids=[123, 122, 121], orderBy="date_desc")
```

### Search Filters

| Parameter | Description |
|-----------|-------------|
| `query` | Search term |
| `limit` | Max results (default: 20) |
| `project` | Project name: `closm-probe` |
| `type` | "observations", "sessions", or "prompts" |
| `obs_type` | bugfix, feature, decision, discovery, change |

### Observation Types

- discovery - Code exploration, file reading
- change - File modifications
- feature - New functionality
- bugfix - Bug fixes
- decision - Architectural decisions

---

## Related Documents

| Document | Purpose |
|----------|---------|
| `docs/specifications/metaphor-mapping.md` | Directory ↔ Universe metaphor reference (single source of truth) |
| `docs/design/product-design.md` | Product design specification |
| `docs/specifications/canvas-rendering.md` | Canvas 2D rendering details |
| `src/components/` | Component implementations |
