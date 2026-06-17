use crate::rules::base::derives::reject_derived_types;
use crate::rules::base::source::reject_terms_in_rust_files;
use crate::rules::util::{parse_rust_file, rust_files};

pub struct ErrorRules<'a> {
    pub crate_path: &'a str,
}

pub fn check_error(rules: ErrorRules<'_>, errors: &mut Vec<String>) {
    reject_terms_in_rust_files(
        rules.crate_path,
        &["bevy"],
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
