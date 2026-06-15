use crate::rules::base::profiles::{NetworkRules, check_network};
use crate::rules::{CheckStatus, finish};

const NETWORK_CRATE: &str = "crates/network";
const NETWORK_PROTOCOL: &str = "AI_PROTOCOL/NETWORK.md";

const REQUIRED_DIRS: &[&str] = &[
    "crates/network/src/connection",
    "crates/network/src/handler",
    "crates/network/src/protocol",
    "crates/network/src/request",
    "crates/network/src/router",
];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "bevy",
    "ecs",
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
    check_network(
        NetworkRules {
            crate_path: NETWORK_CRATE,
            protocol_path: NETWORK_PROTOCOL,
            required_dirs: REQUIRED_DIRS,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
        },
        &mut errors,
    );
    finish(errors)
}
