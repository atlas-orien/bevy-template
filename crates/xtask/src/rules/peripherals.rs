use std::path::Path;

use super::CheckStatus;
use super::util::{
    derived_names, manifest_has_workspace_dependency, parse_rust_file, read_file_if_exists,
    require_mod_rs_in_subdirs, require_path, rust_files,
};

const PERIPHERALS_CRATE: &str = "crates/peripherals";
const PERIPHERALS_PROTOCOL: &str = "AI_PROTOCOL/PERIPHERALS.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        PERIPHERALS_CRATE,
        &mut errors,
        "peripherals is the Bevy-App-internal local keyboard/mouse/gamepad adapter layer",
    );
    require_path(
        PERIPHERALS_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/PERIPHERALS.md documents the local peripherals boundary rules",
    );
    for path in [
        "crates/peripherals/src/keyboard",
        "crates/peripherals/src/mouse",
        "crates/peripherals/src/gamepad",
    ] {
        require_path(
            path,
            &mut errors,
            "local peripheral adapters should stay grouped by keyboard/mouse/gamepad directories",
        );
    }

    require_mod_rs_in_subdirs(Path::new(PERIPHERALS_CRATE).join("src"), &mut errors);
    reject_dependencies(&mut errors);
    reject_core_data_definitions(&mut errors);
    reject_world_mutation(&mut errors);
    reject_network_details(&mut errors);
    reject_interaction_module(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_interaction_module(errors: &mut Vec<String>) {
    let path = Path::new(PERIPHERALS_CRATE).join("src/ui");
    if path.exists() {
        errors.push(format!(
            "{} exists; Bevy interaction events belong in crates/interaction, not peripherals",
            path.display()
        ));
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(PERIPHERALS_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in [
        "ecs",
        "physics",
        "prefab",
        "render_2d",
        "render_3d",
        "external_runtime",
        "audio",
    ] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; peripherals should translate local Bevy input into semantic requests without owning lower-level world systems or external runtime",
                manifest.display()
            ));
        }
    }
}

fn reject_core_data_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(PERIPHERALS_CRATE)) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; peripherals should not define core ECS data, so put world data in ecs/prefab/gameplay boundaries",
                            file.display()
                        ));
                    }
                }
            }
        }
    }
}

fn reject_world_mutation(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(PERIPHERALS_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in [
            "Commands",
            "Query<(&mut Transform",
            "Query<&mut Transform",
            "Transform",
            "PhysicsBody",
            "PhysicsCollider",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; peripherals should translate local input into semantic requests instead of mutating world results",
                    file.display()
                ));
            }
        }
    }
}

fn reject_network_details(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(PERIPHERALS_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in [
            "protobuf",
            "prost",
            "socket",
            "TcpStream",
            "UdpSocket",
            "WebSocket",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; peripherals should emit semantic actions and leave network transport to a dedicated outbound bridge",
                    file.display()
                ));
            }
        }
    }
}
