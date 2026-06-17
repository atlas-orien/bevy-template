use std::path::Path;

use crate::rules::base::source::reject_terms_in_rust_files;

pub struct TilemapRules<'a> {
    pub tilemap_dir: &'a str,
    pub forbidden_terms: &'a [&'a str],
}

pub fn check_tilemap(rules: TilemapRules<'_>, errors: &mut Vec<String>) {
    let tilemap_dir = Path::new(rules.tilemap_dir);
    reject_terms_in_rust_files(
        tilemap_dir,
        rules.forbidden_terms,
        errors,
        "tilemap is a generic primitive module, so demo names and hardcoded demo layout must stay outside render_2d/tilemap",
    );
}
