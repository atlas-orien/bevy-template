use std::fs;
use std::path::{Path, PathBuf};

use super::CheckStatus;
use syn::visit::Visit;

const ERROR_CRATE: &str = "crates/error";
const ERROR_PROTOCOL: &str = "AI_PROTOCOL/ERROR.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(ERROR_CRATE, &mut errors);
    require_path(ERROR_PROTOCOL, &mut errors);
    reject_bevy_dependency(&mut errors);
    reject_bevy_runtime_types(&mut errors);
    check_manifests_depend_on_error(&mut errors);
    check_result_aliases(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_bevy_dependency(errors: &mut Vec<String>) {
    let manifest = Path::new(ERROR_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    if source.contains("bevy.workspace = true") {
        errors.push(format!(
            "{} depends on `bevy`; error should stay a pure error type crate",
            manifest.display()
        ));
    }
}

fn reject_bevy_runtime_types(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(ERROR_CRATE)) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };
        let Ok(parsed) = syn::parse_file(&source) else {
            continue;
        };

        if source.contains("use bevy") || source.contains("bevy::") {
            errors.push(format!(
                "{} imports `bevy`; error should stay a pure error type crate",
                file.display()
            ));
        }

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Resource", "Message", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; error should not define Bevy runtime or ECS types",
                            file.display()
                        ));
                    }
                }
            }

            if let syn::Item::Struct(item) = item {
                let name = item.ident.to_string();
                if name.ends_with("Plugin") {
                    errors.push(format!(
                        "{} defines `{name}`; error should not register Bevy plugins",
                        file.display()
                    ));
                }
            }
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

fn check_manifests_depend_on_error(errors: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir("crates") else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() || path == Path::new(ERROR_CRATE) {
            continue;
        }

        let manifest = path.join("Cargo.toml");
        let Ok(source) = fs::read_to_string(&manifest) else {
            continue;
        };

        if !source.contains("error.workspace = true") {
            errors.push(format!(
                "{} must depend on the shared error crate",
                manifest.display()
            ));
        }
    }
}

fn check_result_aliases(errors: &mut Vec<String>) {
    for file in rust_files(Path::new("crates")) {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };
        let Ok(parsed) = syn::parse_file(&source) else {
            continue;
        };

        for item in &parsed.items {
            let syn::Item::Type(alias) = item else {
                continue;
            };

            if alias.ident == "Result" && !file.starts_with(ERROR_CRATE) {
                errors.push(format!(
                    "{} defines a `Result` alias; use error::Result instead",
                    file.display()
                ));
            }
        }

        if !file.starts_with(ERROR_CRATE) && uses_std_or_core_result(&parsed) {
            errors.push(format!(
                "{} uses std/core Result directly; use error::Result instead",
                file.display()
            ));
        }
    }
}

fn uses_std_or_core_result(file: &syn::File) -> bool {
    let mut visitor = DirectResultUse { found: false };
    visitor.visit_file(file);
    visitor.found
}

struct DirectResultUse {
    found: bool,
}

impl<'ast> Visit<'ast> for DirectResultUse {
    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        let mut segments = node
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string());

        if matches!(
            (segments.next(), segments.next(), segments.next()),
            (Some(first), Some(second), Some(third))
                if (first == "std" || first == "core") && second == "result" && third == "Result"
        ) {
            self.found = true;
            return;
        }

        syn::visit::visit_type_path(self, node);
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
