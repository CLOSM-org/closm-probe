# Bevy Technical Notes

Technical findings from initial prototyping.

---

## Environment

| Item | Version |
|------|---------|
| Bevy | 0.15 |
| Rust | Edition 2024 |
| Platform | macOS (Apple Silicon) |

---

## Bevy 0.15 Basic Structure

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)      // Runs once at start
        .add_systems(Update, update_loop) // Runs every frame
        .run();
}
```

### ECS Pattern

| Concept | Usage |
|---------|-------|
| Component | Data attached to entities (`#[derive(Component)]`) |
| Resource | Global state (`#[derive(Resource)]`) |
| System | Functions that query and modify data |
| Entity | ID that components attach to |

### Spawning 3D Objects (Bevy 0.15)

```rust
commands.spawn((
    Mesh3d(meshes.add(Sphere::new(1.0))),
    MeshMaterial3d(materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.2),
        emissive: color.into(),
        ..default()
    })),
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

### Lighting (Bevy 0.15)

```rust
// AmbientLight is a Resource, not a Component
commands.insert_resource(AmbientLight {
    color: Color::WHITE,
    brightness: 200.0,
});

// PointLight is spawned as entity
commands.spawn((
    PointLight {
        intensity: 2_000_000.0,
        shadows_enabled: true,
        ..default()
    },
    Transform::from_xyz(0.0, 5.0, 0.0),
));
```

---

## rfd (File Dialog) Issues on macOS

### Problem

Synchronous file dialogs (`rfd::FileDialog::new().pick_folder()`) **block the winit event loop** on macOS, causing the window to freeze.

**Symptom**: Window frame appears but content never renders (stuck in loading state).

### Root Cause

- rfd uses `runModal` for synchronous dialogs
- This blocks the main thread where winit runs its event loop
- Bevy depends on winit for window management

### Solutions

1. **Don't open dialogs at startup** - Wait for user action (key press, button click)
2. **Use async dialogs** - `rfd::AsyncFileDialog` with futures
3. **Use bevy_file_dialog plugin** - Handles async operations automatically

### Working Pattern: Async Dialog with IoTaskPool

```rust
use bevy::tasks::IoTaskPool;
use futures_lite::future;

// Resource to hold async task
#[derive(Resource, Default)]
pub struct FileDialogTask {
    pub task: Option<Task<Option<PathBuf>>>,
}

// Spawn async dialog on button click
fn open_folder_dialog(mut dialog_task: ResMut<FileDialogTask>) {
    let task = IoTaskPool::get().spawn(async move {
        let handle = rfd::AsyncFileDialog::new().pick_folder().await;
        handle.map(|h| h.path().to_path_buf())
    });
    dialog_task.task = Some(task);
}

// Poll task each frame
fn poll_file_dialog(
    mut dialog_task: ResMut<FileDialogTask>,
    mut pending: ResMut<PendingFolderSelection>,
) {
    if let Some(ref mut task) = dialog_task.task {
        if let Some(result) = future::block_on(future::poll_once(task)) {
            if let Some(path) = result {
                pending.path = Some(path);
            }
            dialog_task.task = None;
        }
    }
}
```

**Key points:**
- Use `rfd::AsyncFileDialog` (not `FileDialog`)
- Spawn on `IoTaskPool` for background execution
- Poll with `future::poll_once` (non-blocking)
- Store result in separate resource for state transition

### References

- [winit issue #3179](https://github.com/rust-windowing/winit/issues/3179)
- [egui discussion #5621](https://github.com/emilk/egui/discussions/5621)
- [bevy_file_dialog](https://github.com/richardhozak/bevy_file_dialog)

---

## File System Access

```rust
use std::fs;

if let Ok(entries) = fs::read_dir(&path) {
    for entry in entries.flatten() {
        let metadata = entry.metadata().ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let name = entry.file_name().to_string_lossy().to_string();
    }
}
```

---

## Performance Tips

| Technique | Purpose |
|-----------|---------|
| `opt-level = 1` in dev | Faster iteration with some optimization |
| `opt-level = 3` for deps | Full optimization for dependencies |
| `lto = "thin"` in release | Link-time optimization |

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
lto = "thin"
```
