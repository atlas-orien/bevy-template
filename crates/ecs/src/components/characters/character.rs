use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct CharacterMarker;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerControlledMarker;

impl Default for CharacterMarker {
    fn default() -> Self {
        Self
    }
}
