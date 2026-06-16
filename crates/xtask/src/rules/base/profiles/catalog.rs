use std::path::Path;

use crate::rules::base::dependencies::reject_dependencies;
use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::paths::{require_mod_rs_under_src, require_paths};
use crate::rules::base::source::{
    reject_bevy_world_access, reject_direct_input_access, reject_terms_in_rust_files,
    require_file_contains_all_terms,
};
use crate::rules::util::require_path;

pub struct CatalogRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub required_paths: &'a [&'a str],
    pub forbidden_dependencies: &'a [&'a str],
    pub managed_asset_paths: &'a [&'a str],
    pub managed_asset_path_consumers: &'a [&'a str],
}

pub fn check_catalog(rules: CatalogRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "catalog is the default prefab resource binding crate and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/CATALOG.md documents default resource catalog rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "catalog needs a crate root that exports default prefab resource bindings",
    );
    require_paths(
        rules.required_paths,
        errors,
        "catalog should keep demo resource bindings in explicit semantic files",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "catalog binds resources to prefab constructors and must not depend on gameplay/runtime layers",
    );
    reject_derived_types(
        rules.crate_path,
        &["Component", "Bundle", "Resource", "Event", "Message"],
        errors,
        "catalog should not define ECS data; put components/bundles in ecs/render/prefab",
    );
    reject_bevy_world_access(
        rules.crate_path,
        errors,
        "catalog should not access Bevy World or spawn entities; gameplay/dev_preview decide timing",
    );
    reject_direct_input_access(rules.crate_path, errors, "catalog should not read input");
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/demo.rs"),
        &["AssetServer", "DemoPlayerPrefab", "DemoGroundPrefab"],
        errors,
        "catalog demo entries should bind default assets to prefab constructors",
    );

    for consumer in rules.managed_asset_path_consumers {
        reject_terms_in_rust_files(
            consumer,
            rules.managed_asset_paths,
            errors,
            "gameplay/dev_preview should use catalog entries instead of hand-written managed demo asset paths",
        );
    }
}
