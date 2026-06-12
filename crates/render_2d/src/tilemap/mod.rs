pub mod demo_tilemap;
pub mod example;

use bevy::prelude::*;

pub use demo_tilemap::{DemoTilemapLayer2d, DemoTilemapLayer2dBundle};

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, _app: &mut App) {}
}
