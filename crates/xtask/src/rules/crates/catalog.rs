use crate::rules::base::profiles::{CatalogRules, check_catalog};
use crate::rules::{CheckStatus, finish};

const CATALOG_CRATE: &str = "crates/catalog";
const CATALOG_PROTOCOL: &str = "AI_PROTOCOL/CATALOG.md";

const REQUIRED_PATHS: &[&str] = &["crates/catalog/src/demo.rs"];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "app",
    "dev_preview",
    "external_runtime",
    "gameplay",
    "intent",
    "network",
    "peripherals",
    "physics",
];

const MANAGED_ASSET_PATHS: &[&str] = &[
    "2d/animated/characters/demo-player/demo-player.frames.ron",
    "2d/static/tilemaps/demo_tileset.png",
    "2d/static/props/demo-skeletal-bone/demo-skeletal-bone.png",
    "2d/static/props/demo-skeletal-joint/demo-skeletal-joint.png",
];

const MANAGED_ASSET_PATH_CONSUMERS: &[&str] = &["crates/gameplay/src", "crates/dev_preview/src"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_catalog(
        CatalogRules {
            crate_path: CATALOG_CRATE,
            protocol_path: CATALOG_PROTOCOL,
            required_paths: REQUIRED_PATHS,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            managed_asset_paths: MANAGED_ASSET_PATHS,
            managed_asset_path_consumers: MANAGED_ASSET_PATH_CONSUMERS,
        },
        &mut errors,
    );
    finish(errors)
}
