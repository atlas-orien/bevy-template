//! 移动规则系统。
//!
//! 这里放根据移动意图、速度、时间和世界规则修改位置或朝向的 ECS 系统函数。

mod systems;

pub use systems::movement_system;
