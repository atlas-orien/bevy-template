use std::path::Path;

use crate::rules::base::paths::{reject_file_names, reject_files_under_dir_except};
use crate::rules::util::{require_path, subdirs};

pub struct SkeletalAnimationRules<'a> {
    pub skeletal_dir: &'a str,
    pub product_required_files: &'a [&'a str],
    pub product_allowed_files: &'a [&'a str],
    pub rig_required_files: &'a [&'a str],
    pub forbidden_file_names: &'a [&'a str],
}

pub fn check_skeletal_animation(rules: SkeletalAnimationRules<'_>, errors: &mut Vec<String>) {
    let skeletal_dir = Path::new(rules.skeletal_dir);
    reject_file_names(
        skeletal_dir,
        rules.forbidden_file_names,
        errors,
        "skeletal_animation products are large custom animations and must live in product directories, not single files",
    );

    for product_dir in immediate_subdirs(skeletal_dir) {
        let Some(product_name) = product_dir.file_name().and_then(|name| name.to_str()) else {
            continue;
        };

        if product_name == "rig" {
            errors.push(format!(
                "{} is not a skeletal product directory; rig belongs under a concrete product such as skeletal_animation/demo/rig",
                product_dir.display()
            ));
            continue;
        }

        for file_name in rules.product_required_files {
            require_path(
                product_dir.join(file_name),
                errors,
                "skeletal product directories must expose a clear entry, rig, systems and tests structure",
            );
        }

        reject_files_under_dir_except(
            &product_dir,
            rules.product_allowed_files,
            errors,
            "skeletal product root should stay small; move rig internals under rig/ and runtime logic into systems.rs",
        );

        let rig_dir = product_dir.join("rig");
        for file_name in rules.rig_required_files {
            require_path(
                rig_dir.join(file_name),
                errors,
                "skeletal rig directories should split structure, parts, bundles and layout so custom animations do not become giant rig files",
            );
        }
    }
}

fn immediate_subdirs(root: &Path) -> Vec<std::path::PathBuf> {
    subdirs(root)
        .into_iter()
        .filter(|dir| dir.parent().is_some_and(|parent| parent == root))
        .collect()
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
            "bevy-template-xtask-skeletal-animation-test-{}-{id}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("temp dir should be created");
        dir
    }

    fn rules<'a>(skeletal_dir: &'a str) -> SkeletalAnimationRules<'a> {
        SkeletalAnimationRules {
            skeletal_dir,
            product_required_files: &["mod.rs", "entry.rs", "systems.rs", "tests.rs"],
            product_allowed_files: &["mod.rs", "entry.rs", "systems.rs", "tests.rs"],
            rig_required_files: &[
                "mod.rs",
                "structure.rs",
                "parts.rs",
                "bundles.rs",
                "layout.rs",
            ],
            forbidden_file_names: &["demo_skeletal_animation.rs"],
        }
    }

    #[test]
    fn rejects_single_file_skeletal_product() {
        let root = temp_rule_dir();
        fs::write(root.join("demo_skeletal_animation.rs"), "").expect("source should be written");

        let mut errors = Vec::new();
        check_skeletal_animation(rules(root.to_str().unwrap()), &mut errors);

        let _ = fs::remove_dir_all(root);

        assert!(errors.iter().any(|error| {
            error.contains("demo_skeletal_animation.rs") && error.contains("single files")
        }));
    }

    #[test]
    fn requires_product_directory_shape() {
        let root = temp_rule_dir();
        fs::create_dir_all(root.join("demo/rig")).expect("product dir should be created");
        fs::write(root.join("demo/mod.rs"), "").expect("mod should be written");

        let mut errors = Vec::new();
        check_skeletal_animation(rules(root.to_str().unwrap()), &mut errors);

        let _ = fs::remove_dir_all(root);

        assert!(errors.iter().any(|error| error.contains("entry.rs")));
        assert!(errors.iter().any(|error| error.contains("structure.rs")));
    }

    #[test]
    fn allows_split_product_directory() {
        let root = temp_rule_dir();
        let demo = root.join("demo");
        let rig = demo.join("rig");
        fs::create_dir_all(&rig).expect("rig dir should be created");
        for file_name in ["mod.rs", "entry.rs", "systems.rs", "tests.rs"] {
            fs::write(demo.join(file_name), "").expect("product file should be written");
        }
        for file_name in [
            "mod.rs",
            "structure.rs",
            "parts.rs",
            "bundles.rs",
            "layout.rs",
        ] {
            fs::write(rig.join(file_name), "").expect("rig file should be written");
        }

        let mut errors = Vec::new();
        check_skeletal_animation(rules(root.to_str().unwrap()), &mut errors);

        let _ = fs::remove_dir_all(root);

        assert!(errors.is_empty());
    }
}
