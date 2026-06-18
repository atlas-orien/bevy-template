use std::path::Path;

use crate::rules::base::source::{
    reject_path_suffixes_in_rust_files, reject_string_literals_containing,
};
use crate::rules::util::{parse_rust_file, rust_files};

pub struct FrameAnimationRules<'a> {
    pub frame_dir: &'a str,
    pub hardcoded_sheet_paths: &'a [&'a [&'a str]],
}

pub fn check_frame_animation(rules: FrameAnimationRules<'_>, errors: &mut Vec<String>) {
    let frame_dir = Path::new(rules.frame_dir);
    reject_path_suffixes_in_rust_files(
        frame_dir,
        rules.hardcoded_sheet_paths,
        errors,
        "frame animation sheet layout, clips, fps and repeat data must come from .frames.ron manifests",
    );
    reject_string_literals_containing(
        frame_dir,
        &[".png", ".jpg", ".jpeg"],
        errors,
        "frame_animation must not bind concrete image paths; catalog loads manifests and concrete render products receive handles",
    );
    reject_frame_demo_public_api(frame_dir, errors);
}

fn reject_frame_demo_public_api(frame_dir: &Path, errors: &mut Vec<String>) {
    for file in rust_files(frame_dir) {
        let Some(parsed) = parse_rust_file(&file, errors) else {
            continue;
        };

        for item in parsed.items {
            if public_item_name(&item).is_some_and(|name| name.starts_with("Demo")) {
                errors.push(format!(
                    "{} exposes demo API from frame_animation; frame animation should expose generic animation/resource/system types only",
                    file.display()
                ));
                break;
            }
        }
    }
}

fn public_item_name(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Const(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
            Some(item.ident.to_string())
        }
        syn::Item::Enum(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
            Some(item.ident.to_string())
        }
        syn::Item::Fn(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
            Some(item.sig.ident.to_string())
        }
        syn::Item::Struct(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
            Some(item.ident.to_string())
        }
        syn::Item::Trait(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
            Some(item.ident.to_string())
        }
        syn::Item::Type(item) if matches!(item.vis, syn::Visibility::Public(_)) => {
            Some(item.ident.to_string())
        }
        _ => None,
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
            hardcoded_sheet_paths: &[&["TextureAtlasLayout", "from_grid"]],
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
            r#"fn load(asset_server: AssetServer) { asset_server.load::<Image>("2d/static/player.png"); }"#,
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
