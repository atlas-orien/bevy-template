use std::path::Path;

use super::CheckStatus;
use super::util::{
    manifest_has_workspace_dependency, read_file_if_exists, require_mod_rs_in_subdirs,
    require_path, rust_files,
};

const PREFAB_CRATE: &str = "crates/prefab";
const PREFAB_PROTOCOL: &str = "AI_PROTOCOL/PREFAB.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        PREFAB_CRATE,
        &mut errors,
        "prefab is the object template library and must remain present",
    );
    require_path(
        PREFAB_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/PREFAB.md documents the prefab boundary rules",
    );
    require_path(
        "crates/prefab/src/lib.rs",
        &mut errors,
        "prefab needs a crate root that exports spawn/template facades",
    );
    require_mod_rs_in_subdirs(Path::new(PREFAB_CRATE).join("src"), &mut errors);
    reject_forbidden_dependencies(&mut errors);
    reject_direct_input(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_forbidden_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(PREFAB_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in ["external_runtime", "intent", "gameplay"] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; prefab should stay an object template library, so keep timing/control decisions in gameplay or external_runtime",
                manifest.display()
            ));
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(PREFAB_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput<", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; external sources belong in external_runtime, so prefab only composes object data",
                    file.display()
                ));
            }
        }
    }
}
