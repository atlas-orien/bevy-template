//! 通用 tilemap chunk layer bundle，基于 Bevy 内置 TilemapChunk。

use bevy::{
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::primitives::markers::TilemapChunkLayer2dMarker;

#[derive(Bundle)]
pub struct TilemapChunkLayer2d {
    marker: TilemapChunkLayer2dMarker,
    pub chunk: TilemapChunk,
    pub tiles: TilemapChunkTileData,
    pub transform: Transform,
}

impl TilemapChunkLayer2d {
    pub fn new(
        chunk_size: UVec2,
        tile_display_size: UVec2,
        tileset: Handle<Image>,
        tile_indices: impl IntoIterator<Item = Option<u16>>,
        translation: Vec3,
    ) -> Self {
        let tile_data = tile_indices
            .into_iter()
            .map(|index| index.map(TileData::from_tileset_index))
            .collect();

        Self {
            marker: TilemapChunkLayer2dMarker,
            chunk: TilemapChunk {
                chunk_size,
                tile_display_size,
                tileset,
                ..default()
            },
            tiles: TilemapChunkTileData(tile_data),
            transform: Transform::from_translation(translation),
        }
    }
}
