use bevy::prelude::*;

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ExampleBone2d {
    pub name: String,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ExampleSkeleton2d {
    pub name: String,
}

#[derive(Component, Debug, Clone, Copy, Default, PartialEq)]
pub struct ExampleSkeletalAnimationPlayback2d {
    pub elapsed_seconds: f32,
    pub playing: bool,
}
