//! Demo tile 地面层 bundle，基于 bevy 内置 TilemapChunk。

use bevy::{
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

const DEMO_TILEMAP_ORIGIN: Vec3 = Vec3::new(-1536.0, -192.0, 0.0);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoTilemapLayer2d;

#[derive(Bundle)]
pub struct DemoTilemapLayer2dBundle {
    pub marker: DemoTilemapLayer2d,
    pub chunk: TilemapChunk,
    pub tiles: TilemapChunkTileData,
    pub transform: Transform,
}

impl DemoTilemapLayer2dBundle {
    pub fn new(
        chunk_size: UVec2,
        tile_display_size: UVec2,
        tileset: Handle<Image>,
        tile_indices: impl IntoIterator<Item = Option<u16>>,
    ) -> Self {
        let tile_data = tile_indices
            .into_iter()
            .map(|index| index.map(TileData::from_tileset_index))
            .collect();

        Self {
            marker: DemoTilemapLayer2d,
            chunk: TilemapChunk {
                chunk_size,
                tile_display_size,
                tileset,
                ..default()
            },
            tiles: TilemapChunkTileData(tile_data),
            transform: Transform::from_translation(DEMO_TILEMAP_ORIGIN),
        }
    }
}
