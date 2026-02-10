# Persistent Cache Design

Two-tier caching: in-memory `DirectoryCache` (fast, volatile) + redb `PersistentCache` (disk, ACID).

---

## Architecture

```
┌─────────────────────────────────────────┐
│            ECS Systems                   │
│  spawning / size_calculation / ui        │
├─────────┬───────────────────────────────┤
│ Tier 1  │  DirectoryCache (in-memory)    │
│         │  LRU 50 entries, 30s TTL       │
├─────────┼───────────────────────────────┤
│ Tier 2  │  PersistentCache (redb)        │
│         │  ACID, 3600s TTL               │
│         │  Background writer thread       │
└─────────┴───────────────────────────────┘
```

---

## redb Tables

| Table | Key | Value | Purpose |
|-------|-----|-------|---------|
| `sizes` | `&str` (path) | `(u64, u64)` (size, epoch_secs) | Directory size cache |
| `history` | `u64` (index) | `&str` (path) | Navigation history (0=newest) |

---

## Data Flow

### Size Cache

```
spawn_celestials()
  ├─ persistent_cache.get_size(path)
  │   ├─ HIT (within TTL): use cached size, no pulse animation
  │   └─ MISS: pulse animation + queue du calculation
  │
update_celestial_sizes()
  └─ on du result: persistent_cache.write_size(path, size)
      └─ non-blocking channel → background writer → redb commit
```

### History

```
check_folder_selection() / handle_drilldown() / handle_navigate_to()
  └─ history.push(path)
  └─ persistent_cache.write_history(entries)
      └─ non-blocking channel → background writer → redb commit

initialize_persistent_cache() [startup]
  └─ persistent_cache.load_history()
  └─ filter: only paths that still exist on disk
  └─ populate NavigationHistory.entries
```

---

## Graceful Degradation

`PersistentCache::new()` returns `Option<Self>`. All system parameters use `Option<Res<PersistentCache>>`.

| Failure | Behavior |
|---------|----------|
| DB creation fails | Warning logged, app runs without persistence |
| Write channel full | Write dropped silently (cache is advisory) |
| DB file deleted | Fresh cache on next launch |
| Corrupted DB | redb handles recovery or `new()` returns None |

---

## Storage Location

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/closm-probe/cache.redb` |
| Linux | `~/.local/share/closm-probe/cache.redb` |
| Windows | `{FOLDERID_LocalAppData}/closm-probe/data/cache.redb` |

---

## Resource Definition

```rust
#[derive(Resource)]
pub struct PersistentCache {
    db: Arc<Database>,              // Thread-safe DB handle
    write_sender: Sender<Cmd>,      // Non-blocking write channel
    size_ttl_secs: u64,             // Cache TTL (default 3600s)
}
```

---

## Not In Scope

- Cache eviction for old entries
- File system change detection / invalidation
- User settings persistence
