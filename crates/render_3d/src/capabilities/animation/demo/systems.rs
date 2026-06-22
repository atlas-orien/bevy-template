use bevy::prelude::*;

use super::entry::DemoFox3dAnimationSet;
use crate::capabilities::animation::{AnimationPlayback3d, AnimationPlaybackMode3d};

pub struct DemoFox3dAnimationSystemPlugin;

impl Plugin for DemoFox3dAnimationSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_demo_fox_animation_state_system);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum DemoFox3dAnimationState {
    #[default]
    Idle,
    Walk,
    Run,
}

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct DemoFox3dAnimationStateSet {
    current: DemoFox3dAnimationState,
}

impl DemoFox3dAnimationStateSet {
    pub const fn new(current: DemoFox3dAnimationState) -> Self {
        Self { current }
    }

    pub const fn current(&self) -> DemoFox3dAnimationState {
        self.current
    }

    pub fn set(&mut self, next: DemoFox3dAnimationState) {
        self.current = next;
    }
}

fn sync_demo_fox_animation_state_system(
    mut foxes: Query<
        (
            &DemoFox3dAnimationStateSet,
            &DemoFox3dAnimationSet,
            &mut AnimationPlayback3d,
        ),
        Changed<DemoFox3dAnimationStateSet>,
    >,
) {
    for (state, animations, mut playback) in &mut foxes {
        playback.set_clip(animations.clip(state.current()));
        playback.set_mode(AnimationPlaybackMode3d::Repeat);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_set_updates_current_state() {
        let mut state = DemoFox3dAnimationStateSet::new(DemoFox3dAnimationState::Idle);

        state.set(DemoFox3dAnimationState::Run);

        assert_eq!(state.current(), DemoFox3dAnimationState::Run);
    }
}
