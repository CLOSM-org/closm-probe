# CLOSM Probe Documentation

Documentation hub for CLOSM Probe - the 3D storage universe explorer.

---

## Quick Links

| Document | Description |
|----------|-------------|
| [Metaphor Mapping](./specifications/metaphor-mapping.md) | Directory ↔ Universe metaphor reference |
| [Product Design](./design/product-design.md) | Full product design (metaphor-centric) |
| [Canvas Rendering](./specifications/canvas-rendering.md) | 3D rendering technical specification |

---

## Directory Structure

```
docs/
├── README.md                     # This file (documentation index)
├── design/                       # Design specifications
│   └── product-design.md         # Product design document
└── specifications/               # Technical specifications
    ├── metaphor-mapping.md       # Directory ↔ Universe metaphor reference
    └── canvas-rendering.md       # Canvas 2D rendering details
```

---

## Product Design Overview

The product design document is organized around the **Physical Space Metaphor** - the core innovation of CLOSM Probe.

### Document Structure

| Part | Focus | Key Sections |
|------|-------|--------------|
| **Part I: The Universe Metaphor** | Core concept | Vision, Solar System Model, Visual Encoding |
| **Part II: Navigating the Universe** | Interaction | Exploration, Discovery, Information Architecture |
| **Part III: Universe Features** | Capabilities | Core & Extended Features |
| **Part IV: Building the Universe** | Technical | Rendering, Data, System Architecture |
| **Part V: Bringing to Users** | Strategy | Users, Roadmap, Market, Metrics |
| **Appendices** | Reference | Formulas, Risks, References |

### Core Metaphor

See **[Metaphor Mapping Reference](./specifications/metaphor-mapping.md)** for complete directory ↔ universe mappings, visual encoding, and file references.

---

## Technical Specifications

Implementation details and coding guidelines.

### [Canvas Rendering](./specifications/canvas-rendering.md)

3D rendering implementation using Canvas 2D API:
- 3D projection (planets/satellites in space)
- Gradient effects (planet glow, satellite shine)
- Animation system (orbital motion, pulse effects)
- User interaction (hit detection, rotation, zoom)
- Performance optimization (LOD, virtualization)

---

## Contributing

When updating documentation:

1. Keep documents in **English** (for token efficiency)
2. Use **metaphor terminology** consistently (planet, satellite, celestial body)
3. Update related documents when making changes
4. Remove outdated content
5. Follow markdown best practices

---

## Related Files

| File | Location | Description |
|------|----------|-------------|
| CLAUDE.md | Project root | Claude Code development guidelines |
