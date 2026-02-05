//! Directory cache resources
//!
//! LRU cache for directory contents to avoid repeated filesystem reads.

use bevy::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};

/// Single file entry from filesystem
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// File or directory name
    pub name: String,
    /// Full path
    pub path: PathBuf,
    /// Size in bytes
    pub size_bytes: u64,
    /// Last modification time
    pub modified: SystemTime,
    /// True if directory
    pub is_directory: bool,
}

/// Cached directory contents
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Directory entries
    pub entries: Vec<FileEntry>,
    /// When this entry was cached
    pub timestamp: Instant,
}

impl CacheEntry {
    /// Check if entry has expired
    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.timestamp.elapsed() > ttl
    }
}

/// LRU cache for directory contents
#[derive(Resource, Debug)]
pub struct DirectoryCache {
    /// Cached entries by path
    cache: HashMap<PathBuf, CacheEntry>,
    /// Maximum number of cached directories
    max_size: usize,
    /// Time-to-live for cache entries
    ttl: Duration,
    /// Access order for LRU eviction
    access_order: Vec<PathBuf>,
}

impl Default for DirectoryCache {
    fn default() -> Self {
        Self {
            cache: HashMap::new(),
            max_size: 50,
            ttl: Duration::from_secs(30),
            access_order: Vec::new(),
        }
    }
}

impl DirectoryCache {
    /// Get cached directory contents if valid
    pub fn get(&mut self, path: &PathBuf) -> Option<Vec<FileEntry>> {
        let ttl = self.ttl;

        // Check if exists and not expired
        if let Some(entry) = self.cache.get(path) {
            if !entry.is_expired(ttl) {
                // Update access order
                self.access_order.retain(|p| p != path);
                self.access_order.push(path.clone());
                return Some(entry.entries.clone());
            }
        }

        // Check again for removal (separate borrow)
        if self.cache.get(path).map_or(false, |e| e.is_expired(ttl)) {
            self.cache.remove(path);
            self.access_order.retain(|p| p != path);
        }

        None
    }

    /// Insert directory contents into cache
    pub fn insert(&mut self, path: PathBuf, entries: Vec<FileEntry>) {
        // Evict oldest if at capacity
        while self.cache.len() >= self.max_size {
            if let Some(oldest) = self.access_order.first().cloned() {
                self.cache.remove(&oldest);
                self.access_order.remove(0);
            } else {
                break;
            }
        }

        // Insert new entry
        self.cache.insert(
            path.clone(),
            CacheEntry {
                entries,
                timestamp: Instant::now(),
            },
        );
        self.access_order.push(path);
    }

    /// Invalidate a specific path
    pub fn invalidate(&mut self, path: &PathBuf) {
        self.cache.remove(path);
        self.access_order.retain(|p| p != path);
    }

    /// Clear entire cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }
}
