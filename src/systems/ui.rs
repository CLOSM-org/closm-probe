//! UI systems
//!
//! egui-based overlay UI.

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Render startup screen with "Open Folder" button
pub fn render_startup_ui(
    mut contexts: EguiContexts,
    mut pending_folder: ResMut<PendingFolderSelection>,
    _theme: Res<ThemeConfig>,
) {
    let ctx = contexts.ctx_mut();

    // Center panel
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 2.0 - 50.0);

                ui.heading("CLOSM Probe");
                ui.add_space(20.0);
                ui.label("3D Storage Visualization");
                ui.add_space(30.0);

                if ui.button("📂 Open Folder").clicked() {
                    // Open file dialog (sync for now, async later)
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        pending_folder.path = Some(path);
                    }
                }
            });
        });
}

/// Check for pending folder selection and transition state
pub fn check_folder_selection(
    mut pending_folder: ResMut<PendingFolderSelection>,
    mut current_dir: ResMut<CurrentDirectory>,
    mut breadcrumb: ResMut<Breadcrumb>,
    mut ui_state: ResMut<UiState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut folder_events: EventWriter<FolderSelectedEvent>,
) {
    if let Some(path) = pending_folder.path.take() {
        current_dir.path = Some(path.clone());
        *breadcrumb = Breadcrumb::from_path(&path);
        ui_state.show_startup = false;

        folder_events.send(FolderSelectedEvent { path: path.clone() });
        next_state.set(AppState::Viewing);

        info!("Folder selected: {}", path.display());
    }
}

/// Render breadcrumb navigation overlay
pub fn render_breadcrumb(
    mut contexts: EguiContexts,
    breadcrumb: Res<Breadcrumb>,
    mut navigate_events: EventWriter<NavigateToEvent>,
    layout: Res<UiLayout>,
) {
    let ctx = contexts.ctx_mut();

    egui::Area::new(egui::Id::new("breadcrumb"))
        .fixed_pos(egui::pos2(layout.padding, layout.padding))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(30, 30, 45, 200))
                .rounding(8.0)
                .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for (i, segment) in breadcrumb.segments.iter().enumerate() {
                            if i > 0 {
                                ui.label(" > ");
                            }

                            let is_last = i == breadcrumb.segments.len() - 1;
                            let text = if is_last {
                                egui::RichText::new(&segment.name).strong()
                            } else {
                                egui::RichText::new(&segment.name)
                            };

                            if ui.link(text).clicked() && !is_last {
                                navigate_events.send(NavigateToEvent {
                                    path: segment.path.clone(),
                                });
                            }
                        }
                    });
                });
        });
}

/// Render sidebar with selection details
pub fn render_sidebar(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    celestials: Query<&CelestialBody>,
    layout: Res<UiLayout>,
    windows: Query<&Window>,
) {
    let ctx = contexts.ctx_mut();
    let Ok(window) = windows.get_single() else {
        return;
    };

    // Sidebar toggle button
    let button_pos = egui::pos2(
        window.width() - layout.padding - 40.0,
        layout.padding,
    );

    egui::Area::new(egui::Id::new("sidebar_toggle"))
        .fixed_pos(button_pos)
        .show(ctx, |ui| {
            if ui.button("≡").clicked() {
                ui_state.sidebar_open = !ui_state.sidebar_open;
            }
        });

    // Sidebar panel
    if ui_state.sidebar_open {
        egui::SidePanel::right("sidebar")
            .resizable(false)
            .default_width(layout.sidebar_width)
            .show(ctx, |ui| {
                ui.heading("Details");
                ui.separator();

                if let Some(entity) = ui_state.selected_entity {
                    if let Ok(celestial) = celestials.get(entity) {
                        ui.label(format!("Name: {}", celestial.name));
                        ui.label(format!("Size: {}", format_size(celestial.size_bytes)));
                        ui.label(format!(
                            "Modified: {}",
                            format_time(celestial.modified)
                        ));
                        ui.label(format!("Path: {}", celestial.path.display()));
                    }
                } else {
                    ui.label("No selection");
                    ui.label("Click a celestial body to view details.");
                }

                ui.separator();

                ui.collapsing("Settings", |ui| {
                    ui.label("(Coming soon)");
                });
            });
    }
}

/// Render tooltip for hovered entity
pub fn render_tooltip(
    mut contexts: EguiContexts,
    ui_state: Res<UiState>,
    celestials: Query<(&CelestialBody, &GlobalTransform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let Some(hovered_entity) = ui_state.hovered_entity else {
        return;
    };

    let Ok((celestial, transform)) = celestials.get(hovered_entity) else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let Ok(_window) = windows.get_single() else {
        return;
    };

    // Convert 3D position to screen coordinates
    let Ok(viewport_position) = camera.world_to_viewport(camera_transform, transform.translation())
    else {
        return;
    };

    let ctx = contexts.ctx_mut();

    // Offset tooltip slightly from cursor
    let tooltip_pos = egui::pos2(
        viewport_position.x + 20.0,
        viewport_position.y - 10.0,
    );

    egui::Area::new(egui::Id::new("hover_tooltip"))
        .fixed_pos(tooltip_pos)
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(20, 20, 30, 230))
                .rounding(4.0)
                .inner_margin(egui::Margin::same(8.0))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&celestial.name).strong());
                    ui.label(format_size(celestial.size_bytes));
                    ui.label(format_relative_time(celestial.modified));
                });
        });
}

/// Format file size for display
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format time for display
fn format_time(time: std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;

    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();

    // Simple formatting - could use chrono for better formatting
    let days = secs / 86400;
    let years = 1970 + (days / 365);
    let month = ((days % 365) / 30) + 1;
    let day = (days % 30) + 1;

    format!("{}-{:02}-{:02}", years, month, day)
}

/// Format relative time (e.g., "2 hours ago")
fn format_relative_time(time: std::time::SystemTime) -> String {
    let age = time
        .elapsed()
        .map(|d| d.as_secs())
        .unwrap_or(u64::MAX);

    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * MINUTE;
    const DAY: u64 = 24 * HOUR;
    const WEEK: u64 = 7 * DAY;
    const MONTH: u64 = 30 * DAY;
    const YEAR: u64 = 365 * DAY;

    if age < MINUTE {
        "just now".to_string()
    } else if age < HOUR {
        format!("{} min ago", age / MINUTE)
    } else if age < DAY {
        format!("{} hours ago", age / HOUR)
    } else if age < WEEK {
        format!("{} days ago", age / DAY)
    } else if age < MONTH {
        format!("{} weeks ago", age / WEEK)
    } else if age < YEAR {
        format!("{} months ago", age / MONTH)
    } else {
        format!("{} years ago", age / YEAR)
    }
}
