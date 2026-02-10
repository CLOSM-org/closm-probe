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
| [Persistent Cache](./persistent-cache.md) | redb two-tier cache, history persistence |

---

## Module Structure

```
src/
├── main.rs           # App entry, plugin registration
├── states.rs         # AppState, ViewingMode
├── events.rs         # All event definitions
├── bundles.rs        # StarBundle, PlanetBundle, etc.
├── components/       # ECS components
│   ├── mod.rs
│   ├── celestial.rs
│   ├── interaction.rs
│   ├── visual.rs
│   └── animation.rs      # PulseAnimation
├── resources/        # Global state
│   ├── mod.rs
│   ├── navigation.rs
│   ├── cache.rs
│   ├── persistent_cache.rs  # redb two-tier cache
│   ├── ui_state.rs
│   └── config.rs
├── systems/          # ECS systems
│   ├── mod.rs
│   ├── setup.rs
│   ├── cleanup.rs
│   ├── filesystem.rs
│   ├── spawning.rs
│   ├── camera.rs
│   ├── interaction.rs
│   ├── ui.rs
│   └── size_calculation/  # Platform-specific
│       ├── mod.rs
│       ├── macos_du.rs    # macOS (du command)
│       └── jwalk_calc.rs  # Fallback (jwalk)
└── utils/
    ├── mod.rs
    ├── visual_encoding.rs
    └── viewport.rs
```

---

## Implementation Status

| Phase | Status |
|-------|--------|
| Phase 1: Foundation | ✅ Complete |
| Phase 2: Scene Graph | ✅ Complete |
| Phase 3: Camera | ✅ Complete |
| Phase 4: UI | ✅ Complete |
| Phase 5: Size Calculation | ✅ Complete |

---

## See Also

- [Requirements](../requirements/index.md) - Requirements specification
- [Reference](../reference/bevy-notes.md) - Bevy technical notes
