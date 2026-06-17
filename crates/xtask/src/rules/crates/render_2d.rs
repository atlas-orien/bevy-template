use crate::rules::base::atlases::AtlasesRules;
use crate::rules::base::camera::CameraRules;
use crate::rules::base::frame_animation::FrameAnimationRules;
use crate::rules::base::profiles::{Render2dRules, check_render_2d};
use crate::rules::base::skeletal_animation::SkeletalAnimationRules;
use crate::rules::base::tilemap::TilemapRules;
use crate::rules::base::visual_primitives::{ImagesRules, TextRules};
use crate::rules::{CheckStatus, finish};

const RENDER_2D_CRATE: &str = "crates/render_2d";

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
    "crates/render_2d/src/atlases",
    "crates/render_2d/src/background",
    "crates/render_2d/src/camera",
    "crates/render_2d/src/characters",
    "crates/render_2d/src/debug",
    "crates/render_2d/src/effects",
    "crates/render_2d/src/environment",
    "crates/render_2d/src/images",
    "crates/render_2d/src/items",
    "crates/render_2d/src/layers",
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
    "crates/render_2d/src/frame_animation",
    "crates/render_2d/src/skeletal_animation",
    "crates/render_2d/src/animation",
    "crates/render_2d/src/animation/frame",
    "crates/render_2d/src/animation/skeletal",
    "crates/render_2d/src/primitives/animation",
    "crates/render_2d/src/primitives/animation/frame",
    "crates/render_2d/src/capabilities/animation",
    "crates/render_2d/src/capabilities/animation/skeletal",
    "crates/render_2d/src/primitives/camera/markers.rs",
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

const HARDCODED_SPRITE_SHEET_PATHS: &[&[&str]] = &[
    &["TextureAtlasLayout", "from_grid"],
    &["ImageArrayLayout", "RowCount"],
    &["ImageArrayLayout", "ColumnCount"],
];

const ATLASES_FORBIDDEN_TERMS: &[&str] = &["AssetServer", "asset_server", ".load(", "Timer"];

const IMAGES_FORBIDDEN_TERMS: &[&str] = &["AssetServer", "asset_server", ".load(", "add_systems"];

const TEXT_FORBIDDEN_TERMS: &[&str] = &["AssetServer", "asset_server", ".load(", "Node"];

const TILEMAP_FORBIDDEN_TERMS: &[&str] = &["DemoTilemap", "demo_tilemap"];

pub fn check() -> CheckStatus {
    let mut errors = Vec::new();
    check_render_2d(
        Render2dRules {
            crate_path: RENDER_2D_CRATE,
            obsolete_paths: OBSOLETE_PATHS,
            world_rule_terms: WORLD_RULE_TERMS,
            hardcoded_sprite_sheet_paths: HARDCODED_SPRITE_SHEET_PATHS,
            atlases: AtlasesRules {
                atlases_dir: "crates/render_2d/src/primitives/atlases",
                forbidden_terms: ATLASES_FORBIDDEN_TERMS,
            },
            camera: CameraRules,
            images: ImagesRules {
                images_dir: "crates/render_2d/src/primitives/images",
                forbidden_terms: IMAGES_FORBIDDEN_TERMS,
            },
            text: TextRules {
                text_dir: "crates/render_2d/src/primitives/text",
                forbidden_terms: TEXT_FORBIDDEN_TERMS,
            },
            tilemap: TilemapRules {
                tilemap_dir: "crates/render_2d/src/primitives/tilemap",
                forbidden_terms: TILEMAP_FORBIDDEN_TERMS,
            },
            frame_animation: FrameAnimationRules {
                frame_dir: "crates/render_2d/src/primitives/frame_animation",
                hardcoded_sheet_paths: HARDCODED_SPRITE_SHEET_PATHS,
            },
            skeletal_animation: SkeletalAnimationRules {
                skeletal_dir: "crates/render_2d/src/capabilities/skeletal_animation",
            },
        },
        &mut errors,
    );
    finish(errors)
}
