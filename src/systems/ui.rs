//! UI systems
//!
//! Each AppState has ONE system that renders ALL UI for that state:
//!   SidePanel (sidebar) + CentralPanel (settings page, if active)
//! This guarantees sidebar state changes and main content rendering
//! happen in the same frame with no ordering ambiguity.

use crate::components::*;
use crate::events::*;
use crate::resources::*;
use crate::states::*;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy_egui::{egui, EguiContexts};
use futures_lite::future;
use std::path::Path;

/// Embedded font: Noto Sans JP (supports Japanese, CJK)
const NOTO_SANS_JP: &[u8] = include_bytes!("../../assets/fonts/NotoSansJP-Regular.ttf");

/// Configure fonts for international text support (Japanese, CJK, etc.)
pub fn setup_fonts(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        egui::FontData::from_static(NOTO_SANS_JP).into(),
    );
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

    let mut style = (*ctx.style()).clone();
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
    style.visuals.window_stroke = egui::Stroke::NONE;
    ctx.set_style(style);
}

// ── Colors ──

fn sidebar_bg() -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(26, 26, 46, 230)
}
fn sidebar_header_bg() -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(35, 35, 55, 240)
}
const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 180, 255);
const FOOTER_HEIGHT: f32 = 44.0;

// ── Shared sidebar components ──

fn sidebar_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(sidebar_bg())
        .stroke(egui::Stroke::NONE)
}

fn render_identity(ui: &mut egui::Ui) {
    ui.add_space(16.0);
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
}

fn render_open_folder_button(
    ui: &mut egui::Ui,
    sidebar_width: f32,
    task_running: bool,
    dialog_task: &mut ResMut<FileDialogTask>,
) {
    ui.add_space(16.0);
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        let button = egui::Button::new(
            egui::RichText::new("  Open Folder").color(egui::Color32::WHITE),
        )
        .fill(ACCENT_COLOR)
        .min_size(egui::vec2(sidebar_width - 32.0, 36.0));

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
}

fn render_history_entries<F>(
    ui: &mut egui::Ui,
    history: &NavigationHistory,
    limit: usize,
    mut on_click: F,
) where
    F: FnMut(&std::path::PathBuf),
{
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
        for entry in history.entries.iter().take(limit) {
            let folder_name = entry
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "/".to_string());
            let path_hint = shorten_path(entry);

            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    if ui
                        .link(
                            egui::RichText::new(format!(" {}", folder_name))
                                .color(egui::Color32::from_rgb(200, 200, 220)),
                        )
                        .clicked()
                    {
                        on_click(entry);
                    }
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(&path_hint)
                                .color(egui::Color32::from_rgb(90, 90, 110))
                                .small(),
                        );
                    });
                });
            });
            ui.add_space(2.0);
        }
    }
}

/// Settings bar at the bottom of sidebar (Zone 3). Toggles MainView.
fn render_settings_bar(ui: &mut egui::Ui, ui_state: &mut UiState) {
    let is_active = ui_state.main_view == MainView::Settings;

    let bg = if is_active {
        egui::Color32::from_rgba_unmultiplied(60, 60, 90, 200)
    } else {
        egui::Color32::from_rgba_unmultiplied(35, 35, 55, 180)
    };
    let text_color = if is_active {
        egui::Color32::WHITE
    } else {
        egui::Color32::GRAY
    };

    egui::Frame::none()
        .fill(bg)
        .rounding(4.0)
        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if ui
                .add(
                    egui::Label::new(egui::RichText::new("Settings").color(text_color))
                        .sense(egui::Sense::click()),
                )
                .clicked()
            {
                ui_state.main_view = if is_active {
                    MainView::Universe
                } else {
                    MainView::Settings
                };
            }
        });
}

fn section_label(ui: &mut egui::Ui, text: &str) {
    ui.horizontal(|ui| {
        ui.add_space(16.0);
        ui.label(
            egui::RichText::new(text)
                .color(egui::Color32::from_rgb(120, 120, 140))
                .small(),
        );
    });
}

fn shorten_path(path: &Path) -> String {
    let display = path.to_string_lossy();
    let home = std::env::var("HOME").unwrap_or_default();
    let shortened = if !home.is_empty() && display.starts_with(&home) {
        format!("~{}", &display[home.len()..])
    } else {
        display.to_string()
    };
    if shortened.len() > 35 {
        let parts: Vec<&str> = shortened.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() > 3 {
            format!("{}/.../{}", parts[0], parts[parts.len() - 1])
        } else {
            shortened
        }
    } else {
        shortened
    }
}

