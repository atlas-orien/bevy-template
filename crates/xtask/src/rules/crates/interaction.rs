use crate::rules::base::profiles::{InteractionRules, check_interaction};
use crate::rules::{CheckStatus, finish};

const INTERACTION_CRATE: &str = "crates/interaction";

const WORLD_MUTATION_TERMS: &[&str] = &["Commands", "Transform", "PhysicsBody", "PhysicsCollider"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_interaction(
        InteractionRules {
            crate_path: INTERACTION_CRATE,
            world_mutation_terms: WORLD_MUTATION_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
