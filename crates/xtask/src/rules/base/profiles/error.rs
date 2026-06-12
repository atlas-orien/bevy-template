use std::path::Path;

use syn::visit::Visit;

use crate::rules::base::dependencies::reject_manifest_terms;
use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::source::reject_terms_in_rust_files;
use crate::rules::util::{parse_rust_file, read_file_if_exists, require_path, rust_files};

pub struct ErrorRules<'a> {
    pub crate_path: &'a str,
    pub protocol_path: &'a str,
}

pub fn check_error(rules: ErrorRules<'_>, errors: &mut Vec<String>) {
    require_path(
        rules.crate_path,
        errors,
        "error is the shared project error crate and must remain present",
    );
    require_path(
        rules.protocol_path,
        errors,
        "AI_PROTOCOL/ERROR.md documents the shared error rules",
    );
    require_path(
        Path::new(rules.crate_path).join("src/lib.rs"),
        errors,
        "error needs a crate root that exports GameError/Result",
    );
    reject_manifest_terms(
        rules.crate_path,
        &["bevy.workspace = true"],
        errors,
        "error should stay a pure error type crate, so keep Bevy integration in gameplay/runtime crates",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        &["use bevy", "bevy::"],
        errors,
        "error should stay a pure error type crate, so move Bevy-specific code to the owning crate",
    );
    reject_derived_types(
        rules.crate_path,
        &["Component", "Resource", "Message", "Event"],
        errors,
        "error should not define Bevy gameplay or ECS types, so define those in their owning crate",
    );
    reject_plugin_structs(rules.crate_path, errors);
    require_workspace_error_dependency(rules.crate_path, errors);
    reject_result_aliases_outside_error(rules.crate_path, errors);
}

fn reject_plugin_structs(root: &str, errors: &mut Vec<String>) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
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

fn require_workspace_error_dependency(error_crate: &str, errors: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir("crates") else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() || path == Path::new(error_crate) {
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

fn reject_result_aliases_outside_error(error_crate: &str, errors: &mut Vec<String>) {
    for file in rust_files("crates") {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in &parsed.items {
            let syn::Item::Type(alias) = item else {
                continue;
            };

            if alias.ident == "Result" && !file.starts_with(error_crate) {
                errors.push(format!(
                    "{} defines a `Result` alias; use error::Result or pub use error::Result instead",
                    file.display()
                ));
            }
        }

        if !file.starts_with(error_crate) && uses_std_or_core_result(&parsed) {
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
