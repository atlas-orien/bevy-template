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
