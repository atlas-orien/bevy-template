use bevy::prelude::*;
use helper::assets::GltfAsset;
use prefab::world_3d::DemoFox3dPrefab;
use render_3d::capabilities::animation::demo::{DemoFox3dAnimationSet, DemoFox3dAnimationState};
use render_3d::products::characters::DemoFox3d as DemoFox3dVisual;

const DEMO_FOX_MODEL: GltfAsset = GltfAsset::new("3d/models/demo-fox/fox.glb");

pub struct DemoFox3d;

impl DemoFox3d {
    pub fn animations(
        asset_server: &AssetServer,
        animation_graphs: &mut Assets<AnimationGraph>,
    ) -> DemoFox3dAnimationSet {
        DemoFox3dAnimationSet::from_gltf_model(asset_server, animation_graphs, DEMO_FOX_MODEL)
    }

    pub fn scene(asset_server: &AssetServer) -> Handle<Scene> {
        DEMO_FOX_MODEL.load_scene(asset_server, 0)
    }

    pub fn prefab_with_scene(
        asset_server: &AssetServer,
        initial_state: DemoFox3dAnimationState,
        transform: Transform,
        animations: DemoFox3dAnimationSet,
    ) -> DemoFox3dPrefab {
        DemoFox3dPrefab::new(DemoFox3dVisual::new(
            Self::scene(asset_server),
            animations,
            initial_state,
            transform.with_scale(Vec3::splat(0.02)),
        ))
    }
}
