use bevy::prelude::*;
use components::characters::player::{MovementIntent, PlayerSpeed};

use crate::flow::AppState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_by_intent.run_if(in_state(AppState::Playing)));
    }
}

fn move_by_intent(
    time: Res<Time>,
    mut movers: Query<(&MovementIntent, &PlayerSpeed, &mut Transform)>,
) {
    for (movement_intent, speed, mut transform) in &mut movers {
        let velocity = movement_intent.direction * speed.0 * time.delta_secs();
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
