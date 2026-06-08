use std::fs;
use std::path::{Path, PathBuf};

use super::CheckStatus;

const ECS_CRATE: &str = "crates/ecs";
const ECS_PROTOCOL: &str = "AI_PROTOCOL/ECS.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(ECS_CRATE, &mut errors);
    require_path(ECS_PROTOCOL, &mut errors);
    reject_path("crates/components", &mut errors);
    reject_path("crates/system", &mut errors);

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
    require_path(root, errors);
    require_path(root.join("README.md"), errors);

    for readme in readmes_below(root) {
        if readme != root.join("README.md") {
            errors.push(format!(
                "{} should not exist; component docs belong in crates/ecs/src/components/README.md",
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
                        "{} defines `{name}`; ECS system functions belong in crates/ecs/src/systems",
                        file.display()
                    ));
                }
            }
        }
    }
}

fn check_resources(errors: &mut Vec<String>) {
    let root = Path::new("crates/ecs/src/resources");
    require_path(root, errors);

    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; resources should define global ECS Resource data",
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
    require_path(root, errors);

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
                for forbidden in ["Component", "Bundle", "Resource"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; events should define ECS Event message data",
                            file.display()
                        ));
                    }
                }
            }
        }
    }
}

fn check_systems(errors: &mut Vec<String>) {
    let root = Path::new("crates/ecs/src/systems");
    require_path(root, errors);

    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; ECS data definitions belong in components/resources/events",
                            file.display()
                        ));
                    }
                }
            }
        }
    }
}

fn require_path(path: impl AsRef<Path>, errors: &mut Vec<String>) {
    let path = path.as_ref();
    if !path.exists() {
        errors.push(format!("required path is missing: {}", path.display()));
    }
}

fn reject_path(path: impl AsRef<Path>, errors: &mut Vec<String>) {
    let path = path.as_ref();
    if path.exists() {
        errors.push(format!(
            "obsolete path should not exist: {}",
            path.display()
        ));
    }
}

fn parse_rust_file(path: &Path, errors: &mut Vec<String>) -> Option<syn::File> {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            errors.push(format!("failed to read {}: {error}", path.display()));
            return None;
        }
    };

    match syn::parse_file(&source) {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            errors.push(format!("failed to parse {}: {error}", path.display()));
            None
        }
    }
}

fn rust_files(root: &Path) -> Vec<PathBuf> {
    files_with_extension(root, "rs")
}

fn readmes_below(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files(root, &mut files);
    files
        .into_iter()
        .filter(|path| path.file_name().is_some_and(|name| name == "README.md"))
        .collect()
}

fn files_with_extension(root: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files(root, &mut files);
    files
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == extension))
        .collect()
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path);
        }
    }
}

fn derived_names(item: &syn::Item) -> Option<Vec<String>> {
    let attrs = match item {
        syn::Item::Struct(item) => &item.attrs,
        syn::Item::Enum(item) => &item.attrs,
        _ => return None,
    };

    let mut names = Vec::new();

    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if let Some(ident) = meta.path.get_ident() {
                names.push(ident.to_string());
            }
            Ok(())
        });
    }

    Some(names)
}
