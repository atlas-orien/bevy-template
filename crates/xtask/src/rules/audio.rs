use std::fs;
use std::path::Path;

use super::CheckStatus;

const AUDIO_CRATE: &str = "crates/audio";
const AUDIO_PROTOCOL: &str = "AI_PROTOCOL/AUDIO.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(AUDIO_CRATE, &mut errors);
    require_path(AUDIO_PROTOCOL, &mut errors);
    require_path("crates/audio/src/source/sample.rs", &mut errors);
    require_path("crates/audio/src/source/procedural.rs", &mut errors);
    require_path("crates/audio/src/spatial/audio_2d.rs", &mut errors);
    require_path("crates/audio/src/spatial/audio_3d.rs", &mut errors);
    require_path("crates/audio/src/playback.rs", &mut errors);
    require_path("crates/audio/src/bus.rs", &mut errors);
    require_path("crates/audio/src/volume.rs", &mut errors);
    require_path("crates/audio/src/request.rs", &mut errors);
    reject_dependencies(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(AUDIO_CRATE).join("Cargo.toml");
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
                "{} depends on `{dependency}`; audio should stay a foundation layer",
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
