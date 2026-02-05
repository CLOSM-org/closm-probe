//! Celestial body components
//!
//! Maps filesystem entities to universe metaphor:
//! - Current folder → Star (center)
//! - Child folder → Planet (sphere)
//! - Child file → Planet (octahedron shape)

#![allow(dead_code)]

use bevy::prelude::*;
use std::path::PathBuf;
use std::time::SystemTime;

/// Base component for all celestial entities
#[derive(Component, Debug, Clone)]
pub struct CelestialBody {
    /// Display name
    pub name: String,
    /// Full filesystem path
    pub path: PathBuf,
    /// Size in bytes
    pub size_bytes: u64,
    /// Last modification time
    pub modified: SystemTime,
}

/// Marker for the central star (current folder)
#[derive(Component, Debug, Default)]
pub struct Star;

/// Planet component (child folder or file)
#[derive(Component, Debug)]
pub struct Planet {
    /// True if this is a directory
    pub is_directory: bool,
}

/// File type classification for color encoding
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FileType {
    /// Source code files (.rs, .js, .py, etc.)
    Code,
    /// Image files (.png, .jpg, .gif, etc.)
    Image,
    /// Video files (.mp4, .mov, .avi, etc.)
    Video,
    /// Document files (.pdf, .doc, .txt, etc.)
    Document,
    /// Data files (.json, .xml, .csv, etc.)
    Data,
    /// Archive files (.zip, .tar, .gz, etc.)
    Archive,
    /// Directory
    #[default]
    Directory,
}

impl FileType {
    /// Get the color for this file type (from visual.md)
    pub fn color(&self) -> Color {
        match self {
            FileType::Code => Color::srgb_u8(0x61, 0xda, 0xfb),      // Cyan #61dafb
            FileType::Image => Color::srgb_u8(0xf5, 0x9e, 0x0b),     // Orange #f59e0b
            FileType::Video => Color::srgb_u8(0xef, 0x44, 0x44),     // Red #ef4444
            FileType::Document => Color::srgb_u8(0x3b, 0x82, 0xf6),  // Blue #3b82f6
            FileType::Data => Color::srgb_u8(0x06, 0xb6, 0xd4),      // Teal #06b6d4
            FileType::Archive => Color::srgb_u8(0x6b, 0x72, 0x80),   // Gray #6b7280
            FileType::Directory => Color::WHITE,
        }
    }

    /// Classify file by extension
    pub fn from_extension(ext: Option<&str>) -> Self {
        match ext.map(|e| e.to_lowercase()).as_deref() {
            // Code
            Some("rs" | "js" | "ts" | "py" | "go" | "java" | "c" | "cpp" | "h" | "hpp"
                | "rb" | "php" | "swift" | "kt" | "scala" | "sh" | "bash" | "zsh"
                | "html" | "css" | "scss" | "sass" | "less" | "vue" | "jsx" | "tsx") => FileType::Code,
            // Image
            Some("png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff" | "heic") => FileType::Image,
            // Video
            Some("mp4" | "mov" | "avi" | "mkv" | "webm" | "flv" | "wmv" | "m4v") => FileType::Video,
            // Document
            Some("pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "md" | "markdown" | "tex") => FileType::Document,
            // Data
            Some("json" | "xml" | "yaml" | "yml" | "csv" | "toml" | "ini" | "cfg" | "conf" | "sql" | "db" | "sqlite") => FileType::Data,
            // Archive
            Some("zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "dmg" | "iso") => FileType::Archive,
            // Default to Document for unknown
            _ => FileType::Document,
        }
    }
}
