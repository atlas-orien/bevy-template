//! 基础组件。
//!
//! 这里放最小颗粒度、可复用的 ECS 数据，例如名字、血量、移动、阵营。
//! 基础组件不表达“它是谁”，只表达“它拥有什么数据”。

pub mod affiliation;
pub mod health;
pub mod identity;
pub mod movement;

pub use affiliation::{Faction, Team};
pub use health::{Health, MaxHealth};
pub use identity::{DisplayName, PublicEntityId};
pub use movement::{Facing, MovementIntent, MovementTarget, Speed, Velocity2d, Velocity3d};
