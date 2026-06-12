use std::path::Path;

use crate::rules::util::{read_file_if_exists, rust_files};

pub fn reject_free_functions_returning_any(
    root: impl AsRef<Path>,
    return_fragments: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    for file in rust_files(root) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        if contains_free_function_returning_any(&source, return_fragments) {
            errors.push(format!("{} {hint}", file.display()));
        }
    }
}

fn contains_free_function_returning_any(source: &str, return_fragments: &[&str]) -> bool {
    source.lines().map(str::trim).any(|line| {
        (line.starts_with("pub fn ") || line.starts_with("fn "))
            && return_fragments
                .iter()
                .any(|fragment| line.contains(fragment))
    })
}

#[cfg(test)]
mod tests {
    use super::contains_free_function_returning_any;

    #[test]
    fn detects_free_function_returning_impl_bundle() {
        let source = "fn menu_button() -> impl Bundle { todo!() }";

        assert!(contains_free_function_returning_any(
            source,
            &["-> impl Bundle"]
        ));
    }

    #[test]
    fn detects_public_free_function_returning_node() {
        let source = "pub fn layout() -> Node { todo!() }";

        assert!(contains_free_function_returning_any(source, &["-> Node"]));
    }

    #[test]
    fn allows_named_bundle_structs() {
        let source = "pub struct MenuButtonBundle { pub node: Node }";

        assert!(!contains_free_function_returning_any(
            source,
            &["-> impl Bundle", "-> Node"]
        ));
    }
}
