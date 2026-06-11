use std::path::Path;

use crate::rules::CheckStatus;
use crate::rules::base::dependencies::reject_workspace_dependencies;
use crate::rules::base::paths::{require_crate_anchor, require_mod_rs_under_src};
use crate::rules::util::{read_file_if_exists, require_path, rust_files};

const PREFAB_CRATE: &str = "crates/prefab";
const PREFAB_PROTOCOL: &str = "AI_PROTOCOL/PREFAB.md";

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();

    require_crate_anchor(PREFAB_CRATE, PREFAB_PROTOCOL, &mut errors);
    require_path(
        "crates/prefab/src/lib.rs",
        &mut errors,
        "prefab needs a crate root that exports spawn/template facades",
    );
    require_mod_rs_under_src(PREFAB_CRATE, &mut errors);
    reject_forbidden_dependencies(&mut errors);
    reject_direct_input(&mut errors);
    reject_nested_prefab_spawn(&mut errors);
    reject_ui_presentation_details(&mut errors);

    if errors.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed(errors)
    }
}

fn reject_ui_presentation_details(errors: &mut Vec<String>) {
    let ui_dir = Path::new(PREFAB_CRATE).join("src/ui");

    for file in rust_files(ui_dir) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        if !contains_ui_presentation_detail(&source) {
            continue;
        }

        for forbidden in UI_PRESENTATION_DETAILS {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; UI visual presentation belongs in render_2d/src/ui, while prefab should only compose render bundles and semantic actions",
                    file.display()
                ));
            }
        }
    }
}

const UI_PRESENTATION_DETAILS: &[&str] = &[
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

fn contains_ui_presentation_detail(source: &str) -> bool {
    UI_PRESENTATION_DETAILS
        .iter()
        .any(|forbidden| source.contains(forbidden))
}

fn reject_nested_prefab_spawn(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(PREFAB_CRATE).join("src")) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        if contains_nested_prefab_spawn(&source) {
            errors.push(format!(
                "{} calls another prefab's `spawn` from inside prefab code; compose multiple prefab instances from gameplay instead",
                file.display()
            ));
        }
    }
}

fn contains_nested_prefab_spawn(source: &str) -> bool {
    source.contains(".spawn(commands)") || source.contains(".spawn(&mut commands)")
}

fn reject_forbidden_dependencies(errors: &mut Vec<String>) {
    reject_workspace_dependencies(
        PREFAB_CRATE,
        &["external_runtime", "intent", "gameplay"],
        errors,
        "prefab should stay an object template library, so keep timing/control decisions in gameplay or external_runtime",
    );
}

fn reject_direct_input(errors: &mut Vec<String>) {
    for file in rust_files(Path::new(PREFAB_CRATE)) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for forbidden in ["ButtonInput<", "KeyCode", "MouseButton", "Gamepad"] {
            if source.contains(forbidden) {
                errors.push(format!(
                    "{} references `{forbidden}`; external sources belong in external_runtime, so prefab only composes object data",
                    file.display()
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{contains_nested_prefab_spawn, contains_ui_presentation_detail};

    #[test]
    fn detects_prefab_spawn_with_commands_reference() {
        let source = "let camera = UiCameraPrefab.spawn(commands);";

        assert!(contains_nested_prefab_spawn(source));
    }

    #[test]
    fn detects_prefab_spawn_with_mut_commands_reference() {
        let source = "DemoMenuPrefab { ui_camera }.spawn(&mut commands);";

        assert!(contains_nested_prefab_spawn(source));
    }

    #[test]
    fn allows_bevy_commands_spawn() {
        let source = "commands.spawn((Camera2d, UiCamera));";

        assert!(!contains_nested_prefab_spawn(source));
    }

    #[test]
    fn detects_ui_visual_presentation_details() {
        let source = "TextFont { font_size: 22.0, ..default() }, TextColor(Color::WHITE)";

        assert!(contains_ui_presentation_detail(source));
    }

    #[test]
    fn allows_ui_semantic_actions() {
        let source = "Button, InteractionAction::new(action), menu_button_bundle(label)";

        assert!(!contains_ui_presentation_detail(source));
    }
}
