use std::path::Path;

use crate::rules::base::dependencies::reject_workspace_dependencies;
use crate::rules::base::derives::{reject_derived_types, reject_derived_types_except_under};
use crate::rules::base::paths::{
    reject_files_under_dir_except, reject_subdirs_except, require_mod_rs_under_src, require_paths,
};
use crate::rules::base::source::{
    reject_direct_input_access, reject_terms_in_file, reject_terms_in_rust_files,
    require_file_contains_all_terms,
};
use crate::rules::util::require_path;

pub struct IntentRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub forbidden_dependencies: &'a [&'a str],
    pub world_mutation_terms: &'a [&'a str],
}

pub fn check_intent(rules: IntentRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "intent is the semantic API for writing entity intent data",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/INTENT.md documents the intent boundary rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "intent needs a crate root that exports semantic intent APIs",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_workspace_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "intent should not depend on that crate, so write through prefab/gameplay-facing APIs instead",
    );
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
    reject_terms_in_rust_files(
        rules.crate_path,
        rules.world_mutation_terms,
        errors,
        "intent should not mutate world results directly, so express the desired action through intent data",
    );
}

pub struct NavigationRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
    pub required_dirs: &'a [&'a str],
    pub forbidden_dependencies: &'a [&'a str],
    pub render_terms: &'a [&'a str],
    pub forbidden_import_terms: &'a [&'a str],
}

pub fn check_navigation(rules: NavigationRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "navigation is the world navigation foundation and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/NAVIGATION.md documents navigation boundaries",
    );
    require_paths(
        rules.required_dirs,
        errors,
        "navigation must keep semantic module directories for agents, targets, paths, queries, and systems",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_workspace_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "navigation should stay a world navigation foundation",
    );
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
    pub protocol_path: &'a str,
    pub api_path: &'a str,
    pub required_dirs: &'a [&'a str],
    pub allowed_interaction_categories: &'a [&'a str],
    pub forbidden_dependencies: &'a [&'a str],
}

pub fn check_gameplay(rules: GameplayRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "gameplay is the state/schedule/API layer and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/GAMEPLAY.md documents the gameplay boundary rules",
    );
    require_paths(
        rules.required_dirs,
        errors,
        "gameplay API, lifecycle, and schedule boundaries should stay explicit directories",
    );
    require_mod_rs_under_src(rules.crate_path, errors);
    reject_workspace_dependencies(
        rules.crate_path,
        rules.forbidden_dependencies,
        errors,
        "gameplay should not depend on that crate, so reach lower-level behavior through prefab/intent APIs",
    );
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

    let interaction_dir = Path::new(rules.crate_path).join("src/interaction");
    reject_terms_in_file(
        interaction_dir.join("mod.rs"),
        &[
            "MessageReader",
            "InteractionEventMessage",
            "match ",
            "info!(",
        ],
        errors,
        "gameplay interaction mod.rs should only declare modules and re-export entry points, while concrete interaction logic belongs in files such as demo_menu.rs",
    );
    require_path(
        interaction_dir.join("ui"),
        errors,
        "gameplay interaction handlers should be grouped by source/domain, starting with interaction/ui for UI action handlers",
    );
    reject_files_under_dir_except(
        &interaction_dir,
        &["mod.rs"],
        errors,
        "put concrete gameplay interaction logic under a category directory such as interaction/ui",
    );
    reject_subdirs_except(
        &interaction_dir,
        rules.allowed_interaction_categories,
        errors,
        "update xtask rules before adding a new interaction domain",
    );

    let demo_menu = interaction_dir.join("ui/demo_menu.rs");
    require_path(
        &demo_menu,
        errors,
        "demo menu UI focus, keyboard activation, and click handling should stay together in gameplay/src/interaction/ui/demo_menu.rs",
    );
    require_file_contains_all_terms(
        demo_menu,
        &[
            "UiNavigationInputMessage",
            "handle_demo_ui_navigation_system",
            "DemoMenuFocused",
        ],
        errors,
        "the demo menu needs an explicit gameplay-side focus/navigation handler instead of scattering UI keyboard logic",
    );
}
