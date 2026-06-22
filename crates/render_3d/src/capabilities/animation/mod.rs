//! 通用 3D 动画播放能力。

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

pub struct Animation3dPlugin;

impl Plugin for Animation3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(play_animation_when_scene_ready);
    }
}

#[derive(Component, Debug, Clone)]
pub struct AnimationClip3d {
    graph: Handle<AnimationGraph>,
    node: AnimationNodeIndex,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum AnimationPlaybackMode3d {
    Once,
    Repeat,
}

#[derive(Component, Debug, Clone)]
pub struct AnimationPlayback3d {
    clip: AnimationClip3d,
    mode: AnimationPlaybackMode3d,
}

#[derive(Bundle)]
pub struct AnimationPlayback3dBundle {
    playback: AnimationPlayback3d,
}

impl AnimationClip3d {
    pub fn new(graph: Handle<AnimationGraph>, node: AnimationNodeIndex) -> Self {
        Self { graph, node }
    }

    pub fn from_clip(
        clip: Handle<AnimationClip>,
        animation_graphs: &mut Assets<AnimationGraph>,
    ) -> Self {
        let (graph, node) = AnimationGraph::from_clip(clip);
        Self::new(animation_graphs.add(graph), node)
    }

    pub fn graph(&self) -> &Handle<AnimationGraph> {
        &self.graph
    }

    pub fn node(&self) -> AnimationNodeIndex {
        self.node
    }
}

impl AnimationPlayback3d {
    pub fn repeat(clip: AnimationClip3d) -> Self {
        Self::new(clip, AnimationPlaybackMode3d::Repeat)
    }

    pub fn once(clip: AnimationClip3d) -> Self {
        Self::new(clip, AnimationPlaybackMode3d::Once)
    }

    pub fn new(clip: AnimationClip3d, mode: AnimationPlaybackMode3d) -> Self {
        Self { clip, mode }
    }

    pub fn clip(&self) -> &AnimationClip3d {
        &self.clip
    }

    pub fn mode(&self) -> AnimationPlaybackMode3d {
        self.mode
    }

    pub fn into_bundle(self) -> AnimationPlayback3dBundle {
        AnimationPlayback3dBundle { playback: self }
    }
}

pub fn play_animation_when_scene_ready(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    playback_roots: Query<&AnimationPlayback3d>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let scene_root = scene_ready.entity;
    let Ok(playback) = playback_roots.get(scene_root) else {
        return;
    };

    for child in children.iter_descendants(scene_root) {
        let Ok(mut player) = players.get_mut(child) else {
            continue;
        };

        let mut active_animation = player.play(playback.clip.node());
        if playback.mode == AnimationPlaybackMode3d::Repeat {
            active_animation = active_animation.repeat();
        }

        active_animation.set_speed(1.0);
        commands
            .entity(child)
            .insert(AnimationGraphHandle(playback.clip.graph().clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_records_repeat_mode() {
        let playback = AnimationPlayback3d::repeat(AnimationClip3d::new(
            Handle::default(),
            AnimationNodeIndex::new(0),
        ));

        assert_eq!(playback.mode(), AnimationPlaybackMode3d::Repeat);
    }
}
