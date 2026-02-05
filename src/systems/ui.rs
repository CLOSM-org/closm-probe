//! UI systems
//!
//! egui-based overlay UI with left sidebar (Claude/ChatGPT style).

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy_egui::{egui, EguiContexts};
use futures_lite::future;

/// Embedded font: Noto Sans JP (supports Japanese, CJK)
/// Download from: https://fonts.google.com/noto/specimen/Noto+Sans+JP
const NOTO_SANS_JP: &[u8] = include_bytes!("../../assets/fonts/NotoSansJP-Regular.ttf");

/// Configure fonts for international text support (Japanese, CJK, etc.)
/// Run once at startup.
pub fn setup_fonts(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    // Font setup
    let mut fonts = egui::FontDefinitions::default();

    // Add Noto Sans JP for CJK support
    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        egui::FontData::from_static(NOTO_SANS_JP).into(),
    );

    // Set priority: Noto Sans JP first, then default fonts
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans_jp".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("noto_sans_jp".to_owned());

    ctx.set_fonts(fonts);

    // Disable all default strokes/borders globally
    let mut style = (*ctx.style()).clone();
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
    style.visuals.window_stroke = egui::Stroke::NONE;
    ctx.set_style(style);
}

/// Sidebar colors (semi-transparent to show 3D scene behind)
fn sidebar_bg() -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(26, 26, 46, 230) // #1a1a2e @ 90%
}
fn sidebar_header_bg() -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(35, 35, 55, 240)
}
const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 180, 255);

/// Render left sidebar for startup state (Empty)
pub fn render_startup_ui(
    mut contexts: EguiContexts,
    mut dialog_task: ResMut<FileDialogTask>,
    layout: Res<UiLayout>,
) {
    let ctx = contexts.ctx_mut();
    let task_running = dialog_task.task.is_some();

    // Left sidebar (semi-transparent, no border)
    egui::SidePanel::left("sidebar")
        .resizable(false)
        .exact_width(layout.sidebar_width)
        .frame(
            egui::Frame::none()
                .fill(sidebar_bg())
                .stroke(egui::Stroke::NONE),
        )
        .show(ctx, |ui| {
            ui.add_space(16.0);

            // Header section
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.heading(egui::RichText::new("CLOSM Probe").color(egui::Color32::WHITE));
            });

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    egui::RichText::new("3D Storage Visualization")
                        .color(egui::Color32::from_rgb(160, 160, 180)),
                );
            });

            ui.add_space(24.0);

            // Open Folder button
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                let button = egui::Button::new(
                    egui::RichText::new("  Open Folder").color(egui::Color32::WHITE),
                )
                .fill(ACCENT_COLOR)
                .min_size(egui::vec2(layout.sidebar_width - 32.0, 36.0));

                if ui.add_enabled(!task_running, button).clicked() {
                    let task = IoTaskPool::get().spawn(async move {
                        let handle = rfd::AsyncFileDialog::new().pick_folder().await;
                        handle.map(|h| h.path().to_path_buf())
                    });
                    dialog_task.task = Some(task);
                }
            });

            if task_running {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.spinner();
                    ui.label(
                        egui::RichText::new("Selecting folder...")
                            .color(egui::Color32::from_rgb(160, 160, 180)),
                    );
                });
            }

            ui.add_space(24.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.separator();
            });

            // Empty history section
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    egui::RichText::new("Recent")
                        .color(egui::Color32::from_rgb(120, 120, 140))
                        .small(),
                );
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(24.0);
                ui.label(
                    egui::RichText::new("No recent folders")
                        .color(egui::Color32::from_rgb(100, 100, 120))
                        .italics(),
                );
            });

            // Spacer to push settings to bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.separator();
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    if ui
                        .button(egui::RichText::new(" Settings").color(egui::Color32::GRAY))
                        .clicked()
                    {
                        // TODO: Settings panel
                    }
                });
                ui.add_space(8.0);
            });
        });
}

