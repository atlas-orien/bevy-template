use std::fs;
use std::path::{Path, PathBuf};

use super::CheckStatus;

const APP_CRATE: &str = "crates/app";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(APP_CRATE, &mut errors);
    reject_dependencies(&mut errors);
    reject_internal_plugins(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(APP_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    for dependency in [
        "ecs",
        "audio",
        "intent",
        "physics",
        "prefab",
        "render_2d",
        "render_3d",
    ] {
        if source.contains(&format!("{dependency}.workspace = true")) {
            errors.push(format!(
                "{} depends on `{dependency}`; app should only depend on gameplay and external adapter crates",
                manifest.display()
            ));
        }
    }
}

fn reject_internal_plugins(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(APP_CRATE)) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        for forbidden in [
            "EcsPlugin",
            "IntentPlugin",
            "PhysicsPlugin",
            "PrefabPlugin",
            "Render2dPlugin",
            "Render3dPlugin",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; app should register gameplay and external adapter plugins only",
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
