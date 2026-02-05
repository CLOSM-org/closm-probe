//! Filesystem reading systems
//!
//! Synchronous directory reading using std::fs.

use crate::resources::{DirectoryCache, FileEntry};
use std::fs;
use std::path::PathBuf;

/// Read directory contents, using cache if available
pub fn read_directory(path: &PathBuf, cache: &mut DirectoryCache) -> Vec<FileEntry> {
    // Check cache first
    if let Some(entries) = cache.get(path) {
        return entries;
    }

    // Read from filesystem
    let entries = read_directory_sync(path);

    // Cache the result
    cache.insert(path.clone(), entries.clone());

    entries
}

/// Synchronous directory reading
fn read_directory_sync(path: &PathBuf) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {
            let entry_path = entry.path();
            let name = entry
                .file_name()
                .to_string_lossy()
                .to_string();

            // Skip hidden files (Unix convention)
            if name.starts_with('.') {
                continue;
            }

            if let Ok(metadata) = entry.metadata() {
                let file_entry = FileEntry {
                    name,
                    path: entry_path,
                    size_bytes: if metadata.is_dir() {
                        // For directories, we could calculate total size
                        // but that's expensive. Use 0 for now.
                        0
                    } else {
                        metadata.len()
                    },
                    modified: metadata.modified().unwrap_or(std::time::UNIX_EPOCH),
                    is_directory: metadata.is_dir(),
                };
                entries.push(file_entry);
            }
        }
    }

    // Sort: directories first, then by name (case-insensitive)
    entries.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    entries
}

/// Count items in a directory (for grandchild ring)
pub fn count_directory_items(path: &PathBuf) -> usize {
    fs::read_dir(path)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| {
                    !e.file_name()
                        .to_string_lossy()
                        .starts_with('.')
                })
                .count()
        })
        .unwrap_or(0)
}

/// Calculate directory size recursively (expensive, use sparingly)
pub fn calculate_directory_size(path: &PathBuf) -> u64 {
    let mut total = 0u64;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let metadata = entry.metadata().ok();
            if let Some(meta) = metadata {
                if meta.is_file() {
                    total += meta.len();
                } else if meta.is_dir() {
                    // Recursion - careful with deep directories
                    total += calculate_directory_size(&entry.path());
                }
            }
        }
    }

    total
}
