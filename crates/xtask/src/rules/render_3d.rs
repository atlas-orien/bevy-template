use std::path::Path;

use super::CheckStatus;
use super::util::{
    manifest_has_workspace_dependency, read_file_if_exists, require_mod_rs_in_subdirs,
    require_path, rust_files,
};

const RENDER_3D_CRATE: &str = "crates/render_3d";
const RENDER_3D_PROTOCOL: &str = "AI_PROTOCOL/RENDER_3D.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        RENDER_3D_CRATE,
        &mut errors,
        "render_3d is the 3D presentation layer and must remain present",
    );
    require_path(
        RENDER_3D_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/RENDER_3D.md documents the 3D render boundary rules",
    );
    require_path(
        "crates/render_3d/src/lib.rs",
        &mut errors,
        "render_3d needs a crate root that exports presentation plugins/types",
    );
    for dir in [
        "crates/render_3d/src/camera",
        "crates/render_3d/src/scene",
        "crates/render_3d/src/ui",
    ] {
        require_path(
            dir,
            &mut errors,
            "3D presentation concepts should stay grouped by semantic directories",
        );
    }
    require_mod_rs_in_subdirs(Path::new(RENDER_3D_CRATE).join("src"), &mut errors);
    reject_dependencies(&mut errors);
    reject_direct_input(&mut errors);
    reject_world_rule_references(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(RENDER_3D_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in [
        "external_runtime",
        "audio",
        "intent",
        "prefab",
        "physics",
        "render_2d",
    ] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; render_3d should stay presentation-only, so communicate through ecs data/facades instead",
                manifest.display()
            ));
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(RENDER_3D_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; external sources belong in external_runtime or a future network crate",
                    file.display()
                ));
            }
        }
    }
}

fn reject_world_rule_references(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(RENDER_3D_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["set_movement_intent", "PhysicsBody", "PhysicsCollider"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; render_3d should not drive gameplay rules, so move the rule to gameplay/ecs/physics",
                    file.display()
                ));
            }
        }
    }
}
