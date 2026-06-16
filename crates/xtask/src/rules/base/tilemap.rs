use std::path::Path;

use crate::rules::base::paths::{reject_files_under_dir_except, reject_subdirs_except};
use crate::rules::base::source::{
    reject_terms_in_file, reject_terms_in_rust_files, require_file_contains_all_terms,
};
use crate::rules::util::require_path;

pub struct TilemapRules<'a> {
    pub tilemap_dir: &'a str,
    pub required_files: &'a [&'a str],
    pub allowed_files: &'a [&'a str],
    pub required_chunk_api_terms: &'a [&'a str],
    pub forbidden_terms: &'a [&'a str],
}

pub fn check_tilemap(rules: TilemapRules<'_>, errors: &mut Vec<String>) {
    let tilemap_dir = Path::new(rules.tilemap_dir);
    require_path(
        tilemap_dir,
        errors,
        "render_2d tilemap must stay as the shared tilemap primitive module",
    );

    for file in rules.required_files {
        require_path(
            tilemap_dir.join(file),
            errors,
            "tilemap must expose Bevy TilemapChunk through a named reusable chunk layer primitive",
        );
    }

    reject_files_under_dir_except(
        tilemap_dir,
        rules.allowed_files,
        errors,
        "tilemap should remain a thin primitive module until concrete tilemap products are introduced elsewhere",
    );
    reject_subdirs_except(
        tilemap_dir,
        &[],
        errors,
        "tilemap should not contain demo products; concrete demo layout belongs in prefab/catalog",
    );
    require_file_contains_all_terms(
        tilemap_dir.join("chunk.rs"),
        rules.required_chunk_api_terms,
        errors,
        "tilemap chunk must expose a reusable TilemapChunkLayer2d primitive with caller-provided transform data",
    );
    reject_terms_in_rust_files(
        tilemap_dir,
        rules.forbidden_terms,
        errors,
        "tilemap is a generic primitive module, so demo names and hardcoded demo layout must stay outside render_2d/tilemap",
    );
    reject_terms_in_file(
        tilemap_dir.join("chunk.rs"),
        &["const ", "DEMO_", "demo_"],
        errors,
        "tilemap chunk should not hardcode concrete map constants; pass map data from prefab/catalog",
    );
}
