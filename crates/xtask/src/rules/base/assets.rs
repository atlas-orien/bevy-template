use std::path::Path;

use crate::rules::util::files_with_extension;

pub fn require_animated_2d_frame_manifests(root: impl AsRef<Path>, errors: &mut Vec<String>) {
    for image in files_with_extension(root, "png") {
        let Some(stem) = image.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        let manifest = image.with_file_name(format!("{stem}.frames.ron"));

        if !manifest.exists() {
            errors.push(format!(
                "{} is an animated 2D sprite sheet without {}; assets/2d/animated resources must ship a matching frame manifest",
                image.display(),
                manifest.display()
            ));
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
            "bevy-template-xtask-assets-test-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("temp dir should be created");
        dir
    }

    #[test]
    fn animated_png_requires_matching_frames_ron() {
        let root = temp_rule_dir();
        let asset_dir = root.join("characters/demo-player");
        fs::create_dir_all(&asset_dir).expect("asset dir should be created");
        fs::write(asset_dir.join("demo-player.png"), []).expect("png should be written");

        let mut errors = Vec::new();
        require_animated_2d_frame_manifests(&root, &mut errors);

        let _ = fs::remove_dir_all(root);

        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("demo-player.frames.ron"));
    }

    #[test]
    fn animated_png_with_matching_frames_ron_passes() {
        let root = temp_rule_dir();
        let asset_dir = root.join("characters/demo-player");
        fs::create_dir_all(&asset_dir).expect("asset dir should be created");
        fs::write(asset_dir.join("demo-player.png"), []).expect("png should be written");
        fs::write(asset_dir.join("demo-player.frames.ron"), [])
            .expect("manifest should be written");

        let mut errors = Vec::new();
        require_animated_2d_frame_manifests(&root, &mut errors);

        let _ = fs::remove_dir_all(root);

        assert!(errors.is_empty());
    }
}
