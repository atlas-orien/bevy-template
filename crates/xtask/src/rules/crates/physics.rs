use crate::rules::base::profiles::{PhysicsRules, check_physics};
use crate::rules::{CheckStatus, finish};

const PHYSICS_CRATE: &str = "crates/physics";
const PHYSICS_PROTOCOL: &str = "AI_PROTOCOL/PHYSICS.md";
const PHYSICS_BACKEND: &str = "crates/physics/src/backend/rapier";

const BACKENDS: &[&str] = &["avian2d", "avian3d", "bevy_rapier2d", "bevy_rapier3d"];

const REQUIRED_PATHS: &[&str] = &[
    "crates/physics/src/body/kind.rs",
    "crates/physics/src/body/control.rs",
    "crates/physics/src/controller/character.rs",
    "crates/physics/src/config/settings.rs",
    "crates/physics/src/collider/shape.rs",
    "crates/physics/src/collider/control.rs",
    "crates/physics/src/collider/filter.rs",
    "crates/physics/src/layer/collision_layer.rs",
    "crates/physics/src/sensor/marker.rs",
    "crates/physics/src/material/surface.rs",
    "crates/physics/src/mass/properties.rs",
    "crates/physics/src/motion/velocity.rs",
    "crates/physics/src/force/linear.rs",
    "crates/physics/src/joint/impulse.rs",
    "crates/physics/src/events/collision.rs",
    "crates/physics/src/events/contact_force.rs",
    "crates/physics/src/query/filter.rs",
    "crates/physics/src/query/point.rs",
    "crates/physics/src/query/raycast.rs",
    "crates/physics/src/query/shape.rs",
    "crates/physics/src/backend/rapier/mod.rs",
    "crates/physics/src/backend/rapier/dim2/mod.rs",
    "crates/physics/src/backend/rapier/dim2/convert.rs",
    "crates/physics/src/backend/rapier/dim2/events.rs",
    "crates/physics/src/backend/rapier/dim2/query.rs",
    "crates/physics/src/backend/rapier/dim2/systems.rs",
    "crates/physics/src/backend/rapier/dim3/mod.rs",
    "crates/physics/src/backend/rapier/dim3/convert.rs",
    "crates/physics/src/backend/rapier/dim3/events.rs",
    "crates/physics/src/backend/rapier/dim3/query.rs",
    "crates/physics/src/backend/rapier/dim3/systems.rs",
];

const OBSOLETE_PATHS: &[&str] = &[
    "crates/physics/src/body.rs",
    "crates/physics/src/body/body.rs",
    "crates/physics/src/body/body_control.rs",
    "crates/physics/src/collider.rs",
    "crates/physics/src/collider/collider.rs",
    "crates/physics/src/collider/collider_control.rs",
    "crates/physics/src/config.rs",
    "crates/physics/src/config/config.rs",
    "crates/physics/src/events.rs",
    "crates/physics/src/force.rs",
    "crates/physics/src/force/force.rs",
    "crates/physics/src/layer.rs",
    "crates/physics/src/layer/layer.rs",
    "crates/physics/src/mass.rs",
    "crates/physics/src/mass/mass.rs",
    "crates/physics/src/material.rs",
    "crates/physics/src/material/material.rs",
    "crates/physics/src/motion.rs",
    "crates/physics/src/rigid_body",
    "crates/physics/src/rigid_body/rigid_body.rs",
    "crates/physics/src/sensor.rs",
    "crates/physics/src/sensor/sensor.rs",
    "crates/physics/src/backend/avian2d",
    "crates/physics/src/backend/avian2d.rs",
    "crates/physics/src/backend/avian3d",
    "crates/physics/src/backend/avian3d.rs",
    "crates/physics/src/backend/rapier2d",
    "crates/physics/src/backend/rapier2d.rs",
    "crates/physics/src/backend/rapier3d",
    "crates/physics/src/backend/rapier3d.rs",
];

const FORBIDDEN_MANIFEST_TERMS: &[&str] = &["[features]", "avian2d", "avian3d"];

const GAMEPLAY_TERMS: &[&str] = &["Hitbox", "Hurtbox", "AttackRange", "SkillRange"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_physics(
        PhysicsRules {
            crate_path: PHYSICS_CRATE,
            protocol_path: PHYSICS_PROTOCOL,
            backend_path: PHYSICS_BACKEND,
            required_paths: REQUIRED_PATHS,
            obsolete_paths: OBSOLETE_PATHS,
            backends: BACKENDS,
            forbidden_manifest_terms: FORBIDDEN_MANIFEST_TERMS,
            gameplay_terms: GAMEPLAY_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
