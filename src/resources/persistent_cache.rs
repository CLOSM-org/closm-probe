//! Persistent cache using redb
//!
//! Two-tier cache: in-memory DirectoryCache (fast, volatile) + redb (persistent, ACID).
//! Background writer thread handles all writes non-blocking via crossbeam channel.

use bevy::prelude::*;
use crossbeam_channel::{Sender, TrySendError};
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Table: path (string) -> (size_bytes: u64, timestamp_epoch_secs: u64)
const SIZE_TABLE: TableDefinition<&str, (u64, u64)> = TableDefinition::new("sizes");

/// Table: index (u64) -> path (string), 0 = newest
const HISTORY_TABLE: TableDefinition<u64, &str> = TableDefinition::new("history");

/// Table: path (string) -> pinned_timestamp (u64)
const PINNED_TABLE: TableDefinition<&str, u64> = TableDefinition::new("pinned");

/// Table: setting_name (string) -> setting_value (string)
const SETTINGS_TABLE: TableDefinition<&str, &str> = TableDefinition::new("settings");

/// Eviction threshold: 30 days in seconds
const EVICTION_TTL_SECS: u64 = 30 * 24 * 60 * 60;

/// Commands sent to the background writer thread
enum CacheWriteCommand {
    WriteSize {
        path: String,
        size: u64,
        timestamp: u64,
    },
    WriteHistory {
        entries: Vec<String>,
    },
    WritePinned {
        path: String,
        pinned: bool,
    },
    WriteSetting {
        key: String,
        value: String,
    },
}

/// Persistent cache resource backed by redb
#[derive(Resource)]
pub struct PersistentCache {
    db: Arc<Database>,
    write_sender: Sender<CacheWriteCommand>,
    size_ttl_secs: u64,
}

