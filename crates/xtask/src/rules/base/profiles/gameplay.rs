use crate::rules::base::derives::{reject_derived_types, reject_derived_types_except_under};
use crate::rules::base::paths::require_mod_rs_under_src;
use crate::rules::base::source::{
    reject_direct_input_access, reject_terms_in_rust_files, reject_type_paths_in_rust_files,
};

pub struct IntentRules<'a> {
    pub crate_path: &'a str,
    pub world_mutation_terms: &'a [&'a str],
}

pub fn check_intent(rules: IntentRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_derived_types(
        rules.crate_path,
        &["Component", "Bundle", "Resource", "Event"],
        errors,
        "intent should only write intent data, so define ECS data in ecs/prefab instead",
    );
    reject_direct_input_access(
        rules.crate_path,
        errors,
        "input sources must be converted before intent, so move source handling to peripherals/external_runtime",
    );
    reject_type_paths_in_rust_files(
        rules.crate_path,
        rules.world_mutation_terms,
        errors,
        "intent should not mutate world results directly, so express the desired action through intent data",
    );
}

pub struct NavigationRules<'a> {
    pub crate_path: &'a str,
    pub render_terms: &'a [&'a str],
    pub forbidden_import_terms: &'a [&'a str],
}

pub fn check_navigation(rules: NavigationRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_direct_input_access(
        rules.crate_path,
        errors,
        "navigation targets must come from external_runtime/gameplay, not direct input",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        rules.render_terms,
        errors,
        "navigation visualization belongs in render crates",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        rules.forbidden_import_terms,
        errors,
        "navigation must not depend on control/source layers",
    );
}

pub struct GameplayRules<'a> {
    pub crate_path: &'a str,
    pub api_path: &'a str,
}

pub fn check_gameplay(rules: GameplayRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_derived_types(
        rules.crate_path,
        &["Component", "Bundle", "Resource", "Event"],
        errors,
        "ECS data definitions belong in ecs/prefab/physics, so keep gameplay focused on flow and scheduling",
    );
    reject_derived_types_except_under(
        rules.crate_path,
        rules.api_path,
        &["Message"],
        errors,
        "gameplay messages must be part of the public api boundary, so move the message under crates/gameplay/src/api",
    );
    reject_direct_input_access(
        rules.crate_path,
        errors,
        "direct input must be converted before gameplay, so source handling belongs in peripherals/external_runtime",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        &["GameplayManager", "ExternalRuntimeManager"],
        errors,
        "manager belongs in external_runtime, gameplay only owns request/update channels",
    );
}
