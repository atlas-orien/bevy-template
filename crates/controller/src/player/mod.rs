use bevy::prelude::*;
use components::characters::player::{Facing, MovementIntent, Player};
use simulation::flow::AppState;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            read_player_movement_input.run_if(in_state(AppState::Playing)),
        );
    }
}

fn read_player_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut MovementIntent, &mut Facing), With<Player>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    let normalized_direction = direction.normalize_or_zero();

    for (mut movement_intent, mut facing) in &mut players {
        movement_intent.direction = normalized_direction;

        if normalized_direction.x < 0.0 {
            *facing = Facing::Left;
        } else if normalized_direction.x > 0.0 {
            *facing = Facing::Right;
        }
    }
}
