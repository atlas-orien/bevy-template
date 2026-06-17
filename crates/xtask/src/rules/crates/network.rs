use crate::rules::base::profiles::{NetworkRules, check_network};
use crate::rules::{CheckStatus, finish};

const NETWORK_CRATE: &str = "crates/network";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_network(
        NetworkRules {
            crate_path: NETWORK_CRATE,
        },
        &mut errors,
    );
    finish(errors)
}
