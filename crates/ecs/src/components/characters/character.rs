use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Character;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerControlled;

impl Default for Character {
    fn default() -> Self {
        Self
    }
}