/// Poll async file dialog task for completion
pub fn poll_file_dialog(
    mut dialog_task: ResMut<FileDialogTask>,
    mut pending_folder: ResMut<PendingFolderSelection>,
) {
    if let Some(ref mut task) = dialog_task.task {
        if let Some(result) = future::block_on(future::poll_once(task)) {
            if let Some(path) = result {
                pending_folder.path = Some(path);
            }
            dialog_task.task = None;
        }
    }
}

/// Check for pending folder selection and transition state
pub fn check_folder_selection(
    mut pending_folder: ResMut<PendingFolderSelection>,
    mut current_dir: ResMut<CurrentDirectory>,
    mut breadcrumb: ResMut<Breadcrumb>,
    mut history: ResMut<NavigationHistory>,
    mut next_state: ResMut<NextState<AppState>>,
    mut folder_events: EventWriter<FolderSelectedEvent>,
) {
    if let Some(path) = pending_folder.path.take() {
        current_dir.path = Some(path.clone());
        *breadcrumb = Breadcrumb::from_path(&path);

        // Add to history
        history.push(path.clone());

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

    // Position breadcrumb after sidebar
    let x_offset = layout.sidebar_width + layout.padding;

    egui::Area::new(egui::Id::new("breadcrumb"))
        .fixed_pos(egui::pos2(x_offset, layout.padding))
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(30, 30, 45, 200))
                .rounding(8.0)
                .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for (i, segment) in breadcrumb.segments.iter().enumerate() {
                            if i > 0 {
                                ui.label(
                                    egui::RichText::new(" > ")
                                        .color(egui::Color32::from_rgb(120, 120, 140)),
                                );
                            }

                            let is_last = i == breadcrumb.segments.len() - 1;
                            let text = if is_last {
                                egui::RichText::new(&segment.name)
                                    .strong()
                                    .color(egui::Color32::WHITE)
                            } else {
                                egui::RichText::new(&segment.name).color(ACCENT_COLOR)
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

/// Render left sidebar with history and selection details (Viewing state)
pub fn render_sidebar(
    mut contexts: EguiContexts,
    ui_state: Res<UiState>,
    celestials: Query<&CelestialBody>,
    layout: Res<UiLayout>,
    history: Res<NavigationHistory>,
    current_dir: Res<CurrentDirectory>,
    mut dialog_task: ResMut<FileDialogTask>,
    mut navigate_events: EventWriter<NavigateToEvent>,
) {
    let ctx = contexts.ctx_mut();
    let task_running = dialog_task.task.is_some();

    egui::SidePanel::left("sidebar")
        .resizable(false)
        .exact_width(layout.sidebar_width)
        .frame(
            egui::Frame::none()
                .fill(sidebar_bg())
                .stroke(egui::Stroke::NONE),
        )
        .show(ctx, |ui| {
            ui.add_space(16.0);

            // Header
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.heading(egui::RichText::new("CLOSM Probe").color(egui::Color32::WHITE));
            });

            ui.add_space(16.0);

            // Open Folder button
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                let button = egui::Button::new(
                    egui::RichText::new("  Open Folder").color(egui::Color32::WHITE),
                )
                .fill(ACCENT_COLOR)
                .min_size(egui::vec2(layout.sidebar_width - 32.0, 32.0));

                if ui.add_enabled(!task_running, button).clicked() {
                    let task = IoTaskPool::get().spawn(async move {
                        let handle = rfd::AsyncFileDialog::new().pick_folder().await;
                        handle.map(|h| h.path().to_path_buf())
                    });
                    dialog_task.task = Some(task);
                }
            });

            if task_running {
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.spinner();
                });
            }

            ui.add_space(16.0);

            // Current folder display
            if let Some(ref path) = current_dir.path {
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    egui::Frame::none()
                        .fill(sidebar_header_bg())
                        .rounding(4.0)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.set_width(layout.sidebar_width - 48.0);
                            let folder_name = path
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| "/".to_string());
                            ui.label(
                                egui::RichText::new(format!(" {}", folder_name))
                                    .color(egui::Color32::WHITE),
                            );
                        });
                });
            }

            ui.add_space(16.0);
            add_separator(ui, layout.sidebar_width);

            // History section
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    egui::RichText::new("Recent")
                        .color(egui::Color32::from_rgb(120, 120, 140))
                        .small(),
                );
            });
            ui.add_space(8.0);

            if history.entries.is_empty() {
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.label(
                        egui::RichText::new("No recent folders")
                            .color(egui::Color32::from_rgb(100, 100, 120))
                            .italics(),
                    );
                });
            } else {
                for entry in history.entries.iter().take(8) {
                    let folder_name = entry
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "/".to_string());

                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        if ui
                            .link(
                                egui::RichText::new(format!(" {}", folder_name))
                                    .color(egui::Color32::from_rgb(200, 200, 220)),
                            )
                            .clicked()
                        {
                            navigate_events.send(NavigateToEvent { path: entry.clone() });
                        }
                    });
                }
            }

            ui.add_space(16.0);
            add_separator(ui, layout.sidebar_width);

            // Selection section
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.label(
                    egui::RichText::new("Selected")
                        .color(egui::Color32::from_rgb(120, 120, 140))
                        .small(),
                );
            });
            ui.add_space(8.0);

            if let Some(entity) = ui_state.selected_entity {
                if let Ok(celestial) = celestials.get(entity) {
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(&celestial.name)
                                    .color(egui::Color32::WHITE)
                                    .strong(),
                            );
                            ui.label(
                                egui::RichText::new(format_size(celestial.size_bytes))
                                    .color(egui::Color32::from_rgb(160, 160, 180)),
                            );
                            ui.label(
                                egui::RichText::new(format_relative_time(celestial.modified))
                                    .color(egui::Color32::from_rgb(160, 160, 180)),
                            );
                        });
                    });
                }
            } else {
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.label(
                        egui::RichText::new("Click a celestial to select")
                            .color(egui::Color32::from_rgb(100, 100, 120))
                            .italics(),
                    );
                });
            }

            // Settings at bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(16.0);
                add_separator(ui, layout.sidebar_width);
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    if ui
                        .button(egui::RichText::new(" Settings").color(egui::Color32::GRAY))
                        .clicked()
                    {
                        // TODO: Settings panel
                    }
                });
                ui.add_space(8.0);
            });
        });
}

