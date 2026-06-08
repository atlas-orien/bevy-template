use bevy::prelude::*;
use ecs::components::characters::player::PlayerBundle;
use physics::{PhysicsBody, PhysicsCollider, PhysicsLayer};
use render_2d::characters::player::PlayerSpriteBundle;

#[derive(Bundle)]
pub struct Player2dPrefabBundle {
    pub player: PlayerBundle,
    pub physics_body: PhysicsBody,
    pub physics_collider: PhysicsCollider,
    pub physics_layer: PhysicsLayer,
}

impl Default for Player2dPrefabBundle {
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

impl Player2dPrefabBundle {
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

pub fn spawn_player_2d_prefab(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    position: Vec3,
) -> Entity {
    let entity = commands
        .spawn(Player2dPrefabBundle::at_position(position))
        .id();

    commands.entity(entity).with_children(|children| {
        children.spawn(PlayerSpriteBundle::from_assets(
            asset_server,
            texture_atlas_layouts,
        ));
    });

    entity
}
