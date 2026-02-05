# Camera Design

Orbital camera system using bevy_panorbit_camera.

---

## Camera Controller

### bevy_panorbit_camera Integration

Uses `bevy_panorbit_camera` 0.22 for orbital controls.

| Feature | Description |
|---------|-------------|
| Orbit rotation | Left mouse drag |
| Zoom | Scroll wheel |
| Pan | Right mouse drag |
| Smooth interpolation | Built-in |

### Configuration

```rust
PanOrbitCamera {
    radius: Some(20.0),      // Initial distance
    pitch: Some(0.4),        // ~23 degrees
    yaw: Some(0.0),          // Initial rotation
    pitch_lower_limit: Some(-80°), // Prevent gimbal lock
    pitch_upper_limit: Some(80°),
    zoom_lower_limit: 5.0,   // Minimum distance
    zoom_upper_limit: Some(100.0), // Maximum distance
}
```

---

## Constraints

| Parameter | Value | Reason |
|-----------|-------|--------|
| Zoom min | 5.0 | Prevent clipping through celestials |
| Zoom max | 100.0 | Keep scene visible |
| Pitch limit | ±80° | Prevent gimbal lock |

---

## Animation System

### CameraAnimation Component

```rust
struct CameraAnimation {
    target: Vec3,        // Focus point
    target_radius: f32,  // Target distance
    progress: f32,       // 0.0 - 1.0
    duration: f32,       // Seconds
    start_radius: f32,   // Initial distance
    start_focus: Vec3,   // Initial focus
}
```

### Animation Types

| Type | Duration | Target | Trigger |
|------|----------|--------|---------|
| Drilldown | 800ms | Selected planet | Double-click directory |
| Return | 600ms | Origin | Double-click star |
| View Reset | 500ms | Origin (default view) | Space key |

### Easing Function

```rust
// Cubic ease-out: smooth deceleration
fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
```

---

## State Integration

### ViewingMode SubState

| State | Camera Behavior |
|-------|-----------------|
| Idle | User controls enabled |
| Animating | Controls disabled, animation running |

### Transition Flow

```
User double-clicks planet
    → Set ViewingMode::Animating
    → Add CameraAnimation component
    → animate_camera system runs each frame
    → On progress >= 1.0:
        → Remove CameraAnimation
        → Set ViewingMode::Idle
        → Spawn new celestials
```

---

## Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `setup_camera` | OnEnter(Empty) | Initialize camera and constraints |
| `animate_camera` | Update in Animating | Process animation |
| `handle_view_reset` | Update in Idle | Respond to Space key |

---

## See Also

- [ECS Architecture](./ecs-architecture.md) - CameraConfig resource
- [Requirements: UI/UX](../requirements/ui-ux.md) - Interaction mapping
