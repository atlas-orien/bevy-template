use bevy::prelude::*;
use prefab::Prefab;

pub trait SpawnItem {
    fn spawn_boxed(self: Box<Self>, commands: &mut Commands) -> Entity;
}

impl<P> SpawnItem for P
where
    P: Prefab,
{
    fn spawn_boxed(self: Box<Self>, commands: &mut Commands) -> Entity {
        (*self).spawn(commands)
    }
}
