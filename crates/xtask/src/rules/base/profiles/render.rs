use std::path::Path;

use crate::rules::base::dependencies::reject_workspace_dependencies;
use crate::rules::base::functions::reject_free_functions_returning_any;
use crate::rules::base::paths::{
    reject_file_names, reject_paths, require_mod_rs_under_src, require_paths,
};
use crate::rules::base::source::{
    reject_direct_input_access, reject_files_containing_all_terms, reject_terms_in_rust_files,
};
use crate::rules::util::require_path;

pub struct Render3dRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub content_dirs: &'a [&'a str],
    pub forbidden_dependencies: &'a [&'a str],
    pub world_rule_terms: &'a [&'a str],
}

pub fn check_render_3d(rules: Render3dRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "render_3d is the 3D presentation layer and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/RENDER_3D.md documents the 3D render boundary rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "render_3d needs a crate root that exports presentation plugins/types",
    );
    require_paths(
        rules.content_dirs,
        errors,
        "3D presentation concepts should stay grouped by semantic directories",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_workspace_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "render_3d should stay presentation-only, so communicate through ecs data/facades instead",
    );
    reject_direct_input_access(
        rules.crate_path,
        errors,
        "external sources belong in peripherals/external_runtime, so render_3d should read presentation state only",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        rules.world_rule_terms,
        errors,
        "render_3d should not drive gameplay rules, so move the rule to gameplay/ecs/physics",
    );
}

pub struct Render2dRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub content_dirs: &'a [&'a str],
    pub obsolete_paths: &'a [&'a str],
    pub forbidden_dependencies: &'a [&'a str],
    pub world_rule_terms: &'a [&'a str],
}

pub fn check_render_2d(rules: Render2dRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "render_2d is the 2D presentation layer and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/RENDER_2D.md documents the 2D render boundary rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "render_2d needs a crate root that exports presentation plugins/types",
    );
    require_paths(
        rules.content_dirs,
        errors,
        "render_2d is a user-editable 2D presentation content crate; keep the agreed content category directories",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_paths(
        rules.obsolete_paths,
        errors,
        "render_2d should not recreate Bevy facade directories; put concrete game presentation code in the content category directories",
    );
    reject_workspace_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "render_2d should stay presentation-only, so communicate through ecs data/facades instead",
    );
    reject_direct_input_access(
        Path::new(rules.crate_path).join("src"),
        errors,
        "external sources belong in peripherals/external_runtime, so render_2d should read presentation state only",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        rules.world_rule_terms,
        errors,
        "render_2d should not drive gameplay rules, so move the rule to gameplay/ecs/physics",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        &["UiCameraTarget"],
        errors,
        "render_2d should expose static camera components/bundles only, while runtime camera-to-UI binding belongs in gameplay spawn code",
    );
    reject_free_functions_returning_any(
        Path::new(rules.crate_path).join("src/ui"),
        &["-> impl Bundle", "-> Node"],
        errors,
        "exposes UI presentation as free functions returning `Node` or `impl Bundle`; render_2d/src/ui should define named Component/Bundle structs for reusable UI presentation",
    );
    reject_files_containing_all_terms(
        Path::new(rules.crate_path).join("src/ui"),
        &[": FullScreenUiNodeBundle", ": Node"],
        errors,
        "combines `FullScreenUiNodeBundle` with another `Node`; Bevy entities may only receive one Node component, so UI root/layout bundles must expose a single composed Node",
    );
    reject_paths(
        &[
            "crates/render_2d/src/ui/camera.rs",
            "crates/render_2d/src/ui/menu.rs",
        ],
        errors,
        "UI camera/menu demo files must use explicit architecture locations and demo names",
    );
    reject_file_names(
        rules.crate_path,
        &["common.rs", "misc.rs", "utils.rs"],
        errors,
        "render_2d files should be named by presentation role",
    );
}
