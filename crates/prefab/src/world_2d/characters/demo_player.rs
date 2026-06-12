use bevy::prelude::*;
use ecs::components::{
    base::{AudioClips, Facing, Health, MaxHealth, MovementIntent, Speed},
    characters::{Character, DemoPlayerControlled},
    world::gameplay::{GameplayEntity, GameplayEntityId, GameplaySessionEntity},
};
use physics::{
    PhysicsActiveCollisionTypes, PhysicsActiveEvents, PhysicsCollider2d, PhysicsRigidBody,
};
use render_2d::camera::DemoCameraFollowTarget;
use render_2d::characters::DemoNpcSprite2dBundle;
use render_2d::characters::DemoPlayerSprite2dBundle;
use render_2d::overlays::{
    DemoHealthBarBackground2dBundle, DemoHealthBarFill2dBundle, DemoHealthBarOverlay2dBundle,
};
use render_2d::particles::DemoParticleEmitter2dBundle;

use crate::Prefab;

pub const DEMO_PLAYER_ENTITY_ID: GameplayEntityId = GameplayEntityId(1);

pub struct DemoPlayerPrefab {
    position: Vec2,
    image: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
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
                Speed(120.0),
                Facing::default(),
                navigation::NavigationAgent2d {
                    speed: 120.0,
                    stopping_distance: 3.0,
                },
                navigation::NavigationTarget2d::default(),
                navigation::NavigationPath2d::default(),
                Transform::from_xyz(self.position.x, self.position.y, 2.0),
                Visibility::default(),
                children![DemoNpcSprite2dBundle::default()],
            ))
            .id()
    }
}

impl DemoPlayerPrefab {
    pub fn new(
        position: Vec2,
        image: Handle<Image>,
        atlas_layout: Handle<TextureAtlasLayout>,
    ) -> Self {
        Self {
            position,
            image,
            atlas_layout,
        }
    }
}

impl Prefab for DemoPlayerPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Character,
                DemoPlayerControlled,
                DemoCameraFollowTarget,
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
                children![
                    DemoPlayerSprite2dBundle::new(self.image, self.atlas_layout),
                    DemoParticleEmitter2dBundle::default(),
                    (
                        DemoHealthBarOverlay2dBundle::default(),
                        children![
                            DemoHealthBarBackground2dBundle::default(),
                            DemoHealthBarFill2dBundle::default(),
                        ],
                    ),
                ],
            ))
            .insert((
                AudioClips::default().with_interact("audio/demo_footstep.ogg"),
                PhysicsRigidBody::Kinematic,
                PhysicsCollider2d::Rectangle {
                    width: 24.0,
                    height: 32.0,
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
