use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy)]
pub struct LocalPlayerControlled;

#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerSpeed(pub f32);

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MovementIntent {
    pub target: MovementTarget,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum MovementTarget {
    #[default]
    None,
    Direction(Vec2),
    Position(Vec2),
}

impl MovementIntent {
    pub fn is_moving(&self) -> bool {
        !matches!(self.target, MovementTarget::None)
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub local_player_controlled: LocalPlayerControlled,
    pub speed: PlayerSpeed,
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

impl Default for PlayerSpeed {
    fn default() -> Self {
        Self(180.0)
    }
}
