use std::path::Path;

use crate::rules::util::{read_file, rust_files};

pub fn check_workspace_readability(crates_root: impl AsRef<Path>, errors: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(crates_root.as_ref()) else {
        return;
    };

    for entry in entries.flatten() {
        let crate_path = entry.path();
        if !crate_path.is_dir() {
            continue;
        }

        if crate_path.join("src/lib.rs").exists() {
            require_crate_lib_doc(&crate_path, errors);
        }
        reject_long_rust_files(&crate_path, 400, errors);
    }
}

pub fn require_crate_lib_doc(crate_path: impl AsRef<Path>, errors: &mut Vec<String>) {
    let lib = crate_path.as_ref().join("src/lib.rs");
    let Some(source) = read_file(&lib, errors) else {
        return;
    };

    if crate_lib_doc_missing(&source) {
        errors.push(format!(
            "{} must start with a crate-level `//!` doc comment describing the crate responsibility",
            lib.display()
        ));
    }
}

pub fn reject_long_rust_files(root: impl AsRef<Path>, max_lines: usize, errors: &mut Vec<String>) {
    for file in rust_files(root) {
        let Some(source) = read_file(&file, errors) else {
            continue;
        };
        if let Some(line_count) = line_count_over_limit(&source, max_lines) {
            errors.push(format!(
                "{} has {line_count} lines; split Rust files to stay at or below {max_lines} lines",
                file.display()
            ));
        }
    }
}

fn crate_lib_doc_missing(source: &str) -> bool {
    !source.trim_start().starts_with("//!")
}

fn line_count_over_limit(source: &str, max_lines: usize) -> Option<usize> {
    let line_count = source.lines().count();
    (line_count > max_lines).then_some(line_count)
}

#[cfg(test)]
mod tests {
    use super::{crate_lib_doc_missing, line_count_over_limit};

    #[test]
    fn crate_lib_doc_detects_missing_and_present_doc() {
        assert!(crate_lib_doc_missing("use bevy::prelude::*;\n"));
        assert!(crate_lib_doc_missing(""));
        assert!(!crate_lib_doc_missing("//! Crate responsibility.\n"));
        assert!(!crate_lib_doc_missing("\n\n//! Doc after blank lines.\n"));
    }

    #[test]
    fn line_count_flags_only_files_over_limit() {
        let at_limit = "line\n".repeat(400);
        let over_limit = "line\n".repeat(401);

        assert_eq!(line_count_over_limit(&at_limit, 400), None);
        assert_eq!(line_count_over_limit(&over_limit, 400), Some(401));
    }
}
