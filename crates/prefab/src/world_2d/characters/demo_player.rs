use bevy::prelude::*;
use ecs::components::{
    base::{Facing, Health, MaxHealth, MovementIntent, Speed},
    characters::{Character, DemoPlayerControlled},
    world::gameplay::{GameplayEntity, GameplayEntityId, GameplaySessionEntity},
};
use render_2d::characters::DemoPlayerSprite2dBundle;

use crate::Prefab;

pub const DEMO_PLAYER_ENTITY_ID: GameplayEntityId = GameplayEntityId(1);

pub struct DemoPlayerPrefab {
    position: Vec2,
}

impl DemoPlayerPrefab {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl Prefab for DemoPlayerPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Character,
                DemoPlayerControlled,
                GameplayEntity,
                GameplaySessionEntity,
                DEMO_PLAYER_ENTITY_ID,
                MovementIntent::default(),
                Speed::default(),
                Facing::default(),
                Health(100.0),
                MaxHealth(100.0),
                Transform::from_xyz(self.position.x, self.position.y, 2.0),
                Visibility::default(),
                children![DemoPlayerSprite2dBundle::default()],
            ))
            .id()
    }
}
