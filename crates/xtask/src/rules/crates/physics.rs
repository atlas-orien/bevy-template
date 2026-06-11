use std::fs;
use std::path::Path;

use crate::rules::CheckStatus;
use crate::rules::util::{
    read_file_if_exists, reject_dir_named_files, reject_path, require_mod_rs_in_subdirs,
    require_path, rust_files,
};

const PHYSICS_CRATE: &str = "crates/physics";
const PHYSICS_PROTOCOL: &str = "AI_PROTOCOL/PHYSICS.md";
const BACKENDS: [&str; 4] = ["avian2d", "avian3d", "bevy_rapier2d", "bevy_rapier3d"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        PHYSICS_CRATE,
        &mut errors,
        "physics is the only physics facade/backend crate and must remain present",
    );
    require_path(
        PHYSICS_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/PHYSICS.md documents the physics backend/facade rules",
    );
    require_path(
        "crates/physics/src/lib.rs",
        &mut errors,
        "physics needs a crate root that exposes project-level facade types",
    );
    require_path(
        "crates/physics/src/backend/rapier",
        &mut errors,
        "Rapier is the only current backend and should live under backend/rapier",
    );
    require_core_layout(&mut errors);
    require_mod_rs_in_subdirs(Path::new(PHYSICS_CRATE).join("src"), &mut errors);
    reject_dir_named_files(Path::new(PHYSICS_CRATE).join("src"), &mut errors);
    reject_obsolete_backend_layout(&mut errors);
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

fn require_core_layout(errors: &mut Vec<String>) {
    for path in [
        "crates/physics/src/body/kind.rs",
        "crates/physics/src/body/control.rs",
        "crates/physics/src/controller/character.rs",
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
        "crates/physics/src/joint/impulse.rs",
        "crates/physics/src/events/collision.rs",
        "crates/physics/src/events/contact_force.rs",
        "crates/physics/src/query/filter.rs",
        "crates/physics/src/query/point.rs",
        "crates/physics/src/query/raycast.rs",
        "crates/physics/src/query/shape.rs",
        "crates/physics/src/backend/rapier/mod.rs",
        "crates/physics/src/backend/rapier/dim2/mod.rs",
        "crates/physics/src/backend/rapier/dim2/convert.rs",
        "crates/physics/src/backend/rapier/dim2/events.rs",
        "crates/physics/src/backend/rapier/dim2/query.rs",
        "crates/physics/src/backend/rapier/dim2/systems.rs",
        "crates/physics/src/backend/rapier/dim3/mod.rs",
        "crates/physics/src/backend/rapier/dim3/convert.rs",
        "crates/physics/src/backend/rapier/dim3/events.rs",
        "crates/physics/src/backend/rapier/dim3/query.rs",
        "crates/physics/src/backend/rapier/dim3/systems.rs",
    ] {
        require_path(
            path,
            errors,
            "physics facade/backend capabilities should keep their documented semantic files",
        );
    }
}

fn reject_obsolete_backend_layout(errors: &mut Vec<String>) {
    for obsolete in [
        "crates/physics/src/body.rs",
        "crates/physics/src/body/body.rs",
        "crates/physics/src/body/body_control.rs",
        "crates/physics/src/collider.rs",
        "crates/physics/src/collider/collider.rs",
        "crates/physics/src/collider/collider_control.rs",
        "crates/physics/src/config.rs",
        "crates/physics/src/config/config.rs",
        "crates/physics/src/events.rs",
        "crates/physics/src/force.rs",
        "crates/physics/src/force/force.rs",
        "crates/physics/src/layer.rs",
        "crates/physics/src/layer/layer.rs",
        "crates/physics/src/mass.rs",
        "crates/physics/src/mass/mass.rs",
        "crates/physics/src/material.rs",
        "crates/physics/src/material/material.rs",
        "crates/physics/src/motion.rs",
        "crates/physics/src/rigid_body",
        "crates/physics/src/rigid_body/rigid_body.rs",
        "crates/physics/src/sensor.rs",
        "crates/physics/src/sensor/sensor.rs",
        "crates/physics/src/backend/avian2d",
        "crates/physics/src/backend/avian2d.rs",
        "crates/physics/src/backend/avian3d",
        "crates/physics/src/backend/avian3d.rs",
        "crates/physics/src/backend/rapier2d",
        "crates/physics/src/backend/rapier2d.rs",
        "crates/physics/src/backend/rapier3d",
        "crates/physics/src/backend/rapier3d.rs",
    ] {
        reject_path(
            obsolete,
            errors,
            "backend implementations should live under crates/physics/src/backend/rapier/{dim2,dim3}",
        );
    }
}

fn reject_physics_backend_features(errors: &mut Vec<String>) {
    let manifest = Path::new(PHYSICS_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    if source.contains("[features]") {
        errors.push(format!(
            "{} defines Cargo features; physics uses bevy_rapier directly and does not switch backends by feature, so remove backend feature gates",
            manifest.display()
        ));
    }
}

fn reject_avian_dependency(errors: &mut Vec<String>) {
    let manifest = Path::new(PHYSICS_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in ["avian2d", "avian3d"] {
        if source.contains(dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; physics uses bevy_rapier as the only backend, so remove the Avian dependency",
                manifest.display()
            ));
        }
    }
}

fn reject_gameplay_hitbox_terms(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(PHYSICS_CRATE).join("src")) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["Hitbox", "Hurtbox", "AttackRange", "SkillRange"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; gameplay hit/hurt/skill ranges do not belong in physics, so move the concept to gameplay/ecs/prefab",
                    file.display()
                ));
            }
        }
    }
}

fn check_public_api(errors: &mut Vec<String>) {
    let public_api = Path::new(PHYSICS_CRATE).join("src/lib.rs");
    let Some(source) = read_file_if_exists(&public_api) else {
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
        let Some(source) = read_file_if_exists(&manifest) else {
            continue;
        };

        for backend in BACKENDS {
            if source.contains(backend) {
                errors.push(format!(
                    "{} depends on `{backend}`; physics backends must be isolated in crates/physics, so depend on the physics facade instead",
                    manifest.display()
                ));
            }
        }
    }
}

fn check_backend_imports(errors: &mut Vec<String>) {
    for file in rust_files("crates") {
        if file.starts_with(PHYSICS_CRATE) {
            continue;
        }

        let Some(source) = read_file_if_exists(&file) else {
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
