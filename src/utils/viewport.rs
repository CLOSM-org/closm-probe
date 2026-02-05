//! Viewport coordinate utilities
//!
//! Handle coordinate conversion between window and viewport coordinate systems.

use bevy::prelude::*;

/// Convert window cursor position to viewport-local coordinates.
///
/// When a camera has a viewport set, `viewport_to_world()` expects coordinates
/// relative to the viewport's top-left corner, not the window's.
///
/// Returns `None` if cursor is outside the viewport (e.g., over sidebar).
pub fn window_to_viewport_cursor(
    window_cursor: Vec2,
    camera: &Camera,
    window: &Window,
) -> Option<Vec2> {
    // If no viewport is set, window coordinates are viewport coordinates
    let Some(viewport) = &camera.viewport else {
        return Some(window_cursor);
    };

    let scale = window.scale_factor();

    // Convert physical pixels to logical pixels for consistency
    let vp_pos = viewport.physical_position.as_vec2() / scale;
    let vp_size = viewport.physical_size.as_vec2() / scale;

    // Transform to viewport-local coordinates
    let vp_cursor = window_cursor - vp_pos;

    // Check if cursor is within viewport bounds
    if vp_cursor.x < 0.0
        || vp_cursor.y < 0.0
        || vp_cursor.x >= vp_size.x
        || vp_cursor.y >= vp_size.y
    {
        return None;
    }

    Some(vp_cursor)
}
