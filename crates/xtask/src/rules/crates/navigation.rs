use crate::rules::base::profiles::{NavigationRules, check_navigation};
use crate::rules::{CheckStatus, finish};

const NAVIGATION_CRATE: &str = "crates/navigation";
const NAVIGATION_PROTOCOL: &str = "AI_PROTOCOL/NAVIGATION.md";

const REQUIRED_DIRS: &[&str] = &[
    "crates/navigation/src/agent",
    "crates/navigation/src/target",
    "crates/navigation/src/path",
    "crates/navigation/src/query",
    "crates/navigation/src/systems",
];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "external_runtime",
    "intent",
    "gameplay",
    "prefab",
    "physics",
    "render_2d",
    "render_3d",
    "network",
    "msrt-udp",
];

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
            protocol_path: NAVIGATION_PROTOCOL,
            required_dirs: REQUIRED_DIRS,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            render_terms: RENDER_TERMS,
            forbidden_import_terms: FORBIDDEN_IMPORT_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
