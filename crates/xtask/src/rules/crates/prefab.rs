use crate::rules::base::profiles::{PrefabRules, check_prefab};
use crate::rules::{CheckStatus, finish};

const PREFAB_CRATE: &str = "crates/prefab";
const PREFAB_PROTOCOL: &str = "AI_PROTOCOL/PREFAB.md";

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "external_runtime",
    "intent",
    "gameplay",
    "network",
    "msrt-udp",
];

const UI_PRESENTATION_TERMS: &[&str] = &[
    "TextFont",
    "TextColor",
    "TextShadow",
    "BackgroundColor",
    "BorderColor",
    "Color::",
    "BorderRadius",
    "UiRect::",
    "px(",
    "percent(",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_prefab(
        PrefabRules {
            crate_path: PREFAB_CRATE,
            protocol_path: PREFAB_PROTOCOL,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            ui_presentation_terms: UI_PRESENTATION_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
