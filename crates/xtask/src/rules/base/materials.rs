use std::path::{Path, PathBuf};

use crate::rules::util::{read_file, rust_files};

pub struct MaterialPresetRules<'a> {
    pub crate_path: &'a str,
    pub allowed_asset_load_dir: &'a str,
}

pub fn check_material_presets(rules: MaterialPresetRules<'_>, errors: &mut Vec<String>) {
    let crate_path = Path::new(rules.crate_path);
    let allowed_dir = Path::new(rules.allowed_asset_load_dir);

    for file in rust_files(crate_path.join("src")) {
        let Some(source) = read_file(&file, errors) else {
            continue;
        };
        if !loads_assets(&source) || is_below(&file, allowed_dir) {
            continue;
        }

        errors.push(format!(
            "{} loads concrete render assets outside {}; material preview/demo asset loading belongs in material presets",
            file.display(),
            allowed_dir.display()
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
