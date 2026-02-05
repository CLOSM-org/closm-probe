# CLOSM Probe

3D storage visualization tool that represents file systems as explorable universe spaces.

## Architecture

- **Engine**: Bevy 0.15 (Rust game engine with ECS)
- **Graphics**: wgpu (via Bevy)
- **UI**: egui (via bevy_egui)
- **Effects**: bevy_hanabi (GPU particles)

## Development

```bash
# Run in development mode
cargo run

# Run with release optimizations
cargo run --release

# Build release binary
cargo build --release
```

## Design Documents

- [Product Design](docs/design/product-design.md)
- [Metaphor Mapping](docs/specifications/metaphor-mapping.md)
