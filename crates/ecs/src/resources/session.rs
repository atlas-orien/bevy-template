use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct GameSession {
    pub id: u64,
}
