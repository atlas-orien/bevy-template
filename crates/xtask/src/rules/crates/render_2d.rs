use crate::rules::base::profiles::{Render2dRules, check_render_2d};
use crate::rules::{CheckStatus, finish};

const RENDER_2D_CRATE: &str = "crates/render_2d";
const RENDER_2D_PROTOCOL: &str = "AI_PROTOCOL/RENDER_2D.md";

const CONTENT_DIRS: &[&str] = &[
    "crates/render_2d/src/animation",
    "crates/render_2d/src/animation/frame",
    "crates/render_2d/src/animation/skeletal",
    "crates/render_2d/src/atlases",
    "crates/render_2d/src/background",
    "crates/render_2d/src/camera",
    "crates/render_2d/src/characters",
    "crates/render_2d/src/debug",
    "crates/render_2d/src/effects",
    "crates/render_2d/src/environment",
    "crates/render_2d/src/items",
    "crates/render_2d/src/lighting",
    "crates/render_2d/src/materials",
    "crates/render_2d/src/mesh",
    "crates/render_2d/src/overlays",
    "crates/render_2d/src/particles",
    "crates/render_2d/src/pixel",
    "crates/render_2d/src/props",
    "crates/render_2d/src/screens",
    "crates/render_2d/src/text",
    "crates/render_2d/src/tilemap",
    "crates/render_2d/src/transitions",
    "crates/render_2d/src/ui",
];

const OBSOLETE_PATHS: &[&str] = &[
    "crates/render_2d/src/appearance",
    "crates/render_2d/src/appearance/color.rs",
    "crates/render_2d/src/appearance/opacity.rs",
    "crates/render_2d/src/appearance/visibility.rs",
    "crates/render_2d/src/geometry",
    "crates/render_2d/src/geometry/anchor.rs",
    "crates/render_2d/src/geometry/color.rs",
    "crates/render_2d/src/geometry/opacity.rs",
    "crates/render_2d/src/geometry/shape.rs",
    "crates/render_2d/src/geometry/size.rs",
    "crates/render_2d/src/geometry/visibility.rs",
    "crates/render_2d/src/geometry/offset.rs",
    "crates/render_2d/src/geometry/scale.rs",
    "crates/render_2d/src/geometry/rotation.rs",
    "crates/render_2d/src/geometry/z_index.rs",
    "crates/render_2d/src/geometry/flip.rs",
    "crates/render_2d/src/ordering",
    "crates/render_2d/src/ordering/z_index.rs",
    "crates/render_2d/src/sprite",
    "crates/render_2d/src/sprite/flip.rs",
    "crates/render_2d/src/transform",
    "crates/render_2d/src/transform/offset.rs",
    "crates/render_2d/src/transform/rotation.rs",
    "crates/render_2d/src/transform/scale.rs",
];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "external_runtime",
    "audio",
    "intent",
    "prefab",
    "physics",
    "render_3d",
    "network",
    "msrt-udp",
];

const WORLD_RULE_TERMS: &[&str] = &[
    "set_movement_intent",
    "PhysicsRigidBody",
    "PhysicsCollider",
    "Hitbox",
    "Hurtbox",
    "Combo",
    "SkillWindow",
    "AttackWindow",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_render_2d(
        Render2dRules {
            crate_path: RENDER_2D_CRATE,
            protocol_path: RENDER_2D_PROTOCOL,
            content_dirs: CONTENT_DIRS,
            obsolete_paths: OBSOLETE_PATHS,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            world_rule_terms: WORLD_RULE_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
