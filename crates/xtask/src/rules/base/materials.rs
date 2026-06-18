use std::path::{Path, PathBuf};

use crate::rules::util::{read_file, rust_files};

pub struct MaterialPresetRules<'a> {
    pub crate_path: &'a str,
    pub allowed_asset_load_dirs: &'a [&'a str],
}

pub fn check_material_presets(rules: MaterialPresetRules<'_>, errors: &mut Vec<String>) {
    let crate_path = Path::new(rules.crate_path);
    let allowed_dirs = rules
        .allowed_asset_load_dirs
        .iter()
        .map(Path::new)
        .collect::<Vec<_>>();

    for file in rust_files(crate_path.join("src")) {
        let Some(source) = read_file(&file, errors) else {
            continue;
        };
        if !loads_assets(&source) || allowed_dirs.iter().any(|allowed| is_below(&file, allowed)) {
            continue;
        }

        errors.push(format!(
            "{} loads concrete render assets outside {:?}; render_3d asset loading should go through texture/shader primitives or material presets",
            file.display(),
            rules.allowed_asset_load_dirs
        ));
    }
}

fn loads_assets(source: &str) -> bool {
    source.contains("AssetServer") || source.contains(".load(")
}

fn is_below(path: &Path, root: &Path) -> bool {
    let path = normalize(path);
    let root = normalize(root);
    path.starts_with(root)
}

fn normalize(path: &Path) -> PathBuf {
    path.components().collect()
}
