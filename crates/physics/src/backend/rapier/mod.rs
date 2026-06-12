mod dim2;
mod dim3;
mod plugin;

pub use dim2::PhysicsQuery2d;
pub use dim3::PhysicsQuery3d;
pub use plugin::{add_debug_backend, add_physics_backend};
