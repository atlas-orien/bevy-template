use std::path::Path;

use crate::rules::CheckStatus;
use crate::rules::util::{manifest_has_workspace_dependency, read_file_if_exists, require_path};

const HELPER_CRATE: &str = "crates/helper";
const HELPER_PROTOCOL: &str = "AI_PROTOCOL/HELPER.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        HELPER_CRATE,
        &mut errors,
        "helper is the shared infrastructure crate and must remain present",
    );
    require_path(
        HELPER_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/HELPER.md documents the helper boundary rules",
    );
    require_path(
        "crates/helper/src/lib.rs",
        &mut errors,
        "helper needs a crate root that exports reusable infrastructure",
    );
    reject_forbidden_dependencies(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_forbidden_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(HELPER_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in [
        "ecs",
        "audio",
        "external_runtime",
        "gameplay",
        "intent",
        "physics",
        "prefab",
        "render_2d",
        "render_3d",
    ] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; helper should stay shared infrastructure, so move game-specific logic to the owning crate",
                manifest.display()
            ));
        }
    }
}
