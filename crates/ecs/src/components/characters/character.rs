use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Character;

impl Default for Character {
    fn default() -> Self {
        Self
    }
}
