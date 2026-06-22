use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::paths::reject_paths;
use crate::rules::base::source::{
    reject_bevy_world_access, reject_direct_input_access, reject_type_paths_in_rust_files,
};
use crate::rules::util::reject_path;

pub struct CatalogRules<'a> {
    pub crate_path: &'a str,
}

pub fn check_catalog(rules: CatalogRules<'_>, errors: &mut Vec<String>) {
    require_catalog_world_namespaces(rules.crate_path, errors);
    reject_obsolete_catalog_paths(rules.crate_path, errors);
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
    reject_type_paths_in_rust_files(
        rules.crate_path,
        &["render_3d::capabilities::animation"],
        errors,
        "catalog must not assemble 3D animation state/clip sets; keep animation playback and state mapping in render_3d products/capabilities",
    );
}

fn require_catalog_world_namespaces(crate_path: &str, errors: &mut Vec<String>) {
    for path in ["src/world_2d", "src/world_3d"] {
        let path = std::path::Path::new(crate_path).join(path);
        if !path.exists() {
            errors.push(format!(
                "{} is missing; catalog should keep world_2d/world_3d namespaces aligned with prefab",
                path.display()
            ));
        }
    }
}

fn reject_obsolete_catalog_paths(crate_path: &str, errors: &mut Vec<String>) {
    reject_paths(
        &[
            &format!("{crate_path}/src/characters"),
            &format!("{crate_path}/src/props"),
            &format!("{crate_path}/src/world"),
            &format!("{crate_path}/src/dev_preview"),
            &format!("{crate_path}/src/paths.rs"),
            &format!("{crate_path}/src/resources.rs"),
            &format!("{crate_path}/src/assets.rs"),
        ],
        errors,
        "catalog content should live under semantic world_2d/world_3d directories, and resource paths should stay with concrete catalog objects",
    );
    reject_path(
        format!("{crate_path}/src/dev_preview.rs"),
        errors,
        "dev preview composition belongs in crates/dev_preview, not catalog",
    );
}
