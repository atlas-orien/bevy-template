use bevy::prelude::*;
use intent::movement::{MovementIntentQuery, set_movement_intent};
use prefab::intent::{LocalMovementIntentEntities, MovementTarget};

pub fn keyboard_movement_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    controlled_entities: LocalMovementIntentEntities,
    mut movement_intents: MovementIntentQuery,
) {
    let direction = keyboard_movement_direction(&keyboard_input);
    let target = if direction == Vec2::ZERO {
        MovementTarget::None
    } else {
        MovementTarget::Direction(direction)
    };

    for entity in &controlled_entities {
        let _ = set_movement_intent(entity, target, &mut movement_intents);
    }
}

fn keyboard_movement_direction(keyboard_input: &ButtonInput<KeyCode>) -> Vec2 {
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

    direction.normalize_or_zero()
}
