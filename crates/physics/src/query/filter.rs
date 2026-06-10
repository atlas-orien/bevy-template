use bevy::prelude::*;

use crate::PhysicsCollisionGroups;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct PhysicsQueryFilter {
    pub exclude_sensors: bool,
    pub exclude_solids: bool,
    pub groups: Option<PhysicsCollisionGroups>,
    pub exclude_collider: Option<Entity>,
    pub exclude_rigid_body: Option<Entity>,
}

impl PhysicsQueryFilter {
    pub fn exclude_sensors(mut self) -> Self {
        self.exclude_sensors = true;
        self
    }

    pub fn exclude_solids(mut self) -> Self {
        self.exclude_solids = true;
        self
    }

    pub fn groups(mut self, groups: PhysicsCollisionGroups) -> Self {
        self.groups = Some(groups);
        self
    }

    pub fn exclude_collider(mut self, collider: Entity) -> Self {
        self.exclude_collider = Some(collider);
        self
    }

    pub fn exclude_rigid_body(mut self, rigid_body: Entity) -> Self {
        self.exclude_rigid_body = Some(rigid_body);
        self
    }
}
