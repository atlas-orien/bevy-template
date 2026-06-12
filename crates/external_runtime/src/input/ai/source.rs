use bevy::prelude::Vec2;
use intent::movement::MovementTarget;
use prefab::world_2d::characters::DemoNpcPrefab;

use crate::manager::{
    ExternalRuntimeManager, RuntimeObjectId, has_object_entity, set_object_movement_intent,
    spawn_prefab_for_object,
};

const DEMO_AI_NPC_OBJECT_ID: RuntimeObjectId = RuntimeObjectId(1);
const DECISION_INTERVAL_TICKS: u32 = 240;
const WAYPOINTS: [Vec2; 4] = [
    Vec2::new(220.0, 96.0),
    Vec2::new(220.0, 180.0),
    Vec2::new(-220.0, 180.0),
    Vec2::new(-220.0, 96.0),
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
        if manager.entities().is_empty() {
            return;
        }

        if !has_object_entity(manager, DEMO_AI_NPC_OBJECT_ID) {
            let _ = spawn_prefab_for_object(
                manager,
                DEMO_AI_NPC_OBJECT_ID,
                DemoNpcPrefab::new(Vec2::new(-120.0, 96.0)),
            );
            return;
        }

        self.tick = self.tick.wrapping_add(1);
        if self.tick >= DECISION_INTERVAL_TICKS {
            self.tick = 0;
            self.waypoint_index = (self.waypoint_index + 1) % WAYPOINTS.len();
        }

        let target = MovementTarget::Position(WAYPOINTS[self.waypoint_index]);
        let _ = set_object_movement_intent(manager, DEMO_AI_NPC_OBJECT_ID, target);
    }
}

impl Default for AiControlSource {
    fn default() -> Self {
        Self::new()
    }
}
