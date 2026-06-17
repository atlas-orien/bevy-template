use crate::rules::base::profiles::{ErrorRules, check_error};
use crate::rules::{CheckStatus, finish};

const ERROR_CRATE: &str = "crates/error";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_error(
        ErrorRules {
            crate_path: ERROR_CRATE,
        },
        &mut errors,
    );
    finish(errors)
}
