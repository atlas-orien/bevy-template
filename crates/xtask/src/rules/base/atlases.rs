use std::path::Path;

use crate::rules::base::paths::{reject_files_under_dir_except, reject_subdirs_except};
use crate::rules::base::source::{reject_terms_in_rust_files, require_file_contains_all_terms};
use crate::rules::util::require_path;

pub struct AtlasesRules<'a> {
    pub atlases_dir: &'a str,
    pub required_files: &'a [&'a str],
    pub allowed_files: &'a [&'a str],
    pub required_sprite_api_terms: &'a [&'a str],
    pub forbidden_terms: &'a [&'a str],
}

pub fn check_atlases(rules: AtlasesRules<'_>, errors: &mut Vec<String>) {
    let atlases_dir = Path::new(rules.atlases_dir);
    require_path(
        atlases_dir,
        errors,
        "render_2d atlases must stay as the shared texture atlas primitive module",
    );

    for file in rules.required_files {
        require_path(
            atlases_dir.join(file),
            errors,
            "atlases must expose a named reusable atlas sprite primitive",
        );
    }

    reject_files_under_dir_except(
        atlases_dir,
        rules.allowed_files,
        errors,
        "atlases should remain a thin primitive module until concrete atlas products are introduced elsewhere",
    );
    reject_subdirs_except(
        atlases_dir,
        &[],
        errors,
        "atlases should not contain demo products; concrete atlas usage belongs in products or prefab",
    );
    require_file_contains_all_terms(
        atlases_dir.join("mod.rs"),
        rules.required_sprite_api_terms,
        errors,
        "atlases must expose AtlasSprite2d with caller-provided image/layout/index data",
    );
    reject_terms_in_rust_files(
        atlases_dir,
        rules.forbidden_terms,
        errors,
        "atlases is a generic primitive module, so resource loading and frame playback belong elsewhere",
    );
}
