use std::path::Path;

use crate::rules::CheckStatus;
use crate::rules::util::{
    derived_names, manifest_has_workspace_dependency, parse_rust_file, read_file_if_exists,
    require_mod_rs_in_subdirs, require_path, rust_files,
};

const GAMEPLAY_CRATE: &str = "crates/gameplay";
const GAMEPLAY_PROTOCOL: &str = "AI_PROTOCOL/GAMEPLAY.md";
const GAMEPLAY_API: &str = "crates/gameplay/src/api";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        GAMEPLAY_CRATE,
        &mut errors,
        "gameplay is the state/schedule/API layer and must remain present",
    );
    require_path(
        GAMEPLAY_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/GAMEPLAY.md documents the gameplay boundary rules",
    );
    for path in [
        GAMEPLAY_API,
        "crates/gameplay/src/lifecycle",
        "crates/gameplay/src/schedule",
        "crates/gameplay/src/interaction",
    ] {
        require_path(
            path,
            &mut errors,
            "gameplay API, lifecycle, and schedule boundaries should stay explicit directories",
        );
    }
    require_mod_rs_in_subdirs(Path::new(GAMEPLAY_CRATE).join("src"), &mut errors);
    reject_dependencies(&mut errors);
    reject_data_definitions(&mut errors);
    reject_direct_input(&mut errors);
    reject_manager_definitions(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_manager_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(GAMEPLAY_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["GameplayManager", "ExternalRuntimeManager"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; manager belongs in external_runtime, gameplay only owns request/update channels",
                    file.display()
                ));
            }
        }
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(GAMEPLAY_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in [
        "ecs",
        "audio",
        "external_runtime",
        "physics",
        "render_2d",
        "render_3d",
    ] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; gameplay should not depend on that crate, so reach lower-level behavior through prefab/intent APIs",
                manifest.display()
            ));
        }
    }
}

fn reject_data_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(GAMEPLAY_CRATE)) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; ECS data definitions belong in ecs/prefab/physics, so keep gameplay focused on flow and scheduling",
                            file.display()
                        ));
                    }
                }

                if derived.iter().any(|name| name == "Message")
                    && !file.starts_with(Path::new(GAMEPLAY_API))
                {
                    errors.push(format!(
                        "{} derives `Message`; gameplay messages must be part of the public api boundary, so move the message under crates/gameplay/src/api",
                        file.display()
                    ));
                }
            }
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(GAMEPLAY_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput<", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; direct input must be converted before gameplay, so source handling belongs in external_runtime",
                    file.display()
                ));
            }
        }
    }
}
