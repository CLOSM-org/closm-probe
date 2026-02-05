//! jwalk-based parallel directory size calculation
//!
//! Fallback implementation for non-macOS platforms.

#![cfg(not(target_os = "macos"))]

use super::SizeResult;
use bevy::tasks::IoTaskPool;
use crossbeam_channel::Sender;
use std::path::PathBuf;

/// Spawn background calculations using jwalk parallel traversal
pub fn spawn_calculations(paths: Vec<PathBuf>, sender: Sender<SizeResult>) {
    IoTaskPool::get()
        .spawn(async move {
            for path in paths {
                let size = calculate_with_jwalk(&path);
                let _ = sender.send(SizeResult { path, size });
            }
        })
        .detach();
}

/// Calculate directory size using jwalk parallel traversal
fn calculate_with_jwalk(path: &PathBuf) -> u64 {
    jwalk::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}
