use crate::rules::base::profiles::{PeripheralsRules, check_peripherals};
use crate::rules::{CheckStatus, finish};

const PERIPHERALS_CRATE: &str = "crates/peripherals";
const PERIPHERALS_PROTOCOL: &str = "AI_PROTOCOL/PERIPHERALS.md";

const REQUIRED_DIRS: &[&str] = &[
    "crates/peripherals/src/keyboard",
    "crates/peripherals/src/mouse",
    "crates/peripherals/src/gamepad",
];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "ecs",
    "physics",
    "prefab",
    "render_2d",
    "render_3d",
    "external_runtime",
    "audio",
];

const REJECTED_PATHS: &[&str] = &["crates/peripherals/src/ui"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_peripherals(
        PeripheralsRules {
            crate_path: PERIPHERALS_CRATE,
            protocol_path: PERIPHERALS_PROTOCOL,
            required_dirs: REQUIRED_DIRS,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            required_dependency: "interaction",
            rejected_paths: REJECTED_PATHS,
        },
        &mut errors,
    );
    finish(errors)
}
