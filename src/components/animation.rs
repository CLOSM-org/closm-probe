//! Animation components for visual feedback

use bevy::prelude::*;

/// Pulsing animation for pending calculations
#[derive(Component, Debug)]
pub struct PulseAnimation {
    pub elapsed: f32,
    pub frequency: f32, // Hz
    pub min_alpha: f32,
    pub max_alpha: f32,
}

impl Default for PulseAnimation {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            frequency: 2.0,  // 2Hz = 0.5s cycle
            min_alpha: 0.4,
            max_alpha: 1.0,
        }
    }
}

/// Marker component for entities waiting for size calculation
#[derive(Component, Debug)]
pub struct PendingSizeCalculation;
