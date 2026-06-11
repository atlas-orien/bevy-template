use std::path::Path;

use crate::rules::CheckStatus;
use crate::rules::util::{
    manifest_has_workspace_dependency, read_file_if_exists, require_mod_rs_in_subdirs,
    require_path, rust_files,
};

const INTERACTION_CRATE: &str = "crates/interaction";
const INTERACTION_PROTOCOL: &str = "AI_PROTOCOL/INTERACTION.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        INTERACTION_CRATE,
        &mut errors,
        "interaction is the Bevy interaction event bridge layer",
    );
    require_path(
        INTERACTION_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/INTERACTION.md documents the interaction boundary rules",
    );

    require_mod_rs_in_subdirs(Path::new(INTERACTION_CRATE).join("src"), &mut errors);
    reject_dependencies(&mut errors);
    reject_world_mutation(&mut errors);
    reject_network_details(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(INTERACTION_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in [
        "physics",
        "prefab",
        "render_2d",
        "render_3d",
        "external_runtime",
        "audio",
    ] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; interaction should translate Bevy interaction state into semantic messages without owning rendering, prefab, world, or external runtime concerns",
                manifest.display()
            ));
        }
    }
}

fn reject_world_mutation(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(INTERACTION_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in [
            "Commands",
            "Query<(&mut Transform",
            "Query<&mut Transform",
            "PhysicsBody",
            "PhysicsCollider",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; interaction should emit interaction messages instead of mutating world results",
                    file.display()
                ));
            }
        }
    }
}

fn reject_network_details(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(INTERACTION_CRATE)) {
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
                    "{} references `{forbidden}`; interaction should emit semantic interaction messages and leave network transport to a dedicated outbound bridge",
                    file.display()
                ));
            }
        }
    }
}
