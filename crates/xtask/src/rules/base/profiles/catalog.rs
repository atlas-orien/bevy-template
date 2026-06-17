use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::source::{reject_bevy_world_access, reject_direct_input_access};

pub struct CatalogRules<'a> {
    pub crate_path: &'a str,
}

pub fn check_catalog(rules: CatalogRules<'_>, errors: &mut Vec<String>) {
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
}
