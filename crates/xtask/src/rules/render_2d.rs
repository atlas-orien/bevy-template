use std::fs;
use std::path::{Path, PathBuf};

use super::CheckStatus;

const RENDER_2D_CRATE: &str = "crates/render_2d";
const RENDER_2D_PROTOCOL: &str = "AI_PROTOCOL/RENDER_2D.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(RENDER_2D_CRATE, &mut errors);
    require_path(RENDER_2D_PROTOCOL, &mut errors);
    require_path("crates/render_2d/src/camera", &mut errors);
    require_path("crates/render_2d/src/characters", &mut errors);
    require_path("crates/render_2d/src/geometry", &mut errors);
    require_path("crates/render_2d/src/screens", &mut errors);
    require_path("crates/render_2d/src/ui", &mut errors);
    require_path("crates/render_2d/src/camera/main_camera.rs", &mut errors);
    require_path("crates/render_2d/src/camera/systems.rs", &mut errors);
    require_path("crates/render_2d/src/characters/character.rs", &mut errors);
    require_path("crates/render_2d/src/geometry/shape.rs", &mut errors);
    require_path("crates/render_2d/src/geometry/size.rs", &mut errors);
    require_path("crates/render_2d/src/geometry/anchor.rs", &mut errors);
    require_path("crates/render_2d/src/geometry/color.rs", &mut errors);
    require_path("crates/render_2d/src/screens/clear_color.rs", &mut errors);
    require_path("crates/render_2d/src/ui/theme.rs", &mut errors);
    require_path("crates/render_2d/src/ui/markers.rs", &mut errors);
    reject_dependencies(&mut errors);
    reject_direct_input(&mut errors);
    reject_world_rule_references(&mut errors);
    reject_ambiguous_files(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(RENDER_2D_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    for dependency in [
        "external_runtime",
        "intent",
        "prefab",
        "physics",
        "render_3d",
    ] {
        if source.contains(&format!("{dependency}.workspace = true")) {
            errors.push(format!(
                "{} depends on `{dependency}`; render_2d should stay presentation-only",
                manifest.display()
            ));
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(&Path::new(RENDER_2D_CRATE).join("src")) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; external sources belong in external_runtime",
                    file.display()
                ));
            }
        }
    }
}

fn reject_world_rule_references(errors: &mut Vec<String>) {
    for file in rust_files(&Path::new(RENDER_2D_CRATE).join("src")) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        for forbidden in [
            "set_movement_intent",
            "PhysicsBody",
            "PhysicsCollider",
            "Hitbox",
            "Hurtbox",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; render_2d should not drive gameplay rules",
                    file.display()
                ));
            }
        }
    }
}

fn reject_ambiguous_files(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(RENDER_2D_CRATE)) {
        let Some(file_name) = file.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if matches!(file_name, "common.rs" | "misc.rs" | "utils.rs") {
            errors.push(format!(
                "{} has an ambiguous name; render_2d files should be named by presentation role",
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
