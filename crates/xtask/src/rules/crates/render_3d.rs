use crate::rules::base::profiles::{Render3dRules, check_render_3d};
use crate::rules::{CheckStatus, finish};

const RENDER_3D_CRATE: &str = "crates/render_3d";

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
            world_rule_terms: WORLD_RULE_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
