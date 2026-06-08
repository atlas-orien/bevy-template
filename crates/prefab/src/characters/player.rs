use bevy::prelude::*;
use ecs::components::characters::player::PlayerBundle;
use physics::{PhysicsBody, PhysicsCollider, PhysicsLayer};

#[derive(Bundle)]
pub struct PlayerPrefabBundle {
    pub player: PlayerBundle,
    pub physics_body: PhysicsBody,
    pub physics_collider: PhysicsCollider,
    pub physics_layer: PhysicsLayer,
}

impl Default for PlayerPrefabBundle {
    fn default() -> Self {
        Self {
            player: PlayerBundle::default(),
            physics_body: PhysicsBody::Dynamic,
            physics_collider: PhysicsCollider::Rectangle {
                width: 24.0,
                height: 24.0,
            },
            physics_layer: PhysicsLayer::Player,
        }
    }
}

impl PlayerPrefabBundle {
    pub fn at_position(position: Vec3) -> Self {
        Self {
            player: PlayerBundle {
                transform: Transform::from_translation(position),
                ..default()
            },
            ..default()
        }
    }
}
