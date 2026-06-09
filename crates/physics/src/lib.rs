mod backend;
mod body;
mod collider;
mod config;
mod events;
mod force;
mod layer;
mod mass;
mod material;
mod motion;
mod plugin;
mod sensor;

pub use body::PhysicsBody;
pub use collider::PhysicsCollider;
pub use config::PhysicsConfig;
pub use error::Result;
pub use events::{PhysicsCollisionEnded, PhysicsCollisionStarted, PhysicsSensorTriggered};
pub use force::{PhysicsForce2d, PhysicsImpulse2d};
pub use layer::PhysicsLayer;
pub use mass::PhysicsMass;
pub use material::PhysicsMaterial;
pub use motion::{PhysicsAngularVelocity2d, PhysicsVelocity2d};
pub use plugin::{PhysicsDebugPlugin, PhysicsPlugin};
pub use sensor::PhysicsSensor;

#[cfg(not(any(feature = "avian2d", feature = "rapier2d")))]
compile_error!("a physics backend must be enabled: choose `avian2d` or `rapier2d`");
