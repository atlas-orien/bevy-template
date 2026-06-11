use bevy::prelude::Vec2;
use intent::movement::MovementTarget;

use crate::manager::{
    ExternalRuntimeManager, RuntimeUserId, has_user_entity, set_user_movement_intent,
};

const DEFAULT_AI_USER_ID: RuntimeUserId = RuntimeUserId(1);
const DECISION_INTERVAL_TICKS: u32 = 240;
const WAYPOINTS: [Vec2; 4] = [
    Vec2::new(180.0, 0.0),
    Vec2::new(180.0, 120.0),
    Vec2::new(-180.0, 120.0),
    Vec2::new(-180.0, 0.0),
];

#[derive(Debug, Clone)]
pub struct AiControlSource {
    tick: u32,
    waypoint_index: usize,
}

impl AiControlSource {
    pub fn new() -> Self {
        Self {
            tick: 0,
            waypoint_index: 0,
        }
    }

    pub fn poll(&mut self, manager: &ExternalRuntimeManager) {
        if !has_user_entity(manager, DEFAULT_AI_USER_ID) {
            return;
        }

        self.tick = self.tick.wrapping_add(1);
        if self.tick >= DECISION_INTERVAL_TICKS {
            self.tick = 0;
            self.waypoint_index = (self.waypoint_index + 1) % WAYPOINTS.len();
        }

        let target = MovementTarget::Position(WAYPOINTS[self.waypoint_index]);
        let _ = set_user_movement_intent(manager, DEFAULT_AI_USER_ID, target);
    }
}

impl Default for AiControlSource {
    fn default() -> Self {
        Self::new()
    }
}
