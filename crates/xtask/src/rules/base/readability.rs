use std::path::Path;

use crate::rules::util::{files_named_below, parse_rust_file, read_file, rust_files};

pub fn check_workspace_readability(crates_root: impl AsRef<Path>, errors: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(crates_root.as_ref()) else {
        return;
    };

    for entry in entries.flatten() {
        let crate_path = entry.path();
        if !crate_path.is_dir() {
            continue;
        }

        if crate_path.join("src/lib.rs").exists() {
            require_crate_lib_doc(&crate_path, errors);
        }
        reject_long_rust_files(&crate_path, 400, errors);
        reject_non_declaration_mod_rs(&crate_path, errors);
    }
}

pub fn require_crate_lib_doc(crate_path: impl AsRef<Path>, errors: &mut Vec<String>) {
    let lib = crate_path.as_ref().join("src/lib.rs");
    let Some(source) = read_file(&lib, errors) else {
        return;
    };

    if !source.trim_start().starts_with("//!") {
        errors.push(format!(
            "{} must start with a crate-level `//!` doc comment describing the crate responsibility",
            lib.display()
        ));
    }
}

pub fn reject_long_rust_files(root: impl AsRef<Path>, max_lines: usize, errors: &mut Vec<String>) {
    for file in rust_files(root) {
        let Some(source) = read_file(&file, errors) else {
            continue;
        };
        let line_count = source.lines().count();
        if line_count > max_lines {
            errors.push(format!(
                "{} has {line_count} lines; split Rust files to stay at or below {max_lines} lines",
                file.display()
            ));
        }
    }
}

pub fn reject_non_declaration_mod_rs(root: impl AsRef<Path>, errors: &mut Vec<String>) {
    for file in files_named_below(root, "mod.rs") {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if !is_allowed_mod_item(&item) {
                errors.push(format!(
                    "{} contains `{}`; mod.rs should only declare modules and re-export names",
                    file.display(),
                    item_kind(&item)
                ));
            }
        }
    }
}

fn is_allowed_mod_item(item: &syn::Item) -> bool {
    matches!(item, syn::Item::Mod(_) | syn::Item::Use(_))
}

fn item_kind(item: &syn::Item) -> &'static str {
    match item {
        syn::Item::Const(_) => "const",
        syn::Item::Enum(_) => "enum",
        syn::Item::ExternCrate(_) => "extern crate",
        syn::Item::Fn(_) => "fn",
        syn::Item::ForeignMod(_) => "extern block",
        syn::Item::Impl(_) => "impl",
        syn::Item::Macro(_) => "macro",
        syn::Item::Mod(_) => "mod",
        syn::Item::Static(_) => "static",
        syn::Item::Struct(_) => "struct",
        syn::Item::Trait(_) => "trait",
        syn::Item::TraitAlias(_) => "trait alias",
        syn::Item::Type(_) => "type",
        syn::Item::Union(_) => "union",
        syn::Item::Use(_) => "use",
        _ => "item",
    }
}
