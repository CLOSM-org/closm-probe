# Detailed Design

Rust/Bevy implementation design documents.

---

## Design Documents

| Document | Content |
|----------|---------|
| [ECS Architecture](./ecs-architecture.md) | Components, Resources, States |
| [Scene Graph](./scene-graph.md) | Bundles, spawning, hierarchy |
| [Camera](./camera.md) | Orbital controls, animation |
| [UI](./ui.md) | egui panels, tooltips, theme |
| [Size Calculation](./size-calculation.md) | Platform-specific size calculation |

---

## Module Structure

```
src/
├── main.rs           # App entry, plugin registration
├── states.rs         # AppState, ViewingMode
├── events.rs         # All event definitions
├── bundles.rs        # StarBundle, PlanetBundle, etc.
├── components/       # ECS components
│   ├── celestial.rs
│   ├── interaction.rs
│   └── visual.rs
├── resources/        # Global state
│   ├── navigation.rs
│   ├── cache.rs
│   ├── ui_state.rs
│   └── config.rs
├── systems/          # ECS systems
│   ├── setup.rs
│   ├── cleanup.rs
│   ├── filesystem.rs
│   ├── spawning.rs
│   ├── camera.rs
│   ├── interaction.rs
│   ├── ui.rs
│   └── size_calculation/  # Platform-specific
│       ├── mod.rs
│       ├── spotlight.rs   # macOS
│       └── jwalk.rs       # Fallback
└── utils/
    └── visual_encoding.rs
```

---

## Implementation Status

| Phase | Status |
|-------|--------|
| Phase 1: Foundation | ✅ Complete |
| Phase 2: Scene Graph | ✅ Complete |
| Phase 3: Camera | ✅ Complete |
| Phase 4: UI | ✅ Complete |
| Phase 5: Size Calculation | 🚧 In Progress |

---

## See Also

- [Requirements](../requirements/index.md) - Requirements specification
- [Reference](../reference/bevy-notes.md) - Bevy technical notes
