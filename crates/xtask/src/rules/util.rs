use std::fs;
use std::path::{Path, PathBuf};

pub fn require_path(path: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    let path = path.as_ref();
    if !path.exists() {
        errors.push(format!(
            "required architecture anchor is missing: {}; {hint}",
            path.display()
        ));
    }
}

pub fn reject_path(path: impl AsRef<Path>, errors: &mut Vec<String>, hint: &str) {
    let path = path.as_ref();
    if path.exists() {
        errors.push(format!(
            "obsolete architecture path exists: {}; {hint}",
            path.display()
        ));
    }
}

pub fn read_file(path: impl AsRef<Path>, errors: &mut Vec<String>) -> Option<String> {
    let path = path.as_ref();
    match fs::read_to_string(path) {
        Ok(source) => Some(source),
        Err(error) => {
            errors.push(format!(
                "failed to read {}: {error}; fix the file permissions or path",
                path.display()
            ));
            None
        }
    }
}

pub fn read_file_if_exists(path: impl AsRef<Path>) -> Option<String> {
    fs::read_to_string(path).ok()
}

pub fn rust_files(root: impl AsRef<Path>) -> Vec<PathBuf> {
    files_with_extension(root, "rs")
}

pub fn files_with_extension(root: impl AsRef<Path>, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files(root.as_ref(), &mut files);
    files
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == extension))
        .collect()
}

pub fn files_named_below(root: impl AsRef<Path>, file_name: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files(root.as_ref(), &mut files);
    files
        .into_iter()
        .filter(|path| path.file_name().is_some_and(|name| name == file_name))
        .collect()
}

pub fn subdirs(root: impl AsRef<Path>) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    collect_dirs(root.as_ref(), &mut dirs);
    dirs
}

pub fn require_mod_rs_in_subdirs(root: impl AsRef<Path>, errors: &mut Vec<String>) {
    let root = root.as_ref();
    for dir in subdirs(root) {
        let mod_rs = dir.join("mod.rs");
        if !mod_rs.exists() {
            errors.push(format!(
                "{} is a module directory without mod.rs; add mod.rs or flatten the directory",
                dir.display()
            ));
        }
    }
}

pub fn reject_dir_named_files(root: impl AsRef<Path>, errors: &mut Vec<String>) {
    let root = root.as_ref();
    for dir in subdirs(root) {
        let Some(dir_name) = dir.file_name() else {
            continue;
        };

        let inside = dir.join(format!("{}.rs", dir_name.to_string_lossy()));
        if inside.exists() {
            errors.push(format!(
                "{} duplicates its parent module name; move concrete types to a semantic file such as kind.rs, settings.rs, or systems.rs",
                inside.display()
            ));
        }

        if let Some(parent) = dir.parent() {
            let sibling = parent.join(format!("{}.rs", dir_name.to_string_lossy()));
            if sibling.exists() {
                errors.push(format!(
                    "{} shadows the {} module directory; keep module directories with mod.rs instead of parallel same-name files",
                    sibling.display(),
                    dir.display()
                ));
            }
        }
    }
}

pub fn parse_rust_file(path: &Path, errors: &mut Vec<String>) -> Option<syn::File> {
    let source = read_file(path, errors)?;
    match syn::parse_file(&source) {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            errors.push(format!(
                "failed to parse {}: {error}; keep Rust syntax valid so architecture checks can inspect it",
                path.display()
            ));
            None
        }
    }
}

pub fn derived_names(item: &syn::Item) -> Option<Vec<String>> {
    let attrs = match item {
        syn::Item::Struct(item) => &item.attrs,
        syn::Item::Enum(item) => &item.attrs,
        _ => return None,
    };

    let mut names = Vec::new();

    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if let Some(ident) = meta.path.get_ident() {
                names.push(ident.to_string());
            }
            Ok(())
        });
    }

    Some(names)
}

pub fn manifest_dependency_names_result(
    manifest_source: &str,
) -> Result<Vec<String>, toml::de::Error> {
    let manifest = manifest_source.parse::<toml::Table>()?;
    Ok(collect_manifest_dependency_names(&manifest))
}

fn collect_manifest_dependency_names(manifest: &toml::Table) -> Vec<String> {
    let mut names = Vec::new();

    collect_dependency_table_names(manifest.get("dependencies"), &mut names);
    collect_dependency_table_names(manifest.get("dev-dependencies"), &mut names);
    collect_dependency_table_names(manifest.get("build-dependencies"), &mut names);

    if let Some(targets) = manifest.get("target").and_then(toml::Value::as_table) {
        for target in targets.values() {
            collect_dependency_table_names(target.get("dependencies"), &mut names);
            collect_dependency_table_names(target.get("dev-dependencies"), &mut names);
            collect_dependency_table_names(target.get("build-dependencies"), &mut names);
        }
    }

    names.sort();
    names.dedup();
    names
}

fn collect_dependency_table_names(table: Option<&toml::Value>, names: &mut Vec<String>) {
    let Some(table) = table.and_then(toml::Value::as_table) else {
        return;
    };

    for (key, value) in table {
        if let Some(package) = value
            .as_table()
            .and_then(|dependency| dependency.get("package"))
            .and_then(toml::Value::as_str)
        {
            names.push(package.to_string());
        } else {
            names.push(key.to_string());
        }
    }
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path);
        }
    }
}

fn collect_dirs(root: &Path, dirs: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            dirs.push(path.clone());
            collect_dirs(&path, dirs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::manifest_dependency_names_result;

    fn names(source: &str) -> Vec<String> {
        manifest_dependency_names_result(source).expect("test manifest should parse")
    }

    #[test]
    fn manifest_dependency_names_detects_supported_forms() {
        let source = r#"
[package]
name = "demo"
version = "0.1.0"

[dependencies]
ecs.workspace = true
inline_workspace = { package = "ecs_inline", workspace = true }
path_dep = { package = "ecs_path", path = "../ecs" }
version_dep = "0.1"
my_ecs = { package = "ecs_renamed", path = "../ecs" }

[dev-dependencies]
dev_ecs = { package = "ecs_dev", path = "../ecs" }

[target.'cfg(unix)'.dependencies]
unix_ecs = { package = "ecs_unix", path = "../ecs" }
"#;

        let names = names(source);

        for expected in [
            "ecs",
            "ecs_inline",
            "ecs_path",
            "version_dep",
            "ecs_renamed",
            "ecs_dev",
            "ecs_unix",
        ] {
            assert!(names.iter().any(|name| name == expected), "{expected}");
        }
    }

    #[test]
    fn manifest_dependency_names_avoids_prefixes_and_comments() {
        let source = r#"
[package]
name = "demo"
version = "0.1.0"

[dependencies]
ecs_helper = { path = "../ecs_helper" }
# ecs = { path = "../ecs" }
"#;

        let names = names(source);

        assert!(names.iter().any(|name| name == "ecs_helper"));
        assert!(!names.iter().any(|name| name == "ecs"));
    }
}
