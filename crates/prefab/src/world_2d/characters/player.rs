use bevy::prelude::*;
use ecs::components::characters::player::{
    Facing, LocalPlayerControlled, MovementIntent, Player, PlayerSpeed,
};
use ecs::components::world::gameplay::{GameplayEntity, GameplaySessionEntity};
use physics::{PhysicsBody, PhysicsCollider, PhysicsLayer};
use render_2d::characters::Character2dRenderBundle;

use crate::Prefab;
use crate::identity::GameplayEntityId;

#[derive(Debug, Clone, Copy)]
pub struct Player2dPrefab {
    pub id: Option<GameplayEntityId>,
    pub position: Vec2,
    pub speed: f32,
    pub size: Vec2,
    pub color: Color,
    pub collider_radius: f32,
}

impl Default for Player2dPrefab {
    fn default() -> Self {
        Self {
            id: Some(GameplayEntityId(1)),
            position: Vec2::ZERO,
            speed: 180.0,
            size: Vec2::new(32.0, 32.0),
            color: Color::srgb(0.18, 0.58, 0.82),
            collider_radius: 16.0,
        }
    }
}

#[derive(Bundle)]
pub struct Player2dPrefabBundle {
    pub gameplay_entity: GameplayEntity,
    pub gameplay_session_entity: GameplaySessionEntity,
    pub player: Player,
    pub local_player_controlled: LocalPlayerControlled,
    pub speed: PlayerSpeed,
    pub movement_intent: MovementIntent,
    pub facing: Facing,
    pub transform: Transform,
    pub visibility: Visibility,
    pub physics_body: PhysicsBody,
    pub physics_collider: PhysicsCollider,
    pub physics_layer: PhysicsLayer,
    pub render: Character2dRenderBundle,
}

impl Player2dPrefabBundle {
    pub fn new(prefab: Player2dPrefab) -> Self {
        Self {
            gameplay_entity: GameplayEntity,
            gameplay_session_entity: GameplaySessionEntity,
            player: Player,
            local_player_controlled: LocalPlayerControlled,
            speed: PlayerSpeed(prefab.speed),
            movement_intent: MovementIntent::default(),
            facing: Facing::default(),
            transform: Transform::from_translation(prefab.position.extend(0.0)),
            visibility: Visibility::default(),
            physics_body: PhysicsBody::Dynamic,
            physics_collider: PhysicsCollider::Circle {
                radius: prefab.collider_radius,
            },
            physics_layer: PhysicsLayer::Player,
            render: Character2dRenderBundle::new(prefab.color, prefab.size),
        }
    }
}

impl Prefab for Player2dPrefab {
    fn spawn(self, commands: &mut Commands) -> Entity {
        let id = self.id;
        let mut entity = commands.spawn(Player2dPrefabBundle::new(self));

        if let Some(id) = id {
            entity.insert(id);
        }

        entity.id()
    }
}

pub use ecs::systems::movement::movement_system as player_2d_movement_system;
