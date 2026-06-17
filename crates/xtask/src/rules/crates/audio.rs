use crate::rules::base::profiles::{SimpleCrateRules, check_simple_crate};
use crate::rules::{CheckStatus, finish};

const AUDIO_CRATE: &str = "crates/audio";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_simple_crate(
        SimpleCrateRules {
            crate_path: AUDIO_CRATE,
            reject_direct_input: Some(
                "audio must not read input, so convert external sources before playback requests",
            ),
        },
        &mut errors,
    );
    finish(errors)
}
