use bevy::prelude::*;

use crate::backend;
use crate::{
    PhysicsCollisionEnded, PhysicsCollisionStarted, PhysicsConfig, PhysicsSensorTriggered,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsConfig>();
        app.add_message::<PhysicsCollisionStarted>()
            .add_message::<PhysicsCollisionEnded>()
            .add_message::<PhysicsSensorTriggered>();
        backend::add_physics_backend(app);
    }
}

pub struct PhysicsDebugPlugin;

impl Plugin for PhysicsDebugPlugin {
    fn build(&self, app: &mut App) {
        backend::add_debug_backend(app);
    }
}
