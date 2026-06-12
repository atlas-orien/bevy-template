use crate::rules::base::profiles::{ExternalRuntimeRules, check_external_runtime};
use crate::rules::{CheckStatus, finish};

const EXTERNAL_RUNTIME_CRATE: &str = "crates/external_runtime";
const EXTERNAL_RUNTIME_PROTOCOL: &str = "AI_PROTOCOL/EXTERNAL_RUNTIME.md";

const REQUIRED_DIRS: &[&str] = &[
    "crates/external_runtime/src/input/ai",
    "crates/external_runtime/src/runtime",
    "crates/external_runtime/src/manager",
    "crates/external_runtime/src/bridge",
];

const REJECTED_PATHS: &[&str] = &[
    "crates/external_runtime/src/input/local",
    "crates/external_runtime/src/input/device",
    "crates/external_runtime/src/local",
    "crates/external_runtime/src/device",
    "crates/external_runtime/src/peripherals",
    "crates/external_runtime/src/network",
];

const FORBIDDEN_DEPENDENCIES: &[&str] = &["ecs", "audio", "physics", "render_2d", "render_3d"];

const FORBIDDEN_PLUGIN_TERMS: &[&str] =
    &["InputPlugin", "ExternalRuntimePlugin", "impl Plugin for"];

const MANAGER_USER_FILES: &[&str] = &[
    "crates/external_runtime/src/manager/user.rs",
    "crates/external_runtime/src/manager/mod.rs",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_external_runtime(
        ExternalRuntimeRules {
            crate_path: EXTERNAL_RUNTIME_CRATE,
            protocol_path: EXTERNAL_RUNTIME_PROTOCOL,
            required_dirs: REQUIRED_DIRS,
            rejected_paths: REJECTED_PATHS,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            forbidden_plugin_terms: FORBIDDEN_PLUGIN_TERMS,
            manager_user_files: MANAGER_USER_FILES,
        },
        &mut errors,
    );
    finish(errors)
}
