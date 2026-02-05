//! macOS Spotlight-based directory size calculation
//!
//! Uses mdfind to query Spotlight index for fast size calculation.

use super::SizeResult;
use bevy::tasks::IoTaskPool;
use crossbeam_channel::Sender;
use std::path::PathBuf;
use std::process::Command;

/// Spawn background calculations using Spotlight
pub fn spawn_calculations(paths: Vec<PathBuf>, sender: Sender<SizeResult>) {
    IoTaskPool::get()
        .spawn(async move {
            for path in paths {
                let size = calculate_with_spotlight(&path);
                let _ = sender.send(SizeResult { path, size });
            }
        })
        .detach();
}

/// Calculate directory size using Spotlight's mdfind command
fn calculate_with_spotlight(path: &PathBuf) -> u64 {
    let path_str = match path.to_str() {
        Some(s) => s,
        None => return calculate_fallback(path),
    };

    // Use mdfind to query all files with size info in the directory
    // mdfind -onlyin /path "kMDItemFSSize > 0" -attr kMDItemFSSize
    let output = Command::new("mdfind")
        .args(["-onlyin", path_str])
        .arg("kMDItemFSSize >= 0")
        .arg("-attr")
        .arg("kMDItemFSSize")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            parse_mdfind_output(&String::from_utf8_lossy(&output.stdout))
        }
        _ => calculate_fallback(path),
    }
}

/// Parse mdfind output to extract total size
/// Output format:
/// /path/to/file
/// kMDItemFSSize = 12345
/// /path/to/another/file
/// kMDItemFSSize = 67890
fn parse_mdfind_output(output: &str) -> u64 {
    let mut total = 0u64;

    for line in output.lines() {
        if line.starts_with("kMDItemFSSize") {
            // Parse "kMDItemFSSize = 12345" or "kMDItemFSSize = (null)"
            if let Some(value_str) = line.split('=').nth(1) {
                let trimmed = value_str.trim();
                if trimmed != "(null)" {
                    if let Ok(size) = trimmed.parse::<u64>() {
                        total += size;
                    }
                }
            }
        }
    }

    total
}

/// Fallback to standard filesystem traversal if Spotlight fails
fn calculate_fallback(path: &PathBuf) -> u64 {
    use std::fs;

    let mut total = 0u64;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total += metadata.len();
                } else if metadata.is_dir() {
                    total += calculate_fallback(&entry.path());
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mdfind_output() {
        let output = r#"/Users/test/file1.txt
kMDItemFSSize = 1234
/Users/test/file2.txt
kMDItemFSSize = 5678
/Users/test/empty.txt
kMDItemFSSize = (null)
"#;
        assert_eq!(parse_mdfind_output(output), 1234 + 5678);
    }

    #[test]
    fn test_parse_empty_output() {
        assert_eq!(parse_mdfind_output(""), 0);
    }
}
