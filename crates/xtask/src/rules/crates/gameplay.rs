use crate::rules::base::profiles::{GameplayRules, check_gameplay};
use crate::rules::{CheckStatus, finish};

const GAMEPLAY_CRATE: &str = "crates/gameplay";
const GAMEPLAY_API: &str = "crates/gameplay/src/api";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_gameplay(
        GameplayRules {
            crate_path: GAMEPLAY_CRATE,
            api_path: GAMEPLAY_API,
        },
        &mut errors,
    );
    finish(errors)
}
