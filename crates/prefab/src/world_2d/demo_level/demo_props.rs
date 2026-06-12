use bevy::prelude::*;
use ecs::components::world::gameplay::GameplaySessionEntity;
use render_2d::props::DemoRock2dBundle;

use crate::Prefab;

pub struct DemoRockPrefab {
    position: Vec2,
}

impl DemoRockPrefab {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl Prefab for DemoRockPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                GameplaySessionEntity,
                DemoRock2dBundle::new(Vec3::new(self.position.x, self.position.y, 1.0)),
            ))
            .id()
    }
}
