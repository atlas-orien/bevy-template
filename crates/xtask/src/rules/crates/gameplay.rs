use crate::rules::base::profiles::{GameplayRules, check_gameplay};
use crate::rules::{CheckStatus, finish};

const GAMEPLAY_CRATE: &str = "crates/gameplay";
const GAMEPLAY_PROTOCOL: &str = "AI_PROTOCOL/GAMEPLAY.md";
const GAMEPLAY_API: &str = "crates/gameplay/src/api";

const REQUIRED_DIRS: &[&str] = &[
    GAMEPLAY_API,
    "crates/gameplay/src/lifecycle",
    "crates/gameplay/src/schedule",
    "crates/gameplay/src/interaction",
];

const ALLOWED_INTERACTION_CATEGORIES: &[&str] = &["ui"];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "ecs",
    "audio",
    "external_runtime",
    "physics",
    "render_3d",
    "network",
    "msrt-udp",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_gameplay(
        GameplayRules {
            crate_path: GAMEPLAY_CRATE,
            protocol_path: GAMEPLAY_PROTOCOL,
            api_path: GAMEPLAY_API,
            required_dirs: REQUIRED_DIRS,
            allowed_interaction_categories: ALLOWED_INTERACTION_CATEGORIES,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
        },
        &mut errors,
    );
    finish(errors)
}
