use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

pub fn add_physics_backend(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
}

pub fn add_debug_backend(app: &mut App) {
    app.add_plugins(RapierDebugRenderPlugin::default());
}
