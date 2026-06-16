//! Demo player 对逐帧动画 clip 的状态选择。

use bevy::prelude::*;
use ecs::components::base::{Facing, MovementIntent};

use crate::animation::frame::FrameAnimation2d;

pub const DEMO_IDLE_CLIP: &str = "idle";
pub const DEMO_WALK_CLIP: &str = "walk";

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerAnimation2d;

pub(in crate::animation::frame::demo) fn demo_player_animation_state_system(
    parents: Query<(&MovementIntent, Option<&Facing>)>,
    mut sprites: Query<(&ChildOf, &mut FrameAnimation2d, &mut Sprite), With<DemoPlayerAnimation2d>>,
) {
    for (parent, mut animation, mut sprite) in &mut sprites {
        let Ok((movement, facing)) = parents.get(parent.parent()) else {
            continue;
        };

        animation.set_clip(if movement.is_moving() {
            DEMO_WALK_CLIP
        } else {
            DEMO_IDLE_CLIP
        });

        if let Some(facing) = facing {
            sprite.flip_x = *facing == Facing::Left;
        }
    }
}

pub fn demo_player_idle_animation() -> FrameAnimation2d {
    FrameAnimation2d::new(DEMO_IDLE_CLIP)
}
