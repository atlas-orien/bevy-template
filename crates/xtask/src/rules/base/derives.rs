use std::path::Path;

use crate::rules::util::{derived_names, parse_rust_file, rust_files};

pub fn reject_derived_types(
    root: impl AsRef<Path>,
    forbidden_derives: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            let Some(derived) = derived_names(&item) else {
                continue;
            };

            for forbidden in forbidden_derives {
                if derived.iter().any(|name| name == forbidden) {
                    errors.push(format!("{} derives `{forbidden}`; {hint}", file.display()));
                }
            }
        }
    }
}

pub fn reject_derived_types_except_under(
    root: impl AsRef<Path>,
    allowed_root: impl AsRef<Path>,
    forbidden_derives: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let allowed_root = allowed_root.as_ref();

    for file in rust_files(root) {
        if file.starts_with(allowed_root) {
            continue;
        }

        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            let Some(derived) = derived_names(&item) else {
                continue;
            };

            for forbidden in forbidden_derives {
                if derived.iter().any(|name| name == forbidden) {
                    errors.push(format!("{} derives `{forbidden}`; {hint}", file.display()));
                }
            }
        }
    }
}

pub fn check_component_marker_names(root: impl AsRef<Path>, errors: &mut Vec<String>) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            let syn::Item::Struct(item) = item else {
                continue;
            };
            if !has_derive(&item.attrs, "Component") {
                continue;
            }

            let name = item.ident.to_string();
            let is_empty = struct_has_no_fields(&item.fields);
            let is_marker = name.ends_with("Marker");

            if is_empty && !is_marker {
                errors.push(format!(
                    "{} defines empty Component `{name}`; empty Component marker structs must end with `Marker`",
                    file.display()
                ));
            }

            if is_marker && !is_empty {
                errors.push(format!(
                    "{} defines Component `{name}` with fields; `*Marker` structs must be empty marker components",
                    file.display()
                ));
            }
        }
    }
}

fn has_derive(attrs: &[syn::Attribute], derive_name: &str) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("derive"))
        .any(|attr| {
            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(derive_name) {
                    found = true;
                }
                Ok(())
            });
            found
        })
}

fn struct_has_no_fields(fields: &syn::Fields) -> bool {
    match fields {
        syn::Fields::Unit => true,
        syn::Fields::Named(fields) => fields.named.is_empty(),
        syn::Fields::Unnamed(fields) => fields.unnamed.is_empty(),
    }
}
