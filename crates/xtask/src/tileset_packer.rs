use std::fs;
use std::path::Path;

use error::{ErrorKind, GameError, Result};
use helper::assets::manifests::{TilesetManifest, TilesetSourceConfig};

const SOURCE_ROOT: &str = "workbench/source_tilesets";
const OUTPUT_IMAGE_ROOT: &str = "assets/2d/static/tilemaps";
const OUTPUT_MANIFEST_ROOT: &str = "assets/2d/manifests/tilesets";

#[derive(Debug, Clone)]
pub struct PackTilesetOptions {
    pub rows: u32,
    pub tile_size: (u32, u32),
    pub from_static: bool,
}

pub fn pack_tileset_target(target: &str, options: PackTilesetOptions) -> Result<()> {
    validate_target(target)?;
    validate_options(options.rows, options.tile_size)?;

    let source_image = if options.from_static {
        output_image_path(target)
    } else {
        Path::new(SOURCE_ROOT).join(format!("{target}.png"))
    };
    let output_image = output_image_path(target);
    let output_manifest = Path::new(OUTPUT_MANIFEST_ROOT).join(format!("{target}.tileset.ron"));

    if !options.from_static {
        fs::create_dir_all(OUTPUT_IMAGE_ROOT)?;
        fs::copy(&source_image, &output_image).map_err(|error| {
            asset_error(
                "pack-tileset-copy",
                format!(
                    "failed to copy {} to {}: {error}",
                    source_image.display(),
                    output_image.display()
                ),
            )
        })?;
    } else if !source_image.exists() {
        return Err(asset_error(
            "pack-tileset-static",
            format!("{} does not exist", source_image.display()),
        ));
    }

    fs::create_dir_all(OUTPUT_MANIFEST_ROOT)?;
    let manifest = TilesetManifest {
        image: format!("2d/static/tilemaps/{target}.png"),
        array_rows: options.rows,
        tile_size: options.tile_size,
    };
    let manifest_text = ron::ser::to_string_pretty(&manifest, ron::ser::PrettyConfig::new())
        .map_err(|error| asset_error("pack-tileset-manifest", error.to_string()))?;
    fs::write(&output_manifest, manifest_text)?;

    println!("tileset image: {}", output_image.display());
    println!("tileset manifest: {}", output_manifest.display());

    Ok(())
}

pub fn pack_tileset_from_config(target: &str) -> Result<()> {
    validate_target(target)?;

    let config_path = Path::new(SOURCE_ROOT).join(format!("{target}.tileset.ron"));
    let config = TilesetSourceConfig::from_path(&config_path).map_err(|error| {
        asset_error(
            "pack-tileset-config",
            format!("failed to read {}: {error}", config_path.display()),
        )
    })?;

    pack_tileset_target(
        target,
        PackTilesetOptions {
            rows: config.rows,
            tile_size: config.tile_size,
            from_static: false,
        },
    )
}

fn output_image_path(target: &str) -> std::path::PathBuf {
    Path::new(OUTPUT_IMAGE_ROOT).join(format!("{target}.png"))
}

fn validate_target(target: &str) -> Result<()> {
    if target.is_empty()
        || target.contains('/')
        || target.contains('\\')
        || target.contains("..")
        || Path::new(target).is_absolute()
    {
        return Err(asset_error(
            "pack-tileset-target",
            "target must be a tileset name like `demo_tileset`",
        ));
    }

    Ok(())
}

fn validate_options(rows: u32, tile_size: (u32, u32)) -> Result<()> {
    if rows == 0 {
        return Err(asset_error("pack-tileset-rows", "--rows must be > 0"));
    }
    if tile_size.0 == 0 || tile_size.1 == 0 {
        return Err(asset_error(
            "pack-tileset-tile-size",
            "--tile-size values must be > 0",
        ));
    }

    Ok(())
}

fn asset_error(code: &'static str, message: impl Into<String>) -> GameError {
    GameError::from_kind(ErrorKind::Asset, code, message)
}
