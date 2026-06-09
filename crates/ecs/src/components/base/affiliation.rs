use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Faction(pub u32);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Team(pub u32);
