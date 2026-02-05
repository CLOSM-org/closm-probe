//! Visual encoding components
//!
//! Components for visual representation of file metadata.

#![allow(dead_code)]

use bevy::prelude::*;

/// Brightness component based on modification time
///
/// Value range: 0.25 (oldest) to 1.0 (newest)
/// See visual.md for time-to-brightness mapping.
#[derive(Component, Debug, Clone)]
pub struct Brightness {
    /// Brightness value (0.25 - 1.0)
    pub value: f32,
}

impl Brightness {
    /// Create brightness from age in seconds
    pub fn from_age_seconds(age_secs: u64) -> Self {
        const HOUR: u64 = 3600;
        const DAY: u64 = 24 * HOUR;
        const WEEK: u64 = 7 * DAY;
        const MONTH: u64 = 30 * DAY;
        const QUARTER: u64 = 90 * DAY;
        const YEAR: u64 = 365 * DAY;

        let value = if age_secs < DAY {
            1.0       // 24 hours
        } else if age_secs < WEEK {
            0.85      // 1 week
        } else if age_secs < MONTH {
            0.70      // 1 month
        } else if age_secs < QUARTER {
            0.55      // 3 months
        } else if age_secs < YEAR {
            0.40      // 1 year
        } else {
            0.25      // older
        };

        Self { value }
    }
}

impl Default for Brightness {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

/// Grandchild abstraction ring
///
/// When a child folder has contents, we show a ring around
/// the planet to indicate nested items without displaying them.
#[derive(Component, Debug, Clone)]
pub struct GrandchildRing {
    /// Number of grandchild items
    pub count: usize,
}

/// Asteroid belt for overflow indication
///
/// When there are more than 20 items, excess items
/// are represented as an asteroid belt using particle effects.
#[derive(Component, Debug, Clone)]
pub struct AsteroidBelt {
    /// Number of items in the belt (items beyond the 20 limit)
    pub count: usize,
}
