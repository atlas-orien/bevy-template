use std::path::Path;

use crate::rules::util::subdirs;

pub struct SkeletalAnimationRules<'a> {
    pub skeletal_dir: &'a str,
}

pub fn check_skeletal_animation(rules: SkeletalAnimationRules<'_>, errors: &mut Vec<String>) {
    let skeletal_dir = Path::new(rules.skeletal_dir);
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
        SkeletalAnimationRules { skeletal_dir }
    }

    #[test]
    fn rejects_root_rig_directory() {
        let root = temp_rule_dir();
        fs::create_dir(root.join("rig")).expect("rig dir should be written");

        let mut errors = Vec::new();
        check_skeletal_animation(rules(root.to_str().unwrap()), &mut errors);

        let _ = fs::remove_dir_all(root);

        assert!(
            errors
                .iter()
                .any(|error| { error.contains("rig") && error.contains("concrete product") })
        );
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
