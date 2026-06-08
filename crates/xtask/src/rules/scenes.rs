use std::fs;
use std::path::{Path, PathBuf};

use super::CheckStatus;

const SCENES_CRATE: &str = "crates/scenes";
const SCENES_PROTOCOL: &str = "AI_PROTOCOL/SCENES.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(SCENES_CRATE, &mut errors);
    require_path(SCENES_PROTOCOL, &mut errors);
    require_path("crates/scenes/src/main_menu", &mut errors);
    require_path("crates/scenes/src/level_01", &mut errors);
    require_path("crates/scenes/src/shared", &mut errors);
    reject_dependencies(&mut errors);
    reject_direct_input(&mut errors);
    reject_gameplay_system_functions(&mut errors);
    reject_intent_writes(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(SCENES_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    for dependency in [
        "simulation",
        "input",
        "intent",
        "physics",
        "render_2d",
        "render_3d",
    ] {
        if source.contains(&format!("{dependency}.workspace = true")) {
            errors.push(format!(
                "{} depends on `{dependency}`; scenes should assemble prefabs without owning that layer",
                manifest.display()
            ));
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(SCENES_CRATE)) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; input belongs in crates/input",
                    file.display()
                ));
            }
        }
    }
}

fn reject_gameplay_system_functions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(SCENES_CRATE)) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };
        let Ok(parsed) = syn::parse_file(&source) else {
            continue;
        };

        for item in parsed.items {
            if let syn::Item::Fn(function) = item {
                let name = function.sig.ident.to_string();
                if name.ends_with("_system") {
                    errors.push(format!(
                        "{} defines `{name}`; scenes should only define scene lifecycle systems",
                        file.display()
                    ));
                }
            }
        }
    }
}

fn reject_intent_writes(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(SCENES_CRATE)) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        if source.contains("set_movement_intent") {
            errors.push(format!(
                "{} writes intent; scenes should only assemble prefabs",
                file.display()
            ));
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
