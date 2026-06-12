use bevy::prelude::*;
use ecs::components::base::{Facing, MovementIntent};

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct DemoFrameAnimation2d {
    pub first_frame: usize,
    pub last_frame: usize,
    pub frame_seconds: f32,
    pub elapsed_seconds: f32,
}

impl DemoFrameAnimation2d {
    pub fn idle() -> Self {
        Self {
            first_frame: 0,
            last_frame: 0,
            frame_seconds: 0.12,
            elapsed_seconds: 0.0,
        }
    }

    pub fn set_range(&mut self, first_frame: usize, last_frame: usize) {
        if self.first_frame == first_frame && self.last_frame == last_frame {
            return;
        }

        self.first_frame = first_frame;
        self.last_frame = last_frame;
        self.elapsed_seconds = 0.0;
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoPlayerAnimation2d;

pub fn demo_player_animation_state_system(
    parents: Query<(&MovementIntent, Option<&Facing>)>,
    mut sprites: Query<
        (&ChildOf, &mut DemoFrameAnimation2d, &mut Sprite),
        With<DemoPlayerAnimation2d>,
    >,
) {
    for (parent, mut animation, mut sprite) in &mut sprites {
        let Ok((movement, facing)) = parents.get(parent.parent()) else {
            continue;
        };

        if movement.is_moving() {
            animation.set_range(1, 6);
        } else {
            animation.set_range(0, 0);
        }

        if let Some(facing) = facing {
            sprite.flip_x = *facing == Facing::Left;
        }
    }
}

pub fn demo_frame_animation_system(
    time: Res<Time>,
    mut sprites: Query<(&mut DemoFrameAnimation2d, &mut Sprite), With<DemoPlayerAnimation2d>>,
) {
    for (mut animation, mut sprite) in &mut sprites {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };

        if animation.first_frame == animation.last_frame {
            atlas.index = animation.first_frame;
            continue;
        }

        animation.elapsed_seconds += time.delta_secs();
        if animation.elapsed_seconds < animation.frame_seconds {
            continue;
        }

        animation.elapsed_seconds = 0.0;
        atlas.index = if atlas.index >= animation.last_frame {
            animation.first_frame
        } else {
            (atlas.index + 1).max(animation.first_frame)
        };
    }
}
