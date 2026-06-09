use bevy::prelude::*;

use crate::components::base::{Facing, MovementIntent, Speed};

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy)]
pub struct LocalPlayerControlled;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub local_player_controlled: LocalPlayerControlled,
    pub speed: Speed,
    pub movement_intent: MovementIntent,
    pub facing: Facing,
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

impl Default for LocalPlayerControlled {
    fn default() -> Self {
        Self
    }
}
