use crate::rules::base::camera::CameraRules;
use crate::rules::base::frame_animation::FrameAnimationRules;
use crate::rules::base::profiles::{Render2dRules, check_render_2d};
use crate::rules::base::skeletal_animation::SkeletalAnimationRules;
use crate::rules::{CheckStatus, finish};

const RENDER_2D_CRATE: &str = "crates/render_2d";
const RENDER_2D_PROTOCOL: &str = "AI_PROTOCOL/RENDER_2D.md";

const CONTENT_DIRS: &[&str] = &[
    "crates/render_2d/src/animation",
    "crates/render_2d/src/animation/frame",
    "crates/render_2d/src/animation/skeletal",
    "crates/render_2d/src/atlases",
    "crates/render_2d/src/background",
    "crates/render_2d/src/camera",
    "crates/render_2d/src/characters",
    "crates/render_2d/src/debug",
    "crates/render_2d/src/effects",
    "crates/render_2d/src/environment",
    "crates/render_2d/src/images",
    "crates/render_2d/src/items",
    "crates/render_2d/src/lighting",
    "crates/render_2d/src/materials",
    "crates/render_2d/src/mesh",
    "crates/render_2d/src/overlays",
    "crates/render_2d/src/particles",
    "crates/render_2d/src/pixel",
    "crates/render_2d/src/props",
    "crates/render_2d/src/screens",
    "crates/render_2d/src/text",
    "crates/render_2d/src/tilemap",
    "crates/render_2d/src/transitions",
    "crates/render_2d/src/ui",
];

const OBSOLETE_PATHS: &[&str] = &[
    "crates/render_2d/src/appearance",
    "crates/render_2d/src/appearance/color.rs",
    "crates/render_2d/src/appearance/opacity.rs",
    "crates/render_2d/src/appearance/visibility.rs",
    "crates/render_2d/src/geometry",
    "crates/render_2d/src/geometry/anchor.rs",
    "crates/render_2d/src/geometry/color.rs",
    "crates/render_2d/src/geometry/opacity.rs",
    "crates/render_2d/src/geometry/shape.rs",
    "crates/render_2d/src/geometry/size.rs",
    "crates/render_2d/src/geometry/visibility.rs",
    "crates/render_2d/src/geometry/offset.rs",
    "crates/render_2d/src/geometry/scale.rs",
    "crates/render_2d/src/geometry/rotation.rs",
    "crates/render_2d/src/geometry/z_index.rs",
    "crates/render_2d/src/geometry/flip.rs",
    "crates/render_2d/src/ordering",
    "crates/render_2d/src/ordering/z_index.rs",
    "crates/render_2d/src/sprite",
    "crates/render_2d/src/sprite/flip.rs",
    "crates/render_2d/src/transform",
    "crates/render_2d/src/transform/offset.rs",
    "crates/render_2d/src/transform/rotation.rs",
    "crates/render_2d/src/transform/scale.rs",
];

const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "external_runtime",
    "audio",
    "intent",
    "prefab",
    "physics",
    "render_3d",
    "network",
    "msrt-udp",
];

const WORLD_RULE_TERMS: &[&str] = &[
    "set_movement_intent",
    "PhysicsRigidBody",
    "PhysicsCollider",
    "Hitbox",
    "Hurtbox",
    "Combo",
    "SkillWindow",
    "AttackWindow",
];

const HARDCODED_SPRITE_SHEET_TERMS: &[&str] = &[
    "TextureAtlasLayout::from_grid",
    "ImageArrayLayout::RowCount",
    "ImageArrayLayout::ColumnCount",
];

const FRAME_ANIMATION_ALLOWED_SUBDIRS: &[&str] = &[];

const FRAME_ANIMATION_FORBIDDEN_FILE_NAMES: &[&str] =
    &["base.rs", "content.rs", "demo.rs", "example.rs"];

const CAMERA_ROOT_REQUIRED_FILES: &[&str] = &["mod.rs", "base.rs", "markers.rs", "plugin.rs"];

const CAMERA_ROOT_ALLOWED_FILES: &[&str] = &["mod.rs", "base.rs", "markers.rs", "plugin.rs"];

const CAMERA_ROOT_ALLOWED_DIRS: &[&str] = &["presets"];

const CAMERA_PRESETS_REQUIRED_FILES: &[&str] = &["mod.rs", "fixed.rs", "follow.rs", "ui.rs"];

const CAMERA_PRESETS_ALLOWED_FILES: &[&str] = &["mod.rs", "fixed.rs", "follow.rs", "ui.rs"];

const SKELETAL_PRODUCT_REQUIRED_FILES: &[&str] = &["mod.rs", "entry.rs", "systems.rs", "tests.rs"];

const SKELETAL_PRODUCT_ALLOWED_FILES: &[&str] = &["mod.rs", "entry.rs", "systems.rs", "tests.rs"];

const SKELETAL_RIG_REQUIRED_FILES: &[&str] = &[
    "mod.rs",
    "structure.rs",
    "parts.rs",
    "bundles.rs",
    "layout.rs",
];

const SKELETAL_FORBIDDEN_FILE_NAMES: &[&str] = &["demo_skeletal_animation.rs", "example.rs"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_render_2d(
        Render2dRules {
            crate_path: RENDER_2D_CRATE,
            protocol_path: RENDER_2D_PROTOCOL,
            content_dirs: CONTENT_DIRS,
            obsolete_paths: OBSOLETE_PATHS,
            forbidden_dependencies: FORBIDDEN_DEPENDENCIES,
            world_rule_terms: WORLD_RULE_TERMS,
            hardcoded_sprite_sheet_terms: HARDCODED_SPRITE_SHEET_TERMS,
            camera: CameraRules {
                camera_dir: "crates/render_2d/src/camera",
                root_required_files: CAMERA_ROOT_REQUIRED_FILES,
                root_allowed_files: CAMERA_ROOT_ALLOWED_FILES,
                root_allowed_dirs: CAMERA_ROOT_ALLOWED_DIRS,
                presets_required_files: CAMERA_PRESETS_REQUIRED_FILES,
                presets_allowed_files: CAMERA_PRESETS_ALLOWED_FILES,
            },
            frame_animation: FrameAnimationRules {
                frame_dir: "crates/render_2d/src/animation/frame",
                forbidden_subdirs: FRAME_ANIMATION_ALLOWED_SUBDIRS,
                forbidden_file_names: FRAME_ANIMATION_FORBIDDEN_FILE_NAMES,
                hardcoded_sheet_terms: HARDCODED_SPRITE_SHEET_TERMS,
            },
            skeletal_animation: SkeletalAnimationRules {
                skeletal_dir: "crates/render_2d/src/animation/skeletal",
                product_required_files: SKELETAL_PRODUCT_REQUIRED_FILES,
                product_allowed_files: SKELETAL_PRODUCT_ALLOWED_FILES,
                rig_required_files: SKELETAL_RIG_REQUIRED_FILES,
                forbidden_file_names: SKELETAL_FORBIDDEN_FILE_NAMES,
            },
        },
        &mut errors,
    );
    finish(errors)
}
