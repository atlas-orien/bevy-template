use std::path::Path;

use crate::rules::base::dependencies::reject_dependencies;
use crate::rules::base::paths::{reject_paths, require_crate_anchor, require_mod_rs_under_src};
use crate::rules::base::source::{
    reject_direct_input_access, reject_lines_containing_all_terms, reject_terms_in_rust_files,
};
use crate::rules::util::require_path;

pub struct PrefabRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub forbidden_dependencies: &'a [&'a str],
    pub ui_presentation_terms: &'a [&'a str],
    pub forbidden_asset_path_terms: &'a [&'a str],
}

pub fn check_prefab(rules: PrefabRules<'_>, errors: &mut Vec<String>) {
    require_crate_anchor(rules.crate_path, rules.protocol_path, errors);
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "prefab needs a crate root that exports spawn/template facades",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "prefab should stay an object template library, so keep timing/control decisions in gameplay or external_runtime",
    );
    reject_direct_input_access(
        rules.crate_path,
        errors,
        "external sources belong in peripherals/external_runtime, so prefab only composes object data",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        &[".spawn(commands)", ".spawn(&mut commands)"],
        errors,
        "calls another prefab's `spawn` from inside prefab code; compose multiple prefab instances from gameplay instead",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        &[".spawn(("],
        errors,
        "spawns a loose tuple directly; prefab must spawn a named bundle/product first, then attach children or narrow follow-up inserts",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        &[".insert(("],
        errors,
        "inserts a loose tuple directly; prefab follow-up inserts must also use named bundle/product types instead of ad hoc component tuples",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        &[".insert("],
        errors,
        "adds components after spawning a prefab root; prefab root objects must be expressed as one complete named bundle",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src/ui"),
        rules.ui_presentation_terms,
        errors,
        "UI visual presentation belongs in render_2d/src/products/ui, while prefab should only compose render bundles and semantic actions",
    );
    reject_paths(
        &[
            "crates/prefab/src/ui/camera.rs",
            "crates/prefab/src/ui/menu.rs",
        ],
        errors,
        "UI camera belongs in render_2d camera configuration and demo UI prefab files must use explicit demo names",
    );
    reject_lines_containing_all_terms(
        Path::new(rules.crate_path).join("src"),
        &["pub ", ": Entity"],
        errors,
        "exposes a public `Entity` field; prefab public API should use semantic handles instead of raw Bevy entity ids",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        &["UiCameraTarget"],
        errors,
        "UI prefab should not accept runtime camera handles, so gameplay must attach UiTargetCamera after spawning the UI prefab root",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        rules.forbidden_asset_path_terms,
        errors,
        "prefab must not hardcode concrete asset paths; catalog should bind resources and pass handles or semantic asset ids into prefab constructors",
    );
}
