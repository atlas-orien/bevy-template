use crate::rules::base::profiles::{DevPreviewRules, check_dev_preview};
use crate::rules::{CheckStatus, finish};

const DEV_PREVIEW_CRATE: &str = "crates/dev_preview";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_dev_preview(
        DevPreviewRules {
            crate_path: DEV_PREVIEW_CRATE,
        },
        &mut errors,
    );
    finish(errors)
}
