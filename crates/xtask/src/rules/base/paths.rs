use std::path::Path;

use crate::rules::util::{reject_path, require_mod_rs_in_subdirs, require_path, rust_files};

pub fn require_crate_anchor(crate_path: &str, protocol_path: &str, errors: &mut Vec<String>) {
    require_path(
        crate_path,
        errors,
        "crate architecture anchor must remain present",
    );
    require_path(
        protocol_path,
        errors,
        "AI protocol for this crate must remain present",
    );
}

pub fn require_mod_rs_under_src(crate_path: &str, errors: &mut Vec<String>) {
    require_mod_rs_in_subdirs(Path::new(crate_path).join("src"), errors);
}

pub fn require_paths(paths: &[&str], errors: &mut Vec<String>, hint: &str) {
    for path in paths {
        require_path(path, errors, hint);
    }
}

pub fn reject_paths(paths: &[&str], errors: &mut Vec<String>, hint: &str) {
    for path in paths {
        reject_path(path, errors, hint);
    }
}

pub fn reject_file_names(
    root: impl AsRef<Path>,
    file_names: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(file_name) = file.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if file_names.contains(&file_name) {
            errors.push(format!(
                "{} has a forbidden file name; {hint}",
                file.display()
            ));
        }
    }
}

pub fn reject_files_under_dir_except(
    dir: impl AsRef<Path>,
    allowed_file_names: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let Ok(entries) = std::fs::read_dir(dir.as_ref()) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if !allowed_file_names.contains(&file_name) {
            errors.push(format!("{} is not an allowed file; {hint}", path.display()));
        }
    }
}

pub fn reject_subdirs_except(
    dir: impl AsRef<Path>,
    allowed_dir_names: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let Ok(entries) = std::fs::read_dir(dir.as_ref()) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let Some(dir_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if !allowed_dir_names.contains(&dir_name) {
            errors.push(format!(
                "{} is not an allowed directory; {hint}",
                path.display()
            ));
        }
    }
}
