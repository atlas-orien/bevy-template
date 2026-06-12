use std::path::Path;

use crate::rules::base::dependencies::reject_dependencies;
use crate::rules::base::paths::{require_mod_rs_under_src, require_paths};
use crate::rules::base::source::reject_direct_input_access;
use crate::rules::util::require_path;

pub struct SimpleCrateRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub anchor_hint: &'a str,
    pub protocol_hint: &'a str,
    pub lib_hint: &'a str,
    pub required_paths: &'a [&'a str],
    pub required_paths_hint: &'a str,
    pub forbidden_dependencies: &'a [&'a str],
    pub dependency_hint: &'a str,
    pub reject_direct_input: Option<&'a str>,
}

pub fn check_simple_crate(rules: SimpleCrateRules<'_>, errors: &mut Vec<String>) {
    require_path(rules.crate_path, errors, rules.anchor_hint);
    require_path(rules.protocol_path, errors, rules.protocol_hint);
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        rules.lib_hint,
    );
    require_paths(rules.required_paths, errors, rules.required_paths_hint);
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        rules.dependency_hint,
    );
    if let Some(hint) = rules.reject_direct_input {
        reject_direct_input_access(rules.crate_path, errors, hint);
    }
}
