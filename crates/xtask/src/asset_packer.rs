use std::fs;
use std::path::Path;

use error::{ErrorKind, GameError, Result};

use crate::frame_packer::{PackFrameOptions, pack_frame_target};
use crate::tileset_packer::pack_tileset_from_config;

const SOURCE_FRAMES_ROOT: &str = "workbench/source_frames";
const SOURCE_TILESETS_ROOT: &str = "workbench/source_tilesets";

pub fn pack_all_assets() -> Result<()> {
    let mut packed_count = 0usize;

    for target in discover_frame_targets()? {
        pack_frame_target(&target, PackFrameOptions::default())?;
        packed_count += 1;
    }

    for target in discover_tileset_targets()? {
        pack_tileset_from_config(&target)?;
        packed_count += 1;
    }

    println!("packed {packed_count} asset target(s)");
    Ok(())
}

fn discover_frame_targets() -> Result<Vec<String>> {
    let root = Path::new(SOURCE_FRAMES_ROOT);
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut targets = Vec::new();
    for category in fs::read_dir(root)? {
        let category = category?;
        let category_path = category.path();
        if !category_path.is_dir() {
            continue;
        }

        for resource in fs::read_dir(&category_path)? {
            let resource = resource?;
            let resource_path = resource.path();
            if !resource_path.is_dir() || !contains_png(&resource_path)? {
                continue;
            }
            targets.push(relative_target(root, &resource_path)?);
        }
    }

    targets.sort();
    Ok(targets)
}

fn discover_tileset_targets() -> Result<Vec<String>> {
    let root = Path::new(SOURCE_TILESETS_ROOT);
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut targets = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() || path.extension().is_none_or(|extension| extension != "png") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        let config = root.join(format!("{stem}.tileset.ron"));
        if !config.exists() {
            return Err(asset_error(
                "pack-assets-tileset-config",
                format!("{} requires {}", path.display(), config.display()),
            ));
        }
        targets.push(stem.to_string());
    }

    targets.sort();
    Ok(targets)
}

fn contains_png(path: &Path) -> Result<bool> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|extension| extension == "png") {
            return Ok(true);
        }
    }

    Ok(false)
}

fn relative_target(root: &Path, path: &Path) -> Result<String> {
    let relative = path.strip_prefix(root).map_err(|error| {
        asset_error(
            "pack-assets-target",
            format!("failed to resolve {}: {error}", path.display()),
        )
    })?;

    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn asset_error(code: &'static str, message: impl Into<String>) -> GameError {
    GameError::from_kind(ErrorKind::Asset, code, message)
}
