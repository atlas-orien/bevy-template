use crate::rules::base::profiles::{AppRules, check_app};
use crate::rules::{CheckStatus, finish};

const APP_CRATE: &str = "crates/app";

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "ecs",
    "audio",
    "intent",
    "physics",
    "prefab",
    "render_2d",
    "render_3d",
    "network",
    "msrt-udp",
];

const FORBIDDEN_PLUGINS: &[&str] = &[
    "EcsPlugin",
    "IntentPlugin",
    "PhysicsPlugin",
    "PrefabPlugin",
    "Render2dPlugin",
    "Render3dPlugin",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_app(
        AppRules {
            crate_path: APP_CRATE,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            forbidden_plugins: FORBIDDEN_PLUGINS,
        },
        &mut errors,
    );
    finish(errors)
}
