use crate::rules::base::profiles::{InteractionRules, check_interaction};
use crate::rules::{CheckStatus, finish};

const INTERACTION_CRATE: &str = "crates/interaction";
const INTERACTION_PROTOCOL: &str = "AI_PROTOCOL/INTERACTION.md";

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "physics",
    "prefab",
    "render_2d",
    "render_3d",
    "external_runtime",
    "audio",
];

const WORLD_MUTATION_TERMS: &[&str] = &["Commands", "Transform", "PhysicsBody", "PhysicsCollider"];

const UI_NAVIGATION_TERMS: &[&str] = &[
    "UiNavigationInputMessage",
    "UiNavigationInputKind",
    "Previous",
    "Next",
    "Activate",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_interaction(
        InteractionRules {
            crate_path: INTERACTION_CRATE,
            protocol_path: INTERACTION_PROTOCOL,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            world_mutation_terms: WORLD_MUTATION_TERMS,
            required_navigation_terms: UI_NAVIGATION_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
