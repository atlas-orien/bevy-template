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
use render_2d::props::{DEMO_SENSOR_ZONE_SIZE, DemoSensorZone2dBundle};

use crate::Prefab;

pub const DEMO_SENSOR_ENTITY_ID: GameplayEntityId = GameplayEntityId(2);
const DEMO_SENSOR_AUDIO: &str = "audio/demo_pickup.ogg";
const DEMO_SENSOR_Z: f32 = 1.5;

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
                AudioClips::default().with_interact(DEMO_SENSOR_AUDIO),
                PhysicsRigidBody::Static,
                PhysicsCollider2d::Rectangle {
                    width: DEMO_SENSOR_ZONE_SIZE.x,
                    height: DEMO_SENSOR_ZONE_SIZE.y,
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
                Transform::from_xyz(self.position.x, self.position.y, DEMO_SENSOR_Z),
                Visibility::default(),
                children![DemoSensorZone2dBundle::default()],
            ))
            .id()
    }
}
