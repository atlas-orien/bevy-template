use std::path::Path;

use crate::rules::base::paths::{reject_file_names, reject_subdirs_except};
use crate::rules::base::source::{reject_lines_containing_all_terms, reject_terms_in_rust_files};
use crate::rules::util::{read_file_if_exists, rust_files};

pub struct FrameAnimationRules<'a> {
    pub frame_dir: &'a str,
    pub forbidden_subdirs: &'a [&'a str],
    pub forbidden_file_names: &'a [&'a str],
    pub hardcoded_sheet_terms: &'a [&'a str],
}

pub fn check_frame_animation(rules: FrameAnimationRules<'_>, errors: &mut Vec<String>) {
    let frame_dir = Path::new(rules.frame_dir);
    reject_subdirs_except(
        frame_dir,
        rules.forbidden_subdirs,
        errors,
        "animation/frame is generic frame animation infrastructure; do not create demo/content/base subdomains here",
    );
    reject_file_names(
        frame_dir,
        rules.forbidden_file_names,
        errors,
        "animation/frame should stay generic; concrete animated characters or demo products belong in semantic render_2d directories such as characters",
    );
    reject_terms_in_rust_files(
        frame_dir,
        rules.hardcoded_sheet_terms,
        errors,
        "frame animation sheet layout, clips, fps and repeat data must come from .frames.ron manifests",
    );
    reject_lines_containing_all_terms(
        frame_dir,
        &["load::<Image>", "\""],
        errors,
        "animation/frame must not bind concrete image paths; catalog loads manifests and concrete render products receive handles",
    );
    reject_frame_demo_public_api(frame_dir, errors);
}

fn reject_frame_demo_public_api(frame_dir: &Path, errors: &mut Vec<String>) {
    for file in rust_files(frame_dir) {
        let Some(source) = read_file_if_exists(&file) else {
            continue;
        };

        for line in source.lines().map(str::trim) {
            if !line.starts_with("pub ") {
                continue;
            }

            if line.contains("Demo") || line.contains("demo_") {
                errors.push(format!(
                    "{} exposes demo API from animation/frame; frame animation should expose generic animation/resource/system types only",
                    file.display()
                ));
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_rule_dir() -> PathBuf {
        let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "bevy-template-xtask-frame-animation-test-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("temp dir should be created");
        dir
    }

    fn rules<'a>(frame_dir: &'a str) -> FrameAnimationRules<'a> {
        FrameAnimationRules {
            frame_dir,
            forbidden_subdirs: &[],
            forbidden_file_names: &["demo.rs", "content.rs"],
            hardcoded_sheet_terms: &["TextureAtlasLayout::from_grid"],
        }
    }

    #[test]
    fn flags_demo_api_in_frame_animation() {
        let root = temp_rule_dir();
        fs::write(
            root.join("animation.rs"),
            "pub struct DemoPlayerAnimation2d;\n",
        )
        .expect("source should be written");

        let mut errors = Vec::new();
        check_frame_animation(rules(root.to_str().unwrap()), &mut errors);

        let _ = fs::remove_dir_all(root);

        assert!(errors.iter().any(|error| error.contains("demo API")));
    }

    #[test]
    fn flags_concrete_image_loading_in_frame_animation() {
        let root = temp_rule_dir();
        fs::write(
            root.join("manifest.rs"),
            r#"asset_server.load::<Image>("2d/animated/player.png");"#,
        )
        .expect("source should be written");

        let mut errors = Vec::new();
        check_frame_animation(rules(root.to_str().unwrap()), &mut errors);

        let _ = fs::remove_dir_all(root);

        assert!(errors.iter().any(|error| error.contains("must not bind")));
    }

    #[test]
    fn allows_generic_frame_animation_api() {
        let root = temp_rule_dir();
        fs::write(
            root.join("animation.rs"),
            "pub struct FrameAnimation2d;\nfn frame_animation_system() {}\n",
        )
        .expect("source should be written");

        let mut errors = Vec::new();
        check_frame_animation(rules(root.to_str().unwrap()), &mut errors);

        let _ = fs::remove_dir_all(root);

        assert!(errors.is_empty());
    }
}
