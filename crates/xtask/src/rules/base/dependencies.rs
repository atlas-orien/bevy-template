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

pub fn require_workspace_dependency(
    crate_path: &str,
    dependency: &str,
    errors: &mut Vec<String>,
    hint: &str,
) {
    let manifest = Path::new(crate_path).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    if !manifest_has_workspace_dependency(&source, dependency) {
        errors.push(format!(
            "{} does not depend on `{dependency}`; {hint}",
            manifest.display()
        ));
    }
}

pub fn reject_manifest_terms(
    crate_path: &str,
    terms: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let manifest = Path::new(crate_path).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
        return;
    };

    for term in terms {
        if source.contains(term) {
            errors.push(format!("{} contains `{term}`; {hint}", manifest.display()));
        }
    }
}

pub fn reject_workspace_manifest_terms_except(
    crates_root: impl AsRef<Path>,
    except_crate_path: impl AsRef<Path>,
    terms: &[&str],
    errors: &mut Vec<String>,
    hint: &str,
) {
    let Ok(entries) = std::fs::read_dir(crates_root.as_ref()) else {
        return;
    };
    let except_crate_path = except_crate_path.as_ref();

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() || path == except_crate_path {
            continue;
        }

        let manifest = path.join("Cargo.toml");
        let Some(source) = read_file_if_exists(&manifest) else {
            continue;
        };

        for term in terms {
            if source.contains(term) {
                errors.push(format!("{} contains `{term}`; {hint}", manifest.display()));
            }
        }
    }
}
