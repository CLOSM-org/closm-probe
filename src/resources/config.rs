//! Configuration resources
//!
//! Visual encoding and theme settings.

use bevy::prelude::*;

/// Visual encoding configuration
#[derive(Resource, Debug)]
pub struct VisualConfig {
    /// Size scale range for directories
    pub dir_size_min: f32,
    pub dir_size_max: f32,
    /// Size scale range for files
    pub file_size_min: f32,
    pub file_size_max: f32,
    /// Fixed size for the star (current folder)
    pub star_size: f32,
    /// Maximum items before asteroid belt
    pub max_display_items: usize,
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            dir_size_min: 0.5,
            dir_size_max: 2.0,
            file_size_min: 0.3,
            file_size_max: 1.8,
            star_size: 2.5,
            max_display_items: 20,
        }
    }
}

/// Theme color palette
#[derive(Debug, Clone)]
pub struct ThemeColors {
    /// Background color
    pub background: Color,
    /// Primary text color
    pub text: Color,
    /// Secondary text color
    pub text_secondary: Color,
    /// Accent color
    pub accent: Color,
    /// UI panel background
    pub panel_bg: Color,
    /// Selection highlight
    pub selection: Color,
}

impl ThemeColors {
    /// Dark cosmic theme (space colors)
    pub fn dark_cosmic() -> Self {
        Self {
            background: Color::srgb_u8(15, 15, 25),
            text: Color::srgb_u8(240, 240, 250),
            text_secondary: Color::srgb_u8(160, 160, 180),
            accent: Color::srgb_u8(100, 180, 255),
            panel_bg: Color::srgba_u8(30, 30, 45, 230),
            selection: Color::srgba_u8(100, 180, 255, 100),
        }
    }

    /// Light cosmic theme (sky colors)
    pub fn light_cosmic() -> Self {
        Self {
            background: Color::srgb_u8(240, 245, 255),
            text: Color::srgb_u8(20, 25, 40),
            text_secondary: Color::srgb_u8(80, 85, 100),
            accent: Color::srgb_u8(50, 120, 200),
            panel_bg: Color::srgba_u8(255, 255, 255, 240),
            selection: Color::srgba_u8(50, 120, 200, 80),
        }
    }
}

/// Theme configuration
#[derive(Resource, Debug)]
pub struct ThemeConfig {
    /// Use dark mode
    pub dark_mode: bool,
    /// Current color palette
    pub colors: ThemeColors,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            dark_mode: true,
            colors: ThemeColors::dark_cosmic(),
        }
    }
}

impl ThemeConfig {
    /// Update colors based on dark_mode setting
    pub fn apply_mode(&mut self) {
        self.colors = if self.dark_mode {
            ThemeColors::dark_cosmic()
        } else {
            ThemeColors::light_cosmic()
        };
    }
}

/// Camera configuration
#[derive(Resource, Debug)]
pub struct CameraConfig {
    /// Minimum zoom distance
    pub zoom_min: f32,
    /// Maximum zoom distance
    pub zoom_max: f32,
    /// Pitch limit in degrees (prevents gimbal lock)
    pub pitch_limit: f32,
    /// Drilldown animation duration in seconds
    pub drilldown_duration: f32,
    /// Return animation duration in seconds
    pub return_duration: f32,
    /// View reset animation duration in seconds
    pub reset_duration: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            zoom_min: 5.0,
            zoom_max: 100.0,
            pitch_limit: 80.0,
            drilldown_duration: 0.8,
            return_duration: 0.6,
            reset_duration: 0.5,
        }
    }
}
