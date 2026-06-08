use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct SceneEntity;

pub fn despawn_scene_entities(mut commands: Commands, entities: Query<Entity, With<SceneEntity>>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
