# Visual Encoding System

**Last Updated**: 2026-02-05

Detailed specification for how storage data maps to visual properties.

---

## 1. Solar System Model

### 1.1 Hierarchical Mapping

```
        ☉ Sun (Current Root Directory)
       /|\
      / | \
     🪐 🪐 🪐  Planets (1st level - directories/files)
     |
    /|\
   🌙🌙🌙  Moons (2nd level - children of planets)
```

### 1.2 Element Mapping

| Storage Element | Celestial Body | Visual Representation |
|-----------------|----------------|----------------------|
| Current Root | Sun | Center, glowing, pulsing |
| 1st Level Directory | Planet | Orbits sun |
| 2nd Level Items | Moon | Orbits parent planet |
| File | Satellite | Smaller sphere, colored by type |

### 1.3 Key Principles

| Principle | Description |
|-----------|-------------|
| **2-Level Display** | Only show 2 levels from current "sun" for clarity |
| **Flat Orbital Plane** | All celestial bodies on Y=0 (like real solar system) |
| **Drill-Down Navigation** | Double-click a planet to make it the new sun |

---

## 2. Polar Coordinate Mapping

Multiple dimensions encode information using polar coordinates:

| Polar Element | Attribute | Expression |
|---------------|-----------|------------|
| **r (radius)** | Creation date | New = close to sun, Old = outer orbit |
| **θ (angle)** | Last modified (sorted) | Equal spacing, sorted by recency |
| **Size** | File capacity | Larger file = larger body |
| **Color** | File type | Code=cyan, Image=orange, etc. |
| **Brightness** | Last modified | Recent = bright, old = dim |

---

## 3. Placement Algorithm

### 3.1 Angular Placement (θ)

Children are placed with **equal angular spacing**, sorted by last modified date:

```rust
// Sort children by last modified (newest first)
let mut sorted_children = children.clone();
sorted_children.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));

// Place with equal spacing, starting from 12 o'clock (-π/2)
for (index, child) in sorted_children.iter().enumerate() {
    let angle = -PI / 2.0 + (index as f32 / sorted_children.len() as f32) * PI * 2.0;
    // ... position calculation
}
```

**Result**:
- Beautiful equal-spaced arrangement
- Recently updated items start at top (12 o'clock direction)
- Clockwise progression toward older items

### 3.2 Radial Placement (r)

```rust
// Sort by creation date, place newer items in inner orbits
let mut sorted_by_creation = children.clone();
sorted_by_creation.sort_by(|a, b| b.created_at.cmp(&a.created_at));

// Assign orbit bands based on creation order
let orbit_index = sorted_by_creation.iter().position(|c| c.id == child.id).unwrap();
let orbit_radius = base_radius * (0.6 + (orbit_index as f32 / sorted_by_creation.len() as f32) * 0.8);
```

---

## 4. Size Encoding (Capacity)

Node size represents storage capacity using logarithmic scale:

### 4.1 Formula

```rust
fn calculate_body_radius(size: u64, body_type: BodyType) -> f32 {
    let (base_size, scale_factor, min_size) = match body_type {
        BodyType::Planet => (1.5, 0.3, 1000),
        BodyType::Satellite => (0.8, 0.2, 100),
    };

    base_size + (size.max(min_size) as f32).log10() * scale_factor
}
```

### 4.2 Reference Sizes

| Type | Base Size | Scale Factor |
|------|-----------|--------------|
| Planet (directory) | 1.5 | log10(size) × 0.3 |
| Satellite (file) | 0.8 | log10(size) × 0.2 |

---

## 5. Color Encoding (File Type)

### 5.1 File Type Colors

| File Type | Color | RGB |
|-----------|-------|-----|
| Code | Cyan | (0.38, 0.85, 0.98) |
| Design | Purple | (0.66, 0.33, 0.97) |
| Image | Orange | (0.96, 0.62, 0.04) |
| Video | Red | (0.94, 0.27, 0.27) |
| PDF | Dark Red | (0.86, 0.15, 0.15) |
| Document | Blue | (0.23, 0.51, 0.96) |
| Data | Teal | (0.02, 0.71, 0.83) |
| Archive | Gray | (0.42, 0.45, 0.50) |
| Directory | Violet | (0.55, 0.36, 0.96) |

### 5.2 Extension Mapping

```rust
fn get_file_type(name: &str) -> FileType {
    let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "rs" | "py" | "js" | "ts" | "go" | "c" | "cpp" => FileType::Code,
        "png" | "jpg" | "jpeg" | "gif" | "svg" => FileType::Image,
        "mp4" | "mov" | "avi" | "webm" => FileType::Video,
        "pdf" => FileType::Pdf,
        "doc" | "docx" | "txt" | "md" => FileType::Document,
        "json" | "yaml" | "toml" | "csv" => FileType::Data,
        "zip" | "tar" | "gz" | "7z" => FileType::Archive,
        _ => FileType::Unknown,
    }
}
```

---

## 6. Brightness Encoding (Recency)

### 6.1 Time-Based Brightness

| Last Modified | Brightness | Visual Effect |
|---------------|------------|---------------|
| Within 24h | 100% | Max brightness + pulse animation |
| Within 1 week | 85% | High brightness |
| Within 1 month | 70% | Medium brightness |
| Within 3 months | 55% | Low brightness |
| Within 1 year | 40% | Dim |
| Over 1 year | 25% | Minimum + grayed out |

### 6.2 Formula

```rust
fn calculate_brightness(last_modified: SystemTime) -> f32 {
    let age = SystemTime::now()
        .duration_since(last_modified)
        .unwrap_or_default();

    let days = age.as_secs() / 86400;

    match days {
        0 => 1.0,           // Within 24h
        1..=7 => 0.85,      // Within 1 week
        8..=30 => 0.70,     // Within 1 month
        31..=90 => 0.55,    // Within 3 months
        91..=365 => 0.40,   // Within 1 year
        _ => 0.25,          // Over 1 year
    }
}
```

---

## 7. Asteroid Belt Model

When a directory contains many small files, they are abstracted as an **asteroid belt**:

```
        ☉ Sun
       /|\
      🪐 🪐 🪐  Planets (directories)
       |
      ~~~  Asteroid Belt (small files consolidated)
       |
      🛰️ 🛰️  Large Satellites (significant files only)
```

### 7.1 Classification Rules

| Category | Criteria | Display |
|----------|----------|---------|
| Planet | Directory | Individual sphere |
| Large Satellite | File > threshold size | Individual sphere |
| Asteroid | File ≤ threshold | Consolidated into belt ring |

### 7.2 Asteroid Belt Behavior

- Displayed as a dotted ring around parent
- Shows count badge (e.g., "+12 files")
- Click to expand into individual view
- Reduces visual clutter while preserving information

---

## 8. UI Color Palette

### 8.1 Background Colors

| Usage | Color | RGB |
|-------|-------|-----|
| Deep space | Dark Navy | (0.04, 0.04, 0.10) |
| Shallow space | Dark Blue | (0.10, 0.10, 0.18) |

### 8.2 Accent Colors

| Usage | Color | RGB |
|-------|-------|-----|
| Primary | Purple | (0.66, 0.33, 0.97) |
| Secondary | Blue | (0.23, 0.51, 0.96) |
| Success | Green | (0.13, 0.77, 0.37) |
| Warning | Orange | (0.96, 0.62, 0.04) |
| Error | Red | (0.94, 0.27, 0.27) |

### 8.3 Text Colors

| Usage | Color | RGB |
|-------|-------|-----|
| Main text | White | (1.0, 1.0, 1.0) |
| Sub text | Gray | (0.53, 0.53, 0.53) |
