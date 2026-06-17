use crate::rules::base::profiles::{SimpleCrateRules, check_simple_crate};
use crate::rules::{CheckStatus, finish};

const HELPER_CRATE: &str = "crates/helper";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_simple_crate(
        SimpleCrateRules {
            crate_path: HELPER_CRATE,
            reject_direct_input: None,
        },
        &mut errors,
    );
    finish(errors)
}
