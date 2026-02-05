# CLOSM Probe Development Guidelines

**Last Updated**: 2026-02-05

Project rules and guidelines for Claude Code assistance.

---

## Project Overview

**CLOSM Probe**: 3D storage visualization tool that represents file systems as explorable universe spaces.

| Item | Value |
|------|-------|
| Repository | closm-probe |
| Design Doc | `docs/design/product-design.md` |

### Tech Stack

| Category | Technology |
|----------|------------|
| Language | Rust (Edition 2024) |
| Engine | Bevy 0.15 (ECS game engine) |
| Graphics | wgpu (via Bevy) |
| UI | egui (via bevy_egui) |
| Particles | bevy_hanabi (GPU particles) |

---

## Absolute Rules

### 1. Commit Policy
- **Never commit without explicit instruction**
- Wait for user approval before committing

### 2. Build Policy
- **Never run `cargo build` or `cargo run` without instruction**
- Exception: Quick syntax checks with `cargo check` are allowed

### 3. Documentation Policy
- Keep docs up-to-date when requirements change
- Remove outdated or confusing content
- Docs should be in **English** (for token efficiency)
- Conversations remain in **Japanese**

### 4. Metaphor Mapping Policy (CRITICAL)
- **`docs/specifications/metaphor-mapping.md` is the Single Source of Truth**
- This file defines ALL metaphor mappings (universe ↔ storage)
- **Bidirectional sync required**:
  - When implementation changes → Update metaphor-mapping.md
  - When metaphor-mapping.md changes → Update implementation
- **Before implementing visual/spatial changes**: Check metaphor-mapping.md first
- **TBD items**: Undefined mappings must be decided and documented before implementation

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

### Bevy ECS Patterns
- Use Components for data, Systems for logic
- Prefer Bundles for related components
- Use Events for decoupled communication
- Use Resources for global state

### Performance Considerations
- Use `Query` filters to minimize iteration
- Consider LOD (Level of Detail) for large file systems
- Use `bevy_hanabi` for particle effects (GPU-accelerated)
- Profile with `bevy_diagnostic` plugin

### File System Access
- Use `std::fs` for native file system operations
- Handle errors gracefully with `Result`
- Consider async for large directory scans

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
cargo check      # Quick syntax/type check
cargo run        # Run in debug mode
cargo run --release  # Run with optimizations
cargo build --release  # Build release binary
cargo clippy     # Lint with Clippy
cargo fmt        # Format code
```

---

## Repository Structure

```
closm-probe/
├── src/
│   └── main.rs             # Application entry point
├── docs/                   # Documentation
│   ├── README.md           # Documentation index
│   ├── design/
│   │   └── product-design.md   # Product design spec
│   └── specifications/
│       └── metaphor-mapping.md # Directory ↔ Universe reference
├── Cargo.toml              # Rust package manifest
└── CLAUDE.md               # This file
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
search(query="bevy rendering", limit=20, project="closm-probe")
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
| **`docs/specifications/metaphor-mapping.md`** | **CRITICAL: Single Source of Truth for all metaphor definitions** |
| `docs/design/product-design.md` | Product design specification |
