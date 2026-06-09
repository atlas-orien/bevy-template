use bevy::prelude::*;

pub trait Prefab {
    fn spawn(self, commands: &mut Commands) -> Entity;
}
