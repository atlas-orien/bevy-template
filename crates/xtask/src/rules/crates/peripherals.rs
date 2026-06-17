use crate::rules::base::profiles::{PeripheralsRules, check_peripherals};
use crate::rules::{CheckStatus, finish};

const PERIPHERALS_CRATE: &str = "crates/peripherals";

const REJECTED_PATHS: &[&str] = &["crates/peripherals/src/ui"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_peripherals(
        PeripheralsRules {
            crate_path: PERIPHERALS_CRATE,
            rejected_paths: REJECTED_PATHS,
        },
        &mut errors,
    );
    finish(errors)
}
