use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GameplayEntity;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GameplaySessionEntity;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct GameplayEntityId(pub u64);
