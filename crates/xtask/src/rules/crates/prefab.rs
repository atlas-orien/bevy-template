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

const FORBIDDEN_ASSET_PATH_TERMS: &[&str] = &[
    "\"audio/", "\"2d/", "\"3d/", ".ogg\"", ".wav\"", ".mp3\"", ".png\"", ".jpg\"", ".jpeg\"",
    ".ron\"", ".glb\"", ".gltf\"",
];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_prefab(
        PrefabRules {
            crate_path: PREFAB_CRATE,
            protocol_path: PREFAB_PROTOCOL,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            ui_presentation_terms: UI_PRESENTATION_TERMS,
            forbidden_asset_path_terms: FORBIDDEN_ASSET_PATH_TERMS,
        },
        &mut errors,
    );
    finish(errors)
}
