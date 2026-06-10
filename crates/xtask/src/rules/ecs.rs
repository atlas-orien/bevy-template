use std::path::Path;

use super::util::{
    derived_names, files_named_below, parse_rust_file, reject_path, require_mod_rs_in_subdirs,
    require_path, rust_files,
};
use super::CheckStatus;

const ECS_CRATE: &str = "crates/ecs";
const ECS_PROTOCOL: &str = "AI_PROTOCOL/ECS.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        ECS_CRATE,
        &mut errors,
        "ecs is the Bevy ECS data/system crate and must remain present",
    );
    require_path(
        ECS_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/ECS.md documents ECS data/system boundaries",
    );
    reject_path(
        "crates/components",
        &mut errors,
        "components belong under crates/ecs/src/components",
    );
    reject_path(
        "crates/system",
        &mut errors,
        "systems belong under crates/ecs/src/systems",
    );

    check_components(&mut errors);
    check_resources(&mut errors);
    check_events(&mut errors);
    check_systems(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn check_components(errors: &mut Vec<String>) {
    let root = Path::new("crates/ecs/src/components");
    require_path(
        root,
        errors,
        "ECS component data must live under crates/ecs/src/components",
    );
    require_path(
        root.join("README.md"),
        errors,
        "component documentation should stay centralized at components/README.md",
    );
    require_mod_rs_in_subdirs(root, errors);

    for readme in files_named_below(root, "README.md") {
        if readme != root.join("README.md") {
            errors.push(format!(
                "{} duplicates component docs; keep component documentation in crates/ecs/src/components/README.md",
                readme.display()
            ));
        }
    }

    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let syn::Item::Fn(function) = item {
                let name = function.sig.ident.to_string();
                if name.ends_with("_system") {
                    errors.push(format!(
                        "{} defines `{name}`; ECS system functions belong in crates/ecs/src/systems, so move behavior out of components",
                        file.display()
                    ));
                }
            }
        }
    }
}

fn check_resources(errors: &mut Vec<String>) {
    let root = Path::new("crates/ecs/src/resources");
    require_path(
        root,
        errors,
        "ECS resource data must live under crates/ecs/src/resources",
    );
    require_path(
        root.join("README.md"),
        errors,
        "resource documentation should stay centralized at resources/README.md",
    );
    require_mod_rs_in_subdirs(root, errors);

    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; resources should define global ECS Resource data, so move this type to components/events as appropriate",
                            file.display()
                        ));
                    }
                }
            }
        }
    }
}

fn check_events(errors: &mut Vec<String>) {
    let root = Path::new("crates/ecs/src/events");
    require_path(
        root,
        errors,
        "ECS event/message data must live under crates/ecs/src/events",
    );
    require_path(
        root.join("README.md"),
        errors,
        "event documentation should stay centralized at events/README.md",
    );
    require_mod_rs_in_subdirs(root, errors);

    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let syn::Item::Fn(function) = &item {
                let name = function.sig.ident.to_string();
                if name.ends_with("_system") {
                    errors.push(format!(
                        "{} defines `{name}`; event handling systems belong in crates/ecs/src/systems",
                        file.display()
                    ));
                }
            }

            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; events should define Message-based event data, so move other ECS data to its owning directory",
                            file.display()
                        ));
                    }
                }

                if file.file_name().is_some_and(|name| name != "mod.rs")
                    && !derived.iter().any(|name| name == "Message")
                {
                    errors.push(format!(
                        "{} defines an event type without deriving `Message`; current Bevy event channels use Message/add_message",
                        file.display()
                    ));
                }
            }
        }
    }
}

fn check_systems(errors: &mut Vec<String>) {
    let root = Path::new("crates/ecs/src/systems");
    require_path(
        root,
        errors,
        "ECS behavior systems must live under crates/ecs/src/systems",
    );
    require_mod_rs_in_subdirs(root, errors);

    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; ECS data definitions belong in components/resources/events, so keep systems focused on behavior",
                            file.display()
                        ));
                    }
                }
            }
        }
    }
}
