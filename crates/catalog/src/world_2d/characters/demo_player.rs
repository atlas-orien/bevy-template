use bevy::prelude::*;
use prefab::world_2d::characters::DemoPlayerPrefab;
use render_2d::primitives::frame_animation::FrameAnimationManifest2d;

const DEMO_PLAYER_FOOTSTEP_AUDIO: &str = "audio/demo_footstep.ogg";
const DEMO_PLAYER_FRAMES: &str =
    "2d/manifests/frames/characters/demo-player/demo-player.frames.ron";

pub struct DemoPlayer {
    position: Vec2,
}

impl DemoPlayer {
    pub fn at(position: Vec2) -> Self {
        Self { position }
    }

    pub fn prefab(self, asset_server: &AssetServer) -> DemoPlayerPrefab {
        DemoPlayerPrefab::new(
            self.position,
            asset_server.load::<FrameAnimationManifest2d>(DEMO_PLAYER_FRAMES),
            DEMO_PLAYER_FOOTSTEP_AUDIO,
        )
    }
}
