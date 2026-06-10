use std::path::Path;

use super::CheckStatus;
use super::util::{
    manifest_has_workspace_dependency, read_file_if_exists, require_path, rust_files,
};

const APP_CRATE: &str = "crates/app";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        APP_CRATE,
        &mut errors,
        "app is the runnable Bevy application crate and must remain present",
    );
    require_path(
        Path::new(APP_CRATE).join("src/lib.rs"),
        &mut errors,
        "app needs a stable crate entry point for application assembly",
    );
    reject_dependencies(&mut errors);
    reject_internal_plugins(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(APP_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in [
        "ecs",
        "audio",
        "intent",
        "physics",
        "prefab",
        "render_2d",
        "render_3d",
    ] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; app should only depend on gameplay/external adapter crates, so move lower-level wiring behind gameplay or prefab",
                manifest.display()
            ));
        }
    }
}

fn reject_internal_plugins(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(APP_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in [
            "EcsPlugin",
            "IntentPlugin",
            "PhysicsPlugin",
            "PrefabPlugin",
            "Render2dPlugin",
            "Render3dPlugin",
        ] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; app should register gameplay and external adapter plugins only, so expose this through gameplay instead",
                    file.display()
                ));
            }
        }
    }
}
