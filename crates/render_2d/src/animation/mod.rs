pub mod frame;
pub mod skeletal;

use bevy::prelude::*;

pub use frame::{SpriteAnimationClip2d, SpriteAnimationFrame2d, SpriteAnimationPlayback2d};
pub use skeletal::{Bone2d, SkeletalAnimationPlayback2d, Skeleton2d};

pub struct Animation2dPlugin;

impl Plugin for Animation2dPlugin {
    fn build(&self, _app: &mut App) {}
}
