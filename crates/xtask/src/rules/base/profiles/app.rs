use std::path::Path;

use crate::rules::base::dependencies::reject_dependencies;
use crate::rules::base::source::reject_terms_in_rust_files;
use crate::rules::util::require_path;

pub struct AppRules<'a> {
    pub crate_path: &'a str,
    pub forbidden_dependencies: &'a [&'a str],
    pub forbidden_plugins: &'a [&'a str],
}

pub fn check_app(rules: AppRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "app is the runnable Bevy application crate and must remain present",
    );
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "app needs a stable crate entry point for application assembly",
    );
    reject_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "app should only depend on gameplay/external adapter crates, so move lower-level wiring behind gameplay or prefab",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        rules.forbidden_plugins,
        errors,
        "app should register gameplay and external adapter plugins only, so expose this through gameplay instead",
    );
}
