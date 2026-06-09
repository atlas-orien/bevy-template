use std::fs;
use std::path::Path;

use super::CheckStatus;

const HELPER_CRATE: &str = "crates/helper";
const HELPER_PROTOCOL: &str = "AI_PROTOCOL/HELPER.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(HELPER_CRATE, &mut errors);
    require_path(HELPER_PROTOCOL, &mut errors);
    require_path("crates/helper/src/channel.rs", &mut errors);
    reject_forbidden_dependencies(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_forbidden_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(HELPER_CRATE).join("Cargo.toml");
    let Ok(source) = fs::read_to_string(&manifest) else {
        return;
    };

    for dependency in [
        "ecs",
        "external_runtime",
        "gameplay",
        "intent",
        "physics",
        "prefab",
        "render_2d",
        "render_3d",
    ] {
        if source.contains(&format!("{dependency}.workspace = true")) {
            errors.push(format!(
                "{} depends on `{dependency}`; helper should stay shared infrastructure",
                manifest.display()
            ));
        }
    }
}

fn require_path(path: impl AsRef<Path>, errors: &mut Vec<String>) {
    let path = path.as_ref();
    if !path.exists() {
        errors.push(format!("required path is missing: {}", path.display()));
    }
}
