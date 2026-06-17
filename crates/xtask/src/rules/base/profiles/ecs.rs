use std::path::Path;

use crate::rules::base::derives::{check_component_marker_names, reject_derived_types};
use crate::rules::base::paths::{reject_paths, require_mod_rs_under_src};
use crate::rules::util::{derived_names, files_named_below, parse_rust_file, rust_files};

pub struct EcsRules<'a> {
    pub obsolete_paths: &'a [&'a str],
    pub components_root: &'a str,
    pub resources_root: &'a str,
    pub events_root: &'a str,
    pub systems_root: &'a str,
}

pub fn check_ecs(rules: EcsRules<'_>, errors: &mut Vec<String>) {
    reject_paths(
        rules.obsolete_paths,
        errors,
        "obsolete ECS paths should move under crates/ecs/src/{components,resources,events,systems}",
    );
    check_ecs_components(rules.components_root, errors);
    check_ecs_resources(rules.resources_root, errors);
    check_ecs_events(rules.events_root, errors);
    check_ecs_systems(rules.systems_root, errors);
}

fn check_ecs_components(root: &str, errors: &mut Vec<String>) {
    let root = Path::new(root);
    require_mod_rs_under_src(root.to_string_lossy().as_ref(), errors);
    check_component_marker_names(root, errors);

    for readme in files_named_below(root, "README.md") {
        if readme != root.join("README.md") {
            errors.push(format!(
                "{} duplicates component docs; keep component documentation in crates/ecs/src/components/README.md",
                readme.display()
            ));
        }
    }

    reject_system_functions(
        root,
        errors,
        "ECS system functions belong in crates/ecs/src/systems, so move behavior out of components",
    );
}

fn check_ecs_resources(root: &str, errors: &mut Vec<String>) {
    let root = Path::new(root);
    require_mod_rs_under_src(root.to_string_lossy().as_ref(), errors);
    reject_derived_types(
        root,
        &["Component", "Bundle", "Event"],
        errors,
        "resources should define global ECS Resource data, so move this type to components/events as appropriate",
    );
}

fn check_ecs_events(root: &str, errors: &mut Vec<String>) {
    let root = Path::new(root);
    require_mod_rs_under_src(root.to_string_lossy().as_ref(), errors);
    reject_system_functions(
        root,
        errors,
        "event handling systems belong in crates/ecs/src/systems",
    );
    reject_derived_types(
        root,
        &["Component", "Bundle", "Resource", "Event"],
        errors,
        "events should define Message-based event data, so move other ECS data to its owning directory",
    );
    require_message_derive_for_event_files(root, errors);
}

fn check_ecs_systems(root: &str, errors: &mut Vec<String>) {
    let root = Path::new(root);
    require_mod_rs_under_src(root.to_string_lossy().as_ref(), errors);
    reject_derived_types(
        root,
        &["Component", "Bundle", "Resource", "Event"],
        errors,
        "ECS data definitions belong in components/resources/events, so keep systems focused on behavior",
    );
}

fn reject_system_functions(root: &Path, errors: &mut Vec<String>, hint: &str) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let syn::Item::Fn(function) = item {
                let name = function.sig.ident.to_string();
                if name.ends_with("_system") {
                    errors.push(format!("{} defines `{name}`; {hint}", file.display()));
                }
            }
        }
    }
}

fn require_message_derive_for_event_files(root: &Path, errors: &mut Vec<String>) {
    for file in rust_files(root) {
        if file
            .file_name()
            .is_some_and(|name| name == "mod.rs" || name == "plugin.rs")
        {
            continue;
        }

        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item)
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
