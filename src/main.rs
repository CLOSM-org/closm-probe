//! CLOSM Probe - 3D Storage Visualization
//!
//! Represents file systems as explorable universe spaces.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod bundles;
mod components;
mod events;
mod resources;
mod states;
mod systems;
mod utils;

use events::*;
use resources::*;
use states::*;
use systems::*;

fn main() {
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CLOSM Probe".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        // Resources (must be registered before state systems)
        .init_resource::<CurrentDirectory>()
        .init_resource::<Breadcrumb>()
        .init_resource::<NavigationHistory>()
        .init_resource::<DirectoryCache>()
        .init_resource::<UiState>()
        .init_resource::<UiLayout>()
        .init_resource::<VisualConfig>()
        .init_resource::<CameraConfig>()
        .init_resource::<PendingFolderSelection>()
        .init_resource::<FileDialogTask>()
        .init_resource::<interaction::ClickState>()
        // States
        .init_state::<AppState>()
        .add_sub_state::<ViewingMode>()
        // Events
        .add_event::<FolderSelectedEvent>()
        .add_event::<DrillDownEvent>()
        .add_event::<DrillUpEvent>()
        .add_event::<SelectionChangedEvent>()
        .add_event::<NavigateToEvent>()
        .add_event::<ViewResetEvent>()
        .add_event::<RespawnCelestialsEvent>()
        // Startup systems
        .add_systems(Startup, (setup_theme, setup_fonts))
        // Global systems (run in all states)
        .add_systems(Update, update_camera_viewport)
        // State: Empty
        .add_systems(OnEnter(AppState::Empty), setup_camera)
        .add_systems(
            Update,
            (render_startup_ui, poll_file_dialog, check_folder_selection)
                .run_if(in_state(AppState::Empty)),
        )
        // State: Viewing
        .add_systems(OnEnter(AppState::Viewing), spawn_celestials)
        .add_systems(OnExit(AppState::Viewing), cleanup_viewing)
        .add_systems(
            Update,
            (
                update_hover,
                handle_selection,
                handle_drilldown,
                handle_keyboard,
                handle_navigate_to,
                handle_respawn_celestials,
                render_breadcrumb,
                render_sidebar,
                render_tooltip,
            )
                .run_if(in_state(AppState::Viewing)),
        )
        // SubState: Animating
        .add_systems(
            Update,
            animate_camera.run_if(in_state(ViewingMode::Animating)),
        )
        .add_systems(
            Update,
            handle_view_reset.run_if(in_state(ViewingMode::Idle)),
        )
        .run();
}
