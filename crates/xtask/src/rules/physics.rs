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
    for path in [
        "crates/physics/src/body/kind.rs",
        "crates/physics/src/body/control.rs",
        "crates/physics/src/config/settings.rs",
        "crates/physics/src/collider/shape.rs",
        "crates/physics/src/collider/control.rs",
        "crates/physics/src/collider/filter.rs",
        "crates/physics/src/layer/collision_layer.rs",
        "crates/physics/src/sensor/marker.rs",
        "crates/physics/src/material/surface.rs",
        "crates/physics/src/mass/properties.rs",
        "crates/physics/src/motion/velocity.rs",
        "crates/physics/src/force/linear.rs",
        "crates/physics/src/events/collision.rs",
        "crates/physics/src/backend/rapier/mod.rs",
        "crates/physics/src/backend/rapier/dim2/mod.rs",
        "crates/physics/src/backend/rapier/dim2/convert.rs",
        "crates/physics/src/backend/rapier/dim2/systems.rs",
        "crates/physics/src/backend/rapier/dim3/mod.rs",
        "crates/physics/src/backend/rapier/dim3/convert.rs",
        "crates/physics/src/backend/rapier/dim3/systems.rs",
    ] {
        require_path(path, &mut errors);
    }
    for obsolete in [
        "crates/physics/src/body.rs",
        "crates/physics/src/body/body.rs",
        "crates/physics/src/body/body_control.rs",
        "crates/physics/src/rigid_body",
        "crates/physics/src/rigid_body/rigid_body.rs",
        "crates/physics/src/config.rs",
        "crates/physics/src/config/config.rs",
        "crates/physics/src/collider.rs",
        "crates/physics/src/collider/collider.rs",
        "crates/physics/src/collider/collider_control.rs",
        "crates/physics/src/layer.rs",
        "crates/physics/src/layer/layer.rs",
        "crates/physics/src/sensor.rs",
        "crates/physics/src/sensor/sensor.rs",
        "crates/physics/src/material.rs",
        "crates/physics/src/material/material.rs",
        "crates/physics/src/mass.rs",
        "crates/physics/src/mass/mass.rs",
        "crates/physics/src/motion.rs",
        "crates/physics/src/force.rs",
        "crates/physics/src/force/force.rs",
        "crates/physics/src/events.rs",
        "crates/physics/src/backend/avian2d",
        "crates/physics/src/backend/avian2d.rs",
        "crates/physics/src/backend/rapier2d",
        "crates/physics/src/backend/rapier2d.rs",
    ] {
        reject_path(obsolete, &mut errors);
    }
    check_backend_dependencies(&mut errors);
    check_backend_imports(&mut errors);
    reject_physics_backend_features(&mut errors);
    reject_avian_dependency(&mut errors);
    check_public_api(&mut errors);
    reject_gameplay_hitbox_terms(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_physics_backend_features(errors: &mut Vec<String>) {
    let manifest = Path::new(PHYSICS_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    if source.contains("[features]") {
        errors.push(format!(
            "{} defines Cargo features; physics uses bevy_rapier directly and does not switch backends by feature",
            manifest.display()
        ));
    }
}

fn reject_avian_dependency(errors: &mut Vec<String>) {
    let manifest = Path::new(PHYSICS_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    for dependency in ["avian2d", "avian3d"] {
        if source.contains(dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; physics uses bevy_rapier as the only backend",
                manifest.display()
            ));
        }
    }
}

fn reject_gameplay_hitbox_terms(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(PHYSICS_CRATE).join("src").as_path()) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        for forbidden in ["Hitbox", "Hurtbox", "AttackRange", "SkillRange"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; gameplay hit/hurt/skill ranges do not belong in physics",
                    file.display()
                ));
            }
        }
    }
}

fn check_public_api(errors: &mut Vec<String>) {
    let public_api = Path::new(PHYSICS_CRATE).join("src/lib.rs");
    let Ok(source) = fs::read_to_string(&public_api) else {
        return;
    };

    for backend in BACKENDS {
        for forbidden in [
            format!("pub use {backend}"),
            format!("pub mod {backend}"),
            format!("pub type {backend}"),
        ] {
            if source.contains(&forbidden) {
                errors.push(format!(
                    "{} exposes `{backend}` in the public API; use project-level physics facade types instead",
                    public_api.display()
                ));
            }
        }
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

fn reject_path(path: impl AsRef<Path>, errors: &mut Vec<String>) {
    let path = path.as_ref();
    if path.exists() {
        errors.push(format!(
            "obsolete path should not exist: {}",
            path.display()
        ));
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