/// Helper to add separator line
fn add_separator(ui: &mut egui::Ui, width: f32) {
    ui.horizontal(|ui| {
        ui.add_space(16.0);
        ui.add(egui::Separator::default().horizontal().spacing(0.0));
        ui.add_space(width - 32.0);
    });
}

/// Render tooltip for hovered entity
pub fn render_tooltip(
    mut contexts: EguiContexts,
    ui_state: Res<UiState>,
    celestials: Query<(&CelestialBody, &GlobalTransform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    _windows: Query<&Window>,
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

    let Ok(viewport_position) =
        camera.world_to_viewport(camera_transform, transform.translation())
    else {
        return;
    };

    let ctx = contexts.ctx_mut();

    let tooltip_pos = egui::pos2(viewport_position.x + 20.0, viewport_position.y - 10.0);

    egui::Area::new(egui::Id::new("hover_tooltip"))
        .fixed_pos(tooltip_pos)
        .show(ctx, |ui| {
            egui::Frame::none()
                .fill(egui::Color32::from_rgba_unmultiplied(20, 20, 30, 230))
                .rounding(4.0)
                .inner_margin(egui::Margin::same(12.0))
                .show(ui, |ui| {
                    // Set minimum width to prevent excessive line wrapping
                    ui.set_min_width(200.0);
                    ui.set_max_width(400.0);

                    ui.label(
                        egui::RichText::new(&celestial.name)
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format_size(celestial.size_bytes))
                            .color(egui::Color32::from_rgb(180, 180, 200)),
                    );
                    ui.label(
                        egui::RichText::new(format_relative_time(celestial.modified))
                            .color(egui::Color32::from_rgb(160, 160, 180)),
                    );
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
