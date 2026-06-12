use std::path::Path;

use crate::rules::base::dependencies::{reject_dependencies, require_workspace_dependency};
use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::paths::{reject_paths, require_mod_rs_under_src, require_paths};
use crate::rules::base::source::{
    reject_bevy_world_access, reject_direct_input_access, reject_network_transport_terms,
    reject_terms_in_file, reject_terms_in_rust_files, reject_type_paths_in_rust_files,
    reject_world_mutation_terms, require_file_contains_all_terms,
};
use crate::rules::util::require_path;

pub struct InteractionRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub forbidden_dependencies: &'a [&'a str],
    pub world_mutation_terms: &'a [&'a str],
    pub required_navigation_terms: &'a [&'a str],
}

pub fn check_interaction(rules: InteractionRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "interaction is the Bevy interaction event bridge layer",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/INTERACTION.md documents the interaction boundary rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/message.rs"),
        errors,
        "interaction semantic messages such as UI press and UI navigation input should stay in the interaction message boundary",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "interaction should translate Bevy interaction state into semantic messages without owning rendering, prefab, world, or external runtime concerns",
    );
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
    require_file_contains_all_terms(
        Path::new(rules.crate_path).join("src/message.rs"),
        rules.required_navigation_terms,
        errors,
        "keyboard/gamepad UI navigation should be converted into semantic interaction messages before gameplay consumes it",
    );
}

pub struct PeripheralsRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub required_dirs: &'a [&'a str],
    pub forbidden_dependencies: &'a [&'a str],
    pub required_dependency: &'a str,
    pub rejected_paths: &'a [&'a str],
}

pub fn check_peripherals(rules: PeripheralsRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "peripherals is the Bevy-App-internal local keyboard/mouse/gamepad adapter layer",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/PERIPHERALS.md documents the local peripherals boundary rules",
    );
    require_paths(
        rules.required_dirs,
        errors,
        "local peripheral adapters should stay grouped by keyboard/mouse/gamepad directories",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "peripherals should translate local Bevy input into semantic requests without owning lower-level world systems or external runtime",
    );
    require_workspace_dependency(
        rules.crate_path,
        rules.required_dependency,
        errors,
        "peripherals should publish semantic local input such as UI navigation through the shared interaction message boundary",
    );
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
    pub protocol_path: &'a str,
    pub required_dirs: &'a [&'a str],
    pub rejected_paths: &'a [&'a str],
    pub forbidden_dependencies: &'a [&'a str],
    pub forbidden_plugin_terms: &'a [&'a str],
    pub manager_user_files: &'a [&'a str],
}

pub fn check_external_runtime(rules: ExternalRuntimeRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "external_runtime is the Bevy-App-external runtime and manager layer",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/EXTERNAL_RUNTIME.md documents the external runtime boundary rules",
    );
    require_paths(
        rules.required_dirs,
        errors,
        "external runtime domains should stay grouped by input/runtime/manager/bridge directories",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_paths(
        rules.rejected_paths,
        errors,
        "local keyboard/mouse/gamepad adapters belong in crates/peripherals, Bevy interaction belongs in crates/interaction, and network is outside external_runtime v1",
    );
    reject_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "external_runtime should not depend on that crate, so enter gameplay through manager/API channels",
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
        reject_terms_in_file(
            file,
            &["GameplayEntityId"],
            errors,
            "manager user API must use RuntimeUserId/RuntimeObjectId and keep gameplay-facing ids internal",
        );
    }
}
