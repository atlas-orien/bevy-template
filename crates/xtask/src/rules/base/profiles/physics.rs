use std::path::Path;

use crate::rules::base::paths::{reject_paths, require_mod_rs_under_src};
use crate::rules::base::source::reject_terms_in_rust_files;
use crate::rules::util::reject_dir_named_files;

pub struct PhysicsRules<'a> {
    pub crate_path: &'a str,
    pub obsolete_paths: &'a [&'a str],
    pub gameplay_terms: &'a [&'a str],
}

pub fn check_physics(rules: PhysicsRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_dir_named_files(Path::new(rules.crate_path).join("src"), errors);
    reject_paths(
        rules.obsolete_paths,
        errors,
        "backend implementations should live under crates/physics/src/backend/rapier/{dim2,dim3}",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        rules.gameplay_terms,
        errors,
        "gameplay hit/hurt/skill ranges do not belong in physics, so move the concept to gameplay/ecs/prefab",
    );
}
