use std::path::Path;

use crate::rules::base::source::reject_terms_in_rust_files;

pub struct ImagesRules<'a> {
    pub images_dir: &'a str,
    pub forbidden_terms: &'a [&'a str],
}

pub struct TextRules<'a> {
    pub text_dir: &'a str,
    pub forbidden_terms: &'a [&'a str],
}

pub fn check_images(rules: ImagesRules<'_>, errors: &mut Vec<String>) {
    let images_dir = Path::new(rules.images_dir);
    reject_terms_in_rust_files(
        images_dir,
        rules.forbidden_terms,
        errors,
        "images must not load concrete resources or define behavior systems; pass handles in from catalog or higher layers",
    );
}

pub fn check_text(rules: TextRules<'_>, errors: &mut Vec<String>) {
    let text_dir = Path::new(rules.text_dir);
    reject_terms_in_rust_files(
        text_dir,
        rules.forbidden_terms,
        errors,
        "text must not load concrete fonts or handle UI layout; pass font handles in from catalog or higher layers",
    );
}
