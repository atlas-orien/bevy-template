use crate::rules::base::derives::check_component_marker_names;
use crate::rules::base::source::reject_direct_input_access;

pub struct SimpleCrateRules<'a> {
    pub crate_path: &'a str,
    pub reject_direct_input: Option<&'a str>,
}

pub fn check_simple_crate(rules: SimpleCrateRules<'_>, errors: &mut Vec<String>) {
    check_component_marker_names(rules.crate_path, errors);

    if let Some(hint) = rules.reject_direct_input {
        reject_direct_input_access(rules.crate_path, errors, hint);
    }
}
