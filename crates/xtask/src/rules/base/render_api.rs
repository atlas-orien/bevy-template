use std::path::Path;

use crate::rules::util::{derived_names, parse_rust_file, rust_files};

pub fn reject_multi_public_render_items(
    root: impl AsRef<Path>,
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        if is_module_scaffold_file(&file) {
            continue;
        }

        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        let public_items = public_render_product_items(&parsed);
        if public_items.len() > 1 {
            errors.push(format!(
                "{} exposes multiple public render product items [{}]; {hint}",
                file.display(),
                public_items.join(", ")
            ));
        }
    }
}

fn is_module_scaffold_file(file: &Path) -> bool {
    file.file_name().is_some_and(|name| {
        matches!(
            name.to_str(),
            Some("lib.rs") | Some("mod.rs") | Some("plugin.rs")
        )
    })
}

fn public_render_product_items(parsed: &syn::File) -> Vec<String> {
    parsed
        .items
        .iter()
        .filter_map(public_render_product_name)
        .collect()
}

fn public_render_product_name(item: &syn::Item) -> Option<String> {
    let (name, vis) = match item {
        syn::Item::Struct(item) => (item.ident.to_string(), &item.vis),
        syn::Item::Enum(item) => (item.ident.to_string(), &item.vis),
        _ => return None,
    };

    if !is_public_api(vis) || name.ends_with("Plugin") {
        return None;
    }

    let derives = derived_names(item).unwrap_or_default();
    if derives.iter().any(|derive| derive == "Bundle") {
        return Some(name);
    }

    None
}

fn is_public_api(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

#[cfg(test)]
mod tests {
    use super::public_render_product_items;

    fn items(source: &str) -> Vec<String> {
        let parsed = syn::parse_file(source).expect("test source should parse");
        public_render_product_items(&parsed)
    }

    #[test]
    fn counts_only_fully_public_bundle_structs() {
        let source = r#"
#[derive(Bundle)]
pub struct DemoProduct;
#[derive(Bundle)]
pub(super) struct InternalMarker;
struct PrivateBundle;
pub(crate) enum CrateOnlyState {}
pub struct PublicState;
"#;

        assert_eq!(items(source), vec!["DemoProduct"]);
    }

    #[test]
    fn reports_multiple_public_render_items() {
        let source = r#"
#[derive(Bundle)]
pub struct DemoProduct;
#[derive(Bundle)]
pub struct DemoProductBundle;
"#;

        assert_eq!(items(source), vec!["DemoProduct", "DemoProductBundle"]);
    }
}
