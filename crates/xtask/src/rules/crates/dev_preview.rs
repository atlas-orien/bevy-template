use crate::rules::base::profiles::{DevPreviewRules, check_dev_preview};
use crate::rules::{CheckStatus, finish};

const DEV_PREVIEW_CRATE: &str = "crates/dev_preview";
const DEV_PREVIEW_PROTOCOL: &str = "AI_PROTOCOL/DEV_PREVIEW.md";

const REQUIRED_PREVIEWS: &[&str] = &["demo_menu.rs"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_dev_preview(
        DevPreviewRules {
            crate_path: DEV_PREVIEW_CRATE,
            protocol_path: DEV_PREVIEW_PROTOCOL,
            required_previews: REQUIRED_PREVIEWS,
        },
        &mut errors,
    );
    finish(errors)
}
