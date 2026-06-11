use std::path::Path;

use crate::rules::util::{manifest_has_workspace_dependency, read_file_if_exists};

pub fn reject_workspace_dependencies(
    crate_path: &str,
    dependencies: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let manifest = Path::new(crate_path).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for dependency in dependencies {
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; {hint}",
                manifest.display()
            ));
        }
    }
}