impl PersistentCache {
    /// Initialize persistent cache at platform data directory.
    /// Returns None if DB creation fails (app continues without persistence).
    pub fn new(ttl_secs: u64) -> Option<Self> {
        let db_path = Self::db_path()?;

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                warn!("Failed to create cache directory: {}", e);
                return None;
            }
        }

        let db = match Database::create(&db_path) {
            Ok(db) => Arc::new(db),
            Err(e) => {
                warn!("Failed to open persistent cache: {}", e);
                return None;
            }
        };

        // Ensure tables exist
        {
            let write_txn = match db.begin_write() {
                Ok(txn) => txn,
                Err(e) => {
                    warn!("Failed to begin write transaction: {}", e);
                    return None;
                }
            };
            if write_txn.open_table(SIZE_TABLE).is_err() {
                warn!("Failed to create size table");
                return None;
            }
            if write_txn.open_table(HISTORY_TABLE).is_err() {
                warn!("Failed to create history table");
                return None;
            }
            if write_txn.open_table(PINNED_TABLE).is_err() {
                warn!("Failed to create pinned table");
                return None;
            }
            if write_txn.open_table(SETTINGS_TABLE).is_err() {
                warn!("Failed to create settings table");
                return None;
            }
            if let Err(e) = write_txn.commit() {
                warn!("Failed to commit table creation: {}", e);
                return None;
            }
        }

        // Spawn background writer thread
        let (sender, receiver) = crossbeam_channel::bounded::<CacheWriteCommand>(64);
        let writer_db = Arc::clone(&db);

        std::thread::Builder::new()
            .name("persistent-cache-writer".into())
            .spawn(move || {
                while let Ok(cmd) = receiver.recv() {
                    match cmd {
                        CacheWriteCommand::WriteSize {
                            path,
                            size,
                            timestamp,
                        } => {
                            if let Ok(write_txn) = writer_db.begin_write() {
                                if let Ok(mut table) = write_txn.open_table(SIZE_TABLE) {
                                    let _ = table.insert(path.as_str(), (size, timestamp));
                                }
                                let _ = write_txn.commit();
                            }
                        }
                        CacheWriteCommand::WriteHistory { entries } => {
                            if let Ok(write_txn) = writer_db.begin_write() {
                                if let Ok(mut table) = write_txn.open_table(HISTORY_TABLE) {
                                    // Clear existing entries
                                    let keys: Vec<u64> = table
                                        .iter()
                                        .ok()
                                        .map(|iter| {
                                            iter.filter_map(|r| r.ok())
                                                .map(|(k, _)| k.value())
                                                .collect()
                                        })
                                        .unwrap_or_default();
                                    for key in keys {
                                        let _ = table.remove(key);
                                    }
                                    // Write new entries (0 = newest)
                                    for (i, entry) in entries.iter().enumerate() {
                                        let _ = table.insert(i as u64, entry.as_str());
                                    }
                                }
                                let _ = write_txn.commit();
                            }
                        }
                        CacheWriteCommand::WritePinned { path, pinned } => {
                            if let Ok(write_txn) = writer_db.begin_write() {
                                if let Ok(mut table) = write_txn.open_table(PINNED_TABLE) {
                                    if pinned {
                                        let now = SystemTime::now()
                                            .duration_since(UNIX_EPOCH)
                                            .map(|d| d.as_secs())
                                            .unwrap_or(0);
                                        let _ = table.insert(path.as_str(), now);
                                    } else {
                                        let _ = table.remove(path.as_str());
                                    }
                                }
                                let _ = write_txn.commit();
                            }
                        }
                        CacheWriteCommand::WriteSetting { key, value } => {
                            if let Ok(write_txn) = writer_db.begin_write() {
                                if let Ok(mut table) = write_txn.open_table(SETTINGS_TABLE) {
                                    let _ = table.insert(key.as_str(), value.as_str());
                                }
                                let _ = write_txn.commit();
                            }
                        }
                    }
                }
            })
            .ok()?;

        info!(
            "Persistent cache initialized at {}",
            db_path.display()
        );

        Some(Self {
            db,
            write_sender: sender,
            size_ttl_secs: ttl_secs,
        })
    }

    /// Get cached directory size if not expired
    pub fn get_size(&self, path: &Path) -> Option<u64> {
        let path_str = path.to_string_lossy();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs();

        let read_txn = self.db.begin_read().ok()?;
        let table = read_txn.open_table(SIZE_TABLE).ok()?;
        let entry = table.get(path_str.as_ref()).ok()??;
        let (size, timestamp) = entry.value();

        // TTL check
        if now.saturating_sub(timestamp) > self.size_ttl_secs {
            return None;
        }

        Some(size)
    }

    /// Queue a size write (non-blocking, drops if channel full)
    pub fn write_size(&self, path: &Path, size: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let cmd = CacheWriteCommand::WriteSize {
            path: path.to_string_lossy().into_owned(),
            size,
            timestamp: now,
        };

        if let Err(TrySendError::Full(_)) = self.write_sender.try_send(cmd) {
            warn!("Persistent cache write channel full, dropping size write");
        }
    }

    /// Load navigation history from persistent storage (synchronous, startup only)
    pub fn load_history(&self) -> Vec<PathBuf> {
        let read_txn = match self.db.begin_read() {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let table = match read_txn.open_table(HISTORY_TABLE) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };

        let mut entries: Vec<(u64, PathBuf)> = Vec::new();
        if let Ok(iter) = table.iter() {
            for item in iter {
                if let Ok((key, value)) = item {
                    entries.push((key.value(), PathBuf::from(value.value())));
                }
            }
        }

        // Sort by index (0 = newest)
        entries.sort_by_key(|(idx, _)| *idx);
        entries.into_iter().map(|(_, path)| path).collect()
    }

    /// Queue history write (non-blocking, drops if channel full)
    pub fn write_history(&self, entries: &[PathBuf]) {
        let string_entries: Vec<String> = entries
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();

        let cmd = CacheWriteCommand::WriteHistory {
            entries: string_entries,
        };

        if let Err(TrySendError::Full(_)) = self.write_sender.try_send(cmd) {
            warn!("Persistent cache write channel full, dropping history write");
        }
    }

    /// Load pinned folders from persistent storage (synchronous, startup only)
    pub fn load_pinned(&self) -> Vec<PathBuf> {
        let read_txn = match self.db.begin_read() {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let table = match read_txn.open_table(PINNED_TABLE) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };

        let mut entries: Vec<(u64, PathBuf)> = Vec::new();
        if let Ok(iter) = table.iter() {
            for item in iter {
                if let Ok((key, value)) = item {
                    entries.push((value.value(), PathBuf::from(key.value())));
                }
            }
        }

        // Sort by pinned timestamp descending (most recently pinned first)
        entries.sort_by(|a, b| b.0.cmp(&a.0));
        entries.into_iter().map(|(_, path)| path).collect()
    }

    /// Queue pin/unpin write (non-blocking, drops if channel full)
    pub fn write_pinned(&self, path: &Path, pinned: bool) {
        let cmd = CacheWriteCommand::WritePinned {
            path: path.to_string_lossy().into_owned(),
            pinned,
        };

        if let Err(TrySendError::Full(_)) = self.write_sender.try_send(cmd) {
            warn!("Persistent cache write channel full, dropping pin write");
        }
    }

    /// Load a setting from persistent storage (synchronous, startup only)
    pub fn load_setting(&self, key: &str) -> Option<String> {
        let read_txn = self.db.begin_read().ok()?;
        let table = read_txn.open_table(SETTINGS_TABLE).ok()?;
        let entry = table.get(key).ok()??;
        Some(entry.value().to_owned())
    }

    /// Queue a setting write (non-blocking, drops if channel full)
    pub fn write_setting(&self, key: &str, value: &str) {
        let cmd = CacheWriteCommand::WriteSetting {
            key: key.to_owned(),
            value: value.to_owned(),
        };

        if let Err(TrySendError::Full(_)) = self.write_sender.try_send(cmd) {
            warn!("Persistent cache write channel full, dropping setting write");
        }
    }

    /// Evict size cache entries older than EVICTION_TTL_SECS.
    /// Runs synchronously (call from background thread at startup).
    pub fn evict_old_entries(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let write_txn = match self.db.begin_write() {
            Ok(txn) => txn,
            Err(e) => {
                warn!("Cache eviction: failed to begin write transaction: {}", e);
                return;
            }
        };

        let mut table = match write_txn.open_table(SIZE_TABLE) {
            Ok(t) => t,
            Err(e) => {
                warn!("Cache eviction: failed to open size table: {}", e);
                return;
            }
        };

        // Collect stale keys
        let stale_keys: Vec<String> = table
            .iter()
            .ok()
            .map(|iter| {
                iter.filter_map(|r| r.ok())
                    .filter(|(_, v)| {
                        let (_, timestamp) = v.value();
                        now.saturating_sub(timestamp) > EVICTION_TTL_SECS
                    })
                    .map(|(k, _)| k.value().to_owned())
                    .collect()
            })
            .unwrap_or_default();

        let eviction_count = stale_keys.len();
        for key in stale_keys {
            let _ = table.remove(key.as_str());
        }

        drop(table);
        let _ = write_txn.commit();

        if eviction_count > 0 {
            info!("Cache eviction: removed {} stale entries", eviction_count);
        }
    }

    /// Platform-correct database path
    fn db_path() -> Option<PathBuf> {
        let proj_dirs = directories::ProjectDirs::from("", "", "closm-probe")?;
        Some(proj_dirs.data_dir().join("cache.redb"))
    }
}
