use bevy::{
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleTilemapLayer2d;

#[derive(Bundle)]
pub struct ExampleTilemapLayer2dBundle {
    pub marker: ExampleTilemapLayer2d,
    pub chunk: TilemapChunk,
    pub tiles: TilemapChunkTileData,
}

impl ExampleTilemapLayer2dBundle {
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
            marker: ExampleTilemapLayer2d,
            chunk: TilemapChunk {
                chunk_size,
                tile_display_size,
                tileset,
                ..default()
            },
            tiles: TilemapChunkTileData(tile_data),
        }
    }
}
