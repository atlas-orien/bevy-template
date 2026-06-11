use std::path::Path;

use crate::rules::CheckStatus;
use crate::rules::util::{
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
    require_content_dirs(&mut errors);
    require_mod_rs_in_subdirs(Path::new(RENDER_2D_CRATE).join("src"), &mut errors);
    reject_obsolete_facade_layout(&mut errors);
    reject_dependencies(&mut errors);
    reject_direct_input(&mut errors);
    reject_world_rule_references(&mut errors);
    reject_runtime_camera_targets(&mut errors);
    reject_ui_free_bundle_functions(&mut errors);
    reject_ambiguous_files(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_ui_free_bundle_functions(errors: &mut Vec<String>) {
    let ui_dir = Path::new(RENDER_2D_CRATE).join("src/ui");

    for file in rust_files(ui_dir) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for signature in ["pub fn ", "fn "] {
            if source.lines().map(str::trim).any(|line| {
                line.starts_with(signature)
                    && (line.contains("-> impl Bundle") || line.contains("-> Node"))
            }) {
                errors.push(format!(
                    "{} exposes UI presentation as free functions returning `Node` or `impl Bundle`; render_2d/src/ui should define named Component/Bundle structs for reusable UI presentation",
                    file.display()
                ));
                break;
            }
        }
    }
}

fn reject_runtime_camera_targets(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(RENDER_2D_CRATE).join("src")) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        if source.contains("UiCameraTarget") {
            errors.push(format!(
                "{} defines or references `UiCameraTarget`; render_2d should expose static camera components/bundles only, while runtime camera-to-UI binding belongs in gameplay spawn code",
                file.display()
            ));
        }
    }
}

fn require_content_dirs(errors: &mut Vec<String>) {
    for dir in [
        "crates/render_2d/src/animation",
        "crates/render_2d/src/animation/frame",
        "crates/render_2d/src/animation/skeletal",
        "crates/render_2d/src/atlases",
        "crates/render_2d/src/background",
        "crates/render_2d/src/camera",
        "crates/render_2d/src/characters",
        "crates/render_2d/src/debug",
        "crates/render_2d/src/effects",
        "crates/render_2d/src/environment",
        "crates/render_2d/src/items",
        "crates/render_2d/src/lighting",
        "crates/render_2d/src/materials",
        "crates/render_2d/src/mesh",
        "crates/render_2d/src/overlays",
        "crates/render_2d/src/particles",
        "crates/render_2d/src/pixel",
        "crates/render_2d/src/props",
        "crates/render_2d/src/screens",
        "crates/render_2d/src/text",
        "crates/render_2d/src/tilemap",
        "crates/render_2d/src/transitions",
        "crates/render_2d/src/ui",
    ] {
        require_path(
            dir,
            errors,
            "render_2d is a user-editable 2D presentation content crate; keep the agreed content category directories",
        );
    }
}

fn reject_obsolete_facade_layout(errors: &mut Vec<String>) {
    for obsolete in [
        "crates/render_2d/src/appearance",
        "crates/render_2d/src/appearance/color.rs",
        "crates/render_2d/src/appearance/opacity.rs",
        "crates/render_2d/src/appearance/visibility.rs",
        "crates/render_2d/src/geometry",
        "crates/render_2d/src/geometry/anchor.rs",
        "crates/render_2d/src/geometry/color.rs",
        "crates/render_2d/src/geometry/opacity.rs",
        "crates/render_2d/src/geometry/shape.rs",
        "crates/render_2d/src/geometry/size.rs",
        "crates/render_2d/src/geometry/visibility.rs",
        "crates/render_2d/src/geometry/offset.rs",
        "crates/render_2d/src/geometry/scale.rs",
        "crates/render_2d/src/geometry/rotation.rs",
        "crates/render_2d/src/geometry/z_index.rs",
        "crates/render_2d/src/geometry/flip.rs",
        "crates/render_2d/src/ordering",
        "crates/render_2d/src/ordering/z_index.rs",
        "crates/render_2d/src/sprite",
        "crates/render_2d/src/sprite/flip.rs",
        "crates/render_2d/src/transform",
        "crates/render_2d/src/transform/offset.rs",
        "crates/render_2d/src/transform/rotation.rs",
        "crates/render_2d/src/transform/scale.rs",
    ] {
        reject_path(
            obsolete,
            errors,
            "render_2d should not recreate Bevy facade directories; put concrete game presentation code in the content category directories",
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
