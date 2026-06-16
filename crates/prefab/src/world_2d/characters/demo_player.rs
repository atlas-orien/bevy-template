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
use render_2d::capabilities::particles::DemoParticleEmitter2d;
use render_2d::primitives::frame_animation::FrameAnimationManifest2d;
use render_2d::primitives::markers::FollowCameraTarget2d;
use render_2d::products::characters::{DemoNpcSprite2d, DemoPlayerSprite2d};
use render_2d::products::overlays::DemoHealthBarOverlay2d;

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

#[derive(Bundle)]
struct DemoCharacterCoreBundle {
    character: Character,
    gameplay: GameplayEntity,
    session: GameplaySessionEntity,
    movement: MovementIntent,
    speed: Speed,
    facing: Facing,
    transform: Transform,
    visibility: Visibility,
}

impl DemoCharacterCoreBundle {
    fn new(position: Vec2, speed: Speed) -> Self {
        Self {
            character: Character,
            gameplay: GameplayEntity,
            session: GameplaySessionEntity,
            movement: MovementIntent::default(),
            speed,
            facing: Facing::default(),
            transform: Transform::from_xyz(position.x, position.y, DEMO_CHARACTER_Z),
            visibility: Visibility::default(),
        }
    }
}

#[derive(Bundle)]
struct DemoNpcNavigationBundle {
    agent: navigation::NavigationAgent2d,
    target: navigation::NavigationTarget2d,
    path: navigation::NavigationPath2d,
}

impl Default for DemoNpcNavigationBundle {
    fn default() -> Self {
        Self {
            agent: navigation::NavigationAgent2d {
                speed: DEMO_NPC_SPEED,
                stopping_distance: DEMO_NPC_STOPPING_DISTANCE,
            },
            target: navigation::NavigationTarget2d::default(),
            path: navigation::NavigationPath2d::default(),
        }
    }
}

#[derive(Bundle)]
struct DemoPlayerIdentityBundle {
    controlled: DemoPlayerControlled,
    camera_target: FollowCameraTarget2d,
    gameplay_id: GameplayEntityId,
}

impl Default for DemoPlayerIdentityBundle {
    fn default() -> Self {
        Self {
            controlled: DemoPlayerControlled,
            camera_target: FollowCameraTarget2d,
            gameplay_id: DEMO_PLAYER_ENTITY_ID,
        }
    }
}

#[derive(Bundle)]
struct DemoPlayerVitalsBundle {
    health: Health,
    max_health: MaxHealth,
}

impl Default for DemoPlayerVitalsBundle {
    fn default() -> Self {
        Self {
            health: Health(DEMO_PLAYER_HEALTH),
            max_health: MaxHealth(DEMO_PLAYER_HEALTH),
        }
    }
}

#[derive(Bundle)]
struct DemoPlayerPhysicsBundle {
    rigid_body: PhysicsRigidBody,
    collider: PhysicsCollider2d,
    active_events: PhysicsActiveEvents,
    active_collision_types: PhysicsActiveCollisionTypes,
}

impl Default for DemoPlayerPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: PhysicsRigidBody::Kinematic,
            collider: PhysicsCollider2d::Rectangle {
                width: DEMO_PLAYER_COLLIDER_WIDTH,
                height: DEMO_PLAYER_COLLIDER_HEIGHT,
            },
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
                static_static: false,
            },
        }
    }
}

#[derive(Bundle)]
struct DemoPlayerAudioBundle {
    clips: AudioClips,
}

impl Default for DemoPlayerAudioBundle {
    fn default() -> Self {
        Self {
            clips: AudioClips::default().with_interact(DEMO_PLAYER_FOOTSTEP_AUDIO),
        }
    }
}

#[derive(Bundle)]
struct DemoNpcBundle {
    core: DemoCharacterCoreBundle,
    navigation: DemoNpcNavigationBundle,
}

impl DemoNpcBundle {
    fn new(position: Vec2) -> Self {
        Self {
            core: DemoCharacterCoreBundle::new(position, Speed(DEMO_NPC_SPEED)),
            navigation: DemoNpcNavigationBundle::default(),
        }
    }
}

#[derive(Bundle)]
struct DemoPlayerBundle {
    core: DemoCharacterCoreBundle,
    identity: DemoPlayerIdentityBundle,
    vitals: DemoPlayerVitalsBundle,
    audio: DemoPlayerAudioBundle,
    physics: DemoPlayerPhysicsBundle,
}

impl DemoPlayerBundle {
    fn new(position: Vec2) -> Self {
        Self {
            core: DemoCharacterCoreBundle::new(position, Speed::default()),
            identity: DemoPlayerIdentityBundle::default(),
            vitals: DemoPlayerVitalsBundle::default(),
            audio: DemoPlayerAudioBundle::default(),
            physics: DemoPlayerPhysicsBundle::default(),
        }
    }
}

impl DemoNpcPrefab {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl Prefab for DemoNpcPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(DemoNpcBundle::new(self.position))
            .with_children(|parent| {
                parent.spawn(DemoNpcSprite2d::default());
            })
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
            .spawn(DemoPlayerBundle::new(self.position))
            .with_children(|parent| {
                parent.spawn(DemoPlayerSprite2d::new(self.frame_manifest));
                parent.spawn(DemoParticleEmitter2d::default());
                parent.spawn(DemoHealthBarOverlay2d.into_bundle());
            })
            .id()
    }
}
