use std::path::Path;

use crate::rules::base::paths::{reject_files_under_dir_except, reject_subdirs_except};
use crate::rules::util::require_path;

pub struct CameraRules<'a> {
    pub camera_dir: &'a str,
    pub root_required_files: &'a [&'a str],
    pub root_allowed_files: &'a [&'a str],
    pub root_allowed_dirs: &'a [&'a str],
    pub presets_required_files: &'a [&'a str],
    pub presets_allowed_files: &'a [&'a str],
}

pub fn check_camera(rules: CameraRules<'_>, errors: &mut Vec<String>) {
    let camera_dir = Path::new(rules.camera_dir);
    require_path(
        camera_dir,
        errors,
        "render_2d camera must stay grouped under primitives/camera",
    );

    for file_name in rules.root_required_files {
        require_path(
            camera_dir.join(file_name),
            errors,
            "camera root keeps shared base, plugin and module exports",
        );
    }

    reject_files_under_dir_except(
        camera_dir,
        rules.root_allowed_files,
        errors,
        "camera root should contain only shared infrastructure; concrete camera products belong under camera/presets",
    );
    reject_subdirs_except(
        camera_dir,
        rules.root_allowed_dirs,
        errors,
        "camera root should keep concrete camera products under camera/presets",
    );

    let presets_dir = camera_dir.join("presets");
    require_path(
        &presets_dir,
        errors,
        "camera/presets contains the concrete camera products available to gameplay",
    );
    for file_name in rules.presets_required_files {
        require_path(
            presets_dir.join(file_name),
            errors,
            "camera/presets must expose the first-party fixed, follow and UI camera products",
        );
    }

    reject_files_under_dir_except(
        &presets_dir,
        rules.presets_allowed_files,
        errors,
        "camera/presets should contain concrete camera product files only",
    );
    reject_subdirs_except(
        &presets_dir,
        &[],
        errors,
        "camera/presets products should stay as files until a product is large enough to justify a directory",
    );
}
