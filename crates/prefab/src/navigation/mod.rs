pub use ::navigation::{
    NavigationAgent2d, NavigationAgent3d, NavigationPath2d, NavigationPath3d, NavigationTarget2d,
    NavigationTarget3d,
};
use bevy::prelude::*;
use ecs::components::base::{MovementIntent, MovementTarget};

pub type NavigationTarget2dQuery<'w, 's> =
    Query<'w, 's, &'static mut NavigationTarget2d, With<NavigationAgent2d>>;
pub type NavigationTarget3dQuery<'w, 's> =
    Query<'w, 's, &'static mut NavigationTarget3d, With<NavigationAgent3d>>;

#[derive(Bundle, Default)]
pub struct Navigation2dBundle {
    pub agent: NavigationAgent2d,
    pub target: NavigationTarget2d,
    pub path: NavigationPath2d,
}

#[derive(Bundle, Default)]
pub struct Navigation3dBundle {
    pub agent: NavigationAgent3d,
    pub target: NavigationTarget3d,
    pub path: NavigationPath3d,
}

pub fn set_navigation_target_2d(
    target: &mut NavigationTarget2d,
    position: impl Into<Option<Vec2>>,
) {
    *target = position
        .into()
        .map_or(NavigationTarget2d::None, NavigationTarget2d::Position);
}

pub fn set_navigation_target_3d(
    target: &mut NavigationTarget3d,
    position: impl Into<Option<Vec3>>,
) {
    *target = position
        .into()
        .map_or(NavigationTarget3d::None, NavigationTarget3d::Position);
}

pub fn sync_demo_navigation_targets_from_intent_system(
    mut agents: Query<(&mut MovementIntent, &mut NavigationTarget2d), With<NavigationAgent2d>>,
) {
    for (mut movement, mut target) in &mut agents {
        let MovementTarget::Position(position) = movement.target else {
            continue;
        };

        set_navigation_target_2d(&mut target, position);
        movement.target = MovementTarget::None;
    }
}
