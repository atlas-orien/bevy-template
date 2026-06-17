use crate::rules::base::profiles::{IntentRules, check_intent};
use crate::rules::{CheckStatus, finish};

const INTENT_CRATE: &str = "crates/intent";

const WORLD_MUTATION_TERMS: &[&str] = &["Commands", "Transform", "PhysicsBody", "PhysicsCollider"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_intent(
        IntentRules {
            crate_path: INTENT_CRATE,
            world_mutation_terms: WORLD_MUTATION_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
