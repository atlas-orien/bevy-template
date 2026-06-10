use std::path::Path;

use super::CheckStatus;
use super::util::{
    manifest_has_workspace_dependency, read_file_if_exists, reject_path, require_mod_rs_in_subdirs,
    require_path, rust_files,
};

const RENDER_2D_CRATE: &str = "crates/render_2d";
const RENDER_2D_PROTOCOL: &str = "AI_PROTOCOL/RENDER_2D.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        RENDER_2D_CRATE,
        &mut errors,
        "render_2d is the 2D presentation layer and must remain present",
    );
    require_path(
        RENDER_2D_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/RENDER_2D.md documents the 2D render boundary rules",
    );
    require_path(
        "crates/render_2d/src/lib.rs",
        &mut errors,
        "render_2d needs a crate root that exports presentation plugins/types",
    );
    for dir in [
        "crates/render_2d/src/animation",
        "crates/render_2d/src/camera",
        "crates/render_2d/src/characters",
        "crates/render_2d/src/appearance",
        "crates/render_2d/src/geometry",
        "crates/render_2d/src/ordering",
        "crates/render_2d/src/screens",
        "crates/render_2d/src/sprite",
        "crates/render_2d/src/transform",
        "crates/render_2d/src/ui",
    ] {
        require_path(
            dir,
            &mut errors,
            "2D presentation concepts should stay grouped by semantic directories",
        );
    }
    require_mod_rs_in_subdirs(Path::new(RENDER_2D_CRATE).join("src"), &mut errors);
    reject_obsolete_geometry_mix(&mut errors);
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

fn reject_obsolete_geometry_mix(errors: &mut Vec<String>) {
    for obsolete in [
        "crates/render_2d/src/geometry/color.rs",
        "crates/render_2d/src/geometry/opacity.rs",
        "crates/render_2d/src/geometry/visibility.rs",
        "crates/render_2d/src/geometry/offset.rs",
        "crates/render_2d/src/geometry/scale.rs",
        "crates/render_2d/src/geometry/rotation.rs",
        "crates/render_2d/src/geometry/z_index.rs",
        "crates/render_2d/src/geometry/flip.rs",
    ] {
        reject_path(
            obsolete,
            errors,
            "geometry should only contain 2D shape/size/anchor concepts; move appearance/transform/ordering/sprite data to the matching directory",
        );
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(RENDER_2D_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in [
        "external_runtime",
        "audio",
        "intent",
        "prefab",
        "physics",
        "render_3d",
    ] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; render_2d should stay presentation-only, so communicate through ecs data/facades instead",
                manifest.display()
            ));
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(RENDER_2D_CRATE).join("src")) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; external sources belong in external_runtime, so render_2d should read presentation state only",
                    file.display()
                ));
            }
        }
    }
}

fn reject_world_rule_references(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(RENDER_2D_CRATE).join("src")) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in [
            "set_movement_intent",
            "PhysicsRigidBody",
            "PhysicsCollider",
            "Hitbox",
            "Hurtbox",
            "Combo",
            "SkillWindow",
            "AttackWindow",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; render_2d should not drive gameplay rules, so move the rule to gameplay/ecs/physics",
                    file.display()
                ));
            }
        }
    }
}

fn reject_ambiguous_files(errors: &mut Vec<String>) {
    for file in rust_files(RENDER_2D_CRATE) {
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
