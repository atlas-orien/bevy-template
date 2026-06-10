use std::path::Path;

use super::CheckStatus;
use super::util::{
    manifest_has_workspace_dependency, read_file_if_exists, require_mod_rs_in_subdirs,
    require_path, rust_files,
};

const AUDIO_CRATE: &str = "crates/audio";
const AUDIO_PROTOCOL: &str = "AI_PROTOCOL/AUDIO.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_path(
        AUDIO_CRATE,
        &mut errors,
        "audio is the foundation layer for sound data and playback requests",
    );
    require_path(
        AUDIO_PROTOCOL,
        &mut errors,
        "AI_PROTOCOL/AUDIO.md documents the audio boundary rules",
    );
    require_path(
        "crates/audio/src/lib.rs",
        &mut errors,
        "audio needs a crate root that exports its public facade",
    );
    for dir in ["crates/audio/src/source", "crates/audio/src/spatial"] {
        require_path(
            dir,
            &mut errors,
            "audio source/spatial concepts should stay grouped by directory",
        );
    }
    require_mod_rs_in_subdirs(Path::new(AUDIO_CRATE).join("src"), &mut errors);
    reject_direct_input(&mut errors);
    reject_dependencies(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_dependencies(errors: &mut Vec<String>) {
    let manifest = Path::new(AUDIO_CRATE).join("Cargo.toml");
    let Some(source) = read_file_if_exists(&manifest) else {
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
        if manifest_has_workspace_dependency(&source, dependency) {
            errors.push(format!(
                "{} depends on `{dependency}`; audio should stay a foundation layer, so move gameplay/content decisions outside audio",
                manifest.display()
            ));
        }
    }
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(AUDIO_CRATE) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; audio must not read input, so convert external sources before playback requests",
                    file.display()
                ));
            }
        }
    }
}
