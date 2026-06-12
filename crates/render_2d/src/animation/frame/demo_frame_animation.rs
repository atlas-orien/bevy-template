//! Demo 角色的逐帧动画状态与推进系统。

use bevy::prelude::*;
use ecs::components::base::{Facing, MovementIntent};

const DEMO_IDLE_FIRST_FRAME: usize = 0;
const DEMO_IDLE_LAST_FRAME: usize = 0;
const DEMO_WALK_FIRST_FRAME: usize = 1;
const DEMO_WALK_LAST_FRAME: usize = 6;
const DEMO_FRAME_SECONDS: f32 = 0.12;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct DemoFrameAnimation2d {
    pub first_frame: usize,
    pub last_frame: usize,
    pub frame_seconds: f32,
    elapsed_seconds: f32,
}

impl DemoFrameAnimation2d {
    pub fn idle() -> Self {
        Self {
            first_frame: DEMO_IDLE_FIRST_FRAME,
            last_frame: DEMO_IDLE_LAST_FRAME,
            frame_seconds: DEMO_FRAME_SECONDS,
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

    pub fn tick(&mut self, delta_seconds: f32) -> bool {
        if self.first_frame == self.last_frame {
            return false;
        }

        self.elapsed_seconds += delta_seconds;
        if self.elapsed_seconds < self.frame_seconds {
            return false;
        }

        self.elapsed_seconds = 0.0;
        true
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
            animation.set_range(DEMO_WALK_FIRST_FRAME, DEMO_WALK_LAST_FRAME);
        } else {
            animation.set_range(DEMO_IDLE_FIRST_FRAME, DEMO_IDLE_LAST_FRAME);
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

        if !animation.tick(time.delta_secs()) {
            continue;
        }

        atlas.index = if atlas.index >= animation.last_frame {
            animation.first_frame
        } else {
            (atlas.index + 1).max(animation.first_frame)
        };
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn animation_app(delta_seconds: f32) -> App {
        let mut app = App::new();
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(delta_seconds));
        app.insert_resource(time)
            .add_systems(Update, demo_frame_animation_system);
        app
    }

    fn animated_sprite(index: usize, animation: DemoFrameAnimation2d) -> impl Bundle {
        (
            animation,
            DemoPlayerAnimation2d,
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: Handle::default(),
                    index,
                }),
                ..default()
            },
        )
    }

    #[test]
    fn single_frame_animation_stays_on_frame() {
        let mut app = animation_app(DEMO_FRAME_SECONDS);
        let entity = app
            .world_mut()
            .spawn(animated_sprite(4, DemoFrameAnimation2d::idle()))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index,
            DEMO_IDLE_FIRST_FRAME
        );
    }

    #[test]
    fn multi_frame_animation_advances_after_frame_seconds() {
        let mut app = animation_app(DEMO_FRAME_SECONDS);
        let mut animation = DemoFrameAnimation2d::idle();
        animation.set_range(DEMO_WALK_FIRST_FRAME, DEMO_WALK_LAST_FRAME);
        let entity = app
            .world_mut()
            .spawn(animated_sprite(DEMO_WALK_FIRST_FRAME, animation))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index,
            DEMO_WALK_FIRST_FRAME + 1
        );
    }

    #[test]
    fn multi_frame_animation_wraps_from_last_to_first() {
        let mut app = animation_app(DEMO_FRAME_SECONDS);
        let mut animation = DemoFrameAnimation2d::idle();
        animation.set_range(DEMO_WALK_FIRST_FRAME, DEMO_WALK_LAST_FRAME);
        let entity = app
            .world_mut()
            .spawn(animated_sprite(DEMO_WALK_LAST_FRAME, animation))
            .id();

        app.update();

        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index,
            DEMO_WALK_FIRST_FRAME
        );
    }

    #[test]
    fn set_range_resets_elapsed_only_when_range_changes() {
        let mut animation = DemoFrameAnimation2d::idle();
        animation.set_range(DEMO_WALK_FIRST_FRAME, DEMO_WALK_LAST_FRAME);
        assert!(!animation.tick(DEMO_FRAME_SECONDS / 2.0));

        animation.set_range(DEMO_WALK_FIRST_FRAME, DEMO_WALK_LAST_FRAME);
        assert!(animation.elapsed_seconds > 0.0);

        animation.set_range(DEMO_IDLE_FIRST_FRAME, DEMO_IDLE_LAST_FRAME);
        assert_eq!(animation.elapsed_seconds, 0.0);
        assert!(!animation.tick(DEMO_FRAME_SECONDS / 2.0));
    }
}
