//! Demo prefab catalog entries with default runtime resources.

use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
};
use prefab::world_2d::characters::DemoPlayerPrefab;
use prefab::world_2d::demo_level::{DemoGroundPrefab, DemoSkeletonPrefab};
use render_2d::primitives::frame_animation::FrameAnimationManifest2d;

pub const DEMO_PLAYER_FRAMES: &str = "2d/animated/characters/demo-player/demo-player.frames.ron";
pub const DEMO_TILESET_IMAGE: &str = "2d/static/tilemaps/demo_tileset.png";
pub const DEMO_SKELETON_BONE_IMAGE: &str =
    "2d/static/props/demo-skeletal-bone/demo-skeletal-bone.png";
pub const DEMO_SKELETON_JOINT_IMAGE: &str =
    "2d/static/props/demo-skeletal-joint/demo-skeletal-joint.png";

pub fn demo_player(position: Vec2, asset_server: &AssetServer) -> DemoPlayerPrefab {
    DemoPlayerPrefab::new(
        position,
        asset_server.load::<FrameAnimationManifest2d>(DEMO_PLAYER_FRAMES),
    )
}

pub fn demo_ground(asset_server: &AssetServer) -> DemoGroundPrefab {
    DemoGroundPrefab::new(asset_server.load_with_settings(
        DEMO_TILESET_IMAGE,
        |settings: &mut ImageLoaderSettings| {
            settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
        },
    ))
}

pub fn demo_skeleton(position: Vec2, asset_server: &AssetServer) -> DemoSkeletonPrefab {
    DemoSkeletonPrefab::new(
        position,
        asset_server.load(DEMO_SKELETON_BONE_IMAGE),
        asset_server.load(DEMO_SKELETON_JOINT_IMAGE),
    )
}
