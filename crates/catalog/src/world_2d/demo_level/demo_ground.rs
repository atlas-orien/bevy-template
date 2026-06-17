use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
};
use prefab::world_2d::demo_level::DemoGroundPrefab;

const DEMO_TILESET_IMAGE: &str = "2d/static/tilemaps/demo_tileset.png";

pub struct DemoGround;

impl DemoGround {
    pub fn prefab(asset_server: &AssetServer) -> DemoGroundPrefab {
        DemoGroundPrefab::new(asset_server.load_with_settings(
            DEMO_TILESET_IMAGE,
            |settings: &mut ImageLoaderSettings| {
                settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
            },
        ))
    }
}
