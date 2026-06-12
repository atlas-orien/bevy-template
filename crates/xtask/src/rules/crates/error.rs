use crate::rules::base::profiles::{ErrorRules, check_error};
use crate::rules::{CheckStatus, finish};

const ERROR_CRATE: &str = "crates/error";
const ERROR_PROTOCOL: &str = "AI_PROTOCOL/ERROR.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_error(
        ErrorRules {
            crate_path: ERROR_CRATE,
            protocol_path: ERROR_PROTOCOL,
        },
        &mut errors,
    );
    finish(errors)
}
