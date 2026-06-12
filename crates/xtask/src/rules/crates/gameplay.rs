use std::path::Path;

use crate::rules::CheckStatus;
use crate::rules::util::{
    derived_names, manifest_has_workspace_dependency, parse_rust_file, read_file_if_exists,
    require_mod_rs_in_subdirs, require_path, rust_files,
};

const GAMEPLAY_CRATE: &str = "crates/gameplay";
const GAMEPLAY_PROTOCOL: &str = "AI_PROTOCOL/GAMEPLAY.md";
const GAMEPLAY_API: &str = "crates/gameplay/src/api";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        GAMEPLAY_CRATE,
        &mut errors,
        "gameplay is the state/schedule/API layer and must remain present",
    );
    require_path(
        GAMEPLAY_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/GAMEPLAY.md documents the gameplay boundary rules",
    );
    for path in [
        GAMEPLAY_API,
        "crates/gameplay/src/lifecycle",
        "crates/gameplay/src/schedule",
        "crates/gameplay/src/interaction",
    ] {
        require_path(
            path,
            &mut errors,
            "gameplay API, lifecycle, and schedule boundaries should stay explicit directories",
        );
    }
    require_mod_rs_in_subdirs(Path::new(GAMEPLAY_CRATE).join("src"), &mut errors);
    reject_dependencies(&mut errors);
    reject_data_definitions(&mut errors);
    reject_direct_input(&mut errors);
    reject_manager_definitions(&mut errors);
    reject_interaction_logic_in_mod_rs(&mut errors);
    require_interaction_category_dirs(&mut errors);
    reject_unknown_interaction_layout(&mut errors);
    require_demo_menu_interaction_handler(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn require_demo_menu_interaction_handler(errors: &mut Vec<String>) {
    let demo_menu = Path::new(GAMEPLAY_CRATE).join("src/interaction/ui/demo_menu.rs");
    require_path(
        &demo_menu,
        errors,
        "demo menu UI focus, keyboard activation, and click handling should stay together in gameplay/src/interaction/ui/demo_menu.rs",
    );

    let Some(source) = read_file_if_exists(&demo_menu) else {
        return;
    };

    for required in [
        "UiNavigationInputMessage",
        "handle_demo_ui_navigation_system",
        "DemoMenuFocused",
    ] {
        if !source.contains(required) {
            errors.push(format!(
                "{} does not contain `{required}`; the demo menu needs an explicit gameplay-side focus/navigation handler instead of scattering UI keyboard logic",
                demo_menu.display()
            ));
        }
    }
}

fn reject_unknown_interaction_layout(errors: &mut Vec<String>) {
    let interaction_dir = Path::new(GAMEPLAY_CRATE).join("src/interaction");
    let Ok(entries) = std::fs::read_dir(&interaction_dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if path.is_file() {
            if name != "mod.rs" {
                errors.push(format!(
                    "{} is a root interaction file; put concrete gameplay interaction logic under a category directory such as interaction/ui",
                    path.display()
                ));
            }
            continue;
        }

        if path.is_dir() && !ALLOWED_INTERACTION_CATEGORIES.contains(&name) {
            errors.push(format!(
                "{} is not an allowed gameplay interaction category; update xtask rules before adding a new interaction domain",
                path.display()
            ));
        }
    }
}

const ALLOWED_INTERACTION_CATEGORIES: &[&str] = &["ui"];

fn require_interaction_category_dirs(errors: &mut Vec<String>) {
    require_path(
        "crates/gameplay/src/interaction/ui",
        errors,
        "gameplay interaction handlers should be grouped by source/domain, starting with interaction/ui for UI action handlers",
    );
}

fn reject_interaction_logic_in_mod_rs(errors: &mut Vec<String>) {
    let mod_rs = Path::new(GAMEPLAY_CRATE).join("src/interaction/mod.rs");
    let Some(source) = read_file_if_exists(&mod_rs) else {
        return;
    };

    for forbidden in [
        "MessageReader",
        "InteractionEventMessage",
        "match ",
        "info!(",
    ] {
        if source.contains(forbidden) {
            errors.push(format!(
                "{} references `{forbidden}`; gameplay interaction mod.rs should only declare modules and re-export entry points, while concrete interaction logic belongs in files such as demo_menu.rs",
                mod_rs.display()
            ));
        }
    }
}

fn reject_manager_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(GAMEPLAY_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["GameplayManager", "ExternalRuntimeManager"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; manager belongs in external_runtime, gameplay only owns request/update channels",
                    file.display()
                ));
            }
        }
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(GAMEPLAY_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in ["ecs", "audio", "external_runtime", "physics", "render_3d"] {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; gameplay should not depend on that crate, so reach lower-level behavior through prefab/intent APIs",
                manifest.display()
            ));
        }
    }
}

fn reject_data_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(GAMEPLAY_CRATE)) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; ECS data definitions belong in ecs/prefab/physics, so keep gameplay focused on flow and scheduling",
                            file.display()
                        ));
                    }
                }

                if derived.iter().any(|name| name == "Message")
                    && !file.starts_with(Path::new(GAMEPLAY_API))
                {
                    errors.push(format!(
                        "{} derives `Message`; gameplay messages must be part of the public api boundary, so move the message under crates/gameplay/src/api",
                        file.display()
                    ));
                }
            }
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(GAMEPLAY_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput<", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; direct input must be converted before gameplay, so source handling belongs in external_runtime",
                    file.display()
                ));
            }
        }
    }
}
