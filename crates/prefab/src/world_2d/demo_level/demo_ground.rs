//! Demo tilemap 地面 prefab。

use bevy::prelude::*;
use render_2d::tilemap::DemoTilemapLayer2d;

use crate::Prefab;

use super::demo_layout::{DEMO_GROUND, DEMO_GROUND_HEIGHT, DEMO_GROUND_WIDTH};

const DEMO_TILE_SIZE: u32 = 32;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoGroundRoot;

pub struct DemoGroundPrefab {
    tileset: Handle<Image>,
}

impl DemoGroundPrefab {
    pub fn new(tileset: Handle<Image>) -> Self {
        Self { tileset }
    }
}

impl Prefab for DemoGroundPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        let tile_indices = DEMO_GROUND.into_iter().flatten();

        commands
            .spawn((
                DemoGroundRoot,
                DemoTilemapLayer2d::new(
                    UVec2::new(DEMO_GROUND_WIDTH as u32, DEMO_GROUND_HEIGHT as u32),
                    UVec2::splat(DEMO_TILE_SIZE),
                    self.tileset,
                    tile_indices,
                ),
            ))
            .id()
    }
}
