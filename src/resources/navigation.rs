//! Navigation resources
//!
//! Track current directory and navigation history.

#![allow(dead_code)]

use bevy::prelude::*;
use std::path::PathBuf;

/// Current directory being visualized
#[derive(Resource, Debug, Default)]
pub struct CurrentDirectory {
    /// Active directory path (None if no folder selected)
    pub path: Option<PathBuf>,
}

/// Breadcrumb navigation path
#[derive(Resource, Debug, Default)]
pub struct Breadcrumb {
    /// Path segments from root to current
    pub segments: Vec<PathSegment>,
}

/// Single breadcrumb segment
#[derive(Debug, Clone)]
pub struct PathSegment {
    /// Display name
    pub name: String,
    /// Full path to this segment
    pub path: PathBuf,
}

impl Breadcrumb {
    /// Build breadcrumb from a path
    pub fn from_path(path: &PathBuf) -> Self {
        let mut segments = Vec::new();
        let mut current = path.clone();

        // Collect all ancestors
        let mut paths = vec![current.clone()];
        while let Some(parent) = current.parent() {
            if parent.as_os_str().is_empty() {
                break;
            }
            paths.push(parent.to_path_buf());
            current = parent.to_path_buf();
        }

        // Reverse to get root-first order
        paths.reverse();

        for p in paths {
            let name = p
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "/".to_string());
            segments.push(PathSegment { name, path: p });
        }

        Self { segments }
    }
}

/// Navigation history for recent folders and back/forward navigation
#[derive(Resource, Debug)]
pub struct NavigationHistory {
    /// Recent folders (for sidebar display), newest first
    pub entries: Vec<PathBuf>,
    /// Maximum entries to keep
    pub max_entries: usize,
    /// Paths for "back" navigation
    pub back: Vec<PathBuf>,
    /// Paths for "forward" navigation
    pub forward: Vec<PathBuf>,
}

impl Default for NavigationHistory {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 10,
            back: Vec::new(),
            forward: Vec::new(),
        }
    }
}

impl NavigationHistory {
    /// Push path to recent entries and back history
    pub fn push(&mut self, path: PathBuf) {
        // Add to recent entries (remove duplicates, newest first)
        self.entries.retain(|p| p != &path);
        self.entries.insert(0, path.clone());
        self.entries.truncate(self.max_entries);

        // Add to back history
        self.back.push(path);
        self.forward.clear();
    }

    /// Go back one step
    pub fn go_back(&mut self, current: PathBuf) -> Option<PathBuf> {
        if let Some(prev) = self.back.pop() {
            self.forward.push(current);
            Some(prev)
        } else {
            None
        }
    }

    /// Go forward one step
    pub fn go_forward(&mut self, current: PathBuf) -> Option<PathBuf> {
        if let Some(next) = self.forward.pop() {
            self.back.push(current);
            Some(next)
        } else {
            None
        }
    }
}
