#[cfg(all(feature = "avian2d", not(feature = "rapier2d")))]
mod avian2d;

#[cfg(feature = "rapier2d")]
mod rapier2d;

use bevy::prelude::*;

pub fn add_physics_backend(app: &mut App) {
    #[cfg(all(feature = "avian2d", not(feature = "rapier2d")))]
    avian2d::add_physics_backend(app);

    #[cfg(feature = "rapier2d")]
    rapier2d::add_physics_backend(app);
}

pub fn add_debug_backend(app: &mut App) {
    #[cfg(all(feature = "avian2d", not(feature = "rapier2d")))]
    avian2d::add_debug_backend(app);

    #[cfg(feature = "rapier2d")]
    rapier2d::add_debug_backend(app);
}