// ── Settings page (shared helper, not a system) ──

/// Draw the settings page as a CentralPanel.
/// Called from within sidebar systems — same frame, no ordering issues.
fn draw_settings_page(
    ctx: &egui::Context,
    sidebar_settings: &mut SidebarSettings,
    theme_config: &mut ThemeConfig,
) {
    egui::CentralPanel::default()
        .frame(
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(18, 18, 32))
                .inner_margin(egui::Margin::same(32.0)),
        )
        .show(ctx, |ui| {
            let max_width = 500.0;
            let available = ui.available_width();
            let offset = ((available - max_width) / 2.0).max(0.0);

            ui.add_space(24.0);

            ui.horizontal(|ui| {
                ui.add_space(offset);
                ui.vertical(|ui| {
                    ui.set_max_width(max_width);

                    ui.heading(
                        egui::RichText::new("Settings")
                            .color(egui::Color32::WHITE)
                            .size(24.0),
                    );
                    ui.add_space(32.0);

                    // Appearance
                    ui.label(
                        egui::RichText::new("Appearance")
                            .color(egui::Color32::from_rgb(160, 160, 180))
                            .size(14.0)
                            .strong(),
                    );
                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.label(
                            egui::RichText::new("Theme")
                                .color(egui::Color32::from_rgb(200, 200, 220)),
                        );
                        ui.add_space(16.0);
                        let label = if theme_config.dark_mode {
                            "Dark"
                        } else {
                            "Light"
                        };
                        if ui
                            .button(
                                egui::RichText::new(label)
                                    .color(egui::Color32::from_rgb(200, 200, 220)),
                            )
                            .clicked()
                        {
                            theme_config.dark_mode = !theme_config.dark_mode;
                            theme_config.apply_mode();
                        }
                    });

                    ui.add_space(28.0);

                    // Display
                    ui.label(
                        egui::RichText::new("Display")
                            .color(egui::Color32::from_rgb(160, 160, 180))
                            .size(14.0)
                            .strong(),
                    );
                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.label(
                            egui::RichText::new("History limit")
                                .color(egui::Color32::from_rgb(200, 200, 220)),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        let mut limit = sidebar_settings.history_limit as f32;
                        let slider = egui::Slider::new(&mut limit, 10.0..=30.0)
                            .step_by(1.0)
                            .show_value(true);
                        if ui.add(slider).changed() {
                            sidebar_settings.history_limit = limit as usize;
                        }
                    });

                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.checkbox(
                            &mut sidebar_settings.show_hidden_files,
                            egui::RichText::new("Show hidden files")
                                .color(egui::Color32::from_rgb(200, 200, 220)),
                        );
                    });
                });
            });
        });
}

// ══════════════════════════════════════════════════════
//  State-specific systems: Sidebar + Main Content
//  Each renders ALL UI for its state in one function.
// ══════════════════════════════════════════════════════

/// Empty state: Sidebar + (Settings page if active)
pub fn render_startup_ui(
    mut contexts: EguiContexts,
    mut dialog_task: ResMut<FileDialogTask>,
    layout: Res<UiLayout>,
    history: Res<NavigationHistory>,
    mut pending_folder: ResMut<PendingFolderSelection>,
    mut sidebar_settings: ResMut<SidebarSettings>,
    mut ui_state: ResMut<UiState>,
    mut theme_config: ResMut<ThemeConfig>,
) {
    let ctx = contexts.ctx_mut();
    let task_running = dialog_task.task.is_some();

    // ── Sidebar ──
    egui::SidePanel::left("sidebar")
        .resizable(false)
        .exact_width(layout.sidebar_width)
        .frame(sidebar_frame())
        .show(ctx, |ui| {
            // Zone 1: Fixed Top
            render_identity(ui);
            ui.add_space(8.0);
            render_open_folder_button(ui, layout.sidebar_width, task_running, &mut dialog_task);
            ui.add_space(20.0);

            // Zone 2: Scrollable Middle
            let scroll_h = (ui.available_height() - FOOTER_HEIGHT).max(0.0);
            egui::ScrollArea::vertical()
                .max_height(scroll_h)
                .auto_shrink(false)
                .show(ui, |ui| {
                    section_label(ui, "Recent");
                    ui.add_space(8.0);

                    let limit = sidebar_settings.history_limit;
                    let mut clicked_path = None;
                    render_history_entries(ui, &history, limit, |entry| {
                        clicked_path = Some(entry.clone());
                    });
                    if let Some(path) = clicked_path {
                        pending_folder.path = Some(path);
                        ui_state.main_view = MainView::Universe;
                    }
                });

            // Zone 3: Fixed Bottom
            render_settings_bar(ui, &mut ui_state);
        });

    // ── Main Content (same frame, after sidebar) ──
    if ui_state.main_view == MainView::Settings {
        draw_settings_page(ctx, &mut sidebar_settings, &mut theme_config);
    }
}

