use std::path::Path;

use super::CheckStatus;
use super::util::{
    manifest_has_workspace_dependency, read_file_if_exists, require_mod_rs_in_subdirs,
    require_path, rust_files,
};

const NAVIGATION_CRATE: &str = "crates/navigation";
const NAVIGATION_PROTOCOL: &str = "AI_PROTOCOL/NAVIGATION.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        NAVIGATION_CRATE,
        &mut errors,
        "navigation is the world navigation foundation and must remain present",
    );
    require_path(
        NAVIGATION_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/NAVIGATION.md documents navigation boundaries",
    );
    for path in [
        "crates/navigation/src/agent",
        "crates/navigation/src/target",
        "crates/navigation/src/path",
        "crates/navigation/src/query",
        "crates/navigation/src/systems",
    ] {
        require_path(
            path,
            &mut errors,
            "navigation must keep semantic module directories for agents, targets, paths, queries, and systems",
        );
    }
    require_mod_rs_in_subdirs(Path::new(NAVIGATION_CRATE).join("src"), &mut errors);
    reject_forbidden_dependencies(&mut errors);
    reject_direct_input(&mut errors);
    reject_render_terms(&mut errors);
    reject_external_runtime_imports(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_forbidden_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(NAVIGATION_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in [
        "external_runtime",
        "intent",
        "gameplay",
        "prefab",
        "physics",
        "render_2d",
        "render_3d",
    ] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; navigation should stay a world navigation foundation",
                manifest.display()
            ));
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(NAVIGATION_CRATE) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput<", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; navigation targets must come from external_runtime/gameplay, not direct input",
                    file.display()
                ));
            }
        }
    }
}

fn reject_render_terms(errors: &mut Vec<String>) {
    for file in rust_files(NAVIGATION_CRATE) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in [
            "Sprite",
            "Camera2d",
            "Camera3d",
            "Text2d",
            "Node",
            "ImageNode",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; navigation visualization belongs in render crates",
                    file.display()
                ));
            }
        }
    }
}

fn reject_external_runtime_imports(errors: &mut Vec<String>) {
    for file in rust_files(NAVIGATION_CRATE) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["external_runtime::", "intent::", "gameplay::", "prefab::"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} imports `{forbidden}`; navigation must not depend on control/source layers",
                    file.display()
                ));
            }
        }
    }
}
