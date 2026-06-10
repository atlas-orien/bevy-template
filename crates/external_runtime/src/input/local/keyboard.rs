use bevy::prelude::Vec2;
use device_query::{DeviceQuery, DeviceState, Keycode};
use intent::movement::MovementTarget;

#[derive(Debug, Clone)]
pub struct LocalKeyboardInput {
    device_state: DeviceState,
}

impl LocalKeyboardInput {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
        }
    }

    pub fn movement_target(&self) -> MovementTarget {
        let keys = self.device_state.get_keys();
        let direction = keyboard_movement_direction(&keys);

        if direction == Vec2::ZERO {
            MovementTarget::None
        } else {
            MovementTarget::Direction(direction)
        }
    }
}

impl Default for LocalKeyboardInput {
    fn default() -> Self {
        Self::new()
    }
}

fn keyboard_movement_direction(keys: &[Keycode]) -> Vec2 {
    let mut direction = Vec2::ZERO;

    if keys.contains(&Keycode::A) || keys.contains(&Keycode::Left) {
        direction.x -= 1.0;
    }

    if keys.contains(&Keycode::D) || keys.contains(&Keycode::Right) {
        direction.x += 1.0;
    }

    if keys.contains(&Keycode::W) || keys.contains(&Keycode::Up) {
        direction.y += 1.0;
    }

    if keys.contains(&Keycode::S) || keys.contains(&Keycode::Down) {
        direction.y -= 1.0;
    }

    direction.normalize_or_zero()
}
