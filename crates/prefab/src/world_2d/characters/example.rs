use bevy::prelude::*;
use ecs::components::base::{Facing, MovementIntent, Speed};
use ecs::components::characters::player::{LocalPlayerControlled, Player};
use ecs::components::world::gameplay::{GameplayEntity, GameplaySessionEntity};
use physics::{PhysicsCollider2d, PhysicsLayer, PhysicsRigidBody};
use render_2d::characters::ExampleGabeCharacter2dBundle;

use crate::identity::GameplayEntityId;

#[derive(Bundle)]
pub struct ExampleGabePlayer2dPrefabBundle {
    pub gameplay_entity: GameplayEntity,
    pub gameplay_session_entity: GameplaySessionEntity,
    pub gameplay_id: GameplayEntityId,
    pub player: Player,
    pub local_player_controlled: LocalPlayerControlled,
    pub speed: Speed,
    pub movement_intent: MovementIntent,
    pub facing: Facing,
    pub transform: Transform,
    pub visibility: Visibility,
    pub physics_body: PhysicsRigidBody,
    pub physics_collider: PhysicsCollider2d,
    pub physics_layer: PhysicsLayer,
    pub render: ExampleGabeCharacter2dBundle,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ExampleGround2d;

#[derive(Bundle)]
pub struct ExampleGround2dPrefabBundle {
    pub marker: ExampleGround2d,
    pub transform: Transform,
    pub visibility: Visibility,
    pub sprite: Sprite,
    pub physics_body: PhysicsRigidBody,
    pub physics_collider: PhysicsCollider2d,
    pub physics_layer: PhysicsLayer,
}

impl ExampleGabePlayer2dPrefabBundle {
    pub fn new(image: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            gameplay_entity: GameplayEntity,
            gameplay_session_entity: GameplaySessionEntity,
            gameplay_id: GameplayEntityId(1),
            player: Player,
            local_player_controlled: LocalPlayerControlled,
            speed: Speed(180.0),
            movement_intent: MovementIntent::default(),
            facing: Facing::default(),
            transform: Transform::from_scale(Vec3::splat(4.0)),
            visibility: Visibility::default(),
            physics_body: PhysicsRigidBody::Dynamic,
            physics_collider: PhysicsCollider2d::Circle { radius: 12.0 },
            physics_layer: PhysicsLayer::Player,
            render: ExampleGabeCharacter2dBundle::new(image, layout),
        }
    }
}

impl ExampleGround2dPrefabBundle {
    pub fn new() -> Self {
        let size = Vec2::new(640.0, 32.0);

        Self {
            marker: ExampleGround2d,
            transform: Transform::from_translation(Vec3::new(0.0, -140.0, 0.0)),
            visibility: Visibility::default(),
            sprite: Sprite::from_color(Color::srgb(0.22, 0.48, 0.28), size),
            physics_body: PhysicsRigidBody::Static,
            physics_collider: PhysicsCollider2d::Rectangle {
                width: size.x,
                height: size.y,
            },
            physics_layer: PhysicsLayer::World,
        }
    }
}

impl Default for ExampleGround2dPrefabBundle {
    fn default() -> Self {
        Self::new()
    }
}

pub fn spawn_example_gabe_player_2d_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("2d/animated/characters/gabe/gabe.png");
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(24),
        7,
        1,
        None,
        None,
    ));

    commands.spawn(ExampleGround2dPrefabBundle::new());
    commands.spawn(ExampleGabePlayer2dPrefabBundle::new(image, layout));
}
