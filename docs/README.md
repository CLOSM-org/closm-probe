# CLOSM Probe Documentation

Documentation hub for CLOSM Probe - the 3D storage universe explorer.

---

## Directory Structure

```
docs/
├── README.md                           # This file
├── requirements/                       # Requirements
│   └── product-requirements.md         # Product requirements document
├── design/                             # Detailed Design
│   ├── visual-encoding.md              # Visual encoding specification
│   └── metaphor-mapping.md             # Directory ↔ Universe mapping
└── reference/                          # Reference
    └── bevy-notes.md                   # Bevy/Rust technical notes
```

---

## Quick Links

### Requirements

| Document | Description |
|----------|-------------|
| [Product Requirements](./requirements/product-requirements.md) | Vision, users, features, roadmap |

### Detailed Design

| Document | Description |
|----------|-------------|
| [Visual Encoding](./design/visual-encoding.md) | How data maps to visual properties |
| [Metaphor Mapping](./design/metaphor-mapping.md) | Directory ↔ Universe mapping reference |

### Reference

| Document | Description |
|----------|-------------|
| [Bevy Notes](./reference/bevy-notes.md) | Rust/Bevy implementation notes |

---

## Core Concept

**CLOSM Probe** visualizes storage as a 3D universe space:

| Storage | Universe |
|---------|----------|
| Directory | Planet |
| File | Satellite |
| Size | Body size |
| Recency | Brightness |
| Type | Color |

See [Product Requirements](./requirements/product-requirements.md) for full vision and [Visual Encoding](./design/visual-encoding.md) for detailed specifications.

---

## Contributing

When updating documentation:

1. Keep documents in **English**
2. Use **metaphor terminology** (planet, satellite, celestial body)
3. Update related documents when making changes
4. Follow markdown best practices
