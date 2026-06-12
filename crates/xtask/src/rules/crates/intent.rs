use crate::rules::base::profiles::{IntentRules, check_intent};
use crate::rules::{CheckStatus, finish};

const INTENT_CRATE: &str = "crates/intent";
const INTENT_PROTOCOL: &str = "AI_PROTOCOL/INTENT.md";

const FORBIDDEN_DEPENDENCIES: &[&str] = &["ecs", "audio", "physics", "render_2d", "render_3d"];

const WORLD_MUTATION_TERMS: &[&str] = &[
    "Commands",
    "Transform",
    "PhysicsRigidBody",
    "PhysicsCollider2d",
    "PhysicsCollider3d",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_intent(
        IntentRules {
            crate_path: INTENT_CRATE,
            protocol_path: INTENT_PROTOCOL,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            world_mutation_terms: WORLD_MUTATION_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
