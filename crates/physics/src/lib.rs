mod backend;
mod body;
mod collider;
mod config;
mod layer;
mod plugin;

pub use body::PhysicsBody;
pub use collider::{PhysicsCollider, PhysicsSensor};
pub use config::PhysicsConfig;
pub use error::Result;
pub use layer::PhysicsLayer;
pub use plugin::{PhysicsDebugPlugin, PhysicsPlugin};

#[cfg(all(feature = "avian2d", feature = "rapier2d"))]
compile_error!("only one physics backend can be enabled: choose `avian2d` or `rapier2d`");

#[cfg(not(any(feature = "avian2d", feature = "rapier2d")))]
compile_error!("a physics backend must be enabled: choose `avian2d` or `rapier2d`");
