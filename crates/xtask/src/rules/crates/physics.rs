use crate::rules::base::profiles::{PhysicsRules, check_physics};
use crate::rules::{CheckStatus, finish};

const PHYSICS_CRATE: &str = "crates/physics";

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
    "crates/physics/src/backend/rapier2d",
    "crates/physics/src/backend/rapier2d.rs",
    "crates/physics/src/backend/rapier3d",
    "crates/physics/src/backend/rapier3d.rs",
];

const GAMEPLAY_TERMS: &[&str] = &["Hitbox", "Hurtbox", "AttackRange", "SkillRange"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_physics(
        PhysicsRules {
            crate_path: PHYSICS_CRATE,
            obsolete_paths: OBSOLETE_PATHS,
            gameplay_terms: GAMEPLAY_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
