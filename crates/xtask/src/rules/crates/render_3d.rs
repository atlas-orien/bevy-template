use crate::rules::base::profiles::{Render3dRules, check_render_3d};
use crate::rules::{CheckStatus, finish};

const RENDER_3D_CRATE: &str = "crates/render_3d";
const RENDER_3D_PROTOCOL: &str = "AI_PROTOCOL/RENDER_3D.md";

const CONTENT_DIRS: &[&str] = &[
    "crates/render_3d/src/animation",
    "crates/render_3d/src/camera",
    "crates/render_3d/src/characters",
    "crates/render_3d/src/debug",
    "crates/render_3d/src/effects",
    "crates/render_3d/src/environment",
    "crates/render_3d/src/items",
    "crates/render_3d/src/lighting",
    "crates/render_3d/src/materials",
    "crates/render_3d/src/models",
    "crates/render_3d/src/overlays",
    "crates/render_3d/src/particles",
    "crates/render_3d/src/props",
    "crates/render_3d/src/scenes",
];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "external_runtime",
    "audio",
    "intent",
    "prefab",
    "physics",
    "render_2d",
];

const WORLD_RULE_TERMS: &[&str] = &[
    "set_movement_intent",
    "PhysicsRigidBody",
    "PhysicsCollider2d",
    "PhysicsCollider3d",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_render_3d(
        Render3dRules {
            crate_path: RENDER_3D_CRATE,
            protocol_path: RENDER_3D_PROTOCOL,
            content_dirs: CONTENT_DIRS,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            world_rule_terms: WORLD_RULE_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
