use std::path::Path;

use crate::rules::base::paths::{reject_files_under_dir_except, reject_subdirs_except};
use crate::rules::base::source::{reject_terms_in_rust_files, require_file_contains_all_terms};
use crate::rules::util::require_path;

pub struct ImagesRules<'a> {
    pub images_dir: &'a str,
    pub allowed_files: &'a [&'a str],
    pub required_api_terms: &'a [&'a str],
    pub forbidden_terms: &'a [&'a str],
}

pub struct TextRules<'a> {
    pub text_dir: &'a str,
    pub allowed_files: &'a [&'a str],
    pub required_api_terms: &'a [&'a str],
    pub forbidden_terms: &'a [&'a str],
}

pub fn check_images(rules: ImagesRules<'_>, errors: &mut Vec<String>) {
    let images_dir = Path::new(rules.images_dir);
    require_path(
        images_dir,
        errors,
        "render_2d images must stay as the shared static image primitive module",
    );
    require_path(
        images_dir.join("mod.rs"),
        errors,
        "render_2d images exposes its primitive API from mod.rs",
    );
    reject_files_under_dir_except(
        images_dir,
        rules.allowed_files,
        errors,
        "images should remain a thin primitive module until concrete image presets are introduced",
    );
    reject_subdirs_except(
        images_dir,
        &[],
        errors,
        "images should not grow subdomains before static image primitives stabilize",
    );
    require_file_contains_all_terms(
        images_dir.join("mod.rs"),
        rules.required_api_terms,
        errors,
        "images must expose a named static image primitive and bundle that higher-level render modules can compose",
    );
    reject_terms_in_rust_files(
        images_dir,
        rules.forbidden_terms,
        errors,
        "images must not load concrete resources or define behavior systems; pass handles in from catalog or higher layers",
    );
}

pub fn check_text(rules: TextRules<'_>, errors: &mut Vec<String>) {
    let text_dir = Path::new(rules.text_dir);
    require_path(
        text_dir,
        errors,
        "render_2d text must stay as the shared world text primitive module",
    );
    require_path(
        text_dir.join("mod.rs"),
        errors,
        "render_2d text exposes its primitive API from mod.rs",
    );
    reject_files_under_dir_except(
        text_dir,
        rules.allowed_files,
        errors,
        "text should contain the world text primitive and its plugin only",
    );
    reject_subdirs_except(
        text_dir,
        &[],
        errors,
        "text should not grow UI text or preset subdomains before world text primitives stabilize",
    );
    require_file_contains_all_terms(
        text_dir.join("mod.rs"),
        rules.required_api_terms,
        errors,
        "text must expose a named world-space text primitive and bundle that higher-level render modules can compose",
    );
    reject_terms_in_rust_files(
        text_dir,
        rules.forbidden_terms,
        errors,
        "text must not load concrete fonts or handle UI layout; pass font handles in from catalog or higher layers",
    );
}
