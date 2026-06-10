use std::fs;
use std::path::Path;

use super::CheckStatus;
use super::util::{derived_names, parse_rust_file, read_file_if_exists, require_path, rust_files};
use syn::visit::Visit;

const ERROR_CRATE: &str = "crates/error";
const ERROR_PROTOCOL: &str = "AI_PROTOCOL/ERROR.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        ERROR_CRATE,
        &mut errors,
        "error is the shared project error crate and must remain present",
    );
    require_path(
        ERROR_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/ERROR.md documents the shared error rules",
    );
    require_path(
        "crates/error/src/lib.rs",
        &mut errors,
        "error needs a crate root that exports GameError/Result",
    );
    reject_bevy_dependency(&mut errors);
    reject_bevy_gameplay_types(&mut errors);
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
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    if source.contains("bevy.workspace = true") {
        errors.push(format!(
            "{} depends on `bevy`; error should stay a pure error type crate, so keep Bevy integration in gameplay/runtime crates",
            manifest.display()
        ));
    }
}

fn reject_bevy_gameplay_types(errors: &mut Vec<String>) {
    for file in rust_files(ERROR_CRATE) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        if source.contains("use bevy") || source.contains("bevy::") {
            errors.push(format!(
                "{} imports `bevy`; error should stay a pure error type crate, so move Bevy-specific code to the owning crate",
                file.display()
            ));
        }

        for item in parsed.items {
            if let Some(derived) = derived_names(&item) {
                for forbidden in ["Component", "Resource", "Message", "Event"] {
                    if derived.iter().any(|name| name == forbidden) {
                        errors.push(format!(
                            "{} derives `{forbidden}`; error should not define Bevy gameplay or ECS types, so define those in their owning crate",
                            file.display()
                        ));
                    }
                }
            }

            if let syn::Item::Struct(item) = item {
                let name = item.ident.to_string();
                if name.ends_with("Plugin") {
                    errors.push(format!(
                        "{} defines `{name}`; error should not register Bevy plugins, so keep plugin setup outside the error crate",
                        file.display()
                    ));
                }
            }
        }
    }
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
        let Some(source) = read_file_if_exists(&manifest) else {
            continue;
        };

        if !source.contains("error.workspace = true") {
            errors.push(format!(
                "{} must depend on the shared error crate; add error.workspace = true and use error::Result",
                manifest.display()
            ));
        }
    }
}

fn check_result_aliases(errors: &mut Vec<String>) {
    for file in rust_files("crates") {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in &parsed.items {
            let syn::Item::Type(alias) = item else {
                continue;
            };

            if alias.ident == "Result" && !file.starts_with(ERROR_CRATE) {
                errors.push(format!(
                    "{} defines a `Result` alias; use error::Result or pub use error::Result instead",
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
