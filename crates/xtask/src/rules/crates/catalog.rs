use crate::rules::base::profiles::{CatalogRules, check_catalog};
use crate::rules::{CheckStatus, finish};

const CATALOG_CRATE: &str = "crates/catalog";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_catalog(
        CatalogRules {
            crate_path: CATALOG_CRATE,
        },
        &mut errors,
    );
    finish(errors)
}
