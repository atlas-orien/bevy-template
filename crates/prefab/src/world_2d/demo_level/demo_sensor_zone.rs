//! Demo 感应区 prefab：sensor collider 与 gameplay 身份组合。

use bevy::prelude::*;
use ecs::components::{
    base::AudioClips,
    world::{
        DemoSensorZoneMarker,
        gameplay::{GameplayEntityId, GameplayEntityMarker, GameplaySessionEntityMarker},
    },
};
use physics::{
    PhysicsActiveCollisionTypes, PhysicsActiveEvents, PhysicsCollider2d, PhysicsRigidBody,
    PhysicsSensorMarker,
};
use render_2d::products::props::{DEMO_SENSOR_ZONE_SIZE, DemoSensorZone2d};

use crate::Prefab;

pub const DEMO_SENSOR_ENTITY_ID: GameplayEntityId = GameplayEntityId(2);
const DEMO_SENSOR_Z: f32 = 1.5;

pub struct DemoSensorZonePrefab {
    position: Vec2,
    interact_audio: String,
}

#[derive(Bundle)]
struct DemoSensorIdentityBundle {
    zone: DemoSensorZoneMarker,
    gameplay: GameplayEntityMarker,
    session: GameplaySessionEntityMarker,
    gameplay_id: GameplayEntityId,
}

impl Default for DemoSensorIdentityBundle {
    fn default() -> Self {
        Self {
            zone: DemoSensorZoneMarker,
            gameplay: GameplayEntityMarker,
            session: GameplaySessionEntityMarker,
            gameplay_id: DEMO_SENSOR_ENTITY_ID,
        }
    }
}

#[derive(Bundle, Default)]
struct DemoSensorAudioBundle {
    clips: AudioClips,
}

impl DemoSensorAudioBundle {
    fn new(interact_audio: impl Into<String>) -> Self {
        Self {
            clips: AudioClips::default().with_interact(interact_audio),
        }
    }
}

#[derive(Bundle)]
struct DemoSensorPhysicsBundle {
    rigid_body: PhysicsRigidBody,
    collider: PhysicsCollider2d,
    sensor: PhysicsSensorMarker,
    active_events: PhysicsActiveEvents,
    active_collision_types: PhysicsActiveCollisionTypes,
}

impl Default for DemoSensorPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: PhysicsRigidBody::Static,
            collider: PhysicsCollider2d::Rectangle {
                width: DEMO_SENSOR_ZONE_SIZE.x,
                height: DEMO_SENSOR_ZONE_SIZE.y,
            },
            sensor: PhysicsSensorMarker,
            active_events: PhysicsActiveEvents {
                collision: true,
                contact_force: false,
            },
            active_collision_types: PhysicsActiveCollisionTypes {
                dynamic_dynamic: true,
                dynamic_kinematic: true,
                dynamic_static: true,
                kinematic_kinematic: true,
                kinematic_static: true,
                static_static: true,
            },
        }
    }
}

#[derive(Bundle)]
struct DemoSensorZoneBundle {
    identity: DemoSensorIdentityBundle,
    audio: DemoSensorAudioBundle,
    physics: DemoSensorPhysicsBundle,
    transform: Transform,
    visibility: Visibility,
}

impl DemoSensorZoneBundle {
    fn new(position: Vec2, interact_audio: impl Into<String>) -> Self {
        Self {
            identity: DemoSensorIdentityBundle::default(),
            audio: DemoSensorAudioBundle::new(interact_audio),
            physics: DemoSensorPhysicsBundle::default(),
            transform: Transform::from_xyz(position.x, position.y, DEMO_SENSOR_Z),
            visibility: Visibility::default(),
        }
    }
}

impl DemoSensorZonePrefab {
    pub fn new(position: Vec2, interact_audio: impl Into<String>) -> Self {
        Self {
            position,
            interact_audio: interact_audio.into(),
        }
    }
}

impl Prefab for DemoSensorZonePrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoSensorZoneBundle::new(
                self.position,
                self.interact_audio,
            ))
            .with_children(|parent| {
                parent.spawn(DemoSensorZone2d::default());
            })
            .id()
    }
}
