use bevy::prelude::*;
use ecs::components::{
    base::AudioClips,
    world::{
        DemoSensorZone,
        gameplay::{GameplayEntity, GameplayEntityId, GameplaySessionEntity},
    },
};
use physics::{
    PhysicsActiveCollisionTypes, PhysicsActiveEvents, PhysicsCollider2d, PhysicsRigidBody,
    PhysicsSensor,
};
use render_2d::props::DemoSensorZone2dBundle;

use crate::Prefab;

pub const DEMO_SENSOR_ENTITY_ID: GameplayEntityId = GameplayEntityId(2);

pub struct DemoSensorZonePrefab {
    position: Vec2,
}

impl DemoSensorZonePrefab {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl Prefab for DemoSensorZonePrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                DemoSensorZone,
                GameplayEntity,
                GameplaySessionEntity,
                DEMO_SENSOR_ENTITY_ID,
                AudioClips::default().with_interact("audio/demo_pickup.ogg"),
                PhysicsRigidBody::Static,
                PhysicsCollider2d::Rectangle {
                    width: 72.0,
                    height: 44.0,
                },
                PhysicsSensor,
                PhysicsActiveEvents {
                    collision: true,
                    contact_force: false,
                },
                PhysicsActiveCollisionTypes {
                    dynamic_dynamic: true,
                    dynamic_kinematic: true,
                    dynamic_static: true,
                    kinematic_kinematic: true,
                    kinematic_static: true,
                    static_static: true,
                },
                Transform::from_xyz(self.position.x, self.position.y, 1.5),
                Visibility::default(),
                children![DemoSensorZone2dBundle::default()],
            ))
            .id()
    }
}
