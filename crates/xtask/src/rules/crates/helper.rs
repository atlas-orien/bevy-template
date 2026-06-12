use crate::rules::base::profiles::{SimpleCrateRules, check_simple_crate};
use crate::rules::{CheckStatus, finish};

const HELPER_CRATE: &str = "crates/helper";
const HELPER_PROTOCOL: &str = "AI_PROTOCOL/HELPER.md";

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "ecs",
    "audio",
    "external_runtime",
    "gameplay",
    "intent",
    "physics",
    "prefab",
    "render_2d",
    "render_3d",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_simple_crate(
        SimpleCrateRules {
            crate_path: HELPER_CRATE,
            protocol_path: HELPER_PROTOCOL,
            anchor_hint: "helper is the shared infrastructure crate and must remain present",
            protocol_hint: "AI_PROTOCOL/HELPER.md documents the helper boundary rules",
            lib_hint: "helper needs a crate root that exports reusable infrastructure",
            required_paths: &[],
            required_paths_hint: "",
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            dependency_hint: "helper should stay shared infrastructure, so move game-specific logic to the owning crate",
            reject_direct_input: None,
        },
        &mut errors,
    );
    finish(errors)
}
