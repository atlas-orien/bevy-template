use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub transform: Transform,
    pub visibility: Visibility,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}

impl Default for Player {
    fn default() -> Self {
        Self
    }
}
