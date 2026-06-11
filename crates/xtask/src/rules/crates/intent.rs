use std::path::Path;

use crate::rules::CheckStatus;
use crate::rules::util::{
    derived_names, manifest_has_workspace_dependency, parse_rust_file, read_file_if_exists,
    require_mod_rs_in_subdirs, require_path, rust_files,
};

const INTENT_CRATE: &str = "crates/intent";
const INTENT_PROTOCOL: &str = "AI_PROTOCOL/INTENT.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        INTENT_CRATE,
        &mut errors,
        "intent is the semantic API for writing entity intent data",
    );
    require_path(
        INTENT_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/INTENT.md documents the intent boundary rules",
    );
    require_path(
        "crates/intent/src/lib.rs",
        &mut errors,
        "intent needs a crate root that exports semantic intent APIs",
    );
    require_mod_rs_in_subdirs(Path::new(INTENT_CRATE).join("src"), &mut errors);
    reject_dependencies(&mut errors);
    reject_data_definitions(&mut errors);
    reject_direct_input(&mut errors);
    reject_world_mutation(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(INTENT_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in ["ecs", "audio", "physics", "render_2d", "render_3d"] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; intent should not depend on that crate, so write through prefab/gameplay-facing APIs instead",
                manifest.display()
            ));
        }
    }
}

fn reject_data_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(INTENT_CRATE)) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; intent should only write intent data, so define ECS data in ecs/prefab instead",
                            file.display()
                        ));
                    }
                }
            }
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(INTENT_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; input sources must be converted before intent, so move source handling to external_runtime",
                    file.display()
                ));
            }
        }
    }
}

fn reject_world_mutation(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(INTENT_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in [
            "Commands",
            "Transform",
            "PhysicsRigidBody",
            "PhysicsCollider2d",
            "PhysicsCollider3d",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; intent should not mutate world results directly, so express the desired action through intent data",
                    file.display()
                ));
            }
        }
    }
}
