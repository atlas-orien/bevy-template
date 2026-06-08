use std::fs;
use std::path::{Path, PathBuf};

use super::CheckStatus;

const PHYSICS_CRATE: &str = "crates/physics";
const PHYSICS_PROTOCOL: &str = "AI_PROTOCOL/PHYSICS.md";
const BACKENDS: [&str; 4] = ["avian2d", "avian3d", "bevy_rapier2d", "bevy_rapier3d"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(PHYSICS_CRATE, &mut errors);
    require_path(PHYSICS_PROTOCOL, &mut errors);
    check_backend_dependencies(&mut errors);
    check_backend_imports(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn check_backend_dependencies(errors: &mut Vec<String>) {
    let crates_root = Path::new("crates");
    let Ok(entries) = fs::read_dir(crates_root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() || path == Path::new(PHYSICS_CRATE) {
            continue;
        }

        let manifest = path.join("Cargo.toml");
        let Ok(source) = fs::read_to_string(&manifest) else {
            continue;
        };

        for backend in BACKENDS {
            if source.contains(backend) {
                errors.push(format!(
                    "{} depends on `{backend}`; physics backends must be isolated in crates/physics",
                    manifest.display()
                ));
            }
        }
    }
}

fn check_backend_imports(errors: &mut Vec<String>) {
    for file in rust_files(Path::new("crates")) {
        if file.starts_with(PHYSICS_CRATE) {
            continue;
        }

        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        for backend in BACKENDS {
            let use_prefix = format!("use {backend}");
            let path_prefix = format!("{backend}::");
            if source.contains(&use_prefix) || source.contains(&path_prefix) {
                errors.push(format!(
                    "{} imports `{backend}`; use the physics crate facade instead",
                    file.display()
                ));
            }
        }
    }
}

fn require_path(path: impl AsRef<Path>, errors: &mut Vec<String>) {
    let path = path.as_ref();
    if !path.exists() {
        errors.push(format!("required path is missing: {}", path.display()));
    }
}

fn rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rust_files(root, &mut files);
    files
}

fn collect_rust_files(root: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}
