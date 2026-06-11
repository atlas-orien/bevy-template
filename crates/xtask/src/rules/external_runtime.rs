use std::path::Path;

use super::CheckStatus;
use super::util::{
    derived_names, manifest_has_workspace_dependency, parse_rust_file, read_file_if_exists,
    reject_path, require_mod_rs_in_subdirs, require_path, rust_files,
};

const EXTERNAL_RUNTIME_CRATE: &str = "crates/external_runtime";
const EXTERNAL_RUNTIME_PROTOCOL: &str = "AI_PROTOCOL/EXTERNAL_RUNTIME.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        EXTERNAL_RUNTIME_CRATE,
        &mut errors,
        "external_runtime is the Bevy-App-external runtime and manager layer",
    );
    require_path(
        EXTERNAL_RUNTIME_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/EXTERNAL_RUNTIME.md documents the external runtime boundary rules",
    );
    for path in [
        "crates/external_runtime/src/input/ai",
        "crates/external_runtime/src/runtime",
        "crates/external_runtime/src/manager",
        "crates/external_runtime/src/bridge",
    ] {
        require_path(
            path,
            &mut errors,
            "external runtime domains should stay grouped by input/runtime/manager/bridge directories",
        );
    }
    require_mod_rs_in_subdirs(Path::new(EXTERNAL_RUNTIME_CRATE).join("src"), &mut errors);
    reject_local_peripheral_domains(&mut errors);
    reject_dependencies(&mut errors);
    reject_network_module(&mut errors);
    reject_data_definitions(&mut errors);
    reject_plugin_definition(&mut errors);
    reject_bevy_input_access(&mut errors);
    reject_runtime_world_access(&mut errors);
    reject_world_mutation(&mut errors);
    reject_gameplay_id_user_api(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_gameplay_id_user_api(errors: &mut Vec<String>) {
    for path in [
        Path::new(EXTERNAL_RUNTIME_CRATE).join("src/manager/user.rs"),
        Path::new(EXTERNAL_RUNTIME_CRATE).join("src/manager/mod.rs"),
    ] {
        let Some(source) = read_file_if_exists(&path) else {
            continue;
        };

        if source.contains("GameplayEntityId") {
            errors.push(format!(
                "{} references `GameplayEntityId`; manager user API must use RuntimeUserId/RuntimeObjectId and keep gameplay-facing ids internal",
                path.display()
            ));
        }
    }
}

fn reject_local_peripheral_domains(errors: &mut Vec<String>) {
    for path in [
        "crates/external_runtime/src/input/local",
        "crates/external_runtime/src/input/device",
        "crates/external_runtime/src/local",
        "crates/external_runtime/src/device",
        "crates/external_runtime/src/peripherals",
    ] {
        reject_path(
            path,
            errors,
            "local keyboard/mouse/gamepad adapters belong in crates/peripherals, and Bevy interaction belongs in crates/interaction, not external_runtime",
        );
    }
}

fn reject_network_module(errors: &mut Vec<String>) {
    let network_path = Path::new(EXTERNAL_RUNTIME_CRATE).join("src/network");
    reject_path(
        network_path,
        errors,
        "network is a bidirectional communication layer and does not belong in external_runtime v1",
    );
}

fn reject_plugin_definition(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(EXTERNAL_RUNTIME_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["InputPlugin", "ExternalRuntimePlugin", "impl Plugin for"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; external_runtime must not be a Bevy plugin, so communicate through manager/bridge instead",
                    file.display()
                ));
            }
        }
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(EXTERNAL_RUNTIME_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in ["ecs", "audio", "physics", "render_2d", "render_3d"] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; external_runtime should not depend on that crate, so enter gameplay through manager/API channels",
                manifest.display()
            ));
        }
    }
}

fn reject_data_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(EXTERNAL_RUNTIME_CRATE)) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; external_runtime should not define core ECS data, so put ECS data in ecs/prefab/physics",
                            file.display()
                        ));
                    }
                }
            }
        }
    }
}

fn reject_runtime_world_access(errors: &mut Vec<String>) {
    for file in rust_files(
        Path::new(EXTERNAL_RUNTIME_CRATE)
            .join("src/runtime")
            .as_path(),
    ) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["World", "Commands", "Query<", "Res<", "ResMut<"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; external_runtime must communicate through manager/bridge, not Bevy World",
                    file.display()
                ));
            }
        }
    }
}

fn reject_bevy_input_access(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(EXTERNAL_RUNTIME_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; local keyboard/mouse/gamepad input belongs in crates/peripherals, not external_runtime",
                    file.display()
                ));
            }
        }
    }
}

fn reject_world_mutation(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(EXTERNAL_RUNTIME_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in [
            "Commands",
            "Query<",
            "Res<",
            "ResMut<",
            "Transform",
            "PhysicsBody",
            "PhysicsCollider",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; external_runtime should use manager request/update channels instead of mutating world results",
                    file.display()
                ));
            }
        }
    }
}
