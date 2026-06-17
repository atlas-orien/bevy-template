use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GameplayEntityMarker;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GameplaySessionEntityMarker;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct GameplayEntityId(pub u64);
