use crate::rules::base::profiles::{NavigationRules, check_navigation};
use crate::rules::{CheckStatus, finish};

const NAVIGATION_CRATE: &str = "crates/navigation";

const RENDER_TERMS: &[&str] = &[
    "Sprite",
    "Camera2d",
    "Camera3d",
    "Text2d",
    "Node",
    "ImageNode",
];

const FORBIDDEN_IMPORT_TERMS: &[&str] =
    &["external_runtime::", "intent::", "gameplay::", "prefab::"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_navigation(
        NavigationRules {
            crate_path: NAVIGATION_CRATE,
            render_terms: RENDER_TERMS,
            forbidden_import_terms: FORBIDDEN_IMPORT_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
