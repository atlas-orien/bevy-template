use std::path::Path;

use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::paths::{reject_paths, require_mod_rs_under_src};
use crate::rules::base::source::{
    reject_bevy_world_access, reject_direct_input_access, reject_network_transport_terms,
    reject_terms_in_rust_files, reject_type_paths_in_rust_files, reject_world_mutation_terms,
};

pub struct InteractionRules<'a> {
    pub crate_path: &'a str,
    pub world_mutation_terms: &'a [&'a str],
}

pub fn check_interaction(rules: InteractionRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_type_paths_in_rust_files(
        rules.crate_path,
        rules.world_mutation_terms,
        errors,
        "interaction should emit interaction messages instead of mutating world results",
    );
    reject_network_transport_terms(
        rules.crate_path,
        errors,
        "interaction should emit semantic interaction messages and leave network transport to a dedicated outbound bridge",
    );
}

pub struct PeripheralsRules<'a> {
    pub crate_path: &'a str,
    pub rejected_paths: &'a [&'a str],
}

pub fn check_peripherals(rules: PeripheralsRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_derived_types(
        rules.crate_path,
        &["Component", "Bundle", "Resource", "Event"],
        errors,
        "peripherals should not define core ECS data, so put world data in ecs/prefab/gameplay boundaries",
    );
    reject_world_mutation_terms(
        rules.crate_path,
        errors,
        "peripherals should translate local input into semantic requests instead of mutating world results",
    );
    reject_network_transport_terms(
        rules.crate_path,
        errors,
        "peripherals should emit semantic actions and leave network transport to a dedicated outbound bridge",
    );
    reject_paths(
        rules.rejected_paths,
        errors,
        "Bevy interaction events belong in crates/interaction, not peripherals",
    );
}

pub struct ExternalRuntimeRules<'a> {
    pub crate_path: &'a str,
    pub rejected_paths: &'a [&'a str],
    pub forbidden_plugin_terms: &'a [&'a str],
    pub manager_user_files: &'a [&'a str],
}

pub fn check_external_runtime(rules: ExternalRuntimeRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_paths(
        rules.rejected_paths,
        errors,
        "local keyboard/mouse/gamepad adapters belong in crates/peripherals, Bevy interaction belongs in crates/interaction, and network is outside external_runtime v1",
    );
    reject_derived_types(
        rules.crate_path,
        &["Component", "Bundle", "Resource", "Event"],
        errors,
        "external_runtime should not define core ECS data, so put ECS data in ecs/prefab/physics",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        rules.forbidden_plugin_terms,
        errors,
        "external_runtime must not be a Bevy plugin, so communicate through manager/bridge instead",
    );
    reject_direct_input_access(
        rules.crate_path,
        errors,
        "local keyboard/mouse/gamepad input belongs in crates/peripherals, not external_runtime",
    );
    reject_bevy_world_access(
        Path::new(rules.crate_path).join("src/runtime"),
        errors,
        "external_runtime must communicate through manager/bridge, not Bevy World",
    );
    reject_world_mutation_terms(
        rules.crate_path,
        errors,
        "external_runtime should use manager request/update channels instead of mutating world results",
    );
    for file in rules.manager_user_files {
        reject_type_paths_in_rust_files(
            file,
            &["GameplayEntityId"],
            errors,
            "manager user API must use RuntimeUserId/RuntimeObjectId and keep gameplay-facing ids internal",
        );
    }
}
