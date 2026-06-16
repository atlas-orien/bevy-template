use std::path::Path;

use crate::rules::util::{parse_rust_file, rust_files};

pub fn reject_free_functions_returning_any(
    root: impl AsRef<Path>,
    return_fragments: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        if contains_free_function_returning_any(&parsed, return_fragments) {
            errors.push(format!("{} {hint}", file.display()));
        }
    }
}

fn contains_free_function_returning_any(parsed: &syn::File, return_fragments: &[&str]) -> bool {
    parsed.items.iter().any(|item| {
        let syn::Item::Fn(function) = item else {
            return false;
        };

        function_return_matches(&function.sig.output, return_fragments)
    })
}

fn function_return_matches(output: &syn::ReturnType, return_fragments: &[&str]) -> bool {
    match output {
        syn::ReturnType::Default => false,
        syn::ReturnType::Type(_, ty) => return_fragments.iter().any(|fragment| match *ty.clone() {
            syn::Type::ImplTrait(impl_trait) if *fragment == "-> impl Bundle" => {
                impl_trait.bounds.iter().any(|bound| {
                    matches!(bound, syn::TypeParamBound::Trait(trait_bound)
                        if trait_bound.path.is_ident("Bundle"))
                })
            }
            syn::Type::Path(path) if *fragment == "-> Node" => path.path.is_ident("Node"),
            _ => false,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::contains_free_function_returning_any;

    fn parsed(source: &str) -> syn::File {
        syn::parse_file(source).expect("test source should parse")
    }

    #[test]
    fn detects_free_function_returning_impl_bundle() {
        let source = "fn menu_button() -> impl Bundle { todo!() }";

        assert!(contains_free_function_returning_any(
            &parsed(source),
            &["-> impl Bundle"]
        ));
    }

    #[test]
    fn detects_public_free_function_returning_node() {
        let source = "pub fn layout() -> Node { todo!() }";

        assert!(contains_free_function_returning_any(
            &parsed(source),
            &["-> Node"]
        ));
    }

    #[test]
    fn allows_methods_returning_impl_bundle() {
        let source = r#"
pub struct MenuButton;

impl MenuButton {
    pub fn into_bundle(self) -> impl Bundle { todo!() }
}
"#;

        assert!(!contains_free_function_returning_any(
            &parsed(source),
            &["-> impl Bundle"]
        ));
    }

    #[test]
    fn allows_named_bundle_structs() {
        let source = "pub struct MenuButtonBundle { pub node: Node }";

        assert!(!contains_free_function_returning_any(
            &parsed(source),
            &["-> impl Bundle", "-> Node"]
        ));
    }
}
