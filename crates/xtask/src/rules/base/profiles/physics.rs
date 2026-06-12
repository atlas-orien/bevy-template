use std::path::Path;

use crate::rules::base::dependencies::{
    reject_manifest_terms, reject_workspace_manifest_terms_except,
};
use crate::rules::base::paths::{reject_paths, require_mod_rs_under_src, require_paths};
use crate::rules::base::source::{
    reject_generated_terms_in_file, reject_generated_terms_in_rust_files_except,
    reject_terms_in_rust_files,
};
use crate::rules::util::{reject_dir_named_files, require_path};

pub struct PhysicsRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub backend_path: &'a str,
    pub required_paths: &'a [&'a str],
    pub obsolete_paths: &'a [&'a str],
    pub backends: &'a [&'a str],
    pub forbidden_manifest_terms: &'a [&'a str],
    pub gameplay_terms: &'a [&'a str],
}

pub fn check_physics(rules: PhysicsRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "physics is the only physics facade/backend crate and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/PHYSICS.md documents the physics backend/facade rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "physics needs a crate root that exposes project-level facade types",
    );
    require_path(
        rules.backend_path,
        errors,
        "Rapier is the only current backend and should live under backend/rapier",
    );
    require_paths(
        rules.required_paths,
        errors,
        "physics facade/backend capabilities should keep their documented semantic files",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_dir_named_files(Path::new(rules.crate_path).join("src"), errors);
    reject_paths(
        rules.obsolete_paths,
        errors,
        "backend implementations should live under crates/physics/src/backend/rapier/{dim2,dim3}",
    );
    reject_workspace_manifest_terms_except(
        "crates",
        rules.crate_path,
        rules.backends,
        errors,
        "physics backends must be isolated in crates/physics, so depend on the physics facade instead",
    );
    reject_generated_terms_in_rust_files_except(
        "crates",
        rules.crate_path,
        rules.backends,
        |backend| vec![format!("use {backend}"), format!("{backend}::")],
        errors,
        "use the physics crate facade instead",
    );
    reject_manifest_terms(
        rules.crate_path,
        rules.forbidden_manifest_terms,
        errors,
        "physics uses bevy_rapier as the only backend and does not switch backends by feature",
    );
    reject_generated_terms_in_file(
        Path::new(rules.crate_path).join("src/lib.rs"),
        rules.backends,
        |backend| {
            vec![
                format!("pub use {backend}"),
                format!("pub mod {backend}"),
                format!("pub type {backend}"),
            ]
        },
        errors,
        "use project-level physics facade types instead",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        rules.gameplay_terms,
        errors,
        "gameplay hit/hurt/skill ranges do not belong in physics, so move the concept to gameplay/ecs/prefab",
    );
}
