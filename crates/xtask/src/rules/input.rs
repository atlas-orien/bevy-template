use std::fs;
use std::path::{Path, PathBuf};

use super::CheckStatus;

const INPUT_CRATE: &str = "crates/input";
const INPUT_PROTOCOL: &str = "AI_PROTOCOL/INPUT.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(INPUT_CRATE, &mut errors);
    require_path(INPUT_PROTOCOL, &mut errors);
    require_path("crates/input/src/local.rs", &mut errors);
    reject_dependencies(&mut errors);
    reject_data_definitions(&mut errors);
    reject_world_mutation(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(INPUT_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    for dependency in ["ecs", "physics", "render_2d", "render_3d"] {
        if source.contains(&format!("{dependency}.workspace = true")) {
            errors.push(format!(
                "{} depends on `{dependency}`; input should not depend on that crate",
                manifest.display()
            ));
        }
    }
}

fn reject_data_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(INPUT_CRATE)) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };
        let Ok(parsed) = syn::parse_file(&source) else {
            continue;
        };

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Bundle", "Resource", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; input should not define core ECS data",
                            file.display()
                        ));
                    }
                }
            }
        }
    }
}

fn reject_world_mutation(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(INPUT_CRATE)) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        for forbidden in ["Commands", "Transform", "PhysicsBody", "PhysicsCollider"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; input should convert sources into intent or gameplay api requests",
                    file.display()
                ));
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

fn rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rust_files(root, &mut files);
    files
}

fn collect_rust_files(root: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
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
