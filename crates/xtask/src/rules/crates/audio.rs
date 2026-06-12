use crate::rules::base::profiles::{SimpleCrateRules, check_simple_crate};
use crate::rules::{CheckStatus, finish};

const AUDIO_CRATE: &str = "crates/audio";
const AUDIO_PROTOCOL: &str = "AI_PROTOCOL/AUDIO.md";

const REQUIRED_PATHS: &[&str] = &["crates/audio/src/source", "crates/audio/src/spatial"];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "ecs",
    "external_runtime",
    "gameplay",
    "intent",
    "physics",
    "prefab",
    "render_2d",
    "render_3d",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_simple_crate(
        SimpleCrateRules {
            crate_path: AUDIO_CRATE,
            protocol_path: AUDIO_PROTOCOL,
            anchor_hint: "audio is the foundation layer for sound data and playback requests",
            protocol_hint: "AI_PROTOCOL/AUDIO.md documents the audio boundary rules",
            lib_hint: "audio needs a crate root that exports its public facade",
            required_paths: REQUIRED_PATHS,
            required_paths_hint: "audio source/spatial concepts should stay grouped by directory",
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            dependency_hint: "audio should stay a foundation layer, so move gameplay/content decisions outside audio",
            reject_direct_input: Some(
                "audio must not read input, so convert external sources before playback requests",
            ),
        },
        &mut errors,
    );
    finish(errors)
}
