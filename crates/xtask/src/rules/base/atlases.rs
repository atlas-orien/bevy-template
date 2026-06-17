use std::path::Path;

use crate::rules::base::source::reject_terms_in_rust_files;

pub struct AtlasesRules<'a> {
    pub atlases_dir: &'a str,
    pub forbidden_terms: &'a [&'a str],
}

pub fn check_atlases(rules: AtlasesRules<'_>, errors: &mut Vec<String>) {
    let atlases_dir = Path::new(rules.atlases_dir);
    reject_terms_in_rust_files(
        atlases_dir,
        rules.forbidden_terms,
        errors,
        "atlases is a generic primitive module, so resource loading and frame playback belong elsewhere",
    );
}
