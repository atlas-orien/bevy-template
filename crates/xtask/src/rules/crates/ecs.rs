use crate::rules::base::profiles::{EcsRules, check_ecs};
use crate::rules::{CheckStatus, finish};

const OBSOLETE_PATHS: &[&str] = &["crates/components", "crates/system"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_ecs(
        EcsRules {
            obsolete_paths: OBSOLETE_PATHS,
            components_root: "crates/ecs/src/components",
            resources_root: "crates/ecs/src/resources",
            events_root: "crates/ecs/src/events",
            systems_root: "crates/ecs/src/systems",
        },
        &mut errors,
    );
    finish(errors)
}
