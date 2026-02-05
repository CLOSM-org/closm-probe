//! macOS directory size calculation
//!
//! Uses `du` command for fast directory size calculation.

use super::SizeResult;
use bevy::log::info;
use crossbeam_channel::Sender;
use std::path::PathBuf;
use std::process::Command;
use std::thread;

/// Spawn background calculations using du command
pub fn spawn_calculations(paths: Vec<PathBuf>, sender: Sender<SizeResult>) {
    info!("Spawning size calculations for {} directories", paths.len());

    // Use std::thread for reliable background execution
    thread::spawn(move || {
        for path in paths {
            let size = calculate_with_du(&path);
            let _ = sender.send(SizeResult { path, size });
        }
    });
}

/// Calculate directory size using `du -sk` command
/// Returns size in bytes
fn calculate_with_du(path: &PathBuf) -> u64 {
    let path_str = match path.to_str() {
        Some(s) => s,
        None => return calculate_fallback(path),
    };

    // du -sk: summarize, kilobytes
    let output = Command::new("du")
        .args(["-sk", path_str])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            parse_du_output(&String::from_utf8_lossy(&output.stdout))
        }
        _ => calculate_fallback(path),
    }
}

/// Parse du output: "12345\t/path/to/dir\n"
/// Returns size in bytes (du outputs kilobytes)
fn parse_du_output(output: &str) -> u64 {
    output
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .map(|kb| kb * 1024) // Convert KB to bytes
        .unwrap_or(0)
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
    fn test_parse_du_output() {
        // du -sk output format: "12345\t/path/to/dir\n"
        let output = "12345\t/Users/test/dir\n";
        assert_eq!(parse_du_output(output), 12345 * 1024); // KB to bytes
    }

    #[test]
    fn test_parse_du_empty_output() {
        assert_eq!(parse_du_output(""), 0);
    }

    #[test]
    fn test_parse_du_with_spaces_in_path() {
        let output = "100\t/Users/test/path with spaces\n";
        assert_eq!(parse_du_output(output), 100 * 1024);
    }
}