/// Viewing state: Sidebar + (Settings page if active)
pub fn render_sidebar(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    celestials: Query<&CelestialBody>,
    layout: Res<UiLayout>,
    history: Res<NavigationHistory>,
    current_dir: Res<CurrentDirectory>,
    mut dialog_task: ResMut<FileDialogTask>,
    mut navigate_events: EventWriter<NavigateToEvent>,
    mut sidebar_settings: ResMut<SidebarSettings>,
    mut theme_config: ResMut<ThemeConfig>,
) {
    let ctx = contexts.ctx_mut();
    let task_running = dialog_task.task.is_some();

    // ── Sidebar ──
    egui::SidePanel::left("sidebar")
        .resizable(false)
        .exact_width(layout.sidebar_width)
        .frame(sidebar_frame())
        .show(ctx, |ui| {
            // Zone 1: Fixed Top
            render_identity(ui);
            ui.add_space(8.0);
            render_open_folder_button(ui, layout.sidebar_width, task_running, &mut dialog_task);

            ui.add_space(12.0);

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

            // Zone 2: Scrollable Middle
            let scroll_h = (ui.available_height() - FOOTER_HEIGHT).max(0.0);
            egui::ScrollArea::vertical()
                .max_height(scroll_h)
                .auto_shrink(false)
                .show(ui, |ui| {
                    section_label(ui, "Recent");
                    ui.add_space(8.0);

                    let limit = sidebar_settings.history_limit;
                    let mut clicked_path = None;
                    render_history_entries(ui, &history, limit, |entry| {
                        clicked_path = Some(entry.clone());
                    });
                    if let Some(path) = clicked_path {
                        navigate_events.send(NavigateToEvent { path });
                        ui_state.main_view = MainView::Universe;
                    }

                    ui.add_space(24.0);

                    // Context section (Selected)
                    section_label(ui, "Selected");
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
                                        egui::RichText::new(format_relative_time(
                                            celestial.modified,
                                        ))
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
                });

            // Zone 3: Fixed Bottom
            render_settings_bar(ui, &mut ui_state);
        });

    // ── Main Content (same frame, after sidebar) ──
    if ui_state.main_view == MainView::Settings {
        draw_settings_page(ctx, &mut sidebar_settings, &mut theme_config);
    }
}

// ── Non-UI systems ──

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
    persistent_cache: Option<Res<PersistentCache>>,
    mut ui_state: ResMut<UiState>,
) {
    if let Some(path) = pending_folder.path.take() {
        current_dir.path = Some(path.clone());
        *breadcrumb = Breadcrumb::from_path(&path);

        history.push(path.clone());

        if let Some(ref cache) = persistent_cache {
            cache.write_history(&history.entries);
        }

        folder_events.send(FolderSelectedEvent { path: path.clone() });
        next_state.set(AppState::Viewing);
        ui_state.main_view = MainView::Universe;

        info!("Folder selected: {}", path.display());
    }
}

// ── Overlays (Viewing + Universe only) ──

/// Render breadcrumb navigation overlay
pub fn render_breadcrumb(
    mut contexts: EguiContexts,
    breadcrumb: Res<Breadcrumb>,
    mut navigate_events: EventWriter<NavigateToEvent>,
    layout: Res<UiLayout>,
    ui_state: Res<UiState>,
) {
    if ui_state.main_view != MainView::Universe {
        return;
    }

    let ctx = contexts.ctx_mut();
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

/// Render tooltip for hovered entity
pub fn render_tooltip(
    mut contexts: EguiContexts,
    ui_state: Res<UiState>,
    celestials: Query<(&CelestialBody, &GlobalTransform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    _windows: Query<&Window>,
) {
    if ui_state.main_view != MainView::Universe {
        return;
    }

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

// ── Utility functions ──

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
