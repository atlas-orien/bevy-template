use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Item {
    pub id: String,
}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, _app: &mut App) {}
}
