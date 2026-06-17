use std::path::Path;

use crate::rules::base::atlases::{AtlasesRules, check_atlases};
use crate::rules::base::camera::{CameraRules, check_camera};
use crate::rules::base::derives::check_component_marker_names;
use crate::rules::base::frame_animation::{FrameAnimationRules, check_frame_animation};
use crate::rules::base::functions::reject_free_functions_returning_any;
use crate::rules::base::paths::{reject_paths, require_mod_rs_under_src};
use crate::rules::base::skeletal_animation::{SkeletalAnimationRules, check_skeletal_animation};
use crate::rules::base::source::{
    reject_direct_input_access, reject_path_suffixes_in_rust_files, reject_terms_in_rust_files,
    reject_type_paths_in_rust_files,
};
use crate::rules::base::tilemap::{TilemapRules, check_tilemap};
use crate::rules::base::visual_primitives::{ImagesRules, TextRules, check_images, check_text};

pub struct Render3dRules<'a> {
    pub crate_path: &'a str,
    pub world_rule_terms: &'a [&'a str],
}

pub fn check_render_3d(rules: Render3dRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    check_component_marker_names(rules.crate_path, errors);
    reject_direct_input_access(
        rules.crate_path,
        errors,
        "external sources belong in peripherals/external_runtime, so render_3d should read presentation state only",
    );
    reject_terms_in_rust_files(
        rules.crate_path,
        rules.world_rule_terms,
        errors,
        "render_3d should not drive gameplay rules, so move the rule to gameplay/ecs/physics",
    );
}

pub struct Render2dRules<'a> {
    pub crate_path: &'a str,
    pub obsolete_paths: &'a [&'a str],
    pub world_rule_terms: &'a [&'a str],
    pub hardcoded_sprite_sheet_paths: &'a [&'a [&'a str]],
    pub atlases: AtlasesRules<'a>,
    pub camera: CameraRules,
    pub images: ImagesRules<'a>,
    pub text: TextRules<'a>,
    pub tilemap: TilemapRules<'a>,
    pub frame_animation: FrameAnimationRules<'a>,
    pub skeletal_animation: SkeletalAnimationRules<'a>,
}

pub fn check_render_2d(rules: Render2dRules<'_>, errors: &mut Vec<String>) {
    require_mod_rs_under_src(rules.crate_path, errors);
    check_component_marker_names(rules.crate_path, errors);
    reject_paths(
        rules.obsolete_paths,
        errors,
        "render_2d should not recreate Bevy facade directories; put concrete game presentation code in the content category directories",
    );
    reject_direct_input_access(
        Path::new(rules.crate_path).join("src"),
        errors,
        "external sources belong in peripherals/external_runtime, so render_2d should read presentation state only",
    );
    reject_terms_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        rules.world_rule_terms,
        errors,
        "render_2d should not drive gameplay rules, so move the rule to gameplay/ecs/physics",
    );
    reject_path_suffixes_in_rust_files(
        Path::new(rules.crate_path).join("src"),
        rules.hardcoded_sprite_sheet_paths,
        errors,
        "render_2d must load sprite sheet layout and frame clips from .frames.ron assets, not hardcode concrete sheet slicing in Rust",
    );
    check_frame_animation(rules.frame_animation, errors);
    check_atlases(rules.atlases, errors);
    check_camera(rules.camera, errors);
    check_images(rules.images, errors);
    check_text(rules.text, errors);
    check_tilemap(rules.tilemap, errors);
    check_skeletal_animation(rules.skeletal_animation, errors);
    reject_free_functions_returning_any(
        Path::new(rules.crate_path).join("src/products/ui"),
        &["-> impl Bundle", "-> Node"],
        errors,
        "exposes UI presentation as free functions returning `Node` or `impl Bundle`; render_2d/src/products/ui should define named Component/Bundle structs for reusable UI presentation",
    );
    reject_type_paths_in_rust_files(
        Path::new(rules.crate_path).join("src/products"),
        &["UiCameraTarget"],
        errors,
        "render_2d should expose static camera components/bundles only, while runtime camera-to-UI binding belongs in gameplay spawn code",
    );
    reject_paths(
        &[
            "crates/render_2d/src/ui/camera.rs",
            "crates/render_2d/src/ui/menu.rs",
            "crates/render_2d/src/products/ui/camera.rs",
            "crates/render_2d/src/products/ui/menu.rs",
        ],
        errors,
        "UI camera/menu demo files must use explicit architecture locations and demo names",
    );
}
