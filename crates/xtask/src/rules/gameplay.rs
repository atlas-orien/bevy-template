use std::fs;
use std::path::{Path, PathBuf};

use super::CheckStatus;

const GAMEPLAY_CRATE: &str = "crates/gameplay";
const GAMEPLAY_PROTOCOL: &str = "AI_PROTOCOL/GAMEPLAY.md";
const GAMEPLAY_API: &str = "crates/gameplay/src/api";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(GAMEPLAY_CRATE, &mut errors);
    require_path(GAMEPLAY_PROTOCOL, &mut errors);
    require_path(GAMEPLAY_API, &mut errors);
    reject_dependencies(&mut errors);
    reject_data_definitions(&mut errors);
    reject_direct_input(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(GAMEPLAY_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    for dependency in ["ecs", "input", "physics", "render_2d", "render_3d"] {
        if source.contains(&format!("{dependency}.workspace = true")) {
            errors.push(format!(
                "{} depends on `{dependency}`; gameplay should not depend on that crate",
                manifest.display()
            ));
        }
    }
}

fn reject_data_definitions(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(GAMEPLAY_CRATE)) {
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
                            "{} derives `{forbidden}`; ECS data definitions belong in ecs/prefab/physics",
                            file.display()
                        ));
                    }
                }

                if derived.iter().any(|name| name == "Message")
                    && !file.starts_with(Path::new(GAMEPLAY_API))
                {
                    errors.push(format!(
                        "{} derives `Message`; gameplay messages must be part of the public api boundary",
                        file.display()
                    ));
                }
            }
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(GAMEPLAY_CRATE)) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput<", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; direct input must be converted before gameplay",
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
