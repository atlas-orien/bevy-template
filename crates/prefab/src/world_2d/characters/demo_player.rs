//! Demo player 与 NPC prefab：组合 ecs、physics、navigation 与 render 数据。

use bevy::prelude::*;
use ecs::components::{
    base::{AudioClips, Facing, Health, MaxHealth, MovementIntent, Speed},
    characters::{Character, DemoPlayerControlled},
    world::gameplay::{GameplayEntity, GameplayEntityId, GameplaySessionEntity},
};
use physics::{
    PhysicsActiveCollisionTypes, PhysicsActiveEvents, PhysicsCollider2d, PhysicsRigidBody,
};
use render_2d::animation::frame::FrameAnimationManifest2d;
use render_2d::camera::FollowCameraTarget2d;
use render_2d::characters::{DemoNpcSprite2d, DemoPlayerSprite2d};
use render_2d::overlays::DemoHealthBarOverlay2d;
use render_2d::particles::DemoParticleEmitter2d;

use crate::Prefab;

pub const DEMO_PLAYER_ENTITY_ID: GameplayEntityId = GameplayEntityId(1);
const DEMO_CHARACTER_Z: f32 = 2.0;
const DEMO_NPC_SPEED: f32 = 120.0;
const DEMO_NPC_STOPPING_DISTANCE: f32 = 3.0;
const DEMO_PLAYER_HEALTH: f32 = 100.0;
const DEMO_PLAYER_COLLIDER_WIDTH: f32 = 24.0;
const DEMO_PLAYER_COLLIDER_HEIGHT: f32 = 32.0;
const DEMO_PLAYER_FOOTSTEP_AUDIO: &str = "audio/demo_footstep.ogg";

pub struct DemoPlayerPrefab {
    position: Vec2,
    frame_manifest: Handle<FrameAnimationManifest2d>,
}

pub struct DemoNpcPrefab {
    position: Vec2,
}

impl DemoNpcPrefab {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl Prefab for DemoNpcPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Character,
                GameplayEntity,
                GameplaySessionEntity,
                MovementIntent::default(),
                Speed(DEMO_NPC_SPEED),
                Facing::default(),
                navigation::NavigationAgent2d {
                    speed: DEMO_NPC_SPEED,
                    stopping_distance: DEMO_NPC_STOPPING_DISTANCE,
                },
                navigation::NavigationTarget2d::default(),
                navigation::NavigationPath2d::default(),
                Transform::from_xyz(self.position.x, self.position.y, DEMO_CHARACTER_Z),
                Visibility::default(),
                children![DemoNpcSprite2d::default()],
            ))
            .id()
    }
}

impl DemoPlayerPrefab {
    pub fn new(position: Vec2, frame_manifest: Handle<FrameAnimationManifest2d>) -> Self {
        Self {
            position,
            frame_manifest,
        }
    }
}

impl Prefab for DemoPlayerPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Character,
                DemoPlayerControlled,
                FollowCameraTarget2d,
                GameplayEntity,
                GameplaySessionEntity,
                DEMO_PLAYER_ENTITY_ID,
                MovementIntent::default(),
                Speed::default(),
                Facing::default(),
                Health(DEMO_PLAYER_HEALTH),
                MaxHealth(DEMO_PLAYER_HEALTH),
                Transform::from_xyz(self.position.x, self.position.y, DEMO_CHARACTER_Z),
                Visibility::default(),
                children![
                    DemoPlayerSprite2d::new(self.frame_manifest),
                    DemoParticleEmitter2d::default(),
                    DemoHealthBarOverlay2d.into_bundle(),
                ],
            ))
            .insert((
                AudioClips::default().with_interact(DEMO_PLAYER_FOOTSTEP_AUDIO),
                PhysicsRigidBody::Kinematic,
                PhysicsCollider2d::Rectangle {
                    width: DEMO_PLAYER_COLLIDER_WIDTH,
                    height: DEMO_PLAYER_COLLIDER_HEIGHT,
                },
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
                    static_static: false,
                },
            ))
            .id()
    }
}
